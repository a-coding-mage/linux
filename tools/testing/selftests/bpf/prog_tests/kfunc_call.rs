// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Translated from C. External declarations correspond to included headers:
 * test_progs.h, network_helpers.h, kfunc_call_*.skel.h,
 * kfunc_call_*.lskel.h, and cap_helpers.h.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type SizeT = usize;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

const EINVAL: c_int = 22;
const CAP_SYS_BOOT: c_int = 22;
const BPF_TCP_CLOSE: c_int = 7;

static mut log_buf_sz: SizeT = 1048576; /* 1 MB */
static mut obj_log_buf: [c_char; 1048576] = [0; 1048576];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum kfunc_test_type {
    tc_test = 0,
    syscall_test,
    syscall_null_ctx_test,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct kfunc_test_params {
    prog_name: *const c_char,
    lskel_prog_desc_offset: c_ulong,
    retval: c_int,
    test_type: kfunc_test_type,
    expected_err_msg: *const c_char,
}

#[repr(C)]
struct syscall_test_args {
    data: [__u8; 16],
    size: SizeT,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: SizeT,
    data_in: *const c_void,
    data_size_in: __u32,
    ctx_in: *const c_void,
    ctx_size_in: __u32,
    retval: __u32,
    repeat: c_int,
}

#[repr(C)]
struct bpf_object_open_opts {
    sz: SizeT,
    kernel_log_buf: *mut c_char,
    kernel_log_size: SizeT,
    kernel_log_level: __u32,
}

#[repr(C)]
struct bpf_prog_desc {
    prog_fd: c_int,
}

#[repr(C)]
struct kfunc_call_test_lskel_progs {
    kfunc_call_test_spin_lock_safe: bpf_prog_desc,
    kfunc_call_test1: bpf_prog_desc,
    kfunc_call_test2: bpf_prog_desc,
    kfunc_call_test4: bpf_prog_desc,
    kfunc_call_test5: bpf_prog_desc,
    kfunc_call_test5_asm: bpf_prog_desc,
    kfunc_call_test_ref_btf_id: bpf_prog_desc,
    kfunc_call_test_get_mem: bpf_prog_desc,
    kfunc_syscall_test: bpf_prog_desc,
    kfunc_syscall_test_null: bpf_prog_desc,
    kfunc_call_test_static_unused_arg: bpf_prog_desc,
    kfunc_call_ctx: bpf_prog_desc,
}

#[repr(C)]
struct kfunc_call_test_lskel {
    progs: kfunc_call_test_lskel_progs,
}

#[repr(C)]
struct kfunc_call_test_subprog_lskel_prog {
    prog_fd: c_int,
}

#[repr(C)]
struct kfunc_call_test_subprog_lskel_progs {
    kfunc_call_test1: kfunc_call_test_subprog_lskel_prog,
}

#[repr(C)]
struct kfunc_call_test_subprog_lskel_data {
    active_res: c_int,
    sk_state_res: c_int,
}

#[repr(C)]
struct kfunc_call_test_subprog_lskel {
    progs: kfunc_call_test_subprog_lskel_progs,
    data: *mut kfunc_call_test_subprog_lskel_data,
}

#[repr(C)]
struct kfunc_call_test {
    obj: *mut bpf_object,
}

#[repr(C)]
struct kfunc_call_fail {
    obj: *mut bpf_object,
}

#[repr(C)]
struct kfunc_call_test_subprog_progs {
    kfunc_call_test1: *mut bpf_program,
}

#[repr(C)]
struct kfunc_call_test_subprog_data {
    active_res: c_int,
    sk_state_res: c_int,
}

#[repr(C)]
struct kfunc_call_test_subprog {
    progs: kfunc_call_test_subprog_progs,
    data: *mut kfunc_call_test_subprog_data,
}

#[repr(C)]
struct kfunc_call_destructive {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    static pkt_v4: [__u8; 0];

    fn kfunc_call_test__open_and_load() -> *mut kfunc_call_test;
    fn kfunc_call_test__destroy(skel: *mut kfunc_call_test);
    fn kfunc_call_test_lskel__open_and_load() -> *mut kfunc_call_test_lskel;
    fn kfunc_call_test_lskel__destroy(skel: *mut kfunc_call_test_lskel);
    fn kfunc_call_fail__open_opts(opts: *const bpf_object_open_opts) -> *mut kfunc_call_fail;
    fn kfunc_call_fail__load(skel: *mut kfunc_call_fail) -> c_int;
    fn kfunc_call_fail__destroy(skel: *mut kfunc_call_fail);
    fn kfunc_call_test_subprog__open_and_load() -> *mut kfunc_call_test_subprog;
    fn kfunc_call_test_subprog__destroy(skel: *mut kfunc_call_test_subprog);
    fn kfunc_call_test_subprog_lskel__open_and_load() -> *mut kfunc_call_test_subprog_lskel;
    fn kfunc_call_test_subprog_lskel__destroy(skel: *mut kfunc_call_test_subprog_lskel);
    fn kfunc_call_destructive__open() -> *mut kfunc_call_destructive;
    fn kfunc_call_destructive__load(skel: *mut kfunc_call_destructive) -> c_int;
    fn kfunc_call_destructive__destroy(skel: *mut kfunc_call_destructive);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn cap_disable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
    fn cap_enable_effective(caps: __u64, old_caps: *mut __u64) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut c_void;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! bpf_test_success {
    ($name:ident, $retval:expr, $type:expr) => {
        kfunc_test_params {
            prog_name: c_str!(stringify!($name)),
            lskel_prog_desc_offset: offset_of!(kfunc_call_test_lskel, progs.$name) as c_ulong,
            retval: $retval,
            test_type: $type,
            expected_err_msg: ptr::null(),
        }
    };
}

macro_rules! bpf_test_fail {
    ($name:ident, $retval:expr, $type:expr, $error_msg:literal) => {
        kfunc_test_params {
            prog_name: c_str!(stringify!($name)),
            lskel_prog_desc_offset: 0, /* unused when test is failing */
            retval: $retval,
            test_type: $type,
            expected_err_msg: c_str!($error_msg),
        }
    };
}

macro_rules! tc_test {
    ($name:ident, $retval:expr) => {
        bpf_test_success!($name, $retval, kfunc_test_type::tc_test)
    };
}

macro_rules! syscall_test {
    ($name:ident, $retval:expr) => {
        bpf_test_success!($name, $retval, kfunc_test_type::syscall_test)
    };
}

macro_rules! syscall_null_ctx_test {
    ($name:ident, $retval:expr) => {
        bpf_test_success!($name, $retval, kfunc_test_type::syscall_null_ctx_test)
    };
}

macro_rules! tc_fail {
    ($name:ident, $retval:expr, $error_msg:literal) => {
        bpf_test_fail!($name, $retval, kfunc_test_type::tc_test, $error_msg)
    };
}

macro_rules! syscall_null_ctx_fail {
    ($name:ident, $retval:expr, $error_msg:literal) => {
        bpf_test_fail!($name, $retval, kfunc_test_type::syscall_null_ctx_test, $error_msg)
    };
}

static mut kfunc_tests: [kfunc_test_params; 24] = [
    /* failure cases:
     * if retval is 0 -> the program will fail to load and the error message is an error
     * if retval is not 0 -> the program can be loaded but running it will gives the
     *                       provided return value. The error message is thus the one
     *                       from a successful load
     */
    syscall_null_ctx_fail!(kfunc_syscall_test_fail, -EINVAL, "processed 4 insns"),
    syscall_null_ctx_fail!(kfunc_syscall_test_null_fail, -EINVAL, "processed 4 insns"),
    tc_fail!(kfunc_call_test_get_mem_fail_rdonly, 0, "R0 cannot write into rdonly_mem"),
    tc_fail!(kfunc_call_test_get_mem_fail_use_after_free, 0, "invalid mem access 'scalar'"),
    tc_fail!(kfunc_call_test_get_mem_fail_oob, 0, "min value is outside of the allowed memory range"),
    tc_fail!(kfunc_call_test_get_mem_fail_zero_size, 0, "min value is outside of the allowed memory range"),
    tc_fail!(kfunc_call_test_get_mem_fail_oversized, 0, "allocation size exceeds u32 max"),
    tc_fail!(kfunc_call_test_get_mem_fail_not_const, 0, "is not a const"),
    tc_fail!(kfunc_call_test_mem_acquire_fail, 0, "acquire kernel function does not return PTR_TO_BTF_ID"),
    tc_fail!(kfunc_call_test_pointer_arg_type_mismatch, 0, "R1 expected pointer to ctx, but got scalar"),
    tc_fail!(kfunc_call_test_spin_lock_unsafe, 0, "function calls are not allowed while holding a lock"),

    /* success cases */
    tc_test!(kfunc_call_test_spin_lock_safe, 0),
    tc_test!(kfunc_call_test1, 12),
    tc_test!(kfunc_call_test2, 3),
    tc_test!(kfunc_call_test4, -1234),
    tc_test!(kfunc_call_test5, 0),
    tc_test!(kfunc_call_test5_asm, 0),
    tc_test!(kfunc_call_test_ref_btf_id, 0),
    tc_test!(kfunc_call_test_get_mem, 42),
    syscall_test!(kfunc_syscall_test, 0),
    syscall_null_ctx_test!(kfunc_syscall_test_null, 0),
    tc_test!(kfunc_call_test_static_unused_arg, 0),
    tc_test!(kfunc_call_ctx, 0),
];

unsafe fn verify_success(param: *mut kfunc_test_params) {
    let mut lskel: *mut kfunc_call_test_lskel = ptr::null_mut();
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: ptr::null(),
        data_size_in: 0,
        ctx_in: ptr::null(),
        ctx_size_in: 0,
        retval: 0,
        repeat: 0,
    };
    let mut lskel_prog: *mut bpf_prog_desc;
    let mut skel: *mut kfunc_call_test;
    let mut prog: *mut bpf_program;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut args = syscall_test_args {
        data: [0; 16],
        size: 10,
    };

    match (*param).test_type {
        kfunc_test_type::syscall_test => {
            topts.ctx_in = &mut args as *mut _ as *const c_void;
            topts.ctx_size_in = size_of::<syscall_test_args>() as __u32;
            /* fallthrough */
        }
        kfunc_test_type::syscall_null_ctx_test => {}
        kfunc_test_type::tc_test => {
            topts.data_in = &pkt_v4 as *const _ as *const c_void;
            topts.data_size_in = size_of_val_pkt_v4() as __u32;
            topts.repeat = 1;
        }
    }

    /* first test with normal libbpf */
    skel = kfunc_call_test__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c_str!("skel")) {
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, (*param).prog_name);
    if !ASSERT_OK_PTR(prog as *const c_void, c_str!("bpf_object__find_program_by_name")) {
        goto_cleanup_success(skel, lskel);
        return;
    }

    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, (*param).prog_name) {
        goto_cleanup_success(skel, lskel);
        return;
    }

    if !ASSERT_EQ(topts.retval as c_int, (*param).retval, c_str!("retval")) {
        goto_cleanup_success(skel, lskel);
        return;
    }

    /* second test with light skeletons */
    lskel = kfunc_call_test_lskel__open_and_load();
    if !ASSERT_OK_PTR(lskel as *const c_void, c_str!("lskel")) {
        goto_cleanup_success(skel, lskel);
        return;
    }

    lskel_prog = (lskel as *mut c_char).add((*param).lskel_prog_desc_offset as usize)
        as *mut bpf_prog_desc;

    prog_fd = (*lskel_prog).prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    if !ASSERT_OK(err, (*param).prog_name) {
        goto_cleanup_success(skel, lskel);
        return;
    }

    ASSERT_EQ(topts.retval as c_int, (*param).retval, c_str!("retval"));

    goto_cleanup_success(skel, lskel);
}

unsafe fn goto_cleanup_success(
    skel: *mut kfunc_call_test,
    lskel: *mut kfunc_call_test_lskel,
) {
    kfunc_call_test__destroy(skel);
    if !lskel.is_null() {
        kfunc_call_test_lskel__destroy(lskel);
    }
}

unsafe fn verify_fail(param: *mut kfunc_test_params) {
    let mut opts = bpf_object_open_opts {
        sz: size_of::<bpf_object_open_opts>(),
        kernel_log_buf: ptr::addr_of_mut!(obj_log_buf) as *mut c_char,
        kernel_log_size: log_buf_sz,
        kernel_log_level: 1,
    };
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: ptr::null(),
        data_size_in: 0,
        ctx_in: ptr::null(),
        ctx_size_in: 0,
        retval: 0,
        repeat: 0,
    };
    let mut prog: *mut bpf_program;
    let mut skel: *mut kfunc_call_fail;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut args = syscall_test_args {
        data: [0; 16],
        size: 10,
    };

    match (*param).test_type {
        kfunc_test_type::syscall_test => {
            topts.ctx_in = &mut args as *mut _ as *const c_void;
            topts.ctx_size_in = size_of::<syscall_test_args>() as __u32;
            /* fallthrough */
        }
        kfunc_test_type::syscall_null_ctx_test => {}
        kfunc_test_type::tc_test => {
            topts.data_in = &pkt_v4 as *const _ as *const c_void;
            topts.data_size_in = size_of_val_pkt_v4() as __u32;
            topts.repeat = 1;
        }
    }

    skel = kfunc_call_fail__open_opts(&mut opts);
    if !ASSERT_OK_PTR(skel as *const c_void, c_str!("kfunc_call_fail__open_opts")) {
        goto_cleanup_fail(skel);
        return;
    }

    prog = bpf_object__find_program_by_name((*skel).obj, (*param).prog_name);
    if !ASSERT_OK_PTR(prog as *const c_void, c_str!("bpf_object__find_program_by_name")) {
        goto_cleanup_fail(skel);
        return;
    }

    bpf_program__set_autoload(prog, true);

    err = kfunc_call_fail__load(skel);
    if (*param).retval == 0 {
        /* the verifier is supposed to complain and refuses to load */
        if !ASSERT_ERR(err, c_str!("unexpected load success")) {
            goto_out_err_with_param(param);
            goto_cleanup_fail(skel);
            return;
        }
    } else {
        /* the program is loaded but must dynamically fail */
        if !ASSERT_OK(err, c_str!("unexpected load error")) {
            goto_out_err_with_param(param);
            goto_cleanup_fail(skel);
            return;
        }

        prog_fd = bpf_program__fd(prog);
        err = bpf_prog_test_run_opts(prog_fd, &mut topts);
        if !ASSERT_EQ(err, (*param).retval, (*param).prog_name) {
            goto_out_err_with_param(param);
            goto_cleanup_fail(skel);
            return;
        }
    }

    goto_out_err_with_param(param);
    goto_cleanup_fail(skel);
}

unsafe fn goto_out_err_with_param(param: *mut kfunc_test_params) {
    if !ASSERT_OK_PTR(
        strstr(ptr::addr_of!(obj_log_buf) as *const c_char, (*param).expected_err_msg)
            as *const c_void,
        c_str!("expected_err_msg"),
    ) {
        fprintf(
            stderr,
            c_str!("Expected err_msg: %s\n"),
            (*param).expected_err_msg,
        );
        fprintf(
            stderr,
            c_str!("Verifier output: %s\n"),
            ptr::addr_of!(obj_log_buf) as *const c_char,
        );
    }
}

unsafe fn goto_cleanup_fail(skel: *mut kfunc_call_fail) {
    kfunc_call_fail__destroy(skel);
}

unsafe fn size_of_val_pkt_v4() -> usize {
    size_of_val_raw(&pkt_v4)
}

unsafe fn size_of_val_raw<T: ?Sized>(_: *const T) -> usize {
    size_of::<[__u8; 0]>()
}

unsafe fn test_main() {
    let mut i: c_int;

    i = 0;
    while (i as usize) < kfunc_tests.len() {
        if !test__start_subtest(kfunc_tests[i as usize].prog_name) {
            i += 1;
            continue;
        }

        if kfunc_tests[i as usize].expected_err_msg.is_null() {
            verify_success(&mut kfunc_tests[i as usize]);
        } else {
            verify_fail(&mut kfunc_tests[i as usize]);
        }
        i += 1;
    }
}

unsafe fn test_subprog() {
    let mut skel: *mut kfunc_call_test_subprog;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: &pkt_v4 as *const _ as *const c_void,
        data_size_in: size_of_val_pkt_v4() as __u32,
        ctx_in: ptr::null(),
        ctx_size_in: 0,
        retval: 0,
        repeat: 1,
    };

    skel = kfunc_call_test_subprog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c_str!("skel")) {
        return;
    }

    prog_fd = bpf_program__fd((*skel).progs.kfunc_call_test1);
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c_str!("bpf_prog_test_run(test1)"));
    ASSERT_EQ(topts.retval as c_int, 10, c_str!("test1-retval"));
    ASSERT_NEQ((*(*skel).data).active_res, -1, c_str!("active_res"));
    ASSERT_EQ(
        (*(*skel).data).sk_state_res,
        BPF_TCP_CLOSE,
        c_str!("sk_state_res"),
    );

    kfunc_call_test_subprog__destroy(skel);
}

unsafe fn test_subprog_lskel() {
    let mut skel: *mut kfunc_call_test_subprog_lskel;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut topts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: &pkt_v4 as *const _ as *const c_void,
        data_size_in: size_of_val_pkt_v4() as __u32,
        ctx_in: ptr::null(),
        ctx_size_in: 0,
        retval: 0,
        repeat: 1,
    };

    skel = kfunc_call_test_subprog_lskel__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c_str!("skel")) {
        return;
    }

    prog_fd = (*skel).progs.kfunc_call_test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c_str!("bpf_prog_test_run(test1)"));
    ASSERT_EQ(topts.retval as c_int, 10, c_str!("test1-retval"));
    ASSERT_NEQ((*(*skel).data).active_res, -1, c_str!("active_res"));
    ASSERT_EQ(
        (*(*skel).data).sk_state_res,
        BPF_TCP_CLOSE,
        c_str!("sk_state_res"),
    );

    kfunc_call_test_subprog_lskel__destroy(skel);
}

unsafe fn test_destructive_open_and_load() -> c_int {
    let mut skel: *mut kfunc_call_destructive;
    let mut err: c_int;

    skel = kfunc_call_destructive__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c_str!("prog_open")) {
        return -1;
    }

    err = kfunc_call_destructive__load(skel);

    kfunc_call_destructive__destroy(skel);

    err
}

unsafe fn test_destructive() {
    let mut save_caps: __u64 = 0;

    ASSERT_OK(test_destructive_open_and_load(), c_str!("successful_load"));

    if !ASSERT_OK(
        cap_disable_effective(1u64 << CAP_SYS_BOOT, &mut save_caps),
        c_str!("drop_caps"),
    ) {
        return;
    }

    ASSERT_EQ(
        test_destructive_open_and_load(),
        -13,
        c_str!("no_caps_failure"),
    );

    cap_enable_effective(save_caps, ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn test_kfunc_call() {
    test_main();

    if test__start_subtest(c_str!("subprog")) {
        test_subprog();
    }

    if test__start_subtest(c_str!("subprog_lskel")) {
        test_subprog_lskel();
    }

    if test__start_subtest(c_str!("destructive")) {
        test_destructive();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
