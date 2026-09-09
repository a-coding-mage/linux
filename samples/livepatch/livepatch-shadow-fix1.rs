// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
 */

/*
 * livepatch-shadow-fix1.c - Shadow variables, livepatch demo
 *
 * Purpose
 * -------
 *
 * Fixes the memory leak introduced in livepatch-shadow-mod through the
 * use of a shadow variable.  This fix demonstrates the "extending" of
 * short-lived data structures by patching its allocation and release
 * functions.
 *
 * This module is not intended to be standalone.  See the "Usage"
 * section of livepatch-shadow-mod.c.
 */

// C headers and kernel-provided symbols are supplied by external dependencies.

const SV_LEAK: i32 = 1;
const ALLOC_PERIOD: u64 = 1;
const CLEANUP_PERIOD: u64 = 3 * ALLOC_PERIOD;
const EXPIRE_PERIOD: u64 = 4 * CLEANUP_PERIOD;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct dummy {
    pub list: list_head,
    pub jiffies_expire: ::core::ffi::c_ulong,
}

extern "C" {
    static mut jiffies: ::core::ffi::c_ulong;
    fn kzalloc(size: usize, flags: ::core::ffi::c_uint) -> *mut ::core::ffi::c_void;
    fn kfree(ptr: *mut ::core::ffi::c_void);
    fn secs_to_jiffies(secs: u64) -> ::core::ffi::c_ulong;
    fn klp_shadow_alloc(
        obj: *mut ::core::ffi::c_void,
        id: ::core::ffi::c_ulong,
        size: usize,
        gfp_flags: ::core::ffi::c_uint,
        ctor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> i32>,
        ctor_data: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn klp_shadow_get(obj: *mut ::core::ffi::c_void, id: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    fn klp_shadow_free(obj: *mut ::core::ffi::c_void, id: ::core::ffi::c_ulong, dtor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>);
    fn klp_shadow_free_all(id: ::core::ffi::c_ulong, dtor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>);
    fn klp_enable_patch(patch: *mut klp_patch) -> i32;
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const ::core::ffi::c_char,
    pub new_func: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const ::core::ffi::c_char,
    pub funcs: *mut klp_func,
}

#[repr(C)]
pub struct klp_patch {
    pub module: *mut ::core::ffi::c_void,
    pub objs: *mut klp_object,
}

const GFP_KERNEL: ::core::ffi::c_uint = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn shadow_leak_ctor(obj: *mut ::core::ffi::c_void, shadow_data: *mut ::core::ffi::c_void, ctor_data: *mut ::core::ffi::c_void) -> i32 {
    let shadow_leak = shadow_data as *mut *mut i32;
    let leak = ctor_data as *mut *mut i32;
    if ctor_data.is_null() { return -22; }
    *shadow_leak = *leak;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn livepatch_fix1_dummy_alloc() -> *mut dummy {
    let d = kzalloc(::core::mem::size_of::<dummy>(), GFP_KERNEL) as *mut dummy;
    if d.is_null() { return ::core::ptr::null_mut(); }
    (*d).jiffies_expire = jiffies.wrapping_add(secs_to_jiffies(EXPIRE_PERIOD));
    let leak = kzalloc(::core::mem::size_of::<*mut i32>(), GFP_KERNEL) as *mut i32;
    if leak.is_null() { kfree(d as *mut _); return ::core::ptr::null_mut(); }
    let shadow_leak = klp_shadow_alloc(d as *mut _, SV_LEAK as _, ::core::mem::size_of::<*mut i32>(), GFP_KERNEL, Some(shadow_leak_ctor), &leak as *const _ as *mut _);
    if shadow_leak.is_null() { kfree(leak as *mut _); kfree(d as *mut _); return ::core::ptr::null_mut(); }
    d
}

pub unsafe extern "C" fn livepatch_fix1_dummy_leak_dtor(obj: *mut ::core::ffi::c_void, shadow_data: *mut ::core::ffi::c_void) {
    let shadow_leak = shadow_data as *mut *mut i32;
    kfree(*shadow_leak as *mut _);
}

pub unsafe extern "C" fn livepatch_fix1_dummy_free(d: *mut dummy) {
    let shadow_leak = klp_shadow_get(d as *mut _, SV_LEAK as _);
    if !shadow_leak.is_null() { klp_shadow_free(d as *mut _, SV_LEAK as _, Some(livepatch_fix1_dummy_leak_dtor)); }
    kfree(d as *mut _);
}

static mut FUNCS: [klp_func; 3] = [
    klp_func { old_name: b"dummy_alloc\0".as_ptr() as *const _, new_func: Some(livepatch_fix1_dummy_alloc as unsafe extern "C" fn()) },
    klp_func { old_name: b"dummy_free\0".as_ptr() as *const _, new_func: Some(livepatch_fix1_dummy_free as unsafe extern "C" fn()) },
    klp_func { old_name: ::core::ptr::null(), new_func: None },
];

static mut OBJS: [klp_object; 2] = [
    klp_object { name: b"livepatch_shadow_mod\0".as_ptr() as *const _, funcs: unsafe { FUNCS.as_mut_ptr() } },
    klp_object { name: ::core::ptr::null(), funcs: ::core::ptr::null_mut() },
];

static mut PATCH: klp_patch = klp_patch { module: ::core::ptr::null_mut(), objs: unsafe { OBJS.as_mut_ptr() } };

pub unsafe extern "C" fn livepatch_shadow_fix1_init() -> i32 { klp_enable_patch(&raw mut PATCH) }
pub unsafe extern "C" fn livepatch_shadow_fix1_exit() { klp_shadow_free_all(SV_LEAK as _, Some(livepatch_fix1_dummy_leak_dtor)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
