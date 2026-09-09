/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2019 Mellanox Technologies. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external here: linux/bits.h, linux/kernel.h, linux/module.h,
// linux/types.h, and linux/workqueue.h.

pub const NET_DIM_PARAMS_NUM_PROFILES: usize = 5;
pub const NET_DIM_DEFAULT_RX_CQ_PKTS_FROM_EQE: u32 = 256;
pub const NET_DIM_DEFAULT_TX_CQ_PKTS_FROM_EQE: u32 = 128;
pub const NET_DIM_DEF_PROFILE_CQE: u32 = 1;
pub const NET_DIM_DEF_PROFILE_EQE: u32 = 1;

pub const DIM_NEVENTS: u32 = 64;

macro_rules! is_significant_diff {
    ($val:expr, $ref_:expr) => {
        ($ref_ != 0) && ((100u64.wrapping_mul(($val - $ref_).abs() as u64) / $ref_ as u64) > 10)
    };
}

macro_rules! bit_gap {
    ($bits:expr, $end:expr, $start:expr) => {
        (($end - $start).wrapping_add(1u64 << $bits) & ((1u64 << $bits) - 1))
    };
}

#[repr(C)]
pub struct dim_cq_moder {
    pub usec: u16,
    pub pkts: u16,
    pub comps: u16,
    pub cq_period_mode: u8,
    pub rcu: rcu_head,
}

pub const DIM_PROFILE_RX: u8 = 1 << 0;
pub const DIM_PROFILE_TX: u8 = 1 << 1;
pub const DIM_COALESCE_USEC: u8 = 1 << 0;
pub const DIM_COALESCE_PKTS: u8 = 1 << 1;
pub const DIM_COALESCE_COMPS: u8 = 1 << 2;

#[repr(C)]
pub struct dim_irq_moder {
    pub profile_flags: u8,
    pub coal_flags: u8,
    pub dim_rx_mode: u8,
    pub dim_tx_mode: u8,
    pub rx_profile: *mut dim_cq_moder,
    pub tx_profile: *mut dim_cq_moder,
    pub rx_dim_work: Option<unsafe extern "C" fn(work: *mut work_struct)>,
    pub tx_dim_work: Option<unsafe extern "C" fn(work: *mut work_struct)>,
}

#[repr(C)]
pub struct dim_sample {
    pub time: ktime_t,
    pub pkt_ctr: u32,
    pub byte_ctr: u32,
    pub event_ctr: u16,
    pub comp_ctr: u32,
}

#[repr(C)]
pub struct dim_stats {
    pub ppms: i32,
    pub bpms: i32,
    pub epms: i32,
    pub cpms: i32,
    pub cpe_ratio: i32,
}

#[repr(C)]
pub struct dim {
    pub state: u8,
    pub prev_stats: dim_stats,
    pub start_sample: dim_sample,
    pub measuring_sample: dim_sample,
    pub work: work_struct,
    pub priv_: *mut core::ffi::c_void,
    pub profile_ix: u8,
    pub mode: u8,
    pub tune_state: u8,
    pub steps_right: u8,
    pub steps_left: u8,
    pub tired: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum dim_cq_period_mode {
    DIM_CQ_PERIOD_MODE_START_FROM_EQE = 0x0,
    DIM_CQ_PERIOD_MODE_START_FROM_CQE = 0x1,
    DIM_CQ_PERIOD_NUM_MODES,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum dim_state {
    DIM_START_MEASURE,
    DIM_MEASURE_IN_PROGRESS,
    DIM_APPLY_NEW_PROFILE,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum dim_tune_state {
    DIM_PARKING_ON_TOP,
    DIM_PARKING_TIRED,
    DIM_GOING_RIGHT,
    DIM_GOING_LEFT,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum dim_stats_state {
    DIM_STATS_WORSE,
    DIM_STATS_SAME,
    DIM_STATS_BETTER,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum dim_step_result {
    DIM_STEPPED,
    DIM_TOO_TIRED,
    DIM_ON_EDGE,
}

extern "C" {
    pub fn net_dim_init_irq_moder(dev: *mut net_device, profile_flags: u8, coal_flags: u8,
        rx_mode: u8, tx_mode: u8,
        rx_dim_work: Option<unsafe extern "C" fn(*mut work_struct)>,
        tx_dim_work: Option<unsafe extern "C" fn(*mut work_struct)>) -> i32;
    pub fn net_dim_free_irq_moder(dev: *mut net_device);
    pub fn net_dim_setting(dev: *mut net_device, dim: *mut dim, is_tx: bool);
    pub fn net_dim_work_cancel(dim: *mut dim);
    pub fn net_dim_get_rx_irq_moder(dev: *mut net_device, dim: *mut dim) -> dim_cq_moder;
    pub fn net_dim_get_tx_irq_moder(dev: *mut net_device, dim: *mut dim) -> dim_cq_moder;
    pub fn net_dim_set_rx_mode(dev: *mut net_device, rx_mode: u8);
    pub fn net_dim_set_tx_mode(dev: *mut net_device, tx_mode: u8);
    pub fn dim_on_top(dim: *mut dim) -> bool;
    pub fn dim_turn(dim: *mut dim);
    pub fn dim_park_on_top(dim: *mut dim);
    pub fn dim_park_tired(dim: *mut dim);
    pub fn dim_calc_stats(start: *const dim_sample, end: *const dim_sample,
        curr_stats: *mut dim_stats) -> bool;
    pub fn net_dim_get_rx_moderation(cq_period_mode: u8, ix: i32) -> dim_cq_moder;
    pub fn net_dim_get_def_rx_moderation(cq_period_mode: u8) -> dim_cq_moder;
    pub fn net_dim_get_tx_moderation(cq_period_mode: u8, ix: i32) -> dim_cq_moder;
    pub fn net_dim_get_def_tx_moderation(cq_period_mode: u8) -> dim_cq_moder;
    pub fn net_dim(dim: *mut dim, end_sample: *const dim_sample);
    pub fn rdma_dim(dim: *mut dim, completions: u64);
}

#[inline]
pub unsafe fn dim_update_sample(event_ctr: u16, packets: u64, bytes: u64, s: *mut dim_sample) {
    (*s).time = ktime_get();
    (*s).pkt_ctr = packets as u32;
    (*s).byte_ctr = bytes as u32;
    (*s).event_ctr = event_ctr;
}

#[inline]
pub unsafe fn dim_update_sample_with_comps(event_ctr: u16, packets: u64, bytes: u64,
    comps: u64, s: *mut dim_sample) {
    dim_update_sample(event_ctr, packets, bytes, s);
    (*s).comp_ctr = comps as u32;
}

pub const RDMA_DIM_PARAMS_NUM_PROFILES: usize = 9;
pub const RDMA_DIM_START_PROFILE: usize = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
