use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("fts-sys: Environment variable 'OUT_DIR' was not defined.");

    println!("cargo:root={}", out_dir.to_str().unwrap());

    generate_bindings(&out_dir)
}

fn generate_bindings(out_dir: &Path) {
    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .size_t_is_usize(true)
        .derive_debug(true)
        .derive_copy(true)
        .impl_debug(true)
        .whitelist_function("fts_(open|read|children|set|close)")
        .whitelist_type("FTSENT")
        .blacklist_type("__.+")
        .blacklist_type("(FTS|stat|timespec|dev_t|ino_t|nlink_t)")
        .whitelist_var("FTS_.+")
        //.clang_arg("-D_FILE_OFFSET_BITS=64")
        .header("src/includes.c")
        .generate()
        .expect("fts-sys: Failed to generate Rust bindings for 'fts.h'.");

    bindings
        .write_to_file(out_dir.join("fts-sys.rs"))
        .expect("fts-sys: Failed to write 'fts-sys.rs'.")
}
