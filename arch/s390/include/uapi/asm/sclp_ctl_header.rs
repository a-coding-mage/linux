/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * IOCTL interface for SCLP
 *
 * Copyright IBM Corp. 2012
 *
 * Author: Michael Holzheu <holzheu@linux.vnet.ibm.com>
 */

// Dependency corresponding to <linux/types.h> is supplied externally.

#[repr(C, packed)]
pub struct sclp_ctl_sccb {
    pub cmdw: u32,
    pub sccb: u64,
}

pub const SCLP_CTL_IOCTL_MAGIC: u32 = 0x10;

// Corresponds to the C _IOWR(SCLP_CTL_IOCTL_MAGIC, 0x10, struct sclp_ctl_sccb)
// macro; _IOWR! is supplied by the surrounding ioctl definitions.
pub const SCLP_CTL_SCCB: u32 = _IOWR!(SCLP_CTL_IOCTL_MAGIC, 0x10, sclp_ctl_sccb);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
