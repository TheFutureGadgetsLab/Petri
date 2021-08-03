use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;


fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let vs = include_str!("src/shaders/particles.vert");
    let mut vs_compiled = glsl_to_spirv::compile(vs, glsl_to_spirv::ShaderType::Vertex).unwrap();
    let mut vs_bytes = Vec::new();
    vs_compiled.read_to_end(&mut vs_bytes).unwrap();
    fs::write(
        &Path::new(&manifest_dir).join("src/shaders/particles.vert.spv"),
        vs_bytes,
    )
    .unwrap();

    let fs = include_str!("src/shaders/particles.frag");
    let mut fs_compiled = glsl_to_spirv::compile(fs, glsl_to_spirv::ShaderType::Fragment).unwrap();
    let mut fs_bytes = Vec::new();
    fs_compiled.read_to_end(&mut fs_bytes).unwrap();
    fs::write(
        &Path::new(&manifest_dir).join("src/shaders/particles.frag.spv"),
        fs_bytes,
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}