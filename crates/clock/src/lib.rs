use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt,
};

/// A unique identifier for each distributed node.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ReplicaId(u16);

impl ReplicaId {
    /// The local replica
    pub const LOCAL: ReplicaId = ReplicaId(0);
    /// The remote replica of the connected remote server.
    pub const REMOTE_SERVER: ReplicaId = ReplicaId(1);
    /// The agent's unique identifier.
    pub const AGENT: ReplicaId = ReplicaId(2);
    /// A local branch.
    pub const LOCAL_BRANCH: ReplicaId = ReplicaId(3);
    /// The first collaborative replica ID, any replica equal or greater than this is a collaborative replica.
    pub const FIRST_COLLAB_ID: ReplicaId = ReplicaId(8);

    pub fn new(id: u16) -> Self {
        ReplicaId(id)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }

    pub fn is_remote(self) -> bool {
        self == ReplicaId::REMOTE_SERVER || self >= ReplicaId::FIRST_COLLAB_ID
    }
}
impl fmt::Debug for ReplicaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == ReplicaId::LOCAL {
            write!(f, "<local>")
        } else if *self == ReplicaId::REMOTE_SERVER {
            write!(f, "<remote>")
        } else if *self == ReplicaId::AGENT {
            write!(f, "<agent>")
        } else if *self == ReplicaId::LOCAL_BRANCH {
            write!(f, "<branch>")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
