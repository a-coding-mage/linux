// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external declarations:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "local_kptr_stash.skel.h"
// #include "local_kptr_stash_fail.skel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: usize,
    pub repeat: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct local_kptr_stash_progs {
    pub stash_rb_nodes: *mut bpf_program,
    pub stash_plain: *mut bpf_program,
    pub stash_local_with_root: *mut bpf_program,
    pub unstash_rb_node: *mut bpf_program,
    pub refcount_acquire_without_unstash: *mut bpf_program,
    pub stash_refcounted_node: *mut bpf_program,
}

#[repr(C)]
pub struct local_kptr_stash {
    pub progs: local_kptr_stash_progs,
}

unsafe extern "C" {
    // Exact Rust type and size of pkt_v4 are supplied by network_helpers.h in C.
    static pkt_v4: [u8; 0];

    fn local_kptr_stash__open_and_load() -> *mut local_kptr_stash;
    fn local_kptr_stash__destroy(skel: *mut local_kptr_stash);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u32, expected: u32, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn RUN_TESTS_local_kptr_stash_fail();
}

unsafe fn test_local_kptr_stash_simple() {
    let mut opts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4).cast(),
        data_size_in: core::mem::size_of_val(&pkt_v4),
        repeat: 1,
        retval: 0,
    };
    let skel: *mut local_kptr_stash;
    let ret: c_int;

    skel = local_kptr_stash__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast(),
        c"local_kptr_stash__open_and_load".as_ptr(),
    ) {
        return;
    }

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.stash_rb_nodes), &mut opts);
    ASSERT_OK(ret, c"local_kptr_stash_add_nodes run".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"local_kptr_stash_add_nodes retval".as_ptr());

    local_kptr_stash__destroy(skel);
}

unsafe fn test_local_kptr_stash_plain() {
    let mut opts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4).cast(),
        data_size_in: core::mem::size_of_val(&pkt_v4),
        repeat: 1,
        retval: 0,
    };
    let skel: *mut local_kptr_stash;
    let ret: c_int;

    skel = local_kptr_stash__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast(),
        c"local_kptr_stash__open_and_load".as_ptr(),
    ) {
        return;
    }

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.stash_plain), &mut opts);
    ASSERT_OK(ret, c"local_kptr_stash_add_plain run".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"local_kptr_stash_add_plain retval".as_ptr());

    local_kptr_stash__destroy(skel);
}

unsafe fn test_local_kptr_stash_local_with_root() {
    let mut opts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4).cast(),
        data_size_in: core::mem::size_of_val(&pkt_v4),
        repeat: 1,
        retval: 0,
    };
    let skel: *mut local_kptr_stash;
    let ret: c_int;

    skel = local_kptr_stash__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast(),
        c"local_kptr_stash__open_and_load".as_ptr(),
    ) {
        return;
    }

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.stash_local_with_root),
        &mut opts,
    );
    ASSERT_OK(ret, c"local_kptr_stash_add_local_with_root run".as_ptr());
    ASSERT_OK(
        opts.retval as c_int,
        c"local_kptr_stash_add_local_with_root retval".as_ptr(),
    );

    local_kptr_stash__destroy(skel);
}

unsafe fn test_local_kptr_stash_unstash() {
    let mut opts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4).cast(),
        data_size_in: core::mem::size_of_val(&pkt_v4),
        repeat: 1,
        retval: 0,
    };
    let skel: *mut local_kptr_stash;
    let ret: c_int;

    skel = local_kptr_stash__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast(),
        c"local_kptr_stash__open_and_load".as_ptr(),
    ) {
        return;
    }

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.stash_rb_nodes), &mut opts);
    ASSERT_OK(ret, c"local_kptr_stash_add_nodes run".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"local_kptr_stash_add_nodes retval".as_ptr());

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.unstash_rb_node), &mut opts);
    ASSERT_OK(ret, c"local_kptr_stash_add_nodes run".as_ptr());
    ASSERT_EQ(opts.retval, 42, c"local_kptr_stash_add_nodes retval".as_ptr());

    local_kptr_stash__destroy(skel);
}

unsafe fn test_refcount_acquire_without_unstash() {
    let mut opts = bpf_test_run_opts {
        data_in: (&raw const pkt_v4).cast(),
        data_size_in: core::mem::size_of_val(&pkt_v4),
        repeat: 1,
        retval: 0,
    };
    let skel: *mut local_kptr_stash;
    let ret: c_int;

    skel = local_kptr_stash__open_and_load();
    if !ASSERT_OK_PTR(
        skel.cast(),
        c"local_kptr_stash__open_and_load".as_ptr(),
    ) {
        return;
    }

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.refcount_acquire_without_unstash),
        &mut opts,
    );
    ASSERT_OK(ret, c"refcount_acquire_without_unstash run".as_ptr());
    ASSERT_EQ(
        opts.retval,
        2,
        c"refcount_acquire_without_unstash retval".as_ptr(),
    );

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.stash_refcounted_node),
        &mut opts,
    );
    ASSERT_OK(ret, c"stash_refcounted_node run".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"stash_refcounted_node retval".as_ptr());

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.refcount_acquire_without_unstash),
        &mut opts,
    );
    ASSERT_OK(
        ret,
        c"refcount_acquire_without_unstash (2) run".as_ptr(),
    );
    ASSERT_EQ(
        opts.retval,
        42,
        c"refcount_acquire_without_unstash (2) retval".as_ptr(),
    );

    local_kptr_stash__destroy(skel);
}

unsafe fn test_local_kptr_stash_fail() {
    RUN_TESTS_local_kptr_stash_fail();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_local_kptr_stash() {
    if test__start_subtest(c"local_kptr_stash_simple".as_ptr()) {
        test_local_kptr_stash_simple();
    }
    if test__start_subtest(c"local_kptr_stash_plain".as_ptr()) {
        test_local_kptr_stash_plain();
    }
    if test__start_subtest(c"local_kptr_stash_local_with_root".as_ptr()) {
        test_local_kptr_stash_local_with_root();
    }
    if test__start_subtest(c"local_kptr_stash_unstash".as_ptr()) {
        test_local_kptr_stash_unstash();
    }
    if test__start_subtest(c"refcount_acquire_without_unstash".as_ptr()) {
        test_refcount_acquire_without_unstash();
    }
    if test__start_subtest(c"local_kptr_stash_fail".as_ptr()) {
        test_local_kptr_stash_fail();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
