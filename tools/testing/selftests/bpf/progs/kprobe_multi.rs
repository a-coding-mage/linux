// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/progs/kprobe_multi.c.
// C includes removed; external BPF/kernel symbols are declared below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_attach_cookie(ctx: *mut core::ffi::c_void) -> __u64;
    fn bpf_get_func_ip(ctx: *mut core::ffi::c_void) -> __u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

unsafe extern "C" {
    #[link_name = "bpf_fentry_test1"]
    static bpf_fentry_test1: core::ffi::c_void;
    #[link_name = "bpf_fentry_test2"]
    static bpf_fentry_test2: core::ffi::c_void;
    #[link_name = "bpf_fentry_test3"]
    static bpf_fentry_test3: core::ffi::c_void;
    #[link_name = "bpf_fentry_test4"]
    static bpf_fentry_test4: core::ffi::c_void;
    #[link_name = "bpf_fentry_test5"]
    static bpf_fentry_test5: core::ffi::c_void;
    #[link_name = "bpf_fentry_test6"]
    static bpf_fentry_test6: core::ffi::c_void;
    #[link_name = "bpf_fentry_test7"]
    static bpf_fentry_test7: core::ffi::c_void;
    #[link_name = "bpf_fentry_test8"]
    static bpf_fentry_test8: core::ffi::c_void;
}

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut test_cookie: bool = false;

#[unsafe(no_mangle)]
pub static mut kprobe_test1_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_test2_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_test3_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_test4_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_test5_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_test6_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_test7_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_test8_result: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut kretprobe_test1_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_test2_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_test3_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_test4_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_test5_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_test6_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_test7_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_test8_result: __u64 = 0;

unsafe fn set_if_match(
    var: *mut __u64,
    addr: __u64,
    expected_addr: *const core::ffi::c_void,
    cookie: __u64,
    expected_cookie: __u64,
) {
    if addr as *const core::ffi::c_void == expected_addr && (!test_cookie || cookie == expected_cookie) {
        *var = 1;
    }
}

unsafe fn kprobe_multi_check(ctx: *mut core::ffi::c_void, is_return: bool) {
    if (bpf_get_current_pid_tgid() >> 32) != pid as __u64 {
        return;
    }

    let cookie: __u64 = if test_cookie {
        bpf_get_attach_cookie(ctx)
    } else {
        0
    };
    let addr: __u64 = bpf_get_func_ip(ctx);

    if is_return {
        set_if_match(&raw mut kretprobe_test1_result, addr, &raw const bpf_fentry_test1, cookie, 8);
        set_if_match(&raw mut kretprobe_test2_result, addr, &raw const bpf_fentry_test2, cookie, 2);
        set_if_match(&raw mut kretprobe_test3_result, addr, &raw const bpf_fentry_test3, cookie, 7);
        set_if_match(&raw mut kretprobe_test4_result, addr, &raw const bpf_fentry_test4, cookie, 6);
        set_if_match(&raw mut kretprobe_test5_result, addr, &raw const bpf_fentry_test5, cookie, 5);
        set_if_match(&raw mut kretprobe_test6_result, addr, &raw const bpf_fentry_test6, cookie, 4);
        set_if_match(&raw mut kretprobe_test7_result, addr, &raw const bpf_fentry_test7, cookie, 3);
        set_if_match(&raw mut kretprobe_test8_result, addr, &raw const bpf_fentry_test8, cookie, 1);
    } else {
        set_if_match(&raw mut kprobe_test1_result, addr, &raw const bpf_fentry_test1, cookie, 1);
        set_if_match(&raw mut kprobe_test2_result, addr, &raw const bpf_fentry_test2, cookie, 7);
        set_if_match(&raw mut kprobe_test3_result, addr, &raw const bpf_fentry_test3, cookie, 2);
        set_if_match(&raw mut kprobe_test4_result, addr, &raw const bpf_fentry_test4, cookie, 3);
        set_if_match(&raw mut kprobe_test5_result, addr, &raw const bpf_fentry_test5, cookie, 4);
        set_if_match(&raw mut kprobe_test6_result, addr, &raw const bpf_fentry_test6, cookie, 5);
        set_if_match(&raw mut kprobe_test7_result, addr, &raw const bpf_fentry_test7, cookie, 6);
        set_if_match(&raw mut kprobe_test8_result, addr, &raw const bpf_fentry_test8, cookie, 8);
    }
}

/*
 * No tests in here, just to trigger 'bpf_fentry_test*'
 * through tracing test_run
 */
#[unsafe(link_section = "fentry/bpf_modify_return_test")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn trigger() -> i32 {
    0
}

#[unsafe(link_section = "kprobe.multi/bpf_fentry_tes??")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kprobe(ctx: *mut pt_regs) -> i32 {
    kprobe_multi_check(ctx as *mut core::ffi::c_void, false);
    0
}

#[unsafe(link_section = "kretprobe.multi/bpf_fentry_test*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kretprobe(ctx: *mut pt_regs) -> i32 {
    kprobe_multi_check(ctx as *mut core::ffi::c_void, true);
    0
}

#[unsafe(link_section = "kprobe.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kprobe_manual(ctx: *mut pt_regs) -> i32 {
    kprobe_multi_check(ctx as *mut core::ffi::c_void, false);
    0
}

#[unsafe(link_section = "kretprobe.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kretprobe_manual(ctx: *mut pt_regs) -> i32 {
    kprobe_multi_check(ctx as *mut core::ffi::c_void, true);
    0
}

unsafe extern "C" {
    #[link_name = "bpf_testmod_fentry_test1"]
    static bpf_testmod_fentry_test1: core::ffi::c_void;
    #[link_name = "bpf_testmod_fentry_test2"]
    static bpf_testmod_fentry_test2: core::ffi::c_void;
    #[link_name = "bpf_testmod_fentry_test3"]
    static bpf_testmod_fentry_test3: core::ffi::c_void;
}

#[unsafe(no_mangle)]
pub static mut kprobe_testmod_test1_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_testmod_test2_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kprobe_testmod_test3_result: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut kretprobe_testmod_test1_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_testmod_test2_result: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut kretprobe_testmod_test3_result: __u64 = 0;

unsafe fn kprobe_multi_testmod_check(ctx: *mut core::ffi::c_void, is_return: bool) {
    if (bpf_get_current_pid_tgid() >> 32) != pid as __u64 {
        return;
    }

    let addr: __u64 = bpf_get_func_ip(ctx);

    if is_return {
        if addr as *const core::ffi::c_void == &raw const bpf_testmod_fentry_test1 {
            kretprobe_testmod_test1_result = 1;
        }
        if addr as *const core::ffi::c_void == &raw const bpf_testmod_fentry_test2 {
            kretprobe_testmod_test2_result = 1;
        }
        if addr as *const core::ffi::c_void == &raw const bpf_testmod_fentry_test3 {
            kretprobe_testmod_test3_result = 1;
        }
    } else {
        if addr as *const core::ffi::c_void == &raw const bpf_testmod_fentry_test1 {
            kprobe_testmod_test1_result = 1;
        }
        if addr as *const core::ffi::c_void == &raw const bpf_testmod_fentry_test2 {
            kprobe_testmod_test2_result = 1;
        }
        if addr as *const core::ffi::c_void == &raw const bpf_testmod_fentry_test3 {
            kprobe_testmod_test3_result = 1;
        }
    }
}

#[unsafe(link_section = "kprobe.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kprobe_testmod(ctx: *mut pt_regs) -> i32 {
    kprobe_multi_testmod_check(ctx as *mut core::ffi::c_void, false);
    0
}

#[unsafe(link_section = "kretprobe.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kretprobe_testmod(ctx: *mut pt_regs) -> i32 {
    kprobe_multi_testmod_check(ctx as *mut core::ffi::c_void, true);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
