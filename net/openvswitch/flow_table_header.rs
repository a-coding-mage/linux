/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2007-2013 Nicira, Inc.
 */

// Translated from flow_table.h. Linux/Open vSwitch dependencies are supplied externally.

#[repr(C)]
pub struct mask_cache_entry {
    pub skb_hash: u32,
    pub mask_index: u32,
}

#[repr(C)]
pub struct mask_cache {
    pub rcu: rcu_head,
    pub cache_size: u32, /* Must be ^2 value. */
    pub mask_cache: *mut mask_cache_entry,
}

#[repr(C)]
pub struct mask_count {
    pub index: i32,
    pub counter: u64,
}

#[repr(C)]
pub struct mask_array_stats {
    pub syncp: u64_stats_sync,
    pub usage_cntrs: [u64; 0],
}

#[repr(C)]
pub struct mask_array {
    pub rcu: rcu_head,
    pub count: i32,
    pub max: i32,
    pub masks_usage_stats: *mut mask_array_stats,
    pub masks_usage_zero_cntr: *mut u64,
    pub masks: [*mut sw_flow_mask; 0],
}

#[repr(C)]
pub struct table_instance {
    pub buckets: *mut hlist_head,
    pub n_buckets: core::ffi::c_uint,
    pub rcu: rcu_head,
    pub node_ver: i32,
    pub hash_seed: u32,
}

#[repr(C)]
pub struct flow_table {
    pub ti: *mut table_instance,
    pub ufid_ti: *mut table_instance,
    pub mask_cache: *mut mask_cache,
    pub mask_array: *mut mask_array,
    pub last_rehash: core::ffi::c_ulong,
    pub count: core::ffi::c_uint,
    pub ufid_count: core::ffi::c_uint,
}

extern "C" {
    pub static mut flow_stats_cache: *mut kmem_cache;

    pub fn ovs_flow_init() -> i32;
    pub fn ovs_flow_exit();

    pub fn ovs_flow_alloc() -> *mut sw_flow;
    pub fn ovs_flow_free(flow: *mut sw_flow, deferred: bool);

    pub fn ovs_flow_tbl_init(table: *mut flow_table) -> i32;
    pub fn ovs_flow_tbl_count(table: *const flow_table) -> i32;
    pub fn ovs_flow_tbl_destroy(table: *mut flow_table);
    pub fn ovs_flow_tbl_flush(flow_table: *mut flow_table) -> i32;

    pub fn ovs_flow_tbl_insert(
        table: *mut flow_table,
        flow: *mut sw_flow,
        mask: *const sw_flow_mask,
    ) -> i32;
    pub fn ovs_flow_tbl_remove(table: *mut flow_table, flow: *mut sw_flow);
    pub fn ovs_flow_tbl_num_masks(table: *const flow_table) -> i32;
    pub fn ovs_flow_tbl_masks_cache_size(table: *const flow_table) -> u32;
    pub fn ovs_flow_tbl_masks_cache_resize(table: *mut flow_table, size: u32) -> i32;
    pub fn ovs_flow_tbl_dump_next(
        table: *mut table_instance,
        bucket: *mut u32,
        idx: *mut u32,
    ) -> *mut sw_flow;
    pub fn ovs_flow_tbl_lookup_stats(
        table: *mut flow_table,
        key: *const sw_flow_key,
        skb_hash: u32,
        n_mask_hit: *mut u32,
        n_cache_hit: *mut u32,
    ) -> *mut sw_flow;
    pub fn ovs_flow_tbl_lookup(table: *mut flow_table, key: *const sw_flow_key) -> *mut sw_flow;
    pub fn ovs_flow_tbl_lookup_exact(
        tbl: *mut flow_table,
        r#match: *const sw_flow_match,
    ) -> *mut sw_flow;
    pub fn ovs_flow_tbl_lookup_ufid(
        table: *mut flow_table,
        ufid: *const sw_flow_id,
    ) -> *mut sw_flow;

    pub fn ovs_flow_cmp(flow: *const sw_flow, match_: *const sw_flow_match) -> bool;

    pub fn ovs_flow_mask_key(
        dst: *mut sw_flow_key,
        src: *const sw_flow_key,
        full: bool,
        mask: *const sw_flow_mask,
    );

    pub fn ovs_flow_masks_rebalance(table: *mut flow_table);
    pub fn table_instance_flow_flush(
        table: *mut flow_table,
        ti: *mut table_instance,
        ufid_ti: *mut table_instance,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
