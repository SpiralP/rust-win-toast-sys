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
    .compile("wintoastlib");

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
    .disable_name_namespacing()
    .default_enum_style(bindgen::EnumVariation::Rust {
      non_exhaustive: false,
    })
    // The input header we would like to generate
    // bindings for.
    .header("./src/interface.cpp")
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    // our custom function in interface.cpp don't have the WinToastLib namespace
    .whitelist_function("WinToast_.*")
    .whitelist_function("WinToastTemplate_.*")
    .whitelist_function("WinToastHandler.*")
    .whitelist_function("WinToastLib::WinToast_isCompatible")
    .whitelist_type("WinToastLib::IWinToastHandler_WinToastDismissalReason")
    .whitelist_type("WinToastLib::WinToastTemplate_Duration")
    .whitelist_type("WinToastLib::WinToastTemplate_AudioOption")
    .whitelist_type("WinToastLib::WinToastTemplate_TextField")
    .whitelist_type("WinToastLib::WinToastTemplate_WinToastTemplateType")
    .whitelist_type("WinToastLib::WinToastTemplate_AudioSystemFile")
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
