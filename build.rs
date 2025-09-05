use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    
    // Only compile and link assembly for BPF targets
    if target.contains("sbf") || target.contains("solana") {
        cc::Build::new()
            .file("assert_pubkey_eq.s")
            .flag("-target")
            .flag(&target)
            .compile("assert_pubkey_eq");
    }
    
    println!("cargo:rerun-if-changed=assert_pubkey_eq.s");
}