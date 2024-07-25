#[allow(unused)]
use std::os::raw::c_int;

#[allow(unused)]
use mlua_sys::lua_State;

#[cfg(feature = "libsqlite3-sys")]
extern crate libsqlite3_sys;
#[cfg(feature = "libsqlite3-sys")]
pub use libsqlite3_sys::{SQLITE_VERSION, SQLITE_VERSION_NUMBER};

#[cfg(feature = "pcre2-sys")]
extern crate pcre2_sys;
#[cfg(feature = "pcre2-sys")]
pub use pcre2_sys::{PCRE2_MAJOR, PCRE2_MINOR};

extern "C-unwind" {
    #[cfg(feature = "lbc")]
    pub fn luaopen_bc(L: *mut lua_State) -> c_int;

    #[cfg(feature = "lpeg")]
    pub fn luaopen_lpeg(L: *mut lua_State) -> c_int;

    #[cfg(feature = "lrexlib")]
    pub fn luaopen_rex_pcre2(L: *mut lua_State) -> c_int;

    #[cfg(feature = "lsqlite3")]
    pub fn luaopen_lsqlite3(L: *mut lua_State) -> c_int;
}
