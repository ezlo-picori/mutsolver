

fn build_dict_registry() {
    
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Generate dict_registry from files found in data/dict
    build_dict_registry();
}