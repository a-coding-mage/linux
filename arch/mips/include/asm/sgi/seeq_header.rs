/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 by Ralf Baechle
 */

// Dependency supplied by <asm/sgi/hpc3.h>.
// Dependency intent from <linux/if_ether.h>: ETH_ALEN is 6.

#[repr(C)]
pub struct sgiseeq_platform_data {
    pub hpc: *mut hpc3_regs,
    pub irq: u32,
    pub mac: [u8; 6],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
