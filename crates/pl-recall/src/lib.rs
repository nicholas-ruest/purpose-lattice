use pl_errors::Result;
use pl_ruvector::PurposeIndex;
use pl_types::Analogue;
pub fn recall(index: &PurposeIndex, embedding: &[f32]) -> Result<Vec<Analogue>> {
    index.search(embedding, 3)
}
