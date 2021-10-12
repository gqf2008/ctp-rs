use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=thostmduserapi_se");
    println!("cargo:rustc-link-lib=thosttraderapi_se");
    println!("cargo:rerun-if-changed=src/bridge.hpp");
    println!("cargo:rerun-if-changed=src/bridge.cpp");
    add_search_path();
    add_llvm_path();
    check_arch();
    cc::Build::new()
        .file("src/bridge.cpp")
        .cpp(true)
        // .warnings(false)
        .flag("-std=c++11")
        .compile("ctp_bridge");
    let bindings = bindgen::Builder::default()
        .header("src/bridge.hpp")
        .ignore_methods()
        .rustified_enum(".*")
        .blocklist_item("CTP_SIDE_TYPE")
        .blocklist_item("CTP_POSITION_EFFECT_TYPE")
        .blocklist_item("TCTPTradeTypeType")
        .blocklist_item("TCTPOrderTypeType")
        .blocklist_function("TradeSpiStub_Rust.*")
        .blocklist_function("QuoteSpiStub_Rust.*")
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_partialeq(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(target_os = "windows"))]
fn add_search_path() {
    for path in std::env::var("LD_LIBRARY_PATH")
        .unwrap_or_else(|_| "".to_string())
        .split(":")
    {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search={}", path);
    }
}

#[cfg(target_os = "windows")]
fn add_search_path() {
    println!("cargo:rustc-link-search={}", "lib/win64");
}

#[cfg(target_arch = "x86_64")]
fn check_arch() {}

#[cfg(target_arch = "x86")]
fn check_arch() {
    unimplemented!("Not implemented for 32 bit system!");
}

fn add_llvm_path() {
    if let Some(llvm_config_path) = option_env!("LLVM_CONFIG_PATH") {
        println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config_path);
    }
}
