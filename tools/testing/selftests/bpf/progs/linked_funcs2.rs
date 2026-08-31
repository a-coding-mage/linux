// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Dependencies from the original C file:
 * "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
 * <bpf/bpf_core_read.h>, and "bpf_misc.h".
 */

type __u64 = u64;
type s32 = i32;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_core_type_size_task_struct() -> i32;
    fn __sink(value: i32);

    fn set_output_val1(x: i32) -> i32;

    /* here we'll force set_output_ctx1() to be __hidden in the final obj file */
    fn set_output_ctx1(ctx: *mut __u64);

    fn bpf_cast_to_kern_ctx(obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}

/* weak and shared between both files */
#[no_mangle]
pub static mut my_tid: i32 = 0;
#[no_mangle]
pub static mut syscall_id: i64 = 0;

#[no_mangle]
pub static mut output_val2: i32 = 0;
#[no_mangle]
pub static mut output_ctx2: i32 = 0;
#[no_mangle]
pub static mut output_weak2: i32 = 0; /* should stay zero */

/* same "subprog" name in all files, but it's ok because they all are static */
#[inline(never)]
fn subprog(x: i32) -> i32 {
    /* but different formula */
    x * 2
}

/* Global functions can't be void */
#[no_mangle]
pub unsafe extern "C" fn set_output_val2(x: i32) -> i32 {
    output_val2 = 2 * x + 2 * subprog(x);
    2 * x
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
pub unsafe extern "C" fn set_output_ctx2(ctx: *mut __u64) {
    output_ctx2 = *ctx.add(1) as i32; /* long id, same as in BPF_PROG below */
}

/* this weak instance should lose, because it will be processed second */
#[no_mangle]
pub unsafe extern "C" fn set_output_weak(x: i32) -> i32 {
    static mut whatever: i32 = 0;

    /* make sure we use CO-RE relocations in a weak function, this used to
     * cause problems for BPF static linker
     */
    whatever = 2 * bpf_core_type_size_task_struct();
    __sink(whatever);

    output_weak2 = x;
    2 * x
}

/* Original section: SEC("?raw_tp/sys_enter")
 * Original declaration: int BPF_PROG(handler2, struct pt_regs *regs, long id)
 */
#[no_mangle]
#[link_section = "?raw_tp/sys_enter"]
pub unsafe extern "C" fn handler2(ctx: *mut __u64) -> i32 {
    static mut whatever: i32 = 0;
    let regs = *ctx.add(0) as *mut pt_regs;
    let id = *ctx.add(1) as i64;
    let _ = regs;

    if my_tid != bpf_get_current_pid_tgid() as s32 || id != syscall_id {
        return 0;
    }

    /* make sure we have CO-RE relocations in main program */
    whatever = bpf_core_type_size_task_struct();
    __sink(whatever);

    set_output_val1(2000);
    set_output_ctx1(ctx); /* ctx definition is hidden in BPF_PROG macro */

    /* keep input value the same across both files to avoid dependency on
     * handler call order; differentiate by output_weak1 vs output_weak2.
     */
    set_output_weak(42);

    0
}

/* Generate BTF FUNC record and test linking with duplicate extern functions */
#[no_mangle]
pub unsafe extern "C" fn kfunc_gen2() {
    bpf_cast_to_kern_ctx(core::ptr::null_mut());
}

#[no_mangle]
#[link_section = "license"]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";
