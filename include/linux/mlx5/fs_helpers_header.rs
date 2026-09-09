/*
 * Copyright (c) 2018, Mellanox Technologies. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses. You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the OpenIB.org BSD
 * license below.
 */

// Dependency supplied by the mlx5 interface definitions.

pub const MLX5_FS_IPV4_VERSION: i32 = 4;
pub const MLX5_FS_IPV6_VERSION: i32 = 6;

pub unsafe fn _mlx5_fs_is_outer_ipv_flow(
    mdev: *mut mlx5_core_dev,
    match_c: *const u32,
    match_v: *const u32,
    version: i32,
) -> bool {
    let match_ipv: i32 = MLX5_CAP_FLOWTABLE_NIC_RX!(mdev, ft_field_support.outer_ip_version);
    let headers_c: *const core::ffi::c_void =
        MLX5_ADDR_OF!(fte_match_param, match_c, outer_headers);
    let headers_v: *const core::ffi::c_void =
        MLX5_ADDR_OF!(fte_match_param, match_v, outer_headers);

    if match_ipv == 0 {
        let ethertype: u16;

        match version {
            MLX5_FS_IPV4_VERSION => {
                ethertype = ETH_P_IP;
            }
            MLX5_FS_IPV6_VERSION => {
                ethertype = ETH_P_IPV6;
            }
            _ => return false,
        }

        return MLX5_GET!(fte_match_set_lyr_2_4, headers_c, ethertype) == 0xffff
            && MLX5_GET!(fte_match_set_lyr_2_4, headers_v, ethertype)
                == ethertype;
    }

    MLX5_GET!(fte_match_set_lyr_2_4, headers_c, ip_version) == 0xf
        && MLX5_GET!(fte_match_set_lyr_2_4, headers_v, ip_version) == version
}

pub unsafe fn mlx5_fs_is_outer_ipv4_flow(
    mdev: *mut mlx5_core_dev,
    match_c: *const u32,
    match_v: *const u32,
) -> bool {
    _mlx5_fs_is_outer_ipv_flow(mdev, match_c, match_v, MLX5_FS_IPV4_VERSION)
}

pub unsafe fn mlx5_fs_is_outer_ipv6_flow(
    mdev: *mut mlx5_core_dev,
    match_c: *const u32,
    match_v: *const u32,
) -> bool {
    _mlx5_fs_is_outer_ipv_flow(mdev, match_c, match_v, MLX5_FS_IPV6_VERSION)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
