use std::error::Error;

use thiserror::Error;

use crate::core::{
    models::{id::ShortLinkId, url::LongUrl},
    ports::{gen_id::IdGenerator, repository::LinkRepository, service::LinkShortenerService},
};

#[derive(Debug, Clone)]
pub struct LinkShortener<R: LinkRepository, G: IdGenerator> {
    database: R,
    generator: G,
}

impl<R: LinkRepository, G: IdGenerator> LinkShortener<R, G> {
    pub fn new(database: R, generator: G) -> Self {
        Self {
            database,
            generator,
        }
    }
}

impl<R: LinkRepository, G: IdGenerator> LinkShortenerService for LinkShortener<R, G> {
    type Error = LinkShortenerError<R::Error>;

    async fn access(&self, id: ShortLinkId) -> Result<Option<LongUrl>, Self::Error> {
        Ok(self.database.get(&id).await?)
    }

    async fn create(&self, long: LongUrl) -> Result<ShortLinkId, Self::Error> {
        let mut short_id = self.generator.generate().await;
        while let Some(_) = self.database.get(&short_id).await? {
            short_id = self.generator.generate().await;
        }
        self.database.insert(short_id.clone(), long).await?;

        Ok(short_id)
    }
}

#[derive(Debug, Error)]
pub enum LinkShortenerError<RE: Error + Send + Sync + 'static> {
    #[error("Repository Error: '{0}'")]
    RepositoryError(#[from] RE),
}
