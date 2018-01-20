extern crate cc;

fn main() {
    #[cfg(feature = "border")] {
        cc::Build::new()
            .cpp(true)
            .warnings(false)
            .flag("-objc")
            .flag("-framework")
            .flag("Cocoa")
            .file("./chunkwm-lib/border/border.mm")
            .compile("border");
    }

    #[cfg(feature = "accessibility")] {
        cc::Build::new()
            .cpp(true)
            .warnings(false)
            .flag("-framework")
            .flag("Carbon")
            .file("./chunkwm-lib/accessibility/element.cpp")
            .compile("accessibility");
    }
}
