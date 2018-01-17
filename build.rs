extern crate cc;

fn main() {
    #[cfg(feature = "border")] {
        cc::Build::new()
            .cpp(true)
            .warnings(false)
            .flag("-objc")
            .flag("-framework")
            .flag("Cocoa")
            .include("./chunkwm/src/common/border/")
            .file("./chunkwm/src/common/border/border.mm")
            .compile("border");
    }
}
