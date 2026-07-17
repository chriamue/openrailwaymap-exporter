#[cfg(not(target_arch = "wasm32"))]
use openrailwaymap_exporter::app3d::init;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    init();
}

#[cfg(target_arch = "wasm32")]
fn main() {}
