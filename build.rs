use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn resolve_include(
    requested_file: &str,
    _rtype: shaderc::IncludeType,
    _source_file: &str,
    _depth: usize,
) -> shaderc::IncludeCallbackResult {
    let p = Path::new("src/shaders/").join(requested_file);
    if let Ok(mut f) = File::open(p) {
        let mut inc_source = String::new();
        f.read_to_string(&mut inc_source).unwrap();
        Ok(shaderc::ResolvedInclude {
            resolved_name: "src/shaders/".to_string() + requested_file,
            content: inc_source,
        })
    } else {
        // TODO: Maybe give the reason?
        Err("Could not open file".to_string())
    }
}

fn main() -> std::io::Result<()> {
    let compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_source_language(shaderc::SourceLanguage::HLSL);
    options.set_include_callback(resolve_include);

    macro_rules! compile_shader {
        ($path:expr) => {
            println!(concat!(concat!("cargo:rerun-if-changed=", $path), ".hlsl"));
            let mut source = String::new();
            let mut f = File::open(concat!($path, ".hlsl"))?;
            f.read_to_string(&mut source)?;
            let spv_vs = compiler
                .compile_into_spirv(
                    &source,
                    shaderc::ShaderKind::Vertex,
                    concat!($path, ".hlsl"),
                    "vs_main",
                    Some(&options),
                )
                .expect("VS compilation failed");
            let spv_fs = compiler
                .compile_into_spirv(
                    &source,
                    shaderc::ShaderKind::Fragment,
                    concat!($path, ".hlsl"),
                    "fs_main",
                    Some(&options),
                )
                .expect("FS compilation failed");

            println!("Successfully compiled shader {}", $path);

            File::create(concat!($path, ".vs.spv"))?.write_all(spv_vs.as_binary_u8())?;
            File::create(concat!($path, ".fs.spv"))?.write_all(spv_fs.as_binary_u8())?;
        };
    }

    compile_shader!("src/shaders/chunk");
    compile_shader!("src/shaders/debug_lines");
    compile_shader!("src/shaders/debug_cube");
    compile_shader!("src/shaders/post/fxaa");

    let path = Path::new(&std::env::var("OUT_DIR").unwrap()).join("build_info.rs");
    let mut output_file = File::create(&path).expect("Failed to create `build_info.rs`");

    output_file
        .write_fmt(format_args!(
            "pub const RUSTC_VERSION: &'static str = \"{}\";\n",
            rustc_version::version().unwrap().to_string()
        ))
        .expect("Failed to write RUST_VERSION constant");

    output_file
        .write_fmt(format_args!(
            "pub const CRATE_VERSION: &'static str = \"{}\";\n",
            version::version!()
        ))
        .expect("Failed to write RUST_VERSION constant");

    Ok(())
}
