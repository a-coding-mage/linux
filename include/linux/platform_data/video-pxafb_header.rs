/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Support for the xscale frame buffer.
 *
 *  Author:     Jean-Frederic Clere
 *  Created:    Sep 22, 2003
 *  Copyright:  jfclere@sinix.net
 */

/* Types supplied by the Linux framebuffer and device headers. */

pub const fn lcd_conn_type(x: u32) -> u32 { x & 0x0f }
pub const fn lcd_conn_width(x: u32) -> u32 { (x >> 4) & 0x1f }

pub const LCD_TYPE_MASK: u32 = 0xf;
pub const LCD_TYPE_UNKNOWN: u32 = 0;
pub const LCD_TYPE_MONO_STN: u32 = 1;
pub const LCD_TYPE_MONO_DSTN: u32 = 2;
pub const LCD_TYPE_COLOR_STN: u32 = 3;
pub const LCD_TYPE_COLOR_DSTN: u32 = 4;
pub const LCD_TYPE_COLOR_TFT: u32 = 5;
pub const LCD_TYPE_SMART_PANEL: u32 = 6;
pub const LCD_TYPE_MAX: u32 = 7;

pub const LCD_MONO_STN_4BPP: u32 = (4 << 4) | LCD_TYPE_MONO_STN;
pub const LCD_MONO_STN_8BPP: u32 = (8 << 4) | LCD_TYPE_MONO_STN;
pub const LCD_MONO_DSTN_8BPP: u32 = (8 << 4) | LCD_TYPE_MONO_DSTN;
pub const LCD_COLOR_STN_8BPP: u32 = (8 << 4) | LCD_TYPE_COLOR_STN;
pub const LCD_COLOR_DSTN_16BPP: u32 = (16 << 4) | LCD_TYPE_COLOR_DSTN;
pub const LCD_COLOR_TFT_8BPP: u32 = (8 << 4) | LCD_TYPE_COLOR_TFT;
pub const LCD_COLOR_TFT_16BPP: u32 = (16 << 4) | LCD_TYPE_COLOR_TFT;
pub const LCD_COLOR_TFT_18BPP: u32 = (18 << 4) | LCD_TYPE_COLOR_TFT;
pub const LCD_SMART_PANEL_8BPP: u32 = (8 << 4) | LCD_TYPE_SMART_PANEL;
pub const LCD_SMART_PANEL_16BPP: u32 = (16 << 4) | LCD_TYPE_SMART_PANEL;
pub const LCD_SMART_PANEL_18BPP: u32 = (18 << 4) | LCD_TYPE_SMART_PANEL;

pub const fn lcd_ac_bias_freq(x: u32) -> u32 { (x & 0xff) << 10 }
pub const LCD_BIAS_ACTIVE_HIGH: u32 = 0 << 18;
pub const LCD_BIAS_ACTIVE_LOW: u32 = 1 << 18;
pub const LCD_PCLK_EDGE_RISE: u32 = 0 << 19;
pub const LCD_PCLK_EDGE_FALL: u32 = 1 << 19;
pub const LCD_ALTERNATE_MAPPING: u32 = 1 << 20;

#[repr(C)]
pub struct pxafb_mode_info {
    pub pixclock: libc::c_ulong,
    pub xres: libc::c_ushort,
    pub yres: libc::c_ushort,
    pub bpp: libc::c_uchar,
    /* C bit-fields: cmap_greyscale:1, depth:8, transparency:1, unused:22. */
    pub flags: libc::c_uint,
    pub hsync_len: libc::c_uchar,
    pub left_margin: libc::c_uchar,
    pub right_margin: libc::c_uchar,
    pub vsync_len: libc::c_uchar,
    pub upper_margin: libc::c_uchar,
    pub lower_margin: libc::c_uchar,
    pub sync: libc::c_uchar,
    pub a0csrd_set_hld: libc::c_uint,
    pub a0cswr_set_hld: libc::c_uint,
    pub wr_pulse_width: libc::c_uint,
    pub rd_pulse_width: libc::c_uint,
    pub cmd_inh_time: libc::c_uint,
    pub op_hold_time: libc::c_uint,
}

#[repr(C)]
pub struct pxafb_mach_info {
    pub modes: *mut pxafb_mode_info,
    pub num_modes: libc::c_uint,
    pub lcd_conn: libc::c_uint,
    pub video_mem_size: libc::c_ulong,
    /* C bit-fields: fixed_modes:1, cmap_inverse:1, cmap_static:1,
     * acceleration_enabled:1, unused:28. */
    pub flags: libc::c_uint,
    pub lccr0: libc::c_uint,
    pub lccr3: libc::c_uint,
    pub lccr4: libc::c_uint,
    pub pxafb_backlight_power: Option<unsafe extern "C" fn(libc::c_int)>,
    pub pxafb_lcd_power: Option<unsafe extern "C" fn(libc::c_int, *mut fb_var_screeninfo)>,
    pub smart_update: Option<unsafe extern "C" fn(*mut fb_info)>,
}

unsafe extern "C" {
    pub fn pxa_set_fb_info(dev: *mut device, inf: *mut pxafb_mach_info);
}

pub const SMART_CMD_A0: u32 = 0x1 << 8;
pub const SMART_CMD_READ_STATUS_REG: u32 = 0x0 << 9;
pub const SMART_CMD_READ_FRAME_BUFFER: u32 = (0x0 << 9) | SMART_CMD_A0;
pub const SMART_CMD_WRITE_COMMAND: u32 = 0x1 << 9;
pub const SMART_CMD_WRITE_DATA: u32 = (0x1 << 9) | SMART_CMD_A0;
pub const SMART_CMD_WRITE_FRAME: u32 = (0x2 << 9) | SMART_CMD_A0;
pub const SMART_CMD_WAIT_FOR_VSYNC: u32 = 0x3 << 9;
pub const SMART_CMD_NOOP: u32 = 0x4 << 9;
pub const SMART_CMD_INTERRUPT: u32 = 0x5 << 9;
pub const fn smart_cmd(x: u32) -> u32 { SMART_CMD_WRITE_COMMAND | (x & 0xff) }
pub const fn smart_dat(x: u32) -> u32 { SMART_CMD_WRITE_DATA | (x & 0xff) }
pub const SMART_CMD_DELAY: u32 = 0x6 << 9;
pub const fn smart_delay(ms: u32) -> u32 { SMART_CMD_DELAY | (ms & 0xff) }

#[cfg(feature = "CONFIG_FB_PXA_SMARTPANEL")]
unsafe extern "C" {
    pub fn pxafb_smart_queue(info: *mut fb_info, cmds: *mut u16, n: libc::c_int) -> libc::c_int;
    pub fn pxafb_smart_flush(info: *mut fb_info) -> libc::c_int;
}

#[cfg(not(feature = "CONFIG_FB_PXA_SMARTPANEL"))]
pub unsafe extern "C" fn pxafb_smart_queue(
    _info: *mut fb_info, _cmds: *mut u16, _n: libc::c_int,
) -> libc::c_int { 0 }

#[cfg(not(feature = "CONFIG_FB_PXA_SMARTPANEL"))]
pub unsafe extern "C" fn pxafb_smart_flush(_info: *mut fb_info) -> libc::c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
