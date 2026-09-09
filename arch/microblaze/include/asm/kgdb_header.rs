/* SPDX-License-Identifier: GPL-2.0 */

// This header is active only when building the kernel and outside assembler
// sources in the original C implementation.

pub const CACHE_FLUSH_IS_SAFE: i32 = 1;
pub const BUFMAX: usize = 2048;

/*
 * 32 32-bit general purpose registers (r0-r31)
 *  6 32-bit special registers (pc, msr, ear, esr, fsr, btr)
 * 12 32-bit PVR
 *   7 32-bit MMU Regs (redr, rpid, rzpr, rtlbx, rtlbsx, rtlblo, rtlbhi)
 * ------
 *  57 registers
 */
pub const NUMREGBYTES: usize = 57 * 4;

pub const BREAK_INSTR_SIZE: usize = 4;

#[inline]
pub unsafe fn arch_kgdb_breakpoint() {
    core::arch::asm!("brki r16, 0x18;");
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn microblaze_kgdb_break(regs: *mut pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
