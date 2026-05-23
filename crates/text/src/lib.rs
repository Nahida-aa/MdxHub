pub use patch::Patch;
use postage::{oneshot, prelude::*};
use smallvec::SmallVec;
pub mod locator;
use locator::Locator;
use std::{
    borrow::Cow,
    cmp::{self, Ordering, Reverse},
    fmt::Display,
    future::Future,
    iter::Iterator,
    num::NonZeroU64,
    ops::{self, Deref, Range, Sub},
    str,
    sync::{Arc, LazyLock},
    time::{Duration, Instant},
};
use undo_map::UndoMap;
pub mod operation_queue;
mod patch;
pub mod subscription;
mod undo_map;
use clock::{Lamport, ReplicaId};
use collections::{HashMap, HashSet};
use operation_queue::OperationQueue;
use rope::Rope;
use sum_tree::{SumTree, TreeMap, TreeSet};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineEnding {
    Unix,
    Windows,
}
pub use subscription::*;
pub struct Buffer {
    snapshot: BufferSnapshot,
    history: History,
    deferred_ops: OperationQueue<Operation>,
    deferred_replicas: HashSet<ReplicaId>,
    pub lamport_clock: clock::Lamport,
    subscriptions: Topic<usize>,
    edit_id_resolvers: HashMap<clock::Lamport, Vec<oneshot::Sender<()>>>,
    wait_for_version_txs: Vec<(clock::Global, oneshot::Sender<()>)>,
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct BufferId(NonZeroU64);

#[derive(Clone)]
pub struct BufferSnapshot {
    visible_text: Rope,
    deleted_text: Rope,
    fragments: SumTree<Fragment>,
    insertions: SumTree<InsertionFragment>,
    insertion_slices: TreeSet<InsertionSlice>,
    undo_map: UndoMap,
    pub version: clock::Global,
    remote_id: BufferId,
    replica_id: ReplicaId,
    line_ending: LineEnding,
}

pub type TransactionId = clock::Lamport;

#[derive(Clone, Debug)]
pub struct HistoryEntry {
    transaction: Transaction,
    first_edit_at: Instant,
    last_edit_at: Instant,
    suppress_grouping: bool,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: TransactionId,
    pub edit_ids: Vec<clock::Lamport>,
    pub start: clock::Global,
}

struct History {
    base_text: Rope,
    operations: TreeMap<clock::Lamport, Operation>,
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    transaction_depth: usize,
    group_interval: Duration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InsertionSlice {
    // Inline the lamports to allow the replica ids to share the same alignment
    // saving 4 bytes space edit_id: clock::Lamport,
    edit_id_value: clock::Seq,
    edit_id_replica_id: ReplicaId,
    // insertion_id: clock::Lamport,
    insertion_id_value: clock::Seq,
    insertion_id_replica_id: ReplicaId,
    range: Range<u32>,
}
impl Ord for InsertionSlice {
    fn cmp(&self, other: &Self) -> Ordering {
        Lamport {
            value: self.edit_id_value,
            replica_id: self.edit_id_replica_id,
        }
        .cmp(&Lamport {
            value: other.edit_id_value,
            replica_id: other.edit_id_replica_id,
        })
        .then_with(|| {
            Lamport {
                value: self.insertion_id_value,
                replica_id: self.insertion_id_replica_id,
            }
            .cmp(&Lamport {
                value: other.insertion_id_value,
                replica_id: other.insertion_id_replica_id,
            })
        })
        .then_with(|| self.range.start.cmp(&other.range.start))
        .then_with(|| self.range.end.cmp(&other.range.end))
    }
}

impl PartialOrd for InsertionSlice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Edit(EditOperation),
    Undo(UndoOperation),
}
impl operation_queue::Operation for Operation {
    fn lamport_timestamp(&self) -> clock::Lamport {
        match self {
            Operation::Edit(edit) => edit.timestamp,
            Operation::Undo(undo) => undo.timestamp,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditOperation {
    pub timestamp: clock::Lamport,
    pub version: clock::Global,
    pub ranges: Vec<Range<FullOffset>>,
    pub new_text: Vec<Arc<str>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UndoOperation {
    pub timestamp: clock::Lamport,
    pub version: clock::Global,
    pub counts: HashMap<clock::Lamport, u32>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Fragment {
    id: Locator,
    timestamp: clock::Lamport,
    insertion_offset: u32,
    len: u32,
    visible: bool,
    deletions: SmallVec<[clock::Lamport; 2]>,
    max_undos: clock::Global,
}
impl sum_tree::Item for Fragment {
    type Summary = FragmentSummary;

    fn summary(&self, _cx: &Option<clock::Global>) -> Self::Summary {
        let mut max_version = clock::Global::new();
        max_version.observe(self.timestamp);
        for deletion in &self.deletions {
            max_version.observe(*deletion);
        }
        max_version.join(&self.max_undos);

        let mut min_insertion_version = clock::Global::new();
        min_insertion_version.observe(self.timestamp);
        let max_insertion_version = min_insertion_version.clone();
        if self.visible {
            FragmentSummary {
                max_id: self.id.clone(),
                text: FragmentTextSummary {
                    visible: self.len as usize,
                    deleted: 0,
                },
                max_version,
                min_insertion_version,
                max_insertion_version,
            }
        } else {
            FragmentSummary {
                max_id: self.id.clone(),
                text: FragmentTextSummary {
                    visible: 0,
                    deleted: self.len as usize,
                },
                max_version,
                min_insertion_version,
                max_insertion_version,
            }
        }
    }
}

impl sum_tree::ContextLessSummary for InsertionFragmentKey {
    fn zero() -> Self {
        InsertionFragmentKey {
            timestamp: Lamport::MIN,
            split_offset: 0,
        }
    }

    fn add_summary(&mut self, summary: &Self) {
        *self = *summary;
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct FragmentSummary {
    text: FragmentTextSummary,
    max_id: Locator,
    max_version: clock::Global,
    min_insertion_version: clock::Global,
    max_insertion_version: clock::Global,
}

#[derive(Copy, Default, Clone, Debug, PartialEq, Eq)]
struct FragmentTextSummary {
    visible: usize,
    deleted: usize,
}

// impl<'a> sum_tree::Dimension<'a, FragmentSummary> for FragmentTextSummary {
//     fn zero(_: &Option<clock::Global>) -> Self {
//         Default::default()
//     }

//     fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<clock::Global>) {
//         self.visible += summary.text.visible;
//         self.deleted += summary.text.deleted;
//     }
// }

#[derive(Eq, PartialEq, Clone, Debug)]
struct InsertionFragment {
    timestamp: clock::Lamport,
    split_offset: u32,
    fragment_id: Locator,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FullOffset(pub usize);

impl ops::AddAssign<usize> for FullOffset {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Edit<D> {
    pub old: Range<D>,
    pub new: Range<D>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct InsertionFragmentKey {
    timestamp: clock::Lamport,
    split_offset: u32,
}

impl sum_tree::Item for InsertionFragment {
    type Summary = InsertionFragmentKey;

    fn summary(&self, _cx: ()) -> Self::Summary {
        InsertionFragmentKey {
            timestamp: self.timestamp,
            split_offset: self.split_offset,
        }
    }
}
