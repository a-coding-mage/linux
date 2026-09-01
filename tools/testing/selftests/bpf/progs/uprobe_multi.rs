// SPDX-License-Identifier: GPL-2.0
// Source dependencies: vmlinux.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h,
// and bpf/usdt.bpf.h provide pt_regs, SEC, and BPF helper declarations.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: __u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
    fn bpf_strncmp(s1: *const i8, s1_sz: __u32, s2: *const i8) -> i32;
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_attach_cookie(ctx: *mut core::ffi::c_void) -> __u64;
    fn bpf_get_func_ip(ctx: *mut core::ffi::c_void) -> __u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_1_addr: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_2_addr: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_3_addr: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_1_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_2_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_3_result: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut uretprobe_multi_func_1_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uretprobe_multi_func_2_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uretprobe_multi_func_3_result: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut uprobe_multi_sleep_result: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut child_pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut child_tid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut child_pid_usdt: i32 = 0;
#[unsafe(no_mangle)]
pub static mut child_tid_usdt: i32 = 0;

#[unsafe(no_mangle)]
pub static mut expect_pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut bad_pid_seen: bool = false;
#[unsafe(no_mangle)]
pub static mut bad_pid_seen_usdt: bool = false;

#[unsafe(no_mangle)]
pub static mut test_cookie: bool = false;
#[unsafe(no_mangle)]
pub static mut user_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

#[inline(always)]
unsafe fn verify_sleepable_user_copy() -> bool {
    let mut data: [i8; 9] = [0; 9];

    unsafe {
        bpf_copy_from_user(
            data.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&data) as __u32,
            user_ptr as *const core::ffi::c_void,
        );
        bpf_strncmp(
            data.as_ptr(),
            core::mem::size_of_val(&data) as __u32,
            c"test_data".as_ptr(),
        ) == 0
    }
}

unsafe fn uprobe_multi_check(ctx: *mut core::ffi::c_void, is_return: bool, is_sleep: bool) {
    let cur_pid_tgid: __u64 = unsafe { bpf_get_current_pid_tgid() };
    let cur_pid: __u32;

    cur_pid = (cur_pid_tgid >> 32) as __u32;
    if unsafe { pid } != 0 && cur_pid != unsafe { pid as __u32 } {
        return;
    }

    if unsafe { expect_pid } != 0 && cur_pid != unsafe { expect_pid as __u32 } {
        unsafe {
            bad_pid_seen = true;
        }
    }

    unsafe {
        child_pid = (cur_pid_tgid >> 32) as i32;
        child_tid = cur_pid_tgid as __u32 as i32;
    }

    let cookie: __u64 = if unsafe { test_cookie } {
        unsafe { bpf_get_attach_cookie(ctx) }
    } else {
        0
    };
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx) };

    macro_rules! SET {
        ($var:ident, $addr:expr, $cookie:expr) => {
            if addr == unsafe { $addr } && (!unsafe { test_cookie } || cookie == $cookie) {
                unsafe {
                    $var = $var.wrapping_add(1);
                }
            }
        };
    }

    if is_return {
        SET!(uretprobe_multi_func_1_result, uprobe_multi_func_1_addr, 2);
        SET!(uretprobe_multi_func_2_result, uprobe_multi_func_2_addr, 3);
        SET!(uretprobe_multi_func_3_result, uprobe_multi_func_3_addr, 1);
    } else {
        SET!(uprobe_multi_func_1_result, uprobe_multi_func_1_addr, 3);
        SET!(uprobe_multi_func_2_result, uprobe_multi_func_2_addr, 1);
        SET!(uprobe_multi_func_3_result, uprobe_multi_func_3_addr, 2);
    }

    if is_sleep && unsafe { verify_sleepable_user_copy() } {
        unsafe {
            uprobe_multi_sleep_result = uprobe_multi_sleep_result.wrapping_add(1);
        }
    }
}

#[unsafe(link_section = "uprobe.multi//proc/self/exe:uprobe_multi_func_*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe(ctx: *mut pt_regs) -> i32 {
    unsafe {
        uprobe_multi_check(ctx as *mut core::ffi::c_void, false, false);
    }
    0
}

#[unsafe(link_section = "uretprobe.multi//proc/self/exe:uprobe_multi_func_*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uretprobe(ctx: *mut pt_regs) -> i32 {
    unsafe {
        uprobe_multi_check(ctx as *mut core::ffi::c_void, true, false);
    }
    0
}

#[unsafe(link_section = "uprobe.multi.s//proc/self/exe:uprobe_multi_func_*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_sleep(ctx: *mut pt_regs) -> i32 {
    unsafe {
        uprobe_multi_check(ctx as *mut core::ffi::c_void, false, true);
    }
    0
}

#[unsafe(link_section = "uretprobe.multi.s//proc/self/exe:uprobe_multi_func_*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uretprobe_sleep(ctx: *mut pt_regs) -> i32 {
    unsafe {
        uprobe_multi_check(ctx as *mut core::ffi::c_void, true, true);
    }
    0
}

#[unsafe(link_section = "uprobe.multi//proc/self/exe:uprobe_multi_func_*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_extra(_ctx: *mut pt_regs) -> i32 {
    /* we need this one just to mix PID-filtered and global uprobes */
    0
}

#[unsafe(link_section = "usdt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_pid(_ctx: *mut pt_regs) -> i32 {
    let cur_pid_tgid: __u64 = unsafe { bpf_get_current_pid_tgid() };
    let cur_pid: __u32;

    cur_pid = (cur_pid_tgid >> 32) as __u32;
    if unsafe { pid } != 0 && cur_pid != unsafe { pid as __u32 } {
        return 0;
    }

    if unsafe { expect_pid } != 0 && cur_pid != unsafe { expect_pid as __u32 } {
        unsafe {
            bad_pid_seen_usdt = true;
        }
    }

    unsafe {
        child_pid_usdt = (cur_pid_tgid >> 32) as i32;
        child_tid_usdt = cur_pid_tgid as __u32 as i32;
    }

    0
}

#[unsafe(link_section = "usdt")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn usdt_extra(_ctx: *mut pt_regs) -> i32 {
    /* we need this one just to mix PID-filtered and global USDT probes */
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
