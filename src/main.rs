use openrailwaymap_exporter::{
    create_edges, generate_dot_string, BasicOpenRailwayMapApiClient, OpenRailwayMapApiClient,
    RailwayElement, RailwayGraph,
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
        conflicts_with("json"),
        help = "Output in Graphviz dot format"
    )]
    dot: bool,

    #[structopt(
        long = "json",
        short,
        conflicts_with("dot"),
        help = "Output raw JSON data"
    )]
    json: bool,

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

    let api_client: Box<dyn OpenRailwayMapApiClient> =
        Box::new(BasicOpenRailwayMapApiClient::new());

    let api_json_value = if let Some(area) = &opt.area {
        api_client.fetch_by_area_name(area).await?
    } else {
        let bbox = opt.bbox.unwrap();
        api_client.fetch_by_bbox(&bbox).await?
    };

    let railway_elements = RailwayElement::from_json(&api_json_value)?;

    let edges = create_edges(&railway_elements);

    println!("Railway Elements: {:?}", edges.len());

    let graph = RailwayGraph::from_railway_elements(&railway_elements);

    println!("Railway Graph: {:?}", &graph.graph.edge_count());

    if let Some(file_path) = opt.output {
        let mut file = File::create(&file_path)?;
        if opt.dot {
            let dot_string = generate_dot_string(&graph)?;
            writeln!(file, "{}", dot_string)?;
        } else if opt.json {
            writeln!(file, "{}", api_json_value)?;
        }
    }

    Ok(())
}
