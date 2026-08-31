// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

unsafe extern "C" {
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut user_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub static mut retval: i32 = 0;

#[unsafe(link_section = "fentry.s")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fentry_xdp() -> i32 {
    let mut buff: [u8; 64] = [0; 64];

    unsafe {
        retval = bpf_copy_from_user(
            buff.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&buff) as u32,
            user_ptr as *const core::ffi::c_void,
        ) as i32;
    }
    0
}
