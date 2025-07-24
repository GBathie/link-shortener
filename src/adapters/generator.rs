use crate::core::{models::id::ShortLinkId, ports::gen_id::IdGenerator};
use rand::Rng;
use rand::seq::IndexedRandom;

#[derive(Debug, Clone)]
pub struct RandomGenerator<R: Rng + Clone + Sync + Send + 'static> {
    rng: R,
    len: usize,
}

impl<R: Rng + Clone + Sync + Send + 'static> RandomGenerator<R> {
    pub fn new_with(rng: R, len: usize) -> Self {
        Self { rng, len }
    }
}

/// 26 Lowercase + Uppercase letters + 10 digits: 62 symbols to use.
const LETTERS: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

impl<R: Rng + Clone + Sync + Send + 'static> IdGenerator for RandomGenerator<R> {
    fn generate(&mut self) -> ShortLinkId {
        let id = LETTERS.choose_multiple(&mut self.rng, self.len).collect();
        ShortLinkId::new(id)
    }
}
