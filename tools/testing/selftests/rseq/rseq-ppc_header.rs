/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-ppc.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 * (C) Copyright 2016-2018 - Boqun Feng <boqun.feng@gmail.com>
 */

/*
 * RSEQ_SIG is used with the following trap instruction:
 *
 * powerpc-be:    0f e5 00 0b           twui   r5,11
 * powerpc64-le:  0b 00 e5 0f           twui   r5,11
 * powerpc64-be:  0f e5 00 0b           twui   r5,11
 */
pub const RSEQ_SIG: u32 = 0x0fe5000b;

#[inline(always)]
pub unsafe fn rseq_smp_mb() {
    core::arch::asm!("sync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn rseq_smp_lwsync() {
    core::arch::asm!("lwsync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn rseq_smp_rmb() {
    rseq_smp_lwsync();
}

#[inline(always)]
pub unsafe fn rseq_smp_wmb() {
    rseq_smp_lwsync();
}

#[inline(always)]
pub unsafe fn rseq_smp_load_acquire<T: Copy>(p: *const T) -> T {
    let ____p1 = core::ptr::read_volatile(p);
    rseq_smp_lwsync();
    ____p1
}

#[inline(always)]
pub unsafe fn rseq_smp_acquire__after_ctrl_dep() {
    rseq_smp_lwsync();
}

#[inline(always)]
pub unsafe fn rseq_smp_store_release<T>(p: *mut T, v: T) {
    rseq_smp_lwsync();
    core::ptr::write_volatile(p, v);
}

/*
 * The __rseq_cs_ptr_array and __rseq_cs sections can be used by debuggers to
 * better handle single-stepping through the restartable critical sections.
 */

#[cfg(target_pointer_width = "64")]
macro_rules! RSEQ_STORE_LONG {
    ($arg:expr) => {
        concat!("std%U[", $arg, "]%X[", $arg, "] ")
    };
}

#[cfg(target_pointer_width = "64")]
macro_rules! RSEQ_STORE_INT {
    ($arg:expr) => {
        concat!("stw%U[", $arg, "]%X[", $arg, "] ")
    };
}

#[cfg(target_pointer_width = "64")]
macro_rules! RSEQ_LOAD_LONG {
    ($arg:expr) => {
        concat!("ld%U[", $arg, "]%X[", $arg, "] ")
    };
}

#[cfg(target_pointer_width = "64")]
macro_rules! RSEQ_LOAD_INT {
    ($arg:expr) => {
        concat!("lwz%U[", $arg, "]%X[", $arg, "] ")
    };
}

#[cfg(target_pointer_width = "64")]
pub const RSEQ_LOADX_LONG: &str = "ldx ";
#[cfg(target_pointer_width = "64")]
pub const RSEQ_CMP_LONG: &str = "cmpd ";
#[cfg(target_pointer_width = "64")]
pub const RSEQ_CMP_LONG_INT: &str = "cmpdi ";

#[cfg(target_pointer_width = "64")]
macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".pushsection __rseq_cs, \"aw\"\n\t",
            ".balign 32\n\t",
            $label, ":\n\t",
            ".long ", $version, ", ", $flags, "\n\t",
            ".quad ", $start_ip, ", ", $post_commit_offset, ", ", $abort_ip, "\n\t",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n\t",
            ".quad ", $label, "b\n\t",
            ".popsection\n\t",
        )
    };
}

#[cfg(target_pointer_width = "64")]
macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            "lis %%r17, (", $cs_label, ")@highest\n\t",
            "ori %%r17, %%r17, (", $cs_label, ")@higher\n\t",
            "rldicr %%r17, %%r17, 32, 31\n\t",
            "oris %%r17, %%r17, (", $cs_label, ")@high\n\t",
            "ori %%r17, %%r17, (", $cs_label, ")@l\n\t",
            "std %%r17, %[", $rseq_cs, "]\n\t",
            $label, ":\n\t",
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
#[cfg(target_pointer_width = "64")]
macro_rules! RSEQ_ASM_DEFINE_EXIT_POINT {
    ($start_ip:expr, $exit_ip:expr) => {
        concat!(
            ".pushsection __rseq_exit_point_array, \"aw\"\n\t",
            ".quad ", $start_ip, ", ", $exit_ip, "\n\t",
            ".popsection\n\t",
        )
    };
}

#[cfg(not(target_pointer_width = "64"))]
macro_rules! RSEQ_STORE_LONG {
    ($arg:expr) => {
        concat!("stw%U[", $arg, "]%X[", $arg, "] ")
    };
}

#[cfg(not(target_pointer_width = "64"))]
macro_rules! RSEQ_STORE_INT {
    ($arg:expr) => {
        RSEQ_STORE_LONG!($arg)
    };
}

#[cfg(not(target_pointer_width = "64"))]
macro_rules! RSEQ_LOAD_LONG {
    ($arg:expr) => {
        concat!("lwz%U[", $arg, "]%X[", $arg, "] ")
    };
}

#[cfg(not(target_pointer_width = "64"))]
macro_rules! RSEQ_LOAD_INT {
    ($arg:expr) => {
        RSEQ_LOAD_LONG!($arg)
    };
}

#[cfg(not(target_pointer_width = "64"))]
pub const RSEQ_LOADX_LONG: &str = "lwzx ";
#[cfg(not(target_pointer_width = "64"))]
pub const RSEQ_CMP_LONG: &str = "cmpw ";
#[cfg(not(target_pointer_width = "64"))]
pub const RSEQ_CMP_LONG_INT: &str = "cmpwi ";

#[cfg(not(target_pointer_width = "64"))]
macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".pushsection __rseq_cs, \"aw\"\n\t",
            ".balign 32\n\t",
            $label, ":\n\t",
            ".long ", $version, ", ", $flags, "\n\t",
            /* 32-bit only supported on BE */
            ".long 0x0, ", $start_ip, ", 0x0, ", $post_commit_offset, ", 0x0, ", $abort_ip, "\n\t",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n\t",
            ".long 0x0, ", $label, "b\n\t",
            ".popsection\n\t",
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
#[cfg(not(target_pointer_width = "64"))]
macro_rules! RSEQ_ASM_DEFINE_EXIT_POINT {
    ($start_ip:expr, $exit_ip:expr) => {
        concat!(
            ".pushsection __rseq_exit_point_array, \"aw\"\n\t",
            /* 32-bit only supported on BE */
            ".long 0x0, ", $start_ip, ", 0x0, ", $exit_ip, "\n\t",
            ".popsection\n\t",
        )
    };
}

#[cfg(not(target_pointer_width = "64"))]
macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            "lis %%r17, (", $cs_label, ")@ha\n\t",
            "addi %%r17, %%r17, (", $cs_label, ")@l\n\t",
            RSEQ_STORE_INT!($rseq_cs), "%%r17, %[", $rseq_cs, "]\n\t",
            $label, ":\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $start_ip:expr, $post_commit_ip:expr, $abort_ip:expr) => {
        __RSEQ_ASM_DEFINE_TABLE!(
            $label,
            "0x0",
            "0x0",
            $start_ip,
            concat!("(", $post_commit_ip, " - ", $start_ip, ")"),
            $abort_ip
        )
    };
}

macro_rules! RSEQ_ASM_CMP_CPU_ID {
    ($cpu_id:expr, $current_cpu_id:expr, $label:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(2),
            RSEQ_LOAD_INT!($current_cpu_id), "%%r17, %[", $current_cpu_id, "]\n\t",
            "cmpw cr7, %[", $cpu_id, "], %%r17\n\t",
            "bne- cr7, ", $label, "\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_ABORT {
    ($label:expr, $abort_label:expr) => {
        concat!(
            ".pushsection __rseq_failure, \"ax\"\n\t",
            ".long ", stringify!(RSEQ_SIG), "\n\t",
            $label, ":\n\t",
            "b %l[", $abort_label, "]\n\t",
            ".popsection\n\t",
        )
    };
}

/*
 * RSEQ_ASM_OPs: asm operations for rseq
 * 	RSEQ_ASM_OP_R_*: has hard-code registers in it
 * 	RSEQ_ASM_OP_* (else): doesn't have hard-code registers(unless cr7)
 */
macro_rules! RSEQ_ASM_OP_CMPEQ {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            RSEQ_LOAD_LONG!($var), "%%r17, %[", $var, "]\n\t",
            RSEQ_CMP_LONG, "cr7, %%r17, %[", $expect, "]\n\t",
            "bne- cr7, ", $label, "\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_OP_CMPNE {
    ($var:expr, $expectnot:expr, $label:expr) => {
        concat!(
            RSEQ_LOAD_LONG!($var), "%%r17, %[", $var, "]\n\t",
            RSEQ_CMP_LONG, "cr7, %%r17, %[", $expectnot, "]\n\t",
            "beq- cr7, ", $label, "\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_OP_STORE {
    ($value:expr, $var:expr) => {
        concat!(RSEQ_STORE_LONG!($var), "%[", $value, "], %[", $var, "]\n\t")
    };
}

/* Load @var to r17 */
macro_rules! RSEQ_ASM_OP_R_LOAD {
    ($var:expr) => {
        concat!(RSEQ_LOAD_LONG!($var), "%%r17, %[", $var, "]\n\t")
    };
}

/* Store r17 to @var */
macro_rules! RSEQ_ASM_OP_R_STORE {
    ($var:expr) => {
        concat!(RSEQ_STORE_LONG!($var), "%%r17, %[", $var, "]\n\t")
    };
}

/* Add @count to r17 */
macro_rules! RSEQ_ASM_OP_R_ADD {
    ($count:expr) => {
        concat!("add %%r17, %[", $count, "], %%r17\n\t")
    };
}

/* Load (r17 + voffp) to r17 */
macro_rules! RSEQ_ASM_OP_R_LOADX {
    ($voffp:expr) => {
        concat!(RSEQ_LOADX_LONG, "%%r17, %[", $voffp, "], %%r17\n\t")
    };
}

/* TODO: implement a faster memcpy. */
macro_rules! RSEQ_ASM_OP_R_MEMCPY {
    () => {
        concat!(
            RSEQ_CMP_LONG_INT, "%%r19, 0\n\t",
            "beq 333f\n\t",
            "addi %%r20, %%r20, -1\n\t",
            "addi %%r21, %%r21, -1\n\t",
            "222:\n\t",
            "lbzu %%r18, 1(%%r20)\n\t",
            "stbu %%r18, 1(%%r21)\n\t",
            "addi %%r19, %%r19, -1\n\t",
            RSEQ_CMP_LONG_INT, "%%r19, 0\n\t",
            "bne 222b\n\t",
            "333:\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_FINAL_STORE {
    ($var:expr, $post_commit_label:expr) => {
        concat!(
            RSEQ_STORE_LONG!($var), "%%r17, %[", $var, "]\n\t",
            $post_commit_label, ":\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_OP_FINAL_STORE {
    ($value:expr, $var:expr, $post_commit_label:expr) => {
        concat!(
            RSEQ_STORE_LONG!($var), "%[", $value, "], %[", $var, "]\n\t",
            $post_commit_label, ":\n\t",
        )
    };
}

/*
 * Per-cpu-id indexing.
 *
 * C template expansion:
 *   #define RSEQ_TEMPLATE_CPU_ID
 *   #define RSEQ_TEMPLATE_MO_RELAXED
 *   #include "rseq-ppc-bits.h"
 *   #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 *   #define RSEQ_TEMPLATE_MO_RELEASE
 *   #include "rseq-ppc-bits.h"
 *   #undef RSEQ_TEMPLATE_MO_RELEASE
 *   #undef RSEQ_TEMPLATE_CPU_ID
 */

/*
 * Per-mm-cid indexing.
 *
 * C template expansion:
 *   #define RSEQ_TEMPLATE_MM_CID
 *   #define RSEQ_TEMPLATE_MO_RELAXED
 *   #include "rseq-ppc-bits.h"
 *   #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 *   #define RSEQ_TEMPLATE_MO_RELEASE
 *   #include "rseq-ppc-bits.h"
 *   #undef RSEQ_TEMPLATE_MO_RELEASE
 *   #undef RSEQ_TEMPLATE_MM_CID
 */

/*
 * APIs which are not based on cpu ids.
 *
 * C template expansion:
 *   #define RSEQ_TEMPLATE_CPU_ID_NONE
 *   #define RSEQ_TEMPLATE_MO_RELAXED
 *   #include "rseq-ppc-bits.h"
 *   #undef RSEQ_TEMPLATE_MO_RELAXED
 *   #undef RSEQ_TEMPLATE_CPU_ID_NONE
 */
