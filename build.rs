#[allow(unused)]
use std::path::Path;

#[allow(unused)]
const ROOT: &str = "c_lib";

fn main() {
    let luajit = luajit_src::Build::new().build();
    luajit.print_cargo_metadata();
    #[allow(unused)]
    let include = luajit.include_dir();
    #[cfg(feature = "lbc")]
    build_lbc(include);
    #[cfg(feature = "lpeg")]
    build_lpeg(include);
    #[cfg(feature = "lrexlib")]
    build_lrexlib(include);
    #[cfg(feature = "lsqlite3")]
    build_lsqlite3(include);
}

#[cfg(feature = "lbc")]
fn build_lbc(include: &Path) {
    let dir = Path::new(ROOT).join("lbc");
    cc::Build::new()
        .include(include)
        .file(dir.join("lbc.c"))
        .file(dir.join("src").join("number.c"))
        .include(dir.join("src"))
        .include(dir)
        .compile("lbc");
}

#[cfg(feature = "lpeg")]
fn build_lpeg(include: &Path) {
    let dir = Path::new(ROOT).join("lpeg");
    cc::Build::new()
        .include(include)
        .warnings(false)
        .file(dir.join("lpcap.c"))
        .file(dir.join("lpcode.c"))
        .file(dir.join("lpprint.c"))
        .file(dir.join("lptree.c"))
        .file(dir.join("lpvm.c"))
        .include(dir)
        .compile("lpeg");
}

#[cfg(feature = "lrexlib")]
fn build_lrexlib(include: &Path) {
    let dir = Path::new(ROOT).join("lrexlib").join("src");
    cc::Build::new()
        .include(include)
        .define("PCRE2_CODE_UNIT_WIDTH", "8")
        .file(dir.join("common.c"))
        .file(dir.join("pcre2").join("lpcre2_f.c"))
        .file(dir.join("pcre2").join("lpcre2.c"))
        .include(dir)
        .include(Path::new(ROOT).join("pcre2-mirror").join("src"))
        .compile("lrexlib");
}

#[cfg(feature = "lsqlite3")]
fn build_lsqlite3(include: &Path) {
    let dir = Path::new(ROOT).join("lsqlite3");
    cc::Build::new()
        .include(include)
        .warnings(false)
        .file(dir.join("lsqlite3.c"))
        .file(dir.join("extras").join("extension-functions.c"))
        .include(dir)
        .include(Path::new(ROOT).join("sqlite-amalgamation"))
        .compile("lsqlite3");
}
