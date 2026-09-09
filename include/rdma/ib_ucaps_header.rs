/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES. All rights reserved
 */

// Translated from the C header `_IB_UCAPS_H_`.

/// Equivalent of the C `UCAP_ENABLED(ucaps, type)` macro.
#[inline]
pub const unsafe fn ucap_enabled(ucaps: u32, type_: u32) -> i32 {
    ((ucaps & (1u32 << type_)) != 0) as i32
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rdma_user_cap {
    RDMA_UCAP_MLX5_CTRL_LOCAL,
    RDMA_UCAP_MLX5_CTRL_OTHER_VHCA,
    RDMA_UCAP_MAX,
}

unsafe extern "C" {
    pub fn ib_get_ucaps(fds: *mut i32, fd_count: i32, idx_mask: *mut u64) -> i32;
}

// The following conditional corresponds to IS_ENABLED(CONFIG_INFINIBAND_USER_ACCESS).
#[cfg(feature = "CONFIG_INFINIBAND_USER_ACCESS")]
unsafe extern "C" {
    pub fn ib_create_ucap(type_: rdma_user_cap) -> i32;
    pub fn ib_remove_ucap(type_: rdma_user_cap);
}

#[cfg(not(feature = "CONFIG_INFINIBAND_USER_ACCESS"))]
#[inline]
pub unsafe fn ib_create_ucap(_type_: rdma_user_cap) -> i32 {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_INFINIBAND_USER_ACCESS"))]
#[inline]
pub unsafe fn ib_remove_ucap(_type_: rdma_user_cap) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
