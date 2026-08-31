// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2021 Hengqi Chen

// Original C dependencies:
// #include <test_progs.h>
// #include <bpf/btf.h>

use core::ffi::{c_char, c_int};

static MODULE_NAME: &[u8] = b"bpf_testmod\0";
static SYMBOL_NAME: &[u8] = b"bpf_testmod_test_read\0";

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct env {
    pub has_testmod: bool,
}

extern "C" {
    static mut env: env;

    fn test__skip();
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__load_module_btf(module_name: *const c_char, vmlinux_btf: *mut btf) -> *mut btf;
    fn btf__find_by_name(btf: *const btf, name: *const c_char) -> i32;
    fn btf__free(btf: *mut btf);

    fn ASSERT_OK_PTR(ptr: *const btf, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: i32, expected: i32, name: *const c_char) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_btf_module() {
    let mut vmlinux_btf: *mut btf;
    let mut module_btf: *mut btf = core::ptr::null_mut();
    let type_id: i32;

    if !env.has_testmod {
        test__skip();
        return;
    }

    vmlinux_btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(
        vmlinux_btf as *const btf,
        c"could not load vmlinux BTF".as_ptr(),
    ) {
        return;
    }

    module_btf = btf__load_module_btf(MODULE_NAME.as_ptr() as *const c_char, vmlinux_btf);
    if !ASSERT_OK_PTR(
        module_btf as *const btf,
        c"could not load module BTF".as_ptr(),
    ) {
        btf__free(vmlinux_btf);
        return;
    }

    type_id = btf__find_by_name(module_btf, SYMBOL_NAME.as_ptr() as *const c_char);
    ASSERT_GT(type_id, 0, c"func not found".as_ptr());

    btf__free(module_btf);
    btf__free(vmlinux_btf);
}
