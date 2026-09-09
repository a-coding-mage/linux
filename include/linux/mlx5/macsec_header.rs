/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. */

/* Translated from C header guard MLX5_MACSEC_H. */

/* The declarations below are present only when CONFIG_MLX5_MACSEC is enabled. */

#[repr(C)]
pub struct mlx5_macsec_event_data {
    pub macsec_fs: *mut mlx5_macsec_fs,
    pub macdev: *mut core::ffi::c_void,
    pub fs_id: u32,
    pub is_tx: bool,
}

extern "C" {
    pub fn mlx5_macsec_add_roce_rule(
        macdev: *mut core::ffi::c_void,
        addr: *const sockaddr,
        gid_idx: u16,
        tx_rules_list: *mut list_head,
        rx_rules_list: *mut list_head,
        macsec_fs: *mut mlx5_macsec_fs,
    ) -> core::ffi::c_int;

    pub fn mlx5_macsec_del_roce_rule(
        gid_idx: u16,
        macsec_fs: *mut mlx5_macsec_fs,
        tx_rules_list: *mut list_head,
        rx_rules_list: *mut list_head,
    );

    pub fn mlx5_macsec_add_roce_sa_rules(
        fs_id: u32,
        addr: *const sockaddr,
        gid_idx: u16,
        tx_rules_list: *mut list_head,
        rx_rules_list: *mut list_head,
        macsec_fs: *mut mlx5_macsec_fs,
        is_tx: bool,
    );

    pub fn mlx5_macsec_del_roce_sa_rules(
        fs_id: u32,
        macsec_fs: *mut mlx5_macsec_fs,
        tx_rules_list: *mut list_head,
        rx_rules_list: *mut list_head,
        is_tx: bool,
    );
}

/* External types supplied by the surrounding kernel translation. */
pub enum mlx5_macsec_fs {}
pub enum sockaddr {}
pub enum list_head {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
