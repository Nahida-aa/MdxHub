use std::{
    borrow::Cow,
    cmp::{self, Ordering},
    ops::{Range, RangeInclusive},
};

pub mod paths;
pub mod rel_path;

pub trait RangeExt<T> {
    fn sorted(&self) -> Self;
    fn to_inclusive(&self) -> RangeInclusive<T>;
    fn overlaps(&self, other: &Range<T>) -> bool;
    fn contains_inclusive(&self, other: &Range<T>) -> bool;
}
