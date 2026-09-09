// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
 */

/*
 * livepatch-shadow-fix2.c - Shadow variables, livepatch demo
 *
 * Purpose
 * -------
 *
 * Adds functionality to livepatch-shadow-mod's in-flight data
 * structures through a shadow variable.  The livepatch patches a
 * routine that periodically inspects data structures, incrementing a
 * per-data-structure counter, creating the counter if needed.
 */

// C headers and kernel-provided declarations are supplied by the surrounding
// kernel environment.

/* Shadow variable enums */
pub const SV_LEAK: u64 = 1;
pub const SV_COUNTER: u64 = 2;

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

unsafe extern "C" {
    fn klp_shadow_get_or_alloc(
        obj: *mut ::core::ffi::c_void,
        id: u64,
        size: usize,
        gfp_flags: ::core::ffi::c_uint,
        ctor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>,
        data: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn klp_shadow_get(obj: *mut ::core::ffi::c_void, id: u64) -> *mut ::core::ffi::c_void;
    fn klp_shadow_free(
        obj: *mut ::core::ffi::c_void,
        id: u64,
        dtor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>,
    );
    fn klp_shadow_free_all(
        id: u64,
        dtor: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void)>,
    );
    fn time_after(a: ::core::ffi::c_ulong, b: ::core::ffi::c_ulong) -> bool;
    fn kfree(ptr: *mut ::core::ffi::c_void);
    static mut THIS_MODULE: *mut ::core::ffi::c_void;
}

const GFP_NOWAIT: ::core::ffi::c_uint = 0;

unsafe fn livepatch_fix2_dummy_check(d: *mut dummy, jiffies: ::core::ffi::c_ulong) -> bool {
    let shadow_count: *mut i32;

    /*
     * Patch: handle in-flight dummy structures, if they do not
     * already have a SV_COUNTER shadow variable, then attach a
     * new one.
     */
    shadow_count = klp_shadow_get_or_alloc(
        d as *mut ::core::ffi::c_void,
        SV_COUNTER,
        ::core::mem::size_of::<i32>(),
        GFP_NOWAIT,
        None,
        ::core::ptr::null_mut(),
    ) as *mut i32;
    if !shadow_count.is_null() {
        *shadow_count = (*shadow_count).wrapping_add(1);
    }

    time_after(jiffies, (*d).jiffies_expire)
}

unsafe extern "C" fn livepatch_fix2_dummy_leak_dtor(
    obj: *mut ::core::ffi::c_void,
    shadow_data: *mut ::core::ffi::c_void,
) {
    let d = obj;
    let shadow_leak = shadow_data as *mut *mut i32;

    // pr_info("%s: dummy @ %p, prevented leak @ %p\n", __func__, d, *shadow_leak);
    kfree(*shadow_leak as *mut ::core::ffi::c_void);
}

unsafe fn livepatch_fix2_dummy_free(d: *mut dummy) {
    let shadow_leak: *mut *mut i32;
    let shadow_count: *mut i32;

    /* Patch: copy the memory leak patch from the fix1 module. */
    shadow_leak = klp_shadow_get(d as *mut ::core::ffi::c_void, SV_LEAK) as *mut *mut i32;
    if !shadow_leak.is_null() {
        klp_shadow_free(
            d as *mut ::core::ffi::c_void,
            SV_LEAK,
            Some(livepatch_fix2_dummy_leak_dtor),
        );
    } else {
        // pr_info("%s: dummy @ %p leaked!\n", __func__, d);
    }

    /*
     * Patch: fetch the SV_COUNTER shadow variable and display
     * the final count.  Detach the shadow variable.
     */
    shadow_count = klp_shadow_get(d as *mut ::core::ffi::c_void, SV_COUNTER) as *mut i32;
    if !shadow_count.is_null() {
        // pr_info("%s: dummy @ %p, check counter = %d\n", __func__, d, *shadow_count);
        klp_shadow_free(d as *mut ::core::ffi::c_void, SV_COUNTER, None);
    }

    kfree(d as *mut ::core::ffi::c_void);
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const ::core::ffi::c_char,
    pub new_func: *const ::core::ffi::c_void,
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

#[no_mangle]
pub static mut funcs: [klp_func; 3] = [
    klp_func { old_name: b"dummy_check\0".as_ptr() as *const _, new_func: livepatch_fix2_dummy_check as *const () as *const _ },
    klp_func { old_name: b"dummy_free\0".as_ptr() as *const _, new_func: livepatch_fix2_dummy_free as *const () as *const _ },
    klp_func { old_name: ::core::ptr::null(), new_func: ::core::ptr::null() },
];

#[no_mangle]
pub static mut objs: [klp_object; 2] = [
    klp_object { name: b"livepatch_shadow_mod\0".as_ptr() as *const _, funcs: funcs.as_ptr() as *mut _ },
    klp_object { name: ::core::ptr::null(), funcs: ::core::ptr::null_mut() },
];

#[no_mangle]
pub static mut patch: klp_patch = klp_patch {
    module: unsafe { THIS_MODULE },
    objs: objs.as_ptr() as *mut _,
};

unsafe fn livepatch_shadow_fix2_init() -> i32 {
    klp_enable_patch(&mut patch)
}

unsafe fn livepatch_shadow_fix2_exit() {
    /* Cleanup any existing SV_COUNTER shadow variables */
    klp_shadow_free_all(SV_COUNTER, None);
}

unsafe extern "C" {
    fn klp_enable_patch(patch: *mut klp_patch) -> i32;
}

// module_init(livepatch_shadow_fix2_init);
// module_exit(livepatch_shadow_fix2_exit);
// MODULE_DESCRIPTION("Live patching demo for shadow variables");
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
