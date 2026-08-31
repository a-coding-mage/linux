// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */
/* Dependencies from C source:
 * #include <test_progs.h>
 * #include <bpf/btf.h>
 * #include "test_log_fixup.skel.h"
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
enum trunc_type {
    TRUNC_NONE,
    TRUNC_PARTIAL,
    TRUNC_FULL,
}

#[repr(C)]
struct test_log_fixup {
    progs: test_log_fixup__progs,
    maps: test_log_fixup__maps,
}

#[repr(C)]
struct test_log_fixup__progs {
    bad_relo: *mut bpf_program,
    bad_relo_subprog: *mut bpf_program,
    use_missing_map: *mut bpf_program,
    use_missing_kfunc: *mut bpf_program,
}

#[repr(C)]
struct test_log_fixup__maps {
    existing_map: *mut bpf_map,
    missing_map: *mut bpf_map,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct test_env {
    verbosity: c_int,
}

const VERBOSE_NONE: c_int = 0;

unsafe extern "C" {
    static env: test_env;

    fn test_log_fixup__open() -> *mut test_log_fixup;
    fn test_log_fixup__load(skel: *mut test_log_fixup) -> c_int;
    fn test_log_fixup__destroy(skel: *mut test_log_fixup);

    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_size: usize);
    fn bpf_program__set_log_level(prog: *mut bpf_program, log_level: c_int);
    fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool);
    fn bpf_map__autocreate(map: *mut bpf_map) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_NULL(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_FALSE(actual: bool, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn printf(fmt: *const c_char, ...) -> c_int;
}

unsafe fn bad_core_relo(log_buf_size: usize, trunc_type: trunc_type) {
    let mut log_buf = [0 as c_char; 8 * 1024];
    let mut skel: *mut test_log_fixup;
    let err: c_int;

    skel = test_log_fixup__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.bad_relo, true);
    log_buf.fill(0);
    bpf_program__set_log_buf(
        (*skel).progs.bad_relo,
        log_buf.as_mut_ptr(),
        if log_buf_size != 0 { log_buf_size } else { log_buf.len() },
    );
    bpf_program__set_log_level((*skel).progs.bad_relo, 1 | 8); /* BPF_LOG_FIXED to force truncation */

    err = test_log_fixup__load(skel);
    if !ASSERT_ERR(err, c"load_fail".as_ptr()) {
        test_log_fixup__destroy(skel);
        return;
    }

    ASSERT_HAS_SUBSTR(
        log_buf.as_ptr(),
        c"0: <invalid CO-RE relocation>\nfailed to resolve CO-RE relocation <byte_sz> ".as_ptr(),
        c"log_buf_part1".as_ptr(),
    );

    match trunc_type {
        trunc_type::TRUNC_NONE => {
            ASSERT_HAS_SUBSTR(
                log_buf.as_ptr(),
                c"struct task_struct___bad.fake_field (0:1 @ offset 4)\n".as_ptr(),
                c"log_buf_part2".as_ptr(),
            );
            ASSERT_HAS_SUBSTR(
                log_buf.as_ptr(),
                c"max_states_per_insn 0 total_states 0 peak_states 0 mark_read 0\n".as_ptr(),
                c"log_buf_end".as_ptr(),
            );
        }
        trunc_type::TRUNC_PARTIAL => {
            /* we should get full libbpf message patch */
            ASSERT_HAS_SUBSTR(
                log_buf.as_ptr(),
                c"struct task_struct___bad.fake_field (0:1 @ offset 4)\n".as_ptr(),
                c"log_buf_part2".as_ptr(),
            );
            /* we shouldn't get full end of BPF verifier log */
            ASSERT_NULL(
                strstr(
                    log_buf.as_ptr(),
                    c"max_states_per_insn 0 total_states 0 peak_states 0 mark_read 0\n".as_ptr(),
                ) as *const c_void,
                c"log_buf_end".as_ptr(),
            );
        }
        trunc_type::TRUNC_FULL => {
            /* we shouldn't get second part of libbpf message patch */
            ASSERT_NULL(
                strstr(
                    log_buf.as_ptr(),
                    c"struct task_struct___bad.fake_field (0:1 @ offset 4)\n".as_ptr(),
                ) as *const c_void,
                c"log_buf_part2".as_ptr(),
            );
            /* we shouldn't get full end of BPF verifier log */
            ASSERT_NULL(
                strstr(
                    log_buf.as_ptr(),
                    c"max_states_per_insn 0 total_states 0 peak_states 0 mark_read 0\n".as_ptr(),
                ) as *const c_void,
                c"log_buf_end".as_ptr(),
            );
        }
    }

    if env.verbosity > VERBOSE_NONE {
        printf(
            c"LOG:   \n=================\n%s=================\n".as_ptr(),
            log_buf.as_ptr(),
        );
    }

    test_log_fixup__destroy(skel);
}

unsafe fn bad_core_relo_subprog() {
    let mut log_buf = [0 as c_char; 8 * 1024];
    let mut skel: *mut test_log_fixup;
    let err: c_int;

    skel = test_log_fixup__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.bad_relo_subprog, true);
    bpf_program__set_log_buf((*skel).progs.bad_relo_subprog, log_buf.as_mut_ptr(), log_buf.len());

    err = test_log_fixup__load(skel);
    if !ASSERT_ERR(err, c"load_fail".as_ptr()) {
        test_log_fixup__destroy(skel);
        return;
    }

    ASSERT_HAS_SUBSTR(
        log_buf.as_ptr(),
        c": <invalid CO-RE relocation>\nfailed to resolve CO-RE relocation <byte_off> ".as_ptr(),
        c"log_buf".as_ptr(),
    );
    ASSERT_HAS_SUBSTR(
        log_buf.as_ptr(),
        c"struct task_struct___bad.fake_field_subprog (0:2 @ offset 8)\n".as_ptr(),
        c"log_buf".as_ptr(),
    );

    if env.verbosity > VERBOSE_NONE {
        printf(
            c"LOG:   \n=================\n%s=================\n".as_ptr(),
            log_buf.as_ptr(),
        );
    }

    test_log_fixup__destroy(skel);
}

unsafe fn missing_map() {
    let mut log_buf = [0 as c_char; 8 * 1024];
    let mut skel: *mut test_log_fixup;
    let err: c_int;

    skel = test_log_fixup__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    bpf_map__set_autocreate((*skel).maps.missing_map, false);

    bpf_program__set_autoload((*skel).progs.use_missing_map, true);
    bpf_program__set_log_buf((*skel).progs.use_missing_map, log_buf.as_mut_ptr(), log_buf.len());

    err = test_log_fixup__load(skel);
    if !ASSERT_ERR(err, c"load_fail".as_ptr()) {
        test_log_fixup__destroy(skel);
        return;
    }

    ASSERT_TRUE(
        bpf_map__autocreate((*skel).maps.existing_map),
        c"existing_map_autocreate".as_ptr(),
    );
    ASSERT_FALSE(
        bpf_map__autocreate((*skel).maps.missing_map),
        c"missing_map_autocreate".as_ptr(),
    );

    ASSERT_HAS_SUBSTR(
        log_buf.as_ptr(),
        c": <invalid BPF map reference>\nBPF map 'missing_map' is referenced but wasn't created\n".as_ptr(),
        c"log_buf".as_ptr(),
    );

    if env.verbosity > VERBOSE_NONE {
        printf(
            c"LOG:   \n=================\n%s=================\n".as_ptr(),
            log_buf.as_ptr(),
        );
    }

    test_log_fixup__destroy(skel);
}

unsafe fn missing_kfunc() {
    let mut log_buf = [0 as c_char; 8 * 1024];
    let mut skel: *mut test_log_fixup;
    let err: c_int;

    skel = test_log_fixup__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.use_missing_kfunc, true);
    bpf_program__set_log_buf((*skel).progs.use_missing_kfunc, log_buf.as_mut_ptr(), log_buf.len());

    err = test_log_fixup__load(skel);
    if !ASSERT_ERR(err, c"load_fail".as_ptr()) {
        test_log_fixup__destroy(skel);
        return;
    }

    ASSERT_HAS_SUBSTR(
        log_buf.as_ptr(),
        c"0: <invalid kfunc call>\nkfunc 'bpf_nonexistent_kfunc' is referenced but wasn't resolved\n".as_ptr(),
        c"log_buf".as_ptr(),
    );

    if env.verbosity > VERBOSE_NONE {
        printf(
            c"LOG:   \n=================\n%s=================\n".as_ptr(),
            log_buf.as_ptr(),
        );
    }

    test_log_fixup__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_log_fixup() {
    if test__start_subtest(c"bad_core_relo_trunc_none".as_ptr()) {
        bad_core_relo(0, trunc_type::TRUNC_NONE /* full buf */);
    }
    if test__start_subtest(c"bad_core_relo_trunc_partial".as_ptr()) {
        bad_core_relo(300, trunc_type::TRUNC_PARTIAL /* truncate original log a bit */);
    }
    if test__start_subtest(c"bad_core_relo_trunc_full".as_ptr()) {
        bad_core_relo(240, trunc_type::TRUNC_FULL /* truncate also libbpf's message patch */);
    }
    if test__start_subtest(c"bad_core_relo_subprog".as_ptr()) {
        bad_core_relo_subprog();
    }
    if test__start_subtest(c"missing_map".as_ptr()) {
        missing_map();
    }
    if test__start_subtest(c"missing_kfunc".as_ptr()) {
        missing_kfunc();
    }
}
