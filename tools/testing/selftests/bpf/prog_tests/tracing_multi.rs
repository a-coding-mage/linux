// SPDX-License-Identifier: GPL-2.0

// Translated from C implementation source. Includes from the original file are
// dependency intent for the surrounding selftest build:
// test_progs.h, bpf/btf.h, search.h, bpf/libbpf_internal.h,
// tracing_multi*.skel.h, and trace_helpers.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type bool_ = bool;

const BTF_KIND_FUNC: c_uint = 12;
const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const INT_MAX: c_int = c_int::MAX;

const fn array_size<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size_type: __u32,
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
pub struct ksym {
    pub name: *const c_char,
}

#[repr(C)]
pub struct ksyms {
    pub syms: *mut ksym,
    pub sym_cnt: size_t,
    pub filtered_syms: *mut *const c_char,
    pub filtered_cnt: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_tracing_multi_opts {
    pub sz: size_t,
    pub ids: *__u32,
    pub cnt: size_t,
    pub cookies: *__u64,
}

impl Default for bpf_tracing_multi_opts {
    fn default() -> Self {
        Self {
            sz: mem::size_of::<Self>(),
            ids: ptr::null_mut(),
            cnt: 0,
            cookies: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_run_opts {
    pub sz: size_t,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self {
            sz: mem::size_of::<Self>(),
        }
    }
}

#[repr(C)]
pub struct tracing_multi_bss {
    pub pid: c_int,
    pub test_cookies: bool_,
    pub test_result_fentry: __u64,
    pub test_result_fexit: __u64,
}

#[repr(C)]
pub struct tracing_multi_progs {
    pub test_fentry: *mut bpf_program,
    pub test_fexit: *mut bpf_program,
    pub test_fentry_s: *mut bpf_program,
    pub test_fexit_s: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_multi_links {
    pub test_fentry: *mut bpf_link,
    pub test_fexit: *mut bpf_link,
    pub test_fentry_s: *mut bpf_link,
    pub test_fexit_s: *mut bpf_link,
}

#[repr(C)]
pub struct tracing_multi {
    pub bss: *mut tracing_multi_bss,
    pub progs: tracing_multi_progs,
    pub links: tracing_multi_links,
}

#[repr(C)]
pub struct tracing_multi_module_bss {
    pub pid: c_int,
    pub test_result_fentry: __u64,
    pub test_result_fexit: __u64,
}

#[repr(C)]
pub struct tracing_multi_module_progs {
    pub test_fentry: *mut bpf_program,
    pub test_fexit: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_multi_module_links {
    pub test_fentry: *mut bpf_link,
    pub test_fexit: *mut bpf_link,
}

#[repr(C)]
pub struct tracing_multi_module {
    pub bss: *mut tracing_multi_module_bss,
    pub progs: tracing_multi_module_progs,
    pub links: tracing_multi_module_links,
}

#[repr(C)]
pub struct tracing_multi_intersect_bss {
    pub pid: c_int,
    pub test_result_fentry_1: __u64,
    pub test_result_fexit_1: __u64,
    pub test_result_fentry_2: __u64,
    pub test_result_fexit_2: __u64,
    pub test_result_fentry: __u64,
}

#[repr(C)]
pub struct tracing_multi_intersect_progs {
    pub fentry_1: *mut bpf_program,
    pub fexit_1: *mut bpf_program,
    pub fentry_2: *mut bpf_program,
    pub fexit_2: *mut bpf_program,
    pub fentry: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_multi_intersect {
    pub bss: *mut tracing_multi_intersect_bss,
    pub progs: tracing_multi_intersect_progs,
}

#[repr(C)]
pub struct tracing_multi_session_bss {
    pub pid: c_int,
    pub test_result_fentry: __u64,
    pub test_result_fexit: __u64,
}

#[repr(C)]
pub struct tracing_multi_session_progs {
    pub test_session_1: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_multi_session {
    pub bss: *mut tracing_multi_session_bss,
    pub progs: tracing_multi_session_progs,
}

#[repr(C)]
pub struct tracing_multi_fail_progs {
    pub test_fentry: *mut bpf_program,
    pub test_fentry_s: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_multi_fail_links {
    pub test_fentry: *mut bpf_link,
    pub test_fentry_s: *mut bpf_link,
}

#[repr(C)]
pub struct tracing_multi_fail {
    pub progs: tracing_multi_fail_progs,
    pub links: tracing_multi_fail_links,
}

#[repr(C)]
pub struct tracing_multi_bench_progs {
    pub bench: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_multi_bench {
    pub progs: tracing_multi_bench_progs,
}

#[repr(C)]
pub struct tracing_multi_rollback_bss {
    pub pid: c_int,
    pub test_result_fentry: __u64,
    pub test_result_fexit: __u64,
}

#[repr(C)]
pub struct tracing_multi_rollback_progs {
    pub test_fentry: *mut bpf_program,
    pub test_fexit: *mut bpf_program,
    pub extra: *mut bpf_program,
    pub filler: *mut bpf_program,
}

#[repr(C)]
pub struct tracing_multi_rollback_links {
    pub test_fentry: *mut bpf_link,
    pub test_fexit: *mut bpf_link,
    pub extra: *mut bpf_link,
    pub filler: *mut bpf_link,
}

#[repr(C)]
pub struct tracing_multi_rollback {
    pub bss: *mut tracing_multi_rollback_bss,
    pub progs: tracing_multi_rollback_progs,
    pub links: tracing_multi_rollback_links,
}

unsafe extern "C" {
    fn rand() -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn getpid() -> c_int;
    fn tsearch(key: *const c_void, rootp: *mut *mut c_void, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int) -> *mut c_void;
    fn tfind(key: *const c_void, rootp: *mut *mut c_void, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int) -> *mut c_void;
    fn tdestroy(root: *mut c_void, free_node: unsafe extern "C" fn(*mut c_void));

    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__load_module_btf(module: *const c_char, vmlinux_btf: *mut btf) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__type_cnt(btf: *const btf) -> __u32;
    fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf_type_is_traceable_func(btf: *const btf, t: *const btf_type) -> bool_;

    fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__attach_tracing_multi(prog: *const bpf_program, pattern: *const c_char, opts: *const bpf_tracing_multi_opts) -> *mut bpf_link;
    fn bpf_program__attach(prog: *const bpf_program) -> *mut bpf_link;
    fn bpf_program__attach_trace(prog: *const bpf_program) -> *mut bpf_link;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool_);
    fn bpf_link__destroy(link: *mut bpf_link) -> c_int;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn libbpf_ensure_mem(pptr: *mut *mut c_void, cap: *mut size_t, elem_sz: size_t, need_cnt: size_t) -> c_int;

    fn trigger_module_test_read(x: c_int) -> c_int;
    fn bpf_get_ksyms(ksyms: *mut *mut ksyms, sym_cnt: bool_) -> c_int;
    fn free_kallsyms_local(ksyms: *mut ksyms);
    fn get_time_ns() -> c_long;
    fn get_bpf_max_tramp_links() -> c_int;

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool_;

    fn tracing_multi__open_and_load() -> *mut tracing_multi;
    fn tracing_multi__attach(skel: *mut tracing_multi) -> c_int;
    fn tracing_multi__destroy(skel: *mut tracing_multi);
    fn tracing_multi_module__open_and_load() -> *mut tracing_multi_module;
    fn tracing_multi_module__attach(skel: *mut tracing_multi_module) -> c_int;
    fn tracing_multi_module__destroy(skel: *mut tracing_multi_module);
    fn tracing_multi_intersect__open_and_load() -> *mut tracing_multi_intersect;
    fn tracing_multi_intersect__destroy(skel: *mut tracing_multi_intersect);
    fn tracing_multi_session__open_and_load() -> *mut tracing_multi_session;
    fn tracing_multi_session__attach(skel: *mut tracing_multi_session) -> c_int;
    fn tracing_multi_session__destroy(skel: *mut tracing_multi_session);
    fn tracing_multi_fail__open_and_load() -> *mut tracing_multi_fail;
    fn tracing_multi_fail__destroy(skel: *mut tracing_multi_fail);
    fn tracing_multi_bench__open_and_load() -> *mut tracing_multi_bench;
    fn tracing_multi_bench__destroy(skel: *mut tracing_multi_bench);
    fn tracing_multi_rollback__open() -> *mut tracing_multi_rollback;
    fn tracing_multi_rollback__load(skel: *mut tracing_multi_rollback) -> c_int;
    fn tracing_multi_rollback__destroy(skel: *mut tracing_multi_rollback);

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool_;
    fn ASSERT_ERR_PTR(ptr: *const c_void, name: *const c_char) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_EQ(actual: c_long, expected: c_long, name: *const c_char) -> bool_;
    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool_;
    fn ASSERT_FAIL(name: *const c_char);
    fn RUN_TESTS_tracing_multi_verifier();
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn BTF_INFO_KIND(info: __u32) -> c_uint {
    (info >> 24) as c_uint
}

static mut bpf_fentry_test_cookies: [__u64; 10] = [
    8,  /* bpf_fentry_test1 */
    9,  /* bpf_fentry_test2 */
    7,  /* bpf_fentry_test3 */
    5,  /* bpf_fentry_test4 */
    4,  /* bpf_fentry_test5 */
    2,  /* bpf_fentry_test6 */
    3,  /* bpf_fentry_test7 */
    1,  /* bpf_fentry_test8 */
    10, /* bpf_fentry_test9 */
    6,  /* bpf_fentry_test10 */
];

static bpf_fentry_test: [*const c_char; 10] = [
    c!("bpf_fentry_test1"),
    c!("bpf_fentry_test2"),
    c!("bpf_fentry_test3"),
    c!("bpf_fentry_test4"),
    c!("bpf_fentry_test5"),
    c!("bpf_fentry_test6"),
    c!("bpf_fentry_test7"),
    c!("bpf_fentry_test8"),
    c!("bpf_fentry_test9"),
    c!("bpf_fentry_test10"),
];

static bpf_testmod_fentry_test: [*const c_char; 5] = [
    c!("bpf_testmod_fentry_test1"),
    c!("bpf_testmod_fentry_test2"),
    c!("bpf_testmod_fentry_test3"),
    c!("bpf_testmod_fentry_test7"),
    c!("bpf_testmod_fentry_test11"),
];

const FUNCS_CNT: usize = 10;

unsafe fn get_random_funcs(funcs: *mut *const c_char) -> c_int {
    let mut cnt: c_int = 0;

    for i in 0..FUNCS_CNT {
        if rand() % 2 != 0 {
            *funcs.add(cnt as usize) = bpf_fentry_test[i];
            cnt += 1;
        }
    }
    /* we always need at least one.. */
    if cnt == 0 {
        *funcs.add(cnt as usize) = bpf_fentry_test[(rand() as usize) % FUNCS_CNT];
        cnt += 1;
    }
    cnt
}

unsafe extern "C" fn compare(ppa: *const c_void, ppb: *const c_void) -> c_int {
    let pa = *(ppa as *const *const c_char);
    let pb = *(ppb as *const *const c_char);

    strcmp(pa, pb)
}

unsafe extern "C" fn tdestroy_free_nop(_ptr: *mut c_void) {}

unsafe fn get_ids(funcs: *const *const c_char, funcs_cnt: c_int, module: *const c_char) -> *__u32 {
    let mut btf: *mut btf;
    let mut vmlinux_btf: *mut btf = ptr::null_mut();
    let mut nr: __u32;
    let mut type_id: __u32;
    let mut cnt: __u32 = 0;
    let mut root: *mut c_void = ptr::null_mut();
    let mut ids: *__u32 = ptr::null_mut();
    let mut err: c_int = 0;

    btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(btf as *const c_void, c!("btf__load_vmlinux_btf")) {
        return ptr::null_mut();
    }

    if !module.is_null() {
        vmlinux_btf = btf;
        btf = btf__load_module_btf(module, vmlinux_btf);
        if !ASSERT_OK_PTR(btf as *const c_void, c!("btf__load_module_btf")) {
            btf__free(vmlinux_btf);
            return ptr::null_mut();
        }
    }

    ids = calloc(funcs_cnt as size_t, mem::size_of::<__u32>()) as *__u32;
    if ids.is_null() {
        tdestroy(root, tdestroy_free_nop);
        btf__free(vmlinux_btf);
        btf__free(btf);
        return ids;
    }

    /*
     * We sort function names by name and search them
     * below for each function.
     */
    for i in 0..funcs_cnt {
        if tsearch(funcs.add(i as usize) as *const c_void, &mut root, compare).is_null() {
            ASSERT_FAIL(c!("tsearch failed"));
            err = -1;
            break;
        }
    }

    if err == 0 {
        nr = btf__type_cnt(btf);
        type_id = 1;
        while type_id < nr && cnt < funcs_cnt as __u32 {
            let type_: *const btf_type;
            let str_: *const c_char;
            let val: *mut *mut *const c_char;
            let idx: c_uint;

            type_ = btf__type_by_id(btf, type_id);
            if type_.is_null() {
                err = -1;
                break;
            }

            if BTF_INFO_KIND((*type_).info) != BTF_KIND_FUNC {
                type_id += 1;
                continue;
            }

            str_ = btf__name_by_offset(btf, (*type_).name_off);
            if str_.is_null() {
                err = -1;
                break;
            }

            val = tfind(&str_ as *const _ as *const c_void, &mut root, compare) as *mut *mut *const c_char;
            if val.is_null() {
                type_id += 1;
                continue;
            }

            /*
             * We keep pointer for each function name so we can get the original
             * array index and have the resulting ids array matching the original
             * function array.
             *
             * Doing it this way allow us to easily test the cookies support,
             * because each cookie is attached to particular function/id.
             */
            idx = (*val).offset_from(funcs) as c_uint;
            *ids.add(idx as usize) = type_id;
            cnt += 1;
            type_id += 1;
        }
    }

    if err != 0 {
        free(ids as *mut c_void);
        ids = ptr::null_mut();
    }

    tdestroy(root, tdestroy_free_nop);
    btf__free(vmlinux_btf);
    btf__free(btf);
    ids
}

unsafe fn tracing_multi_test_run(skel: *mut tracing_multi) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    prog_fd = bpf_program__fd((*skel).progs.test_fentry);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c!("test_run"));

    /* extra +1 count for sleepable programs */
    ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, (FUNCS_CNT + 1) as c_long, c!("test_result_fentry"));
    ASSERT_EQ((*(*skel).bss).test_result_fexit as c_long, (FUNCS_CNT + 1) as c_long, c!("test_result_fexit"));
}

unsafe fn test_skel_api() {
    let skel: *mut tracing_multi;
    let err: c_int;

    skel = tracing_multi__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    err = tracing_multi__attach(skel);
    if ASSERT_OK(err, c!("tracing_multi__attach")) {
        tracing_multi_test_run(skel);
    }

    tracing_multi__destroy(skel);
}

unsafe fn test_link_api_pattern() {
    let skel: *mut tracing_multi;

    skel = tracing_multi__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, c!("bpf_fentry_test*"), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.test_fentry as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi__destroy(skel);
        return;
    }

    (*skel).links.test_fexit = bpf_program__attach_tracing_multi((*skel).progs.test_fexit, c!("bpf_fentry_test*"), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.test_fexit as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi__destroy(skel);
        return;
    }

    (*skel).links.test_fentry_s = bpf_program__attach_tracing_multi((*skel).progs.test_fentry_s, c!("bpf_fentry_test1"), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.test_fentry_s as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi__destroy(skel);
        return;
    }

    (*skel).links.test_fexit_s = bpf_program__attach_tracing_multi((*skel).progs.test_fexit_s, c!("bpf_fentry_test1"), ptr::null());
    if ASSERT_OK_PTR((*skel).links.test_fexit_s as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi_test_run(skel);
    }

    tracing_multi__destroy(skel);
}

unsafe fn test_link_api_ids(test_cookies: bool_) {
    let mut opts = bpf_tracing_multi_opts::default();
    let skel: *mut tracing_multi;
    let cnt: size_t = FUNCS_CNT;
    let ids: *__u32;

    skel = tracing_multi__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();
    (*(*skel).bss).test_cookies = test_cookies;

    ids = get_ids(bpf_fentry_test.as_ptr(), cnt as c_int, ptr::null());
    if !ASSERT_OK_PTR(ids as *const c_void, c!("get_ids")) {
        tracing_multi__destroy(skel);
        return;
    }

    opts.ids = ids;
    opts.cnt = cnt;

    if test_cookies {
        opts.cookies = bpf_fentry_test_cookies.as_mut_ptr();
    }

    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, ptr::null(), &opts);
    if !ASSERT_OK_PTR((*skel).links.test_fentry as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi__destroy(skel);
        free(ids as *mut c_void);
        return;
    }

    (*skel).links.test_fexit = bpf_program__attach_tracing_multi((*skel).progs.test_fexit, ptr::null(), &opts);
    if !ASSERT_OK_PTR((*skel).links.test_fexit as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi__destroy(skel);
        free(ids as *mut c_void);
        return;
    }

    /* Only bpf_fentry_test1 is allowed for sleepable programs. */
    opts.cnt = 1;
    (*skel).links.test_fentry_s = bpf_program__attach_tracing_multi((*skel).progs.test_fentry_s, ptr::null(), &opts);
    if !ASSERT_OK_PTR((*skel).links.test_fentry_s as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi__destroy(skel);
        free(ids as *mut c_void);
        return;
    }

    (*skel).links.test_fexit_s = bpf_program__attach_tracing_multi((*skel).progs.test_fexit_s, ptr::null(), &opts);
    if ASSERT_OK_PTR((*skel).links.test_fexit_s as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi_test_run(skel);
    }

    tracing_multi__destroy(skel);
    free(ids as *mut c_void);
}

unsafe fn test_module_skel_api() {
    let mut skel: *mut tracing_multi_module = ptr::null_mut();
    let err: c_int;

    skel = tracing_multi_module__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    err = tracing_multi_module__attach(skel);
    if ASSERT_OK(err, c!("tracing_multi__attach")) {
        ASSERT_OK(trigger_module_test_read(1), c!("trigger_read"));
        ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 5, c!("test_result_fentry"));
        ASSERT_EQ((*(*skel).bss).test_result_fexit as c_long, 5, c!("test_result_fexit"));
    }

    tracing_multi_module__destroy(skel);
}

unsafe fn test_module_link_api_pattern() {
    let mut skel: *mut tracing_multi_module = ptr::null_mut();

    skel = tracing_multi_module__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_module__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, c!("bpf_testmod:bpf_testmod_fentry_test*"), ptr::null());
    if !ASSERT_OK_PTR((*skel).links.test_fentry as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi_module__destroy(skel);
        return;
    }

    (*skel).links.test_fexit = bpf_program__attach_tracing_multi((*skel).progs.test_fexit, c!("bpf_testmod:bpf_testmod_fentry_test*"), ptr::null());
    if ASSERT_OK_PTR((*skel).links.test_fexit as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        ASSERT_OK(trigger_module_test_read(1), c!("trigger_read"));
        ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 5, c!("test_result_fentry"));
        ASSERT_EQ((*(*skel).bss).test_result_fexit as c_long, 5, c!("test_result_fexit"));
    }

    tracing_multi_module__destroy(skel);
}

unsafe fn test_module_link_api_ids() {
    let cnt: size_t = array_size(&bpf_testmod_fentry_test);
    let mut opts = bpf_tracing_multi_opts::default();
    let mut skel: *mut tracing_multi_module = ptr::null_mut();
    let ids: *__u32;

    skel = tracing_multi_module__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_module__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    ids = get_ids(bpf_testmod_fentry_test.as_ptr(), cnt as c_int, c!("bpf_testmod"));
    if !ASSERT_OK_PTR(ids as *const c_void, c!("get_ids")) {
        tracing_multi_module__destroy(skel);
        return;
    }

    opts.ids = ids;
    opts.cnt = cnt;

    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, ptr::null(), &opts);
    if !ASSERT_OK_PTR((*skel).links.test_fentry as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        tracing_multi_module__destroy(skel);
        free(ids as *mut c_void);
        return;
    }

    (*skel).links.test_fexit = bpf_program__attach_tracing_multi((*skel).progs.test_fexit, ptr::null(), &opts);
    if ASSERT_OK_PTR((*skel).links.test_fexit as *const c_void, c!("bpf_program__attach_tracing_multi")) {
        ASSERT_OK(trigger_module_test_read(1), c!("trigger_read"));
        ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 5, c!("test_result_fentry"));
        ASSERT_EQ((*(*skel).bss).test_result_fexit as c_long, 5, c!("test_result_fexit"));
    }

    tracing_multi_module__destroy(skel);
    free(ids as *mut c_void);
}

unsafe fn is_set(mask: __u32, bit: __u32) -> bool_ {
    ((1u32 << bit) & mask) != 0
}

unsafe fn __test_intersect(mask: __u32, progs: [*const bpf_program; 4], test_results: [*mut __u64; 4]) {
    let mut opts = bpf_tracing_multi_opts::default();
    let mut topts = bpf_test_run_opts::default();
    let mut links: [*mut bpf_link; 4] = [ptr::null_mut(); 4];
    let mut funcs: [*const c_char; FUNCS_CNT] = [ptr::null(); FUNCS_CNT];
    let mut expected: [__u64; 4] = [0; 4];
    let mut i: __u32 = 0;

    /*
     * We have 4 programs in progs and the mask bits pick which
     * of them gets attached to randomly chosen functions.
     */
    while i < 4 {
        if is_set(mask, i) {
            let cnt = get_random_funcs(funcs.as_mut_ptr());
            let ids = get_ids(funcs.as_ptr(), cnt, ptr::null());
            if !ASSERT_OK_PTR(ids as *const c_void, c!("get_ids")) {
                break;
            }

            opts.ids = ids;
            opts.cnt = cnt as size_t;
            links[i as usize] = bpf_program__attach_tracing_multi(progs[i as usize], ptr::null(), &opts);
            free(ids as *mut c_void);

            if !ASSERT_OK_PTR(links[i as usize] as *const c_void, c!("bpf_program__attach_tracing_multi")) {
                break;
            }

            expected[i as usize] = *test_results[i as usize] + cnt as __u64;
        }
        i += 1;
    }

    let err = bpf_prog_test_run_opts(bpf_program__fd(progs[0]), &mut topts);
    ASSERT_OK(err, c!("test_run"));

    i = 0;
    while i < 4 {
        if is_set(mask, i) {
            ASSERT_EQ(*test_results[i as usize] as c_long, expected[i as usize] as c_long, c!("test_results"));
        }
        i += 1;
    }

    i = 0;
    while i < 4 {
        bpf_link__destroy(links[i as usize]);
        i += 1;
    }
}

unsafe fn test_intersect() {
    let skel: *mut tracing_multi_intersect;
    let mut progs: [*const bpf_program; 4] = [ptr::null(); 4];
    let mut test_results: [*mut __u64; 4] = [ptr::null_mut(); 4];
    let mut i: __u32;

    skel = tracing_multi_intersect__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_intersect__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    progs[0] = (*skel).progs.fentry_1;
    progs[1] = (*skel).progs.fexit_1;
    progs[2] = (*skel).progs.fentry_2;
    progs[3] = (*skel).progs.fexit_2;

    test_results[0] = &mut (*(*skel).bss).test_result_fentry_1;
    test_results[1] = &mut (*(*skel).bss).test_result_fexit_1;
    test_results[2] = &mut (*(*skel).bss).test_result_fentry_2;
    test_results[3] = &mut (*(*skel).bss).test_result_fexit_2;

    i = 1;
    while i < 16 {
        __test_intersect(i, progs, test_results);
        i += 1;
    }

    tracing_multi_intersect__destroy(skel);
}

unsafe fn test_fentry_after_multi() {
    static funcs: [*const c_char; 1] = [c!("bpf_fentry_test1")];
    let mut fentry_link: *mut bpf_link = ptr::null_mut();
    let mut multi_link: *mut bpf_link = ptr::null_mut();
    let mut skel: *mut tracing_multi_intersect = ptr::null_mut();
    let mut opts = bpf_tracing_multi_opts::default();
    let mut topts = bpf_test_run_opts::default();
    let mut ids: *__u32 = ptr::null_mut();
    let mut err: c_int;

    skel = tracing_multi_intersect__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_intersect__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    ids = get_ids(funcs.as_ptr(), array_size(&funcs) as c_int, ptr::null());
    if ASSERT_OK_PTR(ids as *const c_void, c!("get_ids")) {
        opts.ids = ids;
        opts.cnt = array_size(&funcs);
        multi_link = bpf_program__attach_tracing_multi((*skel).progs.fentry_1, ptr::null(), &opts);
        if ASSERT_OK_PTR(multi_link as *const c_void, c!("attach_multi")) {
            fentry_link = bpf_program__attach((*skel).progs.fentry);
            if ASSERT_OK_PTR(fentry_link as *const c_void, c!("attach_fentry")) {
                err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.fentry_1), &mut topts);
                if ASSERT_OK(err, c!("test_run")) {
                    ASSERT_EQ((*(*skel).bss).test_result_fentry_1 as c_long, 1, c!("multi_fentry"));
                    ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 1, c!("fentry"));

                    err = bpf_link__destroy(fentry_link);
                    fentry_link = ptr::null_mut();
                    if ASSERT_OK(err, c!("destroy_fentry")) {
                        err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.fentry_1), &mut topts);
                        if ASSERT_OK(err, c!("test_run_multi")) {
                            ASSERT_EQ((*(*skel).bss).test_result_fentry_1 as c_long, 2, c!("multi_fentry_only"));
                            ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 1, c!("fentry_detached"));

                            err = bpf_link__destroy(multi_link);
                            multi_link = ptr::null_mut();
                            if ASSERT_OK(err, c!("destroy_multi")) {
                                err = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.fentry_1), &mut topts);
                                if ASSERT_OK(err, c!("test_run_detached")) {
                                    ASSERT_EQ((*(*skel).bss).test_result_fentry_1 as c_long, 2, c!("multi_fentry_detached"));
                                    ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 1, c!("fentry_still_detached"));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    bpf_link__destroy(fentry_link);
    bpf_link__destroy(multi_link);
    free(ids as *mut c_void);
    tracing_multi_intersect__destroy(skel);
}

unsafe fn test_session() {
    let mut topts = bpf_test_run_opts::default();
    let skel: *mut tracing_multi_session;
    let mut err: c_int;
    let prog_fd: c_int;

    skel = tracing_multi_session__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_session__open_and_load")) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    err = tracing_multi_session__attach(skel);
    if ASSERT_OK(err, c!("tracing_multi_session__attach")) {
        /* execute kernel session */
        prog_fd = bpf_program__fd((*skel).progs.test_session_1);
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        ASSERT_OK(err, c!("test_run"));

        /* 10 for test_session_1, 1 for test_fsession_s */
        ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 11, c!("test_result_fentry"));
        /* extra count (+1 for each fexit execution) for test_result_fexit cookie check/inc */
        ASSERT_EQ((*(*skel).bss).test_result_fexit as c_long, 22, c!("test_result_fexit"));

        (*(*skel).bss).test_result_fentry = 0;
        (*(*skel).bss).test_result_fexit = 0;

        /* execute bpf_testmo.ko session */
        ASSERT_OK(trigger_module_test_read(1), c!("trigger_read"));

        /* 5 for test_session_2 */
        ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 5, c!("test_result_fentry"));
        /* extra count (+1 for each fexit execution) for test_result_fexit cookie */
        ASSERT_EQ((*(*skel).bss).test_result_fexit as c_long, 10, c!("test_result_fexit"));
    }

    tracing_multi_session__destroy(skel);
}

unsafe fn test_attach_api_fails() {
    let mut opts = bpf_tracing_multi_opts::default();
    static func: [*const c_char; 1] = [c!("bpf_fentry_test2")];
    let mut skel: *mut tracing_multi_fail = ptr::null_mut();
    let mut ids: [__u32; 2] = [0; 2];
    let mut ids2: *__u32 = ptr::null_mut();
    let mut cookies: [__u64; 2] = [0; 2];

    skel = tracing_multi_fail__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_fail__open_and_load")) {
        return;
    }

    /* fail#1 (libbpf) pattern and opts NULL */
    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, ptr::null(), ptr::null());
    if !ASSERT_EQ(libbpf_get_error((*skel).links.test_fentry as *const c_void), -EINVAL as c_long, c!("fail_1")) {
        tracing_multi_fail__destroy(skel);
        return;
    }

    /* fail#2 (libbpf) pattern and ids */
    opts = bpf_tracing_multi_opts::default();
    opts.ids = ids.as_mut_ptr();
    opts.cnt = 2;

    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, c!("bpf_fentry_test*"), &opts);
    if !ASSERT_EQ(libbpf_get_error((*skel).links.test_fentry as *const c_void), -EINVAL as c_long, c!("fail_2")) {
        tracing_multi_fail__destroy(skel);
        return;
    }

    /* fail#3 (libbpf) pattern and cookies */
    opts = bpf_tracing_multi_opts::default();
    opts.ids = ptr::null_mut();
    opts.cnt = 2;
    opts.cookies = cookies.as_mut_ptr();

    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, c!("bpf_fentry_test*"), &opts);
    if !ASSERT_EQ(libbpf_get_error((*skel).links.test_fentry as *const c_void), -EINVAL as c_long, c!("fail_3")) {
        tracing_multi_fail__destroy(skel);
        return;
    }

    /* fail#4 (libbpf) bogus pattern */
    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, c!("bpf_not_really_a_function*"), ptr::null());
    if !ASSERT_EQ(libbpf_get_error((*skel).links.test_fentry as *const c_void), -EINVAL as c_long, c!("fail_4")) {
        tracing_multi_fail__destroy(skel);
        return;
    }

    /* fail#5 (kernel) abnormal cnt */
    opts = bpf_tracing_multi_opts::default();
    opts.ids = ids.as_mut_ptr();
    opts.cnt = INT_MAX as size_t;

    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, ptr::null(), &opts);
    if !ASSERT_EQ(libbpf_get_error((*skel).links.test_fentry as *const c_void), -E2BIG as c_long, c!("fail_5")) {
        tracing_multi_fail__destroy(skel);
        return;
    }

    /* fail#6 (kernel) attach sleepable program to not-allowed function */
    ids2 = get_ids(func.as_ptr(), 1, ptr::null());
    if ASSERT_OK_PTR(ids2 as *const c_void, c!("get_ids")) {
        opts = bpf_tracing_multi_opts::default();
        opts.ids = ids2;
        opts.cnt = 1;

        (*skel).links.test_fentry_s = bpf_program__attach_tracing_multi((*skel).progs.test_fentry_s, ptr::null(), &opts);
        if ASSERT_EQ(libbpf_get_error((*skel).links.test_fentry_s as *const c_void), -EINVAL as c_long, c!("fail_6")) {
            /* fail#7 (kernel) attach with duplicate id */
            ids[0] = *ids2.add(0);
            ids[1] = *ids2.add(0);

            opts = bpf_tracing_multi_opts::default();
            opts.ids = ids.as_mut_ptr();
            opts.cnt = 2;

            (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, ptr::null(), &opts);
            ASSERT_EQ(libbpf_get_error((*skel).links.test_fentry as *const c_void), -EINVAL as c_long, c!("fail_7"));
        }
    }

    tracing_multi_fail__destroy(skel);
    free(ids2 as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_tracing_multi_bench_attach() {
    let mut opts = bpf_tracing_multi_opts::default();
    let mut skel: *mut tracing_multi_bench = ptr::null_mut();
    let attach_start_ns: c_long;
    let attach_end_ns: c_long;
    let detach_start_ns: c_long;
    let detach_end_ns: c_long;
    let attach_delta: c_double;
    let detach_delta: c_double;
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut i: size_t;
    let mut cap: size_t = 0;
    let mut cnt: size_t = 0;
    let mut ksyms: *mut ksyms = ptr::null_mut();
    let mut root: *mut c_void = ptr::null_mut();
    let mut dups: *mut c_void = ptr::null_mut();
    let mut ids: *__u32 = ptr::null_mut();
    let nr: __u32;
    let mut type_id: __u32;
    let btf: *mut btf;
    let mut err: c_int;

    // Original C skips this test under #ifndef __x86_64__.
    #[cfg(not(target_arch = "x86_64"))]
    {
        test__skip();
        return;
    }

    btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(btf as *const c_void, c!("btf__load_vmlinux_btf")) {
        return;
    }

    skel = tracing_multi_bench__open_and_load();
    if ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_bench__open_and_load")) {
        if ASSERT_OK(bpf_get_ksyms(&mut ksyms, true), c!("get_syms")) {
            /* Get all ftrace 'safe' symbols.. */
            i = 0;
            while i < (*ksyms).filtered_cnt {
                if tsearch((*ksyms).filtered_syms.add(i) as *const c_void, &mut root, compare).is_null() {
                    ASSERT_FAIL(c!("tsearch failed"));
                    break;
                }
                i += 1;
            }

            /*
             * Collect names that are not unique in kallsyms. The kernel resolves a
             * tracing-multi BTF id to an address with kallsyms_lookup_name(), which
             * returns the first symbol of that name. For a duplicate name that may
             * be a different (non-ftrace-able) instance than the ftrace-able one in
             * available_filter_functions, so attaching to it by BTF id fails with
             * -ENOENT (e.g. t_start/t_next/t_stop). ksyms->syms is sorted by name,
             * so equal names are adjacent.
             */
            i = 1;
            while i < (*ksyms).sym_cnt {
                if strcmp((*(*ksyms).syms.add(i)).name, (*(*ksyms).syms.add(i - 1)).name) == 0 {
                    if tsearch(&(*(*ksyms).syms.add(i)).name as *const _ as *const c_void, &mut dups, compare).is_null() {
                        ASSERT_FAIL(c!("tsearch failed"));
                        break;
                    }
                }
                i += 1;
            }

            /* ..and filter them through BTF and btf_type_is_traceable_func. */
            nr = btf__type_cnt(btf);
            type_id = 1;
            while type_id < nr {
                let type_: *const btf_type;
                let str_: *const c_char;

                type_ = btf__type_by_id(btf, type_id);
                if type_.is_null() {
                    break;
                }

                if BTF_INFO_KIND((*type_).info) != BTF_KIND_FUNC {
                    type_id += 1;
                    continue;
                }

                str_ = btf__name_by_offset(btf, (*type_).name_off);
                if str_.is_null() {
                    break;
                }

                if tfind(&str_ as *const _ as *const c_void, &mut root, compare).is_null() {
                    type_id += 1;
                    continue;
                }

                /* Skip names that are not unique in kallsyms, see above. */
                if !tfind(&str_ as *const _ as *const c_void, &mut dups, compare).is_null() {
                    type_id += 1;
                    continue;
                }

                if !btf_type_is_traceable_func(btf, type_) {
                    type_id += 1;
                    continue;
                }

                err = libbpf_ensure_mem(&mut ids as *mut _ as *mut *mut c_void, &mut cap, mem::size_of::<__u32>(), cnt + 1);
                if err != 0 {
                    break;
                }

                *ids.add(cnt) = type_id;
                cnt += 1;
                type_id += 1;
            }

            opts.ids = ids;
            opts.cnt = cnt;

            attach_start_ns = get_time_ns();
            link = bpf_program__attach_tracing_multi((*skel).progs.bench, ptr::null(), &opts);
            attach_end_ns = get_time_ns();

            if ASSERT_OK_PTR(link as *const c_void, c!("bpf_program__attach_tracing_multi")) {
                detach_start_ns = get_time_ns();
                bpf_link__destroy(link);
                detach_end_ns = get_time_ns();

                attach_delta = (attach_end_ns - attach_start_ns) as c_double / 1000000000.0;
                detach_delta = (detach_end_ns - detach_start_ns) as c_double / 1000000000.0;

                printf(c!("%s: found %lu functions\n"), c!("serial_test_tracing_multi_bench_attach"), cnt as c_ulong);
                printf(c!("%s: attached in %7.3lfs\n"), c!("serial_test_tracing_multi_bench_attach"), attach_delta);
                printf(c!("%s: detached in %7.3lfs\n"), c!("serial_test_tracing_multi_bench_attach"), detach_delta);
            }
        }
    }

    tracing_multi_bench__destroy(skel);
    tdestroy(root, tdestroy_free_nop);
    tdestroy(dups, tdestroy_free_nop);
    free_kallsyms_local(ksyms);
    free(ids as *mut c_void);
    btf__free(btf);
}

unsafe fn tracing_multi_rollback_run(skel: *mut tracing_multi_rollback) {
    let mut topts = bpf_test_run_opts::default();
    let err: c_int;
    let prog_fd: c_int;

    prog_fd = bpf_program__fd((*skel).progs.test_fentry);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c!("test_run"));

    /* make sure the rollback code did not leave any program attached */
    ASSERT_EQ((*(*skel).bss).test_result_fentry as c_long, 0, c!("test_result_fentry"));
    ASSERT_EQ((*(*skel).bss).test_result_fexit as c_long, 0, c!("test_result_fexit"));
}

unsafe fn test_rollback_put() {
    let mut opts = bpf_tracing_multi_opts::default();
    let mut skel: *mut tracing_multi_rollback = ptr::null_mut();
    let cnt: size_t = FUNCS_CNT;
    let mut ids: *__u32 = ptr::null_mut();
    let mut err: c_int;

    skel = tracing_multi_rollback__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_rollback__open")) {
        return;
    }

    bpf_program__set_autoload((*skel).progs.test_fentry, true);
    bpf_program__set_autoload((*skel).progs.test_fexit, true);

    err = tracing_multi_rollback__load(skel);
    if ASSERT_OK(err, c!("tracing_multi_rollback__load")) {
        ids = get_ids(bpf_fentry_test.as_ptr(), cnt as c_int, ptr::null());
        if ASSERT_OK_PTR(ids as *const c_void, c!("get_ids")) {
            /*
             * Mangle last id to trigger rollback, which needs to do put
             * on get-ed trampolines.
             */
            *ids.add(9) = 0;

            opts.ids = ids;
            opts.cnt = cnt;

            (*(*skel).bss).pid = getpid();

            (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, ptr::null(), &opts);
            if ASSERT_ERR_PTR((*skel).links.test_fentry as *const c_void, c!("bpf_program__attach_tracing_multi")) {
                (*skel).links.test_fexit = bpf_program__attach_tracing_multi((*skel).progs.test_fexit, ptr::null(), &opts);
                if ASSERT_ERR_PTR((*skel).links.test_fexit as *const c_void, c!("bpf_program__attach_tracing_multi")) {
                    /* We don't really attach any program, but let's make sure. */
                    tracing_multi_rollback_run(skel);
                }
            }
        }
    }

    tracing_multi_rollback__destroy(skel);
    free(ids as *mut c_void);
}

unsafe fn fillers_cleanup(skels: *mut *mut tracing_multi_rollback, cnt: c_int) {
    let mut i: c_int;

    i = 0;
    while i < cnt {
        tracing_multi_rollback__destroy(*skels.add(i as usize));
        i += 1;
    }

    free(skels as *mut c_void);
}

unsafe fn extra_load_and_link() -> *mut tracing_multi_rollback {
    let skel: *mut tracing_multi_rollback;
    let err: c_int;

    skel = tracing_multi_rollback__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_rollback__open")) {
        tracing_multi_rollback__destroy(skel);
        return ptr::null_mut();
    }

    bpf_program__set_autoload((*skel).progs.extra, true);

    err = tracing_multi_rollback__load(skel);
    if !ASSERT_OK(err, c!("tracing_multi_rollback__load")) {
        tracing_multi_rollback__destroy(skel);
        return ptr::null_mut();
    }

    (*skel).links.extra = bpf_program__attach_trace((*skel).progs.extra);
    if !ASSERT_OK_PTR((*skel).links.extra as *const c_void, c!("bpf_program__attach_trace")) {
        tracing_multi_rollback__destroy(skel);
        return ptr::null_mut();
    }

    skel
}

unsafe fn fillers_load_and_link(max: c_int) -> *mut *mut tracing_multi_rollback {
    let skels: *mut *mut tracing_multi_rollback;
    let mut skel: *mut tracing_multi_rollback;
    let mut i: c_int;
    let mut err: c_int;

    skels = calloc((max + 1) as size_t, mem::size_of::<*mut tracing_multi_rollback>()) as *mut *mut tracing_multi_rollback;
    if !ASSERT_OK_PTR(skels as *const c_void, c!("calloc")) {
        return ptr::null_mut();
    }

    i = 0;
    while i < max {
        skel = tracing_multi_rollback__open();
        *skels.add(i as usize) = skel;
        if !ASSERT_OK_PTR(*skels.add(i as usize) as *const c_void, c!("tracing_multi_rollback__open")) {
            fillers_cleanup(skels, i + 1);
            return ptr::null_mut();
        }

        bpf_program__set_autoload((*skel).progs.filler, true);

        err = tracing_multi_rollback__load(skel);
        if !ASSERT_OK(err, c!("tracing_multi_rollback__load")) {
            fillers_cleanup(skels, i + 1);
            return ptr::null_mut();
        }

        (*skel).links.filler = bpf_program__attach_trace((*skel).progs.filler);
        if !ASSERT_OK_PTR((*(*skels.add(i as usize))).links.filler as *const c_void, c!("bpf_program__attach_trace")) {
            fillers_cleanup(skels, i + 1);
            return ptr::null_mut();
        }
        i += 1;
    }

    skels
}

unsafe fn test_rollback_unlink() {
    let mut skel: *mut tracing_multi_rollback = ptr::null_mut();
    let mut extra: *mut tracing_multi_rollback = ptr::null_mut();
    let mut opts = bpf_tracing_multi_opts::default();
    let mut fillers: *mut *mut tracing_multi_rollback = ptr::null_mut();
    let cnt: size_t = FUNCS_CNT;
    let mut ids: *__u32 = ptr::null_mut();
    let mut err: c_int;
    let max: c_int;

    max = get_bpf_max_tramp_links();
    if !ASSERT_GE(max as c_long, 1, c!("bpf_max_tramp_links")) {
        return;
    }

    /* Attach maximum allowed programs to bpf_fentry_test10 */
    fillers = fillers_load_and_link(max);
    if !ASSERT_OK_PTR(fillers as *const c_void, c!("fillers_load_and_link")) {
        return;
    }

    extra = extra_load_and_link();
    if ASSERT_OK_PTR(extra as *const c_void, c!("extra_load_and_link")) {
        skel = tracing_multi_rollback__open();
        if ASSERT_OK_PTR(skel as *const c_void, c!("tracing_multi_rollback__open")) {
            bpf_program__set_autoload((*skel).progs.test_fentry, true);
            bpf_program__set_autoload((*skel).progs.test_fexit, true);

            /*
             * Attach tracing_multi link on bpf_fentry_test1-10, which will
             * fail on bpf_fentry_test10 function, because it already has
             * maximum allowed programs attached.
             *
             * The rollback needs to unlink already link-ed trampolines and
             * put all of them.
             */
            err = tracing_multi_rollback__load(skel);
            if ASSERT_OK(err, c!("tracing_multi_rollback__load")) {
                ids = get_ids(bpf_fentry_test.as_ptr(), cnt as c_int, ptr::null());
                if ASSERT_OK_PTR(ids as *const c_void, c!("get_ids")) {
                    opts.ids = ids;
                    opts.cnt = cnt;

                    (*(*skel).bss).pid = getpid();

                    (*skel).links.test_fentry = bpf_program__attach_tracing_multi((*skel).progs.test_fentry, ptr::null(), &opts);
                    if ASSERT_ERR_PTR((*skel).links.test_fentry as *const c_void, c!("bpf_program__attach_tracing_multi")) {
                        (*skel).links.test_fexit = bpf_program__attach_tracing_multi((*skel).progs.test_fexit, ptr::null(), &opts);
                        if ASSERT_ERR_PTR((*skel).links.test_fexit as *const c_void, c!("bpf_program__attach_tracing_multi")) {
                            tracing_multi_rollback_run(skel);
                        }
                    }
                }
            }
        }
    }

    fillers_cleanup(fillers, max);
    tracing_multi_rollback__destroy(extra);
    tracing_multi_rollback__destroy(skel);
    free(ids as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_tracing_multi_attach_rollback() {
    if test__start_subtest(c!("put")) {
        test_rollback_put();
    }
    if test__start_subtest(c!("unlink")) {
        test_rollback_unlink();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_tracing_multi_test() {
    // Original C skips this test under #ifndef __x86_64__.
    #[cfg(not(target_arch = "x86_64"))]
    {
        test__skip();
        return;
    }

    if test__start_subtest(c!("skel_api")) {
        test_skel_api();
    }
    if test__start_subtest(c!("link_api_pattern")) {
        test_link_api_pattern();
    }
    if test__start_subtest(c!("link_api_ids")) {
        test_link_api_ids(false);
    }
    if test__start_subtest(c!("module_skel_api")) {
        test_module_skel_api();
    }
    if test__start_subtest(c!("module_link_api_pattern")) {
        test_module_link_api_pattern();
    }
    if test__start_subtest(c!("module_link_api_ids")) {
        test_module_link_api_ids();
    }
    if test__start_subtest(c!("intersect")) {
        test_intersect();
    }
    if test__start_subtest(c!("cookies")) {
        test_link_api_ids(true);
    }
    if test__start_subtest(c!("session")) {
        test_session();
    }
    if test__start_subtest(c!("attach_api_fails")) {
        test_attach_api_fails();
    }
    RUN_TESTS_tracing_multi_verifier();
    if test__start_subtest(c!("fentry_after_multi")) {
        test_fentry_after_multi();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
