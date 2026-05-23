/// Indicate whether a [`Buffer`] has permissions to edit.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Capability {
    /// The buffer is a mutable replica.
    ReadWrite,
    /// The buffer is a mutable replica, but toggled to be only readable.
    Read,
    /// The buffer is a read-only replica.
    ReadOnly,
}
impl Capability {
    /// Returns `true` if the capability is `ReadWrite`.
    pub fn editable(self) -> bool {
        matches!(self, Capability::ReadWrite)
    }
}
