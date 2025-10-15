use std::path::Path;

use regex::Regex;
use tokio::{fs::File, io::copy};

use crate::core::error::PotdError;

pub async fn fetch_image_metadata(api_endpoint: &str, market: &str) -> Result<String, PotdError> {
    let api_url = format!(
        "{}/HPImageArchive.aspx?format=xml&idx=0&n=1&mkt={}",
        api_endpoint, market
    );

    let response = reqwest::get(api_url).await?.text().await?;
    Ok(response)
}

pub fn extract_image_url(api_endpoint: &str, xml: &str) -> Result<String, PotdError> {
    let regex_pattern = Regex::new(r"<url>(?<url>.*)<\/url>").unwrap();

    match regex_pattern.captures(xml) {
        Some(caps) => Ok(format!("{}{}", api_endpoint, &caps["url"])),
        None => Err(PotdError::ImageUrlNotFound(xml.to_string())),
    }
}

pub async fn download_image(url: &str, save_path: &Path) -> Result<(), PotdError> {
    let response = reqwest::get(url).await?.bytes().await?;

    let mut dest = File::create(save_path).await?;

    copy(&mut response.as_ref(), &mut dest).await?;
    Ok(())
}
