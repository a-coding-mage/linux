/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ADB through the IOP
 * Written by Joshua M. Thompson
 */

/* IOP number and channel number for ADB */

// IOP_NUM_ISM is supplied by an external dependency.
pub const ADB_IOP: usize = IOP_NUM_ISM as usize;
pub const ADB_CHAN: u8 = 2;

/* From the A/UX headers...maybe important, maybe not */

pub const ADB_IOP_LISTEN: u8 = 0x01;
pub const ADB_IOP_TALK: u8 = 0x02;
pub const ADB_IOP_EXISTS: u8 = 0x04;
pub const ADB_IOP_FLUSH: u8 = 0x08;
pub const ADB_IOP_RESET: u8 = 0x10;
pub const ADB_IOP_INT: u8 = 0x20;
pub const ADB_IOP_POLL: u8 = 0x40;
pub const ADB_IOP_UNINT: u8 = 0x80;

pub const AIF_RESET: u8 = 0x00;
pub const AIF_FLUSH: u8 = 0x01;
pub const AIF_LISTEN: u8 = 0x08;
pub const AIF_TALK: u8 = 0x0C;

/* Flag bits in struct adb_iopmsg */

pub const ADB_IOP_EXPLICIT: u8 = 0x80; /* nonzero if explicit command */
pub const ADB_IOP_AUTOPOLL: u8 = 0x40; /* auto/SRQ polling enabled    */
pub const ADB_IOP_SET_AUTOPOLL: u8 = 0x20; /* set autopoll device list    */
pub const ADB_IOP_SRQ: u8 = 0x04; /* SRQ detected                */
pub const ADB_IOP_TIMEOUT: u8 = 0x02; /* nonzero if timeout          */

#[repr(C)]
pub struct adb_iopmsg {
    pub flags: u8,     /* ADB flags         */
    pub count: u8,     /* no. of data bytes */
    pub cmd: u8,       /* ADB command       */
    pub data: [u8; 8], /* ADB data          */
    pub spare: [u8; 21], /* spare             */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
