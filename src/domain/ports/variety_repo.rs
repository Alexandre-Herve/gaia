use async_trait::async_trait;
use super::super::models::variety::Variety;

#[async_trait]
pub trait VarietyRepo {
    async fn create(variety: &Variety)
}
