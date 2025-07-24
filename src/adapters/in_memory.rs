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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InMemoryRepoError {}

#[cfg(test)]
mod test {
    use crate::{
        adapters::in_memory::InMemoryRepository,
        core::{
            models::{id::ShortLinkId, url::LongUrl},
            ports::repository::LinkRepository,
        },
    };

    #[tokio::test]
    async fn insert_and_get() {
        let repo = InMemoryRepository::default();
        let id = ShortLinkId::new("test_id".into());
        let url = LongUrl::new("http://example.com".into());

        let res = repo.insert(id.clone(), url.clone()).await;
        assert_eq!(res, Ok(None));

        let get_res = repo.get(&id).await;
        assert_eq!(get_res, Ok(Some(url)));
    }
}
