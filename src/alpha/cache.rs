use std::collections::BTreeMap;

use crate::{PayloadRecord, ProbeExecutionRecord};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlphaCacheKey {
    pub artifact_hash: String,
    pub model_id: String,
    pub prompt_id: String,
}

impl AlphaCacheKey {
    pub fn as_string(&self) -> String {
        format!("{}::{}::{}", self.artifact_hash, self.model_id, self.prompt_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CacheStatus {
    Hit,
    Miss,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CachedProbeRecord {
    pub key: AlphaCacheKey,
    pub execution: ProbeExecutionRecord,
    pub payload: PayloadRecord,
    pub values: Vec<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheLookup {
    pub status: CacheStatus,
    pub record: CachedProbeRecord,
}

#[derive(Clone, Debug, Default)]
pub struct AlphaProbeCache {
    records: BTreeMap<AlphaCacheKey, CachedProbeRecord>,
}

impl AlphaProbeCache {
    pub fn get_or_insert_with<F>(&mut self, key: AlphaCacheKey, build: F) -> CacheLookup
    where
        F: FnOnce(&AlphaCacheKey) -> CachedProbeRecord,
    {
        if let Some(record) = self.records.get(&key) {
            return CacheLookup {
                status: CacheStatus::Hit,
                record: record.clone(),
            };
        }

        let record = build(&key);
        self.records.insert(key, record.clone());
        CacheLookup {
            status: CacheStatus::Miss,
            record,
        }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
