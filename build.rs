extern crate cc;

fn main() {

    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cpp")
        .compile("wrapper");

}
