use collections::HashMap;
use gpui::{AnyEntity, AnyWeakEntity, AsyncApp, BackgroundExecutor, Entity, FutureExt as _};
use std::{
    any::{Any, TypeId},
    sync::{
        Arc, OnceLock,
        atomic::{self, AtomicU64},
    },
    time::Duration,
};

pub enum EntityMessageSubscriber {
    Entity { handle: AnyWeakEntity },
    // Pending(Vec<Box<dyn AnyTypedEnvelope>>),
}

#[derive(Default)]
pub struct ProtoMessageHandlerSet {
    pub entity_types_by_message_type: HashMap<TypeId, TypeId>,
    pub entities_by_type_and_remote_id: HashMap<(TypeId, u64), EntityMessageSubscriber>,
    // pub entity_id_extractors: HashMap<TypeId, fn(&dyn AnyTypedEnvelope) -> u64>,
    pub entities_by_message_type: HashMap<TypeId, AnyWeakEntity>,
    // pub message_handlers: HashMap<TypeId, ProtoMessageHandler>,
}
