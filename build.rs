extern crate cc;

fn main() {
    #[cfg(feature = "border")]
    {
        cc::Build::new()
            .warnings(false)
            .flag("-objc")
            .flag("-w")
            .file("./chunkwm-lib/border/border.mm")
            .compile("border");
    }

    #[cfg(feature = "accessibility")]
    {
        cc::Build::new()
            .warnings(false)
            .flag("-objc")
            .flag("-w")
            .file("./chunkwm-lib/accessibility/application.mm")
            .file("./chunkwm-lib/accessibility/display.mm")
            .file("./chunkwm-lib/accessibility/element.cpp")
            .file("./chunkwm-lib/accessibility/window.cpp")
            .compile("accessibility");
    }
}
