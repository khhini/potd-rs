use std::path::PathBuf;

use ::clap::Args;

#[derive(Args, Debug)]
pub struct BingCommand {
    #[arg(
        short,
        long,
        default_value = "https://www.bing.com",
        help = "Bing API endpoint"
    )]
    pub api_endpoint: String,
    #[arg(
        short,
        long,
        default_value = "en-ID",
        help = "Market code (e.g., en-ID, en-US)"
    )]
    pub market: String,

    #[arg(
        short,
        long,
        default_value = "./",
        help = "Directory to save the image"
    )]
    pub save_dir: PathBuf,

    #[arg(
        short,
        long,
        default_value = "bing_potd.jpg",
        help = "Name of the saved image file"
    )]
    pub image_name: String,
}
