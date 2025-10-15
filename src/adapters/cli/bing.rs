use async_trait::async_trait;

use crate::core::{
    entities::bing::BingCommand,
    error::PotdError,
    port::Command,
    usecase::image_downloader::{download_image, extract_image_url, fetch_image_metadata},
};

#[async_trait]
impl Command for BingCommand {
    async fn run(&self) -> Result<(), PotdError> {
        let image_metadata =
            fetch_image_metadata(self.api_endpoint.as_str(), self.market.as_str()).await?;
        let image_url = extract_image_url(self.api_endpoint.as_str(), &image_metadata)?;
        let save_path = self.save_dir.join(self.image_name.as_str());

        download_image(&image_url, &save_path).await?;

        Ok(())
    }

    fn dry_run(&self) -> bool {
        true
    }
}
