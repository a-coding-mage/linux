/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Source-level Rust translation of bpmp-abi.h.
 * The ABI declarations below intentionally retain C layout and naming.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const BPMP_MAIL_DO_ACK: u32 = 1u32 << 0;
pub const BPMP_MAIL_RING_DB: u32 = 1u32 << 1;
pub const BPMP_MAIL_CRC_PRESENT: u32 = 1u32 << 2;

#[repr(C, packed)]
pub struct mrq_request {
    pub mrq: u32,
    pub flags: u32,
}

#[repr(C, packed)]
pub struct mrq_response {
    pub err: i32,
    pub flags: u32,
}

pub const MSG_MIN_SZ: u32 = 128;
pub const MSG_DATA_MIN_SZ: u32 = 120;
pub const MRQ_PING: u32 = 0;
pub const MRQ_QUERY_TAG: u32 = 1;
pub const MRQ_THREADED_PING: u32 = 9;
pub const MRQ_DEBUGFS: u32 = 19;
pub const MRQ_RESET: u32 = 20;
pub const MRQ_I2C: u32 = 21;
pub const MRQ_CLK: u32 = 22;
pub const MRQ_QUERY_ABI: u32 = 23;
pub const MRQ_THERMAL: u32 = 27;
pub const MRQ_CPU_VHINT: u32 = 28;
pub const MRQ_ABI_RATCHET: u32 = 29;
pub const MRQ_EMC_DVFS_LATENCY: u32 = 31;
pub const MRQ_SHUTDOWN: u32 = 49;
pub const MRQ_RINGBUF_CONSOLE: u32 = 65;
pub const MRQ_PG: u32 = 66;
pub const MRQ_CPU_NDIV_LIMITS: u32 = 67;
pub const MRQ_STRAP: u32 = 68;
pub const MRQ_UPHY: u32 = 69;
pub const MRQ_CPU_AUTO_CC3: u32 = 70;
pub const MRQ_QUERY_FW_TAG: u32 = 71;
pub const MRQ_FMON: u32 = 72;
pub const MRQ_EC: u32 = 73;
pub const MRQ_DEBUG: u32 = 75;
pub const MRQ_EMC_DVFS_EMCHUB: u32 = 76;
pub const MRQ_BWMGR: u32 = 77;
pub const MRQ_ISO_CLIENT: u32 = 78;
pub const MRQ_EMC_DISP_RFL: u32 = 79;
pub const MRQ_TELEMETRY: u32 = 80;
pub const MRQ_PWR_LIMIT: u32 = 81;
pub const MRQ_GEARS: u32 = 82;
pub const MRQ_BWMGR_INT: u32 = 83;
pub const MRQ_OC_STATUS: u32 = 84;
pub const MRQ_C2C: u32 = 85;
pub const MRQ_THROTTLE: u32 = 86;
pub const MRQ_PWRMODEL: u32 = 87;
pub const MRQ_PCIE: u32 = 88;
pub const MRQ_PWR_CNTRL: u32 = 89;
pub const MRQ_CR7: u32 = 90;
pub const MRQ_SLC: u32 = 91;
pub const MRQ_TELEMETRY_EX: u32 = 92;
pub const MRQ_HWPM: u32 = 93;
pub const MRQ_DVFS: u32 = 94;
pub const MRQ_PPP_PROFILE: u32 = 95;
pub const MAX_CPU_MRQ_ID: u32 = 95;

pub const BPMP_EPERM: i32 = 1;
pub const BPMP_ENOENT: i32 = 2;
pub const BPMP_ENOHANDLER: i32 = 3;
pub const BPMP_EIO: i32 = 5;
pub const BPMP_EBADCMD: i32 = 6;
pub const BPMP_EAGAIN: i32 = 11;
pub const BPMP_ENOMEM: i32 = 12;
pub const BPMP_EACCES: i32 = 13;
pub const BPMP_EFAULT: i32 = 14;
pub const BPMP_EBUSY: i32 = 16;
pub const BPMP_ENODEV: i32 = 19;
pub const BPMP_EINVAL: i32 = 22;
pub const BPMP_ETIMEDOUT: i32 = 23;
pub const BPMP_ERANGE: i32 = 34;
pub const BPMP_ENOSYS: i32 = 38;
pub const BPMP_EBADMSG: i32 = 77;
pub const BPMP_EOPNOTSUPP: i32 = 95;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
