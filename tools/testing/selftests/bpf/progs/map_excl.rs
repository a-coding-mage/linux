// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2025 Google LLC. */

/* Dependencies from the original C source:
 * <linux/bpf.h>, <time.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
 */

pub const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct ExclMap {
    pub type_: *mut [u32; BPF_MAP_TYPE_ARRAY as usize],
    pub key: *mut u32,
    pub value: *mut u32,
    pub max_entries: *mut [u32; 1],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut excl_map: ExclMap = ExclMap {
    type_: core::ptr::null_mut(),
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
};

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    pub fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

/* Original section:
 * SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
 */
#[no_mangle]
pub unsafe extern "C" fn should_have_access(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 0;
    let mut value: i32 = 0xdeadbeefu32 as i32;

    let _ = ctx;
    unsafe {
        bpf_map_update_elem(
            &raw mut excl_map as *mut core::ffi::c_void,
            &raw const key as *const core::ffi::c_void,
            &raw const value as *const core::ffi::c_void,
            0,
        );
    }
    0
}

/* Original section:
 * SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
 */
#[no_mangle]
pub unsafe extern "C" fn should_not_have_access(ctx: *mut core::ffi::c_void) -> i32 {
    let mut key: i32 = 0;
    let mut value: i32 = 0xdeadbeefu32 as i32;

    let _ = ctx;
    unsafe {
        bpf_map_update_elem(
            &raw mut excl_map as *mut core::ffi::c_void,
            &raw const key as *const core::ffi::c_void,
            &raw const value as *const core::ffi::c_void,
            0,
        );
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
