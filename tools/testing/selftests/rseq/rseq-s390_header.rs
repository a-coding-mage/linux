/* SPDX-License-Identifier: LGPL-2.1 OR MIT */

/*
 * RSEQ_SIG uses the trap4 instruction. As Linux does not make use of the
 * access-register mode nor the linkage stack this instruction will always
 * cause a special-operation exception (the trap-enabled bit in the DUCT
 * is and will stay 0). The instruction pattern is
 *	b2 ff 0f ff	trap4	4095(%r0)
 */
pub const RSEQ_SIG: u32 = 0xB2FF0FFF;

#[inline(always)]
pub unsafe fn rseq_smp_mb() {
    unsafe {
        core::arch::asm!("bcr 15,0", options(nostack, preserves_flags));
    }
}

#[inline(always)]
pub unsafe fn rseq_smp_rmb() {
    unsafe {
        rseq_smp_mb();
    }
}

#[inline(always)]
pub unsafe fn rseq_smp_wmb() {
    unsafe {
        rseq_smp_mb();
    }
}

#[inline(always)]
pub unsafe fn rseq_smp_load_acquire<T: Copy>(p: *const T) -> T {
    let ____p1 = unsafe { RSEQ_READ_ONCE(p) };
    unsafe {
        rseq_barrier();
    }
    ____p1
}

#[inline(always)]
pub unsafe fn rseq_smp_acquire__after_ctrl_dep() {
    unsafe {
        rseq_smp_rmb();
    }
}

#[inline(always)]
pub unsafe fn rseq_smp_store_release<T>(p: *mut T, v: T) {
    unsafe {
        rseq_barrier();
        RSEQ_WRITE_ONCE(p, v);
    }
}

pub const LONG_L: &str = "lg";
pub const LONG_S: &str = "stg";
pub const LONG_LT_R: &str = "ltgr";
pub const LONG_CMP: &str = "cg";
pub const LONG_CMP_R: &str = "cgr";
pub const LONG_ADDI: &str = "aghi";
pub const LONG_ADD_R: &str = "agr";

macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".pushsection __rseq_cs, \"aw\"\n\t",
            ".balign 32\n\t",
            stringify!($label),
            ":\n\t",
            ".long ",
            stringify!($version),
            ", ",
            stringify!($flags),
            "\n\t",
            ".quad ",
            stringify!($start_ip),
            ", ",
            stringify!($post_commit_offset),
            ", ",
            stringify!($abort_ip),
            "\n\t",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n\t",
            ".quad ",
            stringify!($label),
            "b\n\t",
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
macro_rules! RSEQ_ASM_DEFINE_EXIT_POINT {
    ($start_ip:expr, $exit_ip:expr) => {
        concat!(
            ".pushsection __rseq_exit_point_array, \"aw\"\n\t",
            ".quad ",
            stringify!($start_ip),
            ", ",
            stringify!($exit_ip),
            "\n\t",
            ".popsection\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $start_ip:expr, $post_commit_ip:expr, $abort_ip:expr) => {
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

macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            "larl %%r0, ",
            stringify!($cs_label),
            "\n\t",
            LONG_S,
            " %%r0, %[",
            stringify!($rseq_cs),
            "]\n\t",
            stringify!($label),
            ":\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_CMP_CPU_ID {
    ($cpu_id:expr, $current_cpu_id:expr, $label:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(2),
            "c %[",
            stringify!($cpu_id),
            "], %[",
            stringify!($current_cpu_id),
            "]\n\t",
            "jnz ",
            stringify!($label),
            "\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_ABORT {
    ($label:expr, $teardown:expr, $abort_label:expr) => {
        concat!(
            ".pushsection __rseq_failure, \"ax\"\n\t",
            ".long ",
            stringify!(RSEQ_SIG),
            "\n\t",
            stringify!($label),
            ":\n\t",
            $teardown,
            "jg %l[",
            stringify!($abort_label),
            "]\n\t",
            ".popsection\n\t",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_CMPFAIL {
    ($label:expr, $teardown:expr, $cmpfail_label:expr) => {
        concat!(
            ".pushsection __rseq_failure, \"ax\"\n\t",
            stringify!($label),
            ":\n\t",
            $teardown,
            "jg %l[",
            stringify!($cmpfail_label),
            "]\n\t",
            ".popsection\n\t",
        )
    };
}

/* Per-cpu-id indexing. */

/*
 * Original C template inclusion sequence:
 * #define RSEQ_TEMPLATE_CPU_ID
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-s390-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 * #define RSEQ_TEMPLATE_MO_RELEASE
 * #include "rseq-s390-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELEASE
 * #undef RSEQ_TEMPLATE_CPU_ID
 */

/* Per-mm-cid indexing. */

/*
 * Original C template inclusion sequence:
 * #define RSEQ_TEMPLATE_MM_CID
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-s390-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 * #define RSEQ_TEMPLATE_MO_RELEASE
 * #include "rseq-s390-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELEASE
 * #undef RSEQ_TEMPLATE_MM_CID
 */

/* APIs which are not based on cpu ids. */

/*
 * Original C template inclusion sequence:
 * #define RSEQ_TEMPLATE_CPU_ID_NONE
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-s390-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 * #undef RSEQ_TEMPLATE_CPU_ID_NONE
 */
