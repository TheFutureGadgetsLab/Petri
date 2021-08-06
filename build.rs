use std::env;
use std::fs;
use std::path::Path;


fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();

    let mut compiler = shaderc::Compiler::new().unwrap();

    let vs = include_str!("src/rendering/shaders/particles.vert");
    let vs_compiled = compiler.compile_into_spirv(
        vs, shaderc::ShaderKind::Vertex,
        "src/rendering/shaders/particles.vert",
        "main",
        None).unwrap();
    fs::write(
        &Path::new(&manifest_dir).join("src/rendering/shaders/particles.vert.spv"),
        vs_compiled.as_binary_u8(),
    )
    .unwrap();

    let fs = include_str!("src/rendering/shaders/particles.frag");
    let fs_compiled = compiler.compile_into_spirv(
        fs, shaderc::ShaderKind::Fragment,
        "src/rendering/shaders/particles.frag",
        "main",
        None).unwrap();
    fs::write(
        &Path::new(&manifest_dir).join("src/rendering/shaders/particles.frag.spv"),
        fs_compiled.as_binary_u8(),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}