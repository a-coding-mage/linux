/* SPDX-License-Identifier: GPL-2.0 */
/*
 *
 */

/* The C header guard and _ASSEMBLER_ conditional are omitted from executable Rust. */

#[repr(C)]
pub struct cpulaunch {
    pub pc: core::ffi::c_ulong,
    pub gp: core::ffi::c_ulong,
    pub sp: core::ffi::c_ulong,
    pub a0: core::ffi::c_ulong,
    pub _pad: [core::ffi::c_ulong; 3], /* pad to cache line size to avoid thrashing */
    pub flags: core::ffi::c_ulong,
}

/* When assembled, these are byte/word offsets used by assembly code. */
pub const LOG2CPULAUNCH: usize = 5;
pub const LAUNCH_PC: usize = 0;
pub const LAUNCH_GP: usize = 4;
pub const LAUNCH_SP: usize = 8;
pub const LAUNCH_A0: usize = 12;
pub const LAUNCH_FLAGS: usize = 28;

pub const LAUNCH_FREADY: usize = 1;
pub const LAUNCH_FGO: usize = 2;
pub const LAUNCH_FGONE: usize = 4;

pub const CPULAUNCH: usize = 0x00000f00;
pub const NCPULAUNCH: usize = 8;

/* Polling period in count cycles for secondary CPU's */
pub const LAUNCHPERIOD: usize = 10000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
