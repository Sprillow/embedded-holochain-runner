use tokio::sync::mpsc;
use tracing::error;

#[derive(Debug, Clone)]
pub enum StateSignal {
    IsFirstRun,
    IsNotFirstRun,
    // IsFirstRun events
    CreatingKeys,
    RegisteringDna,
    InstallingApp,
    EnablingApp,
    AddingAppInterface,
    // Done/Ready Event
    IsReady,
}

pub async fn emit(event_channel: &Option<mpsc::Sender<StateSignal>>, event: StateSignal) {
    if let Some(is_sender) = event_channel {
        match is_sender.send(event).await {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e.to_string());
            }
        };
    }
}
