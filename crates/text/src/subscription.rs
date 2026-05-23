use crate::{
    // Edit,
    Patch,
};
use parking_lot::Mutex;
use std::{
    mem,
    sync::{Arc, Weak},
};
#[derive(Default)]
pub struct Topic<T>(Mutex<Vec<Weak<Mutex<Patch<T>>>>>);
