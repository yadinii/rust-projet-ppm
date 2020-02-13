extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/lib_c/ppma_io.c")
        .compile("libppm.a");
}