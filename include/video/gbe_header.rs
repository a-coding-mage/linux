/* SPDX-License-Identifier: GPL-2.0-only */
/* include/video/gbe.h -- SGI GBE (Graphics Back End) */

#[repr(C)]
pub struct sgi_gbe {
    pub ctrlstat: core::cell::UnsafeCell<u32>, pub dotclock: core::cell::UnsafeCell<u32>,
    pub i2c: core::cell::UnsafeCell<u32>, pub sysclk: core::cell::UnsafeCell<u32>,
    pub i2cfp: core::cell::UnsafeCell<u32>, pub id: core::cell::UnsafeCell<u32>,
    pub config: core::cell::UnsafeCell<u32>, pub bist: core::cell::UnsafeCell<u32>,
    pub _pad0: [u32; 0x010000 / 4 - 8],
    pub vt_xy: core::cell::UnsafeCell<u32>, pub vt_xymax: core::cell::UnsafeCell<u32>,
    pub vt_vsync: core::cell::UnsafeCell<u32>, pub vt_hsync: core::cell::UnsafeCell<u32>,
    pub vt_vblank: core::cell::UnsafeCell<u32>, pub vt_hblank: core::cell::UnsafeCell<u32>,
    pub vt_flags: core::cell::UnsafeCell<u32>, pub vt_f2rf_lock: core::cell::UnsafeCell<u32>,
    pub vt_intr01: core::cell::UnsafeCell<u32>, pub vt_intr23: core::cell::UnsafeCell<u32>,
    pub fp_hdrv: core::cell::UnsafeCell<u32>, pub fp_vdrv: core::cell::UnsafeCell<u32>,
    pub fp_de: core::cell::UnsafeCell<u32>, pub vt_hpixen: core::cell::UnsafeCell<u32>,
    pub vt_vpixen: core::cell::UnsafeCell<u32>, pub vt_hcmap: core::cell::UnsafeCell<u32>,
    pub vt_vcmap: core::cell::UnsafeCell<u32>, pub did_start_xy: core::cell::UnsafeCell<u32>,
    pub crs_start_xy: core::cell::UnsafeCell<u32>, pub vc_start_xy: core::cell::UnsafeCell<u32>,
    pub _pad1: [u32; 0xffb0 / 4],
    pub ovr_width_tile: core::cell::UnsafeCell<u32>, pub ovr_inhwctrl: core::cell::UnsafeCell<u32>,
    pub ovr_control: core::cell::UnsafeCell<u32>, pub _pad2: [u32; 0xfff4 / 4],
    pub frm_size_tile: core::cell::UnsafeCell<u32>, pub frm_size_pixel: core::cell::UnsafeCell<u32>,
    pub frm_inhwctrl: core::cell::UnsafeCell<u32>, pub frm_control: core::cell::UnsafeCell<u32>,
    pub _pad3: [u32; 0xfff0 / 4], pub did_inhwctrl: core::cell::UnsafeCell<u32>,
    pub did_control: core::cell::UnsafeCell<u32>, pub _pad4: [u32; 0x7ff8 / 4],
    pub mode_regs: [core::cell::UnsafeCell<u32>; 32], pub _pad5: [u32; 0x7f80 / 4],
    pub cmap: [core::cell::UnsafeCell<u32>; 6144], pub _pad6: [u32; 0x2000 / 4],
    pub cm_fifo: core::cell::UnsafeCell<u32>, pub _pad7: [u32; 0x7ffc / 4],
    pub gmap: [core::cell::UnsafeCell<u32>; 256], pub _pad8: [u32; 0x7c00 / 4],
    pub gmap10: [core::cell::UnsafeCell<u32>; 1024], pub _pad9: [u32; 0x7000 / 4],
    pub crs_pos: core::cell::UnsafeCell<u32>, pub crs_ctl: core::cell::UnsafeCell<u32>,
    pub crs_cmap: [core::cell::UnsafeCell<u32>; 3], pub _pad10: [u32; 0x7fec / 4],
    pub crs_glyph: [core::cell::UnsafeCell<u32>; 64], pub _pad11: [u32; 0x7f00 / 4],
    pub vc_0: core::cell::UnsafeCell<u32>, pub vc_1: core::cell::UnsafeCell<u32>,
    pub vc_2: core::cell::UnsafeCell<u32>, pub vc_3: core::cell::UnsafeCell<u32>,
    pub vc_4: core::cell::UnsafeCell<u32>, pub vc_5: core::cell::UnsafeCell<u32>,
    pub vc_6: core::cell::UnsafeCell<u32>, pub vc_7: core::cell::UnsafeCell<u32>,
    pub vc_8: core::cell::UnsafeCell<u32>,
}

#[inline] pub const fn MASK(msb: u32, lsb: u32) -> u32 { (((1u32 << (msb - lsb + 1)) - 1) << lsb) }
#[inline] pub const fn GET(v: u32, msb: u32, lsb: u32) -> u32 { (v & MASK(msb, lsb)) >> lsb }
#[inline] pub const fn SET(v: u32, f: u32, msb: u32, lsb: u32) -> u32 { (v & !MASK(msb, lsb)) | (((f << lsb) & MASK(msb, lsb))) }
/* GET_GBE_FIELD(reg, field, v) and SET_GBE_FIELD(reg, field, v, f) use the
 * corresponding GBE_<reg>_<field>_MSB/LSB constants; Rust callers use GET/SET. */

/* Bit mask information */

macro_rules! gbe_bits { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
gbe_bits! {
 GBE_CTRLSTAT_CHIPID_MSB=3, GBE_CTRLSTAT_CHIPID_LSB=0, GBE_CTRLSTAT_SENSE_N_MSB=4, GBE_CTRLSTAT_SENSE_N_LSB=4, GBE_CTRLSTAT_PCLKSEL_MSB=29, GBE_CTRLSTAT_PCLKSEL_LSB=28,
 GBE_DOTCLK_M_MSB=7, GBE_DOTCLK_M_LSB=0, GBE_DOTCLK_N_MSB=13, GBE_DOTCLK_N_LSB=8, GBE_DOTCLK_P_MSB=15, GBE_DOTCLK_P_LSB=14, GBE_DOTCLK_RUN_MSB=20, GBE_DOTCLK_RUN_LSB=20,
 GBE_VT_XY_Y_MSB=23, GBE_VT_XY_Y_LSB=12, GBE_VT_XY_X_MSB=11, GBE_VT_XY_X_LSB=0, GBE_VT_XY_FREEZE_MSB=31, GBE_VT_XY_FREEZE_LSB=31,
 GBE_FP_VDRV_ON_MSB=23, GBE_FP_VDRV_ON_LSB=12, GBE_FP_VDRV_OFF_MSB=11, GBE_FP_VDRV_OFF_LSB=0, GBE_FP_HDRV_ON_MSB=23, GBE_FP_HDRV_ON_LSB=12, GBE_FP_HDRV_OFF_MSB=11, GBE_FP_HDRV_OFF_LSB=0, GBE_FP_DE_ON_MSB=23, GBE_FP_DE_ON_LSB=12, GBE_FP_DE_OFF_MSB=11, GBE_FP_DE_OFF_LSB=0,
 GBE_VT_VSYNC_VSYNC_ON_MSB=23, GBE_VT_VSYNC_VSYNC_ON_LSB=12, GBE_VT_VSYNC_VSYNC_OFF_MSB=11, GBE_VT_VSYNC_VSYNC_OFF_LSB=0, GBE_VT_HSYNC_HSYNC_ON_MSB=23, GBE_VT_HSYNC_HSYNC_ON_LSB=12, GBE_VT_HSYNC_HSYNC_OFF_MSB=11, GBE_VT_HSYNC_HSYNC_OFF_LSB=0,
 GBE_VT_VBLANK_VBLANK_ON_MSB=23, GBE_VT_VBLANK_VBLANK_ON_LSB=12, GBE_VT_VBLANK_VBLANK_OFF_MSB=11, GBE_VT_VBLANK_VBLANK_OFF_LSB=0, GBE_VT_HBLANK_HBLANK_ON_MSB=23, GBE_VT_HBLANK_HBLANK_ON_LSB=12, GBE_VT_HBLANK_HBLANK_OFF_MSB=11, GBE_VT_HBLANK_HBLANK_OFF_LSB=0,
 GBE_VT_FLAGS_F2RF_HIGH_MSB=6, GBE_VT_FLAGS_F2RF_HIGH_LSB=6, GBE_VT_FLAGS_SYNC_LOW_MSB=5, GBE_VT_FLAGS_SYNC_LOW_LSB=5, GBE_VT_FLAGS_SYNC_HIGH_MSB=4, GBE_VT_FLAGS_SYNC_HIGH_LSB=4, GBE_VT_FLAGS_HDRV_LOW_MSB=3, GBE_VT_FLAGS_HDRV_LOW_LSB=3, GBE_VT_FLAGS_HDRV_INVERT_MSB=2, GBE_VT_FLAGS_HDRV_INVERT_LSB=2, GBE_VT_FLAGS_VDRV_LOW_MSB=1, GBE_VT_FLAGS_VDRV_LOW_LSB=1, GBE_VT_FLAGS_VDRV_INVERT_MSB=0, GBE_VT_FLAGS_VDRV_INVERT_LSB=0,
 GBE_VT_VCMAP_VCMAP_ON_MSB=23, GBE_VT_VCMAP_VCMAP_ON_LSB=12, GBE_VT_VCMAP_VCMAP_OFF_MSB=11, GBE_VT_VCMAP_VCMAP_OFF_LSB=0, GBE_VT_HCMAP_HCMAP_ON_MSB=23, GBE_VT_HCMAP_HCMAP_ON_LSB=12, GBE_VT_HCMAP_HCMAP_OFF_MSB=11, GBE_VT_HCMAP_HCMAP_OFF_LSB=0,
 GBE_VT_XYMAX_MAXX_MSB=11, GBE_VT_XYMAX_MAXX_LSB=0, GBE_VT_XYMAX_MAXY_MSB=23, GBE_VT_XYMAX_MAXY_LSB=12, GBE_VT_HPIXEN_HPIXEN_ON_MSB=23, GBE_VT_HPIXEN_HPIXEN_ON_LSB=12, GBE_VT_HPIXEN_HPIXEN_OFF_MSB=11, GBE_VT_HPIXEN_HPIXEN_OFF_LSB=0, GBE_VT_VPIXEN_VPIXEN_ON_MSB=23, GBE_VT_VPIXEN_VPIXEN_ON_LSB=12, GBE_VT_VPIXEN_VPIXEN_OFF_MSB=11, GBE_VT_VPIXEN_VPIXEN_OFF_LSB=0,
 GBE_OVR_CONTROL_OVR_DMA_ENABLE_MSB=0, GBE_OVR_CONTROL_OVR_DMA_ENABLE_LSB=0, GBE_OVR_INHWCTRL_OVR_DMA_ENABLE_MSB=0, GBE_OVR_INHWCTRL_OVR_DMA_ENABLE_LSB=0, GBE_OVR_WIDTH_TILE_OVR_FIFO_RESET_MSB=13, GBE_OVR_WIDTH_TILE_OVR_FIFO_RESET_LSB=13,
 GBE_FRM_CONTROL_FRM_DMA_ENABLE_MSB=0, GBE_FRM_CONTROL_FRM_DMA_ENABLE_LSB=0, GBE_FRM_CONTROL_FRM_TILE_PTR_MSB=31, GBE_FRM_CONTROL_FRM_TILE_PTR_LSB=9, GBE_FRM_CONTROL_FRM_LINEAR_MSB=1, GBE_FRM_CONTROL_FRM_LINEAR_LSB=1, GBE_FRM_INHWCTRL_FRM_DMA_ENABLE_MSB=0, GBE_FRM_INHWCTRL_FRM_DMA_ENABLE_LSB=0,
 GBE_FRM_SIZE_TILE_FRM_WIDTH_TILE_MSB=12, GBE_FRM_SIZE_TILE_FRM_WIDTH_TILE_LSB=5, GBE_FRM_SIZE_TILE_FRM_RHS_MSB=4, GBE_FRM_SIZE_TILE_FRM_RHS_LSB=0, GBE_FRM_SIZE_TILE_FRM_DEPTH_MSB=14, GBE_FRM_SIZE_TILE_FRM_DEPTH_LSB=13, GBE_FRM_SIZE_TILE_FRM_FIFO_RESET_MSB=15, GBE_FRM_SIZE_TILE_FRM_FIFO_RESET_LSB=15, GBE_FRM_SIZE_PIXEL_FB_HEIGHT_PIX_MSB=31, GBE_FRM_SIZE_PIXEL_FB_HEIGHT_PIX_LSB=16,
 GBE_DID_CONTROL_DID_DMA_ENABLE_MSB=0, GBE_DID_CONTROL_DID_DMA_ENABLE_LSB=0, GBE_DID_INHWCTRL_DID_DMA_ENABLE_MSB=0, GBE_DID_INHWCTRL_DID_DMA_ENABLE_LSB=0, GBE_DID_START_XY_DID_STARTY_MSB=23, GBE_DID_START_XY_DID_STARTY_LSB=12, GBE_DID_START_XY_DID_STARTX_MSB=11, GBE_DID_START_XY_DID_STARTX_LSB=0, GBE_CRS_START_XY_CRS_STARTY_MSB=23, GBE_CRS_START_XY_CRS_STARTY_LSB=12, GBE_CRS_START_XY_CRS_STARTX_MSB=11, GBE_CRS_START_XY_CRS_STARTX_LSB=0,
 GBE_WID_AUX_MSB=12, GBE_WID_AUX_LSB=11, GBE_WID_GAMMA_MSB=10, GBE_WID_GAMMA_LSB=10, GBE_WID_CM_MSB=9, GBE_WID_CM_LSB=5, GBE_WID_TYP_MSB=4, GBE_WID_TYP_LSB=2, GBE_WID_BUF_MSB=1, GBE_WID_BUF_LSB=0, GBE_VC_START_XY_VC_STARTY_MSB=23, GBE_VC_START_XY_VC_STARTY_LSB=12, GBE_VC_START_XY_VC_STARTX_MSB=11, GBE_VC_START_XY_VC_STARTX_LSB=0
}

pub const GBE_FRM_DEPTH_8: i32=0; pub const GBE_FRM_DEPTH_16: i32=1; pub const GBE_FRM_DEPTH_32: i32=2;
pub const GBE_CMODE_I8: i32=0; pub const GBE_CMODE_I12: i32=1; pub const GBE_CMODE_RG3B2: i32=2; pub const GBE_CMODE_RGB4: i32=3; pub const GBE_CMODE_ARGB5: i32=4; pub const GBE_CMODE_RGB8: i32=5; pub const GBE_CMODE_RGBA5: i32=6; pub const GBE_CMODE_RGB10: i32=7;
pub const GBE_BMODE_BOTH: i32=3; pub const GBE_CRS_MAGIC: i32=54; pub const GBE_PIXEN_MAGIC_ON: i32=19; pub const GBE_PIXEN_MAGIC_OFF: i32=2; pub const GBE_TLB_SIZE: i32=128;

#[repr(C)]
pub struct gbe_timing_info { pub flags: i32, pub width: i16, pub height: i16, pub fields_sec: i32, pub cfreq: i32, pub htotal: i16, pub hblank_start: i16, pub hblank_end: i16, pub hsync_start: i16, pub hsync_end: i16, pub vtotal: i16, pub vblank_start: i16, pub vblank_end: i16, pub vsync_start: i16, pub vsync_end: i16, pub pll_m: i16, pub pll_n: i16, pub pll_p: i16 }

pub const GBE_VOF_UNKNOWNMON: i32=1; pub const GBE_VOF_STEREO: i32=2; pub const GBE_VOF_DO_GENSYNC: i32=4; pub const GBE_VOF_SYNC_ON_GREEN: i32=8; pub const GBE_VOF_FLATPANEL: i32=0x1000; pub const GBE_VOF_MAGICKEY: i32=0x2000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
