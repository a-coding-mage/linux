/* SPDX-License-Identifier: GPL-2.0-only */

/* Maximum size for a single dma descriptor. */
pub const STEDMA40_MAX_SEG_SIZE: u32 = 0xFFFF;

/* dev types for memcpy */
pub const STEDMA40_DEV_DST_MEMORY: i32 = -1;
pub const STEDMA40_DEV_SRC_MEMORY: i32 = -1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum stedma40_mode {
    STEDMA40_MODE_LOGICAL = 0,
    STEDMA40_MODE_PHYSICAL,
    STEDMA40_MODE_OPERATION,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum stedma40_mode_opt {
    STEDMA40_PCHAN_BASIC_MODE = 0,
    STEDMA40_LCHAN_SRC_LOG_DST_LOG = 0,
    STEDMA40_PCHAN_MODULO_MODE,
    STEDMA40_PCHAN_DOUBLE_DST_MODE,
    STEDMA40_LCHAN_SRC_PHY_DST_LOG,
    STEDMA40_LCHAN_SRC_LOG_DST_PHY,
}

pub const STEDMA40_ESIZE_8_BIT: i32 = 0x0;
pub const STEDMA40_ESIZE_16_BIT: i32 = 0x1;
pub const STEDMA40_ESIZE_32_BIT: i32 = 0x2;
pub const STEDMA40_ESIZE_64_BIT: i32 = 0x3;

/* The value 4 indicates that PEN-reg shall be set to 0 */
pub const STEDMA40_PSIZE_PHY_1: i32 = 0x4;
pub const STEDMA40_PSIZE_PHY_2: i32 = 0x0;
pub const STEDMA40_PSIZE_PHY_4: i32 = 0x1;
pub const STEDMA40_PSIZE_PHY_8: i32 = 0x2;
pub const STEDMA40_PSIZE_PHY_16: i32 = 0x3;

/* The number of elements differ in logical and physical mode. */
pub const STEDMA40_PSIZE_LOG_1: i32 = STEDMA40_PSIZE_PHY_2;
pub const STEDMA40_PSIZE_LOG_4: i32 = STEDMA40_PSIZE_PHY_4;
pub const STEDMA40_PSIZE_LOG_8: i32 = STEDMA40_PSIZE_PHY_8;
pub const STEDMA40_PSIZE_LOG_16: i32 = STEDMA40_PSIZE_PHY_16;

/* Maximum number of possible physical channels */
pub const STEDMA40_MAX_PHYS: i32 = 32;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum stedma40_flow_ctrl {
    STEDMA40_NO_FLOW_CTRL,
    STEDMA40_FLOW_CTRL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stedma40_half_channel_info {
    pub big_endian: bool,
    pub data_width: dma_slave_buswidth,
    pub psize: i32,
    pub flow_ctrl: stedma40_flow_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stedma40_chan_cfg {
    pub dir: dma_transfer_direction,
    pub high_priority: bool,
    pub realtime: bool,
    pub mode: stedma40_mode,
    pub mode_opt: stedma40_mode_opt,
    pub dev_type: i32,
    pub src_info: stedma40_half_channel_info,
    pub dst_info: stedma40_half_channel_info,
    pub use_fixed_channel: bool,
    pub phy_channel: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
