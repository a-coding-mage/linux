/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Definitions for ADB (Apple Desktop Bus) support.
 */

/* ADB commands */
pub const ADB_BUSRESET: i32 = 0;

pub const fn ADB_FLUSH(id: i32) -> i32 {
    0x01 | (id << 4)
}

pub const fn ADB_WRITEREG(id: i32, reg: i32) -> i32 {
    0x08 | reg | (id << 4)
}

pub const fn ADB_READREG(id: i32, reg: i32) -> i32 {
    0x0C | reg | (id << 4)
}

/* ADB default device IDs (upper 4 bits of ADB command byte) */
pub const ADB_DONGLE: i32 = 1; /* "software execution control" devices */
pub const ADB_KEYBOARD: i32 = 2;
pub const ADB_MOUSE: i32 = 3;
pub const ADB_TABLET: i32 = 4;
pub const ADB_MODEM: i32 = 5;
pub const ADB_MISC: i32 = 7; /* maybe a monitor */

pub const ADB_RET_OK: i32 = 0;
pub const ADB_RET_TIMEOUT: i32 = 3;

/* The kind of ADB request. The controller may emulate some
   or all of those CUDA/PMU packet kinds */
pub const ADB_PACKET: i32 = 0;
pub const CUDA_PACKET: i32 = 1;
pub const ERROR_PACKET: i32 = 2;
pub const TIMER_PACKET: i32 = 3;
pub const POWER_PACKET: i32 = 4;
pub const MACIIC_PACKET: i32 = 5;
pub const PMU_PACKET: i32 = 6;
pub const ADB_QUERY: i32 = 7;

/* ADB queries */

/* ADB_QUERY_GETDEVINFO
 * Query ADB slot for device presence
 * data[2] = id, rep[0] = orig addr, rep[1] = handler_id
 */
pub const ADB_QUERY_GETDEVINFO: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
