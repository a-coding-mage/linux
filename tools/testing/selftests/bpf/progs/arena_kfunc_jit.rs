// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

/*
 * Verify the JIT-emitted rebase sequences for __arena and __arena__nullable
 * kfunc arguments. The capture kfuncs take the argument without
 * dereferencing it, so these tests pin only the emitted code.
 */
/* BPF_NO_KFUNC_PROTOTYPES */
/* C includes translated as external dependencies:
 * <vmlinux.h>
 * <bpf/bpf_helpers.h>
 * "bpf_misc.h"
 * "bpf_experimental.h"
 * <bpf_arena_common.h>
 * "../test_kmods/bpf_testmod_kfunc.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u64 = u64;

const BPF_MAP_TYPE_ARENA: u32 = 0; /* external BPF constant */
const BPF_F_MMAPABLE: u32 = 0; /* external BPF constant */
const NUMA_NO_NODE: i32 = -1; /* external NUMA constant */

#[repr(C)]
pub struct arena_map {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut arena: arena_map = arena_map {
    type_: BPF_MAP_TYPE_ARENA,
    map_flags: BPF_F_MMAPABLE,
    max_entries: 1,
};

/* volatile to force the scalar reloads below */
#[no_mangle]
pub static mut stash: u64 = 0;

unsafe extern "C" {
    fn bpf_arena_alloc_pages(
        arena: *mut arena_map,
        addr: *mut core::ffi::c_void,
        page_cnt: u32,
        node: i32,
        flags: u64,
    ) -> *mut u64;

    fn bpf_kfunc_arena_cap_test(arg: *mut u64);
    fn bpf_kfunc_arena_cap_nullable_test(arg: *mut u64);
    fn bpf_kfunc_arena_args5_test(
        arg0: *mut u64,
        arg1: *mut u64,
        arg2: *mut u64,
        arg3: *mut u64,
        arg4: *mut u64,
    );
}

/* #if defined(__BPF_FEATURE_ADDR_SPACE_CAST) */

/*
 * SEC("syscall")
 * __arch_x86_64
 * __jited("...")
 * __jited("	movl	%edi, %edi")
 * __jited("	addq	%r12, %rdi")
 * __jited("...")
 * __jited("	callq	{{.*}}")
 * __arch_arm64
 * __jited("...")
 * __jited("	add	x0, x28, w0, uxtw")
 * __jited("	{{(bl|mov)	.*}}")
 * __success
 */
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_jit_rebase(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    core::ptr::write_volatile(
        core::ptr::addr_of_mut!(stash),
        bpf_arena_alloc_pages(
            core::ptr::addr_of_mut!(arena),
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        ) as u64,
    );
    bpf_kfunc_arena_cap_test(core::ptr::read_volatile(core::ptr::addr_of!(stash)) as *mut u64);
    return 0;
}

/*
 * SEC("syscall")
 * __arch_x86_64
 * __jited("...")
 * __jited("	movl	%edi, %edi")
 * __jited("	testl	%edi, %edi")
 * __jited("	je	L0")
 * __jited("	addq	%r12, %rdi")
 * __jited("L0:	callq	{{.*}}")
 * __arch_arm64
 * __jited("...")
 * __jited("	mov	w0, w0")
 * __jited("	cbz	w0, L0")
 * __jited("	add	x0, x28, w0, uxtw")
 * __jited("L0:	{{.*}}")
 * __success
 */
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_jit_nullable(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;

    core::ptr::write_volatile(
        core::ptr::addr_of_mut!(stash),
        bpf_arena_alloc_pages(
            core::ptr::addr_of_mut!(arena),
            core::ptr::null_mut(),
            1,
            NUMA_NO_NODE,
            0,
        ) as u64,
    );
    bpf_kfunc_arena_cap_nullable_test(
        core::ptr::read_volatile(core::ptr::addr_of!(stash)) as *mut u64,
    );
    return 0;
}

/*
 * SEC("syscall")
 * __arch_x86_64
 * __jited("...")
 * __jited("	movl	%edi, %edi")
 * __jited("	addq	%r12, %rdi")
 * __jited("	movl	%esi, %esi")
 * __jited("	addq	%r12, %rsi")
 * __jited("	movl	%edx, %edx")
 * __jited("	addq	%r12, %rdx")
 * __jited("	movl	%ecx, %ecx")
 * __jited("	addq	%r12, %rcx")
 * __jited("	movl	%r8d, %r8d")
 * __jited("	testl	%r8d, %r8d")
 * __jited("	je	L0")
 * __jited("	addq	%r12, %r8")
 * __jited("L0:	callq	{{.*}}")
 * __arch_arm64
 * __jited("...")
 * __jited("	add	x0, x28, w0, uxtw")
 * __jited("	add	x1, x28, w1, uxtw")
 * __jited("	add	x2, x28, w2, uxtw")
 * __jited("	add	x3, x28, w3, uxtw")
 * __jited("	mov	w4, w4")
 * __jited("	cbz	w4, L0")
 * __jited("	add	x4, x28, w4, uxtw")
 * __jited("L0:	{{.*}}")
 * __success
 */
#[no_mangle]
#[link_section = "syscall"]
pub unsafe extern "C" fn arena_arg_jit_args5(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let mut val: *mut u64;

    val = bpf_arena_alloc_pages(
        core::ptr::addr_of_mut!(arena),
        core::ptr::null_mut(),
        1,
        NUMA_NO_NODE,
        0,
    );
    if val.is_null() {
        return 1;
    }

    *val.add(0) = 1;
    *val.add(1) = 2;
    *val.add(2) = 4;
    *val.add(3) = 8;
    *val.add(4) = 16;

    bpf_kfunc_arena_args5_test(
        val.add(0) as *mut u64,
        val.add(1) as *mut u64,
        val.add(2) as *mut u64,
        val.add(3) as *mut u64,
        val.add(4) as *mut u64,
    );
    return 0;
}

/* #endif /* __BPF_FEATURE_ADDR_SPACE_CAST */ */

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
