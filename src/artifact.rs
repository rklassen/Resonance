use crate::ids::{ArtifactId, HashDigest, ValueRef};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetadataEntry {
    pub key: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRecord {
    pub id: ArtifactId,
    pub hash: HashDigest,
    pub source: ValueRef,
    /// Metadata entries are normalized before being stored on the record.
    pub metadata: Vec<MetadataEntry>,
}
