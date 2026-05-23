use smallvec::SmallVec;
use std::iter;

/// An identifier for a position in a ordered collection.
///
/// Allows prepending and appending without needing to renumber existing locators
/// using `Locator::between(lhs, rhs)`.
///
/// The initial location for a collection should be `Locator::between(Locator::min(), Locator::max())`,
/// leaving room for items to be inserted before and after it.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Locator(SmallVec<[u64; 2]>);

impl Clone for Locator {
    fn clone(&self) -> Self {
        // We manually implement clone to avoid the overhead of SmallVec's clone implementation.
        // Using `from_slice` is faster than `clone` for SmallVec as we can use our `Copy` implementation of u64.
        Self {
            0: SmallVec::from_slice(&self.0),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}
impl Locator {
    pub const fn min() -> Self {
        // SAFETY: 1 is <= 2
        Self(unsafe { SmallVec::from_const_with_len_unchecked([u64::MIN; 2], 1) })
    }
    pub const fn max() -> Self {
        // SAFETY: 1 is <= 2
        Self(unsafe { SmallVec::from_const_with_len_unchecked([u64::MAX; 2], 1) })
    }

    pub const fn min_ref() -> &'static Self {
        const { &Self::min() }
    }

    pub const fn max_ref() -> &'static Self {
        const { &Self::max() }
    }
    pub fn assign(&mut self, other: &Self) {
        self.0.resize(other.0.len(), 0);
        self.0.copy_from_slice(&other.0);
    }
}
