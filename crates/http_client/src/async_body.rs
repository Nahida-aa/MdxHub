use std::{
    io::{Cursor, Read},
    pin::Pin,
    task::Poll,
};

use bytes::Bytes;
use futures::AsyncRead;
/// Based on the implementation of AsyncBody in
/// <https://github.com/sagebind/isahc/blob/5c533f1ef4d6bdf1fd291b5103c22110f41d0bf0/src/body/mod.rs>.
pub struct AsyncBody(pub Inner);

pub enum Inner {
    /// An empty body.
    Empty,

    /// A body stored in memory.
    Bytes(std::io::Cursor<Bytes>),

    /// An asynchronous reader.
    AsyncReader(Pin<Box<dyn futures::AsyncRead + Send + Sync>>),
}

impl AsyncBody {
    /// Create a new empty body.
    ///
    /// An empty body represents the *absence* of a body, which is semantically
    /// different than the presence of a body of zero length.
    pub fn empty() -> Self {
        Self(Inner::Empty)
    }
    /// Create a streaming body that reads from the given reader.
    pub fn from_reader<R>(read: R) -> Self
    where
        R: AsyncRead + Send + Sync + 'static,
    {
        Self(Inner::AsyncReader(Box::pin(read)))
    }

    pub fn from_bytes(bytes: Bytes) -> Self {
        Self(Inner::Bytes(Cursor::new(bytes)))
    }
}

impl Default for AsyncBody {
    fn default() -> Self {
        Self(Inner::Empty)
    }
}
