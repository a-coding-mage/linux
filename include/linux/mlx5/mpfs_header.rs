/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
 * Copyright (c) 2021 Mellanox Technologies Ltd.
 */

// C header guard: _MLX5_MPFS_

#[repr(C)]
pub struct mlx5_core_dev {
    _private: [u8; 0],
}

// Build-time condition preserved from CONFIG_MLX5_MPFS.
#[cfg(feature = "CONFIG_MLX5_MPFS")]
unsafe extern "C" {
    pub fn mlx5_mpfs_add_mac(dev: *mut mlx5_core_dev, mac: *mut u8) -> i32;
    pub fn mlx5_mpfs_del_mac(dev: *mut mlx5_core_dev, mac: *mut u8) -> i32;
}

// Fallback when CONFIG_MLX5_MPFS is not enabled.
#[cfg(not(feature = "CONFIG_MLX5_MPFS"))]
#[inline]
pub unsafe fn mlx5_mpfs_add_mac(_dev: *mut mlx5_core_dev, _mac: *mut u8) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_MLX5_MPFS"))]
#[inline]
pub unsafe fn mlx5_mpfs_del_mac(_dev: *mut mlx5_core_dev, _mac: *mut u8) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
