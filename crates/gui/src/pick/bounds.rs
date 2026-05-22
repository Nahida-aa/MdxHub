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
