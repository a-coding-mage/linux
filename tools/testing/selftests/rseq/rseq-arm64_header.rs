/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * rseq-arm64.h
 *
 * (C) Copyright 2016-2022 - Mathieu Desnoyers <mathieu.desnoyers@efficios.com>
 * (C) Copyright 2018 - Will Deacon <will.deacon@arm.com>
 */

/*
 * aarch64 -mbig-endian generates mixed endianness code vs data:
 * little-endian code and big-endian data. Ensure the RSEQ_SIG signature
 * matches code endianness.
 */
pub const RSEQ_SIG_CODE: u32 = 0xd428bc00; /* BRK #0x45E0.  */

#[cfg(target_endian = "big")]
pub const RSEQ_SIG_DATA: u32 = 0x00bc28d4; /* BRK #0x45E0.  */
#[cfg(not(target_endian = "big"))]
pub const RSEQ_SIG_DATA: u32 = RSEQ_SIG_CODE;

pub const RSEQ_SIG: u32 = RSEQ_SIG_DATA;

#[macro_export]
macro_rules! rseq_smp_mb {
    () => {
        core::arch::asm!("dmb ish", options(nostack, preserves_flags))
    };
}

#[macro_export]
macro_rules! rseq_smp_rmb {
    () => {
        core::arch::asm!("dmb ishld", options(nostack, preserves_flags))
    };
}

#[macro_export]
macro_rules! rseq_smp_wmb {
    () => {
        core::arch::asm!("dmb ishst", options(nostack, preserves_flags))
    };
}

#[macro_export]
macro_rules! rseq_smp_load_acquire {
    ($p:expr) => {{
        let __p = $p;
        match core::mem::size_of_val(&*__p) {
            1 => {
                let mut __val: u8;
                core::arch::asm!(
                    "ldarb {0:w}, [{1}]",
                    lateout(reg) __val,
                    in(reg) __p,
                    options(nostack, preserves_flags)
                );
                __val as _
            }
            2 => {
                let mut __val: u16;
                core::arch::asm!(
                    "ldarh {0:w}, [{1}]",
                    lateout(reg) __val,
                    in(reg) __p,
                    options(nostack, preserves_flags)
                );
                __val as _
            }
            4 => {
                let mut __val: u32;
                core::arch::asm!(
                    "ldar {0:w}, [{1}]",
                    lateout(reg) __val,
                    in(reg) __p,
                    options(nostack, preserves_flags)
                );
                __val as _
            }
            8 => {
                let mut __val: u64;
                core::arch::asm!(
                    "ldar {0}, [{1}]",
                    lateout(reg) __val,
                    in(reg) __p,
                    options(nostack, preserves_flags)
                );
                __val as _
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }};
}

#[macro_export]
macro_rules! rseq_smp_acquire__after_ctrl_dep {
    () => {
        rseq_smp_rmb!()
    };
}

#[macro_export]
macro_rules! rseq_smp_store_release {
    ($p:expr, $v:expr) => {{
        let __p = $p;
        match core::mem::size_of_val(&*__p) {
            1 => {
                let __val = $v as u8;
                core::arch::asm!(
                    "stlrb {1:w}, [{0}]",
                    in(reg) __p,
                    in(reg) __val,
                    options(nostack, preserves_flags)
                );
            }
            2 => {
                let __val = $v as u16;
                core::arch::asm!(
                    "stlrh {1:w}, [{0}]",
                    in(reg) __p,
                    in(reg) __val,
                    options(nostack, preserves_flags)
                );
            }
            4 => {
                let __val = $v as u32;
                core::arch::asm!(
                    "stlr {1:w}, [{0}]",
                    in(reg) __p,
                    in(reg) __val,
                    options(nostack, preserves_flags)
                );
            }
            8 => {
                let __val = $v as u64;
                core::arch::asm!(
                    "stlr {1}, [{0}]",
                    in(reg) __p,
                    in(reg) __val,
                    options(nostack, preserves_flags)
                );
            }
            _ => core::hint::unreachable_unchecked(),
        }
    }};
}

pub const RSEQ_ASM_TMP_REG32: &str = "w15";
pub const RSEQ_ASM_TMP_REG: &str = "x15";
pub const RSEQ_ASM_TMP_REG_2: &str = "x14";

#[macro_export]
macro_rules! __RSEQ_ASM_DEFINE_TABLE {
    ($label:expr, $version:expr, $flags:expr, $start_ip:expr, $post_commit_offset:expr, $abort_ip:expr) => {
        concat!(
            "	.pushsection	__rseq_cs, \"aw\"\n",
            "	.balign	32\n",
            stringify!($label),
            ":\n",
            "	.long	",
            stringify!($version),
            ", ",
            stringify!($flags),
            "\n",
            "	.quad	",
            stringify!($start_ip),
            ", ",
            stringify!($post_commit_offset),
            ", ",
            stringify!($abort_ip),
            "\n",
            "	.popsection\n\t",
            "	.pushsection __rseq_cs_ptr_array, \"aw\"\n",
            "	.quad ",
            stringify!($label),
            "b\n",
            "	.popsection\n"
        )
    };
}

#[macro_export]
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

/*
 * Exit points of a rseq critical section consist of all instructions outside
 * of the critical section where a critical section can either branch to or
 * reach through the normal course of its execution. The abort IP and the
 * post-commit IP are already part of the __rseq_cs section and should not be
 * explicitly defined as additional exit points. Knowing all exit points is
 * useful to assist debuggers stepping over the critical section.
 */
#[macro_export]
macro_rules! RSEQ_ASM_DEFINE_EXIT_POINT {
    ($start_ip:expr, $exit_ip:expr) => {
        concat!(
            "	.pushsection __rseq_exit_point_array, \"aw\"\n",
            "	.quad ",
            stringify!($start_ip),
            ", ",
            stringify!($exit_ip),
            "\n",
            "	.popsection\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_STORE_RSEQ_CS {
    ($label:expr, $cs_label:expr, $rseq_cs:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(1),
            "	adrp	",
            RSEQ_ASM_TMP_REG,
            ", ",
            stringify!($cs_label),
            "\n",
            "	add	",
            RSEQ_ASM_TMP_REG,
            ", ",
            RSEQ_ASM_TMP_REG,
            ", :lo12:",
            stringify!($cs_label),
            "\n",
            "	str	",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($rseq_cs),
            "]\n",
            stringify!($label),
            ":\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_DEFINE_ABORT {
    ($label:expr, $abort_label:expr) => {
        concat!(
            "	b	222f\n",
            "	.inst 	",
            stringify!(RSEQ_SIG_CODE),
            "\n",
            stringify!($label),
            ":\n",
            "	b	%l[",
            stringify!($abort_label),
            "]\n",
            "222:\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_STORE {
    ($value:expr, $var:expr) => {
        concat!("	str	%[", stringify!($value), "], %[", stringify!($var), "]\n")
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_STORE_RELEASE {
    ($value:expr, $var:expr) => {
        concat!("	stlr	%[", stringify!($value), "], %[", stringify!($var), "]\n")
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_FINAL_STORE {
    ($value:expr, $var:expr, $post_commit_label:expr) => {
        concat!(
            RSEQ_ASM_OP_STORE!($value, $var),
            stringify!($post_commit_label),
            ":\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_FINAL_STORE_RELEASE {
    ($value:expr, $var:expr, $post_commit_label:expr) => {
        concat!(
            RSEQ_ASM_OP_STORE_RELEASE!($value, $var),
            stringify!($post_commit_label),
            ":\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_CMPEQ {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            "	ldr	",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($var),
            "]\n",
            "	sub	",
            RSEQ_ASM_TMP_REG,
            ", ",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($expect),
            "]\n",
            "	cbnz	",
            RSEQ_ASM_TMP_REG,
            ", ",
            stringify!($label),
            "\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_CMPEQ32 {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            "	ldr	",
            RSEQ_ASM_TMP_REG32,
            ", %[",
            stringify!($var),
            "]\n",
            "	sub	",
            RSEQ_ASM_TMP_REG32,
            ", ",
            RSEQ_ASM_TMP_REG32,
            ", %w[",
            stringify!($expect),
            "]\n",
            "	cbnz	",
            RSEQ_ASM_TMP_REG32,
            ", ",
            stringify!($label),
            "\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_CMPNE {
    ($var:expr, $expect:expr, $label:expr) => {
        concat!(
            "	ldr	",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($var),
            "]\n",
            "	sub	",
            RSEQ_ASM_TMP_REG,
            ", ",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($expect),
            "]\n",
            "	cbz	",
            RSEQ_ASM_TMP_REG,
            ", ",
            stringify!($label),
            "\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_CMP_CPU_ID {
    ($cpu_id:expr, $current_cpu_id:expr, $label:expr) => {
        concat!(
            RSEQ_INJECT_ASM!(2),
            RSEQ_ASM_OP_CMPEQ32!($current_cpu_id, $cpu_id, $label)
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_R_LOAD {
    ($var:expr) => {
        concat!("	ldr	", RSEQ_ASM_TMP_REG, ", %[", stringify!($var), "]\n")
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_R_STORE {
    ($var:expr) => {
        concat!("	str	", RSEQ_ASM_TMP_REG, ", %[", stringify!($var), "]\n")
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_R_LOAD_OFF {
    ($offset:expr) => {
        concat!(
            "	ldr	",
            RSEQ_ASM_TMP_REG,
            ", [",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($offset),
            "]]\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_R_ADD {
    ($count:expr) => {
        concat!(
            "	add	",
            RSEQ_ASM_TMP_REG,
            ", ",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($count),
            "]\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_R_FINAL_STORE {
    ($var:expr, $post_commit_label:expr) => {
        concat!(
            "	str	",
            RSEQ_ASM_TMP_REG,
            ", %[",
            stringify!($var),
            "]\n",
            stringify!($post_commit_label),
            ":\n"
        )
    };
}

#[macro_export]
macro_rules! RSEQ_ASM_OP_R_BAD_MEMCPY {
    ($dst:expr, $src:expr, $len:expr) => {
        concat!(
            "	cbz	%[",
            stringify!($len),
            "], 333f\n",
            "	mov	",
            RSEQ_ASM_TMP_REG_2,
            ", %[",
            stringify!($len),
            "]\n",
            "222:	sub	",
            RSEQ_ASM_TMP_REG_2,
            ", ",
            RSEQ_ASM_TMP_REG_2,
            ", #1\n",
            "	ldrb	",
            RSEQ_ASM_TMP_REG32,
            ", [%[",
            stringify!($src),
            "], ",
            RSEQ_ASM_TMP_REG_2,
            "]\n",
            "	strb	",
            RSEQ_ASM_TMP_REG32,
            ", [%[",
            stringify!($dst),
            "], ",
            RSEQ_ASM_TMP_REG_2,
            "]\n",
            "	cbnz	",
            RSEQ_ASM_TMP_REG_2,
            ", 222b\n",
            "333:\n"
        )
    };
}

/* Per-cpu-id indexing. */

/* Defines RSEQ_TEMPLATE_CPU_ID and RSEQ_TEMPLATE_MO_RELAXED before including
 * rseq-arm64-bits.h, then undefines RSEQ_TEMPLATE_MO_RELAXED.
 */

/* Defines RSEQ_TEMPLATE_MO_RELEASE before including rseq-arm64-bits.h, then
 * undefines RSEQ_TEMPLATE_MO_RELEASE and RSEQ_TEMPLATE_CPU_ID.
 */

/* Per-mm-cid indexing. */

/* Defines RSEQ_TEMPLATE_MM_CID and RSEQ_TEMPLATE_MO_RELAXED before including
 * rseq-arm64-bits.h, then undefines RSEQ_TEMPLATE_MO_RELAXED.
 */

/* Defines RSEQ_TEMPLATE_MO_RELEASE before including rseq-arm64-bits.h, then
 * undefines RSEQ_TEMPLATE_MO_RELEASE and RSEQ_TEMPLATE_MM_CID.
 */

/* APIs which are not based on cpu ids. */

/* Defines RSEQ_TEMPLATE_CPU_ID_NONE and RSEQ_TEMPLATE_MO_RELAXED before
 * including rseq-arm64-bits.h, then undefines RSEQ_TEMPLATE_MO_RELAXED and
 * RSEQ_TEMPLATE_CPU_ID_NONE.
 */
