// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2025 Linaro Ltd.
 */

// External kernel dependencies supplied by the surrounding implementation.

const ACPM_DVFS_ID: u32 = 0x0000_0fff;
const ACPM_DVFS_REQ_TYPE: u32 = 0x0000_ffff;

const ACPM_DVFS_FREQ_REQ: u32 = 0;
const ACPM_DVFS_FREQ_GET: u32 = 1;
const HZ_PER_KHZ: u32 = 1000;

#[repr(C)]
pub struct acpm_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpm_xfer {
    pub txd: *mut u32,
    pub txd_size: usize,
    pub channel: u32,
    pub wait: bool,
    pub rxd: [u32; 4],
}

unsafe extern "C" {
    fn acpm_set_xfer(
        xfer: *mut acpm_xfer,
        cmd: *mut u32,
        len: usize,
        acpm_chan_id: u32,
        wait: bool,
    );
    fn acpm_do_xfer(handle: *mut acpm_handle, xfer: *mut acpm_xfer) -> i32;
    fn ktime_get() -> i64;
    fn ktime_to_ms(value: i64) -> i64;
}

#[inline]
fn field_prep(mask: u32, value: u32) -> u32 {
    (value << mask.trailing_zeros()) & mask
}

unsafe fn acpm_dvfs_init_set_rate_cmd(
    cmd: *mut u32,
    clk_id: u32,
    rate: u64,
) {
    (*cmd.add(0)) = field_prep(ACPM_DVFS_ID, clk_id);
    (*cmd.add(1)) = (rate / HZ_PER_KHZ as u64) as u32;
    (*cmd.add(2)) = field_prep(ACPM_DVFS_REQ_TYPE, ACPM_DVFS_FREQ_REQ);
    (*cmd.add(3)) = ktime_to_ms(ktime_get()) as u32;
}

pub unsafe fn acpm_dvfs_set_rate(
    handle: *mut acpm_handle,
    acpm_chan_id: u32,
    clk_id: u32,
    rate: u64,
) -> i32 {
    let mut xfer: acpm_xfer = core::mem::zeroed();
    let mut cmd: [u32; 4] = [0; 4];

    acpm_dvfs_init_set_rate_cmd(cmd.as_mut_ptr(), clk_id, rate);
    acpm_set_xfer(
        &mut xfer,
        cmd.as_mut_ptr(),
        cmd.len(),
        acpm_chan_id,
        false,
    );

    acpm_do_xfer(handle, &mut xfer)
}

unsafe fn acpm_dvfs_init_get_rate_cmd(cmd: *mut u32, clk_id: u32) {
    (*cmd.add(0)) = field_prep(ACPM_DVFS_ID, clk_id);
    (*cmd.add(2)) = field_prep(ACPM_DVFS_REQ_TYPE, ACPM_DVFS_FREQ_GET);
    (*cmd.add(3)) = ktime_to_ms(ktime_get()) as u32;
}

pub unsafe fn acpm_dvfs_get_rate(
    handle: *mut acpm_handle,
    acpm_chan_id: u32,
    clk_id: u32,
) -> u64 {
    let mut xfer: acpm_xfer = core::mem::uninitialized();
    let mut cmd: [u32; 4] = [0; 4];
    let ret: i32;

    acpm_dvfs_init_get_rate_cmd(cmd.as_mut_ptr(), clk_id);
    acpm_set_xfer(
        &mut xfer,
        cmd.as_mut_ptr(),
        cmd.len(),
        acpm_chan_id,
        true,
    );

    ret = acpm_do_xfer(handle, &mut xfer);
    if ret != 0 {
        return 0;
    }

    xfer.rxd[1] as u64 * HZ_PER_KHZ as u64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
