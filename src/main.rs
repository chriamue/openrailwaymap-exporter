use openrailwaymap_exporter::{BasicOpenRailwayMapApiClient, OpenRailwayMapApiClient};
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "openrailwaymap_exporter",
    about = "A tool to download and export OpenRailwayMap data."
)]
struct Opt {
    #[structopt(
        short,
        long,
        help = "Bounding box for the data in the format 'left,bottom,right,top'."
    )]
    bbox: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    let client = BasicOpenRailwayMapApiClient::new();
    match client.connect(&opt.bbox).await {
        Ok(data) => {
            println!("{:?}", data);
        }
        Err(e) => {
            eprintln!("Error downloading data: {}", e);
            process::exit(1);
        }
    }
}
