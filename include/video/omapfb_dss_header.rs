/* SPDX-License-Identifier: GPL-2.0-or-later */
/* C header translated literally to Rust; external kernel types are dependencies. */

use core::ffi::c_void;

pub const OMAP_DSS_MAX_DSI_PINS: usize = 22;
pub const DISPC_IRQ_FRAMEDONE: u32 = 1 << 0;
pub const DISPC_IRQ_VSYNC: u32 = 1 << 1;
pub const DISPC_IRQ_EVSYNC_EVEN: u32 = 1 << 2;
pub const DISPC_IRQ_EVSYNC_ODD: u32 = 1 << 3;
pub const DISPC_IRQ_ACBIAS_COUNT_STAT: u32 = 1 << 4;
pub const DISPC_IRQ_PROG_LINE_NUM: u32 = 1 << 5;
pub const DISPC_IRQ_GFX_FIFO_UNDERFLOW: u32 = 1 << 6;
pub const DISPC_IRQ_GFX_END_WIN: u32 = 1 << 7;
pub const DISPC_IRQ_PAL_GAMMA_MASK: u32 = 1 << 8;
pub const DISPC_IRQ_OCP_ERR: u32 = 1 << 9;
pub const DISPC_IRQ_VID1_FIFO_UNDERFLOW: u32 = 1 << 10;
pub const DISPC_IRQ_VID1_END_WIN: u32 = 1 << 11;
pub const DISPC_IRQ_VID2_FIFO_UNDERFLOW: u32 = 1 << 12;
pub const DISPC_IRQ_VID2_END_WIN: u32 = 1 << 13;
pub const DISPC_IRQ_SYNC_LOST: u32 = 1 << 14;
pub const DISPC_IRQ_SYNC_LOST_DIGIT: u32 = 1 << 15;
pub const DISPC_IRQ_WAKEUP: u32 = 1 << 16;
pub const DISPC_IRQ_SYNC_LOST2: u32 = 1 << 17;
pub const DISPC_IRQ_VSYNC2: u32 = 1 << 18;
pub const DISPC_IRQ_VID3_END_WIN: u32 = 1 << 19;
pub const DISPC_IRQ_VID3_FIFO_UNDERFLOW: u32 = 1 << 20;
pub const DISPC_IRQ_ACBIAS_COUNT_STAT2: u32 = 1 << 21;
pub const DISPC_IRQ_FRAMEDONE2: u32 = 1 << 22;
pub const DISPC_IRQ_FRAMEDONEWB: u32 = 1 << 23;
pub const DISPC_IRQ_FRAMEDONETV: u32 = 1 << 24;
pub const DISPC_IRQ_WBBUFFEROVERFLOW: u32 = 1 << 25;
pub const DISPC_IRQ_WBUNCOMPLETEERROR: u32 = 1 << 26;
pub const DISPC_IRQ_SYNC_LOST3: u32 = 1 << 27;
pub const DISPC_IRQ_VSYNC3: u32 = 1 << 28;
pub const DISPC_IRQ_ACBIAS_COUNT_STAT3: u32 = 1 << 29;
pub const DISPC_IRQ_FRAMEDONE3: u32 = 1 << 30;

#[repr(C)] pub struct omap_dss_device { pub _opaque: [u8; 0] }
#[repr(C)] pub struct omap_overlay_manager { pub _opaque: [u8; 0] }
#[repr(C)] pub struct dss_lcd_mgr_config { pub _opaque: [u8; 0] }
#[repr(C)] pub struct snd_aes_iec958 { pub _opaque: [u8; 0] }
#[repr(C)] pub struct snd_cea_861_aud_if { pub _opaque: [u8; 0] }
#[repr(C)] pub struct hdmi_avi_infoframe { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kobject { pub _opaque: [u8; 0] }
#[repr(C)] pub struct list_head { pub _opaque: [u8; 0] }
#[repr(C)] pub struct device { pub _opaque: [u8; 0] }
#[repr(C)] pub struct module { pub _opaque: [u8; 0] }
#[repr(C)] pub struct device_node { pub _opaque: [u8; 0] }
pub type dma_addr_t = usize;

macro_rules! c_enum { ($n:ident { $($v:ident = $x:expr),* $(,)? }) => { #[repr(C)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum $n { $($v = $x),* } }; }
c_enum!(omap_display_type { OMAP_DISPLAY_TYPE_NONE=0, OMAP_DISPLAY_TYPE_DPI=1<<0, OMAP_DISPLAY_TYPE_DBI=1<<1, OMAP_DISPLAY_TYPE_SDI=1<<2, OMAP_DISPLAY_TYPE_DSI=1<<3, OMAP_DISPLAY_TYPE_VENC=1<<4, OMAP_DISPLAY_TYPE_HDMI=1<<5, OMAP_DISPLAY_TYPE_DVI=1<<6 });
c_enum!(omap_plane { OMAP_DSS_GFX=0, OMAP_DSS_VIDEO1=1, OMAP_DSS_VIDEO2=2, OMAP_DSS_VIDEO3=3, OMAP_DSS_WB=4 });
c_enum!(omap_channel { OMAP_DSS_CHANNEL_LCD=0, OMAP_DSS_CHANNEL_DIGIT=1, OMAP_DSS_CHANNEL_LCD2=2, OMAP_DSS_CHANNEL_LCD3=3, OMAP_DSS_CHANNEL_WB=4 });
c_enum!(omap_color_mode { OMAP_DSS_COLOR_CLUT1=1<<0, OMAP_DSS_COLOR_CLUT2=1<<1, OMAP_DSS_COLOR_CLUT4=1<<2, OMAP_DSS_COLOR_CLUT8=1<<3, OMAP_DSS_COLOR_RGB12U=1<<4, OMAP_DSS_COLOR_ARGB16=1<<5, OMAP_DSS_COLOR_RGB16=1<<6, OMAP_DSS_COLOR_RGB24U=1<<7, OMAP_DSS_COLOR_RGB24P=1<<8, OMAP_DSS_COLOR_YUV2=1<<9, OMAP_DSS_COLOR_UYVY=1<<10, OMAP_DSS_COLOR_ARGB32=1<<11, OMAP_DSS_COLOR_RGBA32=1<<12, OMAP_DSS_COLOR_RGBX32=1<<13, OMAP_DSS_COLOR_NV12=1<<14, OMAP_DSS_COLOR_RGBA16=1<<15, OMAP_DSS_COLOR_RGBX16=1<<16, OMAP_DSS_COLOR_ARGB16_1555=1<<17, OMAP_DSS_COLOR_XRGB16_1555=1<<18 });
c_enum!(omap_dss_load_mode { OMAP_DSS_LOAD_CLUT_AND_FRAME=0, OMAP_DSS_LOAD_CLUT_ONLY=1, OMAP_DSS_LOAD_FRAME_ONLY=2, OMAP_DSS_LOAD_CLUT_ONCE_FRAME=3 });
c_enum!(omap_dss_trans_key_type { OMAP_DSS_COLOR_KEY_GFX_DST=0, OMAP_DSS_COLOR_KEY_VID_SRC=1 });
c_enum!(omap_dss_signal_level { OMAPDSS_SIG_ACTIVE_LOW=0, OMAPDSS_SIG_ACTIVE_HIGH=1 });
c_enum!(omap_dss_signal_edge { OMAPDSS_DRIVE_SIG_FALLING_EDGE=0, OMAPDSS_DRIVE_SIG_RISING_EDGE=1 });
c_enum!(omap_dss_venc_type { OMAP_DSS_VENC_TYPE_COMPOSITE=0, OMAP_DSS_VENC_TYPE_SVIDEO=1 });
c_enum!(omap_dss_dsi_pixel_format { OMAP_DSS_DSI_FMT_RGB888=0, OMAP_DSS_DSI_FMT_RGB666=1, OMAP_DSS_DSI_FMT_RGB666_PACKED=2, OMAP_DSS_DSI_FMT_RGB565=3 });
c_enum!(omap_dss_dsi_mode { OMAP_DSS_DSI_CMD_MODE=0, OMAP_DSS_DSI_VIDEO_MODE=1 });
c_enum!(omap_display_caps { OMAP_DSS_DISPLAY_CAP_MANUAL_UPDATE=1<<0, OMAP_DSS_DISPLAY_CAP_TEAR_ELIM=1<<1 });
c_enum!(omap_dss_display_state { OMAP_DSS_DISPLAY_DISABLED=0, OMAP_DSS_DISPLAY_ACTIVE=1 });
c_enum!(omap_dss_rotation_type { OMAP_DSS_ROT_DMA=1<<0, OMAP_DSS_ROT_VRFB=1<<1, OMAP_DSS_ROT_TILER=1<<2 });
c_enum!(omap_dss_rotation_angle { OMAP_DSS_ROT_0=0, OMAP_DSS_ROT_90=1, OMAP_DSS_ROT_180=2, OMAP_DSS_ROT_270=3 });
c_enum!(omap_overlay_caps { OMAP_DSS_OVL_CAP_SCALE=1<<0, OMAP_DSS_OVL_CAP_GLOBAL_ALPHA=1<<1, OMAP_DSS_OVL_CAP_PRE_MULT_ALPHA=1<<2, OMAP_DSS_OVL_CAP_ZORDER=1<<3, OMAP_DSS_OVL_CAP_POS=1<<4, OMAP_DSS_OVL_CAP_REPLICATION=1<<5 });
c_enum!(omap_dss_output_id { OMAP_DSS_OUTPUT_DPI=1<<0, OMAP_DSS_OUTPUT_DBI=1<<1, OMAP_DSS_OUTPUT_SDI=1<<2, OMAP_DSS_OUTPUT_DSI1=1<<3, OMAP_DSS_OUTPUT_DSI2=1<<4, OMAP_DSS_OUTPUT_VENC=1<<5, OMAP_DSS_OUTPUT_HDMI=1<<6 });
c_enum!(omap_dss_dsi_trans_mode { OMAP_DSS_DSI_PULSE_MODE=0, OMAP_DSS_DSI_EVENT_MODE=1, OMAP_DSS_DSI_BURST_MODE=2 });

#[repr(C)] pub struct omap_dss_dsi_videomode_timings { pub hsclk: usize, pub ndl:u32, pub bitspp:u32, pub hact:u16,pub vact:u16,pub hss:u16,pub hsa:u16,pub hse:u16,pub hfp:u16,pub hbp:u16,pub vsa:u16,pub vfp:u16,pub vbp:u16,pub blanking_mode:i32,pub hsa_blanking_mode:i32,pub hbp_blanking_mode:i32,pub hfp_blanking_mode:i32,pub trans_mode:omap_dss_dsi_trans_mode,pub ddr_clk_always_on:bool,pub window_sync:i32 }
#[repr(C)] pub struct omap_video_timings { pub x_res:u16,pub y_res:u16,pub pixelclock:u32,pub hsw:u16,pub hfp:u16,pub hbp:u16,pub vsw:u16,pub vfp:u16,pub vbp:u16,pub vsync_level:omap_dss_signal_level,pub hsync_level:omap_dss_signal_level,pub interlace:bool,pub data_pclk_edge:omap_dss_signal_edge,pub de_level:omap_dss_signal_level,pub sync_pclk_edge:omap_dss_signal_edge,pub double_pixel:bool }
#[repr(C)] pub struct omap_dss_dsi_config { pub mode:omap_dss_dsi_mode,pub pixel_format:omap_dss_dsi_pixel_format,pub timings:*const omap_video_timings,pub hs_clk_min:usize,pub hs_clk_max:usize,pub lp_clk_min:usize,pub lp_clk_max:usize,pub ddr_clk_always_on:bool,pub trans_mode:omap_dss_dsi_trans_mode }
#[repr(C)] pub struct omap_dss_cpr_coefs { pub rr:i16,pub rg:i16,pub rb:i16,pub gr:i16,pub gg:i16,pub gb:i16,pub br:i16,pub bg:i16,pub bb:i16 }
#[repr(C)] pub struct omap_dsi_pin_config { pub num_pins:i32,pub pins:[i32;OMAP_DSS_MAX_DSI_PINS] }
#[repr(C)] pub struct omap_overlay_info { pub paddr:usize,pub p_uv_addr:usize,pub screen_width:u16,pub width:u16,pub height:u16,pub color_mode:omap_color_mode,pub rotation:u8,pub rotation_type:omap_dss_rotation_type,pub mirror:bool,pub pos_x:u16,pub pos_y:u16,pub out_width:u16,pub out_height:u16,pub global_alpha:u8,pub pre_mult_alpha:u8,pub zorder:u8 }
#[repr(C)] pub struct omap_dss_writeback_info { pub paddr:u32,pub p_uv_addr:u32,pub buf_width:u16,pub width:u16,pub height:u16,pub color_mode:omap_color_mode,pub rotation:u8,pub rotation_type:omap_dss_rotation_type,pub mirror:bool,pub pre_mult_alpha:u8 }

#[repr(C)] pub struct omap_overlay { pub _opaque:[u8;0] }
#[repr(C)] pub struct omap_overlay_manager_info { pub _opaque:[u8;0] }
#[repr(C)] pub struct omapdss_dpi_ops { pub _opaque:[u8;0] }
#[repr(C)] pub struct omapdss_sdi_ops { pub _opaque:[u8;0] }
#[repr(C)] pub struct omapdss_dvi_ops { pub _opaque:[u8;0] }
#[repr(C)] pub struct omapdss_atv_ops { pub _opaque:[u8;0] }
#[repr(C)] pub struct omapdss_hdmi_ops { pub _opaque:[u8;0] }
#[repr(C)] pub struct omapdss_dsi_ops { pub _opaque:[u8;0] }
#[repr(C)] pub struct omap_dss_driver { pub _opaque:[u8;0] }
pub type omap_dispc_isr_t = unsafe extern "C" fn(*mut c_void, u32);

/* C conditional CONFIG_FB_OMAP2 API: declarations remain external; disabled
 * configurations are supplied by the build's compatibility layer. */
extern "C" {
    pub fn omap_dss_register_driver(driver:*mut omap_dss_driver)->i32;
    pub fn omap_dss_unregister_driver(driver:*mut omap_dss_driver);
    pub fn omapdss_register_display(dssdev:*mut omap_dss_device)->i32;
    pub fn omapdss_unregister_display(dssdev:*mut omap_dss_device);
    pub fn omap_dss_get_device(dssdev:*mut omap_dss_device)->*mut omap_dss_device;
    pub fn omap_dss_put_device(dssdev:*mut omap_dss_device);
    pub fn omap_dss_get_num_overlay_managers()->i32;
    pub fn omap_dss_get_overlay_manager(num:i32)->*mut omap_overlay_manager;
    pub fn omap_dss_get_num_overlays()->i32;
    pub fn omap_dss_get_overlay(num:i32)->*mut omap_overlay;
    pub fn omapdss_register_output(output:*mut omap_dss_device)->i32;
    pub fn omapdss_unregister_output(output:*mut omap_dss_device);
    pub fn omap_dss_get_output(id:omap_dss_output_id)->*mut omap_dss_device;
    pub fn omap_dispc_register_isr(isr:omap_dispc_isr_t,arg:*mut c_void,mask:u32)->i32;
    pub fn omap_dispc_unregister_isr(isr:omap_dispc_isr_t,arg:*mut c_void,mask:u32)->i32;
    pub fn omapdss_compat_init()->i32;
    pub fn omapdss_compat_uninit();
}

/* Function-pointer-bearing kernel objects are represented with opaque layouts here;
 * their declarations and ABI are supplied by the corresponding kernel bindings. */
extern "C" {
    pub static omap_dss_pal_timings: omap_video_timings;
    pub static omap_dss_ntsc_timings: omap_video_timings;
    pub fn omapdss_get_version() -> i32;
    pub fn omapdss_is_initialized() -> bool;
    pub fn omap_dss_get_next_device(from: *mut omap_dss_device) -> *mut omap_dss_device;
}

#[inline] pub unsafe fn omapdss_device_is_connected(dssdev:*mut omap_dss_device)->bool { !dssdev.is_null() }
#[inline] pub unsafe fn omapdss_device_is_enabled(_dssdev:*mut omap_dss_device)->bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
