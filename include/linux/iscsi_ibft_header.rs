/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright 2007 Red Hat, Inc.
 *  by Peter Jones <pjones@redhat.com>
 *  Copyright 2007 IBM, Inc.
 *  by Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
 *  Copyright 2008
 *  by Konrad Rzeszutek <ketuzsezr@darnok.org>
 *
 * This code exposes the iSCSI Boot Format Table to userland via sysfs.
 */

// C header dependency: <linux/types.h>

/*
 * Physical location of iSCSI Boot Format Table.
 * If the value is 0 there is no iBFT on the machine.
 */
extern "C" {
    pub static mut ibft_phys_addr: phys_addr_t;
}

#[cfg(CONFIG_ISCSI_IBFT_FIND)]

/*
 * Routine used to find and reserve the iSCSI Boot Format Table. The
 * physical address is set in the ibft_phys_addr variable.
 */
extern "C" {
    pub fn reserve_ibft_region();
}

/*
 * Physical bounds to search for the iSCSI Boot Format Table.
 */
#[cfg(CONFIG_ISCSI_IBFT_FIND)]
pub const IBFT_START: usize = 0x80000; /* 512kB */
#[cfg(CONFIG_ISCSI_IBFT_FIND)]
pub const IBFT_END: usize = 0x100000; /* 1MB */

#[cfg(not(CONFIG_ISCSI_IBFT_FIND))]
#[inline]
pub fn reserve_ibft_region() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
