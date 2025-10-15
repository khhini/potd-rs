use std::path::{Path, PathBuf};

use clap::Args;
use regex::Regex;

use async_trait::async_trait;
use tokio::{fs::File, io::copy};

use crate::{command::Command, error::PotdError};

#[derive(Args, Debug)]
pub struct BingCommand {
    #[arg(
        short,
        long,
        default_value = "https://www.bing.com",
        help = "Bing API endpoint"
    )]
    api_endpoint: String,
    #[arg(
        short,
        long,
        default_value = "en-ID",
        help = "Market code (e.g., en-ID, en-US)"
    )]
    market: String,

    #[arg(
        short,
        long,
        default_value = "./",
        help = "Directory to save the image"
    )]
    save_dir: PathBuf,

    #[arg(
        short,
        long,
        default_value = "bing_potd.jpg",
        help = "Name of the saved image file"
    )]
    image_name: String,
}

impl BingCommand {
    async fn fetch_image_metadata(&self) -> Result<String, PotdError> {
        let api_url = format!(
            "{}/HPImageArchive.aspx?format=xml&idx=0&n=1&mkt={}",
            self.api_endpoint, self.market
        );

        let response = reqwest::get(api_url).await?.text().await?;
        Ok(response)
    }

    fn extract_image_url(&self, xml: &str) -> Result<String, PotdError> {
        let regex_pattern = Regex::new(r"<url>(?<url>.*)<\/url>").unwrap();

        match regex_pattern.captures(xml) {
            Some(caps) => Ok(format!("{}{}", self.api_endpoint, &caps["url"])),
            None => Err(PotdError::ImageUrlNotFound(xml.to_string())),
        }
    }

    async fn download_image(&self, url: &str, save_path: &Path) -> Result<(), PotdError> {
        let response = reqwest::get(url).await?.bytes().await?;

        let mut dest = File::create(save_path).await?;

        copy(&mut response.as_ref(), &mut dest).await?;
        Ok(())
    }
}

#[async_trait]
impl Command for BingCommand {
    async fn run(&self) -> Result<(), PotdError> {
        let image_metadata = self.fetch_image_metadata().await?;
        let image_url = self.extract_image_url(&image_metadata)?;
        let save_path = self.save_dir.join(self.image_name.as_str());

        self.download_image(&image_url, &save_path).await?;

        Ok(())
    }

    fn dry_run(&self) -> bool {
        true
    }
}
