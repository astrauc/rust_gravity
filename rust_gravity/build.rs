// use std::env;

use std::{env, path::Path};


fn main() {
    let src = ["lib/RustSpdLog.cpp", "lib/RustGravityNode.cpp", 
        "lib/RustGravityDataProduct.cpp", "lib/RustFutureResponse.cpp",
         "lib/RustGravitySubscriber.cpp", "lib/RustGravityRequestor.cpp",
         "lib/RustGravityHeartbeatListener.cpp", "lib/RustGravitySubscriptionMonitor.cpp",
         "lib/RustGravityServiceProvider.cpp"
         ];

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cwd = Path::new(&dir);
    let gravity_root = cwd.parent().unwrap();
    let lib_path = gravity_root.join("git/gravity/build/install/lib/");
    let include_path = gravity_root.join("git/gravity/build/install/include");
    // println!("cargo:rustc-link-lib=add");   "libs/mult.cpp",

    cxx_build::bridge("src/ffi.rs")
        .files(src.iter())
        .include(include_path)
        .compile("rust_gravity");
    
        println!("cargo::rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=gravity_protobufs");
    println!("cargo:rustc-link-lib=gravity");
    println!("cargo:rustc-link-lib=spdlog");
   
}