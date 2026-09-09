/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2000 Jens Axboe <axboe@suse.de>
 * Copyright (C) 2001-2004 Peter Osterlund <petero2@telia.com>
 *
 * Packet writing layer for ATAPI and SCSI CD-R, CD-RW, DVD-R, and
 * DVD-RW devices.
 */

// Dependency supplied by the Linux types environment: __u32 is u32.

pub const PACKET_DEBUG: u32 = 1;
pub const MAX_WRITERS: u32 = 8;
pub const PKT_RB_POOL_SIZE: u32 = 512;

/* HZ is supplied by the kernel environment. */
pub const PACKET_WAIT_TIME: u32 = HZ * 5 / 1000;

pub const PACKET_CDR: u32 = 1;
pub const PACKET_CDRW: u32 = 2;
pub const PACKET_DVDR: u32 = 3;
pub const PACKET_DVDRW: u32 = 4;

pub const PACKET_WRITABLE: u32 = 1;
pub const PACKET_NWA_VALID: u32 = 2;
pub const PACKET_LRA_VALID: u32 = 3;
pub const PACKET_MERGE_SEGS: u32 = 4;

pub const PACKET_DISC_EMPTY: u32 = 0;
pub const PACKET_DISC_INCOMPLETE: u32 = 1;
pub const PACKET_DISC_COMPLETE: u32 = 2;
pub const PACKET_DISC_OTHER: u32 = 3;

pub const PACKET_MODE1: u32 = 1;
pub const PACKET_MODE2: u32 = 2;
pub const PACKET_BLOCK_MODE1: u32 = 8;
pub const PACKET_BLOCK_MODE2: u32 = 10;

pub const PACKET_SESSION_EMPTY: u32 = 0;
pub const PACKET_SESSION_INCOMPLETE: u32 = 1;
pub const PACKET_SESSION_RESERVED: u32 = 2;
pub const PACKET_SESSION_COMPLETE: u32 = 3;

pub const PACKET_MCN: &str = "4a656e734178626f65323030300000";

pub const PKT_CTRL_CMD_SETUP: u32 = 0;
pub const PKT_CTRL_CMD_TEARDOWN: u32 = 1;
pub const PKT_CTRL_CMD_STATUS: u32 = 2;

#[repr(C)]
pub struct pkt_ctrl_command {
    pub command: u32,    // in: Setup, teardown, status
    pub dev_index: u32,  // in/out: Device index
    pub dev: u32,        // in/out: Device nr for cdrw device
    pub pkt_dev: u32,    // in/out: Device nr for packet device
    pub num_devices: u32, // out: Largest device index + 1
    pub padding: u32,    // Not used
}

pub const PACKET_IOCTL_MAGIC: u32 = 'X' as u32;

// _IOWR is supplied by the Linux ioctl environment.
pub const PACKET_CTRL_CMD: u32 = _IOWR!(PACKET_IOCTL_MAGIC, 1, pkt_ctrl_command);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
