/* SPDX-License-Identifier: GPL-2.0 */
/*
 * User space memory access functions
 *
 * Copyright (C) 1999, 2002  Niibe Yutaka
 * Copyright (C) 2003 - 2008  Paul Mundt
 *
 * Based on the MIPS implementation version 1.15 and the i386 version.
 */

/* C header guard: __ASM_SH_UACCESS_32_H */
/* The following macros preserve the original C macro interfaces. */

#[macro_export]
macro_rules! __get_user_size {
    ($x:expr, $ptr:expr, $size:expr, $retval:expr) => {{
        $retval = 0;
        match $size {
            1 => __get_user_asm!($x, $ptr, $retval, "b"),
            2 => __get_user_asm!($x, $ptr, $retval, "w"),
            4 => __get_user_asm!($x, $ptr, $retval, "l"),
            8 => __get_user_u64!($x, $ptr, $retval),
            _ => unsafe { __get_user_unknown() },
        }
    }};
}

#[cfg(CONFIG_MMU)]
#[macro_export]
macro_rules! __get_user_asm {
    ($x:expr, $addr:expr, $err:expr, $insn:expr) => {{
        /* C inline assembly and exception-table fixup, preserved verbatim. */
        unsafe {
            core::arch::asm!(
                concat!("1:\n\t", "mov.", $insn, "\t%2, %1\n\t", "2:\n",
                    ".section\t.fixup,\"ax\"\n", "3:\n\t",
                    "mov\t#0, %1\n\t", "mov.l\t4f, %0\n\t", "jmp\t@%0\n\t",
                    " mov\t%3, %0\n\t", ".balign\t4\n", "4:\t.long\t2b\n\t",
                    ".previous\n", ".section\t__ex_table,\"a\"\n\t",
                    ".long\t1b, 3b\n\t", ".previous"),
                out(reg) $err, out(reg) $x, in(reg) __m($addr), const -EFAULT,
                in("r0") $err,
            );
        }
    }};
}

#[cfg(not(CONFIG_MMU))]
#[macro_export]
macro_rules! __get_user_asm {
    ($x:expr, $addr:expr, $err:expr, $insn:expr) => {{
        unsafe { core::arch::asm!(concat!("mov.", $insn, "\t%1, %0\n\t"), out(reg) $x, in(reg) __m($addr)); }
    }};
}

extern "C" {
    pub fn __get_user_unknown();
}

#[cfg(CONFIG_CPU_LITTLE_ENDIAN)]
#[macro_export]
macro_rules! __get_user_u64 {
    ($x:expr, $addr:expr, $err:expr) => {{
        unsafe { core::arch::asm!("mov.l\t%2,%R1\n\t mov.l\t%T2,%S1", out(reg) $err, out(reg) $x, in(reg) __m($addr)); }
    }};
}

#[cfg(not(CONFIG_CPU_LITTLE_ENDIAN))]
#[macro_export]
macro_rules! __get_user_u64 {
    ($x:expr, $addr:expr, $err:expr) => {{
        unsafe { core::arch::asm!("mov.l\t%2,%S1\n\t mov.l\t%T2,%R1", out(reg) $err, out(reg) $x, in(reg) __m($addr)); }
    }};
}

#[macro_export]
macro_rules! __put_user_size {
    ($x:expr, $ptr:expr, $size:expr, $retval:expr) => {{
        $retval = 0;
        match $size {
            1 => __put_user_asm!($x, $ptr, $retval, "b"),
            2 => __put_user_asm!($x, $ptr, $retval, "w"),
            4 => __put_user_asm!($x, $ptr, $retval, "l"),
            8 => __put_user_u64!($x, $ptr, $retval),
            _ => unsafe { __put_user_unknown() },
        }
    }};
}

#[cfg(CONFIG_MMU)]
#[macro_export]
macro_rules! __put_user_asm {
    ($x:expr, $addr:expr, $err:expr, $insn:expr) => {{
        unsafe { core::arch::asm!(concat!("mov.", $insn, "\t%1, %2\n\t"), out(reg) $err, in(reg) $x, in(reg) __m($addr), options(nostack)); }
    }};
}

#[cfg(not(CONFIG_MMU))]
#[macro_export]
macro_rules! __put_user_asm {
    ($x:expr, $addr:expr, $err:expr, $insn:expr) => {{
        unsafe { core::arch::asm!(concat!("mov.", $insn, "\t%0, %1\n\t"), in(reg) $x, in(reg) __m($addr), options(nostack)); }
    }};
}

#[cfg(CONFIG_CPU_LITTLE_ENDIAN)]
#[macro_export]
macro_rules! __put_user_u64 {
    ($val:expr, $addr:expr, $retval:expr) => {{ unsafe { core::arch::asm!("mov.l\t%R1,%2\n\t mov.l\t%S1,%T2", out(reg) $retval, in(reg) $val, in(reg) __m($addr)); } }};
}

#[cfg(not(CONFIG_CPU_LITTLE_ENDIAN))]
#[macro_export]
macro_rules! __put_user_u64 {
    ($val:expr, $addr:expr, $retval:expr) => {{ unsafe { core::arch::asm!("mov.l\t%S1,%2\n\t mov.l\t%R1,%T2", out(reg) $retval, in(reg) $val, in(reg) __m($addr)); } }};
}

extern "C" {
    pub fn __put_user_unknown();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
