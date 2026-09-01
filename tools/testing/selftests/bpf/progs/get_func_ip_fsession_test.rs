// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

use core::ffi::c_void;

extern "C" {
    fn bpf_get_func_ip(ctx: *mut c_void) -> u64;
    fn bpf_session_is_return(ctx: *mut c_void) -> i32;

    static bpf_fentry_test1: u8;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut test1_entry_result: u64 = 0;

#[no_mangle]
pub static mut test1_exit_result: u64 = 0;

#[no_mangle]
#[link_section = "fsession/bpf_fentry_test1"]
pub unsafe extern "C" fn test1(ctx: *mut c_void, _a: i32) -> i32 {
    let addr: u64 = bpf_get_func_ip(ctx);

    if bpf_session_is_return(ctx) != 0 {
        test1_exit_result = ((addr as *const c_void) == (&bpf_fentry_test1 as *const u8).cast::<c_void>()) as u64;
    } else {
        test1_entry_result = ((addr as *const c_void) == (&bpf_fentry_test1 as *const u8).cast::<c_void>()) as u64;
    }
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
