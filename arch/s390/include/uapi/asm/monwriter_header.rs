/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright IBM Corp. 2006
 * Character device driver for writing z/VM APPLDATA monitor records
 * Version 1.0
 * Author(s): Melissa Howland <melissah@us.ibm.com>
 *
 */

/* mon_function values */
pub const MONWRITE_START_INTERVAL: u8 = 0x00; /* start interval recording */
pub const MONWRITE_STOP_INTERVAL: u8 = 0x01; /* stop interval or config recording */
pub const MONWRITE_GEN_EVENT: u8 = 0x02; /* generate event record */
pub const MONWRITE_START_CONFIG: u8 = 0x03; /* start configuration recording */

/* the header the app uses in its write() data */
#[repr(C, packed)]
pub struct monwrite_hdr {
    pub mon_function: u8,
    pub applid: u16,
    pub record_num: u8,
    pub version: u16,
    pub release: u16,
    pub mod_level: u16,
    pub datalen: u16,
    pub hdrlen: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
