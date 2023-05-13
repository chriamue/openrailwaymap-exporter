use openrailwaymap_exporter::exporter::svg::generate_svg_string;
use openrailwaymap_exporter::prelude::{
    generate_dot_string, OverpassApiClient, OverpassImporter, RailwayApiClient,
    RailwayGraphImporter,
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

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // Check if no parameters are given
    if opt.bbox.is_none() && opt.area.is_none() {
        // Display help message
        Opt::clap().print_help()?;
        println!();
        return Ok(());
    }

    let api_client: Box<dyn RailwayApiClient> = Box::new(OverpassApiClient::new());

    let api_json_value = if let Some(area) = &opt.area {
        api_client.fetch_by_area_name(area).await?
    } else {
        let bbox = opt.bbox.unwrap();
        api_client.fetch_by_bbox(&bbox).await?
    };

    let graph = OverpassImporter::import(&api_json_value).unwrap();

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
