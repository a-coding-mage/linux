/*
 * Copyright (c) 2016, Mellanox Technologies. All rights reserved.
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

#[repr(C)]
pub enum mlx5_beacon_duration {
    MLX5_BEACON_DURATION_OFF = 0x0,
    MLX5_BEACON_DURATION_INF = 0xffff,
}

#[repr(C)]
pub enum mlx5_module_id {
    MLX5_MODULE_ID_SFP = 0x3,
    MLX5_MODULE_ID_QSFP = 0xC,
    MLX5_MODULE_ID_QSFP_PLUS = 0xD,
    MLX5_MODULE_ID_QSFP28 = 0x11,
    MLX5_MODULE_ID_DSFP = 0x1B,
}

#[repr(C)]
pub enum mlx5_an_status {
    MLX5_AN_UNAVAILABLE = 0,
    MLX5_AN_COMPLETE = 1,
    MLX5_AN_FAILED = 2,
    MLX5_AN_LINK_UP = 3,
    MLX5_AN_LINK_DOWN = 4,
}

pub const MLX5_I2C_ADDR_LOW: u32 = 0x50;
pub const MLX5_I2C_ADDR_HIGH: u32 = 0x51;
pub const MLX5_EEPROM_PAGE_LENGTH: u32 = 256;
pub const MLX5_EEPROM_HIGH_PAGE_LENGTH: u32 = 128;

#[repr(C)]
pub enum mlx5e_link_mode {
    MLX5E_1000BASE_CX_SGMII = 0,
    MLX5E_1000BASE_KX = 1,
    MLX5E_10GBASE_CX4 = 2,
    MLX5E_10GBASE_KX4 = 3,
    MLX5E_10GBASE_KR = 4,
    MLX5E_20GBASE_KR2 = 5,
    MLX5E_40GBASE_CR4 = 6,
    MLX5E_40GBASE_KR4 = 7,
    MLX5E_56GBASE_R4 = 8,
    MLX5E_10GBASE_CR = 12,
    MLX5E_10GBASE_SR = 13,
    MLX5E_10GBASE_ER = 14,
    MLX5E_40GBASE_SR4 = 15,
    MLX5E_40GBASE_LR4 = 16,
    MLX5E_50GBASE_SR2 = 18,
    MLX5E_100GBASE_CR4 = 20,
    MLX5E_100GBASE_SR4 = 21,
    MLX5E_100GBASE_KR4 = 22,
    MLX5E_100GBASE_LR4 = 23,
    MLX5E_100BASE_TX = 24,
    MLX5E_1000BASE_T = 25,
    MLX5E_10GBASE_T = 26,
    MLX5E_25GBASE_CR = 27,
    MLX5E_25GBASE_KR = 28,
    MLX5E_25GBASE_SR = 29,
    MLX5E_50GBASE_CR2 = 30,
    MLX5E_50GBASE_KR2 = 31,
    MLX5E_LINK_MODES_NUMBER,
}

#[repr(C)]
pub enum mlx5e_ext_link_mode {
    MLX5E_SGMII_100M = 0,
    MLX5E_1000BASE_X_SGMII = 1,
    MLX5E_5GBASE_R = 3,
    MLX5E_10GBASE_XFI_XAUI_1 = 4,
    MLX5E_40GBASE_XLAUI_4_XLPPI_4 = 5,
    MLX5E_25GAUI_1_25GBASE_CR_KR = 6,
    MLX5E_50GAUI_2_LAUI_2_50GBASE_CR2_KR2 = 7,
    MLX5E_50GAUI_1_LAUI_1_50GBASE_CR_KR = 8,
    MLX5E_CAUI_4_100GBASE_CR4_KR4 = 9,
    MLX5E_100GAUI_2_100GBASE_CR2_KR2 = 10,
    MLX5E_100GAUI_1_100GBASE_CR_KR = 11,
    MLX5E_200GAUI_4_200GBASE_CR4_KR4 = 12,
    MLX5E_200GAUI_2_200GBASE_CR2_KR2 = 13,
    MLX5E_200GAUI_1_200GBASE_CR1_KR1 = 14,
    MLX5E_400GAUI_8_400GBASE_CR8 = 15,
    MLX5E_400GAUI_4_400GBASE_CR4_KR4 = 16,
    MLX5E_400GAUI_2_400GBASE_CR2_KR2 = 17,
    MLX5E_800GAUI_8_800GBASE_CR8_KR8 = 19,
    MLX5E_800GAUI_4_800GBASE_CR4_KR4 = 20,
    MLX5E_1600GAUI_8_1600GBASE_CR8_KR8 = 23,
    MLX5E_EXT_LINK_MODES_NUMBER,
}

#[repr(C)]
pub enum mlx5e_connector_type {
    MLX5E_PORT_UNKNOWN = 0,
    MLX5E_PORT_NONE = 1,
    MLX5E_PORT_TP = 2,
    MLX5E_PORT_AUI = 3,
    MLX5E_PORT_BNC = 4,
    MLX5E_PORT_MII = 5,
    MLX5E_PORT_FIBRE = 6,
    MLX5E_PORT_DA = 7,
    MLX5E_PORT_OTHER = 8,
    MLX5E_CONNECTOR_TYPE_NUMBER,
}

#[repr(C)]
pub enum mlx5_ptys_width {
    MLX5_PTYS_WIDTH_1X = 1 << 0,
    MLX5_PTYS_WIDTH_2X = 1 << 1,
    MLX5_PTYS_WIDTH_4X = 1 << 2,
    MLX5_PTYS_WIDTH_8X = 1 << 3,
    MLX5_PTYS_WIDTH_12X = 1 << 4,
}

#[macro_export]
macro_rules! MLX5E_PROT_MASK {
    ($link_mode:expr) => { 1u32 << $link_mode };
}

// MLX5_GET_ETH_PROTO preserves the external MLX5_GET macro dependency.
#[macro_export]
macro_rules! MLX5_GET_ETH_PROTO {
    ($reg:expr, $out:expr, $ext:expr, $field:ident) => {
        if $ext { MLX5_GET!($reg, $out, ext_$field) } else { MLX5_GET!($reg, $out, $field) }
    };
}

#[repr(C)]
pub struct mlx5_core_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn mlx5_set_port_caps(dev: *mut mlx5_core_dev, port_num: u8, caps: u32) -> i32;
    pub fn mlx5_query_port_ptys(
        dev: *mut mlx5_core_dev,
        ptys: *mut u32,
        ptys_size: i32,
        proto_mask: i32,
        local_port: u8,
        plane_index: u8,
    ) -> i32;
    pub fn mlx5_query_ib_port_oper(
        dev: *mut mlx5_core_dev,
        link_width_oper: *mut u16,
        proto_oper: *mut u16,
        local_port: u8,
        plane_index: u8,
    ) -> i32;
    pub fn mlx5_query_port_max_mtu(dev: *mut mlx5_core_dev, max_mtu: *mut u16, port: u8);
    pub fn mlx5_query_port_oper_mtu(dev: *mut mlx5_core_dev, oper_mtu: *mut u16, port: u8);
    pub fn mlx5_query_port_vl_hw_cap(dev: *mut mlx5_core_dev, vl_hw_cap: *mut u8, local_port: u8) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
