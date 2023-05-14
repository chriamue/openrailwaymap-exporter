# Chapter 6: Railway Model

The Railway Model module is a core component of the OpenRailwayMap Exporter. It provides data structures and functions for working with railway infrastructure data. The main components of this module are the RailwayNode, RailwayEdge, and RailwayGraph structs, as well as a RailwayGraphBuilder for creating RailwayGraphs from raw data.

## RailwayNode

RailwayNode represents a single node in the railway network. A node can represent a railway station, a junction, or any other point of interest within the railway infrastructure. Each node has a unique ID, latitude, and longitude.

## RailwayEdge

RailwayEdge represents a railway segment connecting two nodes in the railway network. Each edge has a unique ID, a length, and a set of attributes such as the track type, maximum speed, or electrification.

## RailwayGraph

RailwayGraph is the main data structure for representing railway networks. It is an undirected graph consisting of RailwayNode instances as nodes and RailwayEdge instances as edges. The graph also stores a HashMap that maps node IDs to their corresponding indices in the graph for easy retrieval.

RailwayGraph provides several methods for working with railway networks, such as:

- Retrieving a node or an edge by its ID
- Retrieving the edge connecting two nodes
- Retrieving all edges connected to a node
- Calculating the bounding box of the graph
- Calculating the total length of the railway network
- Finding the nearest node to a given position on an edge

## RailwayGraphBuilder

The RailwayGraphBuilder is a helper struct for constructing RailwayGraph instances from raw data. It provides methods for adding nodes and edges to the graph and ensures that the graph remains consistent during construction.

# Usage Example

Suppose you have imported railway infrastructure data from OpenStreetMap using the Overpass API. You can create a RailwayGraph instance from this data using the RailwayGraphBuilder, like this:

```rust
let mut builder = RailwayGraphBuilder::new();

// Add nodes and edges from the raw data
for node in raw_nodes {
    builder.add_node(node.id, node.lat, node.lon);
}
for edge in raw_edges {
    builder.add_edge(edge.id, edge.start_node_id, edge.end_node_id, edge.length, edge.attributes);
}

// Build the RailwayGraph
let railway_graph = builder.build();
```

Now you can use the methods provided by the RailwayGraph struct to interact with the railway network. For example, you can find the nearest node to a given position on an edge like this:

```rust
let edge_id = 12345;
let position_on_edge = 0.75;
let current_node_id = Some(67890);

let nearest_node_id = railway_graph.nearest_node(edge_id, position_on_edge, current_node_id);

println!("The nearest node has ID: {}", nearest_node_id.unwrap());
```