# Chapter 5: Python Bindings for OpenRailwayMap Exporter

In this chapter, we will discuss how to use the Python bindings for the OpenRailwayMap Exporter, allowing users to interact with railway graphs and import them using Python.

## 5.1 Python Wrapper for OverpassImporter

The OpenRailwayMap Exporter provides a Python wrapper for the `OverpassImporter` struct called `PyOverpassImporter`. This wrapper enables users to import railway graph data from a JSON string in Python. To create a new `PyOverpassImporter` instance, simply call its constructor:

```python
import openrailwaymap_exporter

# Create a new PyOverpassImporter instance.
overpass_importer = openrailwaymap_exporter.PyOverpassImporter()
```

To import a railway graph from a JSON string, use the `import_graph` method:

```python
input_json = "..."
railway_graph = overpass_importer.import_graph(input_json)
```

## 5.2 Python Wrapper for RailwayGraph

The OpenRailwayMap Exporter also provides a Python wrapper for the `RailwayGraph` struct, called `PyRailwayGraph`. This wrapper allows users to interact with railway graphs in Python, providing access to the following methods:

- `node_count`: Get the number of nodes in the railway graph.
- `edge_count`: Get the number of edges in the railway graph.
- `get_node_by_id`: Get a node by its ID from the railway graph.
- `get_edge_by_id`: Get an edge by its ID from the railway graph.

```python
# Get the number of nodes and edges in the railway graph.
node_count = railway_graph.node_count()
edge_count = railway_graph.edge_count()

# Get a node and an edge by their IDs.
node = railway_graph.get_node_by_id(node_id)
edge = railway_graph.get_edge_by_id(edge_id)
```

## 5.3 Exporting Railway Graphs to SVG

The Python bindings also provide a function, `export_svg`, that generates an SVG representation of a given `PyRailwayGraph`:

```python
import openrailwaymap_exporter

# Export the railway graph to an SVG string.
svg_string = openrailwaymap_exporter.export_svg(railway_graph)
```

## 5.4 Summary

In this chapter, we have seen how the Python bindings for the OpenRailwayMap Exporter make it easy for users to interact with railway graphs and import them using Python. By providing Python wrappers for the `OverpassImporter` and `RailwayGraph` structs, along with an SVG export function, the OpenRailwayMap Exporter can be used in various Python applications, opening up new possibilities for developers and users alike.
