use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=./WinToast/src/wintoastlib.cpp");
    println!("cargo:rerun-if-changed=./src/interface.cpp");

    let mut builder = cc::Build::new();
    builder.cpp(true);

    #[cfg(not(debug_assertions))]
    {
        builder.define("NDEBUG", None);
    }

    builder
        .file("./WinToast/src/wintoastlib.cpp")
        .file("./src/interface.cpp")
        .include("./WinToast/include")
        .compile("wintoastlib");

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
