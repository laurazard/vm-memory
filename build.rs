fn main() {
    #[cfg(feature = "hvf")]
    {
        println!("cargo:rustc-link-lib=framework=Hypervisor");
    }
}
