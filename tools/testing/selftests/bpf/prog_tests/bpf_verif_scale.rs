// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
// Translated from C source using declarations supplied by test_progs.h/libbpf.

use core::ffi::{c_char, c_int, c_uint, c_void};

type va_list = *mut c_void;
type libbpf_print_fn_t = Option<
    unsafe extern "C" fn(
        level: libbpf_print_level,
        format: *const c_char,
        args: va_list,
    ) -> c_int,
>;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_print_level {
    LIBBPF_WARN = 0,
    LIBBPF_INFO = 1,
    LIBBPF_DEBUG = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_prog_type {
    __BPF_PROG_TYPE_EXTERNAL = 0,
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
pub struct test_env {
    pub verifier_stats: bool,
}

unsafe extern "C" {
    static mut env: test_env;
    static mut extra_prog_load_log_flags: c_int;

    // Enum constants supplied by external BPF/libbpf headers in the C source.
    static BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type;
    static BPF_PROG_TYPE_RAW_TRACEPOINT: bpf_prog_type;
    static BPF_PROG_TYPE_KPROBE: bpf_prog_type;
    static BPF_PROG_TYPE_CGROUP_SYSCTL: bpf_prog_type;
    static BPF_PROG_TYPE_XDP: bpf_prog_type;
    static BPF_PROG_TYPE_LWT_SEG6LOCAL: bpf_prog_type;
    static BPF_PROG_TYPE_CGROUP_SKB: bpf_prog_type;

    static ENOENT: c_int;

    fn vprintf(format: *const c_char, args: va_list) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn bpf_object__open_file(file: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const bpf_object) -> c_int;
    fn bpf_object__next_program(
        obj: *const bpf_object,
        prog: *const bpf_program,
    ) -> *mut bpf_program;
    fn bpf_program__set_type(prog: *mut bpf_program, prog_type: bpf_prog_type);
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: c_uint);
    fn bpf_program__set_log_level(prog: *mut bpf_program, log_level: c_uint);
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn testing_prog_flags() -> c_uint;

    fn test__force_log();
    fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;

    fn ASSERT_ERR(err: c_int, name: *const c_char);
    fn ASSERT_OK(err: c_int, name: *const c_char);
}

unsafe extern "C" fn libbpf_debug_print(
    level: libbpf_print_level,
    format: *const c_char,
    args: va_list,
) -> c_int {
    if level != libbpf_print_level::LIBBPF_DEBUG {
        unsafe {
            vprintf(format, args);
        }
        return 0;
    }

    if unsafe { strstr(format, c"verifier log".as_ptr()) }.is_null() {
        return 0;
    }
    unsafe {
        vprintf(c"%s".as_ptr(), args);
    }
    0
}

unsafe fn check_load(file: *const c_char, type_: bpf_prog_type) -> c_int {
    let mut obj: *mut bpf_object = core::ptr::null_mut();
    let prog: *mut bpf_program;
    let mut err: c_int;

    obj = unsafe { bpf_object__open_file(file, core::ptr::null()) };
    err = unsafe { libbpf_get_error(obj) };
    if err != 0 {
        return err;
    }

    prog = unsafe { bpf_object__next_program(obj, core::ptr::null()) };
    if prog.is_null() {
        err = -unsafe { ENOENT };
    } else {
        unsafe {
            bpf_program__set_type(prog, type_);
            bpf_program__set_flags(prog, testing_prog_flags());
            bpf_program__set_log_level(prog, (4 | extra_prog_load_log_flags) as c_uint);

            err = bpf_object__load(obj);
        }
    }

    unsafe {
        bpf_object__close(obj);
    }
    err
}

unsafe fn scale_test(file: *const c_char, attach_type: bpf_prog_type, should_fail: bool) {
    let mut old_print_fn: libbpf_print_fn_t = None;
    let err: c_int;

    if unsafe { env.verifier_stats } {
        unsafe {
            test__force_log();
            old_print_fn = libbpf_set_print(Some(libbpf_debug_print));
        }
    }

    err = unsafe { check_load(file, attach_type) };
    if should_fail {
        unsafe {
            ASSERT_ERR(err, c"expect_error".as_ptr());
        }
    } else {
        unsafe {
            ASSERT_OK(err, c"expect_success".as_ptr());
        }
    }

    if unsafe { env.verifier_stats } {
        unsafe {
            libbpf_set_print(old_print_fn);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale1() {
    unsafe {
        scale_test(c"test_verif_scale1.bpf.o".as_ptr(), BPF_PROG_TYPE_SCHED_CLS, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale2() {
    unsafe {
        scale_test(c"test_verif_scale2.bpf.o".as_ptr(), BPF_PROG_TYPE_SCHED_CLS, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale3() {
    unsafe {
        scale_test(c"test_verif_scale3.bpf.o".as_ptr(), BPF_PROG_TYPE_SCHED_CLS, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf_global() {
    unsafe {
        scale_test(c"pyperf_global.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf_subprogs() {
    unsafe {
        scale_test(c"pyperf_subprogs.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf50() {
    /* full unroll by llvm */
    unsafe {
        scale_test(c"pyperf50.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf100() {
    /* full unroll by llvm */
    unsafe {
        scale_test(c"pyperf100.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf180() {
    /* full unroll by llvm */
    unsafe {
        scale_test(c"pyperf180.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf600() {
    /* partial unroll. llvm will unroll loop ~150 times.
     * C loop count -> 600.
     * Asm loop count -> 4.
     * 16k insns in loop body.
     * Total of 5 such loops. Total program size ~82k insns.
     */
    unsafe {
        scale_test(c"pyperf600.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf600_bpf_loop() {
    /* use the bpf_loop helper*/
    unsafe {
        scale_test(
            c"pyperf600_bpf_loop.bpf.o".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            false,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf600_nounroll() {
    /* no unroll at all.
     * C loop count -> 600.
     * ASM loop count -> 600.
     * ~110 insns in loop body.
     * Total of 5 such loops. Total program size ~1500 insns.
     */
    unsafe {
        scale_test(
            c"pyperf600_nounroll.bpf.o".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            false,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_pyperf600_iter() {
    /* open-coded BPF iterator version */
    unsafe {
        scale_test(c"pyperf600_iter.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_loop1() {
    unsafe {
        scale_test(c"loop1.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_loop2() {
    unsafe {
        scale_test(c"loop2.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_loop3_fail() {
    unsafe {
        scale_test(c"loop3.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, true /* fails */);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_loop4() {
    unsafe {
        scale_test(c"loop4.bpf.o".as_ptr(), BPF_PROG_TYPE_SCHED_CLS, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_loop5() {
    unsafe {
        scale_test(c"loop5.bpf.o".as_ptr(), BPF_PROG_TYPE_SCHED_CLS, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_loop6() {
    unsafe {
        scale_test(c"loop6.bpf.o".as_ptr(), BPF_PROG_TYPE_KPROBE, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_strobemeta() {
    /* partial unroll. 19k insn in a loop.
     * Total program size 20.8k insn.
     * ~350k processed_insns
     */
    unsafe {
        scale_test(c"strobemeta.bpf.o".as_ptr(), BPF_PROG_TYPE_RAW_TRACEPOINT, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_strobemeta_bpf_loop() {
    /* use the bpf_loop helper*/
    unsafe {
        scale_test(
            c"strobemeta_bpf_loop.bpf.o".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            false,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_strobemeta_nounroll1() {
    /* no unroll, tiny loops */
    unsafe {
        scale_test(
            c"strobemeta_nounroll1.bpf.o".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            false,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_strobemeta_nounroll2() {
    /* no unroll, tiny loops */
    unsafe {
        scale_test(
            c"strobemeta_nounroll2.bpf.o".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            false,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_strobemeta_subprogs() {
    /* non-inlined subprogs */
    unsafe {
        scale_test(
            c"strobemeta_subprogs.bpf.o".as_ptr(),
            BPF_PROG_TYPE_RAW_TRACEPOINT,
            false,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_sysctl_loop1() {
    unsafe {
        scale_test(c"test_sysctl_loop1.bpf.o".as_ptr(), BPF_PROG_TYPE_CGROUP_SYSCTL, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_sysctl_loop2() {
    unsafe {
        scale_test(c"test_sysctl_loop2.bpf.o".as_ptr(), BPF_PROG_TYPE_CGROUP_SYSCTL, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_xdp_loop() {
    unsafe {
        scale_test(c"test_xdp_loop.bpf.o".as_ptr(), BPF_PROG_TYPE_XDP, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_scale_seg6_loop() {
    unsafe {
        scale_test(c"test_seg6_loop.bpf.o".as_ptr(), BPF_PROG_TYPE_LWT_SEG6LOCAL, false);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_verif_twfw() {
    unsafe {
        scale_test(c"twfw.bpf.o".as_ptr(), BPF_PROG_TYPE_CGROUP_SKB, false);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
