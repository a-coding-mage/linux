// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <stdbool.h>,
// "bpf_kfuncs.h", "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut uprobe_session_result: [__u64; 3] = [0; 3];

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

unsafe fn uprobe_multi_check(_ctx: *mut core::ffi::c_void, idx: i32) -> i32 {
    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid } as __u64 {
        return 1;
    }

    unsafe {
        uprobe_session_result[idx as usize] = uprobe_session_result[idx as usize].wrapping_add(1);
    }

    /* only consumer 1 executes return probe */
    if idx == 0 || idx == 2 {
        return 1;
    }

    return 0;
}

#[unsafe(link_section = "uprobe.session//proc/self/exe:uprobe_multi_func_1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_0(ctx: *mut pt_regs) -> i32 {
    return unsafe { uprobe_multi_check(ctx as *mut core::ffi::c_void, 0) };
}

#[unsafe(link_section = "uprobe.session//proc/self/exe:uprobe_multi_func_1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_1(ctx: *mut pt_regs) -> i32 {
    return unsafe { uprobe_multi_check(ctx as *mut core::ffi::c_void, 1) };
}

#[unsafe(link_section = "uprobe.session//proc/self/exe:uprobe_multi_func_1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_2(ctx: *mut pt_regs) -> i32 {
    return unsafe { uprobe_multi_check(ctx as *mut core::ffi::c_void, 2) };
}
