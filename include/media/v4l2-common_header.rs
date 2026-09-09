/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of v4l2-common.h. C header dependencies are supplied externally. */

#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_device { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_subdev { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_subdev_ops { _private: [u8; 0] }
#[repr(C)] pub struct i2c_adapter { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { _private: [u8; 0] }
#[repr(C)] pub struct i2c_board_info { _private: [u8; 0] }
#[repr(C)] pub struct spi_controller { _private: [u8; 0] }
#[repr(C)] pub struct spi_board_info { _private: [u8; 0] }
#[repr(C)] pub struct spi_device { _private: [u8; 0] }
#[repr(C)] pub struct video_device { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_streamparm { _private: [u8; 0] }
#[repr(C)] pub struct media_pad { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_frmsize_stepwise { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_pix_format { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_pix_format_mplane { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_buffer { _private: [u8; 0] }
#[repr(C)] pub struct v4l2_queryctrl { _private: [u8; 0] }

pub type s32 = i32; pub type u8_ = u8; pub type u32_ = u32; pub type u64_ = u64; pub type s64 = i64;

/* printk helper macros retain their C-side expansion contract. */
#[macro_export] macro_rules! v4l_printk { ($level:expr, $name:expr, $adapter:expr, $addr:expr, $fmt:expr $(, $arg:expr)*) => { printk!(concat!($level, "%s %d-%04x: ", $fmt), $name, i2c_adapter_id!($adapter), $addr $(, $arg)*) } }
#[macro_export] macro_rules! v4l_client_printk { ($level:expr, $client:expr, $fmt:expr $(, $arg:expr)*) => { v4l_printk!($level, ($client).dev.driver.name, ($client).adapter, ($client).addr, $fmt $(, $arg)*) } }
#[macro_export] macro_rules! v4l_err { ($client:expr, $fmt:expr $(, $arg:expr)*) => { v4l_client_printk!(KERN_ERR, $client, $fmt $(, $arg)*) } }
#[macro_export] macro_rules! v4l_warn { ($client:expr, $fmt:expr $(, $arg:expr)*) => { v4l_client_printk!(KERN_WARNING, $client, $fmt $(, $arg)*) } }
#[macro_export] macro_rules! v4l_info { ($client:expr, $fmt:expr $(, $arg:expr)*) => { v4l_client_printk!(KERN_INFO, $client, $fmt $(, $arg)*) } }
#[macro_export] macro_rules! v4l2_printk { ($level:expr, $dev:expr, $fmt:expr $(, $arg:expr)*) => { printk!(concat!($level, "%s: ", $fmt), ($dev).name $(, $arg)*) } }
#[macro_export] macro_rules! v4l2_err { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { v4l2_printk!(KERN_ERR, $dev, $fmt $(, $arg)*) } }
#[macro_export] macro_rules! v4l2_warn { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { v4l2_printk!(KERN_WARNING, $dev, $fmt $(, $arg)*) } }
#[macro_export] macro_rules! v4l2_info { ($dev:expr, $fmt:expr $(, $arg:expr)*) => { v4l2_printk!(KERN_INFO, $dev, $fmt $(, $arg)*) } }

/* Build-time CONFIG_VIDEO_V4L2_I2C and CONFIG_SPI branches are represented by
 * the external declarations above; disabled configurations provide NULL/no-op
 * definitions in the consuming kernel binding. */

extern "C" {
    pub fn v4l2_ctrl_query_fill(qctrl: *mut v4l2_queryctrl, min: s32, max: s32, step: s32, def: s32) -> i32;
    pub fn v4l_bound_align_image(width: *mut u32, wmin: u32, wmax: u32, walign: u32, height: *mut u32, hmin: u32, hmax: u32, halign: u32, salign: u32);
    pub fn __v4l2_find_nearest_size_conditional(array: *const core::ffi::c_void, array_size: usize, entry_size: usize, width_offset: usize, height_offset: usize, width: s32, height: s32, func: Option<unsafe extern "C" fn(*const core::ffi::c_void, usize, *const core::ffi::c_void) -> bool>, context: *const core::ffi::c_void) -> *const core::ffi::c_void;
    pub fn v4l2_g_parm_cap(vdev: *mut video_device, sd: *mut v4l2_subdev, a: *mut v4l2_streamparm) -> i32;
    pub fn v4l2_s_parm_cap(vdev: *mut video_device, sd: *mut v4l2_subdev, a: *mut v4l2_streamparm) -> i32;
    pub fn v4l2_format_info(format: u32) -> *const v4l2_format_info;
    pub fn v4l2_apply_frmsize_constraints(width: *mut u32, height: *mut u32, frmsize: *const v4l2_frmsize_stepwise);
    pub fn v4l2_fill_pixfmt(pixfmt: *mut v4l2_pix_format, pixelformat: u32, width: u32, height: u32) -> i32;
    pub fn v4l2_fill_pixfmt_mp(pixfmt: *mut v4l2_pix_format_mplane, pixelformat: u32, width: u32, height: u32) -> i32;
    pub fn v4l2_fill_pixfmt_mp_aligned(pixfmt: *mut v4l2_pix_format_mplane, pixelformat: u32, width: u32, height: u32, stride_alignment: u8) -> i32;
    pub fn v4l2_simplify_fraction(numerator: *mut u32, denominator: *mut u32, n_terms: u32, threshold: u32);
    pub fn v4l2_fraction_to_interval(numerator: u32, denominator: u32) -> u32;
    pub fn v4l2_link_freq_to_bitmap(dev: *mut device, fw: *const u64, nfw: u32, driver: *const s64, nd: u32, bitmap: *mut usize) -> i32;
    pub fn __devm_v4l2_sensor_clk_get(dev: *mut device, id: *const i8, legacy: bool, fixed_rate: bool, clk_rate: usize) -> *mut clk;
}

#[repr(C)] pub struct v4l2_i2c_tuner_type(pub u32);
pub const ADDRS_RADIO: v4l2_i2c_tuner_type = v4l2_i2c_tuner_type(0);
pub const ADDRS_DEMOD: v4l2_i2c_tuner_type = v4l2_i2c_tuner_type(1);
pub const ADDRS_TV: v4l2_i2c_tuner_type = v4l2_i2c_tuner_type(2);
pub const ADDRS_TV_WITH_DEMOD: v4l2_i2c_tuner_type = v4l2_i2c_tuner_type(3);

extern "C" {
    pub fn v4l2_i2c_new_subdev(v4l2_dev: *mut v4l2_device, adapter: *mut i2c_adapter, client_type: *const i8, addr: u8, probe_addrs: *const u16) -> *mut v4l2_subdev;
    pub fn v4l2_i2c_new_subdev_board(v4l2_dev: *mut v4l2_device, adapter: *mut i2c_adapter, info: *mut i2c_board_info, probe_addrs: *const u16) -> *mut v4l2_subdev;
    pub fn v4l2_i2c_subdev_set_name(sd: *mut v4l2_subdev, client: *mut i2c_client, devname: *const i8, postfix: *const i8);
    pub fn v4l2_i2c_subdev_init(sd: *mut v4l2_subdev, client: *mut i2c_client, ops: *const v4l2_subdev_ops);
    pub fn v4l2_i2c_subdev_addr(sd: *mut v4l2_subdev) -> u16;
    pub fn v4l2_i2c_tuner_addrs(ty: v4l2_i2c_tuner_type) -> *const u16;
    pub fn v4l2_i2c_subdev_unregister(sd: *mut v4l2_subdev);
    pub fn v4l2_spi_new_subdev(v4l2_dev: *mut v4l2_device, ctlr: *mut spi_controller, info: *mut spi_board_info) -> *mut v4l2_subdev;
    pub fn v4l2_spi_subdev_init(sd: *mut v4l2_subdev, spi: *mut spi_device, ops: *const v4l2_subdev_ops);
    pub fn v4l2_spi_subdev_unregister(sd: *mut v4l2_subdev);
}

#[repr(C)] pub struct v4l2_priv_tun_config { pub tuner: i32, pub priv_: *mut core::ffi::c_void }
pub const V4L2_PIXEL_ENC_UNKNOWN: u8 = 0; pub const V4L2_PIXEL_ENC_YUV: u8 = 1; pub const V4L2_PIXEL_ENC_RGB: u8 = 2; pub const V4L2_PIXEL_ENC_BAYER: u8 = 3;

#[repr(C)] pub struct v4l2_format_info { pub format: u32, pub pixel_enc: u8, pub mem_planes: u8, pub comp_planes: u8, pub bpp: [u8;4], pub bpp_div: [u8;4], pub hdiv: u8, pub vdiv: u8, pub block_w: [u8;4], pub block_h: [u8;4], pub has_alpha: bool }
#[inline] pub unsafe fn v4l2_is_format_rgb(f: *const v4l2_format_info) -> bool { !f.is_null() && (*f).pixel_enc == V4L2_PIXEL_ENC_RGB }
#[inline] pub unsafe fn v4l2_is_format_yuv(f: *const v4l2_format_info) -> bool { !f.is_null() && (*f).pixel_enc == V4L2_PIXEL_ENC_YUV }
#[inline] pub unsafe fn v4l2_is_format_bayer(f: *const v4l2_format_info) -> bool { !f.is_null() && (*f).pixel_enc == V4L2_PIXEL_ENC_BAYER }

#[inline] pub unsafe fn devm_v4l2_sensor_clk_get(dev: *mut device, id: *const i8) -> *mut clk { __devm_v4l2_sensor_clk_get(dev, id, false, false, 0) }
#[inline] pub unsafe fn devm_v4l2_sensor_clk_get_legacy(dev: *mut device, id: *const i8, fixed_rate: bool, clk_rate: usize) -> *mut clk { __devm_v4l2_sensor_clk_get(dev, id, true, fixed_rate, clk_rate) }

#[inline] pub fn v4l2_is_colorspace_valid(c: u32) -> bool { c > V4L2_COLORSPACE_DEFAULT && c < V4L2_COLORSPACE_LAST }
#[inline] pub fn v4l2_is_xfer_func_valid(c: u32) -> bool { c > V4L2_XFER_FUNC_DEFAULT && c < V4L2_XFER_FUNC_LAST }
#[inline] pub fn v4l2_is_ycbcr_enc_valid(c: u8) -> bool { c > V4L2_YCBCR_ENC_DEFAULT && c < V4L2_YCBCR_ENC_LAST }
#[inline] pub fn v4l2_is_hsv_enc_valid(c: u8) -> bool { c == V4L2_HSV_ENC_180 || c == V4L2_HSV_ENC_256 }
#[inline] pub fn v4l2_is_quant_valid(c: u8) -> bool { c == V4L2_QUANTIZATION_FULL_RANGE || c == V4L2_QUANTIZATION_LIM_RANGE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
