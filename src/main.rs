use openrailwaymap_exporter::prelude::overpass_api_client::RailwayElement;
use openrailwaymap_exporter::prelude::{
    from_railway_elements, generate_dot_string, generate_svg_string, OverpassApiClient,
    RailwayApiClient,
};
use std::fs::File;
use std::io::Write;
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

    #[structopt(
        long = "dot",
        short,
        conflicts_with("json,svg"),
        help = "Output in Graphviz dot format"
    )]
    dot: bool,

    #[structopt(
        long = "json",
        short,
        conflicts_with("dot,svg"),
        help = "Output raw JSON data"
    )]
    json: bool,

    #[structopt(long = "svg", conflicts_with("dot,json"), help = "Output svg image")]
    svg: bool,

    #[structopt(
        long = "output",
        short = "o",
        name = "filename",
        help = "Output filename"
    )]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let api_client: Box<dyn RailwayApiClient> = Box::new(OverpassApiClient::new());

    let api_json_value = if let Some(area) = &opt.area {
        api_client.fetch_by_area_name(area).await?
    } else {
        let bbox = opt.bbox.unwrap();
        api_client.fetch_by_bbox(&bbox).await?
    };

    let railway_elements = RailwayElement::from_json(&api_json_value)?;

    let graph = from_railway_elements(&railway_elements);

    println!("Railway Graph: {:?}", &graph.graph.edge_count());

    if let Some(file_path) = opt.output {
        let mut file = File::create(file_path)?;
        if opt.dot {
            let dot_string = generate_dot_string(&graph)?;
            writeln!(file, "{}", dot_string)?;
        } else if opt.json {
            writeln!(file, "{}", api_json_value)?;
        } else if opt.svg {
            let svg_string = generate_svg_string(&graph)?;
            writeln!(file, "{}", svg_string)?;
        }
    }

    Ok(())
}
