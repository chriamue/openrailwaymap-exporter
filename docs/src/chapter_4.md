# Chapter 4: Downloading Railway Graphs from OpenStreetMap

In this chapter, we will discuss how the OpenRailwayMap Exporter can download railway graphs from OpenStreetMap using a client that fetches data through an API.

## 4.1 The Railway API Client

The OpenRailwayMap Exporter includes a module called the Railway API Client, which provides a trait and an implementation for fetching railway infrastructure data from an API. The `RailwayApiClient` trait offers a common asynchronous interface for retrieving data by area name or bounding box.

By implementing the `RailwayApiClient` trait, developers can create custom API clients that can fetch railway graph data from different APIs or sources. The OpenRailwayMap Exporter comes with a built-in API client called the `OverpassApiClient`, which fetches railway data from OpenStreetMap using the Overpass API.

## 4.2 Fetching Railway Data by Area Name or Bounding Box

The `RailwayApiClient` trait has two primary methods for fetching railway data: `fetch_by_area_name` and `fetch_by_bbox`. The `fetch_by_area_name` method allows users to download railway data by specifying the name of an area, while the `fetch_by_bbox` method enables users to provide a bounding box to define the area of interest.

These methods return a JSON `Value` containing the fetched railway data, which can then be passed to a `RailwayGraphImporter` to import the railway graph data into the OpenRailwayMap Exporter.

## 4.3 Using the Overpass API Client

To use the built-in Overpass API client, you can create a new instance of the client and connect it to the OpenRailwayMap API using the desired URL. Once connected, you can fetch railway data by area name or bounding box:

```rust
use crate::api_client::OverpassApiClient;

// Create a new OverpassApiClient instance.
let mut client = OverpassApiClient::new();

// Connect to the OpenRailwayMap API.
client.connect("https://overpass-api.de/api/interpreter").await?;

// Fetch railway data by area name or bounding box.
let area_name = "Frankfurt am Main";
let bbox = "49.9,8.4,50.2,8.8";

let railway_data_by_area = client.fetch_by_area_name(area_name).await?;
let railway_data_by_bbox = client.fetch_by_bbox(bbox).await?;
```

In summary, the Railway API Client is a crucial component of the OpenRailwayMap Exporter, enabling users to fetch railway data from OpenStreetMap and other sources easily. By implementing the `RailwayApiClient` trait, developers can create custom clients that fetch railway data from different APIs or sources, providing flexibility and extensibility to the OpenRailwayMap Exporter.
