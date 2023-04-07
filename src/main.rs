use openrailwaymap_exporter::{
    create_edges, generate_dot_string, generate_json_string, BasicOpenRailwayMapApiClient,
    OpenRailwayMapApiClient, RailwayGraph,
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

    let railway_elements = if let Some(area) = &opt.area {
        api_client.fetch_railway_elements_by_area_name(area).await?
    } else {
        let bbox = opt.bbox.unwrap();
        api_client.fetch_railway_elements_by_bbox(&bbox).await?
    };

    let edges = create_edges(&railway_elements);

    println!("Railway Elements: {:?}", edges);

    let graph = RailwayGraph::from_railway_elements(&railway_elements);

    println!("Railway Graph: {:?}", &graph);

    if let Some(file_path) = opt.output {
        let mut file = File::create(&file_path)?;
        if opt.dot {
            let dot_string = generate_dot_string(&graph)?;
            writeln!(file, "{}", dot_string)?;
        } else if opt.json {
            let json_string = generate_json_string(&railway_elements)?;
            writeln!(file, "{}", json_string)?;
        }
    }

    Ok(())
}
