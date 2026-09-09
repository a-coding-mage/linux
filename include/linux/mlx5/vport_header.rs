/*
 * Copyright (c) 2013-2015, Mellanox Technologies, Ltd.  All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// MLX5_VPORT_MANAGER(mdev) is dependency-defined as:
// MLX5_CAP_GEN(mdev, vport_group_manager) &&
// (MLX5_CAP_GEN(mdev, port_type) == MLX5_CAP_PORT_TYPE_ETH) &&
// mlx5_core_is_pf(mdev)

pub const MLX5_MAX_TX_SPEED_UNIT: u32 = 100;

pub const MLX5_CAP_INLINE_MODE_L2: i32 = 0;
pub const MLX5_CAP_INLINE_MODE_VPORT_CONTEXT: i32 = 1;
pub const MLX5_CAP_INLINE_MODE_NOT_REQUIRED: i32 = 2;

/* Vport number for each function must keep unchanged */
pub const MLX5_VPORT_HOST_PF: u16 = 0x0;
pub const MLX5_VPORT_FIRST_HOST_VF: u16 = 0x1;
pub const MLX5_VPORT_ECPF: u16 = 0xfffe;
pub const MLX5_VPORT_UPLINK: u16 = 0xffff;

extern "C" {
    pub fn mlx5_query_vport_state(mdev: *mut mlx5_core_dev, opmod: u8, vport: u16) -> u8;
    pub fn mlx5_modify_vport_admin_state(mdev: *mut mlx5_core_dev, opmod: u8, vport: u16, other_vport: u8, state: u8) -> i32;
    pub fn mlx5_query_vport_max_tx_speed(mdev: *mut mlx5_core_dev, op_mod: u8, vport: u16, other_vport: u8, max_tx_speed: *mut u32, state: *mut u8) -> i32;
    pub fn mlx5_modify_vport_max_tx_speed(mdev: *mut mlx5_core_dev, opmod: u8, vport: u16, other_vport: u8, max_tx_speed: u16) -> i32;
    pub fn mlx5_query_nic_vport_mac_address(mdev: *mut mlx5_core_dev, vport: u16, other: bool, addr: *mut u8) -> i32;
    pub fn mlx5_query_mac_address(mdev: *mut mlx5_core_dev, addr: *mut u8) -> i32;
    pub fn mlx5_query_nic_vport_min_inline(mdev: *mut mlx5_core_dev, vport: u16, min_inline: *mut u8) -> i32;
    pub fn mlx5_query_min_inline(mdev: *mut mlx5_core_dev, min_inline: *mut u8);
    pub fn mlx5_modify_nic_vport_min_inline(mdev: *mut mlx5_core_dev, vport: u16, min_inline: u8) -> i32;
    pub fn mlx5_modify_nic_vport_mac_address(dev: *mut mlx5_core_dev, vport: u16, addr: *const u8) -> i32;
    pub fn mlx5_query_nic_vport_mtu(mdev: *mut mlx5_core_dev, mtu: *mut u16) -> i32;
    pub fn mlx5_modify_nic_vport_mtu(mdev: *mut mlx5_core_dev, mtu: u16) -> i32;
    pub fn mlx5_query_nic_vport_system_image_guid(mdev: *mut mlx5_core_dev, system_image_guid: *mut u64) -> i32;
    pub fn mlx5_query_nic_vport_sd_group(mdev: *mut mlx5_core_dev, sd_group: *mut u8, sd_group_size: *mut u8) -> i32;
    pub fn mlx5_query_nic_vport_node_guid(mdev: *mut mlx5_core_dev, vport: u16, other_vport: bool, node_guid: *mut u64) -> i32;
    pub fn mlx5_modify_nic_vport_node_guid(mdev: *mut mlx5_core_dev, vport: u16, node_guid: u64) -> i32;
    pub fn mlx5_query_nic_vport_qkey_viol_cntr(mdev: *mut mlx5_core_dev, qkey_viol_cntr: *mut u16) -> i32;
    pub fn mlx5_query_hca_vport_gid(dev: *mut mlx5_core_dev, other_vport: u8, port_num: u8, vf_num: u16, gid_index: u16, gid: *mut ib_gid) -> i32;
    pub fn mlx5_query_hca_vport_pkey(dev: *mut mlx5_core_dev, other_vport: u8, port_num: u8, vf_num: u16, pkey_index: u16, pkey: *mut u16) -> i32;
    pub fn mlx5_query_hca_vport_context(dev: *mut mlx5_core_dev, other_vport: u8, port_num: u8, vf_num: u16, rep: *mut mlx5_hca_vport_context) -> i32;
    pub fn mlx5_query_hca_vport_system_image_guid(dev: *mut mlx5_core_dev, sys_image_guid: *mut u64) -> i32;
    pub fn mlx5_query_hca_vport_node_guid(dev: *mut mlx5_core_dev, node_guid: *mut u64) -> i32;
    pub fn mlx5_query_nic_vport_mac_list(dev: *mut mlx5_core_dev, vport: u16, list_type: mlx5_list_type, mac_list: *mut *mut [u8; ETH_ALEN], mac_list_size: *mut i32) -> i32;
    pub fn mlx5_modify_nic_vport_mac_list(dev: *mut mlx5_core_dev, list_type: mlx5_list_type, addr_list: *mut [u8; ETH_ALEN], list_size: i32) -> i32;
    pub fn mlx5_query_nic_vport_promisc(mdev: *mut mlx5_core_dev, vport: u16, promisc_uc: *mut i32, promisc_mc: *mut i32, promisc_all: *mut i32) -> i32;
    pub fn mlx5_modify_nic_vport_promisc(mdev: *mut mlx5_core_dev, promisc_uc: i32, promisc_mc: i32, promisc_all: i32) -> i32;
    pub fn mlx5_modify_nic_vport_vlans(dev: *mut mlx5_core_dev, vlans: *mut u16, list_size: i32) -> i32;
    pub fn mlx5_nic_vport_enable_roce(mdev: *mut mlx5_core_dev) -> i32;
    pub fn mlx5_nic_vport_disable_roce(mdev: *mut mlx5_core_dev) -> i32;
    pub fn mlx5_query_vport_down_stats(mdev: *mut mlx5_core_dev, vport: u16, other_vport: u8, rx_discard_vport_down: *mut u64, tx_discard_vport_down: *mut u64) -> i32;
    pub fn mlx5_core_query_vport_counter(dev: *mut mlx5_core_dev, other_vport: u8, vf: i32, port_num: u8, out: *mut core::ffi::c_void) -> i32;
    pub fn mlx5_core_modify_hca_vport_context(dev: *mut mlx5_core_dev, other_vport: u8, port_num: u8, vf: i32, req: *mut mlx5_hca_vport_context) -> i32;
    pub fn mlx5_nic_vport_update_local_lb(mdev: *mut mlx5_core_dev, enable: bool) -> i32;
    pub fn mlx5_nic_vport_query_local_lb(mdev: *mut mlx5_core_dev, status: *mut bool) -> i32;
    pub fn mlx5_nic_vport_affiliate_multiport(master_mdev: *mut mlx5_core_dev, port_mdev: *mut mlx5_core_dev) -> i32;
    pub fn mlx5_nic_vport_unaffiliate_multiport(port_mdev: *mut mlx5_core_dev) -> i32;
    pub fn mlx5_query_nic_system_image_guid(mdev: *mut mlx5_core_dev) -> u64;
    pub fn mlx5_vport_get_other_func_cap(dev: *mut mlx5_core_dev, vport: u16, out: *mut core::ffi::c_void, opmod: u16) -> i32;
    pub fn mlx5_vport_get_vhca_id(dev: *mut mlx5_core_dev, vport: u16, vhca_id: *mut u16) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
