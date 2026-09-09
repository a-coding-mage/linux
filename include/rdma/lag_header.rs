/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2020 Mellanox Technologies. All rights reserved.
 */

// Dependency intent: declarations supplied by <net/lag.h> are referenced here.

#[repr(C)]
pub struct ib_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rdma_ah_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub enum rdma_lag_flags {
    RDMA_LAG_FLAGS_HASH_ALL_SLAVES = 1 << 0,
}

unsafe extern "C" {
    pub fn rdma_lag_put_ah_roce_slave(xmit_slave: *mut net_device);
    pub fn rdma_lag_get_ah_roce_slave(
        device: *mut ib_device,
        ah_attr: *mut rdma_ah_attr,
        flags: gfp_t,
    ) -> *mut net_device;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
