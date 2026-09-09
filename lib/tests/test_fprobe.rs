// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_fprobe.c - simple sanity test for fprobe
 */

// Kernel headers and KUnit declarations are supplied by the surrounding build.

const DIV_FACTOR: u32 = 3;

#[repr(C)]
pub struct kunit { _private: [u8; 0] }
#[repr(C)]
pub struct ftrace_regs { _private: [u8; 0] }

type U32 = u32;
type EntryHandler = unsafe extern "C" fn(*mut fprobe, usize, usize, *mut ftrace_regs, *mut core::ffi::c_void) -> i32;
type ExitHandler = unsafe extern "C" fn(*mut fprobe, usize, usize, *mut ftrace_regs, *mut core::ffi::c_void);

#[repr(C)]
pub struct fprobe {
    pub entry_handler: Option<EntryHandler>,
    pub exit_handler: Option<ExitHandler>,
    pub entry_data_size: usize,
    pub nmissed: usize,
}

extern "C" {
    fn preemptible() -> bool;
    fn ftrace_regs_get_return_value(regs: *mut ftrace_regs) -> usize;
    fn register_fprobe(fp: *mut fprobe, pattern: *const i8, data: *mut core::ffi::c_void) -> i32;
    fn register_fprobe_syms(fp: *mut fprobe, syms: *const *const i8, num: usize) -> i32;
    fn unregister_fprobe(fp: *mut fprobe) -> i32;
    fn get_random_u32_above(value: u32) -> u32;
    fn kallsyms_lookup_size_offset(addr: usize, size: *mut usize, offset: *mut usize) -> bool;
    fn ftrace_location_range(start: usize, end: usize) -> usize;
}

static mut current_test: *mut kunit = core::ptr::null_mut();
static mut rand1: U32 = 0;
static mut entry_only_val: U32 = 0;
static mut entry_val: U32 = 0;
static mut exit_val: U32 = 0;
static mut entry_only_count: U32 = 0;
static mut entry_count: U32 = 0;
static mut exit_count: U32 = 0;

/* Use indirect calls to avoid inlining the target functions */
static mut target: Option<unsafe extern "C" fn(U32) -> U32> = None;
static mut target2: Option<unsafe extern "C" fn(U32) -> U32> = None;
static mut target_ip: usize = 0;
static mut target2_ip: usize = 0;
static mut entry_return_value: i32 = 0;

#[inline(never)]
unsafe extern "C" fn fprobe_selftest_target(value: U32) -> U32 {
    value / DIV_FACTOR
}

#[inline(never)]
unsafe extern "C" fn fprobe_selftest_target2(value: U32) -> U32 {
    value / DIV_FACTOR + 1
}

unsafe extern "C" fn fp_entry_handler(fp: *mut fprobe, ip: usize, _ret_ip: usize,
                                       _fregs: *mut ftrace_regs, data: *mut core::ffi::c_void) -> i32 {
    let _ = preemptible();
    if ip != target_ip { assert_eq!(ip, target2_ip); }
    entry_val = rand1 / DIV_FACTOR;
    if (*fp).entry_data_size != 0 {
        if !data.is_null() { *(data as *mut U32) = entry_val; }
    } else { assert!(data.is_null()); }
    entry_return_value
}

unsafe extern "C" fn fp_exit_handler(fp: *mut fprobe, ip: usize, _ret_ip: usize,
                                      fregs: *mut ftrace_regs, data: *mut core::ffi::c_void) {
    let ret = ftrace_regs_get_return_value(fregs);
    let _ = preemptible();
    if ip != target_ip { assert_eq!(ip, target2_ip); assert_eq!(ret, (rand1 / DIV_FACTOR + 1) as usize); }
    else { assert_eq!(ret, (rand1 / DIV_FACTOR) as usize); }
    assert_eq!(entry_val, rand1 / DIV_FACTOR);
    exit_val = entry_val + DIV_FACTOR;
    if (*fp).entry_data_size != 0 { if !data.is_null() { assert_eq!(*(data as *mut U32), entry_val); } }
    else { assert!(data.is_null()); }
}

unsafe fn test_fprobe_entry(test: *mut kunit) {
    let mut fp = fprobe { entry_handler: Some(fp_entry_handler), exit_handler: None, entry_data_size: 0, nmissed: 0 };
    current_test = test;
    assert_ne!(unregister_fprobe(&mut fp), 0);
    assert_eq!(register_fprobe(&mut fp, b"fprobe_selftest_target*\0".as_ptr() as *const i8, core::ptr::null_mut()), 0);
    entry_val = 0; exit_val = 0; target.unwrap()(rand1); assert_ne!(entry_val, 0); assert_eq!(exit_val, 0);
    entry_val = 0; exit_val = 0; target2.unwrap()(rand1); assert_ne!(entry_val, 0); assert_eq!(exit_val, 0);
    assert_eq!(unregister_fprobe(&mut fp), 0);
}

unsafe fn test_fprobe(test: *mut kunit) {
    let mut fp = fprobe { entry_handler: Some(fp_entry_handler), exit_handler: Some(fp_exit_handler), entry_data_size: 0, nmissed: 0 };
    current_test = test;
    assert_eq!(register_fprobe(&mut fp, b"fprobe_selftest_target*\0".as_ptr() as *const i8, core::ptr::null_mut()), 0);
    entry_val = 0; exit_val = 0; target.unwrap()(rand1); assert_ne!(entry_val, 0); assert_eq!(entry_val + DIV_FACTOR, exit_val);
    entry_val = 0; exit_val = 0; target2.unwrap()(rand1); assert_ne!(entry_val, 0); assert_eq!(entry_val + DIV_FACTOR, exit_val);
    assert_eq!(unregister_fprobe(&mut fp), 0);
}

unsafe fn test_fprobe_syms(test: *mut kunit) {
    let syms: [*const i8; 2] = [b"fprobe_selftest_target\0".as_ptr() as _, b"fprobe_selftest_target2\0".as_ptr() as _];
    let mut fp = fprobe { entry_handler: Some(fp_entry_handler), exit_handler: Some(fp_exit_handler), entry_data_size: 0, nmissed: 0 };
    current_test = test; assert_eq!(register_fprobe_syms(&mut fp, syms.as_ptr(), 2), 0);
    entry_val = 0; exit_val = 0; target.unwrap()(rand1); assert_ne!(entry_val, 0); assert_eq!(entry_val + DIV_FACTOR, exit_val);
    entry_val = 0; exit_val = 0; target2.unwrap()(rand1); assert_ne!(entry_val, 0); assert_eq!(entry_val + DIV_FACTOR, exit_val);
    assert_eq!(unregister_fprobe(&mut fp), 0);
}
unsafe fn test_fprobe_data(test: *mut kunit) { let mut fp = fprobe { entry_handler: Some(fp_entry_handler), exit_handler: Some(fp_exit_handler), entry_data_size: core::mem::size_of::<U32>(), nmissed: 0 }; current_test = test; assert_eq!(register_fprobe(&mut fp, b"fprobe_selftest_target\0".as_ptr() as _, core::ptr::null_mut()), 0); target.unwrap()(rand1); assert_eq!(unregister_fprobe(&mut fp), 0); }
unsafe fn test_fprobe_skip(test: *mut kunit) { let mut fp = fprobe { entry_handler: Some(fp_entry_handler), exit_handler: Some(fp_exit_handler), entry_data_size: 0, nmissed: 0 }; current_test = test; assert_eq!(register_fprobe(&mut fp, b"fprobe_selftest_target\0".as_ptr() as _, core::ptr::null_mut()), 0); entry_return_value = 1; entry_val = 0; exit_val = 0; target.unwrap()(rand1); assert_ne!(entry_val, 0); assert_eq!(exit_val, 0); assert_eq!(fp.nmissed, 0); entry_return_value = 0; assert_eq!(unregister_fprobe(&mut fp), 0); }
unsafe extern "C" fn entry_only_handler(_fp: *mut fprobe, ip: usize, _ret_ip: usize, _fregs: *mut ftrace_regs, _data: *mut core::ffi::c_void) -> i32 { let _ = preemptible(); assert_eq!(ip, target_ip); entry_only_count += 1; entry_only_val = rand1 / DIV_FACTOR; 0 }
unsafe extern "C" fn fprobe_entry_multi_handler(_fp: *mut fprobe, ip: usize, _ret_ip: usize, _fregs: *mut ftrace_regs, _data: *mut core::ffi::c_void) -> i32 { let _ = preemptible(); assert_eq!(ip, target_ip); entry_count += 1; entry_val = rand1 / DIV_FACTOR; 0 }
unsafe extern "C" fn fprobe_exit_multi_handler(_fp: *mut fprobe, ip: usize, _ret_ip: usize, fregs: *mut ftrace_regs, _data: *mut core::ffi::c_void) { let ret = ftrace_regs_get_return_value(fregs); let _ = preemptible(); assert_eq!(ip, target_ip); assert_eq!(ret, (rand1 / DIV_FACTOR) as usize); exit_count += 1; exit_val = ret as U32; }
unsafe fn check_fprobe_multi(_test: *mut kunit) { target.unwrap()(rand1); assert_eq!(entry_only_count, 1); assert_eq!(entry_count, 1); assert_eq!(exit_count, 1); assert_eq!(entry_only_val, rand1 / DIV_FACTOR); assert_eq!(entry_val, rand1 / DIV_FACTOR); assert_eq!(exit_val, rand1 / DIV_FACTOR); }
unsafe fn test_fprobe_multi(test: *mut kunit) { let mut fp1 = fprobe { entry_handler: Some(fprobe_entry_multi_handler), exit_handler: Some(fprobe_exit_multi_handler), entry_data_size: 0, nmissed: 0 }; let mut fp2 = fprobe { entry_handler: Some(entry_only_handler), exit_handler: None, entry_data_size: 0, nmissed: 0 }; current_test = test; let p = b"fprobe_selftest_target\0".as_ptr() as _; assert_eq!(register_fprobe(&mut fp1,p,core::ptr::null_mut()),0); assert_eq!(register_fprobe(&mut fp2,p,core::ptr::null_mut()),0); entry_only_count=0; entry_count=0; exit_count=0; entry_only_val=0; entry_val=0; exit_val=0; check_fprobe_multi(test); assert_eq!(unregister_fprobe(&mut fp1),0); assert_eq!(unregister_fprobe(&mut fp2),0); assert_eq!(register_fprobe(&mut fp2,p,core::ptr::null_mut()),0); assert_eq!(register_fprobe(&mut fp1,p,core::ptr::null_mut()),0); entry_only_count=0; entry_count=0; exit_count=0; entry_only_val=0; entry_val=0; exit_val=0; check_fprobe_multi(test); assert_eq!(unregister_fprobe(&mut fp1),0); assert_eq!(unregister_fprobe(&mut fp2),0); }
unsafe fn get_ftrace_location(func: usize) -> usize { let mut size=0; if !kallsyms_lookup_size_offset(func,&mut size,core::ptr::null_mut()) || size==0 { return 0; } ftrace_location_range(func,func+size-1) }
unsafe fn fprobe_test_init(_test: *mut kunit) -> i32 { rand1=get_random_u32_above(DIV_FACTOR); target=Some(fprobe_selftest_target); target2=Some(fprobe_selftest_target2); target_ip=get_ftrace_location(fprobe_selftest_target as usize); target2_ip=get_ftrace_location(fprobe_selftest_target2 as usize); 0 }
// KUnit suite registration preserves the original six test cases and suite name "fprobe_test".

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
