// SPDX-License-Identifier: GPL-2.0
// Translated from C source using linux/bpf.h, bpf_helpers.h, bpf_tracing.h,
// and stdbool.h dependencies.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

type __u32 = u32;
type __u64 = u64;
type __s32 = i32;
type __s64 = i64;

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
}

unsafe fn sync_fetch_and_add_u64(ptr: *mut __u64, val: __u64) -> __u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

unsafe fn sync_fetch_and_add_u32(ptr: *mut __u32, val: __u32) -> __u32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_add(val));
    old
}

unsafe fn sync_fetch_and_sub_s64(ptr: *mut __s64, val: __s64) -> __s64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_sub(val));
    old
}

unsafe fn sync_fetch_and_sub_s32(ptr: *mut __s32, val: __s32) -> __s32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_sub(val));
    old
}

unsafe fn sync_fetch_and_sub_u64(ptr: *mut __u64, val: __u64) -> __u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old.wrapping_sub(val));
    old
}

unsafe fn sync_fetch_and_and_u64(ptr: *mut __u64, val: __u64) -> __u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old & val);
    old
}

unsafe fn sync_fetch_and_and_u32(ptr: *mut __u32, val: __u32) -> __u32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old & val);
    old
}

unsafe fn sync_fetch_and_or_u64(ptr: *mut __u64, val: __u64) -> __u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old | val);
    old
}

unsafe fn sync_fetch_and_or_u32(ptr: *mut __u32, val: __u32) -> __u32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old | val);
    old
}

unsafe fn sync_fetch_and_xor_u64(ptr: *mut __u64, val: __u64) -> __u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old ^ val);
    old
}

unsafe fn sync_fetch_and_xor_u32(ptr: *mut __u32, val: __u32) -> __u32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, old ^ val);
    old
}

unsafe fn sync_val_compare_and_swap_u64(ptr: *mut __u64, oldval: __u64, newval: __u64) -> __u64 {
    let old = core::ptr::read_volatile(ptr);
    if old == oldval {
        core::ptr::write_volatile(ptr, newval);
    }
    old
}

unsafe fn sync_val_compare_and_swap_u32(ptr: *mut __u32, oldval: __u32, newval: __u32) -> __u32 {
    let old = core::ptr::read_volatile(ptr);
    if old == oldval {
        core::ptr::write_volatile(ptr, newval);
    }
    old
}

unsafe fn sync_lock_test_and_set_u64(ptr: *mut __u64, val: __u64) -> __u64 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, val);
    old
}

unsafe fn sync_lock_test_and_set_u32(ptr: *mut __u32, val: __u32) -> __u32 {
    let old = core::ptr::read_volatile(ptr);
    core::ptr::write_volatile(ptr, val);
    old
}

#[cfg(ENABLE_ATOMICS_TESTS)]
#[link_section = ".data"]
pub static mut skip_tests: bool = false;
#[cfg(not(ENABLE_ATOMICS_TESTS))]
pub static mut skip_tests: bool = true;

pub static mut pid: __u32 = 0;

pub static mut add64_value: __u64 = 1;
pub static mut add64_result: __u64 = 0;
pub static mut add32_value: __u32 = 1;
pub static mut add32_result: __u32 = 0;
pub static mut add_stack_value_copy: __u64 = 0;
pub static mut add_stack_result: __u64 = 0;
pub static mut add_noreturn_value: __u64 = 1;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn add(ctx: *const core::ffi::c_void) -> i32 {
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        let mut add_stack_value: __u64 = 1;

        add64_result = sync_fetch_and_add_u64(&mut add64_value, 2);
        add32_result = sync_fetch_and_add_u32(&mut add32_value, 2);
        add_stack_result = sync_fetch_and_add_u64(&mut add_stack_value, 2);
        add_stack_value_copy = add_stack_value;
        sync_fetch_and_add_u64(&mut add_noreturn_value, 2);
    }

    let _ = ctx;
    return 0;
}

pub static mut sub64_value: __s64 = 1;
pub static mut sub64_result: __s64 = 0;
pub static mut sub32_value: __s32 = 1;
pub static mut sub32_result: __s32 = 0;
pub static mut sub_stack_value_copy: __s64 = 0;
pub static mut sub_stack_result: __s64 = 0;
pub static mut sub_noreturn_value: __s64 = 1;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn sub(ctx: *const core::ffi::c_void) -> i32 {
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        let mut sub_stack_value: __u64 = 1;

        sub64_result = sync_fetch_and_sub_s64(&mut sub64_value, 2);
        sub32_result = sync_fetch_and_sub_s32(&mut sub32_value, 2);
        sub_stack_result = sync_fetch_and_sub_u64(&mut sub_stack_value, 2) as __s64;
        sub_stack_value_copy = sub_stack_value as __s64;
        sync_fetch_and_sub_s64(&mut sub_noreturn_value, 2);
    }

    let _ = ctx;
    return 0;
}

pub static mut and64_value: __u64 = 0x110u64 << 32;
pub static mut and64_result: __u64 = 0;
pub static mut and32_value: __u32 = 0x110;
pub static mut and32_result: __u32 = 0;
pub static mut and_noreturn_value: __u64 = 0x110u64 << 32;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn r#and(ctx: *const core::ffi::c_void) -> i32 {
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        and64_result = sync_fetch_and_and_u64(&mut and64_value, 0x011u64 << 32);
        and32_result = sync_fetch_and_and_u32(&mut and32_value, 0x011);
        sync_fetch_and_and_u64(&mut and_noreturn_value, 0x011u64 << 32);
    }

    let _ = ctx;
    return 0;
}

pub static mut or64_value: __u64 = 0x110u64 << 32;
pub static mut or64_result: __u64 = 0;
pub static mut or32_value: __u32 = 0x110;
pub static mut or32_result: __u32 = 0;
pub static mut or_noreturn_value: __u64 = 0x110u64 << 32;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn r#or(ctx: *const core::ffi::c_void) -> i32 {
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        or64_result = sync_fetch_and_or_u64(&mut or64_value, 0x011u64 << 32);
        or32_result = sync_fetch_and_or_u32(&mut or32_value, 0x011);
        sync_fetch_and_or_u64(&mut or_noreturn_value, 0x011u64 << 32);
    }

    let _ = ctx;
    return 0;
}

pub static mut xor64_value: __u64 = 0x110u64 << 32;
pub static mut xor64_result: __u64 = 0;
pub static mut xor32_value: __u32 = 0x110;
pub static mut xor32_result: __u32 = 0;
pub static mut xor_noreturn_value: __u64 = 0x110u64 << 32;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn r#xor(ctx: *const core::ffi::c_void) -> i32 {
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        xor64_result = sync_fetch_and_xor_u64(&mut xor64_value, 0x011u64 << 32);
        xor32_result = sync_fetch_and_xor_u32(&mut xor32_value, 0x011);
        sync_fetch_and_xor_u64(&mut xor_noreturn_value, 0x011u64 << 32);
    }

    let _ = ctx;
    return 0;
}

pub static mut cmpxchg64_value: __u64 = 1;
pub static mut cmpxchg64_result_fail: __u64 = 0;
pub static mut cmpxchg64_result_succeed: __u64 = 0;
pub static mut cmpxchg32_value: __u32 = 1;
pub static mut cmpxchg32_result_fail: __u32 = 0;
pub static mut cmpxchg32_result_succeed: __u32 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn cmpxchg(ctx: *const core::ffi::c_void) -> i32 {
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        cmpxchg64_result_fail = sync_val_compare_and_swap_u64(&mut cmpxchg64_value, 0, 3);
        cmpxchg64_result_succeed = sync_val_compare_and_swap_u64(&mut cmpxchg64_value, 1, 2);

        cmpxchg32_result_fail = sync_val_compare_and_swap_u32(&mut cmpxchg32_value, 0, 3);
        cmpxchg32_result_succeed = sync_val_compare_and_swap_u32(&mut cmpxchg32_value, 1, 2);
    }

    let _ = ctx;
    return 0;
}

pub static mut xchg64_value: __u64 = 1;
pub static mut xchg64_result: __u64 = 0;
pub static mut xchg32_value: __u32 = 1;
pub static mut xchg32_result: __u32 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn xchg(ctx: *const core::ffi::c_void) -> i32 {
    if pid != (bpf_get_current_pid_tgid() >> 32) as __u32 {
        return 0;
    }
    #[cfg(ENABLE_ATOMICS_TESTS)]
    {
        let val64: __u64 = 2;
        let val32: __u32 = 2;

        xchg64_result = sync_lock_test_and_set_u64(&mut xchg64_value, val64);
        xchg32_result = sync_lock_test_and_set_u32(&mut xchg32_value, val32);
    }

    let _ = ctx;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
