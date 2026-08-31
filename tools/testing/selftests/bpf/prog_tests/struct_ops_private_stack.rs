// SPDX-License-Identifier: GPL-2.0

// Translated from C source that depends on:
// <test_progs.h>
// "struct_ops_private_stack.skel.h"
// "struct_ops_private_stack_fail.skel.h"
// "struct_ops_private_stack_recur.skel.h"

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct struct_ops_private_stack_maps {
    pub testmod_1: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct struct_ops_private_stack_bss {
    pub val_i: i32,
    pub val_j: i32,
}

#[repr(C)]
pub struct struct_ops_private_stack {
    pub maps: struct_ops_private_stack_maps,
    pub bss: *mut struct_ops_private_stack_bss,
}

#[repr(C)]
pub struct struct_ops_private_stack_fail {
    _private: [u8; 0],
}

#[repr(C)]
pub struct struct_ops_private_stack_recur_maps {
    pub testmod_1: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct struct_ops_private_stack_recur_bss {
    pub val_j: i32,
}

#[repr(C)]
pub struct struct_ops_private_stack_recur {
    pub maps: struct_ops_private_stack_recur_maps,
    pub bss: *mut struct_ops_private_stack_recur_bss,
}

unsafe extern "C" {
    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_OK(err: core::ffi::c_int, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_ERR(err: core::ffi::c_int, name: *const core::ffi::c_char) -> bool;
    fn ASSERT_EQ(
        actual: core::ffi::c_int,
        expected: core::ffi::c_int,
        name: *const core::ffi::c_char,
    ) -> bool;
    fn test__start_subtest(name: *const core::ffi::c_char) -> bool;
    fn test__skip();
    fn trigger_module_test_read(sz: core::ffi::c_int) -> core::ffi::c_int;

    fn struct_ops_private_stack__open() -> *mut struct_ops_private_stack;
    fn struct_ops_private_stack__load(skel: *mut struct_ops_private_stack) -> core::ffi::c_int;
    fn struct_ops_private_stack__destroy(skel: *mut struct_ops_private_stack);

    fn struct_ops_private_stack_fail__open() -> *mut struct_ops_private_stack_fail;
    fn struct_ops_private_stack_fail__load(
        skel: *mut struct_ops_private_stack_fail,
    ) -> core::ffi::c_int;
    fn struct_ops_private_stack_fail__destroy(skel: *mut struct_ops_private_stack_fail);

    fn struct_ops_private_stack_recur__open() -> *mut struct_ops_private_stack_recur;
    fn struct_ops_private_stack_recur__load(
        skel: *mut struct_ops_private_stack_recur,
    ) -> core::ffi::c_int;
    fn struct_ops_private_stack_recur__destroy(skel: *mut struct_ops_private_stack_recur);

    fn bpf_map__attach_struct_ops(map: *mut core::ffi::c_void) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64"
))]
unsafe fn test_private_stack() {
    let skel: *mut struct_ops_private_stack;
    let link: *mut bpf_link;
    let err: core::ffi::c_int;

    skel = struct_ops_private_stack__open();
    if !ASSERT_OK_PTR(
        skel as *const core::ffi::c_void,
        c"struct_ops_private_stack__open".as_ptr(),
    ) {
        return;
    }

    err = struct_ops_private_stack__load(skel);
    if !ASSERT_OK(err, c"struct_ops_private_stack__load".as_ptr()) {
        struct_ops_private_stack__destroy(skel);
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.testmod_1);
    if !ASSERT_OK_PTR(link as *const core::ffi::c_void, c"attach_struct_ops".as_ptr()) {
        struct_ops_private_stack__destroy(skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(256), c"trigger_read".as_ptr());

    ASSERT_EQ((*(*skel).bss).val_i, 3, c"val_i".as_ptr());
    ASSERT_EQ((*(*skel).bss).val_j, 8, c"val_j".as_ptr());

    bpf_link__destroy(link);

    struct_ops_private_stack__destroy(skel);
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64"
))]
unsafe fn test_private_stack_fail() {
    let skel: *mut struct_ops_private_stack_fail;
    let err: core::ffi::c_int;

    skel = struct_ops_private_stack_fail__open();
    if !ASSERT_OK_PTR(
        skel as *const core::ffi::c_void,
        c"struct_ops_private_stack_fail__open".as_ptr(),
    ) {
        return;
    }

    err = struct_ops_private_stack_fail__load(skel);
    ASSERT_ERR(err, c"struct_ops_private_stack_fail__load".as_ptr());

    struct_ops_private_stack_fail__destroy(skel);
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64"
))]
unsafe fn test_private_stack_recur() {
    let skel: *mut struct_ops_private_stack_recur;
    let link: *mut bpf_link;
    let err: core::ffi::c_int;

    skel = struct_ops_private_stack_recur__open();
    if !ASSERT_OK_PTR(
        skel as *const core::ffi::c_void,
        c"struct_ops_private_stack_recur__open".as_ptr(),
    ) {
        return;
    }

    err = struct_ops_private_stack_recur__load(skel);
    if !ASSERT_OK(err, c"struct_ops_private_stack_recur__load".as_ptr()) {
        struct_ops_private_stack_recur__destroy(skel);
        return;
    }

    link = bpf_map__attach_struct_ops((*skel).maps.testmod_1);
    if !ASSERT_OK_PTR(link as *const core::ffi::c_void, c"attach_struct_ops".as_ptr()) {
        struct_ops_private_stack_recur__destroy(skel);
        return;
    }

    ASSERT_OK(trigger_module_test_read(256), c"trigger_read".as_ptr());

    ASSERT_EQ((*(*skel).bss).val_j, 3, c"val_j".as_ptr());

    bpf_link__destroy(link);

    struct_ops_private_stack_recur__destroy(skel);
}

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64"
))]
unsafe fn __test_struct_ops_private_stack() {
    if test__start_subtest(c"private_stack".as_ptr()) {
        test_private_stack();
    }
    if test__start_subtest(c"private_stack_fail".as_ptr()) {
        test_private_stack_fail();
    }
    if test__start_subtest(c"private_stack_recur".as_ptr()) {
        test_private_stack_recur();
    }
}

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64"
)))]
unsafe fn __test_struct_ops_private_stack() {
    test__skip();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_struct_ops_private_stack() {
    __test_struct_ops_private_stack();
}
