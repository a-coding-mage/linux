/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-arm.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * - ARM little endian
 *
 * RSEQ_SIG uses the udf A32 instruction with an uncommon immediate operand
 * value 0x5de3. This traps if user-space reaches this instruction by mistake,
 * and the uncommon operand ensures the kernel does not move the instruction
 * pointer to attacker-controlled code on rseq abort.
 *
 * The instruction pattern in the A32 instruction set is:
 *
 * e7f5def3    udf    #24035    ; 0x5de3
 *
 * This translates to the following instruction pattern in the T16 instruction
 * set:
 *
 * little endian:
 * def3        udf    #243      ; 0xf3
 * e7f5        b.n    <7f5>
 *
 * - ARMv6+ big endian (BE8):
 *
 * ARMv6+ -mbig-endian generates mixed endianness code vs data: little-endian
 * code and big-endian data. The data value of the signature needs to have its
 * byte order reversed to generate the trap instruction:
 *
 * Data: 0xf3def5e7
 *
 * Translates to this A32 instruction pattern:
 *
 * e7f5def3    udf    #24035    ; 0x5de3
 *
 * Translates to this T16 instruction pattern:
 *
 * def3        udf    #243      ; 0xf3
 * e7f5        b.n    <7f5>
 *
 * - Prior to ARMv6 big endian (BE32):
 *
 * Prior to ARMv6, -mbig-endian generates big-endian code and data
 * (which match), so the endianness of the data representation of the
 * signature should not be reversed. However, the choice between BE32
 * and BE8 is done by the linker, so we cannot know whether code and
 * data endianness will be mixed before the linker is invoked. So rather
 * than try to play tricks with the linker, the rseq signature is simply
 * data (not a trap instruction) prior to ARMv6 on big endian. This is
 * why the signature is expressed as data (.word) rather than as
 * instruction (.inst) in assembler.
 */

#[cfg(target_endian = "big")]
pub const RSEQ_SIG: u32 = 0xf3def5e7; /* udf    #24035    ; 0x5de3 (ARMv6+) */
#[cfg(not(target_endian = "big"))]
pub const RSEQ_SIG: u32 = 0xe7f5def3; /* udf    #24035    ; 0x5de3 */

#[inline(always)]
pub unsafe fn rseq_smp_mb() {
    core::arch::asm!("dmb", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn rseq_smp_rmb() {
    core::arch::asm!("dmb", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn rseq_smp_wmb() {
    core::arch::asm!("dmb", options(nostack, preserves_flags));
}

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

#[inline(always)]
pub unsafe fn rseq_smp_store_release<T>(p: *mut T, v: T) {
    rseq_smp_mb();
    RSEQ_WRITE_ONCE(p, v);
}

macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr,
     $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".pushsection __rseq_cs, \"aw\"\n\t",
            ".balign 32\n\t",
            $label, ":\n\t",
            ".word ", $version, ", ", $flags, "\n\t",
            ".word ", $start_ip, ", 0x0, ", $post_commit_offset, ", 0x0, ", $abort_ip, ", 0x0\n\t",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n\t",
            ".word ", $label, "b, 0x0\n\t",
            ".popsection\n\t",
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

/*
 * Exit points of a rseq critical section consist of all instructions outside
 * of the critical section where a critical section can either branch to or
 * reach through the normal course of its execution. The abort IP and the
 * post-commit IP are already part of the __rseq_cs section and should not be
 * explicitly defined as additional exit points. Knowing all exit points is
 * useful to assist debuggers stepping over the critical section.
 */
macro_rules! RSEQ_ASM_DEFINE_EXIT_POINT {
    ($start_ip:expr, $exit_ip:expr) => {
        concat!(
            ".pushsection __rseq_exit_point_array, \"aw\"\n\t",
            ".word ", $start_ip, ", 0x0, ", $exit_ip, ", 0x0\n\t",
            ".popsection\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            "adr r0, ", $cs_label, "\n\t",
            "str r0, %[", $rseq_cs, "]\n\t",
            $label, ":\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_CMP_CPU_ID {
    ($cpu_id:expr, $current_cpu_id:expr, $label:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(2),
            "ldr r0, %[", $current_cpu_id, "]\n\t",
            "cmp %[", $cpu_id, "], r0\n\t",
            "bne ", $label, "\n\t",
        )
    };
}

macro_rules! __RSEQ_ASM_DEFINE_ABORT {
    ($table_label:expr, $label:expr, $teardown:expr,
     $abort_label:expr, $version:expr, $flags:expr,
     $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".balign 32\n\t",
            $table_label, ":\n\t",
            ".word ", $version, ", ", $flags, "\n\t",
            ".word ", $start_ip, ", 0x0, ", $post_commit_offset, ", 0x0, ", $abort_ip, ", 0x0\n\t",
            ".word ", RSEQ_SIG_STR, "\n\t",
            $label, ":\n\t",
            $teardown,
            "b %l[", $abort_label, "]\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_ABORT {
    ($table_label:expr, $label:expr, $teardown:expr, $abort_label:expr,
     $start_ip:expr, $post_commit_ip:expr, $abort_ip:expr) => {
        __RSEQ_ASM_DEFINE_ABORT!(
            $table_label,
            $label,
            $teardown,
            $abort_label,
            "0x0",
            "0x0",
            $start_ip,
            concat!("(", $post_commit_ip, " - ", $start_ip, ")"),
            $abort_ip
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_CMPFAIL {
    ($label:expr, $teardown:expr, $cmpfail_label:expr) => {
        concat!(
            $label, ":\n\t",
            $teardown,
            "b %l[", $cmpfail_label, "]\n\t",
        )
    };
}

#[cfg(target_endian = "big")]
pub const RSEQ_SIG_STR: &str = "0xf3def5e7";
#[cfg(not(target_endian = "big"))]
pub const RSEQ_SIG_STR: &str = "0xe7f5def3";

/*
 * Per-cpu-id indexing.
 *
 * C template expansion:
 * #define RSEQ_TEMPLATE_CPU_ID
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-arm-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 * #define RSEQ_TEMPLATE_MO_RELEASE
 * #include "rseq-arm-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELEASE
 * #undef RSEQ_TEMPLATE_CPU_ID
 */

/*
 * Per-mm-cid indexing.
 *
 * C template expansion:
 * #define RSEQ_TEMPLATE_MM_CID
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-arm-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 * #define RSEQ_TEMPLATE_MO_RELEASE
 * #include "rseq-arm-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELEASE
 * #undef RSEQ_TEMPLATE_MM_CID
 */

/*
 * APIs which are not based on cpu ids.
 *
 * C template expansion:
 * #define RSEQ_TEMPLATE_CPU_ID_NONE
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-arm-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 * #undef RSEQ_TEMPLATE_CPU_ID_NONE
 */
