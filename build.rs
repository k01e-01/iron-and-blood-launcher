use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let res_dir = "assets"; // Directory containing your gresource XML file
    let xml_file = format!("{}/resources.gresource.xml", res_dir);
    let gresource_file = format!("{}/resources.gresource", out_dir);

    // Run the `glib-compile-resources` command
    Command::new("glib-compile-resources")
        .args(&["--target", &gresource_file, &xml_file])
        .status()
        .expect("Failed to compile resources");

    // Tell Cargo to rerun this script if the resource file changes
    println!("cargo:rerun-if-changed={}", xml_file);
}

