/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Exception frame offsets.
 */
pub const EF_V0: usize = 0;
pub const EF_T0: usize = 1;
pub const EF_T1: usize = 2;
pub const EF_T2: usize = 3;
pub const EF_T3: usize = 4;
pub const EF_T4: usize = 5;
pub const EF_T5: usize = 6;
pub const EF_T6: usize = 7;
pub const EF_T7: usize = 8;
pub const EF_S0: usize = 9;
pub const EF_S1: usize = 10;
pub const EF_S2: usize = 11;
pub const EF_S3: usize = 12;
pub const EF_S4: usize = 13;
pub const EF_S5: usize = 14;
pub const EF_S6: usize = 15;
pub const EF_A3: usize = 16;
pub const EF_A4: usize = 17;
pub const EF_A5: usize = 18;
pub const EF_T8: usize = 19;
pub const EF_T9: usize = 20;
pub const EF_T10: usize = 21;
pub const EF_T11: usize = 22;
pub const EF_RA: usize = 23;
pub const EF_T12: usize = 24;
pub const EF_AT: usize = 25;
pub const EF_SP: usize = 26;
pub const EF_PS: usize = 27;
pub const EF_PC: usize = 28;
pub const EF_GP: usize = 29;
pub const EF_A0: usize = 30;
pub const EF_A1: usize = 31;
pub const EF_A2: usize = 32;

pub const EF_SIZE: usize = 33 * 8;
pub const HWEF_SIZE: usize = 6 * 8; /* size of PAL frame (PS-A2) */

pub const EF_SSIZE: usize = EF_SIZE - HWEF_SIZE;

/*
 * Map register number into core file offset.
 */
#[macro_export]
macro_rules! CORE_REG {
	($reg:expr, $ubase:expr) => {
		unsafe {
			*((($ubase as usize) as *const usize).add($reg as usize))
		}
	};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
