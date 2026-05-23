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

/// A [version vector](https://en.wikipedia.org/wiki/Version_vector).
#[derive(Default, Hash, Eq, PartialEq)]
pub struct Global {
    // 4 is chosen as it is the biggest count that does not increase the size of the field itself.
    // Coincidentally, it also covers all the important non-collab replica ids.
    values: SmallVec<[u32; 4]>,
}
impl Clone for Global {
    fn clone(&self) -> Self {
        // We manually implement clone to avoid the overhead of SmallVec's clone implementation.
        // Using `from_slice` is faster than `clone` for SmallVec as we can use our `Copy` implementation of u32.
        Self {
            values: SmallVec::from_slice(&self.values),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.values.clone_from(&source.values);
    }
}
impl Global {
    /// Iterates all replicas observed by this global as well as any unobserved replicas whose ID is lower than the highest observed replica.
    pub fn iter(&self) -> impl Iterator<Item = Lamport> + '_ {
        self.values
            .iter()
            .enumerate()
            .map(|(replica_id, seq)| Lamport {
                replica_id: ReplicaId(replica_id as u16),
                value: *seq,
            })
    }
}

/// A [Lamport sequence number](https://en.wikipedia.org/wiki/Lamport_timestamp).
pub type Seq = u32;

/// A [Lamport timestamp](https://en.wikipedia.org/wiki/Lamport_timestamp),
/// used to determine the ordering of events in the editor.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Lamport {
    pub value: Seq,
    pub replica_id: ReplicaId,
}
impl Lamport {
    pub const MIN: Self = Self {
        replica_id: ReplicaId(u16::MIN),
        value: Seq::MIN,
    };

    pub const MAX: Self = Self {
        replica_id: ReplicaId(u16::MAX),
        value: Seq::MAX,
    };
}
impl Ord for Lamport {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use the replica id to break ties between concurrent events.
        self.value
            .cmp(&other.value)
            .then_with(|| self.replica_id.cmp(&other.replica_id))
    }
}
impl PartialOrd for Lamport {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Debug for Lamport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Self::MAX {
            write!(f, "Lamport {{MAX}}")
        } else if *self == Self::MIN {
            write!(f, "Lamport {{MIN}}")
        } else {
            write!(f, "Lamport {{{:?}: {}}}", self.replica_id, self.value)
        }
    }
}

impl fmt::Debug for Global {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Global {{")?;
        for timestamp in self.iter().filter(|t| t.value > 0) {
            if timestamp.replica_id.0 > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}: {}", timestamp.replica_id, timestamp.value)?;
        }
        write!(f, "}}")
    }
}
