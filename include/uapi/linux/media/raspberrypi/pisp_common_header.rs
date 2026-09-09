/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * RP1 PiSP common definitions.
 *
 * Copyright (C) 2021 - Raspberry Pi Ltd.
 */

#[repr(C, packed)]
pub struct pisp_image_format_config {
    /* size in pixels */
    pub width: u16,
    pub height: u16,
    /* must match struct pisp_image_format below */
    pub format: u32,
    pub stride: i32,
    /* some planar image formats will need a second stride */
    pub stride2: i32,
}

#[repr(u32)]
pub enum pisp_bayer_order {
    /*
     * Note how bayer_order&1 tells you if G is on the even pixels of the
     * checkerboard or not, and bayer_order&2 tells you if R is on the even
     * rows or is swapped with B. Note that if the top (of the 8) bits is
     * set, this denotes a monochrome or greyscale image, and the lower bits
     * should all be ignored.
     */
    PISP_BAYER_ORDER_RGGB = 0,
    PISP_BAYER_ORDER_GBRG = 1,
    PISP_BAYER_ORDER_BGGR = 2,
    PISP_BAYER_ORDER_GRBG = 3,
    PISP_BAYER_ORDER_GREYSCALE = 128,
}

#[repr(u32)]
pub enum pisp_image_format {
    /* Precise values are mostly tbd. Generally these will be portmanteau
     * values comprising bit fields and flags. This format must be shared
     * throughout the PiSP.
     */
    PISP_IMAGE_FORMAT_BPS_8 = 0x00000000,
    PISP_IMAGE_FORMAT_BPS_10 = 0x00000001,
    PISP_IMAGE_FORMAT_BPS_12 = 0x00000002,
    PISP_IMAGE_FORMAT_BPS_16 = 0x00000003,
    PISP_IMAGE_FORMAT_BPS_MASK = 0x00000003,
    PISP_IMAGE_FORMAT_PLANARITY_INTERLEAVED = 0x00000000,
    PISP_IMAGE_FORMAT_PLANARITY_SEMI_PLANAR = 0x00000010,
    PISP_IMAGE_FORMAT_PLANARITY_PLANAR = 0x00000020,
    PISP_IMAGE_FORMAT_PLANARITY_MASK = 0x00000030,
    PISP_IMAGE_FORMAT_SAMPLING_444 = 0x00000000,
    PISP_IMAGE_FORMAT_SAMPLING_422 = 0x00000100,
    PISP_IMAGE_FORMAT_SAMPLING_420 = 0x00000200,
    PISP_IMAGE_FORMAT_SAMPLING_MASK = 0x00000300,
    PISP_IMAGE_FORMAT_ORDER_NORMAL = 0x00000000,
    PISP_IMAGE_FORMAT_ORDER_SWAPPED = 0x00001000,
    PISP_IMAGE_FORMAT_SHIFT_0 = 0x00000000,
    PISP_IMAGE_FORMAT_SHIFT_1 = 0x00010000,
    PISP_IMAGE_FORMAT_SHIFT_2 = 0x00020000,
    PISP_IMAGE_FORMAT_SHIFT_3 = 0x00030000,
    PISP_IMAGE_FORMAT_SHIFT_4 = 0x00040000,
    PISP_IMAGE_FORMAT_SHIFT_5 = 0x00050000,
    PISP_IMAGE_FORMAT_SHIFT_6 = 0x00060000,
    PISP_IMAGE_FORMAT_SHIFT_7 = 0x00070000,
    PISP_IMAGE_FORMAT_SHIFT_8 = 0x00080000,
    PISP_IMAGE_FORMAT_SHIFT_MASK = 0x000f0000,
    PISP_IMAGE_FORMAT_BPP_32 = 0x00100000,
    PISP_IMAGE_FORMAT_UNCOMPRESSED = 0x00000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MODE_1 = 0x01000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MODE_2 = 0x02000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MODE_3 = 0x03000000,
    PISP_IMAGE_FORMAT_COMPRESSION_MASK = 0x03000000,
    PISP_IMAGE_FORMAT_HOG_SIGNED = 0x04000000,
    PISP_IMAGE_FORMAT_HOG_UNSIGNED = 0x08000000,
    PISP_IMAGE_FORMAT_INTEGRAL_IMAGE = 0x10000000,
    PISP_IMAGE_FORMAT_WALLPAPER_ROLL = 0x20000000,
    PISP_IMAGE_FORMAT_THREE_CHANNEL = 0x40000000,
    PISP_IMAGE_FORMAT_SINGLE_16 = PISP_IMAGE_FORMAT_BPS_16,
    PISP_IMAGE_FORMAT_THREE_16 = PISP_IMAGE_FORMAT_BPS_16 as u32 | PISP_IMAGE_FORMAT_THREE_CHANNEL as u32,
}

macro_rules! PISP_IMAGE_FORMAT_BPS_8 { ($fmt:expr) => { (($fmt & pisp_image_format::PISP_IMAGE_FORMAT_BPS_MASK as u32) == pisp_image_format::PISP_IMAGE_FORMAT_BPS_8 as u32) }; }
macro_rules! PISP_IMAGE_FORMAT_BPS_10 { ($fmt:expr) => { (($fmt & 3) == 1) }; }
macro_rules! PISP_IMAGE_FORMAT_BPS_12 { ($fmt:expr) => { (($fmt & 3) == 2) }; }
macro_rules! PISP_IMAGE_FORMAT_BPS_16 { ($fmt:expr) => { (($fmt & 3) == 3) }; }
macro_rules! PISP_IMAGE_FORMAT_BPS { ($fmt:expr) => { if ($fmt & 3) != 0 { 8 + (2 << (($fmt & 3) - 1)) } else { 8 } }; }
macro_rules! PISP_IMAGE_FORMAT_SHIFT { ($fmt:expr) => { (($fmt & 0x000f0000) / 0x00010000) }; }
macro_rules! PISP_IMAGE_FORMAT_THREE_CHANNEL { ($fmt:expr) => { ($fmt & 0x40000000) }; }
macro_rules! PISP_IMAGE_FORMAT_SINGLE_CHANNEL { ($fmt:expr) => { !(($fmt & 0x40000000) != 0) }; }
macro_rules! PISP_IMAGE_FORMAT_COMPRESSED { ($fmt:expr) => { (($fmt & 0x03000000) != 0) }; }
macro_rules! PISP_IMAGE_FORMAT_SAMPLING_444 { ($fmt:expr) => { (($fmt & 0x00000300) == 0) }; }
macro_rules! PISP_IMAGE_FORMAT_SAMPLING_422 { ($fmt:expr) => { (($fmt & 0x00000300) == 0x00000100) }; }
macro_rules! PISP_IMAGE_FORMAT_SAMPLING_420 { ($fmt:expr) => { (($fmt & 0x00000300) == 0x00000200) }; }
macro_rules! PISP_IMAGE_FORMAT_ORDER_NORMAL { ($fmt:expr) => { !(($fmt & 0x00001000) != 0) }; }
macro_rules! PISP_IMAGE_FORMAT_ORDER_SWAPPED { ($fmt:expr) => { ($fmt & 0x00001000) }; }
macro_rules! PISP_IMAGE_FORMAT_INTERLEAVED { ($fmt:expr) => { (($fmt & 0x30) == 0) }; }
macro_rules! PISP_IMAGE_FORMAT_SEMIPLANAR { ($fmt:expr) => { (($fmt & 0x30) == 0x10) }; }
macro_rules! PISP_IMAGE_FORMAT_PLANAR { ($fmt:expr) => { (($fmt & 0x30) == 0x20) }; }
macro_rules! PISP_IMAGE_FORMAT_WALLPAPER { ($fmt:expr) => { ($fmt & 0x20000000) }; }
macro_rules! PISP_IMAGE_FORMAT_BPP_32 { ($fmt:expr) => { ($fmt & 0x00100000) }; }
macro_rules! PISP_IMAGE_FORMAT_HOG { ($fmt:expr) => { ($fmt & (0x04000000 | 0x08000000)) }; }

pub const PISP_WALLPAPER_WIDTH: u32 = 128; /* in bytes */

#[repr(C, packed)]
pub struct pisp_bla_config {
    pub black_level_r: u16,
    pub black_level_gr: u16,
    pub black_level_gb: u16,
    pub black_level_b: u16,
    pub output_black_level: u16,
    pub pad: [u8; 2],
}

#[repr(C, packed)]
pub struct pisp_wbg_config {
    pub gain_r: u16,
    pub gain_g: u16,
    pub gain_b: u16,
    pub pad: [u8; 2],
}

#[repr(C, packed)]
pub struct pisp_compress_config {
    /* value subtracted from incoming data */
    pub offset: u16,
    pub pad: u8,
    /* 1 => Companding; 2 => Delta (recommended); 3 => Combined (for HDR) */
    pub mode: u8,
}

#[repr(C, packed)]
pub struct pisp_decompress_config {
    /* value added to reconstructed data */
    pub offset: u16,
    pub pad: u8,
    /* 1 => Companding; 2 => Delta (recommended); 3 => Combined (for HDR) */
    pub mode: u8,
}

#[repr(u32)]
pub enum pisp_axi_flags {
    /* round down bursts to end at a 32-byte boundary, to align following bursts */
    PISP_AXI_FLAG_ALIGN = 128,
    /* for FE writer: force WSTRB high, to pad output to 16-byte boundary */
    PISP_AXI_FLAG_PAD = 64,
    /* for FE writer: Use Output FIFO level to trigger "panic" */
    PISP_AXI_FLAG_PANIC = 32,
}

#[repr(C, packed)]
pub struct pisp_axi_config {
    /* burst length minus one, which must be in the range 0:15; OR'd with flags */
    pub maxlen_flags: u8,
    /* { prot[2:0], cache[3:0] } fields, echoed on AXI bus */
    pub cache_prot: u8,
    /* QoS field(s) (4x4 bits for FE writer; 4 bits for other masters) */
    pub qos: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
