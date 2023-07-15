extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        println!("Running task from `winres` to set icon and manifest.");
        let mut res = winres::WindowsResource::new();
        res.set_icon("./assets/icon.ico");
        res.compile().unwrap();

        println!("Running `static_vcruntime::metabuild()`.");
        static_vcruntime::metabuild();
    }
}
