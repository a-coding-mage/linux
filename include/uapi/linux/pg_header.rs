/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */
/*
 * pg.h (c) 1998 Grant R. Guenther <grant@torque.net>
 *
 * Rust translation of the user interface to the generic ATAPI packet
 * command driver for parallel port ATAPI devices (pg).
 *
 * The original interface permits future changes by placing a single
 * character magic flag in the read and write buffers.  Currently this
 * flag must be the character "P".
 */

pub const PG_MAGIC: u8 = b'P';
pub const PG_RESET: u8 = b'Z';
pub const PG_COMMAND: u8 = b'C';

pub const PG_MAX_DATA: i32 = 32768;

#[repr(C)]
pub struct pg_write_hdr {
    pub magic: u8,       /* == PG_MAGIC */
    pub func: u8,        /* PG_RESET or PG_COMMAND */
    pub dlen: i32,       /* number of bytes expected to transfer */
    pub timeout: i32,    /* number of seconds before timeout */
    pub packet: [u8; 12], /* packet command */
}

#[repr(C)]
pub struct pg_read_hdr {
    pub magic: u8,       /* == PG_MAGIC */
    pub scsi: u8,        /* "scsi" status == sense key */
    pub dlen: i32,       /* size of device transfer request */
    pub duration: i32,   /* time in seconds command took */
    pub pad: [u8; 12],   /* not used */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
