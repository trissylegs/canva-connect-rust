extern crate canva_connect;
extern crate tokio;

include!(concat!(env!("OUT_DIR"), "/skeptic-tests.rs"));

fn main() {
    // This test is disabled due to dependency resolution issues with skeptic
    println!("Skeptic tests are disabled due to dependency resolution issues");
}
