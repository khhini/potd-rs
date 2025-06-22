use std::path::{Path, PathBuf};

use clap::Parser;
use regex::Regex;
use thiserror::Error;
use tokio::{fs::File, io::copy};

#[derive(Error, Debug)]
enum PotdError {
    #[error("Failed to parse Bing API response: Image URL not found")]
    ImageUrlNotFound,
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Environment variable error: {0}")]
    Var(#[from] std::env::VarError),
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short,
        long,
        default_value = "en-ID",
        help = "Market code (e.g., en-ID, en-US)"
    )]
    market: String,

    #[clap(
        short,
        long,
        default_value = "./",
        help = "Directory to save the image"
    )]
    save_dir: PathBuf,

    #[clap(
        short,
        long,
        default_value = "bing_potd.jpg",
        help = "Name of the saved image file"
    )]
    image_name: String,
}

const BING_API_BASE_URL: &str = "https://www.bing.com";

#[tokio::main]
async fn main() -> Result<(), PotdError> {
    let args = Args::parse();
    let mkt = args.market;
    let save_dir = args.save_dir;
    let image_name = args.image_name;

    let client = reqwest::Client::new();

    // Fetch image metadata
    let image_metadata = fetch_image_metadata(&client, mkt.as_str()).await?;

    // Extract image URL
    let image_url = extract_image_url(&image_metadata)?;

    // Determine save path
    let save_path = save_dir.join(image_name);

    // Download and save the image
    download_image(&client, image_url.as_str(), &save_path).await?;

    println!("Successfully downloaded image to: {}", save_path.display());

    Ok(())
}

async fn fetch_image_metadata(client: &reqwest::Client, mkt: &str) -> Result<String, PotdError> {
    let api_url = format!(
        "{}/HPImageArchive.aspx?format=xml&idx=0&n=1&mkt={}",
        BING_API_BASE_URL, mkt
    );

    let response = client.get(api_url).send().await?;
    Ok(response.text().await?)
}

fn extract_image_url(xml: &str) -> Result<String, PotdError> {
    let regex_pattern = Regex::new(r"<url>(?<url>.*)<\/url>").unwrap();

    match regex_pattern.captures(xml) {
        Some(caps) => Ok(format!("{}{}", BING_API_BASE_URL, &caps["url"])),
        None => Err(PotdError::ImageUrlNotFound),
    }
}

async fn download_image(
    client: &reqwest::Client,
    url: &str,
    save_path: &Path,
) -> Result<(), PotdError> {
    let response = client.get(url).send().await?;

    let content = response.bytes().await?;
    let mut dest = File::create(save_path).await?;

    copy(&mut content.as_ref(), &mut dest).await?;
    Ok(())
}
