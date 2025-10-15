use async_trait::async_trait;

use crate::core::error::PotdError;
#[async_trait]
pub trait Command {
    async fn run(&self) -> Result<(), PotdError>;
    fn dry_run(&self) -> bool;
}
