// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) Meta Platforms, Inc. and affiliates. */

/* Translated from:
 * #include <test_progs.h>
 * #include "test_subskeleton.skel.h"
 * #include "test_subskeleton_lib.subskel.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const EINVAL: c_int = 22;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_subskeleton_lib {
    pub rodata: test_subskeleton_lib__rodata_ptrs,
    pub data: test_subskeleton_lib__data_ptrs,
    pub bss: test_subskeleton_lib__bss_ptrs,
    pub kconfig: test_subskeleton_lib__kconfig_ptrs,
    pub progs: test_subskeleton_lib__progs,
    pub maps: test_subskeleton_lib__maps,
}

#[repr(C)]
pub struct test_subskeleton_lib__rodata_ptrs {
    pub var1: *mut c_int,
}

#[repr(C)]
pub struct test_subskeleton_lib__data_ptrs {
    pub var2: *mut c_int,
    pub var5: *mut c_int,
    pub var6: *mut c_int,
}

#[repr(C)]
pub struct test_subskeleton_lib__bss_ptrs {
    pub var3: *mut test_subskeleton_lib__var3,
    pub libout1: *mut c_int,
}

#[repr(C)]
pub struct test_subskeleton_lib__var3 {
    pub var3_1: c_int,
    pub var3_2: c_int,
}

#[repr(C)]
pub struct test_subskeleton_lib__kconfig_ptrs {
    pub CONFIG_BPF_SYSCALL: *mut bool,
}

#[repr(C)]
pub struct test_subskeleton_lib__progs {
    pub lib_perf_handler: *mut bpf_program,
}

#[repr(C)]
pub struct test_subskeleton_lib__maps {
    pub map1: *mut bpf_map,
}

#[repr(C)]
pub struct test_subskeleton {
    pub obj: *mut bpf_object,
    pub rodata: *mut test_subskeleton__rodata,
    pub bss: *mut test_subskeleton__bss,
}

#[repr(C)]
pub struct test_subskeleton__rodata {
    pub rovar1: c_int,
    pub var1: c_int,
}

#[repr(C)]
pub struct test_subskeleton__bss {
    pub out1: c_int,
}

unsafe extern "C" {
    fn test_subskeleton_lib__open(obj: *mut bpf_object) -> *mut test_subskeleton_lib;
    fn test_subskeleton_lib__destroy(lib: *mut test_subskeleton_lib);

    fn test_subskeleton__open() -> *mut test_subskeleton;
    fn test_subskeleton__load(skel: *mut test_subskeleton) -> c_int;
    fn test_subskeleton__attach(skel: *mut test_subskeleton) -> c_int;
    fn test_subskeleton__destroy(skel: *mut test_subskeleton);
    fn test_subskeleton__elf_bytes(sz: *mut usize) -> *const c_void;

    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_map__name(map: *const bpf_map) -> *const c_char;
    fn bpf_object__open_mem(
        obj_buf: *const c_void,
        obj_buf_sz: usize,
        opts: *const c_void,
    ) -> *mut bpf_object;
    fn bpf_object__find_map_by_name(
        obj: *const bpf_object,
        name: *const c_char,
    ) -> *const bpf_map;
    fn bpf_map__initial_value(map: *const bpf_map, psize: *mut usize) -> *mut c_void;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *const bpf_object,
        name: *const c_char,
    ) -> *const bpf_program;
    fn bpf_program__attach(prog: *const bpf_program) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__close(obj: *mut bpf_object);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn usleep(usec: c_uint) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_TRUE(value: bool, name: *const c_char) -> bool;
}

unsafe fn subskeleton_lib_setup(obj: *mut bpf_object) {
    let lib: *mut test_subskeleton_lib = test_subskeleton_lib__open(obj);

    if !ASSERT_OK_PTR(lib.cast::<c_void>(), c"open subskeleton".as_ptr()) {
        return;
    }

    *(*lib).rodata.var1 = 1;
    *(*lib).data.var2 = 2;
    (*(*lib).bss.var3).var3_1 = 3;
    (*(*lib).bss.var3).var3_2 = 4;

    test_subskeleton_lib__destroy(lib);
}

unsafe fn subskeleton_lib_subresult(obj: *mut bpf_object) -> c_int {
    let lib: *mut test_subskeleton_lib = test_subskeleton_lib__open(obj);
    let result: c_int;

    if !ASSERT_OK_PTR(lib.cast::<c_void>(), c"open subskeleton".as_ptr()) {
        return -EINVAL;
    }

    result = *(*lib).bss.libout1;
    ASSERT_EQ(result, 1 + 2 + 3 + 4 + 5 + 6, c"lib subresult".as_ptr());

    ASSERT_OK_PTR(
        (*lib).progs.lib_perf_handler.cast::<c_void>(),
        c"lib_perf_handler".as_ptr(),
    );
    ASSERT_STREQ(
        bpf_program__name((*lib).progs.lib_perf_handler),
        c"lib_perf_handler".as_ptr(),
        c"program name".as_ptr(),
    );

    ASSERT_OK_PTR((*lib).maps.map1.cast::<c_void>(), c"map1".as_ptr());
    ASSERT_STREQ(
        bpf_map__name((*lib).maps.map1),
        c"map1".as_ptr(),
        c"map name".as_ptr(),
    );

    ASSERT_EQ(*(*lib).data.var5, 5, c"__weak var5".as_ptr());
    ASSERT_EQ(*(*lib).data.var6, 6, c"extern var6".as_ptr());
    ASSERT_TRUE(
        *(*lib).kconfig.CONFIG_BPF_SYSCALL,
        c"CONFIG_BPF_SYSCALL".as_ptr(),
    );

    test_subskeleton_lib__destroy(lib);
    result
}

/* initialize and load through skeleton, then instantiate subskeleton out of it */
unsafe fn subtest_skel_subskeleton() {
    let mut err: c_int;
    let result: c_int;
    let skel: *mut test_subskeleton;

    skel = test_subskeleton__open();
    if !ASSERT_OK_PTR(skel.cast::<c_void>(), c"skel_open".as_ptr()) {
        return;
    }

    (*(*skel).rodata).rovar1 = 10;
    (*(*skel).rodata).var1 = 1;
    subskeleton_lib_setup((*skel).obj);

    err = test_subskeleton__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        test_subskeleton__destroy(skel);
        return;
    }

    err = test_subskeleton__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        test_subskeleton__destroy(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    result = subskeleton_lib_subresult((*skel).obj) * 10;
    ASSERT_EQ((*(*skel).bss).out1, result, c"unexpected calculation".as_ptr());

    test_subskeleton__destroy(skel);
}

/* initialize and load through generic bpf_object API, then instantiate subskeleton out of it */
unsafe fn subtest_obj_subskeleton() {
    let mut err: c_int;
    let result: c_int;
    let elf_bytes: *const c_void;
    let mut elf_bytes_sz: usize = 0;
    let mut rodata_sz: usize = 0;
    let mut bss_sz: usize = 0;
    let obj: *mut bpf_object;
    let mut map: *const bpf_map;
    let prog: *const bpf_program;
    let mut link: *mut bpf_link = ptr::null_mut();
    let rodata: *mut test_subskeleton__rodata;
    let bss: *mut test_subskeleton__bss;

    elf_bytes = test_subskeleton__elf_bytes(&mut elf_bytes_sz);
    if !ASSERT_OK_PTR(elf_bytes, c"elf_bytes".as_ptr()) {
        return;
    }

    obj = bpf_object__open_mem(elf_bytes, elf_bytes_sz, ptr::null());
    if !ASSERT_OK_PTR(obj.cast::<c_void>(), c"obj_open_mem".as_ptr()) {
        return;
    }

    map = bpf_object__find_map_by_name(obj, c".rodata".as_ptr());
    if !ASSERT_OK_PTR(map.cast::<c_void>(), c"rodata_map_by_name".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    rodata = bpf_map__initial_value(map, &mut rodata_sz).cast::<test_subskeleton__rodata>();
    if !ASSERT_OK_PTR(rodata.cast::<c_void>(), c"rodata_get".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    (*rodata).rovar1 = 10;
    (*rodata).var1 = 1;
    subskeleton_lib_setup(obj);

    err = bpf_object__load(obj);
    if !ASSERT_OK(err, c"obj_load".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    prog = bpf_object__find_program_by_name(obj, c"handler1".as_ptr());
    if !ASSERT_OK_PTR(prog.cast::<c_void>(), c"prog_by_name".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    link = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(link.cast::<c_void>(), c"prog_attach".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    map = bpf_object__find_map_by_name(obj, c".bss".as_ptr());
    if !ASSERT_OK_PTR(map.cast::<c_void>(), c"bss_map_by_name".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    bss = bpf_map__initial_value(map, &mut bss_sz).cast::<test_subskeleton__bss>();
    if !ASSERT_OK_PTR(rodata.cast::<c_void>(), c"rodata_get".as_ptr()) {
        bpf_link__destroy(link);
        bpf_object__close(obj);
        return;
    }

    result = subskeleton_lib_subresult(obj) * 10;
    ASSERT_EQ((*bss).out1, result, c"out1".as_ptr());

    bpf_link__destroy(link);
    bpf_object__close(obj);
}

#[no_mangle]
pub unsafe extern "C" fn test_subskeleton() {
    if test__start_subtest(c"skel_subskel".as_ptr()) {
        subtest_skel_subskeleton();
    }
    if test__start_subtest(c"obj_subskel".as_ptr()) {
        subtest_obj_subskeleton();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
