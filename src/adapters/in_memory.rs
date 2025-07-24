use std::{collections::HashMap, sync::Arc};

use thiserror::Error;
use tokio::sync::RwLock;

use crate::core::{
    models::{id::ShortLinkId, url::LongUrl},
    ports::repository::LinkRepository,
};

#[derive(Debug, Clone, Default)]
pub struct InMemoryRepository {
    data: Arc<RwLock<HashMap<ShortLinkId, LongUrl>>>,
}

impl LinkRepository for InMemoryRepository {
    type Error = InMemoryRepoError;

    async fn get(&self, id: &ShortLinkId) -> Result<Option<LongUrl>, Self::Error> {
        Ok(self.data.read().await.get(id).cloned())
    }

    async fn insert(&self, id: ShortLinkId, long: LongUrl) -> Result<Option<LongUrl>, Self::Error> {
        Ok(self.data.write().await.insert(id, long))
    }
}

#[derive(Debug, Error)]
pub enum InMemoryRepoError {}
