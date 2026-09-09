/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2017 Nicira, Inc.
 */

// Declarations supplied by the corresponding kernel/Open vSwitch headers are
// intentionally left as external Rust dependencies.

pub const DP_MAX_BANDS: u32 = 1;
pub const DP_METER_ARRAY_SIZE_MIN: u64 = 1u64 << 10;
pub const DP_METER_NUM_MAX: u64 = 200000u64;

#[repr(C)]
pub struct dp_meter_band {
    pub type_: u32,
    pub rate: u32,
    pub burst_size: u32,
    pub bucket: u64, // 1/1000 packets, or in bits
    pub stats: ovs_flow_stats,
}

#[repr(C)]
pub struct dp_meter {
    pub lock: spinlock_t, // Per meter lock
    pub rcu: rcu_head,
    pub id: u32,
    // C bit-fields: kbps:1, keep_stats:1; remaining bits are padding.
    pub kbps_keep_stats: u16,
    pub n_bands: u16,
    pub max_delta_t: u32,
    pub used: u64,
    pub stats: ovs_flow_stats,
    pub bands: [dp_meter_band; 0],
}

#[repr(C)]
pub struct dp_meter_instance {
    pub rcu: rcu_head,
    pub n_meters: u32,
    pub dp_meters: [*mut dp_meter; 0],
}

#[repr(C)]
pub struct dp_meter_table {
    pub ti: *mut dp_meter_instance,
    pub count: u32,
    pub max_meters_allowed: u32,
}

unsafe extern "C" {
    pub static mut dp_meter_genl_family: genl_family;

    pub fn ovs_meters_init(dp: *mut datapath) -> i32;
    pub fn ovs_meters_exit(dp: *mut datapath);
    pub fn ovs_meter_execute(
        dp: *mut datapath,
        skb: *mut sk_buff,
        key: *mut sw_flow_key,
        meter_id: u32,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
