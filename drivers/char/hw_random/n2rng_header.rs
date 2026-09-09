/* SPDX-License-Identifier: GPL-2.0 */
/* n2rng.h: Niagara2 RNG defines.
 *
 * Copyright (C) 2008 David S. Miller <davem@davemloft.net>
 */

use core::ffi::{c_int, c_ulong};

pub const RNG_v1_CTL_WAIT: u64 = 0x0000000001fffe00;
pub const RNG_v1_CTL_WAIT_SHIFT: u32 = 9;
pub const RNG_v1_CTL_BYPASS: u64 = 0x0000000000000100;
pub const RNG_v1_CTL_VCO: u64 = 0x00000000000000c0;
pub const RNG_v1_CTL_VCO_SHIFT: u32 = 6;
pub const RNG_v1_CTL_ASEL: u64 = 0x0000000000000030;
pub const RNG_v1_CTL_ASEL_SHIFT: u32 = 4;
pub const RNG_v1_CTL_ASEL_NOOUT: u32 = 2;

pub const RNG_CTL_LFSR: u64 = 0x0000000000000008;
pub const RNG_CTL_ES3: u64 = 0x0000000000000004;
pub const RNG_CTL_ES2: u64 = 0x0000000000000002;
pub const RNG_CTL_ES1: u64 = 0x0000000000000001;

pub const RNG_v2_CTL_WAIT: u64 = 0x0000000007fff800;
pub const RNG_v2_CTL_WAIT_SHIFT: u32 = 12;
pub const RNG_v2_CTL_BYPASS: u64 = 0x0000000000000400;
pub const RNG_v2_CTL_VCO: u64 = 0x0000000000000300;
pub const RNG_v2_CTL_VCO_SHIFT: u32 = 9;
pub const RNG_v2_CTL_PERF: u64 = 0x0000000000000180;
pub const RNG_v2_CTL_ASEL: u64 = 0x0000000000000070;
pub const RNG_v2_CTL_ASEL_SHIFT: u32 = 4;
pub const RNG_v2_CTL_ASEL_NOOUT: u32 = 7;

pub const HV_FAST_RNG_GET_DIAG_CTL: u32 = 0x130;
pub const HV_FAST_RNG_CTL_READ: u32 = 0x131;
pub const HV_FAST_RNG_CTL_WRITE: u32 = 0x132;
pub const HV_FAST_RNG_DATA_READ_DIAG: u32 = 0x133;
pub const HV_FAST_RNG_DATA_READ: u32 = 0x134;

pub const HV_RNG_STATE_UNCONFIGURED: u32 = 0;
pub const HV_RNG_STATE_CONFIGURED: u32 = 1;
pub const HV_RNG_STATE_HEALTHCHECK: u32 = 2;
pub const HV_RNG_STATE_ERROR: u32 = 3;
pub const HV_RNG_NUM_CONTROL: usize = 4;

extern "C" {
    pub fn sun4v_rng_get_diag_ctl() -> c_ulong;
    pub fn sun4v_rng_ctl_read_v1(ctl_regs_ra: c_ulong, state: *mut c_ulong, tick_delta: *mut c_ulong) -> c_ulong;
    pub fn sun4v_rng_ctl_read_v2(ctl_regs_ra: c_ulong, unit: c_ulong, state: *mut c_ulong, tick_delta: *mut c_ulong, watchdog: *mut c_ulong, write_status: *mut c_ulong) -> c_ulong;
    pub fn sun4v_rng_ctl_write_v1(ctl_regs_ra: c_ulong, state: c_ulong, write_timeout: c_ulong, tick_delta: *mut c_ulong) -> c_ulong;
    pub fn sun4v_rng_ctl_write_v2(ctl_regs_ra: c_ulong, state: c_ulong, write_timeout: c_ulong, unit: c_ulong) -> c_ulong;
    pub fn sun4v_rng_data_read_diag_v1(data_ra: c_ulong, len: c_ulong, tick_delta: *mut c_ulong) -> c_ulong;
    pub fn sun4v_rng_data_read_diag_v2(data_ra: c_ulong, len: c_ulong, unit: c_ulong, tick_delta: *mut c_ulong) -> c_ulong;
    pub fn sun4v_rng_data_read(data_ra: c_ulong, tick_delta: *mut c_ulong) -> c_ulong;
}

#[repr(C)]
pub enum n2rng_compat_id { N2_n2_rng, N2_vf_rng, N2_kt_rng, N2_m4_rng, N2_m7_rng }

#[repr(C)]
pub struct n2rng_template { pub id: n2rng_compat_id, pub multi_capable: c_int, pub chip_version: c_int }

#[repr(C)]
pub struct n2rng_unit { pub control: [u64; HV_RNG_NUM_CONTROL] }

pub const N2RNG_FLAG_MULTI: c_ulong = 0x00000001;
pub const N2RNG_FLAG_CONTROL: c_ulong = 0x00000002;
pub const N2RNG_FLAG_READY: c_ulong = 0x00000008;
pub const N2RNG_FLAG_SHUTDOWN: c_ulong = 0x00000010;
pub const N2RNG_FLAG_BUFFER_VALID: c_ulong = 0x00000020;

#[repr(C)]
pub struct n2rng {
    pub op: *mut platform_device,
    pub flags: c_ulong,
    pub data: *mut n2rng_template,
    pub num_units: c_int,
    pub units: *mut n2rng_unit,
    pub hwrng: hwrng,
    pub buffer: u32,
    pub hvapi_major: c_ulong,
    pub hvapi_minor: c_ulong,
    pub work: delayed_work,
    pub hv_state: c_ulong,
    pub health_check_sec: c_ulong,
    pub accum_cycles: c_ulong,
    pub wd_timeo: c_ulong,
    pub scratch_control: [u64; HV_RNG_NUM_CONTROL],
    pub test_data: u64,
    pub test_control: [u64; HV_RNG_NUM_CONTROL],
    pub test_buffer: [u64; 8],
}

pub const N2RNG_HEALTH_CHECK_SEC_DEFAULT: c_ulong = 0;
pub const N2RNG_ACCUM_CYCLES_DEFAULT: c_ulong = 2048;
pub const N2RNG_WD_TIMEO_DEFAULT: c_ulong = 0;
pub const RNG_v1_SELFTEST_TICKS: u64 = 38859;
pub const RNG_v1_SELFTEST_VAL: u64 = 0xB8820C7BD387E32C;
pub const RNG_v2_SELFTEST_TICKS: u64 = 64;
pub const RNG_v2_SELFTEST_VAL: u64 = 0xffffffffffffffff;
pub const SELFTEST_POLY: u64 = 0x231DCEE91262B8A3;
pub const SELFTEST_MATCH_GOAL: u32 = 6;
pub const SELFTEST_LOOPS_MAX: u32 = 40000;
pub const SELFTEST_BUFFER_WORDS: usize = 8;
pub const N2RNG_BLOCK_LIMIT: u32 = 60000;
pub const N2RNG_BUSY_LIMIT: u32 = 100;
pub const N2RNG_HCHECK_LIMIT: u32 = 100;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
