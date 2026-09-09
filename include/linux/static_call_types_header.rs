/* SPDX-License-Identifier: GPL-2.0 */

// Translated from static_call_types.h. C preprocessor symbol pasting and
// configuration conditions are retained below as Rust macros/comments.

pub const STATIC_CALL_KEY_PREFIX_STR: &str = "__SCK__";
pub const STATIC_CALL_KEY_PREFIX_LEN: usize = STATIC_CALL_KEY_PREFIX_STR.len();
pub const STATIC_CALL_TRAMP_PREFIX_STR: &str = "__SCT__";
pub const STATIC_CALL_TRAMP_PREFIX_LEN: usize = STATIC_CALL_TRAMP_PREFIX_STR.len();

// STATIC_CALL_KEY_PREFIX and STATIC_CALL_TRAMP_PREFIX are token prefixes:
// __SCK__ and __SCT__, respectively.
macro_rules! STATIC_CALL_KEY_STR {
    ($name:ident) => { stringify!(__SCK__$name) };
}
macro_rules! STATIC_CALL_TRAMP_STR {
    ($name:ident) => { stringify!(__SCT__$name) };
}

/* Flags in the low bits of static_call_site::key. */
pub const STATIC_CALL_SITE_TAIL: usize = 1;
pub const STATIC_CALL_SITE_INIT: usize = 2;
pub const STATIC_CALL_SITE_FLAGS: usize = 3;

/* The static call site table is created by external tooling. */
#[repr(C)]
pub struct static_call_site {
    pub addr: i32,
    pub key: i32,
}

// External dependency supplied by the surrounding translation unit.
#[repr(C)]
pub struct static_call_mod {
    _private: [u8; 0],
}

/* DECLARE_STATIC_CALL(name, func) */
macro_rules! DECLARE_STATIC_CALL {
    ($name:ident, $func:ident) => {
        extern "C" {
            static mut $name: static_call_key;
            static mut $func: *const ();
        }
    };
}

// CONFIG_HAVE_STATIC_CALL
#[cfg(feature = "CONFIG_HAVE_STATIC_CALL_INLINE")]
#[repr(C)]
pub union static_call_key_target {
    /* bit 0: 0 = mods, 1 = sites */
    pub r#type: usize,
    pub mods: *mut static_call_mod,
    pub sites: *mut static_call_site,
}

#[cfg(feature = "CONFIG_HAVE_STATIC_CALL_INLINE")]
#[repr(C)]
pub struct static_call_key {
    pub func: *mut (),
    pub target: static_call_key_target,
}

#[cfg(not(feature = "CONFIG_HAVE_STATIC_CALL_INLINE"))]
#[repr(C)]
pub struct static_call_key {
    pub func: *mut (),
}

#[cfg(not(feature = "CONFIG_HAVE_STATIC_CALL"))]
#[repr(C)]
pub struct static_call_key_no_static_call {
    pub func: *mut (),
}

// __ADDRESSABLE() is an external compiler/object-tool facility; these macros
// preserve the original call structure where the corresponding configuration
// is enabled.
#[cfg(feature = "CONFIG_HAVE_STATIC_CALL_INLINE")]
macro_rules! __STATIC_CALL_ADDRESSABLE {
    ($name:ident) => {{
        let _ = stringify!(__SCK__$name);
    }};
}

#[cfg(feature = "CONFIG_HAVE_STATIC_CALL_INLINE")]
macro_rules! __static_call {
    ($name:ident) => {{
        __STATIC_CALL_ADDRESSABLE!($name);
        unsafe { &__SCT__$name }
    }};
}

#[cfg(not(feature = "CONFIG_HAVE_STATIC_CALL_INLINE"))]
macro_rules! __STATIC_CALL_ADDRESSABLE {
    ($name:ident) => {{}};
}

#[cfg(not(feature = "CONFIG_HAVE_STATIC_CALL_INLINE"))]
macro_rules! __static_call {
    ($name:ident) => {{ unsafe { &__SCT__$name } }};
}

macro_rules! __raw_static_call {
    ($name:ident) => {{ unsafe { &__SCT__$name } }};
}

#[cfg(feature = "MODULE")]
macro_rules! __STATIC_CALL_MOD_ADDRESSABLE {
    ($name:ident) => {{}};
}

#[cfg(not(feature = "MODULE"))]
macro_rules! __STATIC_CALL_MOD_ADDRESSABLE {
    ($name:ident) => { __STATIC_CALL_ADDRESSABLE!($name) };
}

macro_rules! static_call_mod {
    ($name:ident) => { __static_call!($name) };
}

macro_rules! static_call {
    ($name:ident) => { __static_call!($name) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
