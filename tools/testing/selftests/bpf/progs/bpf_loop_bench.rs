// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies from the original C source:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

pub type u32 = u32;
pub type __u32 = u32;

unsafe extern "C" {
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(index: __u32, data: *mut core::ffi::c_void) -> i32,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

#[unsafe(no_mangle)]
pub static mut nr_loops: u32 = 0;
#[unsafe(no_mangle)]
pub static mut hits: i64 = 0;

unsafe extern "C" fn empty_callback(_index: __u32, _data: *mut core::ffi::c_void) -> i32 {
    return 0;
}

unsafe extern "C" fn outer_loop(_index: __u32, _data: *mut core::ffi::c_void) -> i32 {
    unsafe {
        bpf_loop(nr_loops, empty_callback, core::ptr::null_mut(), 0);
        core::sync::atomic::AtomicI64::from_ptr(core::ptr::addr_of_mut!(hits))
            .fetch_add(nr_loops as i64, core::sync::atomic::Ordering::SeqCst)
            + nr_loops as i64;
    }
    return 0;
}

/* Original section: SEC("fentry/" SYS_PREFIX "sys_getpgid") */
#[unsafe(link_section = "fentry/sys_getpgid")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn benchmark(_ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        bpf_loop(1000, outer_loop, core::ptr::null_mut(), 0);
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
