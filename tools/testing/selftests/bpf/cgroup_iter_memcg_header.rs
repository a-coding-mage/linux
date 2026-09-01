// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
pub struct memcg_query {
    /* some node_stat_item's */
    pub nr_anon_mapped: core::ffi::c_ulong,
    pub nr_shmem: core::ffi::c_ulong,
    pub nr_file_pages: core::ffi::c_ulong,
    pub nr_file_mapped: core::ffi::c_ulong,
    /* some vm_event_item */
    pub pgfault: core::ffi::c_ulong,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
