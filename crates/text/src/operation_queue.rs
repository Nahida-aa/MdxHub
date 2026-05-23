use clock::Lamport;
use std::{fmt::Debug, ops::Add};
use sum_tree::{ContextLessSummary, Item, SumTree};

pub trait Operation: Clone + Debug {
    fn lamport_timestamp(&self) -> clock::Lamport;
}

#[derive(Clone, Debug)]
struct OperationItem<T>(T);

#[derive(Clone, Debug)]
pub struct OperationQueue<T: Operation>(SumTree<OperationItem<T>>);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OperationKey(clock::Lamport);
impl OperationKey {
    pub fn new(timestamp: clock::Lamport) -> Self {
        Self(timestamp)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationSummary {
    pub key: OperationKey,
    pub len: usize,
}
impl ContextLessSummary for OperationSummary {
    fn zero() -> Self {
        OperationSummary {
            key: OperationKey::new(Lamport::MIN),
            len: 0,
        }
    }

    fn add_summary(&mut self, other: &Self) {
        assert!(self.key < other.key);
        self.key = other.key;
        self.len += other.len;
    }
}

impl<T: Operation> Item for OperationItem<T> {
    type Summary = OperationSummary;

    fn summary(&self, _cx: ()) -> Self::Summary {
        OperationSummary {
            key: OperationKey::new(self.0.lamport_timestamp()),
            len: 1,
        }
    }
}
