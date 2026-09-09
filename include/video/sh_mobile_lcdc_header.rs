/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from sh_mobile_lcdc.h. */

/* Dependency intent: linux/fb.h supplies fb_videomode. */

/* Register definitions */
pub const _LDDCKR: u32 = 0x410;
pub const LDDCKR_ICKSEL_BUS: u32 = 0 << 16;
pub const LDDCKR_ICKSEL_MIPI: u32 = 1 << 16;
pub const LDDCKR_ICKSEL_HDMI: u32 = 2 << 16;
pub const LDDCKR_ICKSEL_EXT: u32 = 3 << 16;
pub const LDDCKR_ICKSEL_MASK: u32 = 7 << 16;
pub const LDDCKR_MOSEL: u32 = 1 << 6;
pub const _LDDCKSTPR: u32 = 0x414;
pub const _LDINTR: u32 = 0x468;
pub const LDINTR_FE: u32 = 1 << 10;
pub const LDINTR_VSE: u32 = 1 << 9;
pub const LDINTR_VEE: u32 = 1 << 8;
pub const LDINTR_FS: u32 = 1 << 2;
pub const LDINTR_VSS: u32 = 1 << 1;
pub const LDINTR_VES: u32 = 1 << 0;
pub const LDINTR_STATUS_MASK: u32 = 0xff << 0;
pub const _LDSR: u32 = 0x46c;
pub const LDSR_MSS: u32 = 1 << 10;
pub const LDSR_MRS: u32 = 1 << 8;
pub const LDSR_AS: u32 = 1 << 1;
pub const _LDCNT1R: u32 = 0x470;
pub const LDCNT1R_DE: u32 = 1 << 0;
pub const _LDCNT2R: u32 = 0x474;
pub const LDCNT2R_BR: u32 = 1 << 8;
pub const LDCNT2R_MD: u32 = 1 << 3;
pub const LDCNT2R_SE: u32 = 1 << 2;
pub const LDCNT2R_ME: u32 = 1 << 1;
pub const LDCNT2R_DO: u32 = 1 << 0;
pub const _LDRCNTR: u32 = 0x478;
pub const LDRCNTR_SRS: u32 = 1 << 17;
pub const LDRCNTR_SRC: u32 = 1 << 16;
pub const LDRCNTR_MRS: u32 = 1 << 1;
pub const LDRCNTR_MRC: u32 = 1 << 0;
pub const _LDDDSR: u32 = 0x47c;
pub const LDDDSR_LS: u32 = 1 << 2;
pub const LDDDSR_WS: u32 = 1 << 1;
pub const LDDDSR_BS: u32 = 1 << 0;

pub const LDMT1R_VPOL: u32 = 1 << 28;
pub const LDMT1R_HPOL: u32 = 1 << 27;
pub const LDMT1R_DWPOL: u32 = 1 << 26;
pub const LDMT1R_DIPOL: u32 = 1 << 25;
pub const LDMT1R_DAPOL: u32 = 1 << 24;
pub const LDMT1R_HSCNT: u32 = 1 << 17;
pub const LDMT1R_DWCNT: u32 = 1 << 16;
pub const LDMT1R_IFM: u32 = 1 << 12;
pub const LDMT1R_MIFTYP_RGB8: u32 = 0x0 << 0;
pub const LDMT1R_MIFTYP_RGB9: u32 = 0x4 << 0;
pub const LDMT1R_MIFTYP_RGB12A: u32 = 0x5 << 0;
pub const LDMT1R_MIFTYP_RGB12B: u32 = 0x6 << 0;
pub const LDMT1R_MIFTYP_RGB16: u32 = 0x7 << 0;
pub const LDMT1R_MIFTYP_RGB18: u32 = 0xa << 0;
pub const LDMT1R_MIFTYP_RGB24: u32 = 0xb << 0;
pub const LDMT1R_MIFTYP_YCBCR: u32 = 0xf << 0;
pub const LDMT1R_MIFTYP_SYS8A: u32 = 0x0 << 0;
pub const LDMT1R_MIFTYP_SYS8B: u32 = 0x1 << 0;
pub const LDMT1R_MIFTYP_SYS8C: u32 = 0x2 << 0;
pub const LDMT1R_MIFTYP_SYS8D: u32 = 0x3 << 0;
pub const LDMT1R_MIFTYP_SYS9: u32 = 0x4 << 0;
pub const LDMT1R_MIFTYP_SYS12: u32 = 0x5 << 0;
pub const LDMT1R_MIFTYP_SYS16A: u32 = 0x7 << 0;
pub const LDMT1R_MIFTYP_SYS16B: u32 = 0x8 << 0;
pub const LDMT1R_MIFTYP_SYS16C: u32 = 0x9 << 0;
pub const LDMT1R_MIFTYP_SYS18: u32 = 0xa << 0;
pub const LDMT1R_MIFTYP_SYS24: u32 = 0xb << 0;
pub const LDMT1R_MIFTYP_MASK: u32 = 0xf << 0;

pub const LDDFR_CF1: u32 = 1 << 18;
pub const LDDFR_CF0: u32 = 1 << 17;
pub const LDDFR_CC: u32 = 1 << 16;
pub const LDDFR_YF_420: u32 = 0 << 8;
pub const LDDFR_YF_422: u32 = 1 << 8;
pub const LDDFR_YF_444: u32 = 2 << 8;
pub const LDDFR_YF_MASK: u32 = 3 << 8;
pub const LDDFR_PKF_ARGB32: u32 = 0x00 << 0;
pub const LDDFR_PKF_RGB16: u32 = 0x03 << 0;
pub const LDDFR_PKF_RGB24: u32 = 0x0b << 0;
pub const LDDFR_PKF_MASK: u32 = 0x1f << 0;
pub const LDSM1R_OS: u32 = 1 << 0;
pub const LDSM2R_OSTRG: u32 = 1 << 0;
pub const LDPMR_LPS: u32 = 3 << 0;
pub const _LDDWD0R: u32 = 0x800;
pub const LDDWDxR_WDACT: u32 = 1 << 28;
pub const LDDWDxR_RSW: u32 = 1 << 24;
pub const _LDDRDR: u32 = 0x840;
pub const LDDRDR_RSR: u32 = 1 << 24;
pub const LDDRDR_DRD_MASK: u32 = 0x3ffff << 0;
pub const _LDDWAR: u32 = 0x900;
pub const LDDWAR_WA: u32 = 1 << 0;
pub const _LDDRAR: u32 = 0x904;
pub const LDDRAR_RA: u32 = 1 << 0;

pub const RGB8: u32 = LDMT1R_MIFTYP_RGB8; // 24bpp, 8:8:8
pub const RGB9: u32 = LDMT1R_MIFTYP_RGB9; // 18bpp, 9:9
pub const RGB12A: u32 = LDMT1R_MIFTYP_RGB12A; // 24bpp, 12:12
pub const RGB12B: u32 = LDMT1R_MIFTYP_RGB12B; // 12bpp
pub const RGB16: u32 = LDMT1R_MIFTYP_RGB16; // 16bpp
pub const RGB18: u32 = LDMT1R_MIFTYP_RGB18; // 18bpp
pub const RGB24: u32 = LDMT1R_MIFTYP_RGB24; // 24bpp
pub const YUV422: u32 = LDMT1R_MIFTYP_YCBCR; // 16bpp
pub const SYS8A: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS8A; // 24bpp, 8:8:8
pub const SYS8B: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS8B; // 18bpp, 8:8:2
pub const SYS8C: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS8C; // 18bpp, 2:8:8
pub const SYS8D: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS8D; // 16bpp, 8:8
pub const SYS9: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS9; // 18bpp, 9:9
pub const SYS12: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS12; // 24bpp, 12:12
pub const SYS16A: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS16A; // 16bpp
pub const SYS16B: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS16B; // 18bpp, 16:2
pub const SYS16C: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS16C; // 18bpp, 2:16
pub const SYS18: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS18; // 18bpp
pub const SYS24: u32 = LDMT1R_IFM | LDMT1R_MIFTYP_SYS24; // 24bpp

pub const LCDC_CHAN_DISABLED: i32 = 0;
pub const LCDC_CHAN_MAINLCD: i32 = 1;
pub const LCDC_CHAN_SUBLCD: i32 = 2;
pub const LCDC_CLK_BUS: i32 = 0;
pub const LCDC_CLK_PERIPHERAL: i32 = 1;
pub const LCDC_CLK_EXTERNAL: i32 = 2;
pub const LCDC_FLAGS_DWPOL: u32 = 1 << 0; // Rising edge dot clock data latch
pub const LCDC_FLAGS_DIPOL: u32 = 1 << 1; // Active low display enable polarity
pub const LCDC_FLAGS_DAPOL: u32 = 1 << 2; // Active low display data polarity
pub const LCDC_FLAGS_HSCNT: u32 = 1 << 3; // Disable HSYNC during VBLANK
pub const LCDC_FLAGS_DWCNT: u32 = 1 << 4; // Disable dotclock during blanking

#[repr(C)]
pub struct sh_mobile_lcdc_sys_bus_cfg { pub ldmt2r: usize, pub ldmt3r: usize, pub deferred_io_msec: usize }

#[repr(C)]
pub struct sh_mobile_lcdc_sys_bus_ops {
    pub write_index: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize)>,
    pub write_data: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize)>,
    pub read_data: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> usize>,
}

#[repr(C)]
pub struct sh_mobile_lcdc_panel_cfg {
    pub width: usize, pub height: usize,
    pub setup_sys: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut sh_mobile_lcdc_sys_bus_ops) -> i32>,
    pub start_transfer: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut sh_mobile_lcdc_sys_bus_ops)>,
    pub display_on: Option<unsafe extern "C" fn()>, pub display_off: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct sh_mobile_lcdc_bl_info {
    pub name: *const core::ffi::c_char, pub max_brightness: i32,
    pub set_brightness: Option<unsafe extern "C" fn(i32) -> i32>,
}

#[repr(C)]
pub struct sh_mobile_lcdc_overlay_cfg { pub fourcc: i32, pub max_xres: u32, pub max_yres: u32 }

/* External types are supplied by the including translation unit. */
#[repr(C)]
pub struct sh_mobile_lcdc_chan_cfg {
    pub chan: i32, pub fourcc: i32, pub colorspace: i32, pub interface_type: i32,
    pub clock_divider: i32, pub flags: usize,
    pub lcd_modes: *const fb_videomode, pub num_modes: i32,
    pub panel_cfg: sh_mobile_lcdc_panel_cfg, pub bl_info: sh_mobile_lcdc_bl_info,
    pub sys_bus_cfg: sh_mobile_lcdc_sys_bus_cfg,
    pub tx_dev: *mut platform_device,
}

#[repr(C)]
pub struct sh_mobile_lcdc_info {
    pub clock_source: i32,
    pub ch: [sh_mobile_lcdc_chan_cfg; 2],
    pub overlays: [sh_mobile_lcdc_overlay_cfg; 4],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
