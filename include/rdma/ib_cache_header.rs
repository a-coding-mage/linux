/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2004 Topspin Communications.  All rights reserved.
 * Copyright (c) 2005 Intel Corporation. All rights reserved.
 * Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
 */

// Translated from <rdma/ib_cache.h>.

extern "C" {
    pub fn rdma_query_gid(
        device: *mut ib_device,
        port_num: u32,
        index: ::core::ffi::c_int,
        gid: *mut ib_gid,
    ) -> ::core::ffi::c_int;

    pub fn rdma_read_gid_hw_context(attr: *const ib_gid_attr) -> *mut ::core::ffi::c_void;

    pub fn rdma_find_gid(
        device: *mut ib_device,
        gid: *const ib_gid,
        gid_type: ib_gid_type,
        ndev: *mut net_device,
    ) -> *const ib_gid_attr;

    pub fn rdma_find_gid_by_port(
        ib_dev: *mut ib_device,
        gid: *const ib_gid,
        gid_type: ib_gid_type,
        port: u32,
        ndev: *mut net_device,
    ) -> *const ib_gid_attr;

    pub fn rdma_find_gid_by_filter(
        device: *mut ib_device,
        gid: *const ib_gid,
        port_num: u32,
        filter: Option<unsafe extern "C" fn(
            gid: *const ib_gid,
            attr: *const ib_gid_attr,
            context: *mut ::core::ffi::c_void,
        ) -> bool>,
        context: *mut ::core::ffi::c_void,
    ) -> *const ib_gid_attr;

    pub fn rdma_read_gid_l2_fields(
        attr: *const ib_gid_attr,
        vlan_id: *mut u16,
        smac: *mut u8,
    ) -> ::core::ffi::c_int;

    pub fn rdma_read_gid_attr_ndev_rcu(attr: *const ib_gid_attr) -> *mut net_device;

    /**
     * ib_get_cached_pkey - Returns a cached PKey table entry
     * @device_handle: The device to query.
     * @port_num: The port number of the device to query.
     * @index: The index into the cached PKey table to query.
     * @pkey: The PKey value found at the specified index.
     *
     * ib_get_cached_pkey() fetches the specified PKey table entry stored in
     * the local software cache.
     */
    pub fn ib_get_cached_pkey(
        device_handle: *mut ib_device,
        port_num: u32,
        index: ::core::ffi::c_int,
        pkey: *mut u16,
    ) -> ::core::ffi::c_int;

    /**
     * ib_find_cached_pkey - Returns the PKey table index where a specified
     *   PKey value occurs.
     * @device: The device to query.
     * @port_num: The port number of the device to search for the PKey.
     * @pkey: The PKey value to search for.
     * @index: The index into the cached PKey table where the PKey was found.
     *
     * ib_find_cached_pkey() searches the specified PKey table in
     * the local software cache.
     */
    pub fn ib_find_cached_pkey(
        device: *mut ib_device,
        port_num: u32,
        pkey: u16,
        index: *mut u16,
    ) -> ::core::ffi::c_int;

    /**
     * ib_get_cached_lmc - Returns a cached lmc table entry
     * @device: The device to query.
     * @port_num: The port number of the device to query.
     * @lmc: The lmc value for the specified port for that device.
     *
     * ib_get_cached_lmc() fetches the specified lmc table entry stored in
     * the local software cache.
     */
    pub fn ib_get_cached_lmc(
        device: *mut ib_device,
        port_num: u32,
        lmc: *mut u8,
    ) -> ::core::ffi::c_int;

    /**
     * ib_get_cached_port_state - Returns a cached port state table entry
     * @device: The device to query.
     * @port_num: The port number of the device to query.
     * @port_active: port_state for the specified port for that device.
     *
     * ib_get_cached_port_state() fetches the specified port_state table entry stored in
     * the local software cache.
     */
    pub fn ib_get_cached_port_state(
        device: *mut ib_device,
        port_num: u32,
        port_active: *mut ib_port_state,
    ) -> ::core::ffi::c_int;

    pub fn rdma_is_zero_gid(gid: *const ib_gid) -> bool;
    pub fn rdma_get_gid_attr(
        device: *mut ib_device,
        port_num: u32,
        index: ::core::ffi::c_int,
    ) -> *const ib_gid_attr;
    pub fn rdma_put_gid_attr(attr: *const ib_gid_attr);
    pub fn rdma_hold_gid_attr(attr: *const ib_gid_attr);
    pub fn rdma_query_gid_table(
        device: *mut ib_device,
        entries: *mut ib_uverbs_gid_entry,
        max_entries: usize,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
