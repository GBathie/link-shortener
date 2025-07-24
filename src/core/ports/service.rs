use crate::core::models::{id::ShortLinkId, url::LongUrl};

pub trait LinkShortenerService: Sync + Send + 'static {
    type Error: std::error::Error + Send + Sync + 'static;
    fn access(
        &self,
        id: ShortLinkId,
    ) -> impl Future<Output = Result<Option<LongUrl>, Self::Error>> + Send;
    fn create(
        &mut self,
        long: LongUrl,
    ) -> impl Future<Output = Result<ShortLinkId, Self::Error>> + Send;
}
