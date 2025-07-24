use crate::core::models::{id::ShortLinkId, url::LongUrl};

pub trait LinkRepository: Sync + Send + Clone + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    fn get(
        &self,
        id: &ShortLinkId,
    ) -> impl Future<Output = Result<Option<LongUrl>, Self::Error>> + Send;
    fn insert(
        &self,
        id: ShortLinkId,
        long: LongUrl,
    ) -> impl Future<Output = Result<Option<LongUrl>, Self::Error>> + Send;
}
