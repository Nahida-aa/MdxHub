use std::{
    sync::{Arc, atomic::AtomicU64},
    time::Instant,
};

use gpui::{AsyncApp, Task};
use http_client::HttpClientWithUrl;
#[cfg(any(test, feature = "test-support"))]
use http_client::Url;
use parking_lot::{Mutex, RwLock};
use rpc::{ConnectionId, Peer};
pub mod user;
use anyhow::{Context as _, Result, anyhow};
use futures::{
    AsyncReadExt, FutureExt, SinkExt, Stream, StreamExt, TryFutureExt as _, TryStreamExt,
    channel::{mpsc, oneshot},
    future::BoxFuture,
    stream::BoxStream,
};
use postage::watch;

pub struct Client {
    id: AtomicU64,
    peer: Arc<Peer>,
    http: Arc<HttpClientWithUrl>,
    // cloud_client: Arc<CloudApiClient>,
    // telemetry: Arc<Telemetry>,
    // credentials_provider: ClientCredentialsProvider,
    state: RwLock<ClientState>,
    // handler_set: Mutex<ProtoMessageHandlerSet>,
    // message_to_client_handlers: Mutex<Vec<MessageToClientHandler>>,
    sign_out_tx: Mutex<Option<mpsc::UnboundedSender<()>>>,

    #[allow(clippy::type_complexity)]
    #[cfg(any(test, feature = "test-support"))]
    authenticate:
        RwLock<Option<Box<dyn 'static + Send + Sync + Fn(&AsyncApp) -> Task<Result<Credentials>>>>>,

    // #[allow(clippy::type_complexity)]
    // #[cfg(any(test, feature = "test-support"))]
    // establish_connection: RwLock<
    //     Option<
    //         Box<
    //             dyn 'static
    //                 + Send
    //                 + Sync
    //                 + Fn(
    //                     &Credentials,
    //                     &AsyncApp,
    //                 ) -> Task<Result<Connection, EstablishConnectionError>>,
    //         >,
    //     >,
    // >,
    #[cfg(any(test, feature = "test-support"))]
    rpc_url: RwLock<Option<Url>>,
}

struct ClientState {
    credentials: Option<Credentials>,
    status: (watch::Sender<Status>, watch::Receiver<Status>),
    _reconnect_task: Option<Task<()>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credentials {
    pub user_id: u64,
    pub access_token: String,
}

impl Credentials {
    pub fn authorization_header(&self) -> String {
        format!("{} {}", self.user_id, self.access_token)
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    SignedOut,
    UpgradeRequired,
    Authenticating,
    Authenticated,
    AuthenticationError,
    Connecting,
    ConnectionError,
    // Connected {
    //     peer_id: PeerId,
    //     connection_id: ConnectionId,
    // },
    ConnectionLost,
    Reauthenticating,
    Reauthenticated,
    Reconnecting,
    ReconnectionError { next_reconnection: Instant },
}
