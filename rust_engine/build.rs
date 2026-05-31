use std::path::Path;

fn main() {
    let lib_dir = Path::new("../c_engine/opengl_wrapper_lib");
    let glfw = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("glfw3")
        .expect("GLFW dev files not found — install libglfw3-dev (provides glfw3.pc)");
    let gl = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("gl")
        .expect("OpenGL dev files not found — install libgl-dev/mesa-common-dev (provides gl.pc)");
    let mut build = cc::Build::new();

    build
        .file(lib_dir.join("opengl_wrapper_lib.c"))
        .include(lib_dir);

    for path in glfw.include_paths.iter().chain(gl.include_paths.iter()) {
        build.include(path);
    }

    build.compile("c_engine");
    pkg_config::probe_library("glfw3").unwrap();
    pkg_config::probe_library("gl").unwrap();
}
