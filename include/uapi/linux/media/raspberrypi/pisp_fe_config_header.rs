/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * RP1 PiSP Front End Driver Configuration structures
 *
 * Copyright (C) 2021 - Raspberry Pi Ltd.
 *
 */

// Dependencies supplied by the surrounding UAPI translation:
// linux/types.h, pisp_common.h, and pisp_fe_statistics.h.

pub const PISP_FE_NUM_OUTPUTS: usize = 2;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pisp_fe_enable {

    PISP_FE_ENABLE_INPUT = 0x000001,
    PISP_FE_ENABLE_DECOMPRESS = 0x000002,
    PISP_FE_ENABLE_DECOMPAND = 0x000004,
    PISP_FE_ENABLE_BLA = 0x000008,
    PISP_FE_ENABLE_DPC = 0x000010,
    PISP_FE_ENABLE_STATS_CROP = 0x000020,
    PISP_FE_ENABLE_DECIMATE = 0x000040,
    PISP_FE_ENABLE_BLC = 0x000080,
    PISP_FE_ENABLE_CDAF_STATS = 0x000100,
    PISP_FE_ENABLE_AWB_STATS = 0x000200,
    PISP_FE_ENABLE_RGBY = 0x000400,
    PISP_FE_ENABLE_LSC = 0x000800,
    PISP_FE_ENABLE_AGC_STATS = 0x001000,
    PISP_FE_ENABLE_CROP0 = 0x010000,
    PISP_FE_ENABLE_DOWNSCALE0 = 0x020000,
    PISP_FE_ENABLE_COMPRESS0 = 0x040000,
    PISP_FE_ENABLE_OUTPUT0 = 0x080000,
    PISP_FE_ENABLE_CROP1 = 0x100000,
    PISP_FE_ENABLE_DOWNSCALE1 = 0x200000,
    PISP_FE_ENABLE_COMPRESS1 = 0x400000,
    PISP_FE_ENABLE_OUTPUT1 = 0x800000,
}

pub const fn PISP_FE_ENABLE_CROP(i: u32) -> u32 { (pisp_fe_enable::PISP_FE_ENABLE_CROP0 as u32) << (4 * i) }
pub const fn PISP_FE_ENABLE_DOWNSCALE(i: u32) -> u32 { (pisp_fe_enable::PISP_FE_ENABLE_DOWNSCALE0 as u32) << (4 * i) }
pub const fn PISP_FE_ENABLE_COMPRESS(i: u32) -> u32 { (pisp_fe_enable::PISP_FE_ENABLE_COMPRESS0 as u32) << (4 * i) }
pub const fn PISP_FE_ENABLE_OUTPUT(i: u32) -> u32 { (pisp_fe_enable::PISP_FE_ENABLE_OUTPUT0 as u32) << (4 * i) }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pisp_fe_dirty {
    PISP_FE_DIRTY_GLOBAL = 0x0001,
    PISP_FE_DIRTY_FLOATING = 0x0002,
    PISP_FE_DIRTY_OUTPUT_AXI = 0x0004,
}

#[repr(C, packed)] pub struct pisp_fe_global_config { pub enables: u32, pub bayer_order: u8, pub pad: [u8; 3] }
#[repr(C, packed)] pub struct pisp_fe_input_axi_config { pub maxlen_flags: u8, pub cache_prot: u8, pub qos: u16 }
#[repr(C, packed)] pub struct pisp_fe_output_axi_config { pub maxlen_flags: u8, pub cache_prot: u8, pub qos: u16, pub thresh: u16, pub throttle: u16 }
#[repr(C, packed)] pub struct pisp_fe_input_config { pub streaming: u8, pub pad: [u8; 3], pub format: pisp_image_format_config, pub axi: pisp_fe_input_axi_config, pub holdoff: u8, pub pad2: [u8; 3] }
#[repr(C, packed)] pub struct pisp_fe_output_config { pub format: pisp_image_format_config, pub ilines: u16, pub pad: [u8; 2] }
#[repr(C, packed)] pub struct pisp_fe_input_buffer_config { pub addr_lo: u32, pub addr_hi: u32, pub frame_id: u16, pub pad: u16 }

pub const PISP_FE_DECOMPAND_LUT_SIZE: usize = 65;
#[repr(C, packed)] pub struct pisp_fe_decompand_config { pub lut: [u16; PISP_FE_DECOMPAND_LUT_SIZE], pub pad: u16 }

pub const PISP_FE_DPC_FLAG_FOLDBACK: u8 = 1;
pub const PISP_FE_DPC_FLAG_VFLAG: u8 = 2;
#[repr(C, packed)] pub struct pisp_fe_dpc_config { pub coeff_level: u8, pub coeff_range: u8, pub coeff_range2: u8, pub flags: u8 }

pub const PISP_FE_LSC_LUT_SIZE: usize = 16;
#[repr(C, packed)] pub struct pisp_fe_lsc_config { pub shift: u8, pub pad0: u8, pub scale: u16, pub centre_x: u16, pub centre_y: u16, pub lut: [u16; PISP_FE_LSC_LUT_SIZE] }
#[repr(C, packed)] pub struct pisp_fe_rgby_config { pub gain_r: u16, pub gain_g: u16, pub gain_b: u16, pub maxflag: u8, pub pad: u8 }
#[repr(C, packed)] pub struct pisp_fe_agc_stats_config { pub offset_x: u16, pub offset_y: u16, pub size_x: u16, pub size_y: u16, pub weights: [u8; PISP_AGC_STATS_NUM_ZONES / 2], pub row_offset_x: u16, pub row_offset_y: u16, pub row_size_x: u16, pub row_size_y: u16, pub row_shift: u8, pub float_shift: u8, pub pad1: [u8; 2] }
#[repr(C, packed)] pub struct pisp_fe_awb_stats_config { pub offset_x: u16, pub offset_y: u16, pub size_x: u16, pub size_y: u16, pub shift: u8, pub pad: [u8; 3], pub r_lo: u16, pub r_hi: u16, pub g_lo: u16, pub g_hi: u16, pub b_lo: u16, pub b_hi: u16 }
#[repr(C, packed)] pub struct pisp_fe_floating_stats_region { pub offset_x: u16, pub offset_y: u16, pub size_x: u16, pub size_y: u16 }
#[repr(C, packed)] pub struct pisp_fe_floating_stats_config { pub regions: [pisp_fe_floating_stats_region; PISP_FLOATING_STATS_NUM_ZONES] }
pub const PISP_FE_CDAF_NUM_WEIGHTS: usize = 8;
#[repr(C, packed)] pub struct pisp_fe_cdaf_stats_config { pub noise_constant: u16, pub noise_slope: u16, pub offset_x: u16, pub offset_y: u16, pub size_x: u16, pub size_y: u16, pub skip_x: u16, pub skip_y: u16, pub mode: u32 }
#[repr(C, packed)] pub struct pisp_fe_stats_buffer_config { pub addr_lo: u32, pub addr_hi: u32 }
#[repr(C, packed)] pub struct pisp_fe_crop_config { pub offset_x: u16, pub offset_y: u16, pub width: u16, pub height: u16 }

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pisp_fe_downscale_flags { DOWNSCALE_BAYER = 1, DOWNSCALE_BIN = 2 }
#[repr(C, packed)] pub struct pisp_fe_downscale_config { pub xin: u8, pub xout: u8, pub yin: u8, pub yout: u8, pub flags: u8, pub pad: [u8; 3], pub output_width: u16, pub output_height: u16 }
#[repr(C, packed)] pub struct pisp_fe_output_buffer_config { pub addr_lo: u32, pub addr_hi: u32 }

/* Each of the two output channels/branches: */
#[repr(C, packed)] pub struct pisp_fe_output_branch_config { pub crop: pisp_fe_crop_config, pub downscale: pisp_fe_downscale_config, pub compress: pisp_compress_config, pub output: pisp_fe_output_config, pub pad: u32 }

/* And finally one to rule them all: */
#[repr(C, packed)] pub struct pisp_fe_config {
    /* I/O configuration: */
    pub stats_buffer: pisp_fe_stats_buffer_config,
    pub output_buffer: [pisp_fe_output_buffer_config; PISP_FE_NUM_OUTPUTS],
    pub input_buffer: pisp_fe_input_buffer_config,
    /* processing configuration: */
    pub global: pisp_fe_global_config,
    pub input: pisp_fe_input_config,
    pub decompress: pisp_decompress_config,
    pub decompand: pisp_fe_decompand_config,
    pub bla: pisp_bla_config,
    pub dpc: pisp_fe_dpc_config,
    pub stats_crop: pisp_fe_crop_config,
    pub spare1: u32,
    pub blc: pisp_bla_config,
    pub rgby: pisp_fe_rgby_config,
    pub lsc: pisp_fe_lsc_config,
    pub agc_stats: pisp_fe_agc_stats_config,
    pub awb_stats: pisp_fe_awb_stats_config,
    pub cdaf_stats: pisp_fe_cdaf_stats_config,
    pub floating_stats: pisp_fe_floating_stats_config,
    pub output_axi: pisp_fe_output_axi_config,
    pub ch: [pisp_fe_output_branch_config; PISP_FE_NUM_OUTPUTS],
    /* non-register fields: */
    pub dirty_flags: u32,
    pub dirty_flags_extra: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
