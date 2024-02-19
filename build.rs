// build.rs

fn main() {
    // Run the cargo build command with the desired options
    // println!("cargo:rerun-if-changed=src"); // Optional: Trigger a rebuild if anything in the src directory changes
    // println!("cargo:rerun-if-changed=Cargo.toml"); // Optional: Trigger a rebuild if the Cargo.toml file changes

    let status = std::process::Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--release")
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        panic!("Cargo build failed");
    }
}
