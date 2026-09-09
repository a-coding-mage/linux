// SPDX-License-Identifier: MIT
// Faithful Rust translation of uapi/drm/drm_fourcc.h.
#![allow(non_upper_case_globals, non_snake_case, dead_code, unused_macros)]

pub const fn fourcc_code(a: u32, b: u32, c: u32, d: u32) -> u32 { a | (b << 8) | (c << 16) | (d << 24) }
pub const DRM_FORMAT_BIG_ENDIAN: u32 = 1u32 << 31;

pub const DRM_FORMAT_INVALID: u64 = 0;
pub const DRM_FORMAT_C1: u64 = fourcc_code(67, 49, 32, 32);
pub const DRM_FORMAT_C2: u64 = fourcc_code(67, 50, 32, 32);
pub const DRM_FORMAT_C4: u64 = fourcc_code(67, 52, 32, 32);
pub const DRM_FORMAT_C8: u64 = fourcc_code(67, 56, 32, 32);
pub const DRM_FORMAT_D1: u64 = fourcc_code(68, 49, 32, 32);
pub const DRM_FORMAT_D2: u64 = fourcc_code(68, 50, 32, 32);
pub const DRM_FORMAT_D4: u64 = fourcc_code(68, 52, 32, 32);
pub const DRM_FORMAT_D8: u64 = fourcc_code(68, 56, 32, 32);
pub const DRM_FORMAT_R1: u64 = fourcc_code(82, 49, 32, 32);
pub const DRM_FORMAT_R2: u64 = fourcc_code(82, 50, 32, 32);
pub const DRM_FORMAT_R4: u64 = fourcc_code(82, 52, 32, 32);
pub const DRM_FORMAT_R8: u64 = fourcc_code(82, 56, 32, 32);
pub const DRM_FORMAT_R10: u64 = fourcc_code(82, 49, 48, 32);
pub const DRM_FORMAT_R12: u64 = fourcc_code(82, 49, 50, 32);
pub const DRM_FORMAT_R16: u64 = fourcc_code(82, 49, 54, 32);
pub const DRM_FORMAT_RG88: u64 = fourcc_code(82, 71, 56, 56);
pub const DRM_FORMAT_GR88: u64 = fourcc_code(71, 82, 56, 56);
pub const DRM_FORMAT_RG1616: u64 = fourcc_code(82, 71, 51, 50);
pub const DRM_FORMAT_GR1616: u64 = fourcc_code(71, 82, 51, 50);
pub const DRM_FORMAT_RGB332: u64 = fourcc_code(82, 71, 66, 56);
pub const DRM_FORMAT_BGR233: u64 = fourcc_code(66, 71, 82, 56);
pub const DRM_FORMAT_XRGB4444: u64 = fourcc_code(88, 82, 49, 50);
pub const DRM_FORMAT_XBGR4444: u64 = fourcc_code(88, 66, 49, 50);
pub const DRM_FORMAT_RGBX4444: u64 = fourcc_code(82, 88, 49, 50);
pub const DRM_FORMAT_BGRX4444: u64 = fourcc_code(66, 88, 49, 50);
pub const DRM_FORMAT_ARGB4444: u64 = fourcc_code(65, 82, 49, 50);
pub const DRM_FORMAT_ABGR4444: u64 = fourcc_code(65, 66, 49, 50);
pub const DRM_FORMAT_RGBA4444: u64 = fourcc_code(82, 65, 49, 50);
pub const DRM_FORMAT_BGRA4444: u64 = fourcc_code(66, 65, 49, 50);
pub const DRM_FORMAT_XRGB1555: u64 = fourcc_code(88, 82, 49, 53);
pub const DRM_FORMAT_XBGR1555: u64 = fourcc_code(88, 66, 49, 53);
pub const DRM_FORMAT_RGBX5551: u64 = fourcc_code(82, 88, 49, 53);
pub const DRM_FORMAT_BGRX5551: u64 = fourcc_code(66, 88, 49, 53);
pub const DRM_FORMAT_ARGB1555: u64 = fourcc_code(65, 82, 49, 53);
pub const DRM_FORMAT_ABGR1555: u64 = fourcc_code(65, 66, 49, 53);
pub const DRM_FORMAT_RGBA5551: u64 = fourcc_code(82, 65, 49, 53);
pub const DRM_FORMAT_BGRA5551: u64 = fourcc_code(66, 65, 49, 53);
pub const DRM_FORMAT_RGB565: u64 = fourcc_code(82, 71, 49, 54);
pub const DRM_FORMAT_BGR565: u64 = fourcc_code(66, 71, 49, 54);
pub const DRM_FORMAT_RGB888: u64 = fourcc_code(82, 71, 50, 52);
pub const DRM_FORMAT_BGR888: u64 = fourcc_code(66, 71, 50, 52);
pub const DRM_FORMAT_XRGB8888: u64 = fourcc_code(88, 82, 50, 52);
pub const DRM_FORMAT_XBGR8888: u64 = fourcc_code(88, 66, 50, 52);
pub const DRM_FORMAT_RGBX8888: u64 = fourcc_code(82, 88, 50, 52);
pub const DRM_FORMAT_BGRX8888: u64 = fourcc_code(66, 88, 50, 52);
pub const DRM_FORMAT_ARGB8888: u64 = fourcc_code(65, 82, 50, 52);
pub const DRM_FORMAT_ABGR8888: u64 = fourcc_code(65, 66, 50, 52);
pub const DRM_FORMAT_RGBA8888: u64 = fourcc_code(82, 65, 50, 52);
pub const DRM_FORMAT_BGRA8888: u64 = fourcc_code(66, 65, 50, 52);
pub const DRM_FORMAT_XRGB2101010: u64 = fourcc_code(88, 82, 51, 48);
pub const DRM_FORMAT_XBGR2101010: u64 = fourcc_code(88, 66, 51, 48);
pub const DRM_FORMAT_RGBX1010102: u64 = fourcc_code(82, 88, 51, 48);
pub const DRM_FORMAT_BGRX1010102: u64 = fourcc_code(66, 88, 51, 48);
pub const DRM_FORMAT_ARGB2101010: u64 = fourcc_code(65, 82, 51, 48);
pub const DRM_FORMAT_ABGR2101010: u64 = fourcc_code(65, 66, 51, 48);
pub const DRM_FORMAT_RGBA1010102: u64 = fourcc_code(82, 65, 51, 48);
pub const DRM_FORMAT_BGRA1010102: u64 = fourcc_code(66, 65, 51, 48);
pub const DRM_FORMAT_RGB161616: u64 = fourcc_code(82, 71, 52, 56);
pub const DRM_FORMAT_BGR161616: u64 = fourcc_code(66, 71, 52, 56);
pub const DRM_FORMAT_XRGB16161616: u64 = fourcc_code(88, 82, 52, 56);
pub const DRM_FORMAT_XBGR16161616: u64 = fourcc_code(88, 66, 52, 56);
pub const DRM_FORMAT_ARGB16161616: u64 = fourcc_code(65, 82, 52, 56);
pub const DRM_FORMAT_ABGR16161616: u64 = fourcc_code(65, 66, 52, 56);
pub const DRM_FORMAT_XRGB16161616F: u64 = fourcc_code(88, 82, 52, 72);
pub const DRM_FORMAT_XBGR16161616F: u64 = fourcc_code(88, 66, 52, 72);
pub const DRM_FORMAT_ARGB16161616F: u64 = fourcc_code(65, 82, 52, 72);
pub const DRM_FORMAT_ABGR16161616F: u64 = fourcc_code(65, 66, 52, 72);
pub const DRM_FORMAT_R16F: u64 = fourcc_code(82, 32, 32, 72);
pub const DRM_FORMAT_GR1616F: u64 = fourcc_code(71, 82, 32, 72);
pub const DRM_FORMAT_BGR161616F: u64 = fourcc_code(66, 71, 82, 72);
pub const DRM_FORMAT_R32F: u64 = fourcc_code(82, 32, 32, 70);
pub const DRM_FORMAT_GR3232F: u64 = fourcc_code(71, 82, 32, 70);
pub const DRM_FORMAT_BGR323232F: u64 = fourcc_code(66, 71, 82, 70);
pub const DRM_FORMAT_ABGR32323232F: u64 = fourcc_code(65, 66, 56, 70);
pub const DRM_FORMAT_AXBXGXRX106106106106: u64 = fourcc_code(65, 66, 49, 48);
pub const DRM_FORMAT_YUYV: u64 = fourcc_code(89, 85, 89, 86);
pub const DRM_FORMAT_YVYU: u64 = fourcc_code(89, 86, 89, 85);
pub const DRM_FORMAT_UYVY: u64 = fourcc_code(85, 89, 86, 89);
pub const DRM_FORMAT_VYUY: u64 = fourcc_code(86, 89, 85, 89);
pub const DRM_FORMAT_AYUV: u64 = fourcc_code(65, 89, 85, 86);
pub const DRM_FORMAT_AVUY8888: u64 = fourcc_code(65, 86, 85, 89);
pub const DRM_FORMAT_XYUV8888: u64 = fourcc_code(88, 89, 85, 86);
pub const DRM_FORMAT_XVUY8888: u64 = fourcc_code(88, 86, 85, 89);
pub const DRM_FORMAT_VUY888: u64 = fourcc_code(86, 85, 50, 52);
pub const DRM_FORMAT_VUY101010: u64 = fourcc_code(86, 85, 51, 48);
pub const DRM_FORMAT_XVUY2101010: u64 = fourcc_code(88, 89, 51, 48);
pub const DRM_FORMAT_Y210: u64 = fourcc_code(89, 50, 49, 48);
pub const DRM_FORMAT_Y212: u64 = fourcc_code(89, 50, 49, 50);
pub const DRM_FORMAT_Y216: u64 = fourcc_code(89, 50, 49, 54);
pub const DRM_FORMAT_Y410: u64 = fourcc_code(89, 52, 49, 48);
pub const DRM_FORMAT_Y412: u64 = fourcc_code(89, 52, 49, 50);
pub const DRM_FORMAT_Y416: u64 = fourcc_code(89, 52, 49, 54);
pub const DRM_FORMAT_XVYU2101010: u64 = fourcc_code(88, 86, 51, 48);
pub const DRM_FORMAT_XVYU12_16161616: u64 = fourcc_code(88, 86, 51, 54);
pub const DRM_FORMAT_XVYU16161616: u64 = fourcc_code(88, 86, 52, 56);
pub const DRM_FORMAT_Y0L0: u64 = fourcc_code(89, 48, 76, 48);
pub const DRM_FORMAT_X0L0: u64 = fourcc_code(88, 48, 76, 48);
pub const DRM_FORMAT_Y0L2: u64 = fourcc_code(89, 48, 76, 50);
pub const DRM_FORMAT_X0L2: u64 = fourcc_code(88, 48, 76, 50);
pub const DRM_FORMAT_YUV420_8BIT: u64 = fourcc_code(89, 85, 48, 56);
pub const DRM_FORMAT_YUV420_10BIT: u64 = fourcc_code(89, 85, 49, 48);
pub const DRM_FORMAT_XRGB8888_A8: u64 = fourcc_code(88, 82, 65, 56);
pub const DRM_FORMAT_XBGR8888_A8: u64 = fourcc_code(88, 66, 65, 56);
pub const DRM_FORMAT_RGBX8888_A8: u64 = fourcc_code(82, 88, 65, 56);
pub const DRM_FORMAT_BGRX8888_A8: u64 = fourcc_code(66, 88, 65, 56);
pub const DRM_FORMAT_RGB888_A8: u64 = fourcc_code(82, 56, 65, 56);
pub const DRM_FORMAT_BGR888_A8: u64 = fourcc_code(66, 56, 65, 56);
pub const DRM_FORMAT_RGB565_A8: u64 = fourcc_code(82, 53, 65, 56);
pub const DRM_FORMAT_BGR565_A8: u64 = fourcc_code(66, 53, 65, 56);
pub const DRM_FORMAT_NV12: u64 = fourcc_code(78, 86, 49, 50);
pub const DRM_FORMAT_NV21: u64 = fourcc_code(78, 86, 50, 49);
pub const DRM_FORMAT_NV16: u64 = fourcc_code(78, 86, 49, 54);
pub const DRM_FORMAT_NV61: u64 = fourcc_code(78, 86, 54, 49);
pub const DRM_FORMAT_NV24: u64 = fourcc_code(78, 86, 50, 52);
pub const DRM_FORMAT_NV42: u64 = fourcc_code(78, 86, 52, 50);
pub const DRM_FORMAT_NV15: u64 = fourcc_code(78, 86, 49, 53);
pub const DRM_FORMAT_NV20: u64 = fourcc_code(78, 86, 50, 48);
pub const DRM_FORMAT_NV30: u64 = fourcc_code(78, 86, 51, 48);
pub const DRM_FORMAT_P210: u64 = fourcc_code(80, 50, 49, 48);
pub const DRM_FORMAT_P010: u64 = fourcc_code(80, 48, 49, 48);
pub const DRM_FORMAT_P012: u64 = fourcc_code(80, 48, 49, 50);
pub const DRM_FORMAT_P016: u64 = fourcc_code(80, 48, 49, 54);
pub const DRM_FORMAT_P030: u64 = fourcc_code(80, 48, 51, 48);
pub const DRM_FORMAT_P230: u64 = fourcc_code(80, 50, 51, 48);
pub const DRM_FORMAT_Q410: u64 = fourcc_code(81, 52, 49, 48);
pub const DRM_FORMAT_Q401: u64 = fourcc_code(81, 52, 48, 49);
pub const DRM_FORMAT_T430: u64 = fourcc_code(84, 52, 51, 48);
pub const DRM_FORMAT_S010: u64 = fourcc_code(83, 48, 49, 48);
pub const DRM_FORMAT_S210: u64 = fourcc_code(83, 50, 49, 48);
pub const DRM_FORMAT_S410: u64 = fourcc_code(83, 52, 49, 48);
pub const DRM_FORMAT_S012: u64 = fourcc_code(83, 48, 49, 50);
pub const DRM_FORMAT_S212: u64 = fourcc_code(83, 50, 49, 50);
pub const DRM_FORMAT_S412: u64 = fourcc_code(83, 52, 49, 50);
pub const DRM_FORMAT_S016: u64 = fourcc_code(83, 48, 49, 54);
pub const DRM_FORMAT_S216: u64 = fourcc_code(83, 50, 49, 54);
pub const DRM_FORMAT_S416: u64 = fourcc_code(83, 52, 49, 54);
pub const DRM_FORMAT_YUV410: u64 = fourcc_code(89, 85, 86, 57);
pub const DRM_FORMAT_YVU410: u64 = fourcc_code(89, 86, 85, 57);
pub const DRM_FORMAT_YUV411: u64 = fourcc_code(89, 85, 49, 49);
pub const DRM_FORMAT_YVU411: u64 = fourcc_code(89, 86, 49, 49);
pub const DRM_FORMAT_YUV420: u64 = fourcc_code(89, 85, 49, 50);
pub const DRM_FORMAT_YVU420: u64 = fourcc_code(89, 86, 49, 50);
pub const DRM_FORMAT_YUV422: u64 = fourcc_code(89, 85, 49, 54);
pub const DRM_FORMAT_YVU422: u64 = fourcc_code(89, 86, 49, 54);
pub const DRM_FORMAT_YUV444: u64 = fourcc_code(89, 85, 50, 52);
pub const DRM_FORMAT_YVU444: u64 = fourcc_code(89, 86, 50, 52);
pub const DRM_FORMAT_Y8: u64 = fourcc_code(71, 82, 69, 89);
pub const DRM_FORMAT_XYYY2101010: u64 = fourcc_code(89, 80, 65, 52);
pub const DRM_FORMAT_MOD_VENDOR_NONE: u64 = 0;
pub const DRM_FORMAT_MOD_VENDOR_INTEL: u64 = 0x01;
pub const DRM_FORMAT_MOD_VENDOR_AMD: u64 = 0x02;
pub const DRM_FORMAT_MOD_VENDOR_NVIDIA: u64 = 0x03;
pub const DRM_FORMAT_MOD_VENDOR_SAMSUNG: u64 = 0x04;
pub const DRM_FORMAT_MOD_VENDOR_QCOM: u64 = 0x05;
pub const DRM_FORMAT_MOD_VENDOR_VIVANTE: u64 = 0x06;
pub const DRM_FORMAT_MOD_VENDOR_BROADCOM: u64 = 0x07;
pub const DRM_FORMAT_MOD_VENDOR_ARM: u64 = 0x08;
pub const DRM_FORMAT_MOD_VENDOR_ALLWINNER: u64 = 0x09;
pub const DRM_FORMAT_MOD_VENDOR_AMLOGIC: u64 = 0x0a;
pub const DRM_FORMAT_MOD_VENDOR_MTK: u64 = 0x0b;
pub const DRM_FORMAT_MOD_VENDOR_APPLE: u64 = 0x0c;
pub const DRM_FORMAT_RESERVED: u64 = ((1u64 << 56) - 1);
pub const DRM_FORMAT_MOD_GENERIC_16_16_TILE: u64 = DRM_FORMAT_MOD_SAMSUNG_16_16_TILE;
pub const DRM_FORMAT_MOD_INVALID: u64 = fourcc_mod_code(NONE, DRM_FORMAT_RESERVED);
pub const DRM_FORMAT_MOD_LINEAR: u64 = fourcc_mod_code(NONE, 0);
pub const DRM_FORMAT_MOD_NONE: u64 = 0;
pub const I915_FORMAT_MOD_X_TILED: u64 = fourcc_mod_code(INTEL, 1);
pub const I915_FORMAT_MOD_Y_TILED: u64 = fourcc_mod_code(INTEL, 2);
pub const I915_FORMAT_MOD_Yf_TILED: u64 = fourcc_mod_code(INTEL, 3);
pub const I915_FORMAT_MOD_Y_TILED_CCS: u64 = fourcc_mod_code(INTEL, 4);
pub const I915_FORMAT_MOD_Yf_TILED_CCS: u64 = fourcc_mod_code(INTEL, 5);
pub const I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS: u64 = fourcc_mod_code(INTEL, 6);
pub const I915_FORMAT_MOD_Y_TILED_GEN12_MC_CCS: u64 = fourcc_mod_code(INTEL, 7);
pub const I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC: u64 = fourcc_mod_code(INTEL, 8);
pub const I915_FORMAT_MOD_4_TILED: u64 = fourcc_mod_code(INTEL, 9);
pub const I915_FORMAT_MOD_4_TILED_DG2_RC_CCS: u64 = fourcc_mod_code(INTEL, 10);
pub const I915_FORMAT_MOD_4_TILED_DG2_MC_CCS: u64 = fourcc_mod_code(INTEL, 11);
pub const I915_FORMAT_MOD_4_TILED_DG2_RC_CCS_CC: u64 = fourcc_mod_code(INTEL, 12);
pub const I915_FORMAT_MOD_4_TILED_MTL_RC_CCS: u64 = fourcc_mod_code(INTEL, 13);
pub const I915_FORMAT_MOD_4_TILED_MTL_MC_CCS: u64 = fourcc_mod_code(INTEL, 14);
pub const I915_FORMAT_MOD_4_TILED_MTL_RC_CCS_CC: u64 = fourcc_mod_code(INTEL, 15);
pub const I915_FORMAT_MOD_4_TILED_LNL_CCS: u64 = fourcc_mod_code(INTEL, 16);
pub const I915_FORMAT_MOD_4_TILED_BMG_CCS: u64 = fourcc_mod_code(INTEL, 17);
pub const DRM_FORMAT_MOD_SAMSUNG_64_32_TILE: u64 = fourcc_mod_code(SAMSUNG, 1);
pub const DRM_FORMAT_MOD_SAMSUNG_16_16_TILE: u64 = fourcc_mod_code(SAMSUNG, 2);
pub const DRM_FORMAT_MOD_QCOM_COMPRESSED: u64 = fourcc_mod_code(QCOM, 1);
pub const DRM_FORMAT_MOD_QCOM_TILED3: u64 = fourcc_mod_code(QCOM, 3);
pub const DRM_FORMAT_MOD_QCOM_TILED2: u64 = fourcc_mod_code(QCOM, 2);
pub const DRM_FORMAT_MOD_VIVANTE_TILED: u64 = fourcc_mod_code(VIVANTE, 1);
pub const DRM_FORMAT_MOD_VIVANTE_SUPER_TILED: u64 = fourcc_mod_code(VIVANTE, 2);
pub const DRM_FORMAT_MOD_VIVANTE_SPLIT_TILED: u64 = fourcc_mod_code(VIVANTE, 3);
pub const DRM_FORMAT_MOD_VIVANTE_SPLIT_SUPER_TILED: u64 = fourcc_mod_code(VIVANTE, 4);
pub const VIVANTE_MOD_TS_64_4: u64 = (1u64 << 48);
pub const VIVANTE_MOD_TS_64_2: u64 = (2u64 << 48);
pub const VIVANTE_MOD_TS_128_4: u64 = (3u64 << 48);
pub const VIVANTE_MOD_TS_256_4: u64 = (4u64 << 48);
pub const VIVANTE_MOD_TS_MASK: u64 = (0xfu64 << 48);
pub const VIVANTE_MOD_COMP_DEC400: u64 = (1u64 << 52);
pub const VIVANTE_MOD_COMP_MASK: u64 = (0xfu64 << 52);
pub const DRM_FORMAT_MOD_NVIDIA_TEGRA_TILED: u64 = fourcc_mod_code(NVIDIA, 1);
pub const __fourcc_mod_broadcom_param_shift: u64 = 8;
pub const __fourcc_mod_broadcom_param_bits: u64 = 48;
pub const DRM_FORMAT_MOD_BROADCOM_VC4_T_TILED: u64 = fourcc_mod_code(BROADCOM, 1);
pub const DRM_FORMAT_MOD_BROADCOM_UIF: u64 = fourcc_mod_code(BROADCOM, 6);
pub const DRM_FORMAT_MOD_ARM_TYPE_AFBC: u64 = 0x00;
pub const DRM_FORMAT_MOD_ARM_TYPE_MISC: u64 = 0x01;
pub const AFBC_FORMAT_MOD_BLOCK_SIZE_MASK: u64 = 0xf;
pub const AFBC_FORMAT_MOD_BLOCK_SIZE_16x16: u64 = (1u64);
pub const AFBC_FORMAT_MOD_BLOCK_SIZE_32x8: u64 = (2u64);
pub const AFBC_FORMAT_MOD_BLOCK_SIZE_64x4: u64 = (3u64);
pub const AFBC_FORMAT_MOD_BLOCK_SIZE_32x8_64x4: u64 = (4u64);
pub const AFBC_FORMAT_MOD_YTR: u64 = (1u64 <<  4);
pub const AFBC_FORMAT_MOD_SPLIT: u64 = (1u64 <<  5);
pub const AFBC_FORMAT_MOD_SPARSE: u64 = (1u64 <<  6);
pub const AFBC_FORMAT_MOD_CBR: u64 = (1u64 <<  7);
pub const AFBC_FORMAT_MOD_TILED: u64 = (1u64 <<  8);
pub const AFBC_FORMAT_MOD_SC: u64 = (1u64 <<  9);
pub const AFBC_FORMAT_MOD_DB: u64 = (1u64 << 10);
pub const AFBC_FORMAT_MOD_BCH: u64 = (1u64 << 11);
pub const AFBC_FORMAT_MOD_USM: u64 = (1u64 << 12);
pub const DRM_FORMAT_MOD_ARM_TYPE_AFRC: u64 = 0x02;
pub const AFRC_FORMAT_MOD_CU_SIZE_MASK: u64 = 0xf;
pub const AFRC_FORMAT_MOD_CU_SIZE_16: u64 = (1u64);
pub const AFRC_FORMAT_MOD_CU_SIZE_24: u64 = (2u64);
pub const AFRC_FORMAT_MOD_CU_SIZE_32: u64 = (3u64);
pub const AFRC_FORMAT_MOD_LAYOUT_SCAN: u64 = (1u64 << 8);
pub const DRM_FORMAT_MOD_ALLWINNER_TILED: u64 = fourcc_mod_code(ALLWINNER, 1);
pub const __fourcc_mod_amlogic_layout_mask: u64 = 0xff;
pub const __fourcc_mod_amlogic_options_shift: u64 = 8;
pub const __fourcc_mod_amlogic_options_mask: u64 = 0xff;
pub const AMLOGIC_FBC_LAYOUT_BASIC: u64 = (1u64);
pub const AMLOGIC_FBC_LAYOUT_SCATTER: u64 = (2u64);
pub const AMLOGIC_FBC_OPTION_MEM_SAVING: u64 = (1u64 << 0);
pub const MTK_FMT_MOD_TILE_MASK: u64 = 0xf;
pub const MTK_FMT_MOD_TILE_NONE: u64 = 0x0;
pub const MTK_FMT_MOD_TILE_16L32S: u64 = 0x1;
pub const MTK_FMT_MOD_COMPRESS_MASK: u64 = (0xf << 8);
pub const MTK_FMT_MOD_COMPRESS_NONE: u64 = (0x0 << 8);
pub const MTK_FMT_MOD_COMPRESS_V1: u64 = (0x1 << 8);
pub const MTK_FMT_MOD_10BIT_LAYOUT_MASK: u64 = (0xf << 16);
pub const MTK_FMT_MOD_10BIT_LAYOUT_PACKED: u64 = (0x0 << 16);
pub const MTK_FMT_MOD_10BIT_LAYOUT_LSBTILED: u64 = (0x1 << 16);
pub const MTK_FMT_MOD_10BIT_LAYOUT_LSBRASTER: u64 = (0x2 << 16);
pub const DRM_FORMAT_MOD_MTK_16L_32S_TILE: u64 = DRM_FORMAT_MOD_MTK(MTK_FMT_MOD_TILE_16L32S);
pub const DRM_FORMAT_MOD_APPLE_GPU_TILED: u64 = fourcc_mod_code(APPLE, 1);
pub const DRM_FORMAT_MOD_APPLE_GPU_TILED_COMPRESSED: u64 = fourcc_mod_code(APPLE, 2);
pub const AMD_FMT_MOD: u64 = fourcc_mod_code(AMD, 0);
pub const AMD_FMT_MOD_TILE_VER_GFX6: u64 = 0;
pub const AMD_FMT_MOD_TILE_VER_GFX9: u64 = 1;
pub const AMD_FMT_MOD_TILE_VER_GFX10: u64 = 2;
pub const AMD_FMT_MOD_TILE_VER_GFX10_RBPLUS: u64 = 3;
pub const AMD_FMT_MOD_TILE_VER_GFX11: u64 = 4;
pub const AMD_FMT_MOD_TILE_VER_GFX12: u64 = 5;
pub const AMD_FMT_MOD_TILE_GFX6_1D_TILED_THIN1: u64 = 0x2;
pub const AMD_FMT_MOD_TILE_GFX6_2D_TILED_THIN1: u64 = 0x4;
pub const AMD_FMT_MOD_TILE_GFX9_64K_S: u64 = 9;
pub const AMD_FMT_MOD_TILE_GFX9_64K_D: u64 = 10;
pub const AMD_FMT_MOD_TILE_GFX9_4K_D_X: u64 = 22;
pub const AMD_FMT_MOD_TILE_GFX9_64K_S_X: u64 = 25;
pub const AMD_FMT_MOD_TILE_GFX9_64K_D_X: u64 = 26;
pub const AMD_FMT_MOD_TILE_GFX9_64K_R_X: u64 = 27;
pub const AMD_FMT_MOD_TILE_GFX11_256K_R_X: u64 = 31;
pub const AMD_FMT_MOD_TILE_GFX12_256B_2D: u64 = 1;
pub const AMD_FMT_MOD_TILE_GFX12_4K_2D: u64 = 2;
pub const AMD_FMT_MOD_TILE_GFX12_64K_2D: u64 = 3;
pub const AMD_FMT_MOD_TILE_GFX12_256K_2D: u64 = 4;
pub const AMD_FMT_MOD_DCC_BLOCK_64B: u64 = 0;
pub const AMD_FMT_MOD_DCC_BLOCK_128B: u64 = 1;
pub const AMD_FMT_MOD_DCC_BLOCK_256B: u64 = 2;
pub const AMD_FMT_MOD_TILE_VERSION_SHIFT: u64 = 0;
pub const AMD_FMT_MOD_TILE_VERSION_MASK: u64 = 0xFF;
pub const AMD_FMT_MOD_TILE_SHIFT: u64 = 8;
pub const AMD_FMT_MOD_TILE_MASK: u64 = 0x1F;
pub const AMD_FMT_MOD_DCC_SHIFT: u64 = 13;
pub const AMD_FMT_MOD_DCC_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_RETILE_SHIFT: u64 = 14;
pub const AMD_FMT_MOD_DCC_RETILE_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_PIPE_ALIGN_SHIFT: u64 = 15;
pub const AMD_FMT_MOD_DCC_PIPE_ALIGN_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_64B_SHIFT: u64 = 16;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_64B_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_128B_SHIFT: u64 = 17;
pub const AMD_FMT_MOD_DCC_INDEPENDENT_128B_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_SHIFT: u64 = 18;
pub const AMD_FMT_MOD_DCC_MAX_COMPRESSED_BLOCK_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_DCC_CONSTANT_ENCODE_SHIFT: u64 = 20;
pub const AMD_FMT_MOD_DCC_CONSTANT_ENCODE_MASK: u64 = 0x1;
pub const AMD_FMT_MOD_PIPE_XOR_BITS_SHIFT: u64 = 21;
pub const AMD_FMT_MOD_PIPE_XOR_BITS_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_BANK_XOR_BITS_SHIFT: u64 = 24;
pub const AMD_FMT_MOD_BANK_XOR_BITS_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_PACKERS_SHIFT: u64 = 27;
pub const AMD_FMT_MOD_PACKERS_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_RB_SHIFT: u64 = 30;
pub const AMD_FMT_MOD_RB_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_PIPE_SHIFT: u64 = 33;
pub const AMD_FMT_MOD_PIPE_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_MICROTILE_SHIFT: u64 = 14u64;
pub const AMD_FMT_MOD_MICROTILE_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_MICROTILE_DISPLAY: u64 = 0x0;
pub const AMD_FMT_MOD_MICROTILE_THIN: u64 = 0x1;
pub const AMD_FMT_MOD_PIPE_CONFIG_SHIFT: u64 = 17u64;
pub const AMD_FMT_MOD_PIPE_CONFIG_MASK: u64 = 0x1f;
pub const AMD_FMT_MOD_PIPE_CONFIG_P2: u64 = 0x0;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_8x16: u64 = 0x4;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_16x16: u64 = 0x5;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_16x32: u64 = 0x6;
pub const AMD_FMT_MOD_PIPE_CONFIG_P4_32x32: u64 = 0x7;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_16x16_8x16: u64 = 0x8;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_8x16: u64 = 0x9;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_8x16: u64 = 0xa;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_16x32_16x16: u64 = 0xb;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x16: u64 = 0xc;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x32_16x32: u64 = 0xd;
pub const AMD_FMT_MOD_PIPE_CONFIG_P8_32x64_32x32: u64 = 0xe;
pub const AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_8x16: u64 = 0x10;
pub const AMD_FMT_MOD_PIPE_CONFIG_P16_32x32_16x16: u64 = 0x11;
pub const AMD_FMT_MOD_TILE_SPLIT_SHIFT: u64 = 22u64;
pub const AMD_FMT_MOD_TILE_SPLIT_MASK: u64 = 0x7;
pub const AMD_FMT_MOD_BANK_WIDTH_SHIFT: u64 = 25u64;
pub const AMD_FMT_MOD_BANK_WIDTH_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_BANK_HEIGHT_SHIFT: u64 = 27u64;
pub const AMD_FMT_MOD_BANK_HEIGHT_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_MACRO_TILE_ASPECT_SHIFT: u64 = 29u64;
pub const AMD_FMT_MOD_MACRO_TILE_ASPECT_MASK: u64 = 0x3;
pub const AMD_FMT_MOD_NUM_BANKS_SHIFT: u64 = 31u64;
pub const AMD_FMT_MOD_NUM_BANKS_MASK: u64 = 0x3;

/* The original header documentation and macro bodies are preserved below as source-level reference. */
/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2011 Intel Corporation
 */
/**
 * DOC: overview
 *
 * In the DRM subsystem, framebuffer pixel formats are described using the
 * fourcc codes defined in `include/uapi/drm/drm_fourcc.h`. In addition to the
 * fourcc code, a Format Modifier may optionally be provided, in order to
 * further describe the buffer's format - for example tiling or compression.
 *
 * Format Modifiers
 * ----------------
 *
 * Format modifiers are used in conjunction with a fourcc code, forming a
 * unique fourcc:modifier pair. This format:modifier pair must fully define the
 * format and data layout of the buffer, and should be the only way to describe
 * that particular buffer.
 *
 * Having multiple fourcc:modifier pairs which describe the same layout should
 * be avoided, as such aliases run the risk of different drivers exposing
 * different names for the same data format, forcing userspace to understand
 * that they are aliases.
 *
 * Format modifiers may change any property of the buffer, including the number
 * of planes and/or the required allocation size. Format modifiers are
 * vendor-namespaced, and as such the relationship between a fourcc code and a
 * modifier is specific to the modifier being used. For example, some modifiers
 * may preserve meaning - such as number of planes - from the fourcc code,
 * whereas others may not.
 *
 * Modifiers must uniquely encode buffer layout. In other words, a buffer must
 * match only a single modifier. A modifier must not be a subset of layouts of
 * another modifier. For instance, it's incorrect to encode pitch alignment in
 * a modifier: a buffer may match a 64-pixel aligned modifier and a 32-pixel
 * aligned modifier. That said, modifiers can have implicit minimal
 * requirements.
 *
 * For modifiers where the combination of fourcc code and modifier can alias,
 * a canonical pair needs to be defined and used by all drivers. Preferred
 * combinations are also encouraged where all combinations might lead to
 * confusion and unnecessarily reduced interoperability. An example for the
 * latter is AFBC, where the ABGR layouts are preferred over ARGB layouts.
 *
 * There are two kinds of modifier users:
 *
 * - Kernel and user-space drivers: for drivers it's important that modifiers
 *   don't alias, otherwise two drivers might support the same format but use
 *   different aliases, preventing them from sharing buffers in an efficient
 *   format.
 * - Higher-level programs interfacing with KMS/GBM/EGL/Vulkan/etc: these users
 *   see modifiers as opaque tokens they can check for equality and intersect.
 *   These users mustn't need to know to reason about the modifier value
 *   (i.e. they are not expected to extract information out of the modifier).
 *
 * Vendors should document their modifier usage in as much detail as
 * possible, to ensure maximum compatibility across devices, drivers and
 * applications.
 *
 * The authoritative list of format modifier codes is found in
 * `include/uapi/drm/drm_fourcc.h`
 *
 * Open Source User Waiver
 * -----------------------
 *
 * Because this is the authoritative source for pixel formats and modifiers
 * referenced by GL, Vulkan extensions and other standards and hence used both
 * by open source and closed source driver stacks, the usual requirement for an
 * upstream in-kernel or open source userspace user does not apply.
 *
 * To ensure, as much as feasible, compatibility across stacks and avoid
 * confusion with incompatible enumerations stakeholders for all relevant driver
 * stacks should approve additions.
 */
/* Reserve 0 for the invalid format specifier */
/* color index */
/* 1 bpp Darkness (inverse relationship between channel value and brightness) */
/* 2 bpp Darkness (inverse relationship between channel value and brightness) */
/* 4 bpp Darkness (inverse relationship between channel value and brightness) */
/* 8 bpp Darkness (inverse relationship between channel value and brightness) */
/* 1 bpp Red (direct relationship between channel value and brightness) */
/* 2 bpp Red (direct relationship between channel value and brightness) */
/* 4 bpp Red (direct relationship between channel value and brightness) */
/* 8 bpp Red (direct relationship between channel value and brightness) */
/* 10 bpp Red (direct relationship between channel value and brightness) */
/* 12 bpp Red (direct relationship between channel value and brightness) */
/* 16 bpp Red (direct relationship between channel value and brightness) */
/* 16 bpp RG */
/* 32 bpp RG */
/* 8 bpp RGB */
/* 16 bpp RGB */
/* 24 bpp RGB */
/* 32 bpp RGB */
/* 48 bpp RGB */
/* 64 bpp RGB */
/*
 * Half-Floating point - 16b/component
 * IEEE 754-2008 binary16 half-precision float
 * [15:0] sign:exponent:mantissa 1:5:10
 */
/*
 * Floating point - 32b/component
 * IEEE 754-2008 binary32 float
 * [31:0] sign:exponent:mantissa 1:8:23
 */
/*
 * RGBA format with 10-bit components packed in 64-bit per pixel, with 6 bits
 * of unused padding per component:
 */
/* packed YCbCr */
/*
 * packed Y2xx indicate for each component, xx valid data occupy msb
 * 16-xx padding occupy lsb
 */
/*
 * packed Y4xx indicate for each component, xx valid data occupy msb
 * 16-xx padding occupy lsb except Y410
 */
/*
 * packed YCbCr420 2x2 tiled formats
 * first 64 bits will contain Y,Cb,Cr components for a 2x2 tile
 */
/* [63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian */
/* [63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian */
/* [63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian */
/* [63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian */
/*
 * 1-plane YUV 4:2:0
 * In these formats, the component ordering is specified (Y, followed by U
 * then V), but the exact Linear layout is undefined.
 * These formats can only be used with a non-Linear modifier.
 */
/*
 * 2 plane RGB + A
 * index 0 = RGB plane, same format as the corresponding non _A8 format has
 * index 1 = A plane, [7:0] A
 */
/*
 * 2 plane YCbCr
 * index 0 = Y plane, [7:0] Y
 * index 1 = Cr:Cb plane, [15:0] Cr:Cb little endian
 * or
 * index 1 = Cb:Cr plane, [15:0] Cb:Cr little endian
 */
/*
 * 2 plane YCbCr
 * index 0 = Y plane, [39:0] Y3:Y2:Y1:Y0 little endian
 * index 1 = Cr:Cb plane, [39:0] Cr1:Cb1:Cr0:Cb0 little endian
 */
/*
 * 2 plane YCbCr MSB aligned
 * index 0 = Y plane, [15:0] Y:x [10:6] little endian
 * index 1 = Cr:Cb plane, [31:0] Cr:x:Cb:x [10:6:10:6] little endian
 */
/*
 * 2 plane YCbCr MSB aligned
 * index 0 = Y plane, [15:0] Y:x [10:6] little endian
 * index 1 = Cr:Cb plane, [31:0] Cr:x:Cb:x [10:6:10:6] little endian
 */
/*
 * 2 plane YCbCr MSB aligned
 * index 0 = Y plane, [15:0] Y:x [12:4] little endian
 * index 1 = Cr:Cb plane, [31:0] Cr:x:Cb:x [12:4:12:4] little endian
 */
/*
 * 2 plane YCbCr MSB aligned
 * index 0 = Y plane, [15:0] Y little endian
 * index 1 = Cr:Cb plane, [31:0] Cr:Cb [16:16] little endian
 */
/* 2 plane YCbCr420.
 * 3 10 bit components and 2 padding bits packed into 4 bytes.
 * index 0 = Y plane, [31:0] x:Y2:Y1:Y0 2:10:10:10 little endian
 * index 1 = Cr:Cb plane, [63:0] x:Cr2:Cb2:Cr1:x:Cb1:Cr0:Cb0 [2:10:10:10:2:10:10:10] little endian
 */
/*
 * 2 plane YCbCr422.
 * 3 10 bit components and 2 padding bits packed into 4 bytes.
 * index 0 = Y plane, [31:0] x:Y2:Y1:Y0 2:10:10:10 little endian
 * index 1 = Cr:Cb plane, [63:0] x:Cr2:Cb2:Cr1:x:Cb1:Cr0:Cb0 [2:10:10:10:2:10:10:10] little endian
 */
/* 3 plane non-subsampled (444) YCbCr
 * 16 bits per component, but only 10 bits are used and 6 bits are padded
 * index 0: Y plane, [15:0] Y:x [10:6] little endian
 * index 1: Cb plane, [15:0] Cb:x [10:6] little endian
 * index 2: Cr plane, [15:0] Cr:x [10:6] little endian
 */
/* 3 plane non-subsampled (444) YCrCb
 * 16 bits per component, but only 10 bits are used and 6 bits are padded
 * index 0: Y plane, [15:0] Y:x [10:6] little endian
 * index 1: Cr plane, [15:0] Cr:x [10:6] little endian
 * index 2: Cb plane, [15:0] Cb:x [10:6] little endian
 */
/*
 * 3 plane non-subsampled (444) YCbCr LSB aligned
 * 10 bpc, 30 bits per sample image data in a single contiguous buffer.
 * index 0: Y plane,  [31:0] x:Y2:Y1:Y0    [2:10:10:10] little endian
 * index 1: Cb plane, [31:0] x:Cb2:Cb1:Cb0 [2:10:10:10] little endian
 * index 2: Cr plane, [31:0] x:Cr2:Cr1:Cr0 [2:10:10:10] little endian
 */
/*
 * 3 plane YCbCr LSB aligned
 * In order to use these formats in a similar fashion to MSB aligned ones
 * implementation can multiply the values by 2^6=64. For that reason the padding
 * must only contain zeros.
 * index 0 = Y plane, [15:0] z:Y [6:10] little endian
 * index 1 = Cb plane, [15:0] z:Cb [6:10] little endian
 * index 2 = Cr plane, [15:0] z:Cr [6:10] little endian
 */
/*
 * 3 plane YCbCr LSB aligned
 * In order to use these formats in a similar fashion to MSB aligned ones
 * implementation can multiply the values by 2^4=16. For that reason the padding
 * must only contain zeros.
 * index 0 = Y plane, [15:0] z:Y [4:12] little endian
 * index 1 = Cb plane, [15:0] z:Cb [4:12] little endian
 * index 2 = Cr plane, [15:0] z:Cr [4:12] little endian
 */
/*
 * 3 plane YCbCr
 * index 0 = Y plane, [15:0] Y little endian
 * index 1 = Cb plane, [15:0] Cb little endian
 * index 2 = Cr plane, [15:0] Cr little endian
 */
/*
 * 3 plane YCbCr
 * index 0: Y plane, [7:0] Y
 * index 1: Cb plane, [7:0] Cb
 * index 2: Cr plane, [7:0] Cr
 * or
 * index 1: Cr plane, [7:0] Cr
 * index 2: Cb plane, [7:0] Cb
 */
/*
 * Y-only (greyscale) formats
 *
 * The Y-only formats are handled similarly to the YCbCr formats in the display
 * pipeline, with the Cb and Cr implicitly neutral (0.0 in nominal values). This
 * also means that COLOR_RANGE property applies to the Y-only formats.
 */
/*
 * Format Modifiers:
 *
 * Format modifiers describe, typically, a re-ordering or modification
 * of the data in a plane of an FB.  This can be used to express tiled/
 * swizzled formats, or compression, or a combination of the two.
 *
 * The upper 8 bits of the format modifier are a vendor-id as assigned
 * below.  The lower 56 bits are assigned as vendor sees fit.
 */
/* Vendor Ids: */
/* add more to the end as needed */
/*
 * Format Modifier tokens:
 *
 * When adding a new token please document the layout with a code comment,
 * similar to the fourcc codes above. drm_fourcc.h is considered the
 * authoritative source for all of these.
 *
 * Generic modifier names:
 *
 * DRM_FORMAT_MOD_GENERIC_* definitions are used to provide vendor-neutral names
 * for layouts which are common across multiple vendors. To preserve
 * compatibility, in cases where a vendor-specific definition already exists and
 * a generic name for it is desired, the common name is a purely symbolic alias
 * and must use the same numerical value as the original definition.
 *
 * Note that generic names should only be used for modifiers which describe
 * generic layouts (such as pixel re-ordering), which may have
 * independently-developed support across multiple vendors.
 *
 * In future cases where a generic layout is identified before merging with a
 * vendor-specific modifier, a new 'GENERIC' vendor or modifier using vendor
 * 'NONE' could be considered. This should only be for obvious, exceptional
 * cases to avoid polluting the 'GENERIC' namespace with modifiers which only
 * apply to a single vendor.
 *
 * Generic names should not be used for cases where multiple hardware vendors
 * have implementations of the same standardised compression scheme (such as
 * AFBC). In those cases, all implementations should use the same format
 * modifier(s), reflecting the vendor of the standard.
 */
/*
 * Invalid Modifier
 *
 * This modifier can be used as a sentinel to terminate the format modifiers
 * list, or to initialize a variable with an invalid modifier. It might also be
 * used to report an error back to userspace for certain APIs.
 */
/*
 * Linear Layout
 *
 * Just plain linear layout. Note that this is different from no specifying any
 * modifier (e.g. not setting DRM_MODE_FB_MODIFIERS in the DRM_ADDFB2 ioctl),
 * which tells the driver to also take driver-internal information into account
 * and so might actually result in a tiled framebuffer.
 */
/*
 * Deprecated: use DRM_FORMAT_MOD_LINEAR instead
 *
 * The "none" format modifier doesn't actually mean that the modifier is
 * implicit, instead it means that the layout is linear. Whether modifiers are
 * used is out-of-band information carried in an API-specific way (e.g. in a
 * flag for drm_mode_fb_cmd2).
 */
/* Intel framebuffer modifiers */
/*
 * Intel X-tiling layout
 *
 * This is a tiled layout using 4Kb tiles (except on gen2 where the tiles 2Kb)
 * in row-major layout. Within the tile bytes are laid out row-major, with
 * a platform-dependent stride. On top of that the memory can apply
 * platform-depending swizzling of some higher address bits into bit6.
 *
 * Note that this layout is only accurate on intel gen 8+ or valleyview chipsets.
 * On earlier platforms the is highly platforms specific and not useful for
 * cross-driver sharing. It exists since on a given platform it does uniquely
 * identify the layout in a simple way for i915-specific userspace, which
 * facilitated conversion of userspace to modifiers. Additionally the exact
 * format on some really old platforms is not known.
 */
/*
 * Intel Y-tiling layout
 *
 * This is a tiled layout using 4Kb tiles (except on gen2 where the tiles 2Kb)
 * in row-major layout. Within the tile bytes are laid out in OWORD (16 bytes)
 * chunks column-major, with a platform-dependent height. On top of that the
 * memory can apply platform-depending swizzling of some higher address bits
 * into bit6.
 *
 * Note that this layout is only accurate on intel gen 8+ or valleyview chipsets.
 * On earlier platforms the is highly platforms specific and not useful for
 * cross-driver sharing. It exists since on a given platform it does uniquely
 * identify the layout in a simple way for i915-specific userspace, which
 * facilitated conversion of userspace to modifiers. Additionally the exact
 * format on some really old platforms is not known.
 */
/*
 * Intel Yf-tiling layout
 *
 * This is a tiled layout using 4Kb tiles in row-major layout.
 * Within the tile pixels are laid out in 16 256 byte units / sub-tiles which
 * are arranged in four groups (two wide, two high) with column-major layout.
 * Each group therefore consists out of four 256 byte units, which are also laid
 * out as 2x2 column-major.
 * 256 byte units are made out of four 64 byte blocks of pixels, producing
 * either a square block or a 2:1 unit.
 * 64 byte blocks of pixels contain four pixel rows of 16 bytes, where the width
 * in pixel depends on the pixel depth.
 */
/*
 * Intel color control surface (CCS) for render compression
 *
 * The framebuffer format must be one of the 8:8:8:8 RGB formats.
 * The main surface will be plane index 0 and must be Y/Yf-tiled,
 * the CCS will be plane index 1.
 *
 * Each CCS tile matches a 1024x512 pixel area of the main surface.
 * To match certain aspects of the 3D hardware the CCS is
 * considered to be made up of normal 128Bx32 Y tiles, Thus
 * the CCS pitch must be specified in multiples of 128 bytes.
 *
 * In reality the CCS tile appears to be a 64Bx64 Y tile, composed
 * of QWORD (8 bytes) chunks instead of OWORD (16 bytes) chunks.
 * But that fact is not relevant unless the memory is accessed
 * directly.
 */
/*
 * Intel color control surfaces (CCS) for Gen-12 render compression.
 *
 * The main surface is Y-tiled and at plane index 0, the CCS is linear and
 * at index 1. A 64B CCS cache line corresponds to an area of 4x1 tiles in
 * main surface. In other words, 4 bits in CCS map to a main surface cache
 * line pair. The main surface pitch is required to be a multiple of four
 * Y-tile widths.
 */
/*
 * Intel color control surfaces (CCS) for Gen-12 media compression
 *
 * The main surface is Y-tiled and at plane index 0, the CCS is linear and
 * at index 1. A 64B CCS cache line corresponds to an area of 4x1 tiles in
 * main surface. In other words, 4 bits in CCS map to a main surface cache
 * line pair. The main surface pitch is required to be a multiple of four
 * Y-tile widths. For semi-planar formats like NV12, CCS planes follow the
 * Y and UV planes i.e., planes 0 and 1 are used for Y and UV surfaces,
 * planes 2 and 3 for the respective CCS.
 */
/*
 * Intel Color Control Surface with Clear Color (CCS) for Gen-12 render
 * compression.
 *
 * The main surface is Y-tiled and is at plane index 0 whereas CCS is linear
 * and at index 1. The clear color is stored at index 2, and the pitch should
 * be 64 bytes aligned. The clear color structure is 256 bits. The first 128 bits
 * represents Raw Clear Color Red, Green, Blue and Alpha color each represented
 * by 32 bits. The raw clear color is consumed by the 3d engine and generates
 * the converted clear color of size 64 bits. The first 32 bits store the Lower
 * Converted Clear Color value and the next 32 bits store the Higher Converted
 * Clear Color value when applicable. The Converted Clear Color values are
 * consumed by the DE. The last 64 bits are used to store Color Discard Enable
 * and Depth Clear Value Valid which are ignored by the DE. A CCS cache line
 * corresponds to an area of 4x1 tiles in the main surface. The main surface
 * pitch is required to be a multiple of 4 tile widths.
 */
/*
 * Intel Tile 4 layout
 *
 * This is a tiled layout using 4KB tiles in a row-major layout. It has the same
 * shape as Tile Y at two granularities: 4KB (128B x 32) and 64B (16B x 4). It
 * only differs from Tile Y at the 256B granularity in between. At this
 * granularity, Tile Y has a shape of 16B x 32 rows, but this tiling has a shape
 * of 64B x 8 rows.
 */
/*
 * Intel color control surfaces (CCS) for DG2 render compression.
 *
 * The main surface is Tile 4 and at plane index 0. The CCS data is stored
 * outside of the GEM object in a reserved memory area dedicated for the
 * storage of the CCS data for all RC/RC_CC/MC compressible GEM objects. The
 * main surface pitch is required to be a multiple of four Tile 4 widths.
 */
/*
 * Intel color control surfaces (CCS) for DG2 media compression.
 *
 * The main surface is Tile 4 and at plane index 0. For semi-planar formats
 * like NV12, the Y and UV planes are Tile 4 and are located at plane indices
 * 0 and 1, respectively. The CCS for all planes are stored outside of the
 * GEM object in a reserved memory area dedicated for the storage of the
 * CCS data for all RC/RC_CC/MC compressible GEM objects. The main surface
 * pitch is required to be a multiple of four Tile 4 widths.
 */
/*
 * Intel Color Control Surface with Clear Color (CCS) for DG2 render compression.
 *
 * The main surface is Tile 4 and at plane index 0. The CCS data is stored
 * outside of the GEM object in a reserved memory area dedicated for the
 * storage of the CCS data for all RC/RC_CC/MC compressible GEM objects. The
 * main surface pitch is required to be a multiple of four Tile 4 widths. The
 * clear color is stored at plane index 1 and the pitch should be 64 bytes
 * aligned. The format of the 256 bits of clear color data matches the one used
 * for the I915_FORMAT_MOD_Y_TILED_GEN12_RC_CCS_CC modifier, see its description
 * for details.
 */
/*
 * Intel Color Control Surfaces (CCS) for display ver. 14 render compression.
 *
 * The main surface is tile4 and at plane index 0, the CCS is linear and
 * at index 1. A 64B CCS cache line corresponds to an area of 4x1 tiles in
 * main surface. In other words, 4 bits in CCS map to a main surface cache
 * line pair. The main surface pitch is required to be a multiple of four
 * tile4 widths.
 */
/*
 * Intel Color Control Surfaces (CCS) for display ver. 14 media compression
 *
 * The main surface is tile4 and at plane index 0, the CCS is linear and
 * at index 1. A 64B CCS cache line corresponds to an area of 4x1 tiles in
 * main surface. In other words, 4 bits in CCS map to a main surface cache
 * line pair. The main surface pitch is required to be a multiple of four
 * tile4 widths. For semi-planar formats like NV12, CCS planes follow the
 * Y and UV planes i.e., planes 0 and 1 are used for Y and UV surfaces,
 * planes 2 and 3 for the respective CCS.
 */
/*
 * Intel Color Control Surface with Clear Color (CCS) for display ver. 14 render
 * compression.
 *
 * The main surface is tile4 and is at plane index 0 whereas CCS is linear
 * and at index 1. The clear color is stored at index 2, and the pitch should
 * be ignored. The clear color structure is 256 bits. The first 128 bits
 * represents Raw Clear Color Red, Green, Blue and Alpha color each represented
 * by 32 bits. The raw clear color is consumed by the 3d engine and generates
 * the converted clear color of size 64 bits. The first 32 bits store the Lower
 * Converted Clear Color value and the next 32 bits store the Higher Converted
 * Clear Color value when applicable. The Converted Clear Color values are
 * consumed by the DE. The last 64 bits are used to store Color Discard Enable
 * and Depth Clear Value Valid which are ignored by the DE. A CCS cache line
 * corresponds to an area of 4x1 tiles in the main surface. The main surface
 * pitch is required to be a multiple of 4 tile widths.
 */
/*
 * Intel Color Control Surfaces (CCS) for graphics ver. 20 unified compression
 * on integrated graphics
 *
 * The main surface is Tile 4 and at plane index 0. For semi-planar formats
 * like NV12, the Y and UV planes are Tile 4 and are located at plane indices
 * 0 and 1, respectively. The CCS for all planes are stored outside of the
 * GEM object in a reserved memory area dedicated for the storage of the
 * CCS data for all compressible GEM objects.
 */
/*
 * Intel Color Control Surfaces (CCS) for graphics ver. 20 unified compression
 * on discrete graphics
 *
 * The main surface is Tile 4 and at plane index 0. For semi-planar formats
 * like NV12, the Y and UV planes are Tile 4 and are located at plane indices
 * 0 and 1, respectively. The CCS for all planes are stored outside of the
 * GEM object in a reserved memory area dedicated for the storage of the
 * CCS data for all compressible GEM objects. The GEM object must be stored in
 * contiguous memory with a size aligned to 64KB
 */
/*
 * Tiled, NV12MT, grouped in 64 (pixels) x 32 (lines) -sized macroblocks
 *
 * Macroblocks are laid in a Z-shape, and each pixel data is following the
 * standard NV12 style.
 * As for NV12, an image is the result of two frame buffers: one for Y,
 * one for the interleaved Cb/Cr components (1/2 the height of the Y buffer).
 * Alignment requirements are (for each buffer):
 * - multiple of 128 pixels for the width
 * - multiple of  32 pixels for the height
 *
 * For more information: see https://linuxtv.org/downloads/v4l-dvb-apis/re32.html
 */
/*
 * Tiled, 16 (pixels) x 16 (lines) - sized macroblocks
 *
 * This is a simple tiled layout using tiles of 16x16 pixels in a row-major
 * layout. For YCbCr formats Cb/Cr components are taken in such a way that
 * they correspond to their 16x16 luma block.
 */
/*
 * Qualcomm Compressed Format
 *
 * Refers to a compressed variant of the base format that is compressed.
 * Implementation may be platform and base-format specific.
 *
 * Each macrotile consists of m x n (mostly 4 x 4) tiles.
 * Pixel data pitch/stride is aligned with macrotile width.
 * Pixel data height is aligned with macrotile height.
 * Entire pixel data buffer is aligned with 4k(bytes).
 */
/*
 * Qualcomm Tiled Format
 *
 * Similar to DRM_FORMAT_MOD_QCOM_COMPRESSED but not compressed.
 * Implementation may be platform and base-format specific.
 *
 * Each macrotile consists of m x n (mostly 4 x 4) tiles.
 * Pixel data pitch/stride is aligned with macrotile width.
 * Pixel data height is aligned with macrotile height.
 * Entire pixel data buffer is aligned with 4k(bytes).
 */
/*
 * Qualcomm Alternate Tiled Format
 *
 * Alternate tiled format typically only used within GMEM.
 * Implementation may be platform and base-format specific.
 */
/* Vivante framebuffer modifiers */
/*
 * Vivante 4x4 tiling layout
 *
 * This is a simple tiled layout using tiles of 4x4 pixels in a row-major
 * layout.
 */
/*
 * Vivante 64x64 super-tiling layout
 *
 * This is a tiled layout using 64x64 pixel super-tiles, where each super-tile
 * contains 8x4 groups of 2x4 tiles of 4x4 pixels (like above) each, all in row-
 * major layout.
 *
 * For more information: see
 * https://github.com/etnaviv/etna_viv/blob/master/doc/hardware.md#texture-tiling
 */
/*
 * Vivante 4x4 tiling layout for dual-pipe
 *
 * Same as the 4x4 tiling layout, except every second 4x4 pixel tile starts at a
 * different base address. Offsets from the base addresses are therefore halved
 * compared to the non-split tiled layout.
 */
/*
 * Vivante 64x64 super-tiling layout for dual-pipe
 *
 * Same as the 64x64 super-tiling layout, except every second 4x4 pixel tile
 * starts at a different base address. Offsets from the base addresses are
 * therefore halved compared to the non-split super-tiled layout.
 */
/*
 * Vivante TS (tile-status) buffer modifiers. They can be combined with all of
 * the color buffer tiling modifiers defined above. When TS is present it's a
 * separate buffer containing the clear/compression status of each tile. The
 * modifiers are defined as VIVANTE_MOD_TS_c_s, where c is the color buffer
 * tile size in bytes covered by one entry in the status buffer and s is the
 * number of status bits per entry.
 * We reserve the top 8 bits of the Vivante modifier space for tile status
 * clear/compression modifiers, as future cores might add some more TS layout
 * variations.
 */
/*
 * Vivante compression modifiers. Those depend on a TS modifier being present
 * as the TS bits get reinterpreted as compression tags instead of simple
 * clear markers when compression is enabled.
 */
/* Masking out the extension bits will yield the base modifier. */
/* NVIDIA frame buffer modifiers */
/*
 * Tegra Tiled Layout, used by Tegra 2, 3 and 4.
 *
 * Pixels are arranged in simple tiles of 16 x 16 bytes.
 */
/*
 * Generalized Block Linear layout, used by desktop GPUs starting with NV50/G80,
 * and Tegra GPUs starting with Tegra K1.
 *
 * Pixels are arranged in Groups of Bytes (GOBs).  GOB size and layout varies
 * based on the architecture generation.  GOBs themselves are then arranged in
 * 3D blocks, with the block dimensions (in terms of GOBs) always being a power
 * of two, and hence expressible as their log2 equivalent (E.g., "2" represents
 * a block depth or height of "4").
 *
 * Chapter 20 "Pixel Memory Formats" of the Tegra X1 TRM describes this format
 * in full detail.
 *
 *       Macro
 * Bits  Param Description
 * ----  ----- -----------------------------------------------------------------
 *
 *  3:0  h     log2(height) of each block, in GOBs.  Placed here for
 *             compatibility with the existing
 *             DRM_FORMAT_MOD_NVIDIA_16BX2_BLOCK()-based modifiers.
 *
 *  4:4  -     Must be 1, to indicate block-linear layout.  Necessary for
 *             compatibility with the existing
 *             DRM_FORMAT_MOD_NVIDIA_16BX2_BLOCK()-based modifiers.
 *
 *  8:5  -     Reserved (To support 3D-surfaces with variable log2(depth) block
 *             size).  Must be zero.
 *
 *             Note there is no log2(width) parameter.  Some portions of the
 *             hardware support a block width of two gobs, but it is impractical
 *             to use due to lack of support elsewhere, and has no known
 *             benefits.
 *
 * 11:9  -     Reserved (To support 2D-array textures with variable array stride
 *             in blocks, specified via log2(tile width in blocks)).  Must be
 *             zero.
 *
 * 19:12 k     Page Kind.  This value directly maps to a field in the page
 *             tables of all GPUs >= NV50.  It affects the exact layout of bits
 *             in memory and can be derived from the tuple
 *
 *               (format, GPU model, compression type, samples per pixel)
 *
 *             Where compression type is defined below.  If GPU model were
 *             implied by the format modifier, format, or memory buffer, page
 *             kind would not need to be included in the modifier itself, but
 *             since the modifier should define the layout of the associated
 *             memory buffer independent from any device or other context, it
 *             must be included here.
 *
 * 21:20 g     GOB Height and Page Kind Generation.  The height of a GOB changed
 *             starting with Fermi GPUs.  Additionally, the mapping between page
 *             kind and bit layout has changed at various points.
 *
 *               0 = Gob Height 8, Fermi - Volta, Tegra K1+ Page Kind mapping
 *               1 = Gob Height 4, G80 - GT2XX Page Kind mapping
 *               2 = Gob Height 8, Turing+ Page Kind mapping
 *               3 = Reserved for future use.
 *
 * 22:22 s     Sector layout.  There is a further bit remapping step that occurs
 * 26:27       at an even lower level than the page kind and block linear
 *             swizzles.  This causes the bit arrangement of surfaces in memory
 *             to differ subtly, and prevents direct sharing of surfaces between
 *             GPUs with different layouts.
 *
 *               0 = Tegra K1 - Tegra Parker/TX2 Layout
 *               1 = Pre-GB20x, GB20x 32+ bpp, GB10, Tegra Xavier-Orin Layout
 *               2 = GB20x(Blackwell 2)+ 8 bpp surface layout
 *               3 = GB20x(Blackwell 2)+ 16 bpp surface layout
 *               4 = Reserved for future use.
 *               5 = Reserved for future use.
 *               6 = Reserved for future use.
 *               7 = Reserved for future use.
 *
 * 25:23 c     Lossless Framebuffer Compression type.
 *
 *               0 = none
 *               1 = ROP/3D, layout 1, exact compression format implied by Page
 *                   Kind field
 *               2 = ROP/3D, layout 2, exact compression format implied by Page
 *                   Kind field
 *               3 = CDE horizontal
 *               4 = CDE vertical
 *               5 = Reserved for future use
 *               6 = Reserved for future use
 *               7 = Reserved for future use
 *
 * 55:28 -     Reserved for future use.  Must be zero.
 */
/* To grandfather in prior block linear format modifiers to the above layout,
 * the page kind "0", which corresponds to "pitch/linear" and hence is unusable
 * with block-linear layouts, is remapped within drivers to the value 0xfe,
 * which corresponds to the "generic" kind used for simple single-sample
 * uncompressed color formats on Fermi - Volta GPUs.
 */
/*
 * 16Bx2 Block Linear layout, used by Tegra K1 and later
 *
 * Pixels are arranged in 64x8 Groups Of Bytes (GOBs). GOBs are then stacked
 * vertically by a power of 2 (1 to 32 GOBs) to form a block.
 *
 * Within a GOB, data is ordered as 16B x 2 lines sectors laid in Z-shape.
 *
 * Parameter 'v' is the log2 encoding of the number of GOBs stacked vertically.
 * Valid values are:
 *
 * 0 == ONE_GOB
 * 1 == TWO_GOBS
 * 2 == FOUR_GOBS
 * 3 == EIGHT_GOBS
 * 4 == SIXTEEN_GOBS
 * 5 == THIRTYTWO_GOBS
 *
 * Chapter 20 "Pixel Memory Formats" of the Tegra X1 TRM describes this format
 * in full detail.
 */
/*
 * Some Broadcom modifiers take parameters, for example the number of
 * vertical lines in the image. Reserve the lower 32 bits for modifier
 * type, and the next 24 bits for parameters. Top 8 bits are the
 * vendor code.
 */
/*
 * Broadcom VC4 "T" format
 *
 * This is the primary layout that the V3D GPU can texture from (it
 * can't do linear).  The T format has:
 *
 * - 64b utiles of pixels in a raster-order grid according to cpp.  It's 4x4
 *   pixels at 32 bit depth.
 *
 * - 1k subtiles made of a 4x4 raster-order grid of 64b utiles (so usually
 *   16x16 pixels).
 *
 * - 4k tiles made of a 2x2 grid of 1k subtiles (so usually 32x32 pixels).  On
 *   even 4k tile rows, they're arranged as (BL, TL, TR, BR), and on odd rows
 *   they're (TR, BR, BL, TL), where bottom left is start of memory.
 *
 * - an image made of 4k tiles in rows either left-to-right (even rows of 4k
 *   tiles) or right-to-left (odd rows of 4k tiles).
 */
/*
 * Broadcom SAND format
 *
 * This is the native format that the H.264 codec block uses.  For VC4
 * HVS, it is only valid for H.264 (NV12/21) and RGBA modes.
 *
 * The image can be considered to be split into columns, and the
 * columns are placed consecutively into memory.  The width of those
 * columns can be either 32, 64, 128, or 256 pixels, but in practice
 * only 128 pixel columns are used.
 *
 * The pitch between the start of each column is set to optimally
 * switch between SDRAM banks. This is passed as the number of lines
 * of column width in the modifier (we can't use the stride value due
 * to various core checks that look at it , so you should set the
 * stride to width*cpp).
 *
 * Note that the column height for this format modifier is the same
 * for all of the planes, assuming that each column contains both Y
 * and UV.  Some SAND-using hardware stores UV in a separate tiled
 * image from Y to reduce the column height, which is not supported
 * with these modifiers.
 *
 * The DRM_FORMAT_MOD_BROADCOM_SAND128_COL_HEIGHT modifier is also
 * supported for DRM_FORMAT_P030 where the columns remain as 128 bytes
 * wide, but as this is a 10 bpp format that translates to 96 pixels.
 */
/* Broadcom UIF format
 *
 * This is the common format for the current Broadcom multimedia
 * blocks, including V3D 3.x and newer, newer video codecs, and
 * displays.
 *
 * The image consists of utiles (64b blocks), UIF blocks (2x2 utiles),
 * and macroblocks (4x4 UIF blocks).  Those 4x4 UIF block groups are
 * stored in columns, with padding between the columns to ensure that
 * moving from one column to the next doesn't hit the same SDRAM page
 * bank.
 *
 * To calculate the padding, it is assumed that each hardware block
 * and the software driving it knows the platform's SDRAM page size,
 * number of banks, and XOR address, and that it's identical between
 * all blocks using the format.  This tiling modifier will use XOR as
 * necessary to reduce the padding.  If a hardware block can't do XOR,
 * the assumption is that a no-XOR tiling modifier will be created.
 */
/*
 * Arm Framebuffer Compression (AFBC) modifiers
 *
 * AFBC is a proprietary lossless image compression protocol and format.
 * It provides fine-grained random access and minimizes the amount of data
 * transferred between IP blocks.
 *
 * AFBC has several features which may be supported and/or used, which are
 * represented using bits in the modifier. Not all combinations are valid,
 * and different devices or use-cases may support different combinations.
 *
 * Further information on the use of AFBC modifiers can be found in
 * Documentation/gpu/afbc.rst
 */
/*
 * The top 4 bits (out of the 56 bits allotted for specifying vendor specific
 * modifiers) denote the category for modifiers. Currently we have three
 * categories of modifiers ie AFBC, MISC and AFRC. We can have a maximum of
 * sixteen different categories.
 */
/*
 * AFBC superblock size
 *
 * Indicates the superblock size(s) used for the AFBC buffer. The buffer
 * size (in pixels) must be aligned to a multiple of the superblock size.
 * Four lowest significant bits(LSBs) are reserved for block size.
 *
 * Where one superblock size is specified, it applies to all planes of the
 * buffer (e.g. 16x16, 32x8). When multiple superblock sizes are specified,
 * the first applies to the Luma plane and the second applies to the Chroma
 * plane(s). e.g. (32x8_64x4 means 32x8 Luma, with 64x4 Chroma).
 * Multiple superblock sizes are only valid for multi-plane YCbCr formats.
 */
/*
 * AFBC lossless colorspace transform
 *
 * Indicates that the buffer makes use of the AFBC lossless colorspace
 * transform.
 */
/*
 * AFBC block-split
 *
 * Indicates that the payload of each superblock is split. The second
 * half of the payload is positioned at a predefined offset from the start
 * of the superblock payload.
 */
/*
 * AFBC sparse layout
 *
 * This flag indicates that the payload of each superblock must be stored at a
 * predefined position relative to the other superblocks in the same AFBC
 * buffer. This order is the same order used by the header buffer. In this mode
 * each superblock is given the same amount of space as an uncompressed
 * superblock of the particular format would require, rounding up to the next
 * multiple of 128 bytes in size.
 */
/*
 * AFBC copy-block restrict
 *
 * Buffers with this flag must obey the copy-block restriction. The restriction
 * is such that there are no copy-blocks referring across the border of 8x8
 * blocks. For the subsampled data the 8x8 limitation is also subsampled.
 */
/*
 * AFBC tiled layout
 *
 * The tiled layout groups superblocks in 8x8 or 4x4 tiles, where all
 * superblocks inside a tile are stored together in memory. 8x8 tiles are used
 * for pixel formats up to and including 32 bpp while 4x4 tiles are used for
 * larger bpp formats. The order between the tiles is scan line.
 * When the tiled layout is used, the buffer size (in pixels) must be aligned
 * to the tile size.
 */
/*
 * AFBC solid color blocks
 *
 * Indicates that the buffer makes use of solid-color blocks, whereby bandwidth
 * can be reduced if a whole superblock is a single color.
 */
/*
 * AFBC double-buffer
 *
 * Indicates that the buffer is allocated in a layout safe for front-buffer
 * rendering.
 */
/*
 * AFBC buffer content hints
 *
 * Indicates that the buffer includes per-superblock content hints.
 */
/* AFBC uncompressed storage mode
 *
 * Indicates that the buffer is using AFBC uncompressed storage mode.
 * In this mode all superblock payloads in the buffer use the uncompressed
 * storage mode, which is usually only used for data which cannot be compressed.
 * The buffer layout is the same as for AFBC buffers without USM set, this only
 * affects the storage mode of the individual superblocks. Note that even a
 * buffer without USM set may use uncompressed storage mode for some or all
 * superblocks, USM just guarantees it for all.
 */
/*
 * Arm Fixed-Rate Compression (AFRC) modifiers
 *
 * AFRC is a proprietary fixed rate image compression protocol and format,
 * designed to provide guaranteed bandwidth and memory footprint
 * reductions in graphics and media use-cases.
 *
 * AFRC buffers consist of one or more planes, with the same components
 * and meaning as an uncompressed buffer using the same pixel format.
 *
 * Within each plane, the pixel/luma/chroma values are grouped into
 * "coding unit" blocks which are individually compressed to a
 * fixed size (in bytes). All coding units within a given plane of a buffer
 * store the same number of values, and have the same compressed size.
 *
 * The coding unit size is configurable, allowing different rates of compression.
 *
 * The start of each AFRC buffer plane must be aligned to an alignment granule which
 * depends on the coding unit size.
 *
 * Coding Unit Size   Plane Alignment
 * ----------------   ---------------
 * 16 bytes           1024 bytes
 * 24 bytes           512  bytes
 * 32 bytes           2048 bytes
 *
 * Coding units are grouped into paging tiles. AFRC buffer dimensions must be aligned
 * to a multiple of the paging tile dimensions.
 * The dimensions of each paging tile depend on whether the buffer is optimised for
 * scanline (SCAN layout) or rotated (ROT layout) access.
 *
 * Layout   Paging Tile Width   Paging Tile Height
 * ------   -----------------   ------------------
 * SCAN     16 coding units     4 coding units
 * ROT      8  coding units     8 coding units
 *
 * The dimensions of each coding unit depend on the number of components
 * in the compressed plane and whether the buffer is optimised for
 * scanline (SCAN layout) or rotated (ROT layout) access.
 *
 * Number of Components in Plane   Layout      Coding Unit Width   Coding Unit Height
 * -----------------------------   ---------   -----------------   ------------------
 * 1                               SCAN        16 samples          4 samples
 * Example: 16x4 luma samples in a 'Y' plane
 *          16x4 chroma 'V' values, in the 'V' plane of a fully-planar YUV buffer
 * -----------------------------   ---------   -----------------   ------------------
 * 1                               ROT         8 samples           8 samples
 * Example: 8x8 luma samples in a 'Y' plane
 *          8x8 chroma 'V' values, in the 'V' plane of a fully-planar YUV buffer
 * -----------------------------   ---------   -----------------   ------------------
 * 2                               DONT CARE   8 samples           4 samples
 * Example: 8x4 chroma pairs in the 'UV' plane of a semi-planar YUV buffer
 * -----------------------------   ---------   -----------------   ------------------
 * 3                               DONT CARE   4 samples           4 samples
 * Example: 4x4 pixels in an RGB buffer without alpha
 * -----------------------------   ---------   -----------------   ------------------
 * 4                               DONT CARE   4 samples           4 samples
 * Example: 4x4 pixels in an RGB buffer with alpha
 */
/*
 * AFRC coding unit size modifier.
 *
 * Indicates the number of bytes used to store each compressed coding unit for
 * one or more planes in an AFRC encoded buffer. The coding unit size for chrominance
 * is the same for both Cb and Cr, which may be stored in separate planes.
 *
 * AFRC_FORMAT_MOD_CU_SIZE_P0 indicates the number of bytes used to store
 * each compressed coding unit in the first plane of the buffer. For RGBA buffers
 * this is the only plane, while for semi-planar and fully-planar YUV buffers,
 * this corresponds to the luma plane.
 *
 * AFRC_FORMAT_MOD_CU_SIZE_P12 indicates the number of bytes used to store
 * each compressed coding unit in the second and third planes in the buffer.
 * For semi-planar and fully-planar YUV buffers, this corresponds to the chroma plane(s).
 *
 * For single-plane buffers, AFRC_FORMAT_MOD_CU_SIZE_P0 must be specified
 * and AFRC_FORMAT_MOD_CU_SIZE_P12 must be zero.
 * For semi-planar and fully-planar buffers, both AFRC_FORMAT_MOD_CU_SIZE_P0 and
 * AFRC_FORMAT_MOD_CU_SIZE_P12 must be specified.
 */
/*
 * AFRC scanline memory layout.
 *
 * Indicates if the buffer uses the scanline-optimised layout
 * for an AFRC encoded buffer, otherwise, it uses the rotation-optimised layout.
 * The memory layout is the same for all planes.
 */
/*
 * Arm 16x16 Block U-Interleaved modifier
 *
 * This is used by Arm Mali Utgard and Midgard GPUs. It divides the image
 * into 16x16 pixel blocks. Blocks are stored linearly in order, but pixels
 * in the block are reordered.
 */
/*
 * ARM 64k interleaved modifier
 *
 * This is used by ARM Mali v10+ GPUs. With this modifier, the plane is divided
 * into 64k byte 1:1 or 2:1 -sided tiles. The 64k tiles are laid out linearly.
 * Each 64k tile is divided into blocks of 16x16 texel blocks, which are
 * themselves laid out linearly within a 64k tile. Then within each 16x16
 * block, texel blocks are laid out according to U order, similar to
 * 16X16_BLOCK_U_INTERLEAVED.
 *
 * Note that unlike 16X16_BLOCK_U_INTERLEAVED, the layout does not change
 * depending on whether a format is compressed or not.
 */
/*
 * Allwinner tiled modifier
 *
 * This tiling mode is implemented by the VPU found on all Allwinner platforms,
 * codenamed sunxi. It is associated with a YUV format that uses either 2 or 3
 * planes.
 *
 * With this tiling, the luminance samples are disposed in tiles representing
 * 32x32 pixels and the chrominance samples in tiles representing 32x64 pixels.
 * The pixel order in each tile is linear and the tiles are disposed linearly,
 * both in row-major order.
 */
/*
 * Amlogic Video Framebuffer Compression modifiers
 *
 * Amlogic uses a proprietary lossless image compression protocol and format
 * for their hardware video codec accelerators, either video decoders or
 * video input encoders.
 *
 * It considerably reduces memory bandwidth while writing and reading
 * frames in memory.
 *
 * The underlying storage is considered to be 3 components, 8bit or 10-bit
 * per component YCbCr 420, single plane :
 * - DRM_FORMAT_YUV420_8BIT
 * - DRM_FORMAT_YUV420_10BIT
 *
 * The first 8 bits of the mode defines the layout, then the following 8 bits
 * defines the options changing the layout.
 *
 * Not all combinations are valid, and different SoCs may support different
 * combinations of layout and options.
 */
/* Amlogic FBC Layouts */
/*
 * Amlogic FBC Basic Layout
 *
 * The basic layout is composed of:
 * - a body content organized in 64x32 superblocks with 4096 bytes per
 *   superblock in default mode.
 * - a 32 bytes per 128x64 header block
 *
 * This layout is transferrable between Amlogic SoCs supporting this modifier.
 */
/*
 * Amlogic FBC Scatter Memory layout
 *
 * Indicates the header contains IOMMU references to the compressed
 * frames content to optimize memory access and layout.
 *
 * In this mode, only the header memory address is needed, thus the
 * content memory organization is tied to the current producer
 * execution and cannot be saved/dumped neither transferrable between
 * Amlogic SoCs supporting this modifier.
 *
 * Due to the nature of the layout, these buffers are not expected to
 * be accessible by the user-space clients, but only accessible by the
 * hardware producers and consumers.
 *
 * The user-space clients should expect a failure while trying to mmap
 * the DMA-BUF handle returned by the producer.
 */
/* Amlogic FBC Layout Options Bit Mask */
/*
 * Amlogic FBC Memory Saving mode
 *
 * Indicates the storage is packed when pixel size is multiple of word
 * boundaries, i.e. 8bit should be stored in this mode to save allocation
 * memory.
 *
 * This mode reduces body layout to 3072 bytes per 64x32 superblock with
 * the basic layout and 3200 bytes per 64x32 superblock combined with
 * the scatter layout.
 */
/* MediaTek modifiers
 * Bits  Parameter                Notes
 * ----- ------------------------ ---------------------------------------------
 *   7: 0 TILE LAYOUT              Values are MTK_FMT_MOD_TILE_*
 *  15: 8 COMPRESSION              Values are MTK_FMT_MOD_COMPRESS_*
 *  23:16 10 BIT LAYOUT            Values are MTK_FMT_MOD_10BIT_LAYOUT_*
 *
 */
/*
 * MediaTek Tiled Modifier
 * The lowest 8 bits of the modifier is used to specify the tiling
 * layout. Only the 16L_32S tiling is used for now, but we define an
 * "untiled" version and leave room for future expansion.
 */
/*
 * Bits 8-15 specify compression options
 */
/*
 * Bits 16-23 specify how the bits of 10 bit formats are
 * stored out in memory
 */
/* alias for the most common tiling format */
/*
 * Apple GPU-tiled layouts.
 *
 * Apple GPUs support nonlinear tilings with optional lossless compression.
 *
 * GPU-tiled images are divided into 16KiB tiles:
 *
 *     Bytes per pixel  Tile size
 *     ---------------  ---------
 *                   1  128x128
 *                   2  128x64
 *                   4  64x64
 *                   8  64x32
 *                  16  32x32
 *
 * Tiles are raster-order. Pixels within a tile are interleaved (Morton order).
 *
 * Compressed images pad the body to 128-bytes and are immediately followed by a
 * metadata section. The metadata section rounds the image dimensions to
 * powers-of-two and contains 8 bytes for each 16x16 compression subtile.
 * Subtiles are interleaved (Morton order).
 *
 * All images are 128-byte aligned.
 *
 * These layouts fundamentally do not have meaningful strides. No matter how we
 * specify strides for these layouts, userspace unaware of Apple image layouts
 * will be unable to use correctly the specified stride for any purpose.
 * Userspace aware of the image layouts do not use strides. The most "correct"
 * convention would be setting the image stride to 0. Unfortunately, some
 * software assumes the stride is at least (width * bytes per pixel). We
 * therefore require that stride equals (width * bytes per pixel). Since the
 * stride is arbitrary here, we pick the simplest convention.
 *
 * Although containing two sections, compressed image layouts are treated in
 * software as a single plane. This is modelled after AFBC, a similar
 * scheme. Attempting to separate the sections to be "explicit" in DRM would
 * only generate more confusion, as software does not treat the image this way.
 *
 * For detailed information on the hardware image layouts, see
 * https://docs.mesa3d.org/drivers/asahi.html#image-layouts
 */
/*
 * AMD modifiers
 *
 * Memory layout:
 *
 * without DCC:
 *   - main surface
 *
 * with DCC & without DCC_RETILE:
 *   - main surface in plane 0
 *   - DCC surface in plane 1 (RB-aligned, pipe-aligned if DCC_PIPE_ALIGN is set)
 *
 * with DCC & DCC_RETILE:
 *   - main surface in plane 0
 *   - displayable DCC surface in plane 1 (not RB-aligned & not pipe-aligned)
 *   - pipe-aligned DCC surface in plane 2 (RB-aligned & pipe-aligned)
 *
 * For multi-plane formats the above surfaces get merged into one plane for
 * each format plane, based on the required alignment only.
 *
 * Bits    Parameter                Notes
 * ------- ------------------------ ---------------------------------------------
 *
 * DRM format modifier fields on AMD GPUs:
 *     7:0 TILE_VERSION             Values are AMD_FMT_MOD_TILE_VER_*
 *    12:8 TILE                     Values are AMD_FMT_MOD_TILE_<version>_*
 *      13 DCC                      Delta Color Compression, supported on GFX8 and newer
 *   55:14 (chip specific)          See below for details, depends on GFX block version
 *   63:56 Vendor                   Value is DRM_FORMAT_MOD_VENDOR_AMD
 *
 * Chip specific fields on Gfx9 and newer:
 *      14 DCC_RETILE
 *      15 DCC_PIPE_ALIGN
 *      16 DCC_INDEPENDENT_64B
 *      17 DCC_INDEPENDENT_128B
 *   19:18 DCC_MAX_COMPRESSED_BLOCK Values are AMD_FMT_MOD_DCC_BLOCK_*
 *      20 DCC_CONSTANT_ENCODE
 *   23:21 PIPE_XOR_BITS            Only for some chips
 *   26:24 BANK_XOR_BITS            Only for some chips
 *   29:27 PACKERS                  Only for some chips
 *   32:30 RB                       Only for some chips
 *   35:33 PIPE                     Only for some chips
 *   55:36 -                        Reserved for future use, must be zero
 *
 * Chip specific fields on Gfx6-8:
 *   16:14 MICROTILE                Micro tile format
 *   21:17 PIPE_CONFIG              Number of pipes and how pipes are interleaved
 *   24:22 TILE_SPLIT               Tile split size
 *   26:25 BANK_WIDTH               Number of tiles in the X direction in the same bank
 *   28:27 BANK_HEIGHT              Number of tiles in the Y direction in the same bank
 *   30:29 MACRO_TILE_ASPECT        Macro tile aspect ratio
 *   32:31 NUM_BANKS                Number of banks
 *   55:33 -                        Reserved for future use, must be zero
 *
 */
/*
 * Gfx6-8 tiling modes.
 * A complete reference implementation is found in addrlib in the Mesa code base.
 *
 * - Microtiled modes (1D):
 *   Pixel data is organized into micro tiles of 8x8 pixels.
 *
 * - Macrotiled modes (2D):
 *   Micro tiles are further organized into macro tiles.
 *   These are optimized for even load distribution among memory channels.
 *
 * Note that only THIN1 modes are exposed here.
 * THICK and XTHICK are for 3D images and not relevant to DRM format modifiers.
 */
/*
 * 64K_S is the same for GFX9/GFX10/GFX10_RBPLUS and hence has GFX9 as canonical
 * version.
 */
/*
 * 64K_D for non-32 bpp is the same for GFX9/GFX10/GFX10_RBPLUS and hence has
 * GFX9 as canonical version.
 *
 * 64K_D_2D on GFX12 is identical to 64K_D on GFX11.
 */
/* Gfx12 swizzle modes:
 *    0 - LINEAR
 *    1 - 256B_2D  - 2D block dimensions
 *    2 - 4KB_2D
 *    3 - 64KB_2D
 *    4 - 256KB_2D
 *    5 - 4KB_3D   - 3D block dimensions
 *    6 - 64KB_3D
 *    7 - 256KB_3D
 */
/* Whether DCC compression is enabled. */
/*
 * Whether to include two DCC surfaces, one which is rb & pipe aligned, and
 * one which is not-aligned.
 */
/* Only set if DCC_RETILE = false */
/*
 * DCC supports embedding some clear colors directly in the DCC surface.
 * However, on older GPUs the rendering HW ignores the embedded clear color
 * and prefers the driver provided color. This necessitates doing a fastclear
 * eliminate operation before a process transfers control.
 *
 * If this bit is set that means the fastclear eliminate is not needed for these
 * embeddable colors.
 */
/*
 * The below fields are for accounting for per GPU differences. These are only
 * relevant for GFX9 and later and if the tile field is *_X/_T.
 *
 * PIPE_XOR_BITS = always needed
 * BANK_XOR_BITS = only for TILE_VER_GFX9
 * PACKERS = only for TILE_VER_GFX10_RBPLUS
 * RB = only for TILE_VER_GFX9 & DCC
 * PIPE = only for TILE_VER_GFX9 & DCC & (DCC_RETILE | DCC_PIPE_ALIGN)
 */
/*
 * MICRO_TILE_MODE, 3 bits. Determines the micro tile format.
 * Only relevant to Gfx6-8.
 *
 * DISPLAY - Displayable tiling
 * THIN - Non-displayable tiling, a.k.a thin micro tiling
 * DEPTH, THICK - not exposed, not relevant to DRM format modifier use cases
 * ROTATED - not exposed, not implemented in Linux or Mesa
 */
/*
 * PIPE_CONFIG, 5 bits. Number of pipes and how pipes are interleaved on the surface,
 * which means the shader engine tile size and packer tile size.
 * Typically matches the number of memory channels, or number of RBs.
 * Only relevant to Gfx6-8 macro tiled modes.
 *
 * P<n>_<a>x<b>_<c>x<d>
 * where:
 * <n> - number of pipes
 * <a>x<b> - shader engine tile size
 * <c>x<d> - packer tile size
 */
/*
 * TILE_SPLIT, 3 bits.
 * Only relevant to Gfx6-8 macro tiled modes.
 *
 * On GFX6 (or with depth tiling modes on GFX7 and newer),
 * the GFX block uses the GB_TILE_MODE.TILE_SPLIT field directly.
 *
 * On GFX7 and newer with non-depth tiling modes, the GFX block uses a
 * split factor which is stored in the GB_TILE_MODE.SAMPLE_SPLIT field.
 * SAMPLE_SPLIT may be: 0 - 1 byte; 1 - 2 bytes; 2 - 4 bytes; 3 - 8 bytes.
 * The actual tile size and tile split bytes are calculated as follows:
 *
 *    bpp = ... <- bits per pixel in the current image
 *    thickness = ... <- depends on array mode; may be: 1, 4, 8
 *    num_samples = ... <- number of samples in the current image
 *    tile_size_pixels = 8 * 8
 *    tile_bytes_1x = thickness * tile_size_pixels * bpp / 8
 *    sample_split_factor = 1 << SAMPLE_SPLIT
 *    tile_split_bytes = clamp(tile_bytes_1x * sample_split_factor, 256, dram_row_size_bytes)
 *    tile_bytes = clamp(tile_bytes_1x * num_samples, 64, tile_split_bytes)
 *
 * In both cases, the display block (DCE) has no SAMPLE_SPLIT
 * and just needs the tile split bytes in the GRPH_CONTROL.GRPH_TILE_SPLIT field.
 * To maximize compatibility between GFX6-7, we don't include the SAMPLE_SPLIT
 * in the format modifiers.
 *
 * The actual tile split in bytes is: 64 << field value
 * Possible values of this field:
 *
 * 0 - Tile split is 64 bytes
 * 1 - Tile split is 128 bytes
 * 2 - Tile split is 256 bytes
 * 3 - Tile split is 512 bytes
 * 4 - Tile split is 1 KiB
 * 5 - Tile split is 2 KiB
 * 6 - Tile split is 4 KiB
 */
/*
 * BANK_WIDTH, 2 bits. Number of tiles in the X direction in the same bank.
 * Only relevant to Gfx6-8 macro tiled modes.
 * The actual bank width is: 1 << field value
 * Possible values:
 *
 * 0 - bank width is 1
 * 1 - bank width is 2
 * 2 - bank width is 4
 * 3 - bank width is 8
 */
/*
 * BANK_HEIGHT, 2 bits. Number of tiles in the Y direction in the same bank.
 * Only relevant to Gfx6-8 macro tiled modes.
 * The actual bank height is: 1 << field value
 * Possible values:
 *
 * 0 - bank height is 1
 * 1 - bank height is 2
 * 2 - bank height is 4
 * 3 - bank height is 8
 */
/*
 * MACRO_TILE_ASPECT, 2 bits. Macro tile aspect ratio.
 * Only relevant to Gfx6-8 macro tiled modes.
 * Possible values:
 *
 * 0 - aspect ratio is 1:1
 * 1 - aspect ratio is 4:1
 * 2 - aspect ratio is 16:1
 * 3 - aspect ratio is 64:1
 */
/*
 * NUM_BANKS, 2 bits. Number of banks.
 * Only relevant to Gfx6-8 macro tiled modes.
 * The actual number of banks is: 2 << field value
 * Possible values:
 *
 * 0 - number of banks is 2
 * 1 - number of banks is 4
 * 2 - number of banks is 8
 * 3 - number of banks is 16
 */

pub const fn drm_fourcc_canonicalize_nvidia_format_mod(modifier: u64) -> u64 {
    if (modifier & 0x10) == 0 || (modifier & (0xffu64 << 12)) != 0 { modifier } else { modifier | (0xfeu64 << 12) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
