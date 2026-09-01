/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Select the instruction "csrw mhartid, x0" as the RSEQ_SIG. Unlike
 * other architectures, the ebreak instruction has no immediate field for
 * distinguishing purposes. Hence, ebreak is not suitable as RSEQ_SIG.
 * "csrw mhartid, x0" can also satisfy the RSEQ requirement because it
 * is an uncommon instruction and will raise an illegal instruction
 * exception when executed in all modes.
 */

/* C header dependencies translated as dependency intent:
 * #include <endian.h>
 * #include <asm/fence.h>
 */

/* The original C header only supports little-endian builds. */
#[cfg(target_endian = "little")]
pub const RSEQ_SIG: u32 = 0xf1401073; /* csrr mhartid, x0 */

#[cfg(not(target_endian = "little"))]
compile_error!("Currently, RSEQ only supports Little-Endian version");

#[cfg(target_pointer_width = "64")]
pub const REG_L: &str = "ld ";
#[cfg(target_pointer_width = "64")]
pub const REG_S: &str = "sd ";

#[cfg(target_pointer_width = "32")]
pub const REG_L: &str = "lw ";
#[cfg(target_pointer_width = "32")]
pub const REG_S: &str = "sw ";

macro_rules! rseq_smp_mb {
    () => {
        RISCV_FENCE!(rw, rw)
    };
}

macro_rules! rseq_smp_rmb {
    () => {
        RISCV_FENCE!(r, r)
    };
}

macro_rules! rseq_smp_wmb {
    () => {
        RISCV_FENCE!(w, w)
    };
}

pub const RSEQ_ASM_TMP_REG_1: &str = "t6";
pub const RSEQ_ASM_TMP_REG_2: &str = "t5";
pub const RSEQ_ASM_TMP_REG_3: &str = "t4";
pub const RSEQ_ASM_TMP_REG_4: &str = "t3";

macro_rules! rseq_smp_load_acquire {
    ($p:expr) => {{
        let ____p1 = RSEQ_READ_ONCE!(unsafe { *$p });
        RISCV_FENCE!(r, rw);
        ____p1
    }};
}

macro_rules! rseq_smp_acquire__after_ctrl_dep {
    () => {
        rseq_smp_rmb!()
    };
}

macro_rules! rseq_smp_store_release {
    ($p:expr, $v:expr) => {{
        RISCV_FENCE!(rw, w);
        RSEQ_WRITE_ONCE!(unsafe { *$p }, $v);
    }};
}

macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            ".pushsection\t__rseq_cs, \"aw\"\n",
            ".balign\t32\n",
            stringify!($label),
            ":\n",
            ".long\t",
            stringify!($version),
            ", ",
            stringify!($flags),
            "\n",
            ".quad\t",
            stringify!($start_ip),
            ", ",
            stringify!($post_commit_offset),
            ", ",
            stringify!($abort_ip),
            "\n",
            ".popsection\n\t",
            ".pushsection __rseq_cs_ptr_array, \"aw\"\n",
            ".quad ",
            stringify!($label),
            "b\n",
            ".popsection\n"
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
            (($post_commit_ip) - ($start_ip)),
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
            ".quad ",
            stringify!($start_ip),
            ", ",
            stringify!($exit_ip),
            "\n",
            ".popsection\n"
        )
    };
}

macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            "la\t",
            RSEQ_ASM_TMP_REG_1,
            ", ",
            stringify!($cs_label),
            "\n",
            REG_S,
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($rseq_cs),
            "]\n",
            stringify!($label),
            ":\n"
        )
    };
}

macro_rules! RSEQ_ASM_DEFINE_ABORT {
    ($label:expr, $abort_label:expr) => {
        concat!(
            "j\t222f\n",
            ".balign\t4\n",
            ".long ",
            stringify!(RSEQ_SIG),
            "\n",
            stringify!($label),
            ":\n",
            "j\t%l[",
            stringify!($abort_label),
            "]\n",
            "222:\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_STORE {
    ($value:expr, $var:expr) => {
        concat!(REG_S, "%[", stringify!($value), "], %[", stringify!($var), "]\n")
    };
}

macro_rules! RSEQ_ASM_OP_CMPEQ {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            REG_L,
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($var),
            "]\n",
            "bne\t",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($expect),
            "] ,",
            stringify!($label),
            "\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_CMPEQ32 {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            "lw\t",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($var),
            "]\n",
            "bne\t",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($expect),
            "] ,",
            stringify!($label),
            "\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_CMPNE {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            REG_L,
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($var),
            "]\n",
            "beq\t",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($expect),
            "] ,",
            stringify!($label),
            "\n"
        )
    };
}

macro_rules! RSEQ_ASM_CMP_CPU_ID {
    ($cpu_id:expr, $current_cpu_id:expr, $label:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(2),
            RSEQ_ASM_OP_CMPEQ32!($current_cpu_id, $cpu_id, $label)
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_LOAD {
    ($var:expr) => {
        concat!(REG_L, RSEQ_ASM_TMP_REG_1, ", %[", stringify!($var), "]\n")
    };
}

macro_rules! RSEQ_ASM_OP_R_STORE {
    ($var:expr) => {
        concat!(REG_S, RSEQ_ASM_TMP_REG_1, ", %[", stringify!($var), "]\n")
    };
}

macro_rules! RSEQ_ASM_OP_R_LOAD_OFF {
    ($offset:expr) => {
        concat!(
            "add\t",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($offset),
            "], ",
            RSEQ_ASM_TMP_REG_1,
            "\n",
            REG_L,
            RSEQ_ASM_TMP_REG_1,
            ", (",
            RSEQ_ASM_TMP_REG_1,
            ")\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_ADD {
    ($count:expr) => {
        concat!(
            "add\t",
            RSEQ_ASM_TMP_REG_1,
            ", ",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($count),
            "]\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_FINAL_STORE {
    ($value:expr, $var:expr, $post_commit_label:expr) => {
        concat!(
            RSEQ_ASM_OP_STORE!($value, $var),
            stringify!($post_commit_label),
            ":\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_FINAL_STORE_RELEASE {
    ($value:expr, $var:expr, $post_commit_label:expr) => {
        concat!(
            "fence\trw, w\n",
            RSEQ_ASM_OP_STORE!($value, $var),
            stringify!($post_commit_label),
            ":\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_FINAL_STORE {
    ($var:expr, $post_commit_label:expr) => {
        concat!(
            REG_S,
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($var),
            "]\n",
            stringify!($post_commit_label),
            ":\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_BAD_MEMCPY {
    ($dst:expr, $src:expr, $len:expr) => {
        concat!(
            "beqz\t%[",
            stringify!($len),
            "], 333f\n",
            "mv\t",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($len),
            "]\n",
            "mv\t",
            RSEQ_ASM_TMP_REG_2,
            ", %[",
            stringify!($src),
            "]\n",
            "mv\t",
            RSEQ_ASM_TMP_REG_3,
            ", %[",
            stringify!($dst),
            "]\n",
            "222:\n",
            "lb\t",
            RSEQ_ASM_TMP_REG_4,
            ", 0(",
            RSEQ_ASM_TMP_REG_2,
            ")\n",
            "sb\t",
            RSEQ_ASM_TMP_REG_4,
            ", 0(",
            RSEQ_ASM_TMP_REG_3,
            ")\n",
            "addi\t",
            RSEQ_ASM_TMP_REG_1,
            ", ",
            RSEQ_ASM_TMP_REG_1,
            ", -1\n",
            "addi\t",
            RSEQ_ASM_TMP_REG_2,
            ", ",
            RSEQ_ASM_TMP_REG_2,
            ", 1\n",
            "addi\t",
            RSEQ_ASM_TMP_REG_3,
            ", ",
            RSEQ_ASM_TMP_REG_3,
            ", 1\n",
            "bnez\t",
            RSEQ_ASM_TMP_REG_1,
            ", 222b\n",
            "333:\n"
        )
    };
}

macro_rules! RSEQ_ASM_OP_R_DEREF_ADDV {
    ($ptr:expr, $off:expr, $inc:expr, $post_commit_label:expr) => {
        concat!(
            "mv\t",
            RSEQ_ASM_TMP_REG_1,
            ", %[",
            stringify!($ptr),
            "]\n",
            RSEQ_ASM_OP_R_ADD!($off),
            REG_L,
            RSEQ_ASM_TMP_REG_1,
            ", 0(",
            RSEQ_ASM_TMP_REG_1,
            ")\n",
            RSEQ_ASM_OP_R_ADD!($inc),
            stringify!($post_commit_label),
            ":\n"
        )
    };
}

/* Per-cpu-id indexing. */

/* Original template expansion intent:
 * #define RSEQ_TEMPLATE_CPU_ID
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-riscv-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 * #define RSEQ_TEMPLATE_MO_RELEASE
 * #include "rseq-riscv-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELEASE
 * #undef RSEQ_TEMPLATE_CPU_ID
 */

/* Per-mm-cid indexing. */

/* Original template expansion intent:
 * #define RSEQ_TEMPLATE_MM_CID
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-riscv-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 *
 * #define RSEQ_TEMPLATE_MO_RELEASE
 * #include "rseq-riscv-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELEASE
 * #undef RSEQ_TEMPLATE_MM_CID
 */

/* APIs which are not based on cpu ids. */

/* Original template expansion intent:
 * #define RSEQ_TEMPLATE_CPU_ID_NONE
 * #define RSEQ_TEMPLATE_MO_RELAXED
 * #include "rseq-riscv-bits.h"
 * #undef RSEQ_TEMPLATE_MO_RELAXED
 * #undef RSEQ_TEMPLATE_CPU_ID_NONE
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
