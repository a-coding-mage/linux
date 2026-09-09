/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2019 Mellanox Technologies. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/RDMA translation.

#[repr(C)]
pub struct auto_mode_param {
    pub qp_type: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct rdma_counter_mode {
    pub mode: enum rdma_nl_counter_mode,
    pub mask: enum rdma_nl_counter_mask,
    pub param: auto_mode_param,
    pub bind_opcnt: bool,
}

#[repr(C)]
pub struct rdma_port_counter {
    pub mode: rdma_counter_mode,
    pub hstats: *mut rdma_hw_stats,
    pub num_counters: u32,
    pub lock: mutex,
}

#[repr(C)]
pub struct rdma_counter {
    pub res: rdma_restrack_entry,
    pub device: *mut ib_device,
    pub id: u32,
    pub kref: kref,
    pub mode: rdma_counter_mode,
    pub lock: mutex,
    pub stats: *mut rdma_hw_stats,
    pub port: u32,
}

extern "C" {
    pub fn rdma_counter_init(dev: *mut ib_device);
    pub fn rdma_counter_release(dev: *mut ib_device);
    pub fn rdma_counter_set_auto_mode(
        dev: *mut ib_device,
        port: u32,
        mask: enum rdma_nl_counter_mask,
        bind_opcnt: bool,
        extack: *mut netlink_ext_ack,
    ) -> ::std::os::raw::c_int;
    pub fn rdma_counter_bind_qp_auto(qp: *mut ib_qp, port: u32) -> ::std::os::raw::c_int;
    pub fn rdma_counter_unbind_qp(
        qp: *mut ib_qp,
        port: u32,
        force: bool,
    ) -> ::std::os::raw::c_int;

    pub fn rdma_counter_query_stats(counter: *mut rdma_counter) -> ::std::os::raw::c_int;
    pub fn rdma_counter_get_hwstat_value(dev: *mut ib_device, port: u32, index: u32) -> u64;
    pub fn rdma_counter_bind_qpn(
        dev: *mut ib_device,
        port: u32,
        qp_num: u32,
        counter_id: u32,
    ) -> ::std::os::raw::c_int;
    pub fn rdma_counter_bind_qpn_alloc(
        dev: *mut ib_device,
        port: u32,
        qp_num: u32,
        counter_id: *mut u32,
    ) -> ::std::os::raw::c_int;
    pub fn rdma_counter_unbind_qpn(
        dev: *mut ib_device,
        port: u32,
        qp_num: u32,
        counter_id: u32,
    ) -> ::std::os::raw::c_int;
    pub fn rdma_counter_get_mode(
        dev: *mut ib_device,
        port: u32,
        mode: *mut enum rdma_nl_counter_mode,
        mask: *mut enum rdma_nl_counter_mask,
        opcnt: *mut bool,
    ) -> ::std::os::raw::c_int;

    pub fn rdma_counter_modify(
        dev: *mut ib_device,
        port: u32,
        index: u32,
        enable: bool,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
