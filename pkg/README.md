# OpenRailwayMap Exporter

[![Github Repo](https://img.shields.io/badge/github-repo-green)](https://github.com/chriamue/openrailwaymap-exporter/)
[![Github Pages Build](https://github.com/chriamue/openrailwaymap-exporter/actions/workflows/gh-pages.yml/badge.svg)](https://chriamue.github.io/openrailwaymap-exporter/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/chriamue/openrailwaymap-exporter/branch/main/graph/badge.svg?token=TFJ8UT9W1J)](https://codecov.io/gh/chriamue/openrailwaymap-exporter)
[![Demo](https://img.shields.io/badge/Demo-online-green.svg)](https://chriamue.github.io/openrailwaymap-exporter/)
[![Doc](https://img.shields.io/badge/Docs-online-green.svg)](https://chriamue.github.io/openrailwaymap-exporter/openrailwaymap_exporter/)

A command-line tool built with Rust that fetches railway data from OpenRailwayMap using the Overpass API and exports it into a custom format.

![Overview](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/chriamue/openrailwaymap-exporter/main/overview.puml)

## Features

- Downloads railway data using Overpass API
- Retrieves railway track data including length, GPS path, IDs, connected elements, and switches
- Accepts bounding box parameter to define the area for which data should be downloaded

Data will be downloaded from [overpass-turbo](https://overpass-turbo.eu/s/1ttN).

## Usage

Run the command-line tool using the following command structure:

```sh
cargo run -- --bbox "latitude_min,longitude_min,latitude_max,longitude_max"

```

For example, to download railway data within a bounding box around Frankfurt, use:

```sh
cargo run -- --bbox "49.9,8.4,50.2,8.8"
```

or use the following:

```sh
cargo run -- --area "Frankfurt am Main"
```

To save the elements in a json file, use the following:

```sh
cargo run -- --area "Frankfurt am Main" -j -o output.json
```

To save the graph as svg format use:

```sh
cargo run -- --area "Frankfurt am Main" -d -o output.dot
```

then you can convert the graphviz format into a svg image using the following command:

```sh
cargo run -- --area "Frankfurt am Main" --svg -o output.svg
```

### Web App

1. Compile the code to WASM:

    ```sh
    wasm-pack build --target web
    ```

2. Run the Web version in your browser

    ```sh
    python3 -m http.server
    ```

3. Open your browser on [Localhost](http://localhost:8000)

### 3d App

```sh
cargo run --features app3d --example app3d
```

## Running Tests

To run tests, use the following command:

```sh
cargo test
```

## Running Benchmarks

To run benchmarks, use the following command:

```sh
cargo bench
```

## Profiling with Flamegraph

To create a flamegraph for a specific example, first, make sure you have `cargo-flamegraph` installed:

```sh
cargo install flamegraph
```

Then, run the following command:

```sh
cargo flamegraph --dev --example vilbel
```

This will generate a flamegraph.svg for the vilbel example.


## Python Bindings

This project includes Python bindings using PyO3 and Maturin. To use the Python bindings, follow these steps:

1. Install the bindings in a virtual environment:

    ```sh
    pip install .
    ```

2. Run the test file:

    ```sh
    python tests/pythonlib.py
    ```

### Python API

The Python bindings provide a `PyOverpassImporter` class for importing railway graph data from a JSON string and a `PyRailwayGraph` class for working with the imported railway graph data. Here's a brief overview of the available methods:

#### `PyOverpassImporter`

- `new()`: Create a new PyOverpassImporter instance.
- `import_graph(input: str) -> PyResult<PyRailwayGraph>`: Import railway graph data from a JSON string and return a `PyRailwayGraph` instance containing the imported data.

#### `PyRailwayGraph`

- `node_count() -> usize`: Get the number of nodes in the railway graph.
- `edge_count() -> usize`: Get the number of edges in the railway graph.


## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
