/* SPDX-License-Identifier: LGPL-2.1 OR MIT */

/*
 * Select the instruction "l.nop 0x35" as the RSEQ_SIG.
 */
pub const RSEQ_SIG: u32 = 0x15000035;

#[inline(always)]
pub unsafe fn rseq_smp_mb() {
    core::arch::asm!("l.msync", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn rseq_smp_rmb() {
    unsafe { rseq_smp_mb() };
}

#[inline(always)]
pub unsafe fn rseq_smp_wmb() {
    unsafe { rseq_smp_mb() };
}

pub const RSEQ_ASM_TMP_REG_1: &str = "r31";
pub const RSEQ_ASM_TMP_REG_2: &str = "r29";
pub const RSEQ_ASM_TMP_REG_3: &str = "r27";
pub const RSEQ_ASM_TMP_REG_4: &str = "r25";

macro_rules! rseq_smp_load_acquire {
    ($p:expr) => {{
        let ____p1 = unsafe { RSEQ_READ_ONCE(*($p)) };
        unsafe { rseq_smp_mb() };
        ____p1
    }};
}

macro_rules! rseq_smp_acquire__after_ctrl_dep {
    () => {
        unsafe { rseq_smp_rmb() }
    };
}

macro_rules! rseq_smp_store_release {
    ($p:expr, $v:expr) => {{
        unsafe { rseq_smp_mb() };
        unsafe { RSEQ_WRITE_ONCE(*($p), $v) };
    }};
}

macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr,
     $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".pushsection\t__rseq_cs, \"aw\"\n",
            ".balign\t32\n",
            $label, ":\n",
            ".long ", $version, ", ", $flags, "\n",
            ".long 0x0, ", $start_ip, ", ",
            "0x0, ", $post_commit_offset, ", ",
            "0x0, ", $abort_ip, "\n",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n",
            ".long 0x0, ", $label, "b\n",
            ".popsection\n",
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
            concat!("((", $post_commit_ip, ") - (", $start_ip, "))"),
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
            ".pushsection __rseq_exit_point_array, \"aw\"\n",
            ".long 0x0, ", $start_ip, ", 0x0, ", $exit_ip, "\n",
            ".popsection\n",
        )
    };
}

macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            "l.movhi ", RSEQ_ASM_TMP_REG_1, ", hi(", $cs_label, ")\n",
            "l.ori   ", RSEQ_ASM_TMP_REG_1, ", ", RSEQ_ASM_TMP_REG_1,
            ", lo(", $cs_label, ")\n",
            "l.sw  %[", $rseq_cs, "], ", RSEQ_ASM_TMP_REG_1, "\n",
            $label, ":\n",
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_ABORT {
    ($label:expr, $abort_label:expr) => {
        concat!(
            "l.j 222f\n",
            " l.nop\n",
            ".balign\t4\n",
            ".long ", stringify!(RSEQ_SIG), "\n",
            $label, ":\n",
            "l.j %l[", $abort_label, "]\n",
            " l.nop\n",
            "222:\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_STORE {
    ($var:expr, $value:expr) => {
        concat!("l.sw %[", $var, "], %[", $value, "]\n")
    };
}

macro_rules! RSEQ_ASM_OP_CMPEQ {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            "l.lwz  ", RSEQ_ASM_TMP_REG_1, ", %[", $var, "]\n",
            "l.sfne ", RSEQ_ASM_TMP_REG_1, ", %[", $expect, "]\n",
            "l.bf   ", $label, "\n",
            " l.nop\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_CMPNE {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            "l.lwz  ", RSEQ_ASM_TMP_REG_1, ", %[", $var, "]\n",
            "l.sfeq ", RSEQ_ASM_TMP_REG_1, ", %[", $expect, "]\n",
            "l.bf   ", $label, "\n",
            " l.nop\n",
        )
    };
}

macro_rules! RSEQ_ASM_CMP_CPU_ID {
    ($cpu_id:expr, $current_cpu_id:expr, $label:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(2),
            RSEQ_ASM_OP_CMPEQ!($current_cpu_id, $cpu_id, $label),
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_LOAD {
    ($var:expr) => {
        concat!("l.lwz ", RSEQ_ASM_TMP_REG_1, ", %[", $var, "]\n")
    };
}

macro_rules! RSEQ_ASM_OP_R_STORE {
    ($var:expr) => {
        concat!("l.sw %[", $var, "], ", RSEQ_ASM_TMP_REG_1, "\n")
    };
}

macro_rules! RSEQ_ASM_OP_R_LOAD_OFF {
    ($offset:expr) => {
        concat!(
            "l.lwz ", RSEQ_ASM_TMP_REG_1, ", ",
            "%[", $offset, "](", RSEQ_ASM_TMP_REG_1, ")\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_ADD {
    ($count:expr) => {
        concat!(
            "l.add ", RSEQ_ASM_TMP_REG_1, ", ", RSEQ_ASM_TMP_REG_1,
            ", %[", $count, "]\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_FINAL_STORE {
    ($var:expr, $value:expr, $post_commit_label:expr) => {
        concat!(
            RSEQ_ASM_OP_STORE!($var, $value),
            $post_commit_label, ":\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_FINAL_STORE_RELEASE {
    ($var:expr, $value:expr, $post_commit_label:expr) => {
        concat!(
            "l.msync\n",
            RSEQ_ASM_OP_STORE!($var, $value),
            $post_commit_label, ":\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_FINAL_STORE {
    ($var:expr, $post_commit_label:expr) => {
        concat!(
            "l.sw %[", $var, "], ", RSEQ_ASM_TMP_REG_1, "\n",
            $post_commit_label, ":\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_BAD_MEMCPY {
    ($dst:expr, $src:expr, $len:expr) => {
        concat!(
            "l.sfeq\t%[", $len, "], r0\n",
            "l.bf 333f\n",
            " l.nop\n",
            "l.ori  ", RSEQ_ASM_TMP_REG_1, ", %[", $len, "], 0\n",
            "l.ori  ", RSEQ_ASM_TMP_REG_2, ", %[", $src, "], 0\n",
            "l.ori  ", RSEQ_ASM_TMP_REG_3, ", %[", $dst, "], 0\n",
            "222:\n",
            "l.lbz  ", RSEQ_ASM_TMP_REG_4, ", 0(", RSEQ_ASM_TMP_REG_2, ")\n",
            "l.sb   0(", RSEQ_ASM_TMP_REG_3, "), ", RSEQ_ASM_TMP_REG_4, "\n",
            "l.addi ", RSEQ_ASM_TMP_REG_1, ", ", RSEQ_ASM_TMP_REG_1, ", -1\n",
            "l.addi ", RSEQ_ASM_TMP_REG_2, ", ", RSEQ_ASM_TMP_REG_2, ", 1\n",
            "l.addi ", RSEQ_ASM_TMP_REG_3, ", ", RSEQ_ASM_TMP_REG_3, ", 1\n",
            "l.sfne ", RSEQ_ASM_TMP_REG_1, ", r0\n",
            "l.bf 222b\n",
            " l.nop\n",
            "333:\n",
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_DEREF_ADDV {
    ($ptr:expr, $off:expr, $inc:expr, $post_commit_label:expr) => {
        concat!(
            "l.ori  ", RSEQ_ASM_TMP_REG_1, ", %[", $ptr, "], 0\n",
            RSEQ_ASM_OP_R_ADD!($off),
            "l.lwz  ", RSEQ_ASM_TMP_REG_1, ", 0(", RSEQ_ASM_TMP_REG_1, ")\n",
            RSEQ_ASM_OP_R_ADD!($inc),
            $post_commit_label, ":\n",
        )
    };
}

/* Per-cpu-id indexing. */

/* RSEQ_TEMPLATE_CPU_ID + RSEQ_TEMPLATE_MO_RELAXED included "rseq-or1k-bits.h" here. */
/* RSEQ_TEMPLATE_CPU_ID + RSEQ_TEMPLATE_MO_RELEASE included "rseq-or1k-bits.h" here. */

/* Per-mm-cid indexing. */

/* RSEQ_TEMPLATE_MM_CID + RSEQ_TEMPLATE_MO_RELAXED included "rseq-or1k-bits.h" here. */
/* RSEQ_TEMPLATE_MM_CID + RSEQ_TEMPLATE_MO_RELEASE included "rseq-or1k-bits.h" here. */

/* APIs which are not based on cpu ids. */

/* RSEQ_TEMPLATE_CPU_ID_NONE + RSEQ_TEMPLATE_MO_RELAXED included "rseq-or1k-bits.h" here. */
