// SPDX-License-Identifier: GPL-2.0
//
// Translated from testing/selftests/bpf/prog_tests/tailcalls.c.
// C include dependencies intentionally remain external: unistd.h,
// test_progs.h, network_helpers.h, and the generated *.skel.h files.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_void};
use core::{mem, ptr};

type __u32 = u32;
type pthread_t = usize;

const BPF_ANY: u64 = 0;
const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;
const ENOENT: c_int = 2;
const JMP_TABLE: &[u8] = b"/sys/fs/bpf/jmp_table\0";

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_void,
    pub data_size_in: u32,
    pub repeat: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct bpf_uprobe_opts {
    pub func_name: *const c_char,
}

#[repr(C)]
pub struct tailcall_bpf2bpf4__bss {
    pub noise: bool,
    pub count: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;
    static pkt_v4: [u8; 0];

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn getpid() -> c_int;
    fn pthread_create(t: *mut pthread_t, attr: *const c_void, f: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn pthread_join(t: pthread_t, retval: *mut c_void) -> c_int;

    fn bpf_prog_test_load(file: *const c_char, prog_type: c_int, obj: *mut *mut bpf_object, prog_fd: *mut c_int) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__find_program_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_program;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_attach_target(prog: *mut bpf_program, fd: c_int, name: *const c_char) -> c_int;
    fn bpf_program__attach_trace(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_program__attach_freplace(prog: *mut bpf_program, fd: c_int, name: *const c_char) -> *mut bpf_link;
    fn bpf_program__attach_uprobe_opts(prog: *mut bpf_program, pid: c_int, binary_path: *const c_char, offset: usize, opts: *mut bpf_uprobe_opts) -> *mut bpf_link;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map__max_entries(map: *mut bpf_map) -> c_int;
    fn bpf_map__is_internal(map: *mut bpf_map) -> bool;
    fn bpf_map__pin(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_map__unpin(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_map__set_pin_path(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;
    fn CHECK_FAIL(cond: bool) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_FALSE(cond: bool, name: *const c_char) -> bool;
    fn ASSERT_GE(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ<T: Into<i128>>(a: T, b: T, name: *const c_char) -> bool;

    fn RUN_TESTS_tailcall_bpf2bpf_hierarchy2();
    fn RUN_TESTS_tailcall_bpf2bpf_hierarchy3();
    fn RUN_TESTS_tailcall_fail();
    fn RUN_TESTS_tailcall_callback();
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

fn test_run_opts_for_buf(buff: &mut [u8; 128]) -> bpf_test_run_opts {
    bpf_test_run_opts {
        data_in: buff.as_mut_ptr() as *mut c_void,
        data_size_in: mem::size_of_val(buff) as u32,
        repeat: 1,
        retval: 0,
    }
}

unsafe fn load_entry_and_jmp_table(
    file: *const c_char,
    obj: *mut *mut bpf_object,
    prog_fd: *mut c_int,
    main_fd: *mut c_int,
    prog_array: *mut *mut bpf_map,
    map_fd: *mut c_int,
) -> bool {
    let mut err = bpf_prog_test_load(file, BPF_PROG_TYPE_SCHED_CLS, obj, prog_fd);
    if CHECK_FAIL(err != 0) {
        return false;
    }
    let prog = bpf_object__find_program_by_name(*obj, c!("entry"));
    if CHECK_FAIL(prog.is_null()) {
        return false;
    }
    *main_fd = bpf_program__fd(prog);
    if CHECK_FAIL(*main_fd < 0) {
        return false;
    }
    *prog_array = bpf_object__find_map_by_name(*obj, c!("jmp_table"));
    if CHECK_FAIL((*prog_array).is_null()) {
        return false;
    }
    *map_fd = bpf_map__fd(*prog_array);
    if CHECK_FAIL(*map_fd < 0) {
        return false;
    }
    err = 0;
    err == 0
}

unsafe fn populate_classifiers(obj: *mut bpf_object, prog_array: *mut bpf_map, map_fd: c_int) -> bool {
    let mut i = 0;
    while i < bpf_map__max_entries(prog_array) {
        let mut prog_name = [0 as c_char; 32];
        snprintf(prog_name.as_mut_ptr(), prog_name.len(), c!("classifier_%d"), i);
        let prog = bpf_object__find_program_by_name(obj, prog_name.as_ptr());
        if CHECK_FAIL(prog.is_null()) {
            return false;
        }
        let prog_fd = bpf_program__fd(prog);
        if CHECK_FAIL(prog_fd < 0) {
            return false;
        }
        let err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if CHECK_FAIL(err != 0) {
            return false;
        }
        i += 1;
    }
    true
}

/* test_tailcall_1 checks basic functionality by patching multiple locations
 * in a single program for a single tail call slot with nop->jmp, jmp->nop
 * and jmp->jmp rewrites. Also checks for nop->nop.
 */
unsafe fn test_tailcall_1() {
    let (mut err, mut prog_fd, mut main_fd, mut map_fd) = (0, 0, 0, 0);
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_array: *mut bpf_map = ptr::null_mut();
    let mut buff = [0u8; 128];
    let mut topts = test_run_opts_for_buf(&mut buff);

    if !load_entry_and_jmp_table(c!("tailcall1.bpf.o"), &mut obj, &mut prog_fd, &mut main_fd, &mut prog_array, &mut map_fd) {
        if !obj.is_null() { bpf_object__close(obj); }
        return;
    }
    if !populate_classifiers(obj, prog_array, map_fd) { bpf_object__close(obj); return; }

    let mut i = 0;
    while i < bpf_map__max_entries(prog_array) {
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, i as i128, c!("tailcall retval"));
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err != 0) { bpf_object__close(obj); return; }
        i += 1;
    }

    err = bpf_prog_test_run_opts(main_fd, &mut topts);
    ASSERT_OK(err, c!("tailcall"));
    ASSERT_EQ(topts.retval as i128, 3, c!("tailcall retval"));

    if !populate_classifiers(obj, prog_array, map_fd) { bpf_object__close(obj); return; }
    err = bpf_prog_test_run_opts(main_fd, &mut topts);
    ASSERT_OK(err, c!("tailcall"));
    ASSERT_OK(topts.retval as c_int, c!("tailcall retval"));

    i = 0;
    while i < bpf_map__max_entries(prog_array) {
        let j = bpf_map__max_entries(prog_array) - 1 - i;
        let mut prog_name = [0 as c_char; 32];
        snprintf(prog_name.as_mut_ptr(), prog_name.len(), c!("classifier_%d"), j);
        let prog = bpf_object__find_program_by_name(obj, prog_name.as_ptr());
        if CHECK_FAIL(prog.is_null()) { bpf_object__close(obj); return; }
        prog_fd = bpf_program__fd(prog);
        if CHECK_FAIL(prog_fd < 0) { bpf_object__close(obj); return; }
        err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if CHECK_FAIL(err != 0) { bpf_object__close(obj); return; }
        i += 1;
    }

    i = 0;
    while i < bpf_map__max_entries(prog_array) {
        let j = bpf_map__max_entries(prog_array) - 1 - i;
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, j as i128, c!("tailcall retval"));
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err != 0) { bpf_object__close(obj); return; }
        i += 1;
    }

    err = bpf_prog_test_run_opts(main_fd, &mut topts);
    ASSERT_OK(err, c!("tailcall"));
    ASSERT_EQ(topts.retval as i128, 3, c!("tailcall retval"));

    i = 0;
    while i < bpf_map__max_entries(prog_array) {
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err >= 0 || errno != ENOENT) { bpf_object__close(obj); return; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, 3, c!("tailcall retval"));
        i += 1;
    }
    bpf_object__close(obj);
}

/* test_tailcall_2 checks that patching multiple programs for a single
 * tail call slot works. It also jumps through several programs and tests
 * the tail call limit counter.
 */
unsafe fn test_tailcall_2() {
    let (mut err, mut prog_fd, mut main_fd, mut map_fd) = (0, 0, 0, 0);
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_array: *mut bpf_map = ptr::null_mut();
    let mut buff = [0u8; 128];
    let mut topts = test_run_opts_for_buf(&mut buff);

    if !load_entry_and_jmp_table(c!("tailcall2.bpf.o"), &mut obj, &mut prog_fd, &mut main_fd, &mut prog_array, &mut map_fd) {
        if !obj.is_null() { bpf_object__close(obj); }
        return;
    }
    if !populate_classifiers(obj, prog_array, map_fd) { bpf_object__close(obj); return; }

    err = bpf_prog_test_run_opts(main_fd, &mut topts);
    ASSERT_OK(err, c!("tailcall"));
    ASSERT_EQ(topts.retval as i128, 2, c!("tailcall retval"));

    let mut i = 2;
    err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
    if CHECK_FAIL(err != 0) { bpf_object__close(obj); return; }
    err = bpf_prog_test_run_opts(main_fd, &mut topts);
    ASSERT_OK(err, c!("tailcall"));
    ASSERT_EQ(topts.retval as i128, 1, c!("tailcall retval"));

    i = 0;
    err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
    if CHECK_FAIL(err != 0) { bpf_object__close(obj); return; }
    err = bpf_prog_test_run_opts(main_fd, &mut topts);
    ASSERT_OK(err, c!("tailcall"));
    ASSERT_EQ(topts.retval as i128, 3, c!("tailcall retval"));
    bpf_object__close(obj);
}

unsafe fn test_tailcall_count(which: *const c_char, test_fentry: bool, test_fexit: bool) {
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut fentry_obj: *mut bpf_object = ptr::null_mut();
    let mut fexit_obj: *mut bpf_object = ptr::null_mut();
    let mut fentry_link: *mut bpf_link = ptr::null_mut();
    let mut fexit_link: *mut bpf_link = ptr::null_mut();
    let (mut err, mut map_fd, mut prog_fd, mut main_fd, mut data_fd, mut val) = (0, 0, 0, 0, 0, 0);
    let mut prog_array: *mut bpf_map = ptr::null_mut();
    let mut buff = [0u8; 128];
    let mut topts = test_run_opts_for_buf(&mut buff);

    'out: loop {
        err = bpf_prog_test_load(which, BPF_PROG_TYPE_SCHED_CLS, &mut obj, &mut prog_fd);
        if CHECK_FAIL(err != 0) { return; }
        let mut prog = bpf_object__find_program_by_name(obj, c!("entry"));
        if CHECK_FAIL(prog.is_null()) { break 'out; }
        main_fd = bpf_program__fd(prog);
        if CHECK_FAIL(main_fd < 0) { break 'out; }
        prog_array = bpf_object__find_map_by_name(obj, c!("jmp_table"));
        if CHECK_FAIL(prog_array.is_null()) { break 'out; }
        map_fd = bpf_map__fd(prog_array);
        if CHECK_FAIL(map_fd < 0) { break 'out; }
        prog = bpf_object__find_program_by_name(obj, c!("classifier_0"));
        if CHECK_FAIL(prog.is_null()) { break 'out; }
        prog_fd = bpf_program__fd(prog);
        if CHECK_FAIL(prog_fd < 0) { break 'out; }
        let mut i = 0;
        err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if CHECK_FAIL(err != 0) { break 'out; }

        if test_fentry {
            fentry_obj = bpf_object__open_file(c!("tailcall_bpf2bpf_fentry.bpf.o"), ptr::null());
            if !ASSERT_OK_PTR(fentry_obj, c!("open fentry_obj file")) { break 'out; }
            prog = bpf_object__find_program_by_name(fentry_obj, c!("fentry"));
            if !ASSERT_OK_PTR(prog, c!("find fentry prog")) { break 'out; }
            err = bpf_program__set_attach_target(prog, prog_fd, c!("subprog_tail"));
            if !ASSERT_OK(err, c!("set_attach_target subprog_tail")) { break 'out; }
            err = bpf_object__load(fentry_obj);
            if !ASSERT_OK(err, c!("load fentry_obj")) { break 'out; }
            fentry_link = bpf_program__attach_trace(prog);
            if !ASSERT_OK_PTR(fentry_link, c!("attach_trace")) { break 'out; }
        }
        if test_fexit {
            fexit_obj = bpf_object__open_file(c!("tailcall_bpf2bpf_fexit.bpf.o"), ptr::null());
            if !ASSERT_OK_PTR(fexit_obj, c!("open fexit_obj file")) { break 'out; }
            prog = bpf_object__find_program_by_name(fexit_obj, c!("fexit"));
            if !ASSERT_OK_PTR(prog, c!("find fexit prog")) { break 'out; }
            err = bpf_program__set_attach_target(prog, prog_fd, c!("subprog_tail"));
            if !ASSERT_OK(err, c!("set_attach_target subprog_tail")) { break 'out; }
            err = bpf_object__load(fexit_obj);
            if !ASSERT_OK(err, c!("load fexit_obj")) { break 'out; }
            fexit_link = bpf_program__attach_trace(prog);
            if !ASSERT_OK_PTR(fexit_link, c!("attach_trace")) { break 'out; }
        }

        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, 1, c!("tailcall retval"));

        let mut data_map = bpf_object__find_map_by_name(obj, c!("tailcall.bss"));
        if CHECK_FAIL(data_map.is_null() || !bpf_map__is_internal(data_map)) { break 'out; }
        data_fd = bpf_map__fd(data_map);
        if CHECK_FAIL(data_fd < 0) { break 'out; }
        i = 0;
        err = bpf_map_lookup_elem(data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
        ASSERT_OK(err, c!("tailcall count"));
        ASSERT_EQ(val as i128, 33, c!("tailcall count"));

        if test_fentry {
            data_map = bpf_object__find_map_by_name(fentry_obj, c!(".bss"));
            if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find tailcall_bpf2bpf_fentry.bss map")) { break 'out; }
            data_fd = bpf_map__fd(data_map);
            if !ASSERT_FALSE(data_fd < 0, c!("find tailcall_bpf2bpf_fentry.bss map fd")) { break 'out; }
            i = 0;
            err = bpf_map_lookup_elem(data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
            ASSERT_OK(err, c!("fentry count"));
            ASSERT_EQ(val as i128, 33, c!("fentry count"));
        }
        if test_fexit {
            data_map = bpf_object__find_map_by_name(fexit_obj, c!(".bss"));
            if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find tailcall_bpf2bpf_fexit.bss map")) { break 'out; }
            data_fd = bpf_map__fd(data_map);
            if !ASSERT_FALSE(data_fd < 0, c!("find tailcall_bpf2bpf_fexit.bss map fd")) { break 'out; }
            i = 0;
            err = bpf_map_lookup_elem(data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
            ASSERT_OK(err, c!("fexit count"));
            ASSERT_EQ(val as i128, 33, c!("fexit count"));
        }

        i = 0;
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err != 0) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_OK(topts.retval as c_int, c!("tailcall retval"));
        break 'out;
    }
    bpf_link__destroy(fentry_link);
    bpf_link__destroy(fexit_link);
    bpf_object__close(fentry_obj);
    bpf_object__close(fexit_obj);
    bpf_object__close(obj);
}

unsafe fn test_tailcall_3() { test_tailcall_count(c!("tailcall3.bpf.o"), false, false); }
unsafe fn test_tailcall_6() { test_tailcall_count(c!("tailcall6.bpf.o"), false, false); }

unsafe fn test_tailcall_4() { test_tailcall_indirect(c!("tailcall4.bpf.o"), false); }
unsafe fn test_tailcall_5() { test_tailcall_indirect(c!("tailcall5.bpf.o"), true); }

unsafe fn test_tailcall_indirect(file: *const c_char, use_key_values: bool) {
    let (mut err, mut prog_fd, mut main_fd, mut map_fd) = (0, 0, 0, 0);
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_array: *mut bpf_map = ptr::null_mut();
    let zero: c_int = 0;
    let key = [1111, 1234, 5678];
    let mut buff = [0u8; 128];
    let mut topts = test_run_opts_for_buf(&mut buff);

    'out: loop {
        if !load_entry_and_jmp_table(file, &mut obj, &mut prog_fd, &mut main_fd, &mut prog_array, &mut map_fd) { return; }
        let data_map = bpf_object__find_map_by_name(obj, c!("tailcall.bss"));
        if CHECK_FAIL(data_map.is_null() || !bpf_map__is_internal(data_map)) { break 'out; }
        let data_fd = bpf_map__fd(data_map);
        if CHECK_FAIL(data_fd < 0) { break 'out; }
        if !populate_classifiers(obj, prog_array, map_fd) { break 'out; }
        let mut i = 0;
        while i < bpf_map__max_entries(prog_array) {
            let input = if use_key_values { key[i as usize] } else { i };
            err = bpf_map_update_elem(data_fd, &zero as *const _ as *const c_void, &input as *const _ as *const c_void, BPF_ANY);
            if CHECK_FAIL(err != 0) { break 'out; }
            err = bpf_prog_test_run_opts(main_fd, &mut topts);
            ASSERT_OK(err, c!("tailcall"));
            ASSERT_EQ(topts.retval as i128, i as i128, c!("tailcall retval"));
            i += 1;
        }
        i = 0;
        while i < bpf_map__max_entries(prog_array) {
            let input = if use_key_values { key[i as usize] } else { i };
            err = bpf_map_update_elem(data_fd, &zero as *const _ as *const c_void, &input as *const _ as *const c_void, BPF_ANY);
            if CHECK_FAIL(err != 0) { break 'out; }
            err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
            if CHECK_FAIL(err != 0) { break 'out; }
            err = bpf_prog_test_run_opts(main_fd, &mut topts);
            ASSERT_OK(err, c!("tailcall"));
            ASSERT_EQ(topts.retval as i128, 3, c!("tailcall retval"));
            i += 1;
        }
        break 'out;
    }
    bpf_object__close(obj);
}

unsafe fn test_tailcall_bpf2bpf_1() {
    let (mut err, mut prog_fd, mut main_fd, mut map_fd) = (0, 0, 0, 0);
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_array: *mut bpf_map = ptr::null_mut();
    let mut topts = bpf_test_run_opts { data_in: &pkt_v4 as *const _ as *mut c_void, data_size_in: mem::size_of_val(&pkt_v4) as u32, repeat: 1, retval: 0 };
    'out: loop {
        if !load_entry_and_jmp_table(c!("tailcall_bpf2bpf1.bpf.o"), &mut obj, &mut prog_fd, &mut main_fd, &mut prog_array, &mut map_fd) { return; }
        /* nop -> jmp */
        if !populate_classifiers(obj, prog_array, map_fd) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, 1, c!("tailcall retval"));
        /* jmp -> nop, call subprog that will do tailcall */
        let mut i = 1;
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err != 0) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_OK(topts.retval as c_int, c!("tailcall retval"));
        /* make sure that subprog can access ctx and entry prog that called this subprog can properly return */
        i = 0;
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err != 0) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, (mem::size_of_val(&pkt_v4) * 2) as i128, c!("tailcall retval"));
        break 'out;
    }
    bpf_object__close(obj);
}

unsafe fn test_tailcall_bpf2bpf_2() { test_tailcall_count(c!("tailcall_bpf2bpf2.bpf.o"), false, false); }

unsafe fn test_tailcall_bpf2bpf_3() {
    let (mut err, mut prog_fd, mut main_fd, mut map_fd) = (0, 0, 0, 0);
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_array: *mut bpf_map = ptr::null_mut();
    let mut topts = bpf_test_run_opts { data_in: &pkt_v4 as *const _ as *mut c_void, data_size_in: mem::size_of_val(&pkt_v4) as u32, repeat: 1, retval: 0 };
    'out: loop {
        if !load_entry_and_jmp_table(c!("tailcall_bpf2bpf3.bpf.o"), &mut obj, &mut prog_fd, &mut main_fd, &mut prog_array, &mut map_fd) { return; }
        if !populate_classifiers(obj, prog_array, map_fd) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, (mem::size_of_val(&pkt_v4) * 3) as i128, c!("tailcall retval"));
        let mut i = 1;
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err != 0) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, mem::size_of_val(&pkt_v4) as i128, c!("tailcall retval"));
        i = 0;
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if CHECK_FAIL(err != 0) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, (mem::size_of_val(&pkt_v4) * 2) as i128, c!("tailcall retval"));
        break 'out;
    }
    bpf_object__close(obj);
}

/* test_tailcall_bpf2bpf_4 checks that tailcall counter is correctly preserved
 * across tailcalls combined with bpf2bpf calls.
 */
unsafe fn test_tailcall_bpf2bpf_4(noise: bool) {
    let (mut err, mut prog_fd, mut main_fd, mut map_fd) = (0, 0, 0, 0);
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_array: *mut bpf_map = ptr::null_mut();
    let mut val = tailcall_bpf2bpf4__bss { noise, count: 0 };
    let mut topts = bpf_test_run_opts { data_in: &pkt_v4 as *const _ as *mut c_void, data_size_in: mem::size_of_val(&pkt_v4) as u32, repeat: 1, retval: 0 };
    'out: loop {
        if !load_entry_and_jmp_table(c!("tailcall_bpf2bpf4.bpf.o"), &mut obj, &mut prog_fd, &mut main_fd, &mut prog_array, &mut map_fd) { return; }
        if !populate_classifiers(obj, prog_array, map_fd) { break 'out; }
        let data_map = bpf_object__find_map_by_name(obj, c!("tailcall.bss"));
        if CHECK_FAIL(data_map.is_null() || !bpf_map__is_internal(data_map)) { break 'out; }
        let data_fd = bpf_map__fd(data_map);
        if CHECK_FAIL(data_fd < 0) { break 'out; }
        let mut i = 0;
        err = bpf_map_update_elem(data_fd, &i as *const _ as *const c_void, &val as *const _ as *const c_void, BPF_ANY);
        if CHECK_FAIL(err != 0) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, (mem::size_of_val(&pkt_v4) * 3) as i128, c!("tailcall retval"));
        i = 0;
        err = bpf_map_lookup_elem(data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
        ASSERT_OK(err, c!("tailcall count"));
        ASSERT_EQ(val.count as i128, 31, c!("tailcall count"));
        break 'out;
    }
    bpf_object__close(obj);
}

#[repr(C)]
pub struct tailcall_poke {
    pub maps: tailcall_poke_maps,
    pub progs: tailcall_poke_progs,
}
#[repr(C)]
pub struct tailcall_poke_maps { pub jmp_table: *mut bpf_map }
#[repr(C)]
pub struct tailcall_poke_progs { pub test: *mut bpf_program, pub call1: *mut bpf_program, pub call2: *mut bpf_program }

#[repr(C)]
pub struct tailcall_bpf2bpf6 {
    pub maps: tailcall_bpf2bpf6_maps,
    pub progs: tailcall_bpf2bpf6_progs,
}
#[repr(C)]
pub struct tailcall_bpf2bpf6_maps { pub jmp_table: *mut bpf_map, pub bss: *mut bpf_map }
#[repr(C)]
pub struct tailcall_bpf2bpf6_progs { pub entry: *mut bpf_program, pub classifier_0: *mut bpf_program }

#[repr(C)]
pub struct tc_bpf2bpf { pub progs: tc_bpf2bpf_progs }
#[repr(C)]
pub struct tc_bpf2bpf_progs { pub entry_tc: *mut bpf_program }

#[repr(C)]
pub struct tailcall_freplace {
    pub maps: tailcall_freplace_maps,
    pub progs: tailcall_freplace_progs,
}
#[repr(C)]
pub struct tailcall_freplace_maps { pub jmp_table: *mut bpf_map }
#[repr(C)]
pub struct tailcall_freplace_progs { pub entry_freplace: *mut bpf_program }

#[repr(C)]
pub struct tailcall_cgrp_storage_owner { pub maps: tailcall_cgrp_storage_owner_maps }
#[repr(C)]
pub struct tailcall_cgrp_storage_owner_maps { pub prog_array: *mut bpf_map, pub storage_map: *mut bpf_map }

#[repr(C)]
pub struct tailcall_cgrp_storage {
    pub obj: *mut bpf_object,
    pub maps: tailcall_cgrp_storage_maps,
    pub progs: tailcall_cgrp_storage_progs,
}
#[repr(C)]
pub struct tailcall_cgrp_storage_maps { pub prog_array: *mut bpf_map, pub storage_map: *mut bpf_map }
#[repr(C)]
pub struct tailcall_cgrp_storage_progs { pub caller_prog: *mut bpf_program, pub callee_prog: *mut bpf_program }

#[repr(C)]
pub struct tailcall_cgrp_storage_no_storage {
    pub obj: *mut bpf_object,
    pub maps: tailcall_cgrp_storage_no_storage_maps,
    pub progs: tailcall_cgrp_storage_no_storage_progs,
}
#[repr(C)]
pub struct tailcall_cgrp_storage_no_storage_maps { pub prog_array: *mut bpf_map }
#[repr(C)]
pub struct tailcall_cgrp_storage_no_storage_progs { pub leaf_prog: *mut bpf_program, pub caller_prog: *mut bpf_program }

#[repr(C)]
pub struct tailcall_sleepable {
    pub maps: tailcall_sleepable_maps,
    pub progs: tailcall_sleepable_progs,
    pub links: tailcall_sleepable_links,
    pub bss: *mut tailcall_sleepable_bss,
}
#[repr(C)]
pub struct tailcall_sleepable_maps { pub jmp_table: *mut bpf_map }
#[repr(C)]
pub struct tailcall_sleepable_progs {
    pub uprobe_normal: *mut bpf_program,
    pub uprobe_sleepable_1: *mut bpf_program,
    pub uprobe_sleepable_2: *mut bpf_program,
}
#[repr(C)]
pub struct tailcall_sleepable_links { pub uprobe_sleepable_1: *mut bpf_link }
#[repr(C)]
pub struct tailcall_sleepable_bss { pub my_pid: c_int, pub executed: c_int }

#[repr(C)]
pub struct tailcall_bpf2bpf2 { pub progs: tailcall_bpf2bpf2_progs }
#[repr(C)]
pub struct tailcall_bpf2bpf2_progs { pub classifier_0: *mut bpf_program }

#[repr(C)]
pub struct tailcall_bpf2bpf_fexit {
    pub progs: tailcall_bpf2bpf_fexit_progs,
    pub links: tailcall_bpf2bpf_fexit_links,
}
#[repr(C)]
pub struct tailcall_bpf2bpf_fexit_progs { pub fexit: *mut bpf_program }
#[repr(C)]
pub struct tailcall_bpf2bpf_fexit_links { pub fexit: *mut bpf_link }

unsafe extern "C" {
    fn tailcall_poke__open_and_load() -> *mut tailcall_poke;
    fn tailcall_poke__open() -> *mut tailcall_poke;
    fn tailcall_poke__load(obj: *mut tailcall_poke) -> c_int;
    fn tailcall_poke__destroy(obj: *mut tailcall_poke);
    fn tailcall_bpf2bpf6__open_and_load() -> *mut tailcall_bpf2bpf6;
    fn tailcall_bpf2bpf6__destroy(obj: *mut tailcall_bpf2bpf6);
    fn tc_bpf2bpf__open_and_load() -> *mut tc_bpf2bpf;
    fn tc_bpf2bpf__destroy(obj: *mut tc_bpf2bpf);
    fn tailcall_freplace__open() -> *mut tailcall_freplace;
    fn tailcall_freplace__load(obj: *mut tailcall_freplace) -> c_int;
    fn tailcall_freplace__destroy(obj: *mut tailcall_freplace);
    fn tailcall_cgrp_storage_owner__open_and_load() -> *mut tailcall_cgrp_storage_owner;
    fn tailcall_cgrp_storage_owner__destroy(obj: *mut tailcall_cgrp_storage_owner);
    fn tailcall_cgrp_storage__open() -> *mut tailcall_cgrp_storage;
    fn tailcall_cgrp_storage__destroy(obj: *mut tailcall_cgrp_storage);
    fn tailcall_cgrp_storage_no_storage__open() -> *mut tailcall_cgrp_storage_no_storage;
    fn tailcall_cgrp_storage_no_storage__open_and_load() -> *mut tailcall_cgrp_storage_no_storage;
    fn tailcall_cgrp_storage_no_storage__destroy(obj: *mut tailcall_cgrp_storage_no_storage);
    fn tailcall_sleepable__open() -> *mut tailcall_sleepable;
    fn tailcall_sleepable__load(obj: *mut tailcall_sleepable) -> c_int;
    fn tailcall_sleepable__destroy(obj: *mut tailcall_sleepable);
    fn tailcall_bpf2bpf2__open_and_load() -> *mut tailcall_bpf2bpf2;
    fn tailcall_bpf2bpf2__destroy(obj: *mut tailcall_bpf2bpf2);
    fn tailcall_bpf2bpf_fexit__open() -> *mut tailcall_bpf2bpf_fexit;
    fn tailcall_bpf2bpf_fexit__load(obj: *mut tailcall_bpf2bpf_fexit) -> c_int;
    fn tailcall_bpf2bpf_fexit__destroy(obj: *mut tailcall_bpf2bpf_fexit);
}

unsafe fn test_tailcall_bpf2bpf_6() {
    let mut err;
    let mut i = 0;
    let mut val: c_int = 0;
    let mut topts = bpf_test_run_opts { data_in: &pkt_v4 as *const _ as *mut c_void, data_size_in: mem::size_of_val(&pkt_v4) as u32, repeat: 1, retval: 0 };
    let obj = tailcall_bpf2bpf6__open_and_load();
    if !ASSERT_OK_PTR(obj, c!("open and load")) { return; }
    'out: loop {
        let main_fd = bpf_program__fd((*obj).progs.entry);
        if !ASSERT_GE(main_fd, 0, c!("entry prog fd")) { break 'out; }
        let map_fd = bpf_map__fd((*obj).maps.jmp_table);
        if !ASSERT_GE(map_fd, 0, c!("jmp_table map fd")) { break 'out; }
        let prog_fd = bpf_program__fd((*obj).progs.classifier_0);
        if !ASSERT_GE(prog_fd, 0, c!("classifier_0 prog fd")) { break 'out; }
        err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_OK(err, c!("jmp_table map update")) { break 'out; }
        err = bpf_prog_test_run_opts(main_fd, &mut topts);
        ASSERT_OK(err, c!("entry prog test run"));
        ASSERT_EQ(topts.retval as i128, 0, c!("tailcall retval"));
        let data_fd = bpf_map__fd((*obj).maps.bss);
        if !ASSERT_GE(data_fd, 0, c!("bss map fd")) { break 'out; }
        err = bpf_map_lookup_elem(data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
        ASSERT_OK(err, c!("bss map lookup"));
        ASSERT_EQ(val as i128, 1, c!("done flag is set"));
        break 'out;
    }
    tailcall_bpf2bpf6__destroy(obj);
}

unsafe fn test_tailcall_bpf2bpf_fentry() { test_tailcall_count(c!("tailcall_bpf2bpf2.bpf.o"), true, false); }
unsafe fn test_tailcall_bpf2bpf_fexit() { test_tailcall_count(c!("tailcall_bpf2bpf2.bpf.o"), false, true); }
unsafe fn test_tailcall_bpf2bpf_fentry_fexit() { test_tailcall_count(c!("tailcall_bpf2bpf2.bpf.o"), true, true); }

unsafe fn test_tailcall_bpf2bpf_fentry_entry() {
    let mut tgt_obj: *mut bpf_object = ptr::null_mut();
    let mut fentry_obj: *mut bpf_object = ptr::null_mut();
    let mut fentry_link: *mut bpf_link = ptr::null_mut();
    let (mut err, mut prog_fd, mut data_fd, mut val) = (0, 0, 0, 0);
    let mut buff = [0u8; 128];
    let mut topts = test_run_opts_for_buf(&mut buff);
    'out: loop {
        err = bpf_prog_test_load(c!("tailcall_bpf2bpf2.bpf.o"), BPF_PROG_TYPE_SCHED_CLS, &mut tgt_obj, &mut prog_fd);
        if !ASSERT_OK(err, c!("load tgt_obj")) { return; }
        let prog_array = bpf_object__find_map_by_name(tgt_obj, c!("jmp_table"));
        if !ASSERT_OK_PTR(prog_array, c!("find jmp_table map")) { break 'out; }
        let map_fd = bpf_map__fd(prog_array);
        if !ASSERT_FALSE(map_fd < 0, c!("find jmp_table map fd")) { break 'out; }
        let mut prog = bpf_object__find_program_by_name(tgt_obj, c!("classifier_0"));
        if !ASSERT_OK_PTR(prog, c!("find classifier_0 prog")) { break 'out; }
        prog_fd = bpf_program__fd(prog);
        if !ASSERT_FALSE(prog_fd < 0, c!("find classifier_0 prog fd")) { break 'out; }
        let mut i = 0;
        err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_OK(err, c!("update jmp_table")) { break 'out; }
        fentry_obj = bpf_object__open_file(c!("tailcall_bpf2bpf_fentry.bpf.o"), ptr::null());
        if !ASSERT_OK_PTR(fentry_obj, c!("open fentry_obj file")) { break 'out; }
        prog = bpf_object__find_program_by_name(fentry_obj, c!("fentry"));
        if !ASSERT_OK_PTR(prog, c!("find fentry prog")) { break 'out; }
        err = bpf_program__set_attach_target(prog, prog_fd, c!("classifier_0"));
        if !ASSERT_OK(err, c!("set_attach_target classifier_0")) { break 'out; }
        err = bpf_object__load(fentry_obj);
        if !ASSERT_OK(err, c!("load fentry_obj")) { break 'out; }
        fentry_link = bpf_program__attach_trace(prog);
        if !ASSERT_OK_PTR(fentry_link, c!("attach_trace")) { break 'out; }
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, 1, c!("tailcall retval"));
        let mut data_map = bpf_object__find_map_by_name(tgt_obj, c!("tailcall.bss"));
        if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find tailcall.bss map")) { break 'out; }
        data_fd = bpf_map__fd(data_map);
        if !ASSERT_FALSE(data_fd < 0, c!("find tailcall.bss map fd")) { break 'out; }
        i = 0;
        err = bpf_map_lookup_elem(data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
        ASSERT_OK(err, c!("tailcall count"));
        ASSERT_EQ(val as i128, 34, c!("tailcall count"));
        data_map = bpf_object__find_map_by_name(fentry_obj, c!(".bss"));
        if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find tailcall_bpf2bpf_fentry.bss map")) { break 'out; }
        data_fd = bpf_map__fd(data_map);
        if !ASSERT_FALSE(data_fd < 0, c!("find tailcall_bpf2bpf_fentry.bss map fd")) { break 'out; }
        i = 0;
        err = bpf_map_lookup_elem(data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
        ASSERT_OK(err, c!("fentry count"));
        ASSERT_EQ(val as i128, 1, c!("fentry count"));
        break 'out;
    }
    bpf_link__destroy(fentry_link);
    bpf_object__close(fentry_obj);
    bpf_object__close(tgt_obj);
}

static mut poke_thread_exit: c_int = 0;

unsafe extern "C" fn poke_update(arg: *mut c_void) -> *mut c_void {
    let zero: __u32 = 0;
    let call = arg as *mut tailcall_poke;
    let map_fd = bpf_map__fd((*call).maps.jmp_table) as __u32;
    let prog1_fd = bpf_program__fd((*call).progs.call1) as __u32;
    let prog2_fd = bpf_program__fd((*call).progs.call2) as __u32;
    while poke_thread_exit == 0 {
        bpf_map_update_elem(map_fd as c_int, &zero as *const _ as *const c_void, &prog1_fd as *const _ as *const c_void, BPF_ANY);
        bpf_map_update_elem(map_fd as c_int, &zero as *const _ as *const c_void, &prog2_fd as *const _ as *const c_void, BPF_ANY);
    }
    ptr::null_mut()
}

unsafe fn test_tailcall_poke() {
    let mut cnt = 10;
    let mut thread: pthread_t = 0;
    unlink(JMP_TABLE.as_ptr() as *const c_char);
    let call = tailcall_poke__open_and_load();
    if !ASSERT_OK_PTR(call, c!("tailcall_poke__open")) { return; }
    'out: loop {
        let mut err = bpf_map__pin((*call).maps.jmp_table, JMP_TABLE.as_ptr() as *const c_char);
        if !ASSERT_OK(err, c!("bpf_map__pin")) { break 'out; }
        err = pthread_create(&mut thread, ptr::null(), poke_update, call as *mut c_void);
        if !ASSERT_OK(err, c!("new toggler")) { break 'out; }
        while cnt != 0 {
            cnt -= 1;
            let test = tailcall_poke__open();
            if !ASSERT_OK_PTR(test, c!("tailcall_poke__open")) { break; }
            err = bpf_map__set_pin_path((*test).maps.jmp_table, JMP_TABLE.as_ptr() as *const c_char);
            if !ASSERT_OK(err, c!("bpf_map__pin")) {
                tailcall_poke__destroy(test);
                break;
            }
            bpf_program__set_autoload((*test).progs.test, true);
            bpf_program__set_autoload((*test).progs.call1, false);
            bpf_program__set_autoload((*test).progs.call2, false);
            err = tailcall_poke__load(test);
            tailcall_poke__destroy(test);
            if !ASSERT_OK(err, c!("tailcall_poke__load")) { break; }
        }
        poke_thread_exit = 1;
        ASSERT_OK(pthread_join(thread, ptr::null_mut()), c!("pthread_join"));
        break 'out;
    }
    bpf_map__unpin((*call).maps.jmp_table, JMP_TABLE.as_ptr() as *const c_char);
    tailcall_poke__destroy(call);
}

unsafe fn test_tailcall_hierarchy_count(which: *const c_char, test_fentry: bool, test_fexit: bool, test_fentry_entry: bool) {
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut fentry_obj: *mut bpf_object = ptr::null_mut();
    let mut fexit_obj: *mut bpf_object = ptr::null_mut();
    let mut fentry_link: *mut bpf_link = ptr::null_mut();
    let mut fexit_link: *mut bpf_link = ptr::null_mut();
    let (mut err, mut prog_fd, mut map_fd, mut val) = (0, 0, 0, 0);
    let mut fentry_data_fd = 0;
    let mut fexit_data_fd = 0;
    let mut data_map: *mut bpf_map = ptr::null_mut();
    let mut buff = [0u8; 128];
    let mut topts = test_run_opts_for_buf(&mut buff);
    'out: loop {
        err = bpf_prog_test_load(which, BPF_PROG_TYPE_SCHED_CLS, &mut obj, &mut prog_fd);
        if !ASSERT_OK(err, c!("load obj")) { return; }
        let mut prog = bpf_object__find_program_by_name(obj, c!("entry"));
        if !ASSERT_OK_PTR(prog, c!("find entry prog")) { break 'out; }
        prog_fd = bpf_program__fd(prog);
        if !ASSERT_GE(prog_fd, 0, c!("prog_fd")) { break 'out; }
        if test_fentry_entry {
            fentry_obj = bpf_object__open_file(c!("tailcall_bpf2bpf_hierarchy_fentry.bpf.o"), ptr::null());
            if !ASSERT_OK_PTR(fentry_obj, c!("open fentry_obj file")) { break 'out; }
            let fentry_prog = bpf_object__find_program_by_name(fentry_obj, c!("fentry"));
            if !ASSERT_OK_PTR(prog, c!("find fentry prog")) { break 'out; }
            err = bpf_program__set_attach_target(fentry_prog, prog_fd, c!("entry"));
            if !ASSERT_OK(err, c!("set_attach_target entry")) { break 'out; }
            err = bpf_object__load(fentry_obj);
            if !ASSERT_OK(err, c!("load fentry_obj")) { break 'out; }
            fentry_link = bpf_program__attach_trace(fentry_prog);
            if !ASSERT_OK_PTR(fentry_link, c!("attach_trace")) { break 'out; }
            let fentry_prog_fd = bpf_program__fd(fentry_prog);
            if !ASSERT_GE(fentry_prog_fd, 0, c!("fentry_prog_fd")) { break 'out; }
            let prog_array = bpf_object__find_map_by_name(fentry_obj, c!("jmp_table"));
            if !ASSERT_OK_PTR(prog_array, c!("find jmp_table")) { break 'out; }
            map_fd = bpf_map__fd(prog_array);
            if !ASSERT_GE(map_fd, 0, c!("map_fd")) { break 'out; }
            let mut i = 0;
            err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &fentry_prog_fd as *const _ as *const c_void, BPF_ANY);
            if !ASSERT_OK(err, c!("update jmp_table")) { break 'out; }
            data_map = bpf_object__find_map_by_name(fentry_obj, c!(".bss"));
            if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find data_map")) { break 'out; }
        } else {
            let prog_array = bpf_object__find_map_by_name(obj, c!("jmp_table"));
            if !ASSERT_OK_PTR(prog_array, c!("find jmp_table")) { break 'out; }
            map_fd = bpf_map__fd(prog_array);
            if !ASSERT_GE(map_fd, 0, c!("map_fd")) { break 'out; }
            let mut i = 0;
            err = bpf_map_update_elem(map_fd, &i as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
            if !ASSERT_OK(err, c!("update jmp_table")) { break 'out; }
            data_map = bpf_object__find_map_by_name(obj, c!(".bss"));
            if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find data_map")) { break 'out; }
        }
        if test_fentry {
            fentry_obj = bpf_object__open_file(c!("tailcall_bpf2bpf_fentry.bpf.o"), ptr::null());
            if !ASSERT_OK_PTR(fentry_obj, c!("open fentry_obj file")) { break 'out; }
            prog = bpf_object__find_program_by_name(fentry_obj, c!("fentry"));
            if !ASSERT_OK_PTR(prog, c!("find fentry prog")) { break 'out; }
            err = bpf_program__set_attach_target(prog, prog_fd, c!("subprog_tail"));
            if !ASSERT_OK(err, c!("set_attach_target subprog_tail")) { break 'out; }
            err = bpf_object__load(fentry_obj);
            if !ASSERT_OK(err, c!("load fentry_obj")) { break 'out; }
            fentry_link = bpf_program__attach_trace(prog);
            if !ASSERT_OK_PTR(fentry_link, c!("attach_trace")) { break 'out; }
        }
        if test_fexit {
            fexit_obj = bpf_object__open_file(c!("tailcall_bpf2bpf_fexit.bpf.o"), ptr::null());
            if !ASSERT_OK_PTR(fexit_obj, c!("open fexit_obj file")) { break 'out; }
            prog = bpf_object__find_program_by_name(fexit_obj, c!("fexit"));
            if !ASSERT_OK_PTR(prog, c!("find fexit prog")) { break 'out; }
            err = bpf_program__set_attach_target(prog, prog_fd, c!("subprog_tail"));
            if !ASSERT_OK(err, c!("set_attach_target subprog_tail")) { break 'out; }
            err = bpf_object__load(fexit_obj);
            if !ASSERT_OK(err, c!("load fexit_obj")) { break 'out; }
            fexit_link = bpf_program__attach_trace(prog);
            if !ASSERT_OK_PTR(fexit_link, c!("attach_trace")) { break 'out; }
        }
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, 1, c!("tailcall retval"));
        let main_data_fd = bpf_map__fd(data_map);
        if !ASSERT_GE(main_data_fd, 0, c!("main_data_fd")) { break 'out; }
        let mut i = 0;
        err = bpf_map_lookup_elem(main_data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
        ASSERT_OK(err, c!("tailcall count"));
        ASSERT_EQ(val as i128, 34, c!("tailcall count"));
        if test_fentry {
            data_map = bpf_object__find_map_by_name(fentry_obj, c!(".bss"));
            if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find tailcall_bpf2bpf_fentry.bss map")) { break 'out; }
            fentry_data_fd = bpf_map__fd(data_map);
            if !ASSERT_GE(fentry_data_fd, 0, c!("find tailcall_bpf2bpf_fentry.bss map fd")) { break 'out; }
            err = bpf_map_lookup_elem(fentry_data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
            ASSERT_OK(err, c!("fentry count"));
            ASSERT_EQ(val as i128, 68, c!("fentry count"));
        }
        if test_fexit {
            data_map = bpf_object__find_map_by_name(fexit_obj, c!(".bss"));
            if !ASSERT_FALSE(data_map.is_null() || !bpf_map__is_internal(data_map), c!("find tailcall_bpf2bpf_fexit.bss map")) { break 'out; }
            fexit_data_fd = bpf_map__fd(data_map);
            if !ASSERT_GE(fexit_data_fd, 0, c!("find tailcall_bpf2bpf_fexit.bss map fd")) { break 'out; }
            err = bpf_map_lookup_elem(fexit_data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
            ASSERT_OK(err, c!("fexit count"));
            ASSERT_EQ(val as i128, 68, c!("fexit count"));
        }
        err = bpf_map_delete_elem(map_fd, &i as *const _ as *const c_void);
        if !ASSERT_OK(err, c!("delete_elem from jmp_table")) { break 'out; }
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, c!("tailcall"));
        ASSERT_EQ(topts.retval as i128, 1, c!("tailcall retval"));
        err = bpf_map_lookup_elem(main_data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
        ASSERT_OK(err, c!("tailcall count"));
        ASSERT_EQ(val as i128, 35, c!("tailcall count"));
        if test_fentry {
            err = bpf_map_lookup_elem(fentry_data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
            ASSERT_OK(err, c!("fentry count"));
            ASSERT_EQ(val as i128, 70, c!("fentry count"));
        }
        if test_fexit {
            err = bpf_map_lookup_elem(fexit_data_fd, &i as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
            ASSERT_OK(err, c!("fexit count"));
            ASSERT_EQ(val as i128, 70, c!("fexit count"));
        }
        break 'out;
    }
    bpf_link__destroy(fentry_link);
    bpf_link__destroy(fexit_link);
    bpf_object__close(fentry_obj);
    bpf_object__close(fexit_obj);
    bpf_object__close(obj);
}

unsafe fn test_tailcall_bpf2bpf_hierarchy_1() { test_tailcall_hierarchy_count(c!("tailcall_bpf2bpf_hierarchy1.bpf.o"), false, false, false); }
unsafe fn test_tailcall_bpf2bpf_hierarchy_fentry() { test_tailcall_hierarchy_count(c!("tailcall_bpf2bpf_hierarchy1.bpf.o"), true, false, false); }
unsafe fn test_tailcall_bpf2bpf_hierarchy_fexit() { test_tailcall_hierarchy_count(c!("tailcall_bpf2bpf_hierarchy1.bpf.o"), false, true, false); }
unsafe fn test_tailcall_bpf2bpf_hierarchy_fentry_fexit() { test_tailcall_hierarchy_count(c!("tailcall_bpf2bpf_hierarchy1.bpf.o"), true, true, false); }
unsafe fn test_tailcall_bpf2bpf_hierarchy_fentry_entry() { test_tailcall_hierarchy_count(c!("tc_dummy.bpf.o"), false, false, true); }
unsafe fn test_tailcall_bpf2bpf_hierarchy_2() { RUN_TESTS_tailcall_bpf2bpf_hierarchy2(); }
unsafe fn test_tailcall_bpf2bpf_hierarchy_3() { RUN_TESTS_tailcall_bpf2bpf_hierarchy3(); }

unsafe fn test_tailcall_freplace() {
    let mut freplace_skel = tailcall_freplace__open();
    let mut freplace_link: *mut bpf_link = ptr::null_mut();
    let mut tc_skel: *mut tc_bpf2bpf = ptr::null_mut();
    let mut buff = [0u8; 128];
    let mut _topts = test_run_opts_for_buf(&mut buff);
    if !ASSERT_OK_PTR(freplace_skel, c!("tailcall_freplace__open")) { return; }
    'out: loop {
        tc_skel = tc_bpf2bpf__open_and_load();
        if !ASSERT_OK_PTR(tc_skel, c!("tc_bpf2bpf__open_and_load")) { break 'out; }
        let tc_prog_fd = bpf_program__fd((*tc_skel).progs.entry_tc);
        let freplace_prog = (*freplace_skel).progs.entry_freplace;
        let mut err = bpf_program__set_attach_target(freplace_prog, tc_prog_fd, c!("subprog_tc"));
        if !ASSERT_OK(err, c!("set_attach_target")) { break 'out; }
        err = tailcall_freplace__load(freplace_skel);
        if !ASSERT_OK(err, c!("tailcall_freplace__load")) { break 'out; }
        let map_fd = bpf_map__fd((*freplace_skel).maps.jmp_table);
        let prog_fd = bpf_program__fd(freplace_prog);
        let key = 0;
        err = bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        ASSERT_ERR(err, c!("update jmp_table failure"));
        freplace_link = bpf_program__attach_freplace(freplace_prog, tc_prog_fd, c!("subprog_tc"));
        if !ASSERT_OK_PTR(freplace_link, c!("attach_freplace")) { break 'out; }
        err = bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        ASSERT_ERR(err, c!("update jmp_table failure"));
        break 'out;
    }
    bpf_link__destroy(freplace_link);
    tailcall_freplace__destroy(freplace_skel);
    tc_bpf2bpf__destroy(tc_skel);
}

unsafe fn test_tailcall_bpf2bpf_freplace() {
    let mut freplace_skel: *mut tailcall_freplace = ptr::null_mut();
    let mut freplace_link: *mut bpf_link = ptr::null_mut();
    let mut tc_skel = tc_bpf2bpf__open_and_load();
    let mut buff = [0u8; 128];
    let mut _topts = test_run_opts_for_buf(&mut buff);
    if !ASSERT_OK_PTR(tc_skel, c!("tc_bpf2bpf__open_and_load")) { tailcall_freplace__destroy(freplace_skel); tc_bpf2bpf__destroy(tc_skel); return; }
    'out: loop {
        let prog_fd = bpf_program__fd((*tc_skel).progs.entry_tc);
        freplace_skel = tailcall_freplace__open();
        if !ASSERT_OK_PTR(freplace_skel, c!("tailcall_freplace__open")) { break 'out; }
        let mut err = bpf_program__set_attach_target((*freplace_skel).progs.entry_freplace, prog_fd, c!("subprog_tc"));
        if !ASSERT_OK(err, c!("set_attach_target")) { break 'out; }
        err = tailcall_freplace__load(freplace_skel);
        if !ASSERT_OK(err, c!("tailcall_freplace__load")) { break 'out; }
        freplace_link = bpf_program__attach_freplace((*freplace_skel).progs.entry_freplace, prog_fd, c!("subprog_tc"));
        if !ASSERT_OK_PTR(freplace_link, c!("attach_freplace")) { break 'out; }
        err = bpf_link__destroy(freplace_link);
        freplace_link = ptr::null_mut();
        if !ASSERT_OK(err, c!("destroy link")) { break 'out; }
        let key = 0;
        let map_fd = bpf_map__fd((*freplace_skel).maps.jmp_table);
        err = bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_OK(err, c!("update jmp_table")) { break 'out; }
        err = bpf_map_delete_elem(map_fd, &key as *const _ as *const c_void);
        if !ASSERT_OK(err, c!("delete_elem from jmp_table")) { break 'out; }
        err = bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_OK(err, c!("update jmp_table")) { break 'out; }
        freplace_link = bpf_program__attach_freplace((*freplace_skel).progs.entry_freplace, prog_fd, c!("subprog_tc"));
        if !ASSERT_ERR_PTR(freplace_link, c!("attach_freplace failure")) { break 'out; }
        err = bpf_map_delete_elem(map_fd, &key as *const _ as *const c_void);
        if !ASSERT_OK(err, c!("delete_elem from jmp_table")) { break 'out; }
        freplace_link = bpf_program__attach_freplace((*freplace_skel).progs.entry_freplace, prog_fd, c!("subprog_tc"));
        if !ASSERT_OK_PTR(freplace_link, c!("attach_freplace")) { break 'out; }
        err = bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_ERR(err, c!("update jmp_table failure")) { break 'out; }
        break 'out;
    }
    bpf_link__destroy(freplace_link);
    tailcall_freplace__destroy(freplace_skel);
    tc_bpf2bpf__destroy(tc_skel);
}

unsafe fn test_tailcall_failure() { RUN_TESTS_tailcall_fail(); }

unsafe fn test_tailcall_cgrp_storage() {
    let owner_skel = tailcall_cgrp_storage_owner__open_and_load();
    if !ASSERT_OK_PTR(owner_skel, c!("owner_open_and_load")) { return; }
    let mut skel: *mut tailcall_cgrp_storage = ptr::null_mut();
    'out: loop {
        let key = 0;
        let prog_array_fd = bpf_map__fd((*owner_skel).maps.prog_array);
        let storage_map_fd = bpf_map__fd((*owner_skel).maps.storage_map);
        skel = tailcall_cgrp_storage__open();
        if !ASSERT_OK_PTR(skel, c!("tailcall_cgrp_storage__open")) { break 'out; }
        let mut err = bpf_map__reuse_fd((*skel).maps.prog_array, prog_array_fd);
        if !ASSERT_OK(err, c!("reuse_prog_array")) { break 'out; }
        err = bpf_map__reuse_fd((*skel).maps.storage_map, storage_map_fd);
        if !ASSERT_OK(err, c!("reuse_storage_map")) { break 'out; }
        err = bpf_object__load((*skel).obj);
        if !ASSERT_OK(err, c!("tailcall_cgrp_storage__load")) { break 'out; }
        let prog_fd = bpf_program__fd((*skel).progs.callee_prog);
        err = bpf_map_update_elem(prog_array_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        ASSERT_OK(err, c!("update_prog_array"));
        break 'out;
    }
    tailcall_cgrp_storage__destroy(skel);
    tailcall_cgrp_storage_owner__destroy(owner_skel);
}

unsafe fn test_tailcall_cgrp_storage_diff_storage() {
    let owner_skel = tailcall_cgrp_storage_owner__open_and_load();
    if !ASSERT_OK_PTR(owner_skel, c!("owner_open_and_load")) { return; }
    let mut skel: *mut tailcall_cgrp_storage = ptr::null_mut();
    'out: loop {
        let prog_array_fd = bpf_map__fd((*owner_skel).maps.prog_array);
        skel = tailcall_cgrp_storage__open();
        if !ASSERT_OK_PTR(skel, c!("tailcall_cgrp_storage__open")) { break 'out; }
        let mut err = bpf_map__reuse_fd((*skel).maps.prog_array, prog_array_fd);
        if !ASSERT_OK(err, c!("reuse_prog_array")) { break 'out; }
        err = bpf_object__load((*skel).obj);
        ASSERT_ERR(err, c!("tailcall_cgrp_storage__load"));
        break 'out;
    }
    tailcall_cgrp_storage__destroy(skel);
    tailcall_cgrp_storage_owner__destroy(owner_skel);
}

unsafe fn test_tailcall_cgrp_storage_no_storage() {
    let owner_skel = tailcall_cgrp_storage_owner__open_and_load();
    if !ASSERT_OK_PTR(owner_skel, c!("owner_open_and_load")) { return; }
    let mut skel: *mut tailcall_cgrp_storage_no_storage = ptr::null_mut();
    'out: loop {
        let prog_array_fd = bpf_map__fd((*owner_skel).maps.prog_array);
        skel = tailcall_cgrp_storage_no_storage__open();
        if !ASSERT_OK_PTR(skel, c!("tailcall_cgrp_storage_no_storage__open")) { break 'out; }
        let mut err = bpf_map__reuse_fd((*skel).maps.prog_array, prog_array_fd);
        if !ASSERT_OK(err, c!("reuse_prog_array")) { break 'out; }
        err = bpf_object__load((*skel).obj);
        ASSERT_ERR(err, c!("tailcall_cgrp_storage_no_storage__load"));
        break 'out;
    }
    tailcall_cgrp_storage_no_storage__destroy(skel);
    tailcall_cgrp_storage_owner__destroy(owner_skel);
}

unsafe fn test_tailcall_cgrp_storage_no_storage_leaf() {
    let owner_skel = tailcall_cgrp_storage_owner__open_and_load();
    if !ASSERT_OK_PTR(owner_skel, c!("owner_open_and_load")) { return; }
    let skel = tailcall_cgrp_storage_no_storage__open_and_load();
    'out: loop {
        if !ASSERT_OK_PTR(skel, c!("tailcall_cgrp_storage_no_storage__open_and_load")) { break 'out; }
        let key = 0;
        let prog_array_fd = bpf_map__fd((*owner_skel).maps.prog_array);
        let mut prog_fd = bpf_program__fd((*skel).progs.leaf_prog);
        let mut err = bpf_map_update_elem(prog_array_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_OK(err, c!("update_prog_array_leaf")) { break 'out; }
        prog_fd = bpf_program__fd((*skel).progs.caller_prog);
        err = bpf_map_update_elem(prog_array_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        ASSERT_ERR(err, c!("update_prog_array_bridge"));
        break 'out;
    }
    tailcall_cgrp_storage_no_storage__destroy(skel);
    tailcall_cgrp_storage_owner__destroy(owner_skel);
}

unsafe fn test_tailcall_cgrp_storage_no_storage_bridge() {
    let owner_skel = tailcall_cgrp_storage_owner__open_and_load();
    if !ASSERT_OK_PTR(owner_skel, c!("owner_open_and_load")) { return; }
    let mut bridge_skel: *mut tailcall_cgrp_storage_no_storage = ptr::null_mut();
    let mut callee_skel: *mut tailcall_cgrp_storage = ptr::null_mut();
    'out: loop {
        let key = 0;
        let prog_array_fd = bpf_map__fd((*owner_skel).maps.prog_array);
        let storage_map_fd = bpf_map__fd((*owner_skel).maps.storage_map);
        callee_skel = tailcall_cgrp_storage__open();
        if !ASSERT_OK_PTR(callee_skel, c!("tailcall_cgrp_storage__open")) { break 'out; }
        bpf_program__set_autoload((*callee_skel).progs.caller_prog, false);
        let mut err = bpf_map__reuse_fd((*callee_skel).maps.prog_array, prog_array_fd);
        if !ASSERT_OK(err, c!("reuse_prog_array")) { break 'out; }
        err = bpf_map__reuse_fd((*callee_skel).maps.storage_map, storage_map_fd);
        if !ASSERT_OK(err, c!("reuse_storage_map")) { break 'out; }
        err = bpf_object__load((*callee_skel).obj);
        if !ASSERT_OK(err, c!("tailcall_cgrp_storage__load")) { break 'out; }
        let prog_fd = bpf_program__fd((*callee_skel).progs.callee_prog);
        err = bpf_map_update_elem(prog_array_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_OK(err, c!("update_prog_array")) { break 'out; }
        bridge_skel = tailcall_cgrp_storage_no_storage__open();
        if !ASSERT_OK_PTR(bridge_skel, c!("tailcall_cgrp_storage_no_storage__open")) { break 'out; }
        err = bpf_map__reuse_fd((*bridge_skel).maps.prog_array, prog_array_fd);
        if !ASSERT_OK(err, c!("reuse_prog_array")) { break 'out; }
        err = bpf_object__load((*bridge_skel).obj);
        ASSERT_ERR(err, c!("tailcall_cgrp_storage_no_storage_bridge__load"));
        break 'out;
    }
    tailcall_cgrp_storage_no_storage__destroy(bridge_skel);
    tailcall_cgrp_storage__destroy(callee_skel);
    tailcall_cgrp_storage_owner__destroy(owner_skel);
}

#[inline(never)]
pub unsafe extern "C" fn uprobe_sleepable_trigger() {
    core::arch::asm!("");
}

unsafe fn test_tailcall_sleepable() {
    let mut opts = bpf_uprobe_opts { func_name: ptr::null() };
    let mut skel = tailcall_sleepable__open();
    if !ASSERT_OK_PTR(skel, c!("tailcall_sleepable__open")) { return; }
    'out: loop {
        bpf_program__set_autoload((*skel).progs.uprobe_normal, true);
        bpf_program__set_autoload((*skel).progs.uprobe_sleepable_1, true);
        let mut err = tailcall_sleepable__load(skel);
        if !ASSERT_ERR(err, c!("tailcall_sleepable__load")) { break 'out; }
        tailcall_sleepable__destroy(skel);
        skel = tailcall_sleepable__open();
        if !ASSERT_OK_PTR(skel, c!("tailcall_sleepable__open")) { return; }
        bpf_program__set_autoload((*skel).progs.uprobe_sleepable_1, true);
        bpf_program__set_autoload((*skel).progs.uprobe_sleepable_2, true);
        err = tailcall_sleepable__load(skel);
        if !ASSERT_OK(err, c!("tailcall_sleepable__load")) { break 'out; }
        let key = 0;
        let prog_fd = bpf_program__fd((*skel).progs.uprobe_sleepable_2);
        let map_fd = bpf_map__fd((*skel).maps.jmp_table);
        err = bpf_map_update_elem(map_fd, &key as *const _ as *const c_void, &prog_fd as *const _ as *const c_void, BPF_ANY);
        if !ASSERT_OK(err, c!("update jmp_table")) { break 'out; }
        (*(*skel).bss).my_pid = getpid();
        opts.func_name = c!("uprobe_sleepable_trigger");
        (*skel).links.uprobe_sleepable_1 = bpf_program__attach_uprobe_opts((*skel).progs.uprobe_sleepable_1, -1, c!("/proc/self/exe"), 0, &mut opts);
        if !ASSERT_OK_PTR((*skel).links.uprobe_sleepable_1, c!("bpf_program__attach_uprobe_opts")) { break 'out; }
        uprobe_sleepable_trigger();
        ASSERT_EQ((*(*skel).bss).executed as i128, 1, c!("executed"));
        break 'out;
    }
    tailcall_sleepable__destroy(skel);
}

unsafe fn test_tailcall_callback() { RUN_TESTS_tailcall_callback(); }

unsafe fn test_tailcall_bpf2bpf_fexit_links() {
    let mut skel1: *mut tailcall_bpf2bpf_fexit = ptr::null_mut();
    let mut skel2: *mut tailcall_bpf2bpf_fexit = ptr::null_mut();
    let skel_tc = tailcall_bpf2bpf2__open_and_load();
    if !ASSERT_OK_PTR(skel_tc, c!("tailcall_bpf2bpf2__open_and_load")) { return; }
    'out: loop {
        skel1 = tailcall_bpf2bpf_fexit__open();
        if !ASSERT_OK_PTR(skel1, c!("tailcall_bpf2bpf_fexit__open")) { break 'out; }
        let prog_fd = bpf_program__fd((*skel_tc).progs.classifier_0);
        let mut err = bpf_program__set_attach_target((*skel1).progs.fexit, prog_fd, c!("subprog_tail"));
        if !ASSERT_OK(err, c!("bpf_program__set_attach_target")) { break 'out; }
        err = tailcall_bpf2bpf_fexit__load(skel1);
        if !ASSERT_OK(err, c!("tailcall_bpf2bpf_fexit__load")) { break 'out; }
        (*skel1).links.fexit = bpf_program__attach_trace((*skel1).progs.fexit);
        if !ASSERT_OK_PTR((*skel1).links.fexit, c!("bpf_program__attach_trace")) { break 'out; }
        skel2 = tailcall_bpf2bpf_fexit__open();
        if !ASSERT_OK_PTR(skel2, c!("tailcall_bpf2bpf_fexit__open")) { break 'out; }
        err = bpf_program__set_attach_target((*skel2).progs.fexit, prog_fd, c!("subprog_tail"));
        if !ASSERT_OK(err, c!("bpf_program__set_attach_target")) { break 'out; }
        err = tailcall_bpf2bpf_fexit__load(skel2);
        ASSERT_OK(err, c!("tailcall_bpf2bpf_fexit__load"));
        break 'out;
    }
    tailcall_bpf2bpf_fexit__destroy(skel1);
    tailcall_bpf2bpf_fexit__destroy(skel2);
    tailcall_bpf2bpf2__destroy(skel_tc);
}

pub unsafe extern "C" fn test_tailcalls() {
    if test__start_subtest(c!("tailcall_1")) { test_tailcall_1(); }
    if test__start_subtest(c!("tailcall_2")) { test_tailcall_2(); }
    if test__start_subtest(c!("tailcall_3")) { test_tailcall_3(); }
    if test__start_subtest(c!("tailcall_4")) { test_tailcall_4(); }
    if test__start_subtest(c!("tailcall_5")) { test_tailcall_5(); }
    if test__start_subtest(c!("tailcall_6")) { test_tailcall_6(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_1")) { test_tailcall_bpf2bpf_1(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_2")) { test_tailcall_bpf2bpf_2(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_3")) { test_tailcall_bpf2bpf_3(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_4")) { test_tailcall_bpf2bpf_4(false); }
    if test__start_subtest(c!("tailcall_bpf2bpf_5")) { test_tailcall_bpf2bpf_4(true); }
    if test__start_subtest(c!("tailcall_bpf2bpf_6")) { test_tailcall_bpf2bpf_6(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_fentry")) { test_tailcall_bpf2bpf_fentry(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_fexit")) { test_tailcall_bpf2bpf_fexit(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_fentry_fexit")) { test_tailcall_bpf2bpf_fentry_fexit(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_fentry_entry")) { test_tailcall_bpf2bpf_fentry_entry(); }
    if test__start_subtest(c!("tailcall_poke")) { test_tailcall_poke(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_hierarchy_1")) { test_tailcall_bpf2bpf_hierarchy_1(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_hierarchy_fentry")) { test_tailcall_bpf2bpf_hierarchy_fentry(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_hierarchy_fexit")) { test_tailcall_bpf2bpf_hierarchy_fexit(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_hierarchy_fentry_fexit")) { test_tailcall_bpf2bpf_hierarchy_fentry_fexit(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_hierarchy_fentry_entry")) { test_tailcall_bpf2bpf_hierarchy_fentry_entry(); }
    test_tailcall_bpf2bpf_hierarchy_2();
    test_tailcall_bpf2bpf_hierarchy_3();
    if test__start_subtest(c!("tailcall_freplace")) { test_tailcall_freplace(); }
    if test__start_subtest(c!("tailcall_bpf2bpf_freplace")) { test_tailcall_bpf2bpf_freplace(); }
    if test__start_subtest(c!("tailcall_failure")) { test_tailcall_failure(); }
    if test__start_subtest(c!("tailcall_sleepable")) { test_tailcall_sleepable(); }
    if test__start_subtest(c!("tailcall_cgrp_storage")) { test_tailcall_cgrp_storage(); }
    if test__start_subtest(c!("tailcall_cgrp_storage_diff_storage")) { test_tailcall_cgrp_storage_diff_storage(); }
    if test__start_subtest(c!("tailcall_cgrp_storage_no_storage")) { test_tailcall_cgrp_storage_no_storage(); }
    if test__start_subtest(c!("tailcall_cgrp_storage_no_storage_leaf")) { test_tailcall_cgrp_storage_no_storage_leaf(); }
    if test__start_subtest(c!("tailcall_cgrp_storage_no_storage_bridge")) { test_tailcall_cgrp_storage_no_storage_bridge(); }
    test_tailcall_callback();
    if test__start_subtest(c!("tailcall_bpf2bpf_fexit_links")) { test_tailcall_bpf2bpf_fexit_links(); }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
