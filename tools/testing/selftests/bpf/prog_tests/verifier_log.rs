// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies translated as external declarations:
// <test_progs.h>, <bpf/btf.h>, "test_log_buf.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type u32 = c_uint;
type __u32 = c_uint;

const BPF_PROG_TYPE_RAW_TRACEPOINT: c_int = 17;
const ENOSPC: c_int = 28;
const BPF_LOG_FIXED: c_uint = 8;

#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: u32,
    pub info: u32,
    pub size: u32,
}

#[repr(C)]
pub struct bpf_prog_load_opts {
    pub sz: size_t,
    pub log_buf: *mut c_char,
    pub log_size: __u32,
    pub log_level: __u32,
    pub log_true_size: __u32,
}

#[repr(C)]
pub struct bpf_btf_load_opts {
    pub sz: size_t,
    pub log_buf: *mut c_char,
    pub log_size: __u32,
    pub log_level: __u32,
    pub log_true_size: __u32,
}

#[repr(C)]
pub struct test_log_buf_progs {
    pub good_prog: *mut bpf_program,
}

#[repr(C)]
pub struct test_log_buf {
    pub obj: *mut bpf_object,
    pub progs: test_log_buf_progs,
}

#[repr(C)]
struct Logs {
    /* strategically placed before others to avoid accidental modification by kernel */
    filler: [c_char; 16384],
    buf: [c_char; 16384],
    /* strategically placed after buf[] to catch more accidental corruptions */
    reference: [c_char; 16384],
}

unsafe extern "C" {
    fn close(fd: c_int) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> isize;

    fn ASSERT_LT(actual: c_int, expected: c_int, tag: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, tag: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, tag: *const c_char) -> bool;
    fn ASSERT_ERR(actual: c_int, tag: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, tag: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, tag: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, tag: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, tag: *const c_char) -> bool;
    fn ASSERT_STRNEQ(
        actual: *const c_char,
        expected: *const c_char,
        len: size_t,
        tag: *const c_char,
    ) -> bool;

    fn bpf_prog_load(
        prog_type: c_int,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *mut bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_btf_load(
        btf_data: *const c_void,
        btf_data_sz: u32,
        opts: *mut bpf_btf_load_opts,
    ) -> c_int;

    fn test_log_buf__open() -> *mut test_log_buf;
    fn test_log_buf__load(skel: *mut test_log_buf) -> c_int;
    fn test_log_buf__destroy(skel: *mut test_log_buf);

    fn bpf_object__next_program(
        obj: *const bpf_object,
        prog: *mut bpf_program,
    ) -> *mut bpf_program;
    fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__insns(prog: *const bpf_program) -> *const bpf_insn;
    fn bpf_program__insn_cnt(prog: *const bpf_program) -> size_t;

    fn btf__new_empty() -> *mut btf;
    fn btf__add_int(btf: *mut btf, name: *const c_char, sz: u32, encoding: u32) -> c_int;
    fn btf__type_by_id(btf: *const btf, type_id: u32) -> *const btf_type;
    fn btf__raw_data(btf: *const btf, btf_size: *mut u32) -> *const c_void;
    fn btf__free(btf: *mut btf);

    fn test__start_subtest(name: *const c_char) -> bool;
}

static mut logs: Logs = Logs {
    filler: [0; 16384],
    buf: [0; 16384],
    reference: [0; 16384],
};
static mut insns: *const bpf_insn = ptr::null();
static mut insn_cnt: size_t = 0;

unsafe fn check_prog_load(prog_fd: c_int, expect_err: bool, tag: *const c_char) -> bool {
    if expect_err {
        if !ASSERT_LT(prog_fd, 0, tag) {
            close(prog_fd);
            return false;
        }
    } else {
        /* !expect_err */
        if !ASSERT_GT(prog_fd, 0, tag) {
            return false;
        }
    }
    if prog_fd >= 0 {
        close(prog_fd);
    }
    true
}

unsafe fn load_prog(opts: *mut bpf_prog_load_opts, expect_load_error: bool) -> c_int {
    let prog_fd: c_int;

    prog_fd = bpf_prog_load(
        BPF_PROG_TYPE_RAW_TRACEPOINT,
        c"log_prog".as_ptr(),
        c"GPL".as_ptr(),
        insns,
        insn_cnt,
        opts,
    );
    check_prog_load(prog_fd, expect_load_error, c"prog_load".as_ptr());

    prog_fd
}

unsafe fn verif_log_subtest(name: *const c_char, expect_load_error: bool, log_level: c_int) {
    let mut opts: bpf_prog_load_opts = zeroed();
    opts.sz = size_of::<bpf_prog_load_opts>();
    let mut exp_log: *mut c_char;
    let mut prog_name: [c_char; 24] = [0; 24];
    let mut op_name: [c_char; 32] = [0; 32];
    let mut skel: *mut test_log_buf;
    let mut prog: *mut bpf_program;
    let fixed_log_sz: size_t;
    let log_true_sz_fixed: __u32;
    let log_true_sz_rolling: __u32;
    let mut i: c_int;
    let mut mode: c_int;
    let mut err: c_int;
    let mut prog_fd: c_int;
    let mut res: c_int;

    skel = test_log_buf__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        return;
    }

    prog = ptr::null_mut();
    loop {
        prog = bpf_object__next_program((*skel).obj, prog);
        if prog.is_null() {
            break;
        }
        if strcmp(bpf_program__name(prog), name) == 0 {
            bpf_program__set_autoload(prog, true);
        } else {
            bpf_program__set_autoload(prog, false);
        }
    }

    err = test_log_buf__load(skel);
    if !expect_load_error && !ASSERT_OK(err, c"unexpected_load_failure".as_ptr()) {
        goto_cleanup_log(skel);
        return;
    }
    if expect_load_error && !ASSERT_ERR(err, c"unexpected_load_success".as_ptr()) {
        goto_cleanup_log(skel);
        return;
    }

    insns = bpf_program__insns((*skel).progs.good_prog);
    insn_cnt = bpf_program__insn_cnt((*skel).progs.good_prog);

    opts.log_buf = logs.reference.as_mut_ptr();
    opts.log_size = size_of_val_u32(&logs.reference);
    opts.log_level = (log_level as __u32) | BPF_LOG_FIXED;
    load_prog(&mut opts, expect_load_error);

    fixed_log_sz = strlen(logs.reference.as_ptr()) + 1;
    if !ASSERT_GT(fixed_log_sz as c_int, 50, c"fixed_log_sz".as_ptr()) {
        goto_cleanup_log(skel);
        return;
    }
    memset(
        logs.reference.as_mut_ptr().add(fixed_log_sz) as *mut c_void,
        0,
        size_of_val_usize(&logs.reference) - fixed_log_sz,
    );

    /* validate BPF_LOG_FIXED works as verifier log used to work, that is:
     * we get -ENOSPC and beginning of the full verifier log. This only
     * works for log_level 2 and log_level 1 + failed program. For log
     * level 2 we don't reset log at all. For log_level 1 + failed program
     * we don't get to verification stats output. With log level 1
     * for successful program  final result will be just verifier stats.
     * But if provided too short log buf, kernel will NULL-out log->ubuf
     * and will stop emitting further log. This means we'll never see
     * predictable verifier stats.
     * Long story short, we do the following -ENOSPC test only for
     * predictable combinations.
     */
    if log_level >= 2 || expect_load_error {
        opts.log_buf = logs.buf.as_mut_ptr();
        opts.log_level = (log_level as __u32) | BPF_LOG_FIXED; /* fixed-length log */
        opts.log_size = 25;

        prog_fd = bpf_prog_load(
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            c"log_fixed25".as_ptr(),
            c"GPL".as_ptr(),
            insns,
            insn_cnt,
            &mut opts,
        );
        if !ASSERT_EQ(
            prog_fd,
            -ENOSPC,
            c"unexpected_log_fixed_prog_load_result".as_ptr(),
        ) {
            if prog_fd >= 0 {
                close(prog_fd);
            }
            goto_cleanup_log(skel);
            return;
        }
        if !ASSERT_EQ(strlen(logs.buf.as_ptr()) as c_int, 24, c"log_fixed_25".as_ptr()) {
            goto_cleanup_log(skel);
            return;
        }
        if !ASSERT_STRNEQ(
            logs.buf.as_ptr(),
            logs.reference.as_ptr(),
            24,
            c"log_fixed_contents_25".as_ptr(),
        ) {
            goto_cleanup_log(skel);
            return;
        }
    }

    /* validate rolling verifier log logic: try all variations of log buf
     * length to force various truncation scenarios
     */
    opts.log_buf = logs.buf.as_mut_ptr();

    /* rotating mode, then fixed mode */
    mode = 1;
    while mode >= 0 {
        /* prefill logs.buf with 'A's to detect any write beyond allowed length */
        memset(logs.filler.as_mut_ptr() as *mut c_void, 'A' as c_int, size_of_val_usize(&logs.filler));
        logs.filler[size_of_val_usize(&logs.filler) - 1] = 0;
        memset(logs.buf.as_mut_ptr() as *mut c_void, 'A' as c_int, size_of_val_usize(&logs.buf));
        logs.buf[size_of_val_usize(&logs.buf) - 1] = 0;

        i = 1;
        while (i as size_t) < fixed_log_sz {
            opts.log_size = i as __u32;
            opts.log_level = (log_level as __u32) | if mode != 0 { 0 } else { BPF_LOG_FIXED };

            snprintf(
                prog_name.as_mut_ptr(),
                size_of_val_usize(&prog_name),
                c"log_%s_%d".as_ptr(),
                if mode != 0 { c"roll".as_ptr() } else { c"fixed".as_ptr() },
                i,
            );
            prog_fd = bpf_prog_load(
                BPF_PROG_TYPE_RAW_TRACEPOINT,
                prog_name.as_ptr(),
                c"GPL".as_ptr(),
                insns,
                insn_cnt,
                &mut opts,
            );

            snprintf(
                op_name.as_mut_ptr(),
                size_of_val_usize(&op_name),
                c"log_%s_prog_load_%d".as_ptr(),
                if mode != 0 { c"roll".as_ptr() } else { c"fixed".as_ptr() },
                i,
            );
            if !ASSERT_EQ(prog_fd, -ENOSPC, op_name.as_ptr()) {
                if prog_fd >= 0 {
                    close(prog_fd);
                }
                goto_cleanup_log(skel);
                return;
            }

            snprintf(
                op_name.as_mut_ptr(),
                size_of_val_usize(&op_name),
                c"log_%s_strlen_%d".as_ptr(),
                if mode != 0 { c"roll".as_ptr() } else { c"fixed".as_ptr() },
                i,
            );
            ASSERT_EQ(strlen(logs.buf.as_ptr()) as c_int, i - 1, op_name.as_ptr());

            if mode != 0 {
                exp_log = logs.reference.as_mut_ptr().add(fixed_log_sz - i as size_t);
            } else {
                exp_log = logs.reference.as_mut_ptr();
            }

            snprintf(
                op_name.as_mut_ptr(),
                size_of_val_usize(&op_name),
                c"log_%s_contents_%d".as_ptr(),
                if mode != 0 { c"roll".as_ptr() } else { c"fixed".as_ptr() },
                i,
            );
            if !ASSERT_STRNEQ(logs.buf.as_ptr(), exp_log, (i - 1) as size_t, op_name.as_ptr()) {
                printf(
                    c"CMP:%d\nS1:'%s'\nS2:'%s'\n".as_ptr(),
                    strncmp(logs.buf.as_ptr(), exp_log, (i - 1) as size_t),
                    logs.buf.as_ptr(),
                    exp_log,
                );
                goto_cleanup_log(skel);
                return;
            }

            /* check that unused portions of logs.buf is not overwritten */
            snprintf(
                op_name.as_mut_ptr(),
                size_of_val_usize(&op_name),
                c"log_%s_unused_%d".as_ptr(),
                if mode != 0 { c"roll".as_ptr() } else { c"fixed".as_ptr() },
                i,
            );
            if !ASSERT_STREQ(
                logs.buf.as_ptr().add(i as size_t),
                logs.filler.as_ptr().add(i as size_t),
                op_name.as_ptr(),
            ) {
                printf(
                    c"CMP:%d\nS1:'%s'\nS2:'%s'\n".as_ptr(),
                    strcmp(
                        logs.buf.as_ptr().add(i as size_t),
                        logs.filler.as_ptr().add(i as size_t),
                    ),
                    logs.buf.as_ptr().add(i as size_t),
                    logs.filler.as_ptr().add(i as size_t),
                );
                goto_cleanup_log(skel);
                return;
            }
            i += 1;
        }
        mode -= 1;
    }

    /* (FIXED) get actual log size */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = (log_level as __u32) | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = size_of_val_u32(&logs.buf);
    opts.log_true_size = 0;
    res = load_prog(&mut opts, expect_load_error);
    ASSERT_NEQ(res, -ENOSPC, c"prog_load_res_fixed".as_ptr());

    log_true_sz_fixed = opts.log_true_size;
    ASSERT_GT(log_true_sz_fixed as c_int, 0, c"log_true_sz_fixed".as_ptr());

    /* (FIXED, NULL) get actual log size */
    opts.log_buf = ptr::null_mut();
    opts.log_level = (log_level as __u32) | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = 0;
    opts.log_true_size = 0;
    res = load_prog(&mut opts, expect_load_error);
    ASSERT_NEQ(res, -ENOSPC, c"prog_load_res_fixed_null".as_ptr());
    ASSERT_EQ(
        opts.log_true_size as c_int,
        log_true_sz_fixed as c_int,
        c"log_sz_fixed_null_eq".as_ptr(),
    );

    /* (ROLLING) get actual log size */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = log_level as __u32;
    opts.log_size = size_of_val_u32(&logs.buf);
    opts.log_true_size = 0;
    res = load_prog(&mut opts, expect_load_error);
    ASSERT_NEQ(res, -ENOSPC, c"prog_load_res_rolling".as_ptr());

    log_true_sz_rolling = opts.log_true_size;
    ASSERT_EQ(
        log_true_sz_rolling as c_int,
        log_true_sz_fixed as c_int,
        c"log_true_sz_eq".as_ptr(),
    );

    /* (ROLLING, NULL) get actual log size */
    opts.log_buf = ptr::null_mut();
    opts.log_level = log_level as __u32;
    opts.log_size = 0;
    opts.log_true_size = 0;
    res = load_prog(&mut opts, expect_load_error);
    ASSERT_NEQ(res, -ENOSPC, c"prog_load_res_rolling_null".as_ptr());
    ASSERT_EQ(
        opts.log_true_size as c_int,
        log_true_sz_rolling as c_int,
        c"log_true_sz_null_eq".as_ptr(),
    );

    /* (FIXED) expect -ENOSPC for one byte short log */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = (log_level as __u32) | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = log_true_sz_fixed - 1;
    opts.log_true_size = 0;
    res = load_prog(&mut opts, true /* should fail */);
    ASSERT_EQ(res, -ENOSPC, c"prog_load_res_too_short_fixed".as_ptr());

    /* (FIXED) expect *not* -ENOSPC with exact log_true_size buffer */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = (log_level as __u32) | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = log_true_sz_fixed;
    opts.log_true_size = 0;
    res = load_prog(&mut opts, expect_load_error);
    ASSERT_NEQ(res, -ENOSPC, c"prog_load_res_just_right_fixed".as_ptr());

    /* (ROLLING) expect -ENOSPC for one byte short log */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = log_level as __u32;
    opts.log_size = log_true_sz_rolling - 1;
    res = load_prog(&mut opts, true /* should fail */);
    ASSERT_EQ(res, -ENOSPC, c"prog_load_res_too_short_rolling".as_ptr());

    /* (ROLLING) expect *not* -ENOSPC with exact log_true_size buffer */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = log_level as __u32;
    opts.log_size = log_true_sz_rolling;
    opts.log_true_size = 0;
    res = load_prog(&mut opts, expect_load_error);
    ASSERT_NEQ(res, -ENOSPC, c"prog_load_res_just_right_rolling".as_ptr());

    goto_cleanup_log(skel);
}

unsafe fn goto_cleanup_log(skel: *mut test_log_buf) {
    test_log_buf__destroy(skel);
}

static mut btf_data: *const c_void = ptr::null();
static mut btf_data_sz: u32 = 0;

unsafe fn load_btf(opts: *mut bpf_btf_load_opts, expect_err: bool) -> c_int {
    let fd: c_int;

    fd = bpf_btf_load(btf_data, btf_data_sz, opts);
    if fd >= 0 {
        close(fd);
    }
    if expect_err {
        ASSERT_LT(fd, 0, c"btf_load_failure".as_ptr());
    } else {
        /* !expect_err */
        ASSERT_GT(fd, 0, c"btf_load_success".as_ptr());
    }
    fd
}

unsafe fn verif_btf_log_subtest(bad_btf: bool) {
    let mut opts: bpf_btf_load_opts = zeroed();
    opts.sz = size_of::<bpf_btf_load_opts>();
    let mut btf: *mut btf;
    let mut t: *mut btf_type;
    let mut exp_log: *mut c_char;
    let mut op_name: [c_char; 32] = [0; 32];
    let fixed_log_sz: size_t;
    let log_true_sz_fixed: __u32;
    let log_true_sz_rolling: __u32;
    let mut i: c_int;
    let mut res: c_int;

    /* prepare simple BTF contents */
    btf = btf__new_empty();
    if !ASSERT_OK_PTR(btf as *const c_void, c"btf_new_empty".as_ptr()) {
        return;
    }
    res = btf__add_int(btf, c"whatever".as_ptr(), 4, 0);
    if !ASSERT_GT(res, 0, c"btf_add_int_id".as_ptr()) {
        goto_cleanup_btf(btf);
        return;
    }
    if bad_btf {
        /* btf__add_int() doesn't allow bad value of size, so we'll just
         * force-cast btf_type pointer and manually override size to invalid
         * 3 if we need to simulate failure
         */
        t = btf__type_by_id(btf, res as u32) as *mut btf_type;
        if !ASSERT_OK_PTR(t as *const c_void, c"int_btf_type".as_ptr()) {
            goto_cleanup_btf(btf);
            return;
        }
        (*t).size = 3;
    }

    btf_data = btf__raw_data(btf, &mut btf_data_sz);
    if !ASSERT_OK_PTR(btf_data, c"btf_data".as_ptr()) {
        goto_cleanup_btf(btf);
        return;
    }

    load_btf(&mut opts, bad_btf);

    opts.log_buf = logs.reference.as_mut_ptr();
    opts.log_size = size_of_val_u32(&logs.reference);
    opts.log_level = 1 | BPF_LOG_FIXED;
    load_btf(&mut opts, bad_btf);

    fixed_log_sz = strlen(logs.reference.as_ptr()) + 1;
    if !ASSERT_GT(fixed_log_sz as c_int, 50, c"fixed_log_sz".as_ptr()) {
        goto_cleanup_btf(btf);
        return;
    }
    memset(
        logs.reference.as_mut_ptr().add(fixed_log_sz) as *mut c_void,
        0,
        size_of_val_usize(&logs.reference) - fixed_log_sz,
    );

    /* validate BPF_LOG_FIXED truncation works as verifier log used to work */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1 | BPF_LOG_FIXED; /* fixed-length log */
    opts.log_size = 25;
    res = load_btf(&mut opts, true);
    ASSERT_EQ(res, -ENOSPC, c"half_log_fd".as_ptr());
    ASSERT_EQ(strlen(logs.buf.as_ptr()) as c_int, 24, c"log_fixed_25".as_ptr());
    strscpy(op_name.as_mut_ptr(), c"log_fixed".as_ptr(), size_of_val_usize(&op_name));
    ASSERT_STRNEQ(logs.buf.as_ptr(), logs.reference.as_ptr(), 24, op_name.as_ptr());

    /* validate rolling verifier log logic: try all variations of log buf
     * length to force various truncation scenarios
     */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1; /* rolling log */

    /* prefill logs.buf with 'A's to detect any write beyond allowed length */
    memset(logs.filler.as_mut_ptr() as *mut c_void, 'A' as c_int, size_of_val_usize(&logs.filler));
    logs.filler[size_of_val_usize(&logs.filler) - 1] = 0;
    memset(logs.buf.as_mut_ptr() as *mut c_void, 'A' as c_int, size_of_val_usize(&logs.buf));
    logs.buf[size_of_val_usize(&logs.buf) - 1] = 0;

    i = 1;
    while (i as size_t) < fixed_log_sz {
        opts.log_size = i as __u32;

        snprintf(
            op_name.as_mut_ptr(),
            size_of_val_usize(&op_name),
            c"log_roll_btf_load_%d".as_ptr(),
            i,
        );
        res = load_btf(&mut opts, true);
        if !ASSERT_EQ(res, -ENOSPC, op_name.as_ptr()) {
            goto_cleanup_btf(btf);
            return;
        }

        exp_log = logs.reference.as_mut_ptr().add(fixed_log_sz - i as size_t);
        snprintf(
            op_name.as_mut_ptr(),
            size_of_val_usize(&op_name),
            c"log_roll_contents_%d".as_ptr(),
            i,
        );
        if !ASSERT_STREQ(logs.buf.as_ptr(), exp_log, op_name.as_ptr()) {
            printf(
                c"CMP:%d\nS1:'%s'\nS2:'%s'\n".as_ptr(),
                strcmp(logs.buf.as_ptr(), exp_log),
                logs.buf.as_ptr(),
                exp_log,
            );
            goto_cleanup_btf(btf);
            return;
        }

        /* check that unused portions of logs.buf are not overwritten */
        snprintf(
            op_name.as_mut_ptr(),
            size_of_val_usize(&op_name),
            c"log_roll_unused_tail_%d".as_ptr(),
            i,
        );
        if !ASSERT_STREQ(
            logs.buf.as_ptr().add(i as size_t),
            logs.filler.as_ptr().add(i as size_t),
            op_name.as_ptr(),
        ) {
            printf(
                c"CMP:%d\nS1:'%s'\nS2:'%s'\n".as_ptr(),
                strcmp(
                    logs.buf.as_ptr().add(i as size_t),
                    logs.filler.as_ptr().add(i as size_t),
                ),
                logs.buf.as_ptr().add(i as size_t),
                logs.filler.as_ptr().add(i as size_t),
            );
            goto_cleanup_btf(btf);
            return;
        }
        i += 1;
    }

    /* (FIXED) get actual log size */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1 | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = size_of_val_u32(&logs.buf);
    opts.log_true_size = 0;
    res = load_btf(&mut opts, bad_btf);
    ASSERT_NEQ(res, -ENOSPC, c"btf_load_res_fixed".as_ptr());

    log_true_sz_fixed = opts.log_true_size;
    ASSERT_GT(log_true_sz_fixed as c_int, 0, c"log_true_sz_fixed".as_ptr());

    /* (FIXED, NULL) get actual log size */
    opts.log_buf = ptr::null_mut();
    opts.log_level = 1 | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = 0;
    opts.log_true_size = 0;
    res = load_btf(&mut opts, bad_btf);
    ASSERT_NEQ(res, -ENOSPC, c"btf_load_res_fixed_null".as_ptr());
    ASSERT_EQ(
        opts.log_true_size as c_int,
        log_true_sz_fixed as c_int,
        c"log_sz_fixed_null_eq".as_ptr(),
    );

    /* (ROLLING) get actual log size */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1;
    opts.log_size = size_of_val_u32(&logs.buf);
    opts.log_true_size = 0;
    res = load_btf(&mut opts, bad_btf);
    ASSERT_NEQ(res, -ENOSPC, c"btf_load_res_rolling".as_ptr());

    log_true_sz_rolling = opts.log_true_size;
    ASSERT_EQ(
        log_true_sz_rolling as c_int,
        log_true_sz_fixed as c_int,
        c"log_true_sz_eq".as_ptr(),
    );

    /* (ROLLING, NULL) get actual log size */
    opts.log_buf = ptr::null_mut();
    opts.log_level = 1;
    opts.log_size = 0;
    opts.log_true_size = 0;
    res = load_btf(&mut opts, bad_btf);
    ASSERT_NEQ(res, -ENOSPC, c"btf_load_res_rolling_null".as_ptr());
    ASSERT_EQ(
        opts.log_true_size as c_int,
        log_true_sz_rolling as c_int,
        c"log_true_sz_null_eq".as_ptr(),
    );

    /* (FIXED) expect -ENOSPC for one byte short log */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1 | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = log_true_sz_fixed - 1;
    opts.log_true_size = 0;
    res = load_btf(&mut opts, true);
    ASSERT_EQ(res, -ENOSPC, c"btf_load_res_too_short_fixed".as_ptr());

    /* (FIXED) expect *not* -ENOSPC with exact log_true_size buffer */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1 | BPF_LOG_FIXED; /* BPF_LOG_FIXED */
    opts.log_size = log_true_sz_fixed;
    opts.log_true_size = 0;
    res = load_btf(&mut opts, bad_btf);
    ASSERT_NEQ(res, -ENOSPC, c"btf_load_res_just_right_fixed".as_ptr());

    /* (ROLLING) expect -ENOSPC for one byte short log */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1;
    opts.log_size = log_true_sz_rolling - 1;
    res = load_btf(&mut opts, true);
    ASSERT_EQ(res, -ENOSPC, c"btf_load_res_too_short_rolling".as_ptr());

    /* (ROLLING) expect *not* -ENOSPC with exact log_true_size buffer */
    opts.log_buf = logs.buf.as_mut_ptr();
    opts.log_level = 1;
    opts.log_size = log_true_sz_rolling;
    opts.log_true_size = 0;
    res = load_btf(&mut opts, bad_btf);
    ASSERT_NEQ(res, -ENOSPC, c"btf_load_res_just_right_rolling".as_ptr());

    goto_cleanup_btf(btf);
}

unsafe fn goto_cleanup_btf(btf: *mut btf) {
    btf__free(btf);
}

fn size_of_val_usize<T>(val: &T) -> usize {
    size_of_val(val)
}

fn size_of_val_u32<T>(val: &T) -> u32 {
    size_of_val(val) as u32
}

fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

#[no_mangle]
pub unsafe extern "C" fn test_verifier_log() {
    if test__start_subtest(c"good_prog-level1".as_ptr()) {
        verif_log_subtest(c"good_prog".as_ptr(), false, 1);
    }
    if test__start_subtest(c"good_prog-level2".as_ptr()) {
        verif_log_subtest(c"good_prog".as_ptr(), false, 2);
    }
    if test__start_subtest(c"bad_prog-level1".as_ptr()) {
        verif_log_subtest(c"bad_prog".as_ptr(), true, 1);
    }
    if test__start_subtest(c"bad_prog-level2".as_ptr()) {
        verif_log_subtest(c"bad_prog".as_ptr(), true, 2);
    }
    if test__start_subtest(c"bad_btf".as_ptr()) {
        verif_btf_log_subtest(true /* bad btf */);
    }
    if test__start_subtest(c"good_btf".as_ptr()) {
        verif_btf_log_subtest(false /* !bad btf */);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
