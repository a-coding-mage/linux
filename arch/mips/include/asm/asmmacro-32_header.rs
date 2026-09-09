/* SPDX-License-Identifier: GPL-2.0 */
/*
 * asmmacro.h: Assembler macros to make things easier to read.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1998, 1999, 2003 Ralf Baechle
 */

// Dependencies supplied by the surrounding MIPS translation:
// asm/asm-offsets.h, asm/regdef.h, asm/fpregdef.h, and asm/mipsregs.h.

/// Save the single-precision FPU register state for `thread`.
#[macro_export]
macro_rules! fpu_save_single {
    ($thread:ident $( $tmp:ident )?) => {
        unsafe {
            core::arch::asm!(
                "cfc1 {tmp}, fcr31",
                "s.d $f0,  THREAD_FPR0({thread})",
                "s.d $f2,  THREAD_FPR2({thread})",
                "s.d $f4,  THREAD_FPR4({thread})",
                "s.d $f6,  THREAD_FPR6({thread})",
                "s.d $f8,  THREAD_FPR8({thread})",
                "s.d $f10, THREAD_FPR10({thread})",
                "s.d $f12, THREAD_FPR12({thread})",
                "s.d $f14, THREAD_FPR14({thread})",
                "s.d $f16, THREAD_FPR16({thread})",
                "s.d $f18, THREAD_FPR18({thread})",
                "s.d $f20, THREAD_FPR20({thread})",
                "s.d $f22, THREAD_FPR22({thread})",
                "s.d $f24, THREAD_FPR24({thread})",
                "s.d $f26, THREAD_FPR26({thread})",
                "s.d $f28, THREAD_FPR28({thread})",
                "s.d $f30, THREAD_FPR30({thread})",
                "sw {tmp}, THREAD_FCR31({thread})",
                tmp = const stringify!($($tmp)?),
                thread = const stringify!($thread),
                options(nostack)
            );
        }
    };
}

/// Restore the single-precision FPU register state for `thread`.
#[macro_export]
macro_rules! fpu_restore_single {
    ($thread:ident $( $tmp:ident )?) => {
        unsafe {
            core::arch::asm!(
                "lw {tmp}, THREAD_FCR31({thread})",
                "l.d $f0,  THREAD_FPR0({thread})",
                "l.d $f2,  THREAD_FPR2({thread})",
                "l.d $f4,  THREAD_FPR4({thread})",
                "l.d $f6,  THREAD_FPR6({thread})",
                "l.d $f8,  THREAD_FPR8({thread})",
                "l.d $f10, THREAD_FPR10({thread})",
                "l.d $f12, THREAD_FPR12({thread})",
                "l.d $f14, THREAD_FPR14({thread})",
                "l.d $f16, THREAD_FPR16({thread})",
                "l.d $f18, THREAD_FPR18({thread})",
                "l.d $f20, THREAD_FPR20({thread})",
                "l.d $f22, THREAD_FPR22({thread})",
                "l.d $f24, THREAD_FPR24({thread})",
                "l.d $f26, THREAD_FPR26({thread})",
                "l.d $f28, THREAD_FPR28({thread})",
                "l.d $f30, THREAD_FPR30({thread})",
                "ctc1 {tmp}, fcr31",
                tmp = const stringify!($($tmp)?),
                thread = const stringify!($thread),
                options(nostack)
            );
        }
    };
}

/// Save the non-scratch CPU registers for `thread`.
#[macro_export]
macro_rules! cpu_save_nonscratch {
    ($thread:ident) => {
        unsafe {
            core::arch::asm!(
                "LONG_S s0, THREAD_REG16({thread})", "LONG_S s1, THREAD_REG17({thread})",
                "LONG_S s2, THREAD_REG18({thread})", "LONG_S s3, THREAD_REG19({thread})",
                "LONG_S s4, THREAD_REG20({thread})", "LONG_S s5, THREAD_REG21({thread})",
                "LONG_S s6, THREAD_REG22({thread})", "LONG_S s7, THREAD_REG23({thread})",
                "LONG_S sp, THREAD_REG29({thread})", "LONG_S fp, THREAD_REG30({thread})",
                thread = const stringify!($thread), options(nostack)
            );
        }
    };
}

/// Restore the non-scratch CPU registers for `thread`.
#[macro_export]
macro_rules! cpu_restore_nonscratch {
    ($thread:ident) => {
        unsafe {
            core::arch::asm!(
                "LONG_L s0, THREAD_REG16({thread})", "LONG_L s1, THREAD_REG17({thread})",
                "LONG_L s2, THREAD_REG18({thread})", "LONG_L s3, THREAD_REG19({thread})",
                "LONG_L s4, THREAD_REG20({thread})", "LONG_L s5, THREAD_REG21({thread})",
                "LONG_L s6, THREAD_REG22({thread})", "LONG_L s7, THREAD_REG23({thread})",
                "LONG_L sp, THREAD_REG29({thread})", "LONG_L fp, THREAD_REG30({thread})",
                "LONG_L ra, THREAD_REG31({thread})",
                thread = const stringify!($thread), options(nostack)
            );
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
