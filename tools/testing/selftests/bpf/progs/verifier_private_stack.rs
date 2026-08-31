// SPDX-License-Identifier: GPL-2.0

// Translated from C source. Original includes:
// <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h", "bpf_experimental.h"

use core::arch::asm;

/* From include/linux/filter.h */
const MAX_BPF_STACK: i32 = 512;

// Original C condition:
// #if defined(__TARGET_ARCH_x86) || defined(__TARGET_ARCH_arm64)
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
pub struct elem {
    pub t: bpf_timer,
    pub pad: [i8; 256],
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
pub struct array_map_def {
    // __uint(type, BPF_MAP_TYPE_ARRAY);
    // __uint(max_entries, 1);
    // __type(key, int);
    // __type(value, struct elem);
    pub _unused: [u8; 0],
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[link_section = ".maps"]
pub static mut array: array_map_def = array_map_def { _unused: [] };

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> i64;
    fn bpf_get_prandom_u32() -> i64;
    fn bpf_loop(nr_loops: u32, callback_fn: *const core::ffi::c_void, callback_ctx: *mut core::ffi::c_void, flags: u64) -> i64;
    fn bpf_throw(cookie: u64) -> !;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_timer_init(timer: *mut bpf_timer, map: *mut core::ffi::c_void, clockid: u64) -> i64;
    fn bpf_timer_set_callback(timer: *mut bpf_timer, callback_fn: unsafe extern "C" fn(*mut core::ffi::c_void, *mut i32, *mut bpf_timer) -> i32) -> i64;
    fn bpf_timer_start(timer: *mut bpf_timer, nsecs: u64, flags: u64) -> i64;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
pub struct bpf_timer {
    _private: [u8; 0],
}

// SEC("kprobe")
// __description("Private stack, single prog")
// __success
// __arch_x86_64
// __jited("	movabsq	$0x{{.*}}, %r9")
// __jited("	addq	%gs:{{.*}}, %r9")
// __jited("	movl	$0x2a, %edi")
// __jited("	movq	%rdi, -0x100(%r9)")
// __arch_arm64
// __jited("	stp	x25, x27, [sp, {{.*}}]!")
// __jited("	mov	x27, {{.*}}")
// __jited("	movk	x27, {{.*}}, lsl #16")
// __jited("	movk	x27, {{.*}}")
// __jited("	mrs	x10, TPIDR_EL{{[0-1]}}")
// __jited("	add	x27, x27, x10")
// __jited("	add	x25, x27, {{.*}}")
// __jited("	mov	x0, #0x2a")
// __jited("	str	x0, [x27]")
// __jited("...")
// __jited("	ldp	x25, x27, [sp], {{.*}}")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_single_prog() {
    unsafe {
        asm!(
            "r1 = 42",
            "*(u64 *)(r10 - 256) = r1",
            "r0 = 0",
            "exit",
            options(noreturn)
        );
    }
}

// SEC("raw_tp")
// __description("No private stack")
// __success
// __arch_x86_64
// __jited("	subq	$0x8, %rsp")
// __arch_arm64
// __jited("	mov	x25, sp")
// __jited("	sub	sp, sp, #0x10")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn no_private_stack_nested() {
    unsafe {
        asm!(
            "r1 = 42",
            "*(u64 *)(r10 - 8) = r1",
            "r0 = 0",
            "exit",
            options(noreturn)
        );
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn cumulative_stack_depth_subprog() {
    unsafe {
        asm!(
            "r1 = 41",
            "*(u64 *)(r10 - 32) = r1",
            "call {bpf_get_smp_processor_id}",
            "exit",
            bpf_get_smp_processor_id = sym bpf_get_smp_processor_id,
            options(noreturn)
        );
    }
}

// SEC("kprobe")
// __description("Private stack, subtree > MAX_BPF_STACK")
// __success
// __log_level(4) __msg("stack depth max 512")
// __msg("subprog 0 (private_stack_nested_1) main {{.*}} stack 512")
// __msg("subprog 1 (cumulative_stack_depth_subprog) static {{.*}} stack 32")
// __arch_x86_64
// private stack fp for the main prog
// __jited("	movabsq	$0x{{.*}}, %r9")
// __jited("	addq	%gs:{{.*}}, %r9")
// __jited("	movl	$0x2a, %edi")
// __jited("	movq	%rdi, -0x200(%r9)")
// __jited("	pushq	%r9")
// __jited("...")
// __jited("	callq	0x{{.*}}")
// __jited("	popq	%r9")
// __jited("	xorl	%eax, %eax")
// __arch_arm64
// __jited("	stp	x25, x27, [sp, {{.*}}]!")
// __jited("	mov	x27, {{.*}}")
// __jited("	movk	x27, {{.*}}, lsl #16")
// __jited("	movk	x27, {{.*}}")
// __jited("	mrs	x10, TPIDR_EL{{[0-1]}}")
// __jited("	add	x27, x27, x10")
// __jited("	add	x25, x27, {{.*}}")
// __jited("	mov	x0, #0x2a")
// __jited("	str	x0, [x27]")
// __jited("	bl	{{.*}}")
// __jited("...")
// __jited("	ldp	x25, x27, [sp], {{.*}}")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_nested_1() {
    unsafe {
        asm!(
            "r1 = 42",
            "*(u64 *)(r10 - {max_bpf_stack}) = r1",
            "call cumulative_stack_depth_subprog",
            "r0 = 0",
            "exit",
            max_bpf_stack = const MAX_BPF_STACK,
            options(noreturn)
        );
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn loop_callback() -> u64 {
    unsafe {
        asm!(
            "call {bpf_get_prandom_u32}",
            "r1 = 42",
            "*(u64 *)(r10 - 512) = r1",
            "call cumulative_stack_depth_subprog",
            "r0 = 0",
            "exit",
            bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
            options(noreturn)
        );
    }
}

// SEC("raw_tp")
// __description("Private stack, callback")
// __success
// __arch_x86_64
// for func loop_callback
// __jited("func #1")
// __jited("	endbr64")
// __jited("	nopl	(%rax,%rax)")
// __jited("	nopl	(%rax)")
// __jited("	pushq	%rbp")
// __jited("	movq	%rsp, %rbp")
// __jited("	endbr64")
// __jited("	movabsq	$0x{{.*}}, %r9")
// __jited("	addq	%gs:{{.*}}, %r9")
// __jited("	pushq	%r9")
// __jited("...")
// __jited("	callq")
// __jited("	popq	%r9")
// __jited("	movl	$0x2a, %edi")
// __jited("	movq	%rdi, -0x200(%r9)")
// __jited("	pushq	%r9")
// __jited("...")
// __jited("	callq")
// __jited("	popq	%r9")
// __arch_arm64
// __jited("func #1")
// __jited("...")
// __jited("	stp	x25, x27, [sp, {{.*}}]!")
// __jited("	mov	x27, {{.*}}")
// __jited("	movk	x27, {{.*}}, lsl #16")
// __jited("	movk	x27, {{.*}}")
// __jited("	mrs	x10, TPIDR_EL{{[0-1]}}")
// __jited("	add	x27, x27, x10")
// __jited("	add	x25, x27, {{.*}}")
// __jited("	bl	0x{{.*}}")
// __jited("	mov	x8, x0")
// __jited("	mov	x0, #0x2a")
// __jited("	str	x0, [x27]")
// __jited("	bl	0x{{.*}}")
// __jited("	mov	x8, x0")
// __jited("	mov	x8, #0x0")
// __jited("	ldp	x25, x27, [sp], {{.*}}")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_callback() {
    unsafe {
        asm!(
            "r1 = 1",
            "r2 = {loop_callback}",
            "r3 = 0",
            "r4 = 0",
            "call {bpf_loop}",
            "r0 = 0",
            "exit",
            loop_callback = sym loop_callback,
            bpf_loop = sym bpf_loop,
            options(noreturn)
        );
    }
}

// SEC("fentry/bpf_fentry_test9")
// __description("Private stack, exception in main prog")
// __success __retval(0)
// __arch_x86_64
// __jited("	pushq	%r9")
// __jited("...")
// __jited("	callq")
// __jited("	popq	%r9")
// __arch_arm64
// __jited("	stp	x29, x30, [sp, #-0x10]!")
// __jited("	mov	x29, sp")
// __jited("	stp	xzr, x26, [sp, #-0x10]!")
// __jited("	mov	x26, sp")
// __jited("	stp	x19, x20, [sp, #-0x10]!")
// __jited("	stp	x21, x22, [sp, #-0x10]!")
// __jited("	stp	x23, x24, [sp, #-0x10]!")
// __jited("	stp	x25, x26, [sp, #-0x10]!")
// __jited("	stp	x27, x28, [sp, #-0x10]!")
// __jited("	mov	x27, {{.*}}")
// __jited("	movk	x27, {{.*}}, lsl #16")
// __jited("	movk	x27, {{.*}}")
// __jited("	mrs	x10, TPIDR_EL{{[0-1]}}")
// __jited("	add	x27, x27, x10")
// __jited("	add	x25, x27, {{.*}}")
// __jited("	mov	x0, #0x2a")
// __jited("	str	x0, [x27]")
// __jited("	mov	x0, #0x0")
// __jited("	bl	0x{{.*}}")
// __jited("	mov	x8, x0")
// __jited("	ldp	x27, x28, [sp], #0x10")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_exception_main_prog() -> i32 {
    unsafe {
        asm!(
            "r1 = 42",
            "*(u64 *)(r10 - 512) = r1",
        );
        bpf_throw(0);
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn subprog_exception() -> i32 {
    unsafe {
        bpf_throw(0);
    }
}

// SEC("fentry/bpf_fentry_test9")
// __description("Private stack, exception in subprog")
// __success __retval(0)
// __arch_x86_64
// __jited("	movq	%rdi, -0x200(%r9)")
// __jited("	pushq	%r9")
// __jited("...")
// __jited("	callq")
// __jited("	popq	%r9")
// __arch_arm64
// __jited("	stp	x27, x28, [sp, #-0x10]!")
// __jited("	mov	x27, {{.*}}")
// __jited("	movk	x27, {{.*}}, lsl #16")
// __jited("	movk	x27, {{.*}}")
// __jited("	mrs	x10, TPIDR_EL{{[0-1]}}")
// __jited("	add	x27, x27, x10")
// __jited("	add	x25, x27, {{.*}}")
// __jited("	mov	x0, #0x2a")
// __jited("	str	x0, [x27]")
// __jited("	bl	0x{{.*}}")
// __jited("	mov	x8, x0")
// __jited("	ldp	x27, x28, [sp], #0x10")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_exception_sub_prog() -> i32 {
    unsafe {
        asm!(
            "r1 = 42",
            "*(u64 *)(r10 - 512) = r1",
            "call subprog_exception",
        );
    }

    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
pub static mut glob: i32 = 0;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn subprog2(val: *mut i32) {
    unsafe {
        glob += *val.add(0) * 2;
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn subprog1(val: *mut i32) {
    let mut tmp: [i32; 64] = [0; 64];

    unsafe {
        tmp[0] = *val;
        subprog2(tmp.as_mut_ptr());
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn timer_cb1(
    _map: *mut core::ffi::c_void,
    key: *mut i32,
    _timer: *mut bpf_timer,
) -> i32 {
    unsafe {
        subprog1(key);
    }
    0
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
unsafe extern "C" fn timer_cb2(
    _map: *mut core::ffi::c_void,
    _key: *mut i32,
    _timer: *mut bpf_timer,
) -> i32 {
    0
}

// SEC("fentry/bpf_fentry_test9")
// __description("Private stack, async callback, not nested")
// __success __retval(0)
// __arch_x86_64
// __jited("	movabsq	$0x{{.*}}, %r9")
// __arch_arm64
// __jited("	mrs	x10, TPIDR_EL{{[0-1]}}")
// __jited("	add	x27, x27, x10")
// __jited("	add	x25, x27, {{.*}}")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_async_callback_1() -> i32 {
    let mut arr_timer: *mut bpf_timer;
    let mut array_key: i32 = 0;

    unsafe {
        arr_timer = bpf_map_lookup_elem(
            (&raw mut array).cast::<core::ffi::c_void>(),
            (&raw const array_key).cast::<core::ffi::c_void>(),
        )
        .cast::<bpf_timer>();
        if arr_timer.is_null() {
            return 0;
        }

        bpf_timer_init(arr_timer, (&raw mut array).cast::<core::ffi::c_void>(), 1);
        bpf_timer_set_callback(arr_timer, timer_cb2);
        bpf_timer_start(arr_timer, 0, 0);
        subprog1(&mut array_key);
    }
    0
}

// SEC("fentry/bpf_fentry_test9")
// __description("Private stack, async callback, potential nesting")
// __success __retval(0)
// __load_if_JITed()
// __log_level(4) __msg("stack depth max 272")
// __msg("subprog 0 (private_stack_async_callback_2) main {{.*}} stack 8")
// __msg("subprog 1 (timer_cb1) static {{.*}} stack 0")
// __msg("subprog 2 (subprog1) static {{.*}} stack 256")
// __msg("subprog 3 (subprog2) static {{.*}} stack 0")
// __arch_x86_64
// __jited("	subq	$0x100, %rsp")
// __arch_arm64
// __jited("	sub	sp, sp, #0x100")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_async_callback_2() -> i32 {
    let mut arr_timer: *mut bpf_timer;
    let mut array_key: i32 = 0;

    unsafe {
        arr_timer = bpf_map_lookup_elem(
            (&raw mut array).cast::<core::ffi::c_void>(),
            (&raw const array_key).cast::<core::ffi::c_void>(),
        )
        .cast::<bpf_timer>();
        if arr_timer.is_null() {
            return 0;
        }

        bpf_timer_init(arr_timer, (&raw mut array).cast::<core::ffi::c_void>(), 1);
        bpf_timer_set_callback(arr_timer, timer_cb1);
        bpf_timer_start(arr_timer, 0, 0);
        subprog1(&mut array_key);
    }
    0
}

// SEC("fentry/bpf_fentry_test9")
// __description("private stack, max stack depth is private stack")
// __success
// __log_level(4) __msg("stack depth max 256")
// __msg("subprog 0 (private_stack_max_depth) main {{.*}} stack 8")
// __msg("subprog 1 (subprog1) static insns_self {{[0-9]+}} insns_total {{[0-9]+}} stack 256")
// __msg("subprog 2 (subprog2) static insns_self {{[0-9]+}} insns_total {{[0-9]+}} stack 0")
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn private_stack_max_depth() -> i32 {
    let mut x: i32 = 0;

    unsafe {
        subprog1(&mut x);
    }
    0
}

// Original C #else for architectures without private stack support.
// SEC("kprobe")
// __description("private stack is not supported, use a dummy test")
// __success
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[unsafe(no_mangle)]
pub extern "C" fn dummy_test() -> i32 {
    0
}

#[link_section = "license"]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
