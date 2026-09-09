/*
 * linux/include/video/neo_reg.h -- NeoMagic Framebuffer Driver
 *
 * Copyright (c) 2001  Denis Oliver Kropp <dok@convergence.de>
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file COPYING in the main directory of this
 * archive for more details.
 */

pub const NEO_BS0_BLT_BUSY: u32 = 0x00000001;
pub const NEO_BS0_FIFO_AVAIL: u32 = 0x00000002;
pub const NEO_BS0_FIFO_PEND: u32 = 0x00000004;

pub const NEO_BC0_DST_Y_DEC: u32 = 0x00000001;
pub const NEO_BC0_X_DEC: u32 = 0x00000002;
pub const NEO_BC0_SRC_TRANS: u32 = 0x00000004;
pub const NEO_BC0_SRC_IS_FG: u32 = 0x00000008;
pub const NEO_BC0_SRC_Y_DEC: u32 = 0x00000010;
pub const NEO_BC0_FILL_PAT: u32 = 0x00000020;
pub const NEO_BC0_SRC_MONO: u32 = 0x00000040;
pub const NEO_BC0_SYS_TO_VID: u32 = 0x00000080;

pub const NEO_BC1_DEPTH8: u32 = 0x00000100;
pub const NEO_BC1_DEPTH16: u32 = 0x00000200;
pub const NEO_BC1_X_320: u32 = 0x00000400;
pub const NEO_BC1_X_640: u32 = 0x00000800;
pub const NEO_BC1_X_800: u32 = 0x00000c00;
pub const NEO_BC1_X_1024: u32 = 0x00001000;
pub const NEO_BC1_X_1152: u32 = 0x00001400;
pub const NEO_BC1_X_1280: u32 = 0x00001800;
pub const NEO_BC1_X_1600: u32 = 0x00001c00;
pub const NEO_BC1_DST_TRANS: u32 = 0x00002000;
pub const NEO_BC1_MSTR_BLT: u32 = 0x00004000;
pub const NEO_BC1_FILTER_Z: u32 = 0x00008000;

pub const NEO_BC2_WR_TR_DST: u32 = 0x00800000;

pub const NEO_BC3_SRC_XY_ADDR: u32 = 0x01000000;
pub const NEO_BC3_DST_XY_ADDR: u32 = 0x02000000;
pub const NEO_BC3_CLIP_ON: u32 = 0x04000000;
pub const NEO_BC3_FIFO_EN: u32 = 0x08000000;
pub const NEO_BC3_BLT_ON_ADDR: u32 = 0x10000000;
pub const NEO_BC3_SKIP_MAPPING: u32 = 0x80000000;

pub const NEO_MODE1_DEPTH8: u16 = 0x0100;
pub const NEO_MODE1_DEPTH16: u16 = 0x0200;
pub const NEO_MODE1_DEPTH24: u16 = 0x0300;
pub const NEO_MODE1_X_320: u16 = 0x0400;
pub const NEO_MODE1_X_640: u16 = 0x0800;
pub const NEO_MODE1_X_800: u16 = 0x0c00;
pub const NEO_MODE1_X_1024: u16 = 0x1000;
pub const NEO_MODE1_X_1152: u16 = 0x1400;
pub const NEO_MODE1_X_1280: u16 = 0x1800;
pub const NEO_MODE1_X_1600: u16 = 0x1c00;
pub const NEO_MODE1_BLT_ON_ADDR: u16 = 0x2000;

/* These are offseted in MMIO space by par->CursorOff */
pub const NEOREG_CURSCNTL: u32 = 0x00;
pub const NEOREG_CURSX: u32 = 0x04;
pub const NEOREG_CURSY: u32 = 0x08;
pub const NEOREG_CURSBGCOLOR: u32 = 0x0c;
pub const NEOREG_CURSFGCOLOR: u32 = 0x10;
pub const NEOREG_CURSMEMPOS: u32 = 0x14;

pub const NEO_CURS_DISABLE: u32 = 0x00000000;
pub const NEO_CURS_ENABLE: u32 = 0x00000001;
pub const NEO_ICON64_ENABLE: u32 = 0x00000008;
pub const NEO_ICON128_ENABLE: u32 = 0x0000000c;
pub const NEO_ICON_BLANK: u32 = 0x00000010;

pub const NEO_GR01_SUPPRESS_VSYNC: u32 = 0x10;
pub const NEO_GR01_SUPPRESS_HSYNC: u32 = 0x20;

/* __KERNEL__ declarations: retained under an equivalent Rust cfg gate. */
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2070: u16 = 0x0001;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2090: u16 = 0x0002;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2093: u16 = 0x0003;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2097: u16 = 0x0083;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2160: u16 = 0x0004;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2200: u16 = 0x0005;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2230: u16 = 0x0025;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2360: u16 = 0x0006;
#[cfg(feature = "__KERNEL__")]
pub const PCI_CHIP_NM2380: u16 = 0x0016;

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct Neo2200 {
    pub bltStat: u32,
    pub bltCntl: u32,
    pub xpColor: u32,
    pub fgColor: u32,
    pub bgColor: u32,
    pub pitch: u32,
    pub clipLT: u32,
    pub clipRB: u32,
    pub srcBitOffset: u32,
    pub srcStart: u32,
    pub reserved0: u32,
    pub dstStart: u32,
    pub xyExt: u32,
    pub reserved1: [u32; 19],
    pub pageCntl: u32,
    pub pageBase: u32,
    pub postBase: u32,
    pub postPtr: u32,
    pub dataPtr: u32,
}

#[cfg(feature = "__KERNEL__")]
pub const MMIO_SIZE: u32 = 0x200000;
#[cfg(feature = "__KERNEL__")]
pub const NEO_EXT_CR_MAX: u32 = 0x85;
#[cfg(feature = "__KERNEL__")]
pub const NEO_EXT_GR_MAX: u32 = 0xc7;

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct neofb_par {
    pub state: vgastate,
    pub ref_count: c_uint,
    pub MiscOutReg: u8,
    pub CRTC: [u8; 25],
    pub Sequencer: [u8; 5],
    pub Graphics: [u8; 9],
    pub Attribute: [u8; 21],
    pub GeneralLockReg: u8,
    pub ExtCRTDispAddr: u8,
    pub ExtCRTOffset: u8,
    pub SysIfaceCntl1: u8,
    pub SysIfaceCntl2: u8,
    pub ExtColorModeSelect: u8,
    pub biosMode: u8,
    pub PanelDispCntlReg1: u8,
    pub PanelDispCntlReg2: u8,
    pub PanelDispCntlReg3: u8,
    pub PanelDispCntlRegRead: u8,
    pub PanelVertCenterReg1: u8,
    pub PanelVertCenterReg2: u8,
    pub PanelVertCenterReg3: u8,
    pub PanelVertCenterReg4: u8,
    pub PanelVertCenterReg5: u8,
    pub PanelHorizCenterReg1: u8,
    pub PanelHorizCenterReg2: u8,
    pub PanelHorizCenterReg3: u8,
    pub PanelHorizCenterReg4: u8,
    pub PanelHorizCenterReg5: u8,
    pub ProgramVCLK: i32,
    pub VCLK3NumeratorLow: u8,
    pub VCLK3NumeratorHigh: u8,
    pub VCLK3Denominator: u8,
    pub VerticalExt: u8,
    pub wc_cookie: i32,
    pub mmio_vbase: *mut u8,
    pub cursorOff: u8,
    pub cursorPad: *mut u8,
    pub neo2200: *mut Neo2200,
    pub NeoPanelWidth: i32,
    pub NeoPanelHeight: i32,
    pub maxClock: i32,
    pub pci_burst: i32,
    pub lcd_stretch: i32,
    pub internal_display: i32,
    pub external_display: i32,
    pub libretto: i32,
    pub palette: [u32; 16],
}

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct biosMode {
    pub x_res: i32,
    pub y_res: i32,
    pub mode: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
