fn main() {
    // The gallery's own `.rsx` files (src/**) -> $OUT_DIR/rsx/...
    rsx_build::compile();
    // The registry, compiled the way an app that ran `gpuicn-rsx add` sees it:
    // registry-rsx/ui/*.rsx -> $OUT_DIR/rsx/components/ui/*.rs, which is
    // where `registry-rsx/ui/mod.rs` looks for them.
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("rsx/components");
    rsx_build::Builder::default().source_dir("../../registry-rsx").out_dir(out).compile();
}
