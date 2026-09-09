/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ARM KGDB support
 *
 * Author: Deepak Saxena <dsaxena@mvista.com>
 *
 * Copyright (C) 2002 MontaVista Software Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux::ptrace and asm::opcodes (__inst_arm).

/*
 * GDB assumes that we're a user process being debugged, so
 * it will send us an SWI command to write into memory as the
 * debug trap. When an SWI occurs, the next instruction addr is
 * placed into R14_svc before jumping to the vector trap.
 * This doesn't work for kernel debugging as we are already in SVC
 * we would loose the kernel's LR, which is a bad thing. This
 * is  bad thing.
 *
 * By doing this as an undefined instruction trap, we force a mode
 * switch from SVC to UND mode, allowing us to save full kernel state.
 *
 * We also define a KGDB_COMPILED_BREAK which can be used to compile
 * in breakpoints. This is important for things like sysrq-G and for
 * the initial breakpoint from trap_init().
 *
 * Note to ARM HW designers: Add real trap support like SH && PPC to
 * make our lives much much simpler. :)
 */
pub const BREAK_INSTR_SIZE: u32 = 4;
pub const GDB_BREAKINST: u32 = 0xef9f0001;
pub const KGDB_BREAKINST: u32 = 0xe7ffdefe;
pub const KGDB_COMPILED_BREAK: u32 = 0xe7ffdeff;
pub const CACHE_FLUSH_IS_SAFE: u32 = 1;

pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!(".word 0xe7ffdeff");
}

pub unsafe extern "C" fn kgdb_handle_bus_error();
pub static mut kgdb_fault_expected: core::ffi::c_int;

/*
 * From Kevin Hilman:
 *
 * gdb is expecting the following registers layout.
 *
 * r0-r15: 1 long word each
 * f0-f7:  unused, 3 long words each !!
 * fps:    unused, 1 long word
 * cpsr:   1 long word
 *
 * Even though f0-f7 and fps are not used, they need to be
 * present in the registers sent for correct processing in the
 * host-side gdb.
 *
 * In particular, it is crucial that CPSR is in the right place,
 * otherwise gdb will not be able to correctly interpret stepping over
 * conditional branches.
 */
pub const _GP_REGS: usize = 16;
pub const _FP_REGS: usize = 8;
pub const _EXTRA_REGS: usize = 2;
pub const GDB_MAX_REGS: usize = _GP_REGS + (_FP_REGS * 3) + _EXTRA_REGS;
pub const DBG_MAX_REG_NUM: usize = _GP_REGS + _FP_REGS + _EXTRA_REGS;

pub const KGDB_MAX_NO_CPUS: usize = 1;
pub const BUFMAX: usize = 400;
pub const NUMREGBYTES: usize = GDB_MAX_REGS << 2;
pub const NUMCRITREGBYTES: usize = 32 << 2;

pub const _R0: usize = 0;
pub const _R1: usize = 1;
pub const _R2: usize = 2;
pub const _R3: usize = 3;
pub const _R4: usize = 4;
pub const _R5: usize = 5;
pub const _R6: usize = 6;
pub const _R7: usize = 7;
pub const _R8: usize = 8;
pub const _R9: usize = 9;
pub const _R10: usize = 10;
pub const _FP: usize = 11;
pub const _IP: usize = 12;
pub const _SPT: usize = 13;
pub const _LR: usize = 14;
pub const _PC: usize = 15;
pub const _CPSR: usize = GDB_MAX_REGS - 1;

/*
 * So that we can denote the end of a frame for tracing,
 * in the simple case:
 */
#[macro_export]
macro_rules! CFI_END_FRAME {
    ($func:expr) => {
        __CFI_END_FRAME!($crate::_PC, $crate::_SPT, $func)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
