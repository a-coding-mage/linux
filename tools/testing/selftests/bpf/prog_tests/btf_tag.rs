// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// Dependencies from the original C file:
// <test_progs.h>, <bpf/btf.h>, "test_btf_decl_tag.skel.h",
// "btf_type_tag.skel.h", "btf_type_tag_user.skel.h",
// and "btf_type_tag_percpu.skel.h".

use core::ffi::{c_char, c_int};
use core::ptr;

const BTF_KIND_TYPE_TAG: u32 = 18;

#[repr(C)]
pub struct btf_type_tag_test {
    pub p: *mut *mut c_int,
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_btf_decl_tag {
    pub rodata: *mut test_btf_decl_tag_rodata,
}

#[repr(C)]
pub struct test_btf_decl_tag_rodata {
    pub skip_tests: bool,
}

#[repr(C)]
pub struct btf_type_tag {
    pub rodata: *mut btf_type_tag_rodata,
}

#[repr(C)]
pub struct btf_type_tag_rodata {
    pub skip_tests: bool,
}

#[repr(C)]
pub struct btf_type_tag_user {
    pub progs: btf_type_tag_user_progs,
}

#[repr(C)]
pub struct btf_type_tag_user_progs {
    pub test_sys_getsockname: *mut bpf_program,
    pub test_user2: *mut bpf_program,
    pub test_user1: *mut bpf_program,
}

#[repr(C)]
pub struct btf_type_tag_percpu {
    pub progs: btf_type_tag_percpu_progs,
}

#[repr(C)]
pub struct btf_type_tag_percpu_progs {
    pub test_percpu_load: *mut bpf_program,
    pub test_percpu_helper: *mut bpf_program,
    pub test_percpu2: *mut bpf_program,
    pub test_percpu1: *mut bpf_program,
}

#[repr(C)]
pub struct test_env {
    pub has_testmod: bool,
}

unsafe extern "C" {
    static env: test_env;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);

    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__load_module_btf(module_name: *const c_char, vmlinux_btf: *mut btf) -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: u32) -> i32;
    fn btf__free(btf: *mut btf);

    fn test_btf_decl_tag__open_and_load() -> *mut test_btf_decl_tag;
    fn test_btf_decl_tag__destroy(skel: *mut test_btf_decl_tag);

    fn btf_type_tag__open_and_load() -> *mut btf_type_tag;
    fn btf_type_tag__destroy(skel: *mut btf_type_tag);

    fn btf_type_tag_user__open() -> *mut btf_type_tag_user;
    fn btf_type_tag_user__load(skel: *mut btf_type_tag_user) -> c_int;
    fn btf_type_tag_user__destroy(skel: *mut btf_type_tag_user);

    fn btf_type_tag_percpu__open() -> *mut btf_type_tag_percpu;
    fn btf_type_tag_percpu__load(skel: *mut btf_type_tag_percpu) -> c_int;
    fn btf_type_tag_percpu__destroy(skel: *mut btf_type_tag_percpu);
}

unsafe fn test_btf_decl_tag() {
    let skel: *mut test_btf_decl_tag;

    skel = test_btf_decl_tag__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"btf_decl_tag".as_ptr()) {
        return;
    }

    if (*(*skel).rodata).skip_tests {
        printf(
            c"%s:SKIP: btf_decl_tag attribute not supported".as_ptr(),
            c"test_btf_decl_tag".as_ptr(),
        );
        test__skip();
    }

    test_btf_decl_tag__destroy(skel);
}

unsafe fn test_btf_type_tag() {
    let skel: *mut btf_type_tag;

    skel = btf_type_tag__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"btf_type_tag".as_ptr()) {
        return;
    }

    if (*(*skel).rodata).skip_tests {
        printf(
            c"%s:SKIP: btf_type_tag attribute not supported".as_ptr(),
            c"test_btf_type_tag".as_ptr(),
        );
        test__skip();
    }

    btf_type_tag__destroy(skel);
}

/* loads vmlinux_btf as well as module_btf. If the caller passes NULL as
 * module_btf, it will not load module btf.
 *
 * Returns 0 on success.
 * Return -1 On error. In case of error, the loaded btf will be freed and the
 * input parameters will be set to pointing to NULL.
 */
unsafe fn load_btfs(
    vmlinux_btf: *mut *mut btf,
    module_btf: *mut *mut btf,
    needs_vmlinux_tag: bool,
) -> c_int {
    let module_name = c"bpf_testmod".as_ptr();
    let mut type_id: i32;

    if !env.has_testmod {
        test__skip();
        return -1;
    }

    *vmlinux_btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR((*vmlinux_btf).cast(), c"could not load vmlinux BTF".as_ptr()) {
        return -1;
    }

    if needs_vmlinux_tag {
        /* skip the test if the vmlinux does not have __user tags */
        type_id = btf__find_by_name_kind(*vmlinux_btf, c"user".as_ptr(), BTF_KIND_TYPE_TAG);
        if type_id <= 0 {
            printf(
                c"%s:SKIP: btf_type_tag attribute not in vmlinux btf".as_ptr(),
                c"load_btfs".as_ptr(),
            );
            test__skip();
            btf__free(*vmlinux_btf);

            *vmlinux_btf = ptr::null_mut();
            if !module_btf.is_null() {
                *module_btf = ptr::null_mut();
            }
            return -1;
        }
    }

    /* skip loading module_btf, if not requested by caller */
    if module_btf.is_null() {
        return 0;
    }

    *module_btf = btf__load_module_btf(module_name, *vmlinux_btf);
    if !ASSERT_OK_PTR((*module_btf).cast(), c"could not load module BTF".as_ptr()) {
        btf__free(*vmlinux_btf);

        *vmlinux_btf = ptr::null_mut();
        if !module_btf.is_null() {
            *module_btf = ptr::null_mut();
        }
        return -1;
    }

    /* skip the test if the module does not have __user tags */
    type_id = btf__find_by_name_kind(*module_btf, c"user".as_ptr(), BTF_KIND_TYPE_TAG);
    if type_id <= 0 {
        printf(
            c"%s:SKIP: btf_type_tag attribute not in %s".as_ptr(),
            c"load_btfs".as_ptr(),
            module_name,
        );
        test__skip();
        btf__free(*module_btf);
        btf__free(*vmlinux_btf);

        *vmlinux_btf = ptr::null_mut();
        if !module_btf.is_null() {
            *module_btf = ptr::null_mut();
        }
        return -1;
    }

    0
}

unsafe fn test_btf_type_tag_mod_user(load_test_user1: bool) {
    let mut vmlinux_btf: *mut btf = ptr::null_mut();
    let mut module_btf: *mut btf = ptr::null_mut();
    let skel: *mut btf_type_tag_user;
    let err: c_int;

    if load_btfs(&mut vmlinux_btf, &mut module_btf, false) != 0 {
        return;
    }

    skel = btf_type_tag_user__open();
    if !ASSERT_OK_PTR(skel.cast(), c"btf_type_tag_user".as_ptr()) {
        btf__free(module_btf);
        btf__free(vmlinux_btf);
        return;
    }

    bpf_program__set_autoload((*skel).progs.test_sys_getsockname, false);
    if load_test_user1 {
        bpf_program__set_autoload((*skel).progs.test_user2, false);
    } else {
        bpf_program__set_autoload((*skel).progs.test_user1, false);
    }

    err = btf_type_tag_user__load(skel);
    ASSERT_ERR(err, c"btf_type_tag_user".as_ptr());

    btf_type_tag_user__destroy(skel);

    btf__free(module_btf);
    btf__free(vmlinux_btf);
}

unsafe fn test_btf_type_tag_vmlinux_user() {
    let skel: *mut btf_type_tag_user;
    let mut vmlinux_btf: *mut btf = ptr::null_mut();
    let err: c_int;

    if load_btfs(&mut vmlinux_btf, ptr::null_mut(), true) != 0 {
        return;
    }

    skel = btf_type_tag_user__open();
    if !ASSERT_OK_PTR(skel.cast(), c"btf_type_tag_user".as_ptr()) {
        btf__free(vmlinux_btf);
        return;
    }

    bpf_program__set_autoload((*skel).progs.test_user2, false);
    bpf_program__set_autoload((*skel).progs.test_user1, false);

    err = btf_type_tag_user__load(skel);
    ASSERT_ERR(err, c"btf_type_tag_user".as_ptr());

    btf_type_tag_user__destroy(skel);

    btf__free(vmlinux_btf);
}

unsafe fn test_btf_type_tag_mod_percpu(load_test_percpu1: bool) {
    let mut vmlinux_btf: *mut btf = ptr::null_mut();
    let mut module_btf: *mut btf = ptr::null_mut();
    let skel: *mut btf_type_tag_percpu;
    let err: c_int;

    if load_btfs(&mut vmlinux_btf, &mut module_btf, false) != 0 {
        return;
    }

    skel = btf_type_tag_percpu__open();
    if !ASSERT_OK_PTR(skel.cast(), c"btf_type_tag_percpu".as_ptr()) {
        btf__free(module_btf);
        btf__free(vmlinux_btf);
        return;
    }

    bpf_program__set_autoload((*skel).progs.test_percpu_load, false);
    bpf_program__set_autoload((*skel).progs.test_percpu_helper, false);
    if load_test_percpu1 {
        bpf_program__set_autoload((*skel).progs.test_percpu2, false);
    } else {
        bpf_program__set_autoload((*skel).progs.test_percpu1, false);
    }

    err = btf_type_tag_percpu__load(skel);
    ASSERT_ERR(err, c"btf_type_tag_percpu".as_ptr());

    btf_type_tag_percpu__destroy(skel);

    btf__free(module_btf);
    btf__free(vmlinux_btf);
}

unsafe fn test_btf_type_tag_vmlinux_percpu(load_test: bool) {
    let skel: *mut btf_type_tag_percpu;
    let mut vmlinux_btf: *mut btf = ptr::null_mut();
    let err: c_int;

    if load_btfs(&mut vmlinux_btf, ptr::null_mut(), true) != 0 {
        return;
    }

    skel = btf_type_tag_percpu__open();
    if !ASSERT_OK_PTR(skel.cast(), c"btf_type_tag_percpu".as_ptr()) {
        btf__free(vmlinux_btf);
        return;
    }

    bpf_program__set_autoload((*skel).progs.test_percpu2, false);
    bpf_program__set_autoload((*skel).progs.test_percpu1, false);
    if load_test {
        bpf_program__set_autoload((*skel).progs.test_percpu_helper, false);

        err = btf_type_tag_percpu__load(skel);
        ASSERT_ERR(err, c"btf_type_tag_percpu_load".as_ptr());
    } else {
        bpf_program__set_autoload((*skel).progs.test_percpu_load, false);

        err = btf_type_tag_percpu__load(skel);
        ASSERT_OK(err, c"btf_type_tag_percpu_helper".as_ptr());
    }

    btf_type_tag_percpu__destroy(skel);

    btf__free(vmlinux_btf);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_btf_tag() {
    if test__start_subtest(c"btf_decl_tag".as_ptr()) {
        test_btf_decl_tag();
    }
    if test__start_subtest(c"btf_type_tag".as_ptr()) {
        test_btf_type_tag();
    }

    if test__start_subtest(c"btf_type_tag_user_mod1".as_ptr()) {
        test_btf_type_tag_mod_user(true);
    }
    if test__start_subtest(c"btf_type_tag_user_mod2".as_ptr()) {
        test_btf_type_tag_mod_user(false);
    }
    if test__start_subtest(c"btf_type_tag_sys_user_vmlinux".as_ptr()) {
        test_btf_type_tag_vmlinux_user();
    }

    if test__start_subtest(c"btf_type_tag_percpu_mod1".as_ptr()) {
        test_btf_type_tag_mod_percpu(true);
    }
    if test__start_subtest(c"btf_type_tag_percpu_mod2".as_ptr()) {
        test_btf_type_tag_mod_percpu(false);
    }
    if test__start_subtest(c"btf_type_tag_percpu_vmlinux_load".as_ptr()) {
        test_btf_type_tag_vmlinux_percpu(true);
    }
    if test__start_subtest(c"btf_type_tag_percpu_vmlinux_helper".as_ptr()) {
        test_btf_type_tag_vmlinux_percpu(false);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
