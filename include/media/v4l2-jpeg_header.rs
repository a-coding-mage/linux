/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * V4L2 JPEG helpers header
 *
 * Copyright (C) 2019 Pengutronix, Philipp Zabel <kernel@pengutronix.de>
 *
 * For reference, see JPEG ITU-T.81 (ISO/IEC 10918-1)
 */

// Dependency supplied by linux/v4l2-controls.h.

pub const V4L2_JPEG_MAX_COMPONENTS: usize = 4;
pub const V4L2_JPEG_MAX_TABLES: usize = 4;

/* Prefixes used to generate huffman table class and destination identifiers. */
pub const V4L2_JPEG_LUM_HT: u8 = 0x00;
pub const V4L2_JPEG_CHR_HT: u8 = 0x01;
pub const V4L2_JPEG_DC_HT: u8 = 0x00;
pub const V4L2_JPEG_AC_HT: u8 = 0x10;

/* Length of reference huffman tables as provided in Table K.3 of ITU-T.81 */
pub const V4L2_JPEG_REF_HT_AC_LEN: usize = 178;
pub const V4L2_JPEG_REF_HT_DC_LEN: usize = 28;

/* Array size for 8x8 block of samples or DCT coefficient */
pub const V4L2_JPEG_PIXELS_IN_BLOCK: usize = 64;

/**
 * struct v4l2_jpeg_reference - reference into the JPEG buffer
 * @start: pointer to the start of the referenced segment or table
 * @length: size of the referenced segment or table
 *
 * Wnen referencing marker segments, start points right after the marker code,
 * and length is the size of the segment parameters, excluding the marker code.
 */
#[repr(C)]
pub struct v4l2_jpeg_reference {
    pub start: *mut u8,
    pub length: usize,
}

#[repr(C)]
pub struct v4l2_jpeg_frame_component_spec {
    pub component_identifier: u8,
    pub horizontal_sampling_factor: u8,
    pub vertical_sampling_factor: u8,
    pub quantization_table_selector: u8,
}

#[repr(C)]
pub struct v4l2_jpeg_frame_header {
    pub height: u16,
    pub width: u16,
    pub precision: u8,
    pub num_components: u8,
    pub component: [v4l2_jpeg_frame_component_spec; V4L2_JPEG_MAX_COMPONENTS],
    pub subsampling: v4l2_jpeg_chroma_subsampling,
}

#[repr(C)]
pub struct v4l2_jpeg_scan_component_spec {
    pub component_selector: u8,
    pub dc_entropy_coding_table_selector: u8,
    pub ac_entropy_coding_table_selector: u8,
}

#[repr(C)]
pub struct v4l2_jpeg_scan_header {
    pub num_components: u8,
    pub component: [v4l2_jpeg_scan_component_spec; V4L2_JPEG_MAX_COMPONENTS],
}

#[repr(i32)]
pub enum v4l2_jpeg_app14_tf {
    V4L2_JPEG_APP14_TF_CMYK_RGB = 0,
    V4L2_JPEG_APP14_TF_YCBCR = 1,
    V4L2_JPEG_APP14_TF_YCCK = 2,
    V4L2_JPEG_APP14_TF_UNKNOWN = -1,
}

#[repr(C)]
pub struct v4l2_jpeg_header {
    pub sof: v4l2_jpeg_reference,
    pub sos: v4l2_jpeg_reference,
    pub num_dht: u32,
    pub dht: [v4l2_jpeg_reference; V4L2_JPEG_MAX_TABLES],
    pub num_dqt: u32,
    pub dqt: [v4l2_jpeg_reference; V4L2_JPEG_MAX_TABLES],
    pub frame: v4l2_jpeg_frame_header,
    pub scan: *mut v4l2_jpeg_scan_header,
    pub quantization_tables: *mut v4l2_jpeg_reference,
    pub huffman_tables: *mut v4l2_jpeg_reference,
    pub restart_interval: u16,
    pub ecs_offset: usize,
    pub app14_tf: v4l2_jpeg_app14_tf,
}

extern "C" {
    pub fn v4l2_jpeg_parse_header(
        buf: *mut core::ffi::c_void,
        len: usize,
        out: *mut v4l2_jpeg_header,
    ) -> i32;

    pub static v4l2_jpeg_zigzag_scan_index: [u8; V4L2_JPEG_PIXELS_IN_BLOCK];
    pub static v4l2_jpeg_ref_table_luma_qt: [u8; V4L2_JPEG_PIXELS_IN_BLOCK];
    pub static v4l2_jpeg_ref_table_chroma_qt: [u8; V4L2_JPEG_PIXELS_IN_BLOCK];
    pub static v4l2_jpeg_ref_table_luma_dc_ht: [u8; V4L2_JPEG_REF_HT_DC_LEN];
    pub static v4l2_jpeg_ref_table_luma_ac_ht: [u8; V4L2_JPEG_REF_HT_AC_LEN];
    pub static v4l2_jpeg_ref_table_chroma_dc_ht: [u8; V4L2_JPEG_REF_HT_DC_LEN];
    pub static v4l2_jpeg_ref_table_chroma_ac_ht: [u8; V4L2_JPEG_REF_HT_AC_LEN];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
