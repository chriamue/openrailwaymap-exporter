use openrailwaymap_exporter::importer::overpass_importer::{from_railway_elements, RailwayElement};

fn railway_elements() -> Vec<RailwayElement> {
    let test_data = serde_json::from_slice(include_bytes!("../src/tests/res/vilbel.json"))
        .expect("Failed to deserialize the JSON data");
    RailwayElement::from_json(&test_data).unwrap()
}

fn main() {
    let railway_elements = railway_elements();
    let graph = from_railway_elements(&railway_elements);
    println!("{:?}", graph.graph);
}
