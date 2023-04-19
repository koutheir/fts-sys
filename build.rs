use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("fts-sys: Environment variable 'OUT_DIR' was not defined.");

    println!("cargo:root={}", out_dir.to_str().unwrap());

    if !target_os_is_supported() {
        return;
    }

    generate_bindings(&out_dir)
}

fn target_os_is_supported() -> bool {
    match get_target_os().as_deref() {
        None => false, // Do not build anything for bare metal architectures.

        Some("linux") | Some("android") | Some("androideabi") | Some("dragonfly")
        | Some("freebsd") | Some("netbsd") | Some("openbsd") => {
            true // Continue, probably supported.
        }

        _ => false, // Do not build anything, probably unsupported.
    }
}

fn get_target_os() -> Option<String> {
    let target =
        env::var("TARGET").expect("selinux-sys: Environment variable 'TARGET' was not defined.");

    let components: Vec<_> = target.split('-').collect();
    let os_index = match components.len() {
        2 => {
            // e.g., aarch64-fuchsia, wasm32-wasi, x86_64-fuchsia
            if components[1] == "none" {
                return None; // Bare metal target.
            }
            1
        }

        3 | 4 => {
            // e.g., aarch64-unknown-freebsd, aarch64-unknown-linux-gnu
            if components[1] == "none" || components[2] == "none" {
                return None; // Bare metal target.
            }
            2
        }

        _ => panic!("Unrecognized target triplet '{target}'"),
    };

    Some(String::from(components[os_index]))
}

fn generate_bindings(out_dir: &Path) {
    let bindings = bindgen::Builder::default()
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .size_t_is_usize(true)
        .derive_debug(true)
        .derive_copy(true)
        .impl_debug(true)
        .allowlist_function("fts_(open|read|children|set|close)")
        .allowlist_type("FTSENT")
        .blocklist_type("__.+")
        .blocklist_type("(FTS|stat|timespec|dev_t|ino_t|nlink_t)")
        .allowlist_var("FTS_.+")
        //.clang_arg("-D_FILE_OFFSET_BITS=64")
        .header("src/includes.c")
        .generate()
        .expect("fts-sys: Failed to generate Rust bindings for 'fts.h'.");

    bindings
        .write_to_file(out_dir.join("fts-sys.rs"))
        .expect("fts-sys: Failed to write 'fts-sys.rs'.")
}
