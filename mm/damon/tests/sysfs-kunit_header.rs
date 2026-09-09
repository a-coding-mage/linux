/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Data Access Monitor Unit Tests
 */

/* Preserved from: #ifdef CONFIG_DAMON_SYSFS_KUNIT_TEST */
/* Preserved from: #ifndef _DAMON_SYSFS_TEST_H */

/* External kernel/KUnit declarations supplied by other translation units. */
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct damon_ctx {
    _private: [u8; 0],
}
#[repr(C)]
pub struct damon_target {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pid {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct damon_sysfs_regions {
    _private: [u8; 0],
}
#[repr(C)]
pub struct damon_sysfs_target {
    pub pid: c_int,
    pub regions: *mut damon_sysfs_regions,
}
#[repr(C)]
pub struct damon_sysfs_targets {
    pub nr: c_uint,
    pub targets_arr: *mut *mut damon_sysfs_target,
}
#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub test_cases: *mut kunit_case,
}

extern "C" {
    fn find_get_pid(nr: c_int) -> *mut pid;
    fn put_pid(pid: *mut pid);
    fn damon_sysfs_targets_alloc() -> *mut damon_sysfs_targets;
    fn damon_sysfs_target_alloc() -> *mut damon_sysfs_target;
    fn damon_sysfs_regions_alloc() -> *mut damon_sysfs_regions;
    fn damon_new_ctx() -> *mut damon_ctx;
    fn damon_sysfs_add_targets(ctx: *mut damon_ctx, targets: *mut damon_sysfs_targets);
    fn damon_destroy_ctx(ctx: *mut damon_ctx);
    fn kmalloc_objs(size: usize, count: usize) -> *mut *mut damon_sysfs_target;
    fn kfree(ptr: *mut c_void);
    fn kunit_skip(test: *mut kunit, message: *const c_char) -> !;
    fn kunit_expect_eq(test: *mut kunit, left: c_uint, right: c_uint);
    fn damon_for_each_target(ctx: *mut damon_ctx, target: *mut *mut damon_target);
}

unsafe fn nr_damon_targets(ctx: *mut damon_ctx) -> c_uint {
    let mut t: *mut damon_target = core::ptr::null_mut();
    let mut nr_targets: c_uint = 0;

    /* Translation of damon_for_each_target(t, ctx). */
    damon_for_each_target(ctx, &mut t);
    while !t.is_null() {
        nr_targets = nr_targets.wrapping_add(1);
        t = core::ptr::null_mut();
        damon_for_each_target(ctx, &mut t);
    }

    nr_targets
}

unsafe fn __damon_sysfs_test_get_any_pid(min: c_int, max: c_int) -> c_int {
    let mut i = min;

    while i <= max {
        let pid = find_get_pid(i);
        if !pid.is_null() {
            put_pid(pid);
            return i;
        }
        i += 1;
    }
    -1
}

unsafe fn damon_sysfs_test_add_targets(test: *mut kunit) {
    let sysfs_targets = damon_sysfs_targets_alloc();
    if sysfs_targets.is_null() {
        kunit_skip(test, c"sysfs_targets alloc fail".as_ptr());
    }
    (*sysfs_targets).nr = 1;
    (*sysfs_targets).targets_arr = kmalloc_objs(core::mem::size_of::<*mut damon_sysfs_target>(), 1);
    if (*sysfs_targets).targets_arr.is_null() {
        kfree(sysfs_targets.cast());
        kunit_skip(test, c"targets_arr alloc fail".as_ptr());
    }

    let sysfs_target = damon_sysfs_target_alloc();
    if sysfs_target.is_null() {
        kfree((*sysfs_targets).targets_arr.cast());
        kfree(sysfs_targets.cast());
        kunit_skip(test, c"sysfs_target alloc fail".as_ptr());
    }
    (*sysfs_target).pid = __damon_sysfs_test_get_any_pid(12, 100);
    (*sysfs_target).regions = damon_sysfs_regions_alloc();
    if (*sysfs_target).regions.is_null() {
        kfree((*sysfs_targets).targets_arr.cast());
        kfree(sysfs_targets.cast());
        kfree(sysfs_target.cast());
        kunit_skip(test, c"sysfs_regions alloc fail".as_ptr());
    }

    *(*sysfs_targets).targets_arr = sysfs_target;

    let ctx = damon_new_ctx();
    if ctx.is_null() {
        kfree((*sysfs_targets).targets_arr.cast());
        kfree(sysfs_targets.cast());
        kfree((*sysfs_target).regions.cast());
        kfree(sysfs_target.cast());
        kunit_skip(test, c"ctx alloc fail".as_ptr());
    }

    damon_sysfs_add_targets(ctx, sysfs_targets);
    kunit_expect_eq(test, 1, nr_damon_targets(ctx));

    (*sysfs_target).pid = __damon_sysfs_test_get_any_pid((*sysfs_target).pid + 1, 200);
    damon_sysfs_add_targets(ctx, sysfs_targets);
    kunit_expect_eq(test, 2, nr_damon_targets(ctx));

    damon_destroy_ctx(ctx);
    kfree((*sysfs_targets).targets_arr.cast());
    kfree(sysfs_targets.cast());
    kfree((*sysfs_target).regions.cast());
    kfree(sysfs_target.cast());
}

/* KUNIT_CASE(damon_sysfs_test_add_targets), followed by the terminating {}. */
static mut damon_sysfs_test_cases: [*const c_void; 2] = [
    damon_sysfs_test_add_targets as *const c_void,
    core::ptr::null(),
];

static mut damon_sysfs_test_suite: kunit_suite = kunit_suite {
    name: c"damon-sysfs".as_ptr(),
    test_cases: damon_sysfs_test_cases.as_mut_ptr() as *mut kunit_case,
};

/* Translation of kunit_test_suite(damon_sysfs_test_suite). */

/* End of _DAMON_SYSFS_TEST_H and CONFIG_DAMON_SYSFS_KUNIT_TEST conditionals. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
