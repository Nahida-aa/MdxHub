use std::sync::{Arc, atomic::AtomicU64};

pub mod user;

pub struct Client {
    id: AtomicU64,
    peer: Arc<Peer>,
    http: Arc<HttpClientWithUrl>,
    cloud_client: Arc<CloudApiClient>,
    telemetry: Arc<Telemetry>,
    credentials_provider: ClientCredentialsProvider,
    state: RwLock<ClientState>,
    handler_set: Mutex<ProtoMessageHandlerSet>,
    message_to_client_handlers: Mutex<Vec<MessageToClientHandler>>,
    sign_out_tx: Mutex<Option<mpsc::UnboundedSender<()>>>,

    #[allow(clippy::type_complexity)]
    #[cfg(any(test, feature = "test-support"))]
    authenticate:
        RwLock<Option<Box<dyn 'static + Send + Sync + Fn(&AsyncApp) -> Task<Result<Credentials>>>>>,

    #[allow(clippy::type_complexity)]
    #[cfg(any(test, feature = "test-support"))]
    establish_connection: RwLock<
        Option<
            Box<
                dyn 'static
                    + Send
                    + Sync
                    + Fn(
                        &Credentials,
                        &AsyncApp,
                    ) -> Task<Result<Connection, EstablishConnectionError>>,
            >,
        >,
    >,

    // #[cfg(any(test, feature = "test-support"))]
    rpc_url: RwLock<Option<Url>>,
}
