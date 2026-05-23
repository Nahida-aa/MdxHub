// use super::{
//     // Connection,
//     // message_stream::{
//     //     // Message,
//     //     // MessageStream
//     // },
// };
use collections::HashMap;
use parking_lot::{Mutex, RwLock};
use std::sync::{Arc, atomic::AtomicU32};

use futures::{
    FutureExt, SinkExt, StreamExt, TryFutureExt,
    channel::{mpsc, oneshot},
    stream::BoxStream,
};
use serde::{Serialize, ser::SerializeStruct};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize)]
pub struct ConnectionId {
    pub owner_id: u32,
    pub id: u32,
}

pub struct Peer {
    epoch: AtomicU32,
    pub connections: RwLock<HashMap<ConnectionId, ConnectionState>>,
    next_connection_id: AtomicU32,
}

#[derive(Clone, Serialize)]
pub struct ConnectionState {
    // #[serde(skip)]
    // outgoing_tx: mpsc::UnboundedSender<Message>,
    next_message_id: Arc<AtomicU32>,
    // #[allow(clippy::type_complexity)]
    // #[serde(skip)]
    // response_channels: Arc<
    //     Mutex<
    //         Option<
    //             HashMap<
    //                 u32,
    //                 oneshot::Sender<(proto::Envelope, std::time::Instant, oneshot::Sender<()>)>,
    //             >,
    //         >,
    //     >,
    // >,
    // #[allow(clippy::type_complexity)]
    // #[serde(skip)]
    // stream_response_channels: Arc<
    //     Mutex<
    //         Option<
    //             HashMap<u32, mpsc::UnboundedSender<(Result<proto::Envelope>, oneshot::Sender<()>)>>,
    //         >,
    //     >,
    // >,
}
