/* SPDX-License-Identifier: GPL-2.0 */

/*
 * File can be included directly by headers who only want to access
 * tracepoint->key to guard out of line trace calls, or the definition of
 * trace_print_flags{_u64}. Otherwise linux/tracepoint.h should be used.
 *
 * C dependencies supplied by other headers are intentionally referenced but
 * not defined here.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};

#[repr(C)]
pub struct StaticCallKey {
    _private: [u8; 0],
}

#[repr(C)]
pub struct StaticKeyFalse {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TracePrintFlags {
    pub mask: c_ulong,
    pub name: *const c_char,
}

#[repr(C)]
pub struct TracePrintFlagsU64 {
    pub mask: c_ulonglong,
    pub name: *const c_char,
}

#[repr(C)]
pub struct TracepointFunc {
    pub func: *mut c_void,
    pub data: *mut c_void,
    pub prio: c_int,
}

#[repr(C)]
pub struct TracepointExt {
    pub regfunc: Option<unsafe extern "C" fn() -> c_int>,
    pub unregfunc: Option<unsafe extern "C" fn()>,
    /* Flags. */
    pub faultable: c_uint,
}

#[repr(C)]
pub struct Tracepoint {
    pub name: *const c_char, /* Tracepoint name */
    pub key: StaticKeyFalse,
    pub static_call_key: *mut StaticCallKey,
    pub static_call_tramp: *mut c_void,
    pub iterator: *mut c_void,
    pub probestub: *mut c_void,
    pub funcs: *mut TracepointFunc,
    pub ext: *mut TracepointExt,
}

/* CONFIG_HAVE_ARCH_PREL32_RELOCATIONS selects the const-int representation. */
#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
pub type TracepointPtr = c_int;

#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
pub type TracepointPtr = *mut Tracepoint;

#[repr(C, align(32))]
pub struct BpfRawEventMap {
    pub tp: *mut Tracepoint,
    pub bpf_func: *mut c_void,
    pub num_args: u32,
    pub writable_size: u32,
}

/*
 * A tracepoint used from a header should be tested with tracepoint_enabled()
 * before calling a wrapper function defined in a C file.
 */
#[macro_export]
macro_rules! DECLARE_TRACEPOINT {
    ($tp:ident) => {
        unsafe extern "C" {
            pub static mut __tracepoint_$tp: $crate::Tracepoint;
        }
    };
}

/* CONFIG_TRACEPOINTS controls whether the external static-branch operation is used. */
#[cfg(CONFIG_TRACEPOINTS)]
#[macro_export]
macro_rules! tracepoint_enabled {
    ($tp:ident) => {{
        unsafe { static_branch_unlikely(&(*core::ptr::addr_of!(__tracepoint_$tp)).key) }
    }};
}

#[cfg(not(CONFIG_TRACEPOINTS))]
#[macro_export]
macro_rules! tracepoint_enabled {
    ($tracepoint:ident) => { false };
}

#[cfg(CONFIG_TRACEPOINTS)]
unsafe extern "C" {
    pub fn static_branch_unlikely(key: *const StaticKeyFalse) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
