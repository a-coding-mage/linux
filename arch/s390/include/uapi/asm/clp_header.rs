/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ioctl interface for /dev/clp
 *
 * Copyright IBM Corp. 2016
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependencies corresponding to <linux/types.h> and <linux/ioctl.h> are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct clp_req {
	/* The C bit-fields occupy the low 16 bits of this u32 storage unit. */
	pub flags: u32,
	pub reserved: u32,
	pub data_p: u64,
}

impl clp_req {
	pub const C_SHIFT: u32 = 0;
	pub const C_MASK: u32 = 0x1;
	pub const R_SHIFT: u32 = 1;
	pub const R_MASK: u32 = 0x1;
	pub const LPS_SHIFT: u32 = 2;
	pub const LPS_MASK: u32 = 0x3f;
	pub const CMD_SHIFT: u32 = 8;
	pub const CMD_MASK: u32 = 0xff;

	#[inline]
	pub const fn c(&self) -> u32 {
		(self.flags >> Self::C_SHIFT) & Self::C_MASK
	}

	#[inline]
	pub const fn r(&self) -> u32 {
		(self.flags >> Self::R_SHIFT) & Self::R_MASK
	}

	#[inline]
	pub const fn lps(&self) -> u32 {
		(self.flags >> Self::LPS_SHIFT) & Self::LPS_MASK
	}

	#[inline]
	pub const fn cmd(&self) -> u32 {
		(self.flags >> Self::CMD_SHIFT) & Self::CMD_MASK
	}
}

pub const CLP_IOCTL_MAGIC: u8 = b'c';

// `_IOWR` is provided by the ioctl interface dependency.
pub const CLP_SYNC: _ = _IOWR(CLP_IOCTL_MAGIC, 0xC1, clp_req);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
