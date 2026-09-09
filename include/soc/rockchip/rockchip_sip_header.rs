/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016, Fuzhou Rockchip Electronics Co., Ltd
 * Author: Lin Huang <hl@rock-chips.com>
 */

// C header guard: __SOC_ROCKCHIP_SIP_H

pub const ROCKCHIP_SIP_SUSPEND_MODE: u32 = 0x82000003;
pub const ROCKCHIP_SLEEP_PD_CONFIG: u32 = 0xff;

pub const ROCKCHIP_SIP_DRAM_FREQ: u32 = 0x82000008;
pub const ROCKCHIP_SIP_CONFIG_DRAM_INIT: u32 = 0x00;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_RATE: u32 = 0x01;
pub const ROCKCHIP_SIP_CONFIG_DRAM_ROUND_RATE: u32 = 0x02;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_AT_SR: u32 = 0x03;
pub const ROCKCHIP_SIP_CONFIG_DRAM_GET_BW: u32 = 0x04;
pub const ROCKCHIP_SIP_CONFIG_DRAM_GET_RATE: u32 = 0x05;
pub const ROCKCHIP_SIP_CONFIG_DRAM_CLR_IRQ: u32 = 0x06;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_PARAM: u32 = 0x07;
pub const ROCKCHIP_SIP_CONFIG_DRAM_SET_ODT_PD: u32 = 0x08;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
