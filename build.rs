fn main() {
    let path = format!("{}/ui/app.slint", env!("CARGO_MANIFEST_DIR"));
    slint_build::compile(path).unwrap();
}
