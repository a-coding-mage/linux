/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved. */

// Translated from linux/mlx5/lag.h.
// The original <linux/types.h> dependency supplies the C integer types;
// `u16` is represented by Rust's built-in unsigned 16-bit integer type.

#[repr(C)]
pub struct mlx5_core_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mlx5_flow_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mlx5_flow_table_attr {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mlx5_lag_demux_init(
        dev: *mut mlx5_core_dev,
        ft_attr: *mut mlx5_flow_table_attr,
    ) -> ::core::ffi::c_int;

    pub fn mlx5_lag_demux_cleanup(dev: *mut mlx5_core_dev);

    pub fn mlx5_lag_demux_rule_add(
        dev: *mut mlx5_core_dev,
        vport_num: u16,
        vport_index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn mlx5_lag_demux_rule_del(
        dev: *mut mlx5_core_dev,
        vport_index: ::core::ffi::c_int,
    );

    pub fn mlx5_lag_get_dev_seq(dev: *mut mlx5_core_dev) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
