/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// - <linux/types.h> provides s32.
// - <linux/stringify.h> provides __stringify().
// - <linux/compiler.h> provides __PASTE() and __ADDRESSABLE().

pub const STATIC_CALL_KEY_PREFIX_STR: &str = "__SCK__";
pub const STATIC_CALL_KEY_PREFIX_LEN: usize = STATIC_CALL_KEY_PREFIX_STR.len();

pub const STATIC_CALL_TRAMP_PREFIX_STR: &str = "__SCT__";
pub const STATIC_CALL_TRAMP_PREFIX_LEN: usize = STATIC_CALL_TRAMP_PREFIX_STR.len();

// C preprocessor identifier construction:
// #define STATIC_CALL_KEY_PREFIX __SCK__
// #define STATIC_CALL_KEY(name) __PASTE(STATIC_CALL_KEY_PREFIX, name)
// #define STATIC_CALL_KEY_STR(name) __stringify(STATIC_CALL_KEY(name))
// #define STATIC_CALL_TRAMP_PREFIX __SCT__
// #define STATIC_CALL_TRAMP(name) __PASTE(STATIC_CALL_TRAMP_PREFIX, name)
// #define STATIC_CALL_TRAMP_STR(name) __stringify(STATIC_CALL_TRAMP(name))
//
// Stable Rust has no direct source-level equivalent for creating an identifier
// from a macro argument. The stringifying forms are representable:
#[macro_export]
macro_rules! STATIC_CALL_KEY_STR {
    ($name:ident) => {
        concat!("__SCK__", stringify!($name))
    };
}

#[macro_export]
macro_rules! STATIC_CALL_TRAMP_STR {
    ($name:ident) => {
        concat!("__SCT__", stringify!($name))
    };
}

/*
 * Flags in the low bits of static_call_site::key.
 */
pub const STATIC_CALL_SITE_TAIL: ::core::ffi::c_ulong = 1; /* tail call */
pub const STATIC_CALL_SITE_INIT: ::core::ffi::c_ulong = 2; /* init section */
pub const STATIC_CALL_SITE_FLAGS: ::core::ffi::c_ulong = 3;

/*
 * The static call site table needs to be created by external tooling (objtool
 * or a compiler plugin).
 */
#[repr(C)]
pub struct static_call_site {
    pub addr: s32,
    pub key: s32,
}

// C declaration macro:
// #define DECLARE_STATIC_CALL(name, func) \
//      extern struct static_call_key STATIC_CALL_KEY(name); \
//      extern typeof(func) STATIC_CALL_TRAMP(name);
//
// This depends on C typeof() and preprocessor identifier pasting. Call sites
// should declare the generated extern key and trampoline symbols directly in
// Rust using their pasted symbol names.

// CONFIG_HAVE_STATIC_CALL selects the static-call implementation. The C header
// exposes different struct layouts and macros for each build-time branch.

#[cfg(CONFIG_HAVE_STATIC_CALL)]
// #define __raw_static_call(name) (&STATIC_CALL_TRAMP(name))
// Rust translation note: taking the address of the pasted trampoline symbol
// requires the concrete generated symbol to be declared by the including code.

#[cfg(all(CONFIG_HAVE_STATIC_CALL, CONFIG_HAVE_STATIC_CALL_INLINE))]
// #define __STATIC_CALL_ADDRESSABLE(name) __ADDRESSABLE(STATIC_CALL_KEY(name))
// #define __static_call(name) ({ __STATIC_CALL_ADDRESSABLE(name); __raw_static_call(name); })
// Rust translation note: __ADDRESSABLE() and pasted identifiers are supplied by
// compiler/preprocessor machinery outside this isolated header.

#[cfg(all(CONFIG_HAVE_STATIC_CALL, CONFIG_HAVE_STATIC_CALL_INLINE))]
#[repr(C)]
pub union static_call_key_data {
    /* bit 0: 0 = mods, 1 = sites */
    pub type_: ::core::ffi::c_ulong,
    pub mods: *mut static_call_mod,
    pub sites: *mut static_call_site,
}

#[cfg(all(CONFIG_HAVE_STATIC_CALL, CONFIG_HAVE_STATIC_CALL_INLINE))]
#[repr(C)]
pub struct static_call_key {
    pub func: *mut ::core::ffi::c_void,
    pub data: static_call_key_data,
}

#[cfg(all(CONFIG_HAVE_STATIC_CALL, not(CONFIG_HAVE_STATIC_CALL_INLINE)))]
// #define __STATIC_CALL_ADDRESSABLE(name)
// #define __static_call(name) __raw_static_call(name)

#[cfg(all(CONFIG_HAVE_STATIC_CALL, not(CONFIG_HAVE_STATIC_CALL_INLINE)))]
#[repr(C)]
pub struct static_call_key {
    pub func: *mut ::core::ffi::c_void,
}

// MODULE:
// #define __STATIC_CALL_MOD_ADDRESSABLE(name)
// #define static_call_mod(name) __raw_static_call(name)
//
// !MODULE:
// #define __STATIC_CALL_MOD_ADDRESSABLE(name) __STATIC_CALL_ADDRESSABLE(name)
// #define static_call_mod(name) __static_call(name)
//
// #define static_call(name) __static_call(name)
//
// These macros all depend on the same pasted key/trampoline identifiers noted
// above.

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
#[repr(C)]
pub struct static_call_key {
    pub func: *mut ::core::ffi::c_void,
}

#[cfg(not(CONFIG_HAVE_STATIC_CALL))]
// #define static_call(name) \
//      ((typeof(STATIC_CALL_TRAMP(name))*)(STATIC_CALL_KEY(name).func))
// Rust translation note: this fallback casts the stored function pointer from
// the pasted static-call key to a pointer to the pasted trampoline type.

