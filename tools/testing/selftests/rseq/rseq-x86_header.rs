/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-x86.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * Original C header guard requires including this through <rseq.h>:
 * #ifndef RSEQ_H
 * #error "Never use <rseq-x86.h> directly; include <rseq.h> instead."
 * #endif
 */

/*
 * RSEQ_SIG is used with the following reserved undefined instructions, which
 * trap in user-space:
 *
 * x86-32:    0f b9 3d 53 30 05 53      ud1    0x53053053,%edi
 * x86-64:    0f b9 3d 53 30 05 53      ud1    0x53053053(%rip),%edi
 */
pub const RSEQ_SIG: u32 = 0x53053053;

/*
 * Due to a compiler optimization bug in gcc-8 with asm goto and TLS asm input
 * operands, we cannot use "m" input operands, and rather pass the __rseq_abi
 * address through a "r" input operand.
 */

/* Offset of cpu_id, rseq_cs, and mm_cid fields in struct rseq. */
pub const RSEQ_CPU_ID_OFFSET: usize = 4;
pub const RSEQ_CS_OFFSET: usize = 8;
pub const RSEQ_MM_CID_OFFSET: usize = 24;

#[cfg(target_arch = "x86_64")]
pub const RSEQ_ASM_TP_SEGMENT: &str = "%%fs";

#[cfg(target_arch = "x86")]
pub const RSEQ_ASM_TP_SEGMENT: &str = "%%gs";

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_smp_mb() {
    core::arch::asm!(
        "lock; addl $0,-128(%rsp)",
        options(nostack, preserves_flags)
    );
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn rseq_smp_mb() {
    core::arch::asm!(
        "lock; addl $0,-128(%esp)",
        options(nostack, preserves_flags)
    );
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_smp_rmb() {
    rseq_barrier();
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn rseq_smp_rmb() {
    rseq_smp_mb();
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_smp_wmb() {
    rseq_barrier();
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn rseq_smp_wmb() {
    rseq_smp_mb();
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_smp_load_acquire<T: Copy>(p: *const T) -> T {
    let ____p1 = RSEQ_READ_ONCE(p);
    rseq_barrier();
    ____p1
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn rseq_smp_load_acquire<T: Copy>(p: *const T) -> T {
    let ____p1 = RSEQ_READ_ONCE(p);
    rseq_smp_mb();
    ____p1
}

#[inline(always)]
pub unsafe fn rseq_smp_acquire__after_ctrl_dep() {
    rseq_smp_rmb();
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn rseq_smp_store_release<T>(p: *mut T, v: T) {
    rseq_barrier();
    RSEQ_WRITE_ONCE(p, v);
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn rseq_smp_store_release<T>(p: *mut T, v: T) {
    rseq_smp_mb();
    RSEQ_WRITE_ONCE(p, v);
}

#[cfg(target_arch = "x86_64")]
macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:tt, $version:tt, $flags:tt, $start_ip:tt, $post_commit_offset:tt, $abort_ip:tt) => {
        concat!(
            ".pushsection __rseq_cs, \"aw\"\n\t",
            ".balign 32\n\t",
            stringify!($label), ":\n\t",
            ".long ", stringify!($version), ", ", stringify!($flags), "\n\t",
            ".quad ", stringify!($start_ip), ", ", stringify!($post_commit_offset), ", ", stringify!($abort_ip), "\n\t",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n\t",
            ".quad ", stringify!($label), "b\n\t",
            ".popsection\n\t",
        )
    };
}

#[cfg(target_arch = "x86")]
macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:tt, $version:tt, $flags:tt, $start_ip:tt, $post_commit_offset:tt, $abort_ip:tt) => {
        concat!(
            ".pushsection __rseq_cs, \"aw\"\n\t",
            ".balign 32\n\t",
            stringify!($label), ":\n\t",
            ".long ", stringify!($version), ", ", stringify!($flags), "\n\t",
            ".long ", stringify!($start_ip), ", 0x0, ", stringify!($post_commit_offset), ", 0x0, ", stringify!($abort_ip), ", 0x0\n\t",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n\t",
            ".long ", stringify!($label), "b, 0x0\n\t",
            ".popsection\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_TABLE {
    ($label:tt, $start_ip:tt, $post_commit_ip:tt, $abort_ip:tt) => {
        __RSEQ_ASM_DEFINE_TABLE!(
            $label,
            0x0,
            0x0,
            $start_ip,
            ($post_commit_ip - $start_ip),
            $abort_ip
        )
    };
}

/*
 * Exit points of a rseq critical section consist of all instructions outside
 * of the critical section where a critical section can either branch to or
 * reach through the normal course of its execution. The abort IP and the
 * post-commit IP are already part of the __rseq_cs section and should not be
 * explicitly defined as additional exit points. Knowing all exit points is
 * useful to assist debuggers stepping over the critical section.
 */

#[cfg(target_arch = "x86_64")]
pub const RSEQ_ASM_DEFINE_EXIT_POINT_BODY: &str =
    ".pushsection __rseq_exit_point_array, \"aw\"\n\t\
     .quad {start_ip}, {exit_ip}\n\t\
     .popsection\n\t";

#[cfg(target_arch = "x86")]
pub const RSEQ_ASM_DEFINE_EXIT_POINT_BODY: &str =
    ".pushsection __rseq_exit_point_array, \"aw\"\n\t\
     .long {start_ip}, 0x0, {exit_ip}, 0x0\n\t\
     .popsection\n\t";

#[cfg(target_arch = "x86_64")]
pub const RSEQ_ASM_STORE_RSEQ_CS_BODY: &str =
    "{RSEQ_INJECT_ASM_1}\
     leaq {cs_label}(%rip), %rax\n\t\
     movq %rax, {rseq_cs}\n\t\
     {label}:\n\t";

#[cfg(target_arch = "x86")]
pub const RSEQ_ASM_STORE_RSEQ_CS_BODY: &str =
    "{RSEQ_INJECT_ASM_1}\
     movl ${cs_label}, {rseq_cs}\n\t\
     {label}:\n\t";

pub const RSEQ_ASM_CMP_CPU_ID_BODY: &str =
    "{RSEQ_INJECT_ASM_2}\
     cmpl %[{cpu_id}], {current_cpu_id}\n\t\
     jnz {label}\n\t";

#[cfg(target_arch = "x86_64")]
pub const RSEQ_ASM_DEFINE_ABORT_BODY: &str =
    ".pushsection __rseq_failure, \"ax\"\n\t\
     .byte 0x0f, 0xb9, 0x3d\n\t\
     .long 0x53053053\n\t\
     {label}:\n\t\
     {teardown}\
     jmp %l[{abort_label}]\n\t\
     .popsection\n\t";

#[cfg(target_arch = "x86")]
pub const RSEQ_ASM_DEFINE_ABORT_BODY: &str =
    ".pushsection __rseq_failure, \"ax\"\n\t\
     .byte 0x0f, 0xb9, 0x3d\n\t\
     .long 0x53053053\n\t\
     {label}:\n\t\
     {teardown}\
     jmp %l[{abort_label}]\n\t\
     .popsection\n\t";

pub const RSEQ_ASM_DEFINE_CMPFAIL_BODY: &str =
    ".pushsection __rseq_failure, \"ax\"\n\t\
     {label}:\n\t\
     {teardown}\
     jmp %l[{cmpfail_label}]\n\t\
     .popsection\n\t";

/*
 * The following C preprocessor template inclusions instantiate APIs from
 * "rseq-x86-bits.h". The dependency and template intent are preserved here;
 * the included file is outside this isolated translation unit.
 */

/* Per-cpu-id indexing. */
/* #define RSEQ_TEMPLATE_CPU_ID */
/* #define RSEQ_TEMPLATE_MO_RELAXED */
/* #include "rseq-x86-bits.h" */
/* #undef RSEQ_TEMPLATE_MO_RELAXED */

/* #define RSEQ_TEMPLATE_MO_RELEASE */
/* #include "rseq-x86-bits.h" */
/* #undef RSEQ_TEMPLATE_MO_RELEASE */
/* #undef RSEQ_TEMPLATE_CPU_ID */

/* Per-mm-cid indexing. */
/* #define RSEQ_TEMPLATE_MM_CID */
/* #define RSEQ_TEMPLATE_MO_RELAXED */
/* #include "rseq-x86-bits.h" */
/* #undef RSEQ_TEMPLATE_MO_RELAXED */

/* #define RSEQ_TEMPLATE_MO_RELEASE */
/* #include "rseq-x86-bits.h" */
/* #undef RSEQ_TEMPLATE_MO_RELEASE */
/* #undef RSEQ_TEMPLATE_MM_CID */

/* APIs which are not based on cpu ids. */
/* #define RSEQ_TEMPLATE_CPU_ID_NONE */
/* #define RSEQ_TEMPLATE_MO_RELAXED */
/* #include "rseq-x86-bits.h" */
/* #undef RSEQ_TEMPLATE_MO_RELAXED */
/* #undef RSEQ_TEMPLATE_CPU_ID_NONE */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
