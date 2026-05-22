use core::fmt::Debug;
use gpui::{App, Axis, Bounds, DisplayId, Half, Pixels, Point, Position, Size, point, px};
use refineable::Refineable;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Sub, SubAssign};

/// Identifies a reference point on a 2D box, used to anchor positioned elements.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anchor {
    /// The top left corner
    TopLeft,
    /// The top right corner
    TopRight,
    /// The bottom left corner
    BottomLeft,
    /// The bottom right corner
    BottomRight,
    /// The top center position
    TopCenter,
    /// The bottom center position
    BottomCenter,
    /// The left center position
    LeftCenter,
    /// The right center position
    RightCenter,
}

impl Anchor {
    /// Returns the corner across from this corner, moving along the specified axis.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gpui::{Axis, Anchor};
    /// let result = Anchor::TopLeft.other_side_along(Axis::Horizontal);
    /// assert_eq!(result, Anchor::TopRight);
    /// ```
    #[must_use]
    pub fn other_side_along(self, axis: Axis) -> Self {
        match axis {
            Axis::Vertical => match self {
                Anchor::TopLeft => Anchor::BottomLeft,
                Anchor::TopRight => Anchor::BottomRight,
                Anchor::BottomLeft => Anchor::TopLeft,
                Anchor::BottomRight => Anchor::TopRight,
                Anchor::TopCenter => Anchor::BottomCenter,
                Anchor::BottomCenter => Anchor::TopCenter,
                Anchor::LeftCenter => Anchor::LeftCenter,
                Anchor::RightCenter => Anchor::RightCenter,
            },
            Axis::Horizontal => match self {
                Anchor::TopLeft => Anchor::TopRight,
                Anchor::TopRight => Anchor::TopLeft,
                Anchor::BottomLeft => Anchor::BottomRight,
                Anchor::BottomRight => Anchor::BottomLeft,
                Anchor::TopCenter => Anchor::TopCenter,
                Anchor::BottomCenter => Anchor::BottomCenter,
                Anchor::LeftCenter => Anchor::RightCenter,
                Anchor::RightCenter => Anchor::LeftCenter,
            },
        }
    }
}

pub trait BoundsExt<T: Clone + Debug + Default + PartialEq> {
    fn from_anchor_and_size(anchor: Anchor, offset: Point<T>, size: Size<T>) -> Bounds<T>;
}
impl<T> BoundsExt<T> for Bounds<T>
where
    T: Sub<Output = T> + Half + Clone + Debug + Default + PartialEq,
{
    // fn from_anchor_and_size(anchor: Corner, offset: Point<T>, size: Size<T>) -> Bounds<T> {

    /// Constructs a `Bounds` from a corner point and size. The specified corner will be placed at
    /// the specified origin.
    fn from_anchor_and_size(corner: Anchor, origin: Point<T>, size: Size<T>) -> Bounds<T> {
        let origin = match corner {
            Anchor::TopLeft => origin,
            Anchor::TopRight => Point {
                x: origin.x - size.width.clone(),
                y: origin.y,
            },
            Anchor::BottomLeft => Point {
                x: origin.x,
                y: origin.y - size.height.clone(),
            },
            Anchor::BottomRight => Point {
                x: origin.x - size.width.clone(),
                y: origin.y - size.height.clone(),
            },
            Anchor::TopCenter => Point {
                x: origin.x - size.width.half(),
                y: origin.y,
            },
            Anchor::BottomCenter => Point {
                x: origin.x - size.width.half(),
                y: origin.y - size.height.clone(),
            },
            Anchor::LeftCenter => Point {
                x: origin.x,
                y: origin.y - size.height.half(),
            },
            Anchor::RightCenter => Point {
                x: origin.x - size.width.clone(),
                y: origin.y - size.height.half(),
            },
        };

        Bounds { origin, size }
    }
}
