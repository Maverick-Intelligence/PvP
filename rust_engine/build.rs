fn main() {
    cc::Build::new()
        .file("../c_engine/opengl_wrapper_lib/opengl_wrapper.h")
        .include("../c_engine/opengl_wrapper_lib")
        .compile("c_engine");
}
