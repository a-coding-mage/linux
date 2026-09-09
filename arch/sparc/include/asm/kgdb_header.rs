/* SPDX-License-Identifier: GPL-2.0 */

// The original header guard was: _SPARC_KGDB_H

// These conditional definitions preserve the build-time CONFIG_SPARC32 intent.
#[cfg(CONFIG_SPARC32)]
pub const BUFMAX: usize = 2048;
#[cfg(not(CONFIG_SPARC32))]
pub const BUFMAX: usize = 4096;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum regnames {
    GDB_G0,
    GDB_G1,
    GDB_G2,
    GDB_G3,
    GDB_G4,
    GDB_G5,
    GDB_G6,
    GDB_G7,
    GDB_O0,
    GDB_O1,
    GDB_O2,
    GDB_O3,
    GDB_O4,
    GDB_O5,
    GDB_SP,
    GDB_O7,
    GDB_L0,
    GDB_L1,
    GDB_L2,
    GDB_L3,
    GDB_L4,
    GDB_L5,
    GDB_L6,
    GDB_L7,
    GDB_I0,
    GDB_I1,
    GDB_I2,
    GDB_I3,
    GDB_I4,
    GDB_I5,
    GDB_FP,
    GDB_I7,
    GDB_F0,
    GDB_F31 = GDB_F0 as isize + 31,
    #[cfg(CONFIG_SPARC32)]
    GDB_Y,
    #[cfg(CONFIG_SPARC32)]
    GDB_PSR,
    #[cfg(CONFIG_SPARC32)]
    GDB_WIM,
    #[cfg(CONFIG_SPARC32)]
    GDB_TBR,
    #[cfg(CONFIG_SPARC32)]
    GDB_PC,
    #[cfg(CONFIG_SPARC32)]
    GDB_NPC,
    #[cfg(CONFIG_SPARC32)]
    GDB_FSR,
    #[cfg(CONFIG_SPARC32)]
    GDB_CSR,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_F32 = GDB_F0 as isize + 32,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_F62 = GDB_F32 as isize + 15,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_PC,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_NPC,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_STATE,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_FSR,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_FPRS,
    #[cfg(not(CONFIG_SPARC32))]
    GDB_Y,
}

#[cfg(CONFIG_SPARC32)]
pub const NUMREGBYTES: usize = (regnames::GDB_CSR as usize + 1) * 4;
#[cfg(not(CONFIG_SPARC32))]
pub const NUMREGBYTES: usize = (regnames::GDB_Y as usize + 1) * 8;

#[repr(C)]
pub struct pt_regs;

extern "C" {
    pub fn kgdb_trap(trap_level: libc::c_ulong, regs: *mut pt_regs);
    pub fn arch_kgdb_breakpoint();
}

pub const BREAK_INSTR_SIZE: usize = 4;
pub const CACHE_FLUSH_IS_SAFE: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
