extern crate cc;

fn main() {
    #[cfg(feature = "border")] {
        cc::Build::new()
            .cpp(true)
            .warnings(false)
            .flag("-objc")
            .flag("-Wno-shadow-ivar")
            .file("./chunkwm-lib/border/border.mm")
            .compile("border");
    }

    #[cfg(feature = "accessibility")] {
        cc::Build::new()
            .cpp(true)
            .warnings(false)
            .flag("-objc")
            .flag("-Wno-deprecated-declarations")
            .file("./chunkwm-lib/accessibility/element.cpp")
            .file("./chunkwm-lib/accessibility/application.mm")
            .file("./chunkwm-lib/accessibility/window.cpp")
            .compile("accessibility");
    }
}
