# Getting Started

This chapter covers the basics of using the OpenRailwayMap Exporter. It will guide you through the installation process and demonstrate how to run the tool to retrieve railway data. Additionally, you will learn about the available command-line options and how to use them effectively.

## 1.1 Installation

To install OpenRailwayMap Exporter, you need to have Rust and Cargo installed on your system. If you haven't installed them yet, visit the [official Rust website](https://www.rust-lang.org/tools/install) and follow the installation instructions for your operating system.

Once you have Rust and Cargo installed, you can clone the OpenRailwayMap Exporter repository from GitHub:

```sh
git clone https://github.com/chriamue/openrailwaymap-exporter.git
```

Navigate to the repository folder and build the project:

```sh
cd openrailwaymap-exporter
cargo build --release
```

After the build process is complete, you will find the compiled binary in the `target/release` folder.

## 1.2 Basic Usage

Now that you have successfully built the OpenRailwayMap Exporter, you can start using it to download railway data.

To download railway data within a bounding box around a specific location, run the following command:

```sh
cargo run -- --bbox "latitude_min,longitude_min,latitude_max,longitude_max"
```

Alternatively, you can download railway data for a specific area by running:

```sh
cargo run -- --area "Area Name"
```

Refer to the main documentation for more examples and detailed usage instructions.

## 1.3 Command-Line Options

OpenRailwayMap Exporter offers several command-line options that allow you to customize the output format and destination, as well as enable additional features. For a comprehensive list of command-line options and their usage, refer to the main documentation.

Some of the available command-line options include:

* `-j`: Save the downloaded elements in a JSON file.
* `-d`: Save the railway graph in Graphviz format.
* `--svg`: Save the railway graph as an SVG image.
* `-o`: Specify the output file name.

For more information on using these options and others, consult the main documentation.

In the next chapter, we will explore the web application.
