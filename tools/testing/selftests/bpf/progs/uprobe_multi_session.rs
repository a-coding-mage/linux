// SPDX-License-Identifier: GPL-2.0
// Dependencies in the original C source:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <stdbool.h>,
// and "bpf_misc.h".

pub type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_func_ip(ctx: *mut core::ffi::c_void) -> __u64;
    fn bpf_session_is_return(ctx: *mut pt_regs) -> bool;
    fn bpf_copy_from_user(
        dst: *mut core::ffi::c_void,
        size: __u64,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i64;
    fn bpf_strncmp(
        s1: *const core::ffi::c_void,
        s1_sz: __u64,
        s2: *const core::ffi::c_char,
    ) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_1_addr: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_2_addr: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut uprobe_multi_func_3_addr: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut uprobe_session_result: [__u64; 3] = [0; 3];
#[unsafe(no_mangle)]
pub static mut uprobe_multi_sleep_result: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut user_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut pid: core::ffi::c_int = 0;

unsafe fn uprobe_multi_check(ctx: *mut core::ffi::c_void, is_return: bool) -> core::ffi::c_int {
    let funcs: [__u64; 3] = [
        unsafe { uprobe_multi_func_1_addr },
        unsafe { uprobe_multi_func_2_addr },
        unsafe { uprobe_multi_func_3_addr },
    ];
    let mut i: core::ffi::c_uint;
    let addr: __u64;

    let _ = is_return;

    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid } as __u64 {
        return 1;
    }

    addr = unsafe { bpf_get_func_ip(ctx) };

    i = 0;
    while (i as usize) < funcs.len() {
        if funcs[i as usize] == addr {
            unsafe {
                uprobe_session_result[i as usize] =
                    uprobe_session_result[i as usize].wrapping_add(1);
            }
            break;
        }
        i = i.wrapping_add(1);
    }

    /* only uprobe_multi_func_2 executes return probe */
    if (addr == unsafe { uprobe_multi_func_1_addr }) || (addr == unsafe { uprobe_multi_func_3_addr }) {
        return 1;
    }

    0
}

#[unsafe(link_section = "uprobe.session//proc/self/exe:uprobe_multi_func_*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe(ctx: *mut pt_regs) -> core::ffi::c_int {
    unsafe {
        uprobe_multi_check(
            ctx as *mut core::ffi::c_void,
            bpf_session_is_return(ctx),
        )
    }
}

#[inline(always)]
unsafe fn verify_sleepable_user_copy() -> bool {
    let mut data: [core::ffi::c_char; 9] = [0; 9];

    unsafe {
        bpf_copy_from_user(
            data.as_mut_ptr() as *mut core::ffi::c_void,
            core::mem::size_of_val(&data) as __u64,
            user_ptr as *const core::ffi::c_void,
        );
        bpf_strncmp(
            data.as_ptr() as *const core::ffi::c_void,
            core::mem::size_of_val(&data) as __u64,
            c"test_data".as_ptr(),
        ) == 0
    }
}

#[unsafe(link_section = "uprobe.session.s//proc/self/exe:uprobe_multi_func_*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_sleepable(ctx: *mut pt_regs) -> core::ffi::c_int {
    unsafe {
        if verify_sleepable_user_copy() {
            uprobe_multi_sleep_result = uprobe_multi_sleep_result.wrapping_add(1);
        }
        uprobe_multi_check(
            ctx as *mut core::ffi::c_void,
            bpf_session_is_return(ctx),
        )
    }
}
