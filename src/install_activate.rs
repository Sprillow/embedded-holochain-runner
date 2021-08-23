use holochain::conductor::{
    api::error::{ConductorApiError, ConductorApiResult},
    error::ConductorError,
    CellError, ConductorHandle,
};
use holochain_keystore::KeystoreSenderExt;
#[allow(deprecated)]
use holochain_types::{
    app::InstalledAppId,
    prelude::{DnaBundle, InstalledCell},
};
use holochain_zome_types::CellId;
use tokio::sync::mpsc;

use crate::emit::{emit, StateSignal};

pub async fn install_app(
    conductor_handle: &ConductorHandle,
    app_id: InstalledAppId,
    dnas: Vec<(Vec<u8>, String)>,
    event_channel: &Option<mpsc::Sender<StateSignal>>,
) -> ConductorApiResult<()> {
    emit(event_channel, StateSignal::CreatingKeys).await;
    println!("Don't recognize you, so generating a new identity for you...");
    let agent_key = conductor_handle
        .keystore()
        .clone()
        .generate_sign_keypair_from_pure_entropy()
        .await?;
    emit(event_channel, StateSignal::RegisteringDna).await;
    println!("Your new private keys are generated, continuing with the installation...");
    // register any dnas
    let tasks = dnas.into_iter().map(|(dna_bytes, nick)| {
        let agent_key = agent_key.clone();
        let conductor_handle_clone = conductor_handle.clone();
        tokio::task::spawn(async move {
            println!("decoding dna bundle");
            let dna = DnaBundle::decode(&dna_bytes)?;
            println!("converting to dna file");
            let (dna_file, dna_hash) = dna.into_dna_file(None, None).await?;
            println!("calling register dna");
            conductor_handle_clone.register_dna(dna_file).await?;
            let cell_id = CellId::from((dna_hash.clone(), agent_key));
            #[allow(deprecated)]
            ConductorApiResult::Ok((InstalledCell::new(cell_id, nick), None))
        })
    });
    // Join all the install tasks
    let cell_ids_with_proofs = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|result| result.unwrap())
        // Check all passed and return the proofs
        .collect::<Result<Vec<_>, _>>()?;
    emit(event_channel, StateSignal::InstallingApp).await;
    // Install the CellIds as an "app", with an installed_app_id
    conductor_handle
        .clone()
        .install_app(app_id, cell_ids_with_proofs.clone())
        .await?;
    Ok(())
}

pub async fn enable_app(
    conductor_handle: &ConductorHandle,
    app_id: InstalledAppId,
    event_channel: &Option<mpsc::Sender<StateSignal>>,
) -> ConductorApiResult<()> {
    // Enable app
    emit(event_channel, StateSignal::EnablingApp).await;
    let (_app, mut errors) = conductor_handle.clone().enable_app(&app_id).await?;
    conductor_handle
        .get_app_info(&app_id)
        .await?
        .ok_or(ConductorError::AppNotInstalled(app_id))?;
    if errors.len() > 0 {
        if let Some((_cell_id, cell_error)) = errors.pop() {
            Err(cell_error.into())
        } else {
            Err(ConductorApiError::CellError(CellError::Todo))
        }
    } else {
        Ok(())
    }
}
