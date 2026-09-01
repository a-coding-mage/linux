/* SPDX-License-Identifier: GPL-2.0 */

pub const AK4396_WRITE: u32 = 0x2000;

pub const AK4396_CONTROL_1: u32 = 0;
pub const AK4396_CONTROL_2: u32 = 1;
pub const AK4396_CONTROL_3: u32 = 2;
pub const AK4396_LCH_ATT: u32 = 3;
pub const AK4396_RCH_ATT: u32 = 4;

/* control 1 */
pub const AK4396_RSTN: u32 = 0x01;
pub const AK4396_DIF_MASK: u32 = 0x0e;
pub const AK4396_DIF_16_LSB: u32 = 0x00;
pub const AK4396_DIF_20_LSB: u32 = 0x02;
pub const AK4396_DIF_24_MSB: u32 = 0x04;
pub const AK4396_DIF_24_I2S: u32 = 0x06;
pub const AK4396_DIF_24_LSB: u32 = 0x08;
pub const AK4396_ACKS: u32 = 0x80;
/* control 2 */
pub const AK4396_SMUTE: u32 = 0x01;
pub const AK4396_DEM_MASK: u32 = 0x06;
pub const AK4396_DEM_441: u32 = 0x00;
pub const AK4396_DEM_OFF: u32 = 0x02;
pub const AK4396_DEM_48: u32 = 0x04;
pub const AK4396_DEM_32: u32 = 0x06;
pub const AK4396_DFS_MASK: u32 = 0x18;
pub const AK4396_DFS_NORMAL: u32 = 0x00;
pub const AK4396_DFS_DOUBLE: u32 = 0x08;
pub const AK4396_DFS_QUAD: u32 = 0x10;
pub const AK4396_SLOW: u32 = 0x20;
pub const AK4396_DZFM: u32 = 0x40;
pub const AK4396_DZFE: u32 = 0x80;
/* control 3 */
pub const AK4396_DZFB: u32 = 0x04;
pub const AK4396_DCKB: u32 = 0x10;
pub const AK4396_DCKS: u32 = 0x20;
pub const AK4396_DSDM: u32 = 0x40;
pub const AK4396_D_P_MASK: u32 = 0x80;
pub const AK4396_PCM: u32 = 0x00;
pub const AK4396_DSD: u32 = 0x80;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
