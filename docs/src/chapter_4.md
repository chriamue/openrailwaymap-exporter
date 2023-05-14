# The Railway Graph Importer

In this chapter, we will discuss the Railway Graph Importer, a core component of the OpenRailwayMap Exporter that is responsible for importing railway graph data from different formats and sources.

## 3.1 Understanding the Railway Graph Importer

The Railway Graph Importer is a module that defines a trait called `RailwayGraphImporter`. This trait provides a unified interface for importing railway graph data from various sources and formats, such as JSON, XML, or other custom formats. By implementing this trait, developers can create custom importers that seamlessly integrate with the OpenRailwayMap Exporter.

The `RailwayGraphImporter` trait has a single method, `import`, which takes a reference to a `serde_json::Value` and returns a `Result<RailwayGraph>`. The method is responsible for converting the input data into a `RailwayGraph` object that can be used by the rest of the OpenRailwayMap Exporter.

## 3.2 Implementing a Custom Railway Graph Importer

To create a custom railway graph importer, you'll need to implement the `RailwayGraphImporter` trait for your importer struct. This involves providing a definition for the `import` method that takes a `serde_json::Value` as input and returns a `Result<RailwayGraph>`.

Here's a basic outline of how you can implement the `RailwayGraphImporter` trait:

```rust
use crate::railway_model::RailwayGraph;
use crate::importer::RailwayGraphImporter;
use serde_json::Value;
use anyhow::Result;

pub struct MyCustomImporter;

impl RailwayGraphImporter for MyCustomImporter {
    fn import(input: &Value) -> Result<RailwayGraph> {
        // Your custom importer logic goes here.

        // 1. Parse the input data.
        // 2. Create a new RailwayGraph instance.
        // 3. Add nodes and edges to the RailwayGraph.
        // 4. Return the RailwayGraph wrapped in a Result.

        // Example:
        // let graph = RailwayGraph::new();
        // Ok(graph)
    }
}
```

With your custom importer implementation, you can now use it in the OpenRailwayMap Exporter to import railway graph data from your custom format or source.

## 3.3 Using the Overpass Importer

The OpenRailwayMap Exporter comes with a built-in importer called `OverpassImporter` that fetches railway data from the OpenRailwayMap using the Overpass API. This importer is an implementation of the `RailwayGraphImporter` trait and serves as an example of how to create custom importers.

To use the `OverpassImporter`, you can simply create a new instance of the importer and call the `import` method with the desired input data:

```rust
use crate::importer::OverpassImporter;
use serde_json::Value;

// Create a new OverpassImporter instance.
let importer = OverpassImporter::new();

// Deserialize the input data as a serde_json::Value.
let input_data: Value = serde_json::from_str(&input_json)?;

// Import the railway graph using the OverpassImporter.
let railway_graph = importer.import(&input_data)?;
```

In the next chapter, we will explore the 3D visualization capabilities of the OpenRailwayMap Exporter.
