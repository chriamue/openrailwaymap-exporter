use openrailwaymap_exporter::{BasicOpenRailwayMapApiClient, OpenRailwayMapApiClient};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "openrailwaymap_exporter",
    about = "A tool to download and export OpenRailwayMap data."
)]
struct Opt {
    #[structopt(long, conflicts_with("area"))]
    bbox: Option<String>,

    #[structopt(long, conflicts_with("bbox"))]
    area: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let api_client: Box<dyn OpenRailwayMapApiClient> =
        Box::new(BasicOpenRailwayMapApiClient::new());

    let railway_elements = if let Some(area) = &opt.area {
        api_client.fetch_railway_elements_by_area_name(area).await?
    } else {
        let bbox = opt.bbox.unwrap();
        api_client.fetch_railway_elements_by_bbox(&bbox).await?
    };

    println!("Railway Elements: {:?}", railway_elements);

    Ok(())
}
