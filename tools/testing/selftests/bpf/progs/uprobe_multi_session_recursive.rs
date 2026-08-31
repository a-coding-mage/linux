// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <stdbool.h>, and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_session_cookie(ctx: *mut pt_regs) -> *mut __u64;
    fn bpf_session_is_return(ctx: *mut pt_regs) -> bool;
    fn bpf_get_current_pid_tgid() -> __u64;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

#[unsafe(no_mangle)]
pub static mut idx_entry: i32 = 0;
#[unsafe(no_mangle)]
pub static mut idx_return: i32 = 0;

#[unsafe(no_mangle)]
pub static mut test_uprobe_cookie_entry: [__u64; 6] = [0; 6];
#[unsafe(no_mangle)]
pub static mut test_uprobe_cookie_return: [__u64; 3] = [0; 3];

unsafe fn check_cookie(ctx: *mut pt_regs) -> i32 {
    let cookie: *mut __u64 = unsafe { bpf_session_cookie(ctx) };

    if unsafe { bpf_session_is_return(ctx) } {
        if unsafe { idx_return } >= test_uprobe_cookie_return.len() as i32 {
            return 1;
        }
        unsafe {
            test_uprobe_cookie_return[idx_return as usize] = *cookie;
            idx_return += 1;
        }
        return 0;
    }

    if unsafe { idx_entry } >= test_uprobe_cookie_entry.len() as i32 {
        return 1;
    }
    unsafe {
        *cookie = test_uprobe_cookie_entry[idx_entry as usize];
        let ret = idx_entry % 2;
        idx_entry += 1;
        ret
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "uprobe.session//proc/self/exe:uprobe_session_recursive")]
pub unsafe extern "C" fn uprobe_recursive(ctx: *mut pt_regs) -> i32 {
    if ((unsafe { bpf_get_current_pid_tgid() } >> 32) as i32) != unsafe { pid } {
        return 1;
    }

    unsafe { check_cookie(ctx) }
}
