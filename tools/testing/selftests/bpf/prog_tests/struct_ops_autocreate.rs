// SPDX-License-Identifier: GPL-2.0

// Translated from testing/selftests/bpf/prog_tests/struct_ops_autocreate.c.
// Dependencies from test_progs.h and the generated skeleton headers are kept as
// external declarations.

use core::ffi::{c_char, c_int};
use core::ptr;

const ENOTSUP: c_int = 95;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct struct_ops_autocreate_maps {
    pub testmod_1: *mut bpf_map,
    pub testmod_2: *mut bpf_map,
    pub optional_map: *mut bpf_map,
    pub optional_map2: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_autocreate_progs {
    pub test_1: *mut bpf_program,
    pub test_2: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_autocreate_bss {
    pub test_1_result: c_int,
}

#[repr(C)]
pub struct struct_ops_autocreate {
    pub maps: struct_ops_autocreate_maps,
    pub progs: struct_ops_autocreate_progs,
    pub bss: *mut struct_ops_autocreate_bss,
}

#[repr(C)]
pub struct bpf_testmod_ops {
    pub test_1: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_autocreate2_struct_ops {
    pub testmod_1: *mut bpf_testmod_ops,
}

#[repr(C)]
pub struct struct_ops_autocreate2_maps {
    pub testmod_1: *mut bpf_map,
}

#[repr(C)]
pub struct struct_ops_autocreate2_progs {
    pub foo: *mut bpf_program,
    pub bar: *mut bpf_program,
}

#[repr(C)]
pub struct struct_ops_autocreate2_bss {
    pub test_1_result: c_int,
}

#[repr(C)]
pub struct struct_ops_autocreate2 {
    pub maps: struct_ops_autocreate2_maps,
    pub progs: struct_ops_autocreate2_progs,
    pub bss: *mut struct_ops_autocreate2_bss,
    pub struct_ops: struct_ops_autocreate2_struct_ops,
}

unsafe extern "C" {
    fn free(ptr: *mut core::ffi::c_void);

    fn ASSERT_OK_PTR(ptr: *const core::ffi::c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> c_int;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_FALSE(actual: bool, name: *const c_char) -> bool;

    fn start_libbpf_log_capture() -> c_int;
    fn stop_libbpf_log_capture() -> *mut c_char;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn struct_ops_autocreate__open() -> *mut struct_ops_autocreate;
    fn struct_ops_autocreate__load(skel: *mut struct_ops_autocreate) -> c_int;
    fn struct_ops_autocreate__destroy(skel: *mut struct_ops_autocreate);

    fn struct_ops_autocreate2__open() -> *mut struct_ops_autocreate2;
    fn struct_ops_autocreate2__load(skel: *mut struct_ops_autocreate2) -> c_int;
    fn struct_ops_autocreate2__destroy(skel: *mut struct_ops_autocreate2);

    fn bpf_map__attach_struct_ops(map: *mut bpf_map) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool) -> c_int;
    fn bpf_map__autocreate(map: *mut bpf_map) -> bool;
    fn bpf_program__autoload(prog: *mut bpf_program) -> bool;
}

unsafe fn cant_load_full_object() {
    let mut log: *mut c_char = ptr::null_mut();
    let err: c_int;

    let skel = struct_ops_autocreate__open();
    if !ASSERT_OK_PTR(
        skel as *const core::ffi::c_void,
        c"struct_ops_autocreate__open".as_ptr(),
    ) {
        return;
    }

    if start_libbpf_log_capture() != 0 {
        free(log as *mut core::ffi::c_void);
        struct_ops_autocreate__destroy(skel);
        return;
    }

    /* The testmod_2 map BTF type (struct bpf_testmod_ops___v2) doesn't
     * match the BTF of the actual struct bpf_testmod_ops defined in the
     * kernel, so we should fail to load it if we don't disable autocreate
     * for that map.
     */
    err = struct_ops_autocreate__load(skel);
    log = stop_libbpf_log_capture();
    if !ASSERT_ERR(err, c"struct_ops_autocreate__load".as_ptr()) {
        free(log as *mut core::ffi::c_void);
        struct_ops_autocreate__destroy(skel);
        return;
    }

    ASSERT_HAS_SUBSTR(
        log,
        c"libbpf: struct_ops init_kern".as_ptr(),
        c"init_kern message".as_ptr(),
    );
    ASSERT_EQ(err, -ENOTSUP, c"errno should be ENOTSUP".as_ptr());

    free(log as *mut core::ffi::c_void);
    struct_ops_autocreate__destroy(skel);
}

unsafe fn check_test_1_link(
    skel: *mut struct_ops_autocreate,
    _map: *mut bpf_map,
) -> c_int {
    let err: c_int;

    let link = bpf_map__attach_struct_ops((*skel).maps.testmod_1);
    if !ASSERT_OK_PTR(
        link as *const core::ffi::c_void,
        c"bpf_map__attach_struct_ops".as_ptr(),
    ) {
        return -1;
    }

    /* test_1() would be called from bpf_dummy_reg2() in bpf_testmod.c */
    err = ASSERT_EQ((*(*skel).bss).test_1_result, 42, c"test_1_result".as_ptr());
    bpf_link__destroy(link);
    err
}

unsafe fn can_load_partial_object() {
    let mut err: c_int;

    let skel = struct_ops_autocreate__open();
    if !ASSERT_OK_PTR(
        skel as *const core::ffi::c_void,
        c"struct_ops_autocreate__open_opts".as_ptr(),
    ) {
        return;
    }

    err = bpf_map__set_autocreate((*skel).maps.testmod_2, false);
    if !ASSERT_OK(err, c"bpf_map__set_autocreate".as_ptr()) {
        struct_ops_autocreate__destroy(skel);
        return;
    }

    ASSERT_TRUE(
        bpf_program__autoload((*skel).progs.test_1),
        c"test_1 default autoload".as_ptr(),
    );
    ASSERT_TRUE(
        bpf_program__autoload((*skel).progs.test_2),
        c"test_2 default autoload".as_ptr(),
    );

    err = struct_ops_autocreate__load(skel);
    if ASSERT_OK(err, c"struct_ops_autocreate__load".as_ptr()) {
        struct_ops_autocreate__destroy(skel);
        return;
    }

    ASSERT_TRUE(
        bpf_program__autoload((*skel).progs.test_1),
        c"test_1 actual autoload".as_ptr(),
    );
    ASSERT_FALSE(
        bpf_program__autoload((*skel).progs.test_2),
        c"test_2 actual autoload".as_ptr(),
    );

    check_test_1_link(skel, (*skel).maps.testmod_1);

    struct_ops_autocreate__destroy(skel);
}

unsafe fn optional_maps() {
    let mut err: c_int;

    let skel = struct_ops_autocreate__open();
    if !ASSERT_OK_PTR(
        skel as *const core::ffi::c_void,
        c"struct_ops_autocreate__open".as_ptr(),
    ) {
        return;
    }

    ASSERT_TRUE(
        bpf_map__autocreate((*skel).maps.testmod_1),
        c"testmod_1 autocreate".as_ptr(),
    );
    ASSERT_TRUE(
        bpf_map__autocreate((*skel).maps.testmod_2),
        c"testmod_2 autocreate".as_ptr(),
    );
    ASSERT_FALSE(
        bpf_map__autocreate((*skel).maps.optional_map),
        c"optional_map autocreate".as_ptr(),
    );
    ASSERT_FALSE(
        bpf_map__autocreate((*skel).maps.optional_map2),
        c"optional_map2 autocreate".as_ptr(),
    );

    err = bpf_map__set_autocreate((*skel).maps.testmod_1, false);
    err |= bpf_map__set_autocreate((*skel).maps.testmod_2, false);
    err |= bpf_map__set_autocreate((*skel).maps.optional_map2, true);
    if !ASSERT_OK(err, c"bpf_map__set_autocreate".as_ptr()) {
        struct_ops_autocreate__destroy(skel);
        return;
    }

    err = struct_ops_autocreate__load(skel);
    if ASSERT_OK(err, c"struct_ops_autocreate__load".as_ptr()) {
        struct_ops_autocreate__destroy(skel);
        return;
    }

    check_test_1_link(skel, (*skel).maps.optional_map2);

    struct_ops_autocreate__destroy(skel);
}

/* Swap test_mod1->test_1 program from 'bar' to 'foo' using shadow vars.
 * test_mod1 load should enable autoload for 'foo'.
 */
unsafe fn autoload_and_shadow_vars() {
    let mut link: *mut bpf_link = ptr::null_mut();
    let err: c_int;

    let skel = struct_ops_autocreate2__open();
    if !ASSERT_OK_PTR(
        skel as *const core::ffi::c_void,
        c"struct_ops_autocreate__open_opts".as_ptr(),
    ) {
        return;
    }

    ASSERT_FALSE(
        bpf_program__autoload((*skel).progs.foo),
        c"foo default autoload".as_ptr(),
    );
    ASSERT_FALSE(
        bpf_program__autoload((*skel).progs.bar),
        c"bar default autoload".as_ptr(),
    );

    /* loading map testmod_1 would switch foo's autoload to true */
    (*(*skel).struct_ops.testmod_1).test_1 = (*skel).progs.foo;

    let load_err = struct_ops_autocreate2__load(skel);
    if ASSERT_OK(load_err, c"struct_ops_autocreate__load".as_ptr()) {
        bpf_link__destroy(link);
        struct_ops_autocreate2__destroy(skel);
        return;
    }

    ASSERT_TRUE(
        bpf_program__autoload((*skel).progs.foo),
        c"foo actual autoload".as_ptr(),
    );
    ASSERT_FALSE(
        bpf_program__autoload((*skel).progs.bar),
        c"bar actual autoload".as_ptr(),
    );

    link = bpf_map__attach_struct_ops((*skel).maps.testmod_1);
    if !ASSERT_OK_PTR(
        link as *const core::ffi::c_void,
        c"bpf_map__attach_struct_ops".as_ptr(),
    ) {
        bpf_link__destroy(link);
        struct_ops_autocreate2__destroy(skel);
        return;
    }

    /* test_1() would be called from bpf_dummy_reg2() in bpf_testmod.c */
    err = ASSERT_EQ((*(*skel).bss).test_1_result, 42, c"test_1_result".as_ptr());
    let _ = err;

    bpf_link__destroy(link);
    struct_ops_autocreate2__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_struct_ops_autocreate() {
    if test__start_subtest(c"cant_load_full_object".as_ptr()) {
        cant_load_full_object();
    }
    if test__start_subtest(c"can_load_partial_object".as_ptr()) {
        can_load_partial_object();
    }
    if test__start_subtest(c"autoload_and_shadow_vars".as_ptr()) {
        autoload_and_shadow_vars();
    }
    if test__start_subtest(c"optional_maps".as_ptr()) {
        optional_maps();
    }
}
