// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel module for testing static keys.
 *
 * Copyright 2015 Akamai Technologies Inc. All Rights Reserved
 *
 * Authors:
 *      Jason Baron       <jbaron@akamai.com>
 */

// Kernel headers and static-key initialization macros are supplied externally.

#[repr(C)]
pub struct test_key {
    pub init_state: bool,
    pub key: *mut static_key,
    pub test_key: unsafe extern "C" fn() -> bool,
}

// The following types, functions, and static-key objects are supplied by the kernel.
extern "C" {
    static mut old_true_key: static_key;
    static mut old_false_key: static_key;
    static mut true_key: static_key_true;
    static mut false_key: static_key_false;

    static mut base_old_true_key: static_key;
    static mut base_inv_old_true_key: static_key;
    static mut base_old_false_key: static_key;
    static mut base_inv_old_false_key: static_key;

    static mut base_true_key: static_key_true;
    static mut base_inv_true_key: static_key_true;
    static mut base_false_key: static_key_false;
    static mut base_inv_false_key: static_key_false;

    fn static_key_enabled(key: *mut static_key) -> bool;
    fn static_key_disable(key: *mut static_key);
    fn static_key_enable(key: *mut static_key);

    fn static_key_true(key: *mut static_key) -> bool;
    fn static_key_false(key: *mut static_key) -> bool;
    fn static_branch_likely(key: *mut static_key) -> bool;
    fn static_branch_unlikely(key: *mut static_key) -> bool;
}

macro_rules! test_key_func {
    ($key:ident, $branch:ident) => {
        unsafe extern "C" fn $key _$branch() -> bool {
            $branch(core::ptr::addr_of_mut!($key) as *mut static_key)
        }
    };
}

unsafe fn invert_key(key: *mut static_key) {
    if static_key_enabled(key) {
        static_key_disable(key);
    } else {
        static_key_enable(key);
    }
}

unsafe fn invert_keys(keys: *mut test_key, size: i32) {
    let mut previous: *mut static_key = core::ptr::null_mut();
    let mut i = 0;

    while i < size {
        let current = (*keys.add(i as usize)).key;
        if previous != current {
            invert_key(current);
            previous = current;
        }
        i += 1;
    }
}

unsafe fn verify_keys(keys: *mut test_key, size: i32, invert: bool) -> i32 {
    let mut i = 0;

    while i < size {
        let key = &*keys.add(i as usize);
        let ret = static_key_enabled(key.key);
        let init = key.init_state;
        if ret != if invert { !init } else { init } {
            return -22; // -EINVAL
        }
        let ret = (key.test_key)();
        if static_key_enabled(key.key) {
            if !ret {
                return -22; // -EINVAL
            }
        } else if ret {
            return -22; // -EINVAL
        }
        i += 1;
    }
    0
}

test_key_func!(old_true_key, static_key_true);
test_key_func!(old_false_key, static_key_false);
test_key_func!(true_key, static_branch_likely);
test_key_func!(true_key, static_branch_unlikely);
test_key_func!(false_key, static_branch_likely);
test_key_func!(false_key, static_branch_unlikely);
test_key_func!(base_old_true_key, static_key_true);
test_key_func!(base_inv_old_true_key, static_key_true);
test_key_func!(base_old_false_key, static_key_false);
test_key_func!(base_inv_old_false_key, static_key_false);
test_key_func!(base_true_key, static_branch_likely);
test_key_func!(base_true_key, static_branch_unlikely);
test_key_func!(base_inv_true_key, static_branch_likely);
test_key_func!(base_inv_true_key, static_branch_unlikely);
test_key_func!(base_false_key, static_branch_likely);
test_key_func!(base_false_key, static_branch_unlikely);
test_key_func!(base_inv_false_key, static_branch_likely);
test_key_func!(base_inv_false_key, static_branch_unlikely);

unsafe extern "C" fn test_static_key_init() -> i32 {
    let mut static_key_tests: [test_key; 18] = [
        test_key { init_state: true, key: core::ptr::addr_of_mut!(old_true_key), test_key: old_true_key_static_key_true },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(old_false_key), test_key: old_false_key_static_key_false },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(true_key) as *mut static_key, test_key: true_key_static_branch_likely },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(true_key) as *mut static_key, test_key: true_key_static_branch_unlikely },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(false_key) as *mut static_key, test_key: false_key_static_branch_likely },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(false_key) as *mut static_key, test_key: false_key_static_branch_unlikely },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(base_old_true_key), test_key: base_old_true_key_static_key_true },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(base_inv_old_true_key), test_key: base_inv_old_true_key_static_key_true },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(base_old_false_key), test_key: base_old_false_key_static_key_false },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(base_inv_old_false_key), test_key: base_inv_old_false_key_static_key_false },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(base_true_key) as *mut static_key, test_key: base_true_key_static_branch_likely },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(base_true_key) as *mut static_key, test_key: base_true_key_static_branch_unlikely },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(base_inv_true_key) as *mut static_key, test_key: base_inv_true_key_static_branch_likely },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(base_inv_true_key) as *mut static_key, test_key: base_inv_true_key_static_branch_unlikely },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(base_false_key) as *mut static_key, test_key: base_false_key_static_branch_likely },
        test_key { init_state: false, key: core::ptr::addr_of_mut!(base_false_key) as *mut static_key, test_key: base_false_key_static_branch_unlikely },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(base_inv_false_key) as *mut static_key, test_key: base_inv_false_key_static_branch_likely },
        test_key { init_state: true, key: core::ptr::addr_of_mut!(base_inv_false_key) as *mut static_key, test_key: base_inv_false_key_static_branch_unlikely },
    ];

    let size = static_key_tests.len() as i32;
    let mut ret = verify_keys(static_key_tests.as_mut_ptr(), size, false);
    if ret != 0 { return ret; }
    invert_keys(static_key_tests.as_mut_ptr(), size);
    ret = verify_keys(static_key_tests.as_mut_ptr(), size, true);
    if ret != 0 { return ret; }
    invert_keys(static_key_tests.as_mut_ptr(), size);
    ret = verify_keys(static_key_tests.as_mut_ptr(), size, false);
    if ret != 0 { return ret; }
    0
}

unsafe extern "C" fn test_static_key_exit() {}

// module_init(test_static_key_init);
// module_exit(test_static_key_exit);
// MODULE_AUTHOR("Jason Baron <jbaron@akamai.com>");
// MODULE_DESCRIPTION("Kernel module for testing static keys");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
