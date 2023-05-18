#[cfg(not(target_arch = "wasm32"))]
use openrailwaymap_exporter::app3d::init;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    init();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}
