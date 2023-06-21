fn main() {
    if cfg!(target_os = "windows") {
        println!("Running `static_vcruntime::metabuild()`");
        static_vcruntime::metabuild();
    }
}
