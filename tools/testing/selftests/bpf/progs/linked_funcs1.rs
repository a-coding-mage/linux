// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies from the original C source:
 * "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
 * <bpf/bpf_core_read.h>, and "bpf_misc.h".
 */

pub type __u32 = u32;
pub type __u64 = u64;
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_core_type_size_task_struct() -> core::ffi::c_int;
    fn __sink(x: core::ffi::c_int);
}

/* weak and shared between two files */
#[no_mangle]
pub static mut my_tid: __u32 = 0;
#[no_mangle]
pub static mut syscall_id: core::ffi::c_long = 0;

#[no_mangle]
pub static mut output_val1: core::ffi::c_int = 0;
#[no_mangle]
pub static mut output_ctx1: core::ffi::c_int = 0;
#[no_mangle]
pub static mut output_weak1: core::ffi::c_int = 0;

/* same "subprog" name in all files, but it's ok because they all are static */
#[inline(never)]
unsafe fn subprog(x: core::ffi::c_int) -> core::ffi::c_int {
    /* but different formula */
    x.wrapping_mul(1)
}

/* Global functions can't be void */
#[no_mangle]
pub unsafe extern "C" fn set_output_val1(x: core::ffi::c_int) -> core::ffi::c_int {
    output_val1 = x.wrapping_add(subprog(x));
    x
}

/* This function can't be verified as global, as it assumes raw_tp/sys_enter
 * context and accesses syscall id (second argument). So we mark it as
 * __hidden, so that libbpf will mark it as static in the final object file,
 * right before verifying it in the kernel.
 *
 * But we don't mark it as __hidden here, rather at extern site. __hidden is
 * "contaminating" visibility, so it will get propagated from either extern or
 * actual definition (including from the losing __weak definition).
 */
#[no_mangle]
pub unsafe extern "C" fn set_output_ctx1(ctx: *mut __u64) {
    output_ctx1 = *ctx.add(1) as core::ffi::c_int; /* long id, same as in BPF_PROG below */
}

/* this weak instance should win because it's the first one */
#[no_mangle]
pub unsafe extern "C" fn set_output_weak(x: core::ffi::c_int) -> core::ffi::c_int {
    static mut WHATEVER: core::ffi::c_int = 0;

    /* make sure we use CO-RE relocations in a weak function, this used to
     * cause problems for BPF static linker
     */
    WHATEVER = bpf_core_type_size_task_struct();
    __sink(WHATEVER);

    output_weak1 = x;
    x
}

extern "C" {
    fn set_output_val2(x: core::ffi::c_int) -> core::ffi::c_int;
}

/* here we'll force set_output_ctx2() to be __hidden in the final obj file */
extern "C" {
    fn set_output_ctx2(ctx: *mut __u64);
}

extern "C" {
    fn bpf_cast_to_kern_ctx(obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}

/* SEC("?raw_tp/sys_enter") */
#[no_mangle]
pub unsafe extern "C" fn handler1(regs: *mut pt_regs, id: core::ffi::c_long) -> core::ffi::c_int {
    static mut WHATEVER: core::ffi::c_int = 0;

    let ctx = regs as *mut __u64;

    if my_tid != bpf_get_current_pid_tgid() as u32 || id != syscall_id {
        return 0;
    }

    /* make sure we have CO-RE relocations in main program */
    WHATEVER = bpf_core_type_size_task_struct();
    __sink(WHATEVER);

    set_output_val2(1000);
    set_output_ctx2(ctx); /* ctx definition is hidden in BPF_PROG macro */

    /* keep input value the same across both files to avoid dependency on
     * handler call order; differentiate by output_weak1 vs output_weak2.
     */
    set_output_weak(42);

    0
}

/* Generate BTF FUNC record and test linking with duplicate extern functions */
#[no_mangle]
pub unsafe extern "C" fn kfunc_gen1() {
    bpf_cast_to_kern_ctx(core::ptr::null_mut());
}

/* char LICENSE[] SEC("license") = "GPL"; */
#[no_mangle]
pub static LICENSE: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
