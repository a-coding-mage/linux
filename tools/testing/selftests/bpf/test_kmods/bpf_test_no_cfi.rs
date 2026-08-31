// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include <linux/bpf.h>
// #include <linux/btf.h>
// #include <linux/init.h>
// #include <linux/module.h>

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_member {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_verifier_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_struct_ops {
    verifier_ops: *const bpf_verifier_ops,
    init: Option<unsafe extern "C" fn(*mut btf) -> c_int>,
    init_member: Option<
        unsafe extern "C" fn(
            *const btf_type,
            *const btf_member,
            *mut c_void,
            *const c_void,
        ) -> c_int,
    >,
    reg: Option<unsafe extern "C" fn(*mut c_void, *mut bpf_link) -> c_int>,
    unreg: Option<unsafe extern "C" fn(*mut c_void, *mut bpf_link)>,
    name: *const c_char,
    owner: *mut c_void,
    cfi_stubs: *mut bpf_test_no_cfi_ops,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;

    fn register_bpf_struct_ops(st_ops: *mut bpf_struct_ops, type_: *const c_void) -> c_int;
}

#[repr(C)]
pub struct bpf_test_no_cfi_ops {
    fn_1: Option<unsafe extern "C" fn()>,
    fn_2: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" fn dummy_init(_btf: *mut btf) -> c_int {
    return 0;
}

unsafe extern "C" fn dummy_init_member(
    _t: *const btf_type,
    _member: *const btf_member,
    _kdata: *mut c_void,
    _udata: *const c_void,
) -> c_int {
    return 0;
}

unsafe extern "C" fn dummy_reg(_kdata: *mut c_void, _link: *mut bpf_link) -> c_int {
    return 0;
}

unsafe extern "C" fn dummy_unreg(_kdata: *mut c_void, _link: *mut bpf_link) {}

static dummy_verifier_ops: bpf_verifier_ops = bpf_verifier_ops { _private: [] };

unsafe extern "C" fn bpf_test_no_cfi_ops__fn_1() {}

unsafe extern "C" fn bpf_test_no_cfi_ops__fn_2() {}

static mut __test_no_cif_ops: bpf_test_no_cfi_ops = bpf_test_no_cfi_ops {
    fn_1: Some(bpf_test_no_cfi_ops__fn_1),
    fn_2: Some(bpf_test_no_cfi_ops__fn_2),
};

static mut test_no_cif_ops: bpf_struct_ops = bpf_struct_ops {
    verifier_ops: &dummy_verifier_ops,
    init: Some(dummy_init),
    init_member: Some(dummy_init_member),
    reg: Some(dummy_reg),
    unreg: Some(dummy_unreg),
    name: b"bpf_test_no_cfi_ops\0".as_ptr() as *const c_char,
    owner: unsafe { THIS_MODULE },
    cfi_stubs: core::ptr::null_mut(),
};

unsafe extern "C" fn bpf_test_no_cfi_init() -> c_int {
    let mut ret: c_int;

    ret = register_bpf_struct_ops(
        &raw mut test_no_cif_ops,
        core::ptr::addr_of!(bpf_test_no_cfi_ops) as *const c_void,
    );
    if ret == 0 {
        return -EINVAL;
    }

    test_no_cif_ops.cfi_stubs = &raw mut __test_no_cif_ops;
    ret = register_bpf_struct_ops(
        &raw mut test_no_cif_ops,
        core::ptr::addr_of!(bpf_test_no_cfi_ops) as *const c_void,
    );
    return ret;
}

unsafe extern "C" fn bpf_test_no_cfi_exit() {}

const EINVAL: c_int = 22;

// module_init(bpf_test_no_cfi_init);
// module_exit(bpf_test_no_cfi_exit);

// MODULE_AUTHOR("Kuifeng Lee");
// MODULE_DESCRIPTION("BPF no cfi_stubs test module");
// MODULE_LICENSE("Dual BSD/GPL");
