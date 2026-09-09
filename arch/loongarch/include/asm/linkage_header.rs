/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the LoongArch assembly linkage header.  The assembler
// directives and helper macros referenced here are supplied by other headers.

pub const __ALIGN: &str = ".align 2";
pub const __ALIGN_STR: &str = __ALIGN;

macro_rules! SYM_FUNC_START {
    ($name:ident) => {{ SYM_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN); cfi_startproc!(); }};
}
macro_rules! SYM_FUNC_START_NOALIGN {
    ($name:ident) => {{ SYM_START!($name, SYM_L_GLOBAL, SYM_A_NONE); cfi_startproc!(); }};
}
macro_rules! SYM_FUNC_START_LOCAL {
    ($name:ident) => {{ SYM_START!($name, SYM_L_LOCAL, SYM_A_ALIGN); cfi_startproc!(); }};
}
macro_rules! SYM_FUNC_START_LOCAL_NOALIGN {
    ($name:ident) => {{ SYM_START!($name, SYM_L_LOCAL, SYM_A_NONE); cfi_startproc!(); }};
}
macro_rules! SYM_FUNC_START_WEAK {
    ($name:ident) => {{ SYM_START!($name, SYM_L_WEAK, SYM_A_ALIGN); cfi_startproc!(); }};
}
macro_rules! SYM_FUNC_START_WEAK_NOALIGN {
    ($name:ident) => {{ SYM_START!($name, SYM_L_WEAK, SYM_A_NONE); cfi_startproc!(); }};
}
macro_rules! SYM_FUNC_END {
    ($name:ident) => {{ cfi_endproc!(); SYM_END!($name, SYM_T_FUNC); }};
}
macro_rules! SYM_CODE_START {
    ($name:ident) => {{ SYM_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN); cfi_startproc!(); }};
}
macro_rules! SYM_CODE_END {
    ($name:ident) => {{ cfi_endproc!(); SYM_END!($name, SYM_T_NONE); }};
}

/*
 * This is for the signal handler trampoline, which is used as the return
 * address of the signal handlers in userspace instead of called normally.
 * The long standing libgcc bug https://gcc.gnu.org/PR124050 requires a
 * nop between .cfi_startproc and the actual address of the trampoline, so
 * we cannot simply use SYM_FUNC_START.
 *
 * This wrapper also contains all the .cfi_* directives for recovering
 * the content of the GPRs and the "return address" (where the rt_sigreturn
 * syscall will jump to), assuming there is a struct rt_sigframe (where
 * a struct sigcontext containing those information we need to recover) at
 * $sp.  The "DWARF for the LoongArch(TM) Architecture" manual states
 * column 0 is for $zero, but it does not make too much sense to
 * save/restore the hardware zero register.  Repurpose this column here
 * for the return address (here it's not the content of $ra we cannot use
 * the default column 3).
 */
macro_rules! SYM_SIGFUNC_START {
    ($name:ident) => {{
        cfi_startproc!();
        cfi_signal_frame!();
        cfi_def_cfa!(3, RT_SIGFRAME_SC);
        cfi_return_column!(0);
        cfi_offset!(0, SC_PC);
        cfi_offset!(1, SC_REGS + 1 * 8);
        cfi_offset!(2, SC_REGS + 2 * 8);
        cfi_offset!(3, SC_REGS + 3 * 8);
        cfi_offset!(4, SC_REGS + 4 * 8);
        cfi_offset!(5, SC_REGS + 5 * 8);
        cfi_offset!(6, SC_REGS + 6 * 8);
        cfi_offset!(7, SC_REGS + 7 * 8);
        cfi_offset!(8, SC_REGS + 8 * 8);
        cfi_offset!(9, SC_REGS + 9 * 8);
        cfi_offset!(10, SC_REGS + 10 * 8);
        cfi_offset!(11, SC_REGS + 11 * 8);
        cfi_offset!(12, SC_REGS + 12 * 8);
        cfi_offset!(13, SC_REGS + 13 * 8);
        cfi_offset!(14, SC_REGS + 14 * 8);
        cfi_offset!(15, SC_REGS + 15 * 8);
        cfi_offset!(16, SC_REGS + 16 * 8);
        cfi_offset!(17, SC_REGS + 17 * 8);
        cfi_offset!(18, SC_REGS + 18 * 8);
        cfi_offset!(19, SC_REGS + 19 * 8);
        cfi_offset!(20, SC_REGS + 20 * 8);
        cfi_offset!(21, SC_REGS + 21 * 8);
        cfi_offset!(22, SC_REGS + 22 * 8);
        cfi_offset!(23, SC_REGS + 23 * 8);
        cfi_offset!(24, SC_REGS + 24 * 8);
        cfi_offset!(25, SC_REGS + 25 * 8);
        cfi_offset!(26, SC_REGS + 26 * 8);
        cfi_offset!(27, SC_REGS + 27 * 8);
        cfi_offset!(28, SC_REGS + 28 * 8);
        cfi_offset!(29, SC_REGS + 29 * 8);
        cfi_offset!(30, SC_REGS + 30 * 8);
        cfi_offset!(31, SC_REGS + 31 * 8);
        nop!();
        SYM_START!($name, SYM_L_GLOBAL, SYM_A_ALIGN);
    }};
}

macro_rules! SYM_SIGFUNC_END {
    ($name:ident) => { SYM_FUNC_END!($name); };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
