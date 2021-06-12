use std::env;
use std::path::{Path, PathBuf};

use cargo_bpf_lib as cargo_bpf;

fn main() {
    let cargo = PathBuf::from(env::var("CARGO").unwrap());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let probes = Path::new("bpf-progs");
    cargo_bpf::build(&cargo, &probes, &out_dir.join("target"), Vec::new())
        .expect("couldn't compile bpf progs");

    cargo_bpf::probe_files(&probes)
        .expect("couldn't list probe files")
        .iter()
        .for_each(|file| {
            println!("cargo:rerun-if-changed={}", file);
        });
}
