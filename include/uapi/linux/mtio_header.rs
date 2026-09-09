/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Linux magnetic tape ioctl header, translated from C. */

use core::ffi::{c_int, c_long, c_short};

/* External kernel type and ioctl macros are supplied by dependent headers. */

#[repr(C)]
pub struct mtop {
    pub mt_op: c_short,
    pub mt_count: c_int,
}

pub const MTRESET: c_int = 0;
pub const MTFSF: c_int = 1;
pub const MTBSF: c_int = 2;
pub const MTFSR: c_int = 3;
pub const MTBSR: c_int = 4;
pub const MTWEOF: c_int = 5;
pub const MTREW: c_int = 6;
pub const MTOFFL: c_int = 7;
pub const MTNOP: c_int = 8;
pub const MTRETEN: c_int = 9;
pub const MTBSFM: c_int = 10;
pub const MTFSFM: c_int = 11;
pub const MTEOM: c_int = 12;
pub const MTERASE: c_int = 13;
pub const MTRAS1: c_int = 14;
pub const MTRAS2: c_int = 15;
pub const MTRAS3: c_int = 16;
pub const MTSETBLK: c_int = 20;
pub const MTSETDENSITY: c_int = 21;
pub const MTSEEK: c_int = 22;
pub const MTTELL: c_int = 23;
pub const MTSETDRVBUFFER: c_int = 24;
pub const MTFSS: c_int = 25;
pub const MTBSS: c_int = 26;
pub const MTWSM: c_int = 27;
pub const MTLOCK: c_int = 28;
pub const MTUNLOCK: c_int = 29;
pub const MTLOAD: c_int = 30;
pub const MTUNLOAD: c_int = 31;
pub const MTCOMPRESSION: c_int = 32;
pub const MTSETPART: c_int = 33;
pub const MTMKPART: c_int = 34;
pub const MTWEOFI: c_int = 35;

#[repr(C)]
pub struct mtget {
    pub mt_type: c_long,
    pub mt_resid: c_long,
    pub mt_dsreg: c_long,
    pub mt_gstat: c_long,
    pub mt_erreg: c_long,
    pub mt_fileno: __kernel_daddr_t,
    pub mt_blkno: __kernel_daddr_t,
}

pub const MT_ISUNKNOWN: c_int = 0x01;
pub const MT_ISQIC02: c_int = 0x02;
pub const MT_ISWT5150: c_int = 0x03;
pub const MT_ISARCHIVE_5945L2: c_int = 0x04;
pub const MT_ISCMSJ500: c_int = 0x05;
pub const MT_ISTDC3610: c_int = 0x06;
pub const MT_ISARCHIVE_VP60I: c_int = 0x07;
pub const MT_ISARCHIVE_2150L: c_int = 0x08;
pub const MT_ISARCHIVE_2060L: c_int = 0x09;
pub const MT_ISARCHIVESC499: c_int = 0x0A;
pub const MT_ISQIC02_ALL_FEATURES: c_int = 0x0F;
pub const MT_ISWT5099EEN24: c_int = 0x11;
pub const MT_ISTEAC_MT2ST: c_int = 0x12;
pub const MT_ISEVEREX_FT40A: c_int = 0x32;
pub const MT_ISDDS1: c_int = 0x51;
pub const MT_ISDDS2: c_int = 0x52;
pub const MT_ISONSTREAM_SC: c_int = 0x61;
pub const MT_ISSCSI1: c_int = 0x71;
pub const MT_ISSCSI2: c_int = 0x72;
pub const MT_ISFTAPE_UNKNOWN: c_int = 0x800000;
pub const MT_ISFTAPE_FLAG: c_int = 0x800000;

#[repr(C)]
pub struct mtpos {
    pub mt_blkno: c_long,
}

pub const MTIOCTOP: _ = _IOW('m' as _, 1, mtop);
pub const MTIOCGET: _ = _IOR('m' as _, 2, mtget);
pub const MTIOCPOS: _ = _IOR('m' as _, 3, mtpos);

pub const fn GMT_EOF(x: u32) -> u32 { x & 0x80000000 }
pub const fn GMT_BOT(x: u32) -> u32 { x & 0x40000000 }
pub const fn GMT_EOT(x: u32) -> u32 { x & 0x20000000 }
pub const fn GMT_SM(x: u32) -> u32 { x & 0x10000000 }
pub const fn GMT_EOD(x: u32) -> u32 { x & 0x08000000 }
pub const fn GMT_WR_PROT(x: u32) -> u32 { x & 0x04000000 }
pub const fn GMT_ONLINE(x: u32) -> u32 { x & 0x01000000 }
pub const fn GMT_D_6250(x: u32) -> u32 { x & 0x00800000 }
pub const fn GMT_D_1600(x: u32) -> u32 { x & 0x00400000 }
pub const fn GMT_D_800(x: u32) -> u32 { x & 0x00200000 }
pub const fn GMT_DR_OPEN(x: u32) -> u32 { x & 0x00040000 }
pub const fn GMT_IM_REP_EN(x: u32) -> u32 { x & 0x00010000 }
pub const fn GMT_CLN(x: u32) -> u32 { x & 0x00008000 }

pub const MT_ST_BLKSIZE_SHIFT: c_int = 0;
pub const MT_ST_BLKSIZE_MASK: u32 = 0xffffff;
pub const MT_ST_DENSITY_SHIFT: c_int = 24;
pub const MT_ST_DENSITY_MASK: u32 = 0xff000000;
pub const MT_ST_SOFTERR_SHIFT: c_int = 0;
pub const MT_ST_SOFTERR_MASK: u32 = 0xffff;
pub const MT_ST_OPTIONS: u32 = 0xf0000000;
pub const MT_ST_BOOLEANS: u32 = 0x10000000;
pub const MT_ST_SETBOOLEANS: u32 = 0x30000000;
pub const MT_ST_CLEARBOOLEANS: u32 = 0x40000000;
pub const MT_ST_WRITE_THRESHOLD: u32 = 0x20000000;
pub const MT_ST_DEF_BLKSIZE: u32 = 0x50000000;
pub const MT_ST_DEF_OPTIONS: u32 = 0x60000000;
pub const MT_ST_TIMEOUTS: u32 = 0x70000000;
pub const MT_ST_SET_TIMEOUT: u32 = MT_ST_TIMEOUTS | 0x000000;
pub const MT_ST_SET_LONG_TIMEOUT: u32 = MT_ST_TIMEOUTS | 0x100000;
pub const MT_ST_SET_CLN: u32 = 0x80000000;
pub const MT_ST_BUFFER_WRITES: u32 = 0x1;
pub const MT_ST_ASYNC_WRITES: u32 = 0x2;
pub const MT_ST_READ_AHEAD: u32 = 0x4;
pub const MT_ST_DEBUGGING: u32 = 0x8;
pub const MT_ST_TWO_FM: u32 = 0x10;
pub const MT_ST_FAST_MTEOM: u32 = 0x20;
pub const MT_ST_AUTO_LOCK: u32 = 0x40;
pub const MT_ST_DEF_WRITES: u32 = 0x80;
pub const MT_ST_CAN_BSR: u32 = 0x100;
pub const MT_ST_NO_BLKLIMS: u32 = 0x200;
pub const MT_ST_CAN_PARTITIONS: u32 = 0x400;
pub const MT_ST_SCSI2LOGICAL: u32 = 0x800;
pub const MT_ST_SYSV: u32 = 0x1000;
pub const MT_ST_NOWAIT: u32 = 0x2000;
pub const MT_ST_SILI: u32 = 0x4000;
pub const MT_ST_NOWAIT_EOF: u32 = 0x8000;
pub const MT_ST_CLEAR_DEFAULT: u32 = 0xfffff;
pub const MT_ST_DEF_DENSITY: u32 = MT_ST_DEF_OPTIONS | 0x100000;
pub const MT_ST_DEF_COMPRESSION: u32 = MT_ST_DEF_OPTIONS | 0x200000;
pub const MT_ST_DEF_DRVBUFFER: u32 = MT_ST_DEF_OPTIONS | 0x300000;
pub const MT_ST_HPLOADER_OFFSET: c_int = 10000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
