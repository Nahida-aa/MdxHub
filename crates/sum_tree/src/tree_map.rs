use std::{cmp::Ordering, fmt::Debug};

use crate::{
    Bias,
    ContextLessSummary,
    Dimension,
    // Edit,
    Item,
    KeyedItem,
    SeekTarget,
    SumTree,
};

/// A cheaply-cloneable ordered map based on a [SumTree](crate::SumTree).
#[derive(Clone, PartialEq, Eq)]
pub struct TreeMap<K, V>(SumTree<MapEntry<K, V>>)
where
    K: Clone + Ord,
    V: Clone;
impl<K: Clone + Ord, V: Clone> TreeMap<K, V> {
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
        self.0.iter().map(|entry| (&entry.key, &entry.value))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapEntry<K, V> {
    key: K,
    value: V,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapKey<K>(Option<K>);

impl<K> Default for MapKey<K> {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeSet<K>(TreeMap<K, ()>)
where
    K: Clone + Ord;

impl<K, V> Item for MapEntry<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    type Summary = MapKey<K>;

    fn summary(&self, _cx: ()) -> Self::Summary {
        self.key()
    }
}

impl<K, V> KeyedItem for MapEntry<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    type Key = MapKey<K>;

    fn key(&self) -> Self::Key {
        MapKey(Some(self.key.clone()))
    }
}

impl<K> ContextLessSummary for MapKey<K>
where
    K: Clone,
{
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        *self = summary.clone()
    }
}

impl<K, V> Debug for TreeMap<K, V>
where
    K: Clone + Debug + Ord,
    V: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
