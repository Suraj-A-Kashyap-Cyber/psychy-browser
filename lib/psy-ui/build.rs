use std::{env, path::PathBuf, process::Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir).join("resources.gresource");

    Command::new("glib-compile-resources")
        .args(&[
            "--target-directory", &out_dir,
            "--sourcedir", "../dist/assets",
            "../dist/assets/resources.xml",
        ])
        .status()
        .expect("glib-compile-resources failed");

    println!("cargo:rerun-if-changed=../dist/assets/resources.xml");
}
