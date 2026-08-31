/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Author: Paul Burton <paul.burton@mips.com>
 * (C) Copyright 2018 MIPS Tech LLC
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 */

/*
 * RSEQ_SIG uses the break instruction. The instruction pattern is:
 *
 * On MIPS:
 *	0350000d        break     0x350
 *
 * On nanoMIPS:
 *      00100350        break     0x350
 *
 * On microMIPS:
 *      0000d407        break     0x350
 *
 * For nanoMIPS32 and microMIPS, the instruction stream is encoded as 16-bit
 * halfwords, so the signature halfwords need to be swapped accordingly for
 * little-endian.
 */
/* C preprocessor mapping:
 * - defined(__nanomips__) && defined(__MIPSEL__): RSEQ_SIG = 0x03500010
 * - defined(__nanomips__): RSEQ_SIG = 0x00100350
 * - defined(__mips_micromips) && defined(__MIPSEL__): RSEQ_SIG = 0xd4070000
 * - defined(__mips_micromips): RSEQ_SIG = 0x0000d407
 * - defined(__mips__): RSEQ_SIG = 0x0350000d
 * - otherwise: unknown MIPS architecture, no RSEQ_SIG definition.
 */
#[cfg(all(nanomips, target_endian = "little"))]
pub const RSEQ_SIG: u32 = 0x03500010;
#[cfg(all(nanomips, not(target_endian = "little")))]
pub const RSEQ_SIG: u32 = 0x00100350;
#[cfg(all(mips_micromips, target_endian = "little"))]
pub const RSEQ_SIG: u32 = 0xd4070000;
#[cfg(all(mips_micromips, not(target_endian = "little")))]
pub const RSEQ_SIG: u32 = 0x0000d407;
#[cfg(all(
    target_arch = "mips",
    not(nanomips),
    not(mips_micromips)
))]
pub const RSEQ_SIG: u32 = 0x0350000d;

#[inline(always)]
pub unsafe fn rseq_smp_mb() {
    core::arch::asm!("sync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn rseq_smp_rmb() {
    rseq_smp_mb();
}

#[inline(always)]
pub unsafe fn rseq_smp_wmb() {
    rseq_smp_mb();
}

#[inline(always)]
pub unsafe fn rseq_smp_load_acquire<T: Copy>(p: *const T) -> T {
    let ____p1 = core::ptr::read_volatile(p);
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
    core::ptr::write_volatile(p, v);
}

/* C preprocessor mapping:
 * - _MIPS_SZLONG == 64:
 *   LONG = ".dword", LONG_LA = "dla", LONG_L = "ld", LONG_S = "sd",
 *   LONG_ADDI = "daddiu", U32_U64_PAD(x) = x
 * - _MIPS_SZLONG == 32:
 *   LONG = ".word", LONG_LA = "la", LONG_L = "lw", LONG_S = "sw",
 *   LONG_ADDI = "addiu"
 *   If __BIG_ENDIAN: U32_U64_PAD(x) = "0x0, " x
 *   Else: U32_U64_PAD(x) = x ", 0x0"
 * - otherwise: unsupported _MIPS_SZLONG
 */
#[cfg(target_pointer_width = "64")]
pub const LONG: &str = ".dword";
#[cfg(target_pointer_width = "64")]
pub const LONG_LA: &str = "dla";
#[cfg(target_pointer_width = "64")]
pub const LONG_L: &str = "ld";
#[cfg(target_pointer_width = "64")]
pub const LONG_S: &str = "sd";
#[cfg(target_pointer_width = "64")]
pub const LONG_ADDI: &str = "daddiu";

#[cfg(target_pointer_width = "32")]
pub const LONG: &str = ".word";
#[cfg(target_pointer_width = "32")]
pub const LONG_LA: &str = "la";
#[cfg(target_pointer_width = "32")]
pub const LONG_L: &str = "lw";
#[cfg(target_pointer_width = "32")]
pub const LONG_S: &str = "sw";
#[cfg(target_pointer_width = "32")]
pub const LONG_ADDI: &str = "addiu";

#[cfg(target_pointer_width = "64")]
macro_rules! U32_U64_PAD {
    ($x:expr) => {
        $x
    };
}

#[cfg(all(target_pointer_width = "32", target_endian = "big"))]
macro_rules! U32_U64_PAD {
    ($x:expr) => {
        concat!("0x0, ", $x)
    };
}

#[cfg(all(target_pointer_width = "32", not(target_endian = "big")))]
macro_rules! U32_U64_PAD {
    ($x:expr) => {
        concat!($x, ", 0x0")
    };
}

macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".pushsection __rseq_cs, \"aw\"\n\t",
            ".balign 32\n\t",
            $label, ":\n\t",
            ".word ", $version, ", ", $flags, "\n\t",
            LONG, " ", U32_U64_PAD!($start_ip), "\n\t",
            LONG, " ", U32_U64_PAD!($post_commit_offset), "\n\t",
            LONG, " ", U32_U64_PAD!($abort_ip), "\n\t",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n\t",
            LONG, " ", U32_U64_PAD!(concat!($label, "b")), "\n\t",
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
            LONG, " ", U32_U64_PAD!($start_ip), "\n\t",
            LONG, " ", U32_U64_PAD!($exit_ip), "\n\t",
            ".popsection\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            LONG_LA, " $4, ", $cs_label, "\n\t",
            LONG_S, " $4, %[", $rseq_cs, "]\n\t",
            $label, ":\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_CMP_CPU_ID {
    ($cpu_id:expr, $current_cpu_id:expr, $label:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(2),
            "lw  $4, %[", $current_cpu_id, "]\n\t",
            "bne $4, %[", $cpu_id, "], ", $label, "\n\t",
        )
    };
}

macro_rules! __RSEQ_ASM_DEFINE_ABORT {
    ($table_label:expr, $label:expr, $teardown:expr, $abort_label:expr, $version:expr, $flags:expr, $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".balign 32\n\t",
            $table_label, ":\n\t",
            ".word ", $version, ", ", $flags, "\n\t",
            LONG, " ", U32_U64_PAD!($start_ip), "\n\t",
            LONG, " ", U32_U64_PAD!($post_commit_offset), "\n\t",
            LONG, " ", U32_U64_PAD!($abort_ip), "\n\t",
            ".word ", stringify!(RSEQ_SIG), "\n\t",
            $label, ":\n\t",
            $teardown,
            "b %l[", $abort_label, "]\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_ABORT {
    ($table_label:expr, $label:expr, $teardown:expr, $abort_label:expr, $start_ip:expr, $post_commit_ip:expr, $abort_ip:expr) => {
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

/* Per-cpu-id indexing. */

/* Template instantiations originally generated by including "rseq-mips-bits.h"
 * with RSEQ_TEMPLATE_CPU_ID and RSEQ_TEMPLATE_MO_RELAXED defined.
 */

/* Template instantiations originally generated by including "rseq-mips-bits.h"
 * with RSEQ_TEMPLATE_CPU_ID and RSEQ_TEMPLATE_MO_RELEASE defined.
 */

/* Per-mm-cid indexing. */

/* Template instantiations originally generated by including "rseq-mips-bits.h"
 * with RSEQ_TEMPLATE_MM_CID and RSEQ_TEMPLATE_MO_RELAXED defined.
 */

/* Template instantiations originally generated by including "rseq-mips-bits.h"
 * with RSEQ_TEMPLATE_MM_CID and RSEQ_TEMPLATE_MO_RELEASE defined.
 */

/* APIs which are not based on cpu ids. */

/* Template instantiations originally generated by including "rseq-mips-bits.h"
 * with RSEQ_TEMPLATE_CPU_ID_NONE and RSEQ_TEMPLATE_MO_RELAXED defined.
 */
