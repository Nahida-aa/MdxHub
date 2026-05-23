use collections::HashMap;
use parking_lot::Mutex;
// use rpc::{
//     // AnyProtoClient, ErrorExt, ProtoClient, ProtoMessageHandlerSet, RpcError,
//     // proto::{
//     //     self,
//     //     Envelope,
//     //     // EnvelopedMessage, PeerId, RequestMessage, build_typed_envelope
//     // },
//     /// TODO
// };
use anyhow::{Context as _, Result, anyhow};
use futures::{
    Future, FutureExt as _, StreamExt as _,
    channel::{
        mpsc::{self, Sender, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    future::{BoxFuture, Shared, WeakShared},
    select, select_biased,
    stream::BoxStream,
};
use gpui::{
    App, AppContext as _, AsyncApp, BackgroundExecutor, BorrowAppContext, Context, Entity,
    EventEmitter, FutureExt, Global, Task, WeakEntity,
};
use rpc::ProtoMessageHandlerSet;
use std::sync::{Arc, atomic::AtomicU32};

use crate::SshConnectionOptions;
use util::{
    ResultExt,
    paths::{
        PathStyle,
        // RemotePathBuf
    },
};

enum State {
    Connecting,
    Connected {
        // remote_connection: Arc<dyn RemoteConnection>,
        // delegate: Arc<dyn RemoteClientDelegate>,
        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    HeartbeatMissed {
        missed_heartbeats: usize,

        // remote_connection: Arc<dyn RemoteConnection>,
        // delegate: Arc<dyn RemoteClientDelegate>,
        multiplex_task: Task<Result<()>>,
        heartbeat_task: Task<Result<()>>,
    },
    Reconnecting,
    ReconnectFailed {
        // remote_connection: Arc<dyn RemoteConnection>,
        // delegate: Arc<dyn RemoteClientDelegate>,
        error: anyhow::Error,
        attempts: usize,
    },
    ReconnectExhausted,
    ServerNotRunning,
}

pub(crate) struct ChannelClient {
    next_message_id: AtomicU32,
    // outgoing_tx: Mutex<mpsc::UnboundedSender<Envelope>>,
    // buffer: Mutex<VecDeque<Envelope>>,
    // response_channels: ResponseChannels,
    // stream_response_channels: StreamResponseChannels,
    message_handlers: Mutex<ProtoMessageHandlerSet>,
    max_received: AtomicU32,
    name: &'static str,
    task: Mutex<Task<Result<()>>>,
    remote_started: Signal<()>,
    has_wsl_interop: bool,
    executor: BackgroundExecutor,
}
/// The state of the ssh connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    HeartbeatMissed,
    Reconnecting,
    Disconnected,
}
impl From<&State> for ConnectionState {
    fn from(value: &State) -> Self {
        match value {
            State::Connecting => Self::Connecting,
            State::Connected { .. } => Self::Connected,
            State::Reconnecting | State::ReconnectFailed { .. } => Self::Reconnecting,
            State::HeartbeatMissed { .. } => Self::HeartbeatMissed,
            State::ReconnectExhausted => Self::Disconnected,
            State::ServerNotRunning => Self::Disconnected,
        }
    }
}
pub struct RemoteClient {
    client: Arc<ChannelClient>,
    unique_identifier: String,
    connection_options: RemoteConnectionOptions,
    path_style: PathStyle,
    state: Option<State>,
}
impl RemoteClient {
    pub fn connection_state(&self) -> ConnectionState {
        self.state
            .as_ref()
            .map(ConnectionState::from)
            .unwrap_or(ConnectionState::Disconnected)
    }
    pub fn is_disconnected(&self) -> bool {
        self.connection_state() == ConnectionState::Disconnected
    }
}

struct Signal<T> {
    tx: Mutex<Option<oneshot::Sender<T>>>,
    rx: Shared<Task<Option<T>>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RemoteConnectionOptions {
    Ssh(SshConnectionOptions),
    // Wsl(WslConnectionOptions),
    // Docker(DockerConnectionOptions),
    // #[cfg(any(test, feature = "test-support"))]
    // Mock(crate::transport::mock::MockConnectionOptions),
}

// #[async_trait(?Send)]
// pub trait RemoteConnection: Send + Sync {
//     fn start_proxy(
//         &self,
//         unique_identifier: String,
//         reconnect: bool,
//         // incoming_tx: UnboundedSender<Envelope>,
//         // outgoing_rx: UnboundedReceiver<Envelope>,
//         connection_activity_tx: Sender<()>,
//         delegate: Arc<dyn RemoteClientDelegate>,
//         cx: &mut AsyncApp,
//     ) -> Task<Result<i32>>;
//     fn upload_directory(
//         &self,
//         src_path: PathBuf,
//         dest_path: RemotePathBuf,
//         cx: &App,
//     ) -> Task<Result<()>>;
//     async fn kill(&self) -> Result<()>;
//     fn has_been_killed(&self) -> bool;
//     fn shares_network_interface(&self) -> bool {
//         false
//     }
//     fn build_command(
//         &self,
//         program: Option<String>,
//         args: &[String],
//         env: &HashMap<String, String>,
//         working_dir: Option<String>,
//         port_forward: Option<(u16, String, u16)>,
//         interactive: Interactive,
//     ) -> Result<CommandTemplate>;
//     fn build_forward_ports_command(
//         &self,
//         forwards: Vec<(u16, String, u16)>,
//     ) -> Result<CommandTemplate>;
//     fn connection_options(&self) -> RemoteConnectionOptions;
//     fn path_style(&self) -> PathStyle;
//     fn shell(&self) -> String;
//     fn default_system_shell(&self) -> String;
//     fn has_wsl_interop(&self) -> bool;

//     #[cfg(any(test, feature = "test-support"))]
//     fn simulate_disconnect(&self, _: &AsyncApp) {}
// }
