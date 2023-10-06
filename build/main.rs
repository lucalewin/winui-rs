use std::{process::Command, path::PathBuf, env};

const WINDOWS_APP_SDK_VERSION: &str = "1.4.230913002";

fn main() {
    // download_and_generate();
    generate_bindings();
}

fn generate_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bootstrap_bindings = bindgen::Builder::default()
        .header("src/wrapper/bootstrap.h")
        // .clang_arg("-Ipackages\\Microsoft.WindowsAppSDK.1.4.230913002\\include")
        .clang_args([
            r"-Igenerated_files",
            r"-Ipackages\Microsoft.WindowsAppSDK.1.4.230913002\include",
            r"-IC:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\ucrt",
            r"-IC:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um",
            r"-IC:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\shared",
            r"-IC:\Program Files (x86)\Windows Kits\10\Lib\10.0.22621.0\um\x64",
            r"-IC:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.37.32822\include",
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
            const_format::concatcp!(
                "packages\\Microsoft.WindowsAppSDK.",
                WINDOWS_APP_SDK_VERSION,
                "\\lib\\uap10.0"
            ),
            "-input",
            const_format::concatcp!(
                "packages\\Microsoft.WindowsAppSDK.",
                WINDOWS_APP_SDK_VERSION,
                "\\lib\\uap10.0.18362"
            ),
            "-input",
            "sdk",
            "-output",
            "generated_files"
        ])
        .spawn()
        .unwrap();
}
