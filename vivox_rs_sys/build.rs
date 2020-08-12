extern crate bindgen;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
  println!("cargo:rerun-if-changed=src/vivox.h");

  let sdk_path = PathBuf::from(env::var("VIVOX_SDK_PATH").expect(MISSING_SDK_PATH));
  let mut header_path = PathBuf::from(&sdk_path);
  header_path.push("SDK");
  header_path.push("include");

  verify_installation(&env::var("TARGET").unwrap(), &sdk_path);
  configure_linkage(&env::var("TARGET").unwrap(), &sdk_path);

  println!("cargo:rerun-if-env-changed=VIVOX_SDK_PATH");
  println!("cargo:rerun-if-changed={}", sdk_path.display());
  println!("cargo:rerun-if-changed={}/SDK/include/vivox-config.h", sdk_path.display());

  // Copy vivox headers to our src folder
  let mut prop_dir = PathBuf::from(env::current_exe().unwrap());
  prop_dir.pop(); // Suffer and wince at my lazy programming
  prop_dir.pop();
  prop_dir.pop();
  prop_dir.pop();
  prop_dir.pop();
  prop_dir.push("src");
  prop_dir.push("vivox-proprietary");
  println!("{}", prop_dir.display());

  fs::create_dir_all(&prop_dir).unwrap();

  // Copy all headers into vivox-proprietary
  for entry in fs::read_dir(header_path).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    fs::copy(path, format!("{}/{}", &prop_dir.display(), entry.file_name().to_str().unwrap())).unwrap();
  }

  let bindings = bindgen::Builder::default()
    .header("src/vivox.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");
  
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  // Delete local proprietary files
  fs::remove_dir_all(prop_dir).unwrap();
}

fn verify_installation(target: &str, sdk_path: &Path) {
  match target {
    "x86_64-apple-darwin" => {
      assert!(
        sdk_path
          .join("SDK/Libraries/Release/libvivoxsdk.dylib")
          .exists(),
        MISSING_SETUP
      );
      assert!(
        sdk_path
          .join("SDK/Libraries/Release/libortp.dylib")
          .exists(),
        MISSING_SETUP
      );
    }

    "x86_64-pc-windows-gnu" | "x86_64-pc-windows-msvc" => {
      assert!(
        sdk_path.join("SDK/Libraries/Release/x64/vivoxsdk.dll").exists(),
        MISSING_SETUP
      );
      assert!(
        sdk_path.join("SDK/Libraries/Release/x64/vivoxsdk.lib").exists(),
        MISSING_SETUP
      );
    }

    _ => panic!(INCOMPATIBLE_PLATFORM),
  }
}

fn configure_linkage(target: &str, sdk_path: &Path) {
  match target {
    "x86_64-apple-darwin" => {
      println!("cargo:rustc-link-lib=vivoxsdk");
      println!("cargo:rustc-link-lib=ortp");
      println!(
        "cargo:rustc-link-search={}",
        sdk_path.join("SDK/Libraries/Release").display()
      );
    }

    "x86_64-pc-windows-gnu" | "x86_64-pc-windows-msvc" => {
      println!("cargo:rustc-link-lib=vivoxsdk");
      println!(
        "cargo:rustc-link-search={}",
        sdk_path.join("SDK/Libraries/Release/x64").display()
      );
    }

    _ => {}
  }
}

const MISSING_SDK_PATH: &str = r#"
vivox-rs: Hello,

You are trying to generate bindings for the Core Vivox SDK.
You will have to download the Core SDK yourself.

If you are a registered Vivox developer, you can get it at:
https://developer.vivox.com/apps

NOTE:
You must have a Vivox Developer account to generate these bindings.

Once you have downloaded it, extract the contents to a folder
and set the environment variable `VIVOX_SDK_PATH` to its path.

Example:
$ export VIVOX_SDK_PATH=$HOME/Downloads/vivox-sdk-1.2.3.45678.hash

Please report any issues you have at:
https://github.com/MOONMOONOSS/vivox-rs

Thanks, and apologies for the inconvenience
"#;

const MISSING_SETUP: &str = r#"
vivox-rs: Hello,

You are trying to link to the Discord Game SDK.
Some additional set-up is required, namely some files need to be copied for the linker:

# Linux: You cannot create bindings for Vivox Core SDK targeting Linux

# Mac OS: prepend with `SDK/Libraries/Release` and add to library search path
$ cp $VIVOX_SDK_PATH/SDK/Libraries/Release/{,lib}vivoxsdk.dylib
$ cp $VIVOX_SDK_PATH/SDK/Libraries/Release/{,lib}ortp.dylib
$ export DYLD_LIBRARY_PATH=${DYLD_LIBRARY_PATH:+${DYLD_LIBRARY_PATH}:}$VIVOX_SDK_PATH/SDK/Libraries/Release

# Windows: No action is needed

After all this, `cargo build` and `cargo run` should function as expected.
Please report any issues you have at:
https://github.com/MOONMOONOSS/vivox-rs

Thanks, and apologies for the inconvenience
"#;

const INCOMPATIBLE_PLATFORM: &str = r#"
vivox-rs: Hello,

You are trying to link to the Vivox Core SDK.

Unfortunately, the platform you are trying to target is not supported.

Vivox has no intentions of providing binaries for Linux.

I sincerely apologize for cutting your adventure short.

All the best.
"#;
