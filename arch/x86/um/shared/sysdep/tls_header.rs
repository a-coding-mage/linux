/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The __UM_HOST__ build condition is preserved with the `um_host` feature.
 * When it is not enabled, user_desc_t refers to the externally supplied
 * user_desc type.
 */

#[cfg(feature = "um_host")]
#[repr(C)]
pub struct um_dup_user_desc {
    pub entry_number: ::core::ffi::c_uint,
    pub base_addr: ::core::ffi::c_uint,
    pub limit: ::core::ffi::c_uint,
    /* C bit-fields; each field is retained as its declared unsigned-int
     * storage value because Rust has no native bit-field syntax. */
    pub seg_32bit: ::core::ffi::c_uint,      /* :1 */
    pub contents: ::core::ffi::c_uint,       /* :2 */
    pub read_exec_only: ::core::ffi::c_uint, /* :1 */
    pub limit_in_pages: ::core::ffi::c_uint, /* :1 */
    pub seg_not_present: ::core::ffi::c_uint, /* :1 */
    pub useable: ::core::ffi::c_uint,        /* :1 */
    #[cfg(target_arch = "x86_64")]
    pub lm: ::core::ffi::c_uint,             /* :1 */
}

#[cfg(feature = "um_host")]
pub type user_desc_t = um_dup_user_desc;

#[cfg(not(feature = "um_host"))]
#[repr(C)]
pub struct user_desc;

#[cfg(not(feature = "um_host"))]
pub type user_desc_t = user_desc;

unsafe extern "C" {
    pub fn os_set_thread_area(info: *mut user_desc_t, pid: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    pub fn os_get_thread_area(info: *mut user_desc_t, pid: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
}

#[cfg(target_arch = "x86")]
pub const GDT_ENTRY_TLS_MIN_I386: ::core::ffi::c_int = 6;

#[cfg(target_arch = "x86")]
pub const GDT_ENTRY_TLS_MIN_X86_64: ::core::ffi::c_int = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
