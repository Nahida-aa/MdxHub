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
