use crate::core::models::id::ShortLinkId;

pub trait IdGenerator: Sync + Send + Clone + 'static {
    fn generate(&self) -> impl Future<Output = ShortLinkId> + Send;
}
