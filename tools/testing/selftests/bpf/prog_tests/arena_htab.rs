// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies translated as external declarations:
 * <test_progs.h>, <sys/mman.h>, <network_helpers.h>, <sys/user.h>,
 * <unistd.h> for PAGE_SIZE/getpagesize() on some archs,
 * "arena_htab_asm.skel.h", "arena_htab.skel.h", and "bpf_arena_htab.h".
 */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn printf(fmt: *const c_char, ...) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char);
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char);
    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn htab_lookup_elem(htab: *mut htab, key: c_int) -> c_int;

    fn arena_htab__open_and_load() -> *mut arena_htab;
    fn arena_htab__destroy(skel: *mut arena_htab);
    fn arena_htab_asm__open_and_load() -> *mut arena_htab_asm;
    fn arena_htab_asm__destroy(skel: *mut arena_htab_asm);

    fn bpf_map__initial_value(map: *mut bpf_map, psize: *mut usize) -> *mut c_void;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
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
pub struct bpf_test_run_opts {
    pub retval: c_int,
}

#[repr(C)]
pub struct htab {
    pub buckets: *mut c_void,
    pub n_buckets: c_int,
}

#[repr(C)]
pub struct arena_htab_maps {
    pub arena: *mut bpf_map,
}

#[repr(C)]
pub struct arena_htab_progs {
    pub arena_htab_llvm: *mut bpf_program,
}

#[repr(C)]
pub struct arena_htab_bss {
    pub skip: bool,
    pub htab_for_user: *mut htab,
}

#[repr(C)]
pub struct arena_htab {
    pub maps: arena_htab_maps,
    pub progs: arena_htab_progs,
    pub bss: *mut arena_htab_bss,
}

#[repr(C)]
pub struct arena_htab_asm_progs {
    pub arena_htab_asm: *mut bpf_program,
}

#[repr(C)]
pub struct arena_htab_asm_bss {
    pub htab_for_user: *mut htab,
}

#[repr(C)]
pub struct arena_htab_asm {
    pub progs: arena_htab_asm_progs,
    pub bss: *mut arena_htab_asm_bss,
}

unsafe fn test_arena_htab_common(htab: *mut htab) {
    let mut i: c_int;

    printf(
        b"htab %p buckets %p n_buckets %d\n\0".as_ptr() as *const c_char,
        htab,
        (*htab).buckets,
        (*htab).n_buckets,
    );
    ASSERT_OK_PTR(
        (*htab).buckets as *const c_void,
        b"htab->buckets shouldn't be NULL\0".as_ptr() as *const c_char,
    );
    i = 0;
    while !(*htab).buckets.is_null() && i < 16 {
        /*
         * Walk htab buckets and link lists since all pointers are correct,
         * though they were written by bpf program.
         */
        let val: c_int = htab_lookup_elem(htab, i);

        ASSERT_EQ(i, val, b"key == value\0".as_ptr() as *const c_char);
        i += 4;
    }
}

unsafe fn test_arena_htab_llvm() {
    let mut opts: bpf_test_run_opts = core::mem::zeroed();
    let mut skel: *mut arena_htab;
    let htab: *mut htab;
    let mut arena_sz: usize = 0;
    let area: *mut c_void;
    let ret: c_int;

    skel = arena_htab__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"arena_htab__open_and_load\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    area = bpf_map__initial_value((*skel).maps.arena, &mut arena_sz);
    /* fault-in a page with pgoff == 0 as sanity check */
    core::ptr::write_volatile(area as *mut c_int, 0x55aa);

    /* bpf prog will allocate more pages */
    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.arena_htab_llvm), &mut opts);
    ASSERT_OK(ret, b"ret\0".as_ptr() as *const c_char);
    ASSERT_OK(opts.retval, b"retval\0".as_ptr() as *const c_char);
    if (*(*skel).bss).skip {
        printf(
            b"%s:SKIP:compiler doesn't support arena_cast\n\0".as_ptr() as *const c_char,
            b"test_arena_htab_llvm\0".as_ptr() as *const c_char,
        );
        test__skip();
    } else {
        htab = (*(*skel).bss).htab_for_user;
        test_arena_htab_common(htab);
    }
    arena_htab__destroy(skel);
}

unsafe fn test_arena_htab_asm() {
    let mut opts: bpf_test_run_opts = core::mem::zeroed();
    let mut skel: *mut arena_htab_asm;
    let htab: *mut htab;
    let ret: c_int;

    skel = arena_htab_asm__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"arena_htab_asm__open_and_load\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.arena_htab_asm), &mut opts);
    ASSERT_OK(ret, b"ret\0".as_ptr() as *const c_char);
    ASSERT_OK(opts.retval, b"retval\0".as_ptr() as *const c_char);
    htab = (*(*skel).bss).htab_for_user;
    test_arena_htab_common(htab);
    arena_htab_asm__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_htab() {
    if test__start_subtest(b"arena_htab_llvm\0".as_ptr() as *const c_char) {
        test_arena_htab_llvm();
    }
    if test__start_subtest(b"arena_htab_asm\0".as_ptr() as *const c_char) {
        test_arena_htab_asm();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
