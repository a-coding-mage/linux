/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2018 Cadence Design Systems Inc.
 *
 * Author: Boris Brezillon <boris.brezillon@bootlin.com>
 */

/* Rust translation of linux/i3c/ccc.h. */

pub const I3C_CCC_RETRIES: u32 = 1;
pub const I3C_CCC_DIRECT: u8 = 1 << 7;

#[inline]
pub const fn I3C_CCC_ID(id: u8, broadcast: bool) -> u8 { id | if broadcast { 0 } else { I3C_CCC_DIRECT } }
#[inline]
pub const fn I3C_CCC_ENEC(broadcast: bool) -> u8 { I3C_CCC_ID(0x0, broadcast) }
#[inline]
pub const fn I3C_CCC_DISEC(broadcast: bool) -> u8 { I3C_CCC_ID(0x1, broadcast) }
#[inline]
pub const fn I3C_CCC_ENTAS(as_: u8, broadcast: bool) -> u8 { I3C_CCC_ID(0x2 + as_, broadcast) }
#[inline]
pub const fn I3C_CCC_RSTDAA(broadcast: bool) -> u8 { I3C_CCC_ID(0x6, broadcast) }
#[inline]
pub const fn I3C_CCC_SETMWL(broadcast: bool) -> u8 { I3C_CCC_ID(0x9, broadcast) }
#[inline]
pub const fn I3C_CCC_SETMRL(broadcast: bool) -> u8 { I3C_CCC_ID(0xa, broadcast) }
#[inline]
pub const fn I3C_CCC_SETXTIME(broadcast: bool) -> u8 { if broadcast { 0x28 } else { 0x98 } }
#[inline]
pub const fn I3C_CCC_VENDOR(id: u8, broadcast: bool) -> u8 { id + if broadcast { 0x61 } else { 0xe0 } }

pub const I3C_CCC_ENTDAA: u8 = 0x7;
pub const I3C_CCC_DEFSLVS: u8 = 0x8;
pub const I3C_CCC_ENTTM: u8 = 0xb;
#[inline] pub const fn I3C_CCC_ENTHDR(x: u8) -> u8 { 0x20 + x }
pub const I3C_CCC_SETAASA: u8 = 0x29;
pub const I3C_CCC_SETDASA: u8 = 0x87;
pub const I3C_CCC_SETNEWDA: u8 = 0x88;
pub const I3C_CCC_GETMWL: u8 = 0x8b;
pub const I3C_CCC_GETMRL: u8 = 0x8c;
pub const I3C_CCC_GETPID: u8 = 0x8d;
pub const I3C_CCC_GETBCR: u8 = 0x8e;
pub const I3C_CCC_GETDCR: u8 = 0x8f;
pub const I3C_CCC_GETSTATUS: u8 = 0x90;
pub const I3C_CCC_GETACCMST: u8 = 0x91;
pub const I3C_CCC_SETBRGTGT: u8 = 0x93;
pub const I3C_CCC_GETMXDS: u8 = 0x94;
pub const I3C_CCC_GETHDRCAP: u8 = 0x95;
pub const I3C_CCC_GETXTIME: u8 = 0x99;

pub const I3C_CCC_EVENT_SIR: u8 = 1 << 0;
pub const I3C_CCC_EVENT_MR: u8 = 1 << 1;
pub const I3C_CCC_EVENT_HJ: u8 = 1 << 3;

#[repr(C)] pub struct i3c_ccc_events { pub events: u8 }
#[repr(C)] pub struct i3c_ccc_mwl { pub len: u16 }
#[repr(C, packed)] pub struct i3c_ccc_mrl { pub read_len: u16, pub ibi_len: u8 }

#[repr(C)] pub union i3c_ccc_dev_desc__bindgen_ty_1 { pub dcr: u8, pub lvr: u8 }
#[repr(C)] pub struct i3c_ccc_dev_desc { pub dyn_addr: u8, pub __bindgen_anon_1: i3c_ccc_dev_desc__bindgen_ty_1, pub bcr: u8, pub static_addr: u8 }
#[repr(C, packed)] pub struct i3c_ccc_defslvs { pub count: u8, pub master: i3c_ccc_dev_desc, pub slaves: [i3c_ccc_dev_desc; 0] }

#[repr(C)] pub enum i3c_ccc_test_mode { I3C_CCC_EXIT_TEST_MODE, I3C_CCC_VENDOR_TEST_MODE }
#[repr(C)] pub struct i3c_ccc_enttm { pub mode: u8 }
#[repr(C)] pub struct i3c_ccc_setda { pub addr: u8 }
#[repr(C)] pub struct i3c_ccc_getpid { pub pid: [u8; 6] }
#[repr(C)] pub struct i3c_ccc_getbcr { pub bcr: u8 }
#[repr(C)] pub struct i3c_ccc_getdcr { pub dcr: u8 }

#[inline] pub const fn I3C_CCC_STATUS_PENDING_INT(status: u16) -> u16 { status & 0xf }
pub const I3C_CCC_STATUS_PROTOCOL_ERROR: u16 = 1 << 5;
#[inline] pub const fn I3C_CCC_STATUS_ACTIVITY_MODE(status: u16) -> u16 { (status & 0xc0) >> 6 }
#[repr(C)] pub struct i3c_ccc_getstatus { pub status: u16 }
#[repr(C)] pub struct i3c_ccc_getaccmst { pub newmaster: u8 }
#[repr(C, packed)] pub struct i3c_ccc_bridged_slave_desc { pub addr: u8, pub id: u16 }
#[repr(C, packed)] pub struct i3c_ccc_setbrgtgt { pub count: u8, pub bslaves: [i3c_ccc_bridged_slave_desc; 0] }

#[repr(C)] pub enum i3c_sdr_max_data_rate { I3C_SDR0_FSCL_MAX, I3C_SDR1_FSCL_8MHZ, I3C_SDR2_FSCL_6MHZ, I3C_SDR3_FSCL_4MHZ, I3C_SDR4_FSCL_2MHZ }
#[repr(C)] pub enum i3c_tsco { I3C_TSCO_8NS, I3C_TSCO_9NS, I3C_TSCO_10NS, I3C_TSCO_11NS, I3C_TSCO_12NS }
pub const I3C_CCC_MAX_SDR_FSCL_MASK: u8 = 0x7;
#[inline] pub const fn I3C_CCC_MAX_SDR_FSCL(x: u8) -> u8 { x & I3C_CCC_MAX_SDR_FSCL_MASK }
#[repr(C, packed)] pub struct i3c_ccc_getmxds { pub maxwr: u8, pub maxrd: u8, pub maxrdturn: [u8; 3] }
#[inline] pub const fn I3C_CCC_HDR_MODE(mode: u32) -> u32 { 1 << mode }
#[repr(C, packed)] pub struct i3c_ccc_gethdrcap { pub modes: u8 }

#[repr(C)] pub enum i3c_ccc_setxtime_subcmd { I3C_CCC_SETXTIME_ST = 0x7f, I3C_CCC_SETXTIME_DT = 0xbf, I3C_CCC_SETXTIME_ENTER_ASYNC_MODE0 = 0xdf, I3C_CCC_SETXTIME_ENTER_ASYNC_MODE1 = 0xef, I3C_CCC_SETXTIME_ENTER_ASYNC_MODE2 = 0xf7, I3C_CCC_SETXTIME_ENTER_ASYNC_MODE3 = 0xfb, I3C_CCC_SETXTIME_ASYNC_TRIGGER = 0xfd, I3C_CCC_SETXTIME_TPH = 0x3f, I3C_CCC_SETXTIME_TU = 0x9f, I3C_CCC_SETXTIME_ODR = 0x8f }
#[repr(C, packed)] pub struct i3c_ccc_setxtime { pub subcmd: u8, pub data: [u8; 0] }
pub const I3C_CCC_GETXTIME_SYNC_MODE: u8 = 1;
#[inline] pub const fn I3C_CCC_GETXTIME_ASYNC_MODE(x: u32) -> u32 { 1 << (x + 1) }
pub const I3C_CCC_GETXTIME_OVERFLOW: u8 = 1 << 7;
#[repr(C, packed)] pub struct i3c_ccc_getxtime { pub supported_modes: u8, pub state: u8, pub frequency: u8, pub inaccuracy: u8 }

#[repr(C)] pub struct i3c_ccc_cmd_payload { pub len: u16, pub actual_len: u16, pub optional_bytes: u16, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct i3c_ccc_cmd_dest { pub addr: u8, pub payload: i3c_ccc_cmd_payload }
/* `i3c_error_code` is supplied by linux/i3c/device.h. */
#[repr(C)] pub struct i3c_ccc_cmd { pub rnw: u8, pub id: u8, pub ndests: core::ffi::c_uint, pub retries: core::ffi::c_uint, pub dests: *mut i3c_ccc_cmd_dest, pub err: i3c_error_code }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
