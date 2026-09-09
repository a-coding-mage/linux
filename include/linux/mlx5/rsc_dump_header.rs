/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2020 Mellanox Technologies inc. */

// Dependency supplied by linux/mlx5/driver.h.

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum mlx5_sgmt_type {
    MLX5_SGMT_TYPE_HW_CQPC,
    MLX5_SGMT_TYPE_HW_SQPC,
    MLX5_SGMT_TYPE_HW_RQPC,
    MLX5_SGMT_TYPE_FULL_SRQC,
    MLX5_SGMT_TYPE_FULL_CQC,
    MLX5_SGMT_TYPE_FULL_EQC,
    MLX5_SGMT_TYPE_FULL_QPC,
    MLX5_SGMT_TYPE_SND_BUFF,
    MLX5_SGMT_TYPE_RCV_BUFF,
    MLX5_SGMT_TYPE_SRQ_BUFF,
    MLX5_SGMT_TYPE_CQ_BUFF,
    MLX5_SGMT_TYPE_EQ_BUFF,
    MLX5_SGMT_TYPE_SX_SLICE,
    MLX5_SGMT_TYPE_SX_SLICE_ALL,
    MLX5_SGMT_TYPE_RDB,
    MLX5_SGMT_TYPE_RX_SLICE_ALL,
    MLX5_SGMT_TYPE_PRM_QUERY_QP,
    MLX5_SGMT_TYPE_PRM_QUERY_CQ,
    MLX5_SGMT_TYPE_PRM_QUERY_MKEY,
    MLX5_SGMT_TYPE_MENU,
    // Keep last.
    MLX5_SGMT_TYPE_TERMINATE,
    MLX5_SGMT_TYPE_NUM,
}

#[repr(C)]
pub struct mlx5_rsc_key {
    pub rsc: mlx5_sgmt_type,
    pub index1: ::core::ffi::c_int,
    pub index2: ::core::ffi::c_int,
    pub num_of_obj1: ::core::ffi::c_int,
    pub num_of_obj2: ::core::ffi::c_int,
    pub size: ::core::ffi::c_int,
}

pub struct mlx5_rsc_dump_cmd {
    _private: [u8; 0],
}

extern "C" {
    pub fn mlx5_rsc_dump_cmd_create(
        dev: *mut mlx5_core_dev,
        key: *mut mlx5_rsc_key,
    ) -> *mut mlx5_rsc_dump_cmd;
    pub fn mlx5_rsc_dump_cmd_destroy(cmd: *mut mlx5_rsc_dump_cmd);
    pub fn mlx5_rsc_dump_next(
        dev: *mut mlx5_core_dev,
        cmd: *mut mlx5_rsc_dump_cmd,
        page: *mut page,
        size: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// Opaque types supplied by linux/mlx5/driver.h and the kernel headers.
pub struct mlx5_core_dev {
    _private: [u8; 0],
}

pub struct page {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
