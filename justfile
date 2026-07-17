# List available recipes
default:
    @just --list

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Export railway data for a bounding box, e.g. just bbox "49.9,8.4,50.2,8.8"
bbox coords:
    cargo run -- --bbox "{{coords}}"

# Export railway data for a named area, e.g. just area "Frankfurt am Main"
area name:
    cargo run -- --area "{{name}}"

# Export railway data for an area as JSON
export-json name output="output.json":
    cargo run -- --area "{{name}}" -j -o {{output}}

# Export railway graph for an area as graphviz dot
export-dot name output="output.dot":
    cargo run -- --area "{{name}}" -d -o {{output}}

# Export railway graph for an area as SVG
export-svg name output="output.svg":
    cargo run -- --area "{{name}}" --svg -o {{output}}

# Run tests
test:
    cargo test

# Run BDD (cucumber) tests
test-bdd:
    cargo test --test bdd_tests --features cucumber

# Build docs
doc:
    cargo doc --no-deps

# Run the 3d app
app3d:
    cargo run --features app3d --example app3d

# Run the vilbel example
vilbel:
    cargo run --example vilbel

# Build WASM package for the web app
wasm:
    wasm-pack build --target web

# Build WASM and serve the web app on http://localhost:8000
serve: wasm
    python3 -m http.server

# Generate a flamegraph for the vilbel example
flamegraph:
    cargo flamegraph --dev --example vilbel
