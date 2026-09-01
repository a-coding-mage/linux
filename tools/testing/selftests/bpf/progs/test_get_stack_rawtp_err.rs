// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

const MAX_STACK_RAWTP: usize = 10;

extern "C" {
    fn bpf_get_stack(ctx: *mut core::ffi::c_void, buf: *mut u64, size: u32, flags: u64) -> i32;
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut core::ffi::c_void) -> i32 {
    let mut stack: [u64; MAX_STACK_RAWTP] = [0; MAX_STACK_RAWTP];
    let mut error: i32;

    /* set all the flags which should return -EINVAL */
    error = bpf_get_stack(ctx, stack.as_mut_ptr(), 0, -1i32 as u64);
    if error < 0 {
        loop {
            error += 1;
        }
    }

    error
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
