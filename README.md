# OpenRailwayMap Exporter

A command-line tool built with Rust that fetches railway data from OpenRailwayMap using the Overpass API and exports it into a custom format.

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
cargo run -- --bbox "Frankfurt am Main"
```

To save the elements in a json file, use the following:

```sh
cargo run -- --area "Frankfurt am Main" -j -o output.json
```

To save the graph as graphviz format use:

```sh
cargo run -- --area "Frankfurt am Main" -d -o output.dot
```

then you can convert the graphviz format into a svg image using the following command:

```sh
dot -Tsvg output.dot > output.svg
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
