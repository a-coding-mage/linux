// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Translated from C implementation source. External libbpf/selftest symbols are
 * declared here as dependencies supplied by the surrounding repository.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type __u32 = u32;
type va_list = *mut c_void;
type libbpf_print_level = c_uint;
type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(level: libbpf_print_level, fmt: *const c_char, args: va_list) -> c_int,
>;

const BPF_REG_0: c_int = 0;
const BPF_PROG_TYPE_SOCKET_FILTER: c_int = 1;
const VERBOSE_NONE: c_int = 0;

#[repr(C)]
struct bpf_insn {
    code: u8,
    dst_src_reg: u8,
    off: i16,
    imm: i32,
}

#[repr(C)]
struct bpf_object_open_opts {
    kernel_log_buf: *mut c_char,
    kernel_log_size: size_t,
    kernel_log_level: c_int,
}

#[repr(C)]
struct bpf_prog_load_opts {
    log_buf: *mut c_char,
    log_size: size_t,
    log_level: c_int,
}

#[repr(C)]
struct bpf_btf_load_opts {
    log_buf: *mut c_char,
    log_size: size_t,
    log_level: c_int,
}

#[repr(C)]
struct test_log_buf_progs {
    good_prog: *mut c_void,
    bad_prog: *mut c_void,
}

#[repr(C)]
struct test_log_buf {
    progs: test_log_buf_progs,
}

#[repr(C)]
struct btf {
    _private: [u8; 0],
}

#[repr(C)]
struct env {
    verbosity: c_int,
}

unsafe extern "C" {
    static env: env;

    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, arg: va_list) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;
    fn test_log_buf__open_opts(opts: *const bpf_object_open_opts) -> *mut test_log_buf;
    fn test_log_buf__load(skel: *mut test_log_buf) -> c_int;
    fn test_log_buf__destroy(skel: *mut test_log_buf);
    fn bpf_program__set_log_buf(prog: *mut c_void, log_buf: *mut c_char, log_size: size_t);
    fn bpf_program__set_log_level(prog: *mut c_void, log_level: c_int);
    fn bpf_prog_load(
        prog_type: c_int,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_btf_load(raw_btf_data: *const c_void, raw_btf_size: __u32, opts: *const bpf_btf_load_opts)
        -> c_int;
    fn btf__new_empty() -> *mut btf;
    fn btf__add_int(btf: *mut btf, name: *const c_char, size: c_uint, encoding: c_uint) -> c_int;
    fn btf__raw_data(btf: *mut btf, size: *mut __u32) -> *const c_void;
    fn btf__add_ptr(btf: *mut btf, type_id: c_int) -> c_int;
    fn btf__free(btf: *mut btf);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> *mut c_void;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_NULL(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_LT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

static mut LIBBPF_LOG_POS: size_t = 0;
static mut LIBBPF_LOG_BUF: [c_char; 1024 * 1024] = [0; 1024 * 1024];
static mut LIBBPF_LOG_ERROR: bool = false;

const fn bpf_mov64_imm(dst: c_int, imm: c_int) -> bpf_insn {
    bpf_insn {
        code: 0xb7,
        dst_src_reg: dst as u8,
        off: 0,
        imm,
    }
}

const fn bpf_exit_insn() -> bpf_insn {
    bpf_insn {
        code: 0x95,
        dst_src_reg: 0,
        off: 0,
        imm: 0,
    }
}

unsafe extern "C" fn libbpf_print_cb(
    _level: libbpf_print_level,
    fmt: *const c_char,
    args: va_list,
) -> c_int {
    let emitted_cnt: c_int;
    let left_cnt: size_t;

    left_cnt = core::mem::size_of_val(&LIBBPF_LOG_BUF) - LIBBPF_LOG_POS;
    emitted_cnt = vsnprintf(
        LIBBPF_LOG_BUF.as_mut_ptr().add(LIBBPF_LOG_POS),
        left_cnt,
        fmt,
        args,
    );

    if emitted_cnt < 0 || emitted_cnt as size_t + 1 > left_cnt {
        LIBBPF_LOG_ERROR = true;
        return 0;
    }

    LIBBPF_LOG_POS += emitted_cnt as size_t;
    0
}

unsafe fn obj_load_log_buf() {
    let old_print_cb: libbpf_print_fn_t = libbpf_set_print(Some(libbpf_print_cb));
    let mut opts = bpf_object_open_opts {
        kernel_log_buf: core::ptr::null_mut(),
        kernel_log_size: 0,
        kernel_log_level: 0,
    };
    let log_buf_sz: size_t = 1024 * 1024;
    let mut skel: *mut test_log_buf;
    let obj_log_buf: *mut c_char;
    let good_log_buf: *mut c_char;
    let bad_log_buf: *mut c_char;
    let mut err: c_int;

    obj_log_buf = malloc(3 * log_buf_sz) as *mut c_char;
    if ASSERT_OK_PTR(obj_log_buf as *const c_void, c"obj_log_buf".as_ptr()) == core::ptr::null_mut()
    {
        return;
    }

    good_log_buf = obj_log_buf.add(log_buf_sz);
    bad_log_buf = obj_log_buf.add(2 * log_buf_sz);
    *bad_log_buf = 0;
    *good_log_buf = *bad_log_buf;
    *obj_log_buf = *good_log_buf;

    opts.kernel_log_buf = obj_log_buf;
    opts.kernel_log_size = log_buf_sz;
    opts.kernel_log_level = 4; /* for BTF this will turn into 1 */

    /* In the first round every prog has its own log_buf, so libbpf logs
     * don't have program failure logs
     */
    skel = test_log_buf__open_opts(&opts);
    if ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) == core::ptr::null_mut() {
        goto_cleanup(obj_log_buf, skel, old_print_cb);
        return;
    }

    /* set very verbose level for good_prog so we always get detailed logs */
    bpf_program__set_log_buf((*skel).progs.good_prog, good_log_buf, log_buf_sz);
    bpf_program__set_log_level((*skel).progs.good_prog, 2);

    bpf_program__set_log_buf((*skel).progs.bad_prog, bad_log_buf, log_buf_sz);
    /* log_level 0 with custom log_buf means that verbose logs are not
     * requested if program load is successful, but libbpf should retry
     * with log_level 1 on error and put program's verbose load log into
     * custom log_buf
     */
    bpf_program__set_log_level((*skel).progs.bad_prog, 0);

    err = test_log_buf__load(skel);
    if !ASSERT_ERR(err, c"unexpected_load_success".as_ptr()) {
        goto_cleanup(obj_log_buf, skel, old_print_cb);
        return;
    }

    ASSERT_FALSE(LIBBPF_LOG_ERROR, c"libbpf_log_error".as_ptr());

    /* there should be no prog loading log because we specified per-prog log buf */
    ASSERT_NULL(
        strstr(
            LIBBPF_LOG_BUF.as_ptr(),
            c"-- BEGIN PROG LOAD LOG --".as_ptr(),
        ) as *const c_void,
        c"unexp_libbpf_log".as_ptr(),
    );
    ASSERT_OK_PTR(
        strstr(
            LIBBPF_LOG_BUF.as_ptr(),
            c"prog 'bad_prog': BPF program load failed".as_ptr(),
        ) as *const c_void,
        c"libbpf_log_not_empty".as_ptr(),
    );
    ASSERT_OK_PTR(
        strstr(obj_log_buf, c"DATASEC license".as_ptr()) as *const c_void,
        c"obj_log_not_empty".as_ptr(),
    );
    ASSERT_OK_PTR(
        strstr(good_log_buf, c"0: R1=ctx() R10=fp0".as_ptr()) as *const c_void,
        c"good_log_verbose".as_ptr(),
    );
    ASSERT_OK_PTR(
        strstr(
            bad_log_buf,
            c"invalid access to map value, value_size=16 off=16000 size=4".as_ptr(),
        ) as *const c_void,
        c"bad_log_not_empty".as_ptr(),
    );

    if env.verbosity > VERBOSE_NONE {
        printf(
            c"LIBBPF LOG:   \n=================\n%s=================\n".as_ptr(),
            LIBBPF_LOG_BUF.as_ptr(),
        );
        printf(
            c"OBJ LOG:      \n=================\n%s=================\n".as_ptr(),
            obj_log_buf,
        );
        printf(
            c"GOOD_PROG LOG:\n=================\n%s=================\n".as_ptr(),
            good_log_buf,
        );
        printf(
            c"BAD_PROG  LOG:\n=================\n%s=================\n".as_ptr(),
            bad_log_buf,
        );
    }

    /* reset everything */
    test_log_buf__destroy(skel);
    *bad_log_buf = 0;
    *good_log_buf = *bad_log_buf;
    *obj_log_buf = *good_log_buf;
    LIBBPF_LOG_BUF[0] = 0;
    LIBBPF_LOG_POS = 0;
    LIBBPF_LOG_ERROR = false;

    /* In the second round we let bad_prog's failure be logged through print callback */
    opts.kernel_log_buf = core::ptr::null_mut(); /* let everything through into print callback */
    opts.kernel_log_size = 0;
    opts.kernel_log_level = 1;

    skel = test_log_buf__open_opts(&opts);
    if ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) == core::ptr::null_mut() {
        goto_cleanup(obj_log_buf, skel, old_print_cb);
        return;
    }

    /* set normal verbose level for good_prog to check log_level is taken into account */
    bpf_program__set_log_buf((*skel).progs.good_prog, good_log_buf, log_buf_sz);
    bpf_program__set_log_level((*skel).progs.good_prog, 1);

    err = test_log_buf__load(skel);
    if !ASSERT_ERR(err, c"unexpected_load_success".as_ptr()) {
        goto_cleanup(obj_log_buf, skel, old_print_cb);
        return;
    }

    ASSERT_FALSE(LIBBPF_LOG_ERROR, c"libbpf_log_error".as_ptr());

    /* this time prog loading error should be logged through print callback */
    ASSERT_OK_PTR(
        strstr(
            LIBBPF_LOG_BUF.as_ptr(),
            c"libbpf: prog 'bad_prog': -- BEGIN PROG LOAD LOG --".as_ptr(),
        ) as *const c_void,
        c"libbpf_log_correct".as_ptr(),
    );
    ASSERT_STREQ(obj_log_buf, c"".as_ptr(), c"obj_log__empty".as_ptr());
    ASSERT_STREQ(
        good_log_buf,
        c"processed 4 insns (limit 1000000) max_states_per_insn 0 total_states 0 peak_states 0 mark_read 0\n".as_ptr(),
        c"good_log_ok".as_ptr(),
    );
    ASSERT_STREQ(bad_log_buf, c"".as_ptr(), c"bad_log_empty".as_ptr());

    if env.verbosity > VERBOSE_NONE {
        printf(
            c"LIBBPF LOG:   \n=================\n%s=================\n".as_ptr(),
            LIBBPF_LOG_BUF.as_ptr(),
        );
        printf(
            c"OBJ LOG:      \n=================\n%s=================\n".as_ptr(),
            obj_log_buf,
        );
        printf(
            c"GOOD_PROG LOG:\n=================\n%s=================\n".as_ptr(),
            good_log_buf,
        );
        printf(
            c"BAD_PROG  LOG:\n=================\n%s=================\n".as_ptr(),
            bad_log_buf,
        );
    }

    goto_cleanup(obj_log_buf, skel, old_print_cb);
}

unsafe fn goto_cleanup(obj_log_buf: *mut c_char, skel: *mut test_log_buf, old_print_cb: libbpf_print_fn_t) {
    free(obj_log_buf as *mut c_void);
    test_log_buf__destroy(skel);
    libbpf_set_print(old_print_cb);
}

unsafe fn bpf_prog_load_log_buf() {
    let good_prog_insns: [bpf_insn; 2] = [bpf_mov64_imm(BPF_REG_0, 0), bpf_exit_insn()];
    let good_prog_insn_cnt: size_t = good_prog_insns.len();
    let bad_prog_insns: [bpf_insn; 1] = [bpf_exit_insn()];
    let bad_prog_insn_cnt: size_t = bad_prog_insns.len();
    let mut opts = bpf_prog_load_opts {
        log_buf: core::ptr::null_mut(),
        log_size: 0,
        log_level: 0,
    };
    let log_buf_sz: size_t = 1024 * 1024;
    let log_buf: *mut c_char;
    let mut fd: c_int = -1;

    log_buf = malloc(log_buf_sz) as *mut c_char;
    if ASSERT_OK_PTR(log_buf as *const c_void, c"log_buf_alloc".as_ptr()) == core::ptr::null_mut()
    {
        return;
    }
    opts.log_buf = log_buf;
    opts.log_size = log_buf_sz;

    /* with log_level == 0 log_buf should stay empty for good prog */
    *log_buf = 0;
    opts.log_level = 0;
    fd = bpf_prog_load(
        BPF_PROG_TYPE_SOCKET_FILTER,
        c"good_prog".as_ptr(),
        c"GPL".as_ptr(),
        good_prog_insns.as_ptr(),
        good_prog_insn_cnt,
        &opts,
    );
    ASSERT_STREQ(log_buf, c"".as_ptr(), c"good_log_0".as_ptr());
    ASSERT_GE(fd, 0, c"good_fd1".as_ptr());
    if fd >= 0 {
        close(fd);
    }

    /* log_level == 2 should always fill log_buf, even for good prog */
    *log_buf = 0;
    opts.log_level = 2;
    fd = bpf_prog_load(
        BPF_PROG_TYPE_SOCKET_FILTER,
        c"good_prog".as_ptr(),
        c"GPL".as_ptr(),
        good_prog_insns.as_ptr(),
        good_prog_insn_cnt,
        &opts,
    );
    ASSERT_OK_PTR(
        strstr(log_buf, c"0: R1=ctx() R10=fp0".as_ptr()) as *const c_void,
        c"good_log_2".as_ptr(),
    );
    ASSERT_GE(fd, 0, c"good_fd2".as_ptr());
    if fd >= 0 {
        close(fd);
    }

    /* log_level == 0 should fill log_buf for bad prog */
    *log_buf = 0;
    opts.log_level = 0;
    fd = bpf_prog_load(
        BPF_PROG_TYPE_SOCKET_FILTER,
        c"bad_prog".as_ptr(),
        c"GPL".as_ptr(),
        bad_prog_insns.as_ptr(),
        bad_prog_insn_cnt,
        &opts,
    );
    ASSERT_OK_PTR(
        strstr(log_buf, c"R0 !read_ok".as_ptr()) as *const c_void,
        c"bad_log_0".as_ptr(),
    );
    ASSERT_LT(fd, 0, c"bad_fd".as_ptr());
    if fd >= 0 {
        close(fd);
    }

    free(log_buf as *mut c_void);
}

unsafe fn bpf_btf_load_log_buf() {
    let mut opts = bpf_btf_load_opts {
        log_buf: core::ptr::null_mut(),
        log_size: 0,
        log_level: 0,
    };
    let log_buf_sz: size_t = 1024 * 1024;
    let mut raw_btf_data: *const c_void;
    let mut raw_btf_size: __u32 = 0;
    let btf: *mut btf;
    let mut log_buf: *mut c_char = core::ptr::null_mut();
    let mut fd: c_int = -1;

    btf = btf__new_empty();
    if ASSERT_OK_PTR(btf as *const c_void, c"empty_btf".as_ptr()) == core::ptr::null_mut() {
        return;
    }

    ASSERT_GT(btf__add_int(btf, c"int".as_ptr(), 4, 0), 0, c"int_type".as_ptr());

    raw_btf_data = btf__raw_data(btf, &mut raw_btf_size);
    if ASSERT_OK_PTR(raw_btf_data, c"raw_btf_data_good".as_ptr()) == core::ptr::null_mut() {
        bpf_btf_cleanup(log_buf, btf);
        return;
    }

    log_buf = malloc(log_buf_sz) as *mut c_char;
    if ASSERT_OK_PTR(log_buf as *const c_void, c"log_buf_alloc".as_ptr()) == core::ptr::null_mut()
    {
        bpf_btf_cleanup(log_buf, btf);
        return;
    }
    opts.log_buf = log_buf;
    opts.log_size = log_buf_sz;

    /* with log_level == 0 log_buf should stay empty for good BTF */
    *log_buf = 0;
    opts.log_level = 0;
    fd = bpf_btf_load(raw_btf_data, raw_btf_size, &opts);
    ASSERT_STREQ(log_buf, c"".as_ptr(), c"good_log_0".as_ptr());
    ASSERT_GE(fd, 0, c"good_fd1".as_ptr());
    if fd >= 0 {
        close(fd);
    }
    fd = -1;

    /* log_level == 2 should always fill log_buf, even for good BTF */
    *log_buf = 0;
    opts.log_level = 2;
    fd = bpf_btf_load(raw_btf_data, raw_btf_size, &opts);
    printf(c"LOG_BUF: %s\n".as_ptr(), log_buf);
    ASSERT_OK_PTR(
        strstr(log_buf, c"magic: 0xeb9f".as_ptr()) as *const c_void,
        c"good_log_2".as_ptr(),
    );
    ASSERT_GE(fd, 0, c"good_fd2".as_ptr());
    if fd >= 0 {
        close(fd);
    }
    fd = -1;

    /* make BTF bad, add pointer pointing to non-existing type */
    ASSERT_GT(btf__add_ptr(btf, 100), 0, c"bad_ptr_type".as_ptr());

    raw_btf_data = btf__raw_data(btf, &mut raw_btf_size);
    if ASSERT_OK_PTR(raw_btf_data, c"raw_btf_data_bad".as_ptr()) == core::ptr::null_mut() {
        bpf_btf_cleanup(log_buf, btf);
        return;
    }

    /* log_level == 0 should fill log_buf for bad BTF */
    *log_buf = 0;
    opts.log_level = 0;
    fd = bpf_btf_load(raw_btf_data, raw_btf_size, &opts);
    printf(c"LOG_BUF: %s\n".as_ptr(), log_buf);
    ASSERT_OK_PTR(
        strstr(
            log_buf,
            c"[2] PTR (anon) type_id=100 Invalid type_id".as_ptr(),
        ) as *const c_void,
        c"bad_log_0".as_ptr(),
    );
    ASSERT_LT(fd, 0, c"bad_fd".as_ptr());
    if fd >= 0 {
        close(fd);
    }
    fd = -1;

    bpf_btf_cleanup(log_buf, btf);
}

unsafe fn bpf_btf_cleanup(log_buf: *mut c_char, btf: *mut btf) {
    free(log_buf as *mut c_void);
    btf__free(btf);
}

#[no_mangle]
pub unsafe extern "C" fn test_log_buf() {
    if test__start_subtest(c"obj_load_log_buf".as_ptr()) {
        obj_load_log_buf();
    }
    if test__start_subtest(c"bpf_prog_load_log_buf".as_ptr()) {
        bpf_prog_load_log_buf();
    }
    if test__start_subtest(c"bpf_btf_load_log_buf".as_ptr()) {
        bpf_btf_load_log_buf();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
