// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include <sys/types.h>

pub type pid_t = i32;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_probe_read_user_str(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

#[no_mangle]
pub static mut pid: pid_t = 0;

#[no_mangle]
pub static mut ret: i64 = 0;

#[no_mangle]
pub static mut user_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

#[no_mangle]
pub static mut buf: [i8; 256] = [0; 256];

#[link_section = "tracepoint/syscalls/sys_enter_nanosleep"]
#[no_mangle]
pub unsafe extern "C" fn on_write(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    if pid != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    ret = bpf_probe_read_user_str(
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&buf) as u32,
        user_ptr as *const core::ffi::c_void,
    );

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
