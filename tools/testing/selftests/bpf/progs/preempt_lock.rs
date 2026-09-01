// SPDX-License-Identifier: GPL-2.0
// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, "bpf_misc.h",
// and "bpf_experimental.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_void};
use core::ptr;

type u32 = u32;
type u64 = u64;

#[repr(C)]
pub struct __sk_buff {
    pub mark: u32,
}

unsafe extern "C" {
    fn bpf_preempt_disable();
    fn bpf_preempt_enable();
    fn bpf_guard_preempt();
    fn bpf_copy_from_user(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> i64;
    fn bpf_printk(fmt: *const c_char, ...) -> i64;

    // extern int bpf_copy_from_user_str(...) __weak __ksym;
    fn bpf_copy_from_user_str(
        dst: *mut c_void,
        dst__sz: u32,
        unsafe_ptr__ign: *const c_void,
        flags: u64,
    ) -> i32;
}

// SEC("?tc")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_lock_missing_1(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_preempt_disable();
    }
    0
}

// SEC("?tc")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_lock_missing_2(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_preempt_disable();
        bpf_preempt_disable();
    }
    0
}

// SEC("?tc")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_lock_missing_3(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_preempt_disable();
        bpf_preempt_disable();
        bpf_preempt_disable();
    }
    0
}

// SEC("?tc")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_lock_missing_3_minus_2(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_preempt_disable();
        bpf_preempt_disable();
        bpf_preempt_disable();
        bpf_preempt_enable();
        bpf_preempt_enable();
    }
    0
}

#[inline(never)]
unsafe fn preempt_disable() {
    unsafe {
        bpf_preempt_disable();
    }
}

#[inline(never)]
unsafe fn preempt_enable() {
    unsafe {
        bpf_preempt_enable();
    }
}

// SEC("?tc")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_lock_missing_1_subprog(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        preempt_disable();
    }
    0
}

// SEC("?tc")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_lock_missing_2_subprog(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        preempt_disable();
        preempt_disable();
    }
    0
}

// SEC("?tc")
// __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_preempt_disable-ed region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_lock_missing_2_minus_1_subprog(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        preempt_disable();
        preempt_disable();
        preempt_enable();
    }
    0
}

#[inline(never)]
unsafe fn preempt_balance_subprog() {
    unsafe {
        preempt_disable();
        preempt_enable();
    }
}

// SEC("?tc")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_balance(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        bpf_guard_preempt();
    }
    0
}

// SEC("?tc")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_balance_subprog_test(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        preempt_balance_subprog();
    }
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure __msg("sleepable helper bpf_copy_from_user#")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_sleepable_helper(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut data: u32 = 0;

    unsafe {
        bpf_preempt_disable();
        bpf_copy_from_user(
            &mut data as *mut u32 as *mut c_void,
            core::mem::size_of_val(&data) as u32,
            ptr::null(),
        );
        bpf_preempt_enable();
    }
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure __msg("kernel func bpf_copy_from_user_str is sleepable within non-preemptible region")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_sleepable_kfunc(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let mut data: u32 = 0;

    unsafe {
        bpf_preempt_disable();
        bpf_copy_from_user_str(
            &mut data as *mut u32 as *mut c_void,
            core::mem::size_of_val(&data) as u32,
            ptr::null(),
            0,
        );
        bpf_preempt_enable();
    }
    0
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_global_subprog() -> i32 {
    unsafe {
        preempt_balance_subprog();
    }
    0
}

// SEC("?tc")
// __success
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_global_subprog_test(ctx: *mut __sk_buff) -> i32 {
    let _ = ctx;
    unsafe {
        preempt_disable();
        preempt_global_subprog();
        preempt_enable();
    }
    0
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_subprog(i: i32) -> i32 {
    if i != 0 {
        static FMT: &[u8] = b"%p\0";
        unsafe {
            bpf_printk(FMT.as_ptr() as *const c_char, &i as *const i32);
        }
    }
    i
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_sleepable_helper_subprog(mut i: i32) -> i32 {
    if i != 0 {
        unsafe {
            bpf_copy_from_user(
                &mut i as *mut i32 as *mut c_void,
                core::mem::size_of_val(&i) as u32,
                ptr::null(),
            );
        }
    }
    i
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_sleepable_kfunc_subprog(mut i: i32) -> i32 {
    if i != 0 {
        unsafe {
            bpf_copy_from_user_str(
                &mut i as *mut i32 as *mut c_void,
                core::mem::size_of_val(&i) as u32,
                ptr::null(),
                0,
            );
        }
    }
    unsafe {
        global_subprog(i);
    }
    i
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn global_subprog_calling_sleepable_global(i: i32) -> i32 {
    if i == 0 {
        unsafe {
            global_sleepable_kfunc_subprog(i);
        }
    }
    i
}

// SEC("?syscall")
// __failure __msg("sleepable global function")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_global_sleepable_helper_subprog(ctx: *mut __sk_buff) -> i32 {
    unsafe {
        preempt_disable();
        if (*ctx).mark != 0 {
            global_sleepable_helper_subprog((*ctx).mark as i32);
        }
        preempt_enable();
    }
    0
}

// SEC("?syscall")
// __failure __msg("sleepable global function")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_global_sleepable_kfunc_subprog(ctx: *mut __sk_buff) -> i32 {
    unsafe {
        preempt_disable();
        if (*ctx).mark != 0 {
            global_sleepable_kfunc_subprog((*ctx).mark as i32);
        }
        preempt_enable();
    }
    0
}

// SEC("?syscall")
// __failure __msg("sleepable global function")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn preempt_global_sleepable_subprog_indirect(ctx: *mut __sk_buff) -> i32 {
    unsafe {
        preempt_disable();
        if (*ctx).mark != 0 {
            global_subprog_calling_sleepable_global((*ctx).mark as i32);
        }
        preempt_enable();
    }
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
