use clock::ReplicaId;

pub type LegacyUserId = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Collaborator {
    // pub peer_id: proto::PeerId,
    pub replica_id: ReplicaId,
    pub user_id: LegacyUserId,
    pub is_host: bool,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
}
