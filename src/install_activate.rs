use holochain::conductor::{
    api::error::{ConductorApiError, ConductorApiResult},
    error::CreateAppError,
    CellError, ConductorHandle,
};
use holochain_keystore::KeystoreSenderExt;
#[allow(deprecated)]
use holochain_types::{
    app::InstalledAppId,
    prelude::{DnaBundle, InstalledCell},
};
use holochain_zome_types::CellId;
use tokio::{sync::mpsc};

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
        .map(|result| {
          result.unwrap()
        })
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

pub async fn activate_app(
    conductor_handle: &ConductorHandle,
    app_id: InstalledAppId,
    event_channel: &Option<mpsc::Sender<StateSignal>>,
) -> ConductorApiResult<()> {
    // Activate app
    emit(event_channel, StateSignal::ActivatingApp).await;
    conductor_handle.activate_app(app_id.clone()).await?;
    // Create cells
    emit(event_channel, StateSignal::SettingUpCells).await;
    let errors = conductor_handle.clone().setup_cells().await?;
    // Check if this app was created successfully
    errors
        .into_iter()
        // We only care about this app for the activate command
        .find(|cell_error| match cell_error {
            CreateAppError::Failed {
                installed_app_id: error_app_id,
                ..
            } => error_app_id == &app_id,
        })
        // There was an error in this app so return it
        .map(|this_app_error| {
            let CreateAppError::Failed { errors: ee, .. } = this_app_error;
            let b = ee[0].to_string();
            tracing::error!("{:?}", b);
            // TODO -> this was annoying because I couldn't Copy the
            // real CellError
            Err(ConductorApiError::CellError(CellError::Todo))
        })
        // Unwrap the Option, and if None, return success
        .unwrap_or(Ok(()))
}
