/* SPDX-License-Identifier: GPL-2.0-only */
/* Media Bus API header. */

// Dependencies supplied by the corresponding V4L2 definitions are intentionally external.

/* Parallel flags and serial flags. */
pub const V4L2_MBUS_MASTER: u32 = 1u32 << 0;
pub const V4L2_MBUS_SLAVE: u32 = 1u32 << 1;
pub const V4L2_MBUS_HSYNC_ACTIVE_HIGH: u32 = 1u32 << 2;
pub const V4L2_MBUS_HSYNC_ACTIVE_LOW: u32 = 1u32 << 3;
pub const V4L2_MBUS_VSYNC_ACTIVE_HIGH: u32 = 1u32 << 4;
pub const V4L2_MBUS_VSYNC_ACTIVE_LOW: u32 = 1u32 << 5;
pub const V4L2_MBUS_PCLK_SAMPLE_RISING: u32 = 1u32 << 6;
pub const V4L2_MBUS_PCLK_SAMPLE_FALLING: u32 = 1u32 << 7;
pub const V4L2_MBUS_PCLK_SAMPLE_DUALEDGE: u32 = 1u32 << 8;
pub const V4L2_MBUS_DATA_ACTIVE_HIGH: u32 = 1u32 << 9;
pub const V4L2_MBUS_DATA_ACTIVE_LOW: u32 = 1u32 << 10;
pub const V4L2_MBUS_FIELD_EVEN_HIGH: u32 = 1u32 << 11;
pub const V4L2_MBUS_FIELD_EVEN_LOW: u32 = 1u32 << 12;
pub const V4L2_MBUS_VIDEO_SOG_ACTIVE_HIGH: u32 = 1u32 << 13;
pub const V4L2_MBUS_VIDEO_SOG_ACTIVE_LOW: u32 = 1u32 << 14;
pub const V4L2_MBUS_DATA_ENABLE_HIGH: u32 = 1u32 << 15;
pub const V4L2_MBUS_DATA_ENABLE_LOW: u32 = 1u32 << 16;
pub const V4L2_MBUS_CSI2_NONCONTINUOUS_CLOCK: u32 = 1u32 << 0;
pub const V4L2_MBUS_CSI2_MAX_DATA_LANES: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum v4l2_mbus_csi2_cphy_line_orders_type {
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_ABC,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_ACB,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_BAC,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_BCA,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_CAB,
    V4L2_MBUS_CSI2_CPHY_LINE_ORDER_CBA,
}

#[repr(C)]
pub struct v4l2_mbus_config_mipi_csi2 {
    pub flags: u32,
    pub data_lanes: [u8; V4L2_MBUS_CSI2_MAX_DATA_LANES],
    pub clock_lane: u8,
    pub num_data_lanes: u8,
    pub lane_polarities: [bool; 1 + V4L2_MBUS_CSI2_MAX_DATA_LANES],
    pub line_orders: [v4l2_mbus_csi2_cphy_line_orders_type; V4L2_MBUS_CSI2_MAX_DATA_LANES],
}

#[repr(C)]
pub struct v4l2_mbus_config_parallel {
    pub flags: u32,
    pub bus_width: u8,
    pub data_shift: u8,
}

#[repr(C)]
pub struct v4l2_mbus_config_mipi_csi1 {
    // C bit-fields clock_inv:1 and strobe:1, represented in their containing byte.
    pub clock_inv: u8,
    pub strobe: u8,
    pub lane_polarity: [bool; 2],
    pub data_lane: u8,
    pub clock_lane: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum v4l2_mbus_type {
    V4L2_MBUS_UNKNOWN,
    V4L2_MBUS_PARALLEL,
    V4L2_MBUS_BT656,
    V4L2_MBUS_CSI1,
    V4L2_MBUS_CCP2,
    V4L2_MBUS_CSI2_DPHY,
    V4L2_MBUS_CSI2_CPHY,
    V4L2_MBUS_DPI,
    V4L2_MBUS_INVALID,
}

#[repr(C)]
pub union v4l2_mbus_config_bus {
    pub parallel: v4l2_mbus_config_parallel,
    pub mipi_csi1: v4l2_mbus_config_mipi_csi1,
    pub mipi_csi2: v4l2_mbus_config_mipi_csi2,
}

#[repr(C)]
pub struct v4l2_mbus_config {
    pub type_: v4l2_mbus_type,
    pub link_freq: u64,
    pub bus: v4l2_mbus_config_bus,
}

pub unsafe fn v4l2_fill_pix_format(
    pix_fmt: *mut v4l2_pix_format,
    mbus_fmt: *const v4l2_mbus_framefmt,
) {
    (*pix_fmt).width = (*mbus_fmt).width;
    (*pix_fmt).height = (*mbus_fmt).height;
    (*pix_fmt).field = (*mbus_fmt).field;
    (*pix_fmt).colorspace = (*mbus_fmt).colorspace;
    (*pix_fmt).ycbcr_enc = (*mbus_fmt).ycbcr_enc;
    (*pix_fmt).quantization = (*mbus_fmt).quantization;
    (*pix_fmt).xfer_func = (*mbus_fmt).xfer_func;
}

pub unsafe fn v4l2_fill_mbus_format(
    mbus_fmt: *mut v4l2_mbus_framefmt,
    pix_fmt: *const v4l2_pix_format,
    code: u32,
) {
    (*mbus_fmt).width = (*pix_fmt).width;
    (*mbus_fmt).height = (*pix_fmt).height;
    (*mbus_fmt).field = (*pix_fmt).field;
    (*mbus_fmt).colorspace = (*pix_fmt).colorspace;
    (*mbus_fmt).ycbcr_enc = (*pix_fmt).ycbcr_enc;
    (*mbus_fmt).quantization = (*pix_fmt).quantization;
    (*mbus_fmt).xfer_func = (*pix_fmt).xfer_func;
    (*mbus_fmt).code = code;
}

pub unsafe fn v4l2_fill_pix_format_mplane(
    pix_mp_fmt: *mut v4l2_pix_format_mplane,
    mbus_fmt: *const v4l2_mbus_framefmt,
) {
    (*pix_mp_fmt).width = (*mbus_fmt).width;
    (*pix_mp_fmt).height = (*mbus_fmt).height;
    (*pix_mp_fmt).field = (*mbus_fmt).field;
    (*pix_mp_fmt).colorspace = (*mbus_fmt).colorspace;
    (*pix_mp_fmt).ycbcr_enc = (*mbus_fmt).ycbcr_enc;
    (*pix_mp_fmt).quantization = (*mbus_fmt).quantization;
    (*pix_mp_fmt).xfer_func = (*mbus_fmt).xfer_func;
}

pub unsafe fn v4l2_fill_mbus_format_mplane(
    mbus_fmt: *mut v4l2_mbus_framefmt,
    pix_mp_fmt: *const v4l2_pix_format_mplane,
) {
    (*mbus_fmt).width = (*pix_mp_fmt).width;
    (*mbus_fmt).height = (*pix_mp_fmt).height;
    (*mbus_fmt).field = (*pix_mp_fmt).field;
    (*mbus_fmt).colorspace = (*pix_mp_fmt).colorspace;
    (*mbus_fmt).ycbcr_enc = (*pix_mp_fmt).ycbcr_enc;
    (*mbus_fmt).quantization = (*pix_mp_fmt).quantization;
    (*mbus_fmt).xfer_func = (*pix_mp_fmt).xfer_func;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
