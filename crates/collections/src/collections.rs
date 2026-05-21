pub use rustc_hash::{FxHashMap, FxHashSet};
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;
pub type HashMap<K, V> = FxHashMap<K, V>;
pub use std::collections::*;
