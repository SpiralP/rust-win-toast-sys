use std::{env, path::PathBuf};

fn main() {
    println!("cmake");

    println!("cargo:rerun-if-changed=./WinToast/src/wintoastlib.cpp");
    let win_toast_library_path = cmake::Config::new("WinToast")
        .no_build_target(true)
        .profile("Release")
        .define("WINTOASTLIB_BUILD_EXAMPLES", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        win_toast_library_path
            .join("build")
            .join("Release")
            .display()
    );
    println!("cargo:rustc-link-lib=static=WinToast");

    println!("cc");
    let mut cc_builder = cc::Build::new();

    #[cfg(not(debug_assertions))]
    {
        cc_builder.define("NDEBUG", None);
    }
    cc_builder
        .cpp(true)
        .file("./src/interface.cpp")
        .include("./WinToast/include")
        .compile("interface");

    println!("bindgen");
    let bindings = bindgen::Builder::default()
        .disable_name_namespacing()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .header("./src/interface.cpp")
        .clang_arg("-I./WinToast/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("WinToast_.*")
        .allowlist_function("WinToastTemplate_.*")
        .allowlist_function("WinToastHandler.*")
        .allowlist_function("WinToastLib::WinToast_isCompatible")
        .allowlist_type("WinToastLib::IWinToastHandler_WinToastDismissalReason")
        .allowlist_type("WinToastLib::WinToastTemplate_Duration")
        .allowlist_type("WinToastLib::WinToastTemplate_AudioOption")
        .allowlist_type("WinToastLib::WinToastTemplate_TextField")
        .allowlist_type("WinToastLib::WinToastTemplate_WinToastTemplateType")
        .allowlist_type("WinToastLib::WinToastTemplate_AudioSystemFile")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
