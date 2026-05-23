mod anchor;

pub use anchor::*;
pub use patch::Patch;
use postage::{oneshot, prelude::*};
use smallvec::SmallVec;
pub mod locator;
use locator::Locator;
pub use rope::*;
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
use util::debug_panic;
pub mod operation_queue;
mod patch;
pub mod subscription;
mod undo_map;
use clock::{Lamport, ReplicaId};
use collections::{HashMap, HashSet};
use operation_queue::OperationQueue;

use sum_tree::{Bias, Dimensions, SumTree, TreeMap, TreeSet};
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

impl Display for BufferId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
impl BufferSnapshot {
    pub fn as_rope(&self) -> &Rope {
        &self.visible_text
    }
    pub fn len(&self) -> usize {
        self.visible_text.len()
    }

    /// Returns an anchor for the given input position that is anchored to the text after the position.
    pub fn anchor_after<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, position: T, bias: Bias) -> Anchor {
        self.anchor_at_offset(position.to_offset(self), bias)
    }

    fn anchor_at_offset(&self, mut offset: usize, bias: Bias) -> Anchor {
        if bias == Bias::Left && offset == 0 {
            Anchor::min_for_buffer(self.remote_id)
        } else if bias == Bias::Right
            && ((!cfg!(debug_assertions) && offset >= self.len()) || offset == self.len())
        {
            Anchor::max_for_buffer(self.remote_id)
        } else {
            if !self
                .visible_text
                .assert_char_boundary::<{ cfg!(debug_assertions) }>(offset)
            {
                offset = match bias {
                    Bias::Left => self.visible_text.floor_char_boundary(offset),
                    Bias::Right => self.visible_text.ceil_char_boundary(offset),
                };
            }
            let (start, _, item) = self.fragments.find::<usize, _>(&None, &offset, bias);
            let Some(fragment) = item else {
                // We got a bad offset, likely out of bounds
                debug_panic!(
                    "Failed to find fragment at offset {} (len: {})",
                    offset,
                    self.len()
                );
                return Anchor::max_for_buffer(self.remote_id);
            };
            let overshoot = offset - start;
            Anchor::new(
                fragment.timestamp,
                fragment.insertion_offset + overshoot as u32,
                bias,
                self.remote_id,
            )
        }
    }

    /// Returns an anchor for the given input position that is anchored to the text before the position.
    pub fn anchor_before<T: ToOffset>(&self, position: T) -> Anchor {
        self.anchor_at(position, Bias::Left)
    }

    #[cold]
    fn panic_bad_anchor(&self, anchor: &Anchor) -> ! {
        if anchor.buffer_id != self.remote_id {
            panic!(
                "invalid anchor - buffer id does not match: anchor {anchor:?}; buffer id: {}, version: {:?}",
                self.remote_id, self.version
            );
        } else if !self.version.observed(anchor.timestamp()) {
            panic!(
                "invalid anchor - snapshot has not observed lamport: {:?}; version: {:?}",
                anchor, self.version
            );
        } else {
            panic!(
                "invalid anchor {:?}. buffer id: {}, version: {:?}",
                anchor, self.remote_id, self.version
            );
        }
    }
    fn fragment_id_for_anchor(&self, anchor: &Anchor) -> &Locator {
        self.try_fragment_id_for_anchor(anchor)
            .unwrap_or_else(|| self.panic_bad_anchor(anchor))
    }

    fn try_fragment_id_for_anchor(&self, anchor: &Anchor) -> Option<&Locator> {
        if anchor.is_min() {
            Some(Locator::min_ref())
        } else if anchor.is_max() {
            Some(Locator::max_ref())
        } else {
            let item = self.try_find_fragment(anchor);
            item.filter(|insertion| {
                !cfg!(debug_assertions) || insertion.timestamp == anchor.timestamp()
            })
            .map(|insertion| &insertion.fragment_id)
        }
    }

    fn try_find_fragment(&self, anchor: &Anchor) -> Option<&InsertionFragment> {
        let anchor_key = InsertionFragmentKey {
            timestamp: anchor.timestamp(),
            split_offset: anchor.offset,
        };
        match self.insertions.find_with_prev::<InsertionFragmentKey, _>(
            (),
            &anchor_key,
            anchor.bias,
        ) {
            (_, _, Some((prev, insertion))) => {
                let comparison = sum_tree::KeyedItem::key(insertion).cmp(&anchor_key);
                if comparison == Ordering::Greater
                    || (anchor.bias == Bias::Left
                        && comparison == Ordering::Equal
                        && anchor.offset > 0)
                {
                    prev
                } else {
                    Some(insertion)
                }
            }
            _ => self.insertions.last(),
        }
    }

    pub fn text_summary_for_range<D, O: ToOffset>(&self, range: Range<O>) -> D
    where
        D: TextDimension,
    {
        self.visible_text
            .cursor(range.start.to_offset(self))
            .summary(range.end.to_offset(self))
    }
    pub fn summary_for_anchor<D>(&self, anchor: &Anchor) -> D
    where
        D: TextDimension,
    {
        self.text_summary_for_range(0..self.offset_for_anchor(anchor))
    }

    pub fn offset_for_anchor(&self, anchor: &Anchor) -> usize {
        if anchor.is_min() {
            0
        } else if anchor.is_max() {
            self.visible_text.len()
        } else {
            debug_assert_eq!(anchor.buffer_id, self.remote_id);
            debug_assert!(
                self.version.observed(anchor.timestamp()),
                "Anchor timestamp {:?} not observed by buffer {:?}",
                anchor.timestamp(),
                self.version
            );
            let item = self.try_find_fragment(anchor);
            let Some(insertion) =
                item.filter(|insertion| insertion.timestamp == anchor.timestamp())
            else {
                self.panic_bad_anchor(anchor);
            };

            let (start, _, item) = self
                .fragments
                .find::<Dimensions<Option<&Locator>, usize>, _>(
                    &None,
                    &Some(&insertion.fragment_id),
                    Bias::Left,
                );
            let fragment = item.unwrap();
            let mut fragment_offset = start.1;
            if fragment.visible {
                fragment_offset += (anchor.offset - insertion.split_offset) as usize;
            }
            fragment_offset
        }
    }

    pub fn offset_to_point(&self, offset: usize) -> Point {
        self.visible_text.offset_to_point(offset)
    }

    pub fn offset_to_point_utf16(&self, offset: usize) -> PointUtf16 {
        self.visible_text.offset_to_point_utf16(offset)
    }
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

impl From<BufferId> for u64 {
    fn from(id: BufferId) -> Self {
        id.0.get()
    }
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

impl sum_tree::Summary for FragmentSummary {
    type Context<'a> = &'a Option<clock::Global>;

    fn zero(_cx: Self::Context<'_>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, other: &Self, _: Self::Context<'_>) {
        self.max_id.assign(&other.max_id);
        self.text.visible += &other.text.visible;
        self.text.deleted += &other.text.deleted;
        self.max_version.join(&other.max_version);
        self.min_insertion_version
            .meet(&other.min_insertion_version);
        self.max_insertion_version
            .join(&other.max_insertion_version);
    }
}
impl Default for FragmentSummary {
    fn default() -> Self {
        FragmentSummary {
            max_id: Locator::min(),
            text: FragmentTextSummary::default(),
            max_version: clock::Global::new(),
            min_insertion_version: clock::Global::new(),
            max_insertion_version: clock::Global::new(),
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
impl sum_tree::KeyedItem for InsertionFragment {
    type Key = InsertionFragmentKey;

    fn key(&self) -> Self::Key {
        sum_tree::Item::summary(self, ())
    }
}

pub trait ToOffset {
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize;
    /// Turns this point into the next offset in the buffer that comes after this, respecting utf8 boundaries.
    fn to_next_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot
            .visible_text
            .ceil_char_boundary(self.to_offset(snapshot) + 1)
    }
    /// Turns this point into the previous offset in the buffer that comes before this, respecting utf8 boundaries.
    fn to_previous_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot
            .visible_text
            .floor_char_boundary(self.to_offset(snapshot).saturating_sub(1))
    }
}

impl ToOffset for Anchor {
    #[inline]
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize {
        snapshot.summary_for_anchor(self)
    }
}

impl<T: ToOffset> ToOffset for &T {
    #[inline]
    fn to_offset(&self, content: &BufferSnapshot) -> usize {
        (*self).to_offset(content)
    }
}

impl ToOffset for usize {
    #[track_caller]
    fn to_offset(&self, snapshot: &BufferSnapshot) -> usize {
        if !snapshot
            .as_rope()
            .assert_char_boundary::<{ cfg!(debug_assertions) }>(*self)
        {
            snapshot.as_rope().floor_char_boundary(*self)
        } else {
            *self
        }
    }
}

impl sum_tree::Dimension<'_, FragmentSummary> for usize {
    fn zero(_: &Option<clock::Global>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &FragmentSummary, _: &Option<clock::Global>) {
        *self += summary.text.visible;
    }
}

impl<'a> sum_tree::Dimension<'a, FragmentSummary> for Option<&'a Locator> {
    fn zero(_: &Option<clock::Global>) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a FragmentSummary, _: &Option<clock::Global>) {
        *self = Some(&summary.max_id);
    }
}

pub trait ToPoint {
    fn to_point(&self, snapshot: &BufferSnapshot) -> Point;
}

impl ToPoint for usize {
    #[inline]
    fn to_point(&self, snapshot: &BufferSnapshot) -> Point {
        snapshot.offset_to_point(*self)
    }
}

impl ToPointUtf16 for usize {
    #[inline]
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.offset_to_point_utf16(*self)
    }
}

pub trait ToPointUtf16 {
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> PointUtf16;
}

impl ToPointUtf16 for Anchor {
    #[inline]
    fn to_point_utf16(&self, snapshot: &BufferSnapshot) -> PointUtf16 {
        snapshot.summary_for_anchor(self)
    }
}
