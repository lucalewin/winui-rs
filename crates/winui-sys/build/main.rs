use std::{process::Command, path::PathBuf, env};

const WINDOWS_APP_SDK_VERSION: &str = "1.4.230913002";

fn main() {
    println!(r#"cargo:rustc-link-search=C:\dev\winui-rs\crates\winui-sys\packages\Microsoft.WindowsAppSDK.{WINDOWS_APP_SDK_VERSION}\lib\win10-x64"#);
    println!(r#"cargo:rustc-link-lib=Microsoft.WindowsAppRuntime.Bootstrap"#);

    download_and_generate();
    generate_bindings();
}

fn generate_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bootstrap_bindings = bindgen::Builder::default()
        .header("src/wrapper/bootstrap.h")
        .allowlist_type("PACKAGE_VERSION")
        .allowlist_type("MddBootstrapInitializeOptions")
        .allowlist_item("MddBootstrapInitialize")
        .allowlist_item("MddBootstrapInitialize2")
        .allowlist_item("MddBootstrapShutdown")
        .clang_args([
            "-x", "c++",
            "-std=c++17",
            "-Wc++17-extensions",
            "-Igenerated_files",
            &format!(r"-Ipackages\Microsoft.WindowsAppSDK.{WINDOWS_APP_SDK_VERSION}\include"),
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bootstrap_bindings
        .write_to_file(out_path.join("bootstrap.rs"))
        .expect("Couldn't write bindings!");
}

/// use `nuget` and `cppwinrt` to download the `WindowsAppSDK` and generate header files
fn download_and_generate() {
    // download / install WindowsAppSDK nuget package
    Command::new("nuget")
        .args([
            "install",
            "Microsoft.WindowsAppSDK",
            "-OutputDirectory",
            "packages",
            "-Version",
            WINDOWS_APP_SDK_VERSION
        ])
        .spawn()
        .unwrap();

    // generate header files
    Command::new("cppwinrt")
        .args([
            "-optimize",
            "-input",
            &format!("packages\\Microsoft.WindowsAppSDK.{WINDOWS_APP_SDK_VERSION}\\lib\\uap10.0"),
            "-input",
            &format!("packages\\Microsoft.WindowsAppSDK.{WINDOWS_APP_SDK_VERSION}\\lib\\uap10.0.18362"),
            "-input",
            "sdk",
            "-output",
            "generated_files"
        ])
        .spawn()
        .unwrap();
}
