/* Rust translation of drm_dp.h. */
/*
 * Copyright © 2008 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting documentation, and
 * that the name of the copyright holders not be used in advertising or
 * publicity pertaining to distribution of the software without specific,
 * written prior permission.  The copyright holders make no representations
 * about the suitability of this software for any purpose.  It is provided "as
 * is" without express or implied warranty.
 *
 * THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
 * OF THIS SOFTWARE.
 */



/*
 * Unless otherwise noted, all values are from the DP 1.1a spec.  Note that
 * DP and DPCD versions are independent.  Differences from 1.0 are not noted,
 * 1.0 devices basically don't exist in the wild.
 *
 * Abbreviations, in chronological order:
 *
 * eDP: Embedded DisplayPort version 1
 * DPI: DisplayPort Interoperability Guideline v1.1a
 * 1.2: DisplayPort 1.2
 * MST: Multistream Transport - part of DP 1.2a
 *
 * 1.2 formally includes both eDP and DPI definitions.
 */

/* MSA (Main Stream Attribute) MISC bits (as MISC1<<8|MISC0) */
pub const DP_MSA_MISC_SYNC_CLOCK: u32 = (1 << 0);
pub const DP_MSA_MISC_INTERLACE_VTOTAL_EVEN: u32 = (1 << 8);
pub const DP_MSA_MISC_STEREO_NO_3D: u32 = (0 << 9);
pub const DP_MSA_MISC_STEREO_PROG_RIGHT_EYE: u32 = (1 << 9);
pub const DP_MSA_MISC_STEREO_PROG_LEFT_EYE: u32 = (3 << 9);
/* bits per component for non-RAW */
pub const DP_MSA_MISC_6_BPC: u32 = (0 << 5);
pub const DP_MSA_MISC_8_BPC: u32 = (1 << 5);
pub const DP_MSA_MISC_10_BPC: u32 = (2 << 5);
pub const DP_MSA_MISC_12_BPC: u32 = (3 << 5);
pub const DP_MSA_MISC_16_BPC: u32 = (4 << 5);
/* bits per component for RAW */
pub const DP_MSA_MISC_RAW_6_BPC: u32 = (1 << 5);
pub const DP_MSA_MISC_RAW_7_BPC: u32 = (2 << 5);
pub const DP_MSA_MISC_RAW_8_BPC: u32 = (3 << 5);
pub const DP_MSA_MISC_RAW_10_BPC: u32 = (4 << 5);
pub const DP_MSA_MISC_RAW_12_BPC: u32 = (5 << 5);
pub const DP_MSA_MISC_RAW_14_BPC: u32 = (6 << 5);
pub const DP_MSA_MISC_RAW_16_BPC: u32 = (7 << 5);
/* pixel encoding/colorimetry format */
// C function-like macro preserved: #define _DP_MSA_MISC_COLOR(misc1_7, misc0_21, misc0_3, misc0_4) \
	((misc1_7) << 15 | (misc0_4) << 4 | (misc0_3) << 3 | ((misc0_21) << 1))
pub const DP_MSA_MISC_COLOR_RGB: u32 = _DP_MSA_MISC_COLOR(0, 0, 0, 0);
pub const DP_MSA_MISC_COLOR_CEA_RGB: u32 = _DP_MSA_MISC_COLOR(0, 0, 1, 0);
pub const DP_MSA_MISC_COLOR_RGB_WIDE_FIXED: u32 = _DP_MSA_MISC_COLOR(0, 3, 0, 0);
pub const DP_MSA_MISC_COLOR_RGB_WIDE_FLOAT: u32 = _DP_MSA_MISC_COLOR(0, 3, 0, 1);
pub const DP_MSA_MISC_COLOR_Y_ONLY: u32 = _DP_MSA_MISC_COLOR(1, 0, 0, 0);
pub const DP_MSA_MISC_COLOR_RAW: u32 = _DP_MSA_MISC_COLOR(1, 1, 0, 0);
pub const DP_MSA_MISC_COLOR_YCBCR_422_BT601: u32 = _DP_MSA_MISC_COLOR(0, 1, 1, 0);
pub const DP_MSA_MISC_COLOR_YCBCR_422_BT709: u32 = _DP_MSA_MISC_COLOR(0, 1, 1, 1);
pub const DP_MSA_MISC_COLOR_YCBCR_444_BT601: u32 = _DP_MSA_MISC_COLOR(0, 2, 1, 0);
pub const DP_MSA_MISC_COLOR_YCBCR_444_BT709: u32 = _DP_MSA_MISC_COLOR(0, 2, 1, 1);
pub const DP_MSA_MISC_COLOR_XVYCC_422_BT601: u32 = _DP_MSA_MISC_COLOR(0, 1, 0, 0);
pub const DP_MSA_MISC_COLOR_XVYCC_422_BT709: u32 = _DP_MSA_MISC_COLOR(0, 1, 0, 1);
pub const DP_MSA_MISC_COLOR_XVYCC_444_BT601: u32 = _DP_MSA_MISC_COLOR(0, 2, 0, 0);
pub const DP_MSA_MISC_COLOR_XVYCC_444_BT709: u32 = _DP_MSA_MISC_COLOR(0, 2, 0, 1);
pub const DP_MSA_MISC_COLOR_OPRGB: u32 = _DP_MSA_MISC_COLOR(0, 0, 1, 1);
pub const DP_MSA_MISC_COLOR_DCI_P3: u32 = _DP_MSA_MISC_COLOR(0, 3, 1, 0);
pub const DP_MSA_MISC_COLOR_COLOR_PROFILE: u32 = _DP_MSA_MISC_COLOR(0, 3, 1, 1);
pub const DP_MSA_MISC_COLOR_VSC_SDP: u32 = (1 << 14);

pub const DP_AUX_MAX_PAYLOAD_BYTES: u32 = 16;

pub const DP_AUX_I2C_WRITE: u32 = 0x0;
pub const DP_AUX_I2C_READ: u32 = 0x1;
pub const DP_AUX_I2C_WRITE_STATUS_UPDATE: u32 = 0x2;
pub const DP_AUX_I2C_MOT: u32 = 0x4;
pub const DP_AUX_NATIVE_WRITE: u32 = 0x8;
pub const DP_AUX_NATIVE_READ: u32 = 0x9;

pub const DP_AUX_NATIVE_REPLY_ACK: u32 = (0x0 << 0);
pub const DP_AUX_NATIVE_REPLY_NACK: u32 = (0x1 << 0);
pub const DP_AUX_NATIVE_REPLY_DEFER: u32 = (0x2 << 0);
pub const DP_AUX_NATIVE_REPLY_MASK: u32 = (0x3 << 0);

pub const DP_AUX_I2C_REPLY_ACK: u32 = (0x0 << 2);
pub const DP_AUX_I2C_REPLY_NACK: u32 = (0x1 << 2);
pub const DP_AUX_I2C_REPLY_DEFER: u32 = (0x2 << 2);
pub const DP_AUX_I2C_REPLY_MASK: u32 = (0x3 << 2);

/* DPCD Field Address Mapping */

/* Receiver Capability */
pub const DP_DPCD_REV: u32 = 0x000;
pub const DP_DPCD_REV_10: u32 = 0x10;
pub const DP_DPCD_REV_11: u32 = 0x11;
pub const DP_DPCD_REV_12: u32 = 0x12;
pub const DP_DPCD_REV_13: u32 = 0x13;
pub const DP_DPCD_REV_14: u32 = 0x14;

pub const DP_MAX_LINK_RATE: u32 = 0x001;

pub const DP_MAX_LANE_COUNT: u32 = 0x002;
pub const DP_MAX_LANE_COUNT_MASK: u32 = 0x1f;
pub const DP_POST_LT_ADJ_REQ_SUPPORTED: u32 = (1 << 5) /* 1.3 */;
pub const DP_TPS3_SUPPORTED: u32 = (1 << 6) /* 1.2 */;
pub const DP_ENHANCED_FRAME_CAP: u32 = (1 << 7);

pub const DP_MAX_DOWNSPREAD: u32 = 0x003;
pub const DP_MAX_DOWNSPREAD_0_5: u32 = (1 << 0);
pub const DP_STREAM_REGENERATION_STATUS_CAP: u32 = (1 << 1) /* 2.0 */;
pub const DP_NO_AUX_HANDSHAKE_LINK_TRAINING: u32 = (1 << 6);
pub const DP_TPS4_SUPPORTED: u32 = (1 << 7);

pub const DP_NORP: u32 = 0x004;

pub const DP_DOWNSTREAMPORT_PRESENT: u32 = 0x005;
pub const DP_DWN_STRM_PORT_PRESENT: u32 = (1 << 0);
pub const DP_DWN_STRM_PORT_TYPE_MASK: u32 = 0x06;
pub const DP_DWN_STRM_PORT_TYPE_DP: u32 = (0 << 1);
pub const DP_DWN_STRM_PORT_TYPE_ANALOG: u32 = (1 << 1);
pub const DP_DWN_STRM_PORT_TYPE_TMDS: u32 = (2 << 1);
pub const DP_DWN_STRM_PORT_TYPE_OTHER: u32 = (3 << 1);
pub const DP_FORMAT_CONVERSION: u32 = (1 << 3);
pub const DP_DETAILED_CAP_INFO_AVAILABLE: u32 = (1 << 4) /* DPI */;

pub const DP_MAIN_LINK_CHANNEL_CODING: u32 = 0x006;
pub const DP_CAP_ANSI_8B10B: u32 = (1 << 0);
pub const DP_CAP_ANSI_128B132B: u32 = (1 << 1) /* 2.0 */;

pub const DP_DOWN_STREAM_PORT_COUNT: u32 = 0x007;
pub const DP_PORT_COUNT_MASK: u32 = 0x0f;
pub const DP_MSA_TIMING_PAR_IGNORED: u32 = (1 << 6) /* eDP */;
pub const DP_OUI_SUPPORT: u32 = (1 << 7);

pub const DP_RECEIVE_PORT_0_CAP_0: u32 = 0x008;
pub const DP_LOCAL_EDID_PRESENT: u32 = (1 << 1);
pub const DP_ASSOCIATED_TO_PRECEDING_PORT: u32 = (1 << 2);
pub const DP_HBLANK_EXPANSION_CAPABLE: u32 = (1 << 3);

pub const DP_RECEIVE_PORT_0_BUFFER_SIZE: u32 = 0x009;

pub const DP_RECEIVE_PORT_1_CAP_0: u32 = 0x00a;
pub const DP_RECEIVE_PORT_1_BUFFER_SIZE: u32 = 0x00b;

pub const DP_I2C_SPEED_CAP: u32 = 0x00c    /* DPI */;
pub const DP_I2C_SPEED_1K: u32 = 0x01;
pub const DP_I2C_SPEED_5K: u32 = 0x02;
pub const DP_I2C_SPEED_10K: u32 = 0x04;
pub const DP_I2C_SPEED_100K: u32 = 0x08;
pub const DP_I2C_SPEED_400K: u32 = 0x10;
pub const DP_I2C_SPEED_1M: u32 = 0x20;

pub const DP_EDP_CONFIGURATION_CAP: u32 = 0x00d   /* XXX 1.2? */;
pub const DP_ALTERNATE_SCRAMBLER_RESET_CAP: u32 = (1 << 0);
pub const DP_FRAMING_CHANGE_CAP: u32 = (1 << 1);
pub const DP_DPCD_DISPLAY_CONTROL_CAPABLE: u32 = (1 << 3) /* edp v1.2 or higher */;

pub const DP_TRAINING_AUX_RD_INTERVAL: u32 = 0x00e   /* XXX 1.2? */;
pub const DP_TRAINING_AUX_RD_MASK: u32 = 0x7F    /* DP 1.3 */;
pub const DP_EXTENDED_RECEIVER_CAP_FIELD_PRESENT: u32 = (1 << 7) /* DP 1.3 */;

pub const DP_ADAPTER_CAP: u32 = 0x00f   /* 1.2 */;
pub const DP_FORCE_LOAD_SENSE_CAP: u32 = (1 << 0);
pub const DP_ALTERNATE_I2C_PATTERN_CAP: u32 = (1 << 1);

pub const DP_SUPPORTED_LINK_RATES: u32 = 0x010 /* eDP 1.4 */;
pub const DP_MAX_SUPPORTED_RATES: u32 = 8	    /* 16-bit little-endian */;

/* Multiple stream transport */
pub const DP_FAUX_CAP: u32 = 0x020   /* 1.2 */;
pub const DP_FAUX_CAP_1: u32 = (1 << 0);

pub const DP_SINK_VIDEO_FALLBACK_FORMATS: u32 = 0x020   /* 2.0 */;
pub const DP_FALLBACK_1024x768_60HZ_24BPP: u32 = (1 << 0);
pub const DP_FALLBACK_1280x720_60HZ_24BPP: u32 = (1 << 1);
pub const DP_FALLBACK_1920x1080_60HZ_24BPP: u32 = (1 << 2);

pub const DP_MSTM_CAP: u32 = 0x021   /* 1.2 */;
pub const DP_MST_CAP: u32 = (1 << 0);
pub const DP_SINGLE_STREAM_SIDEBAND_MSG: u32 = (1 << 1) /* 2.0 */;

pub const DP_NUMBER_OF_AUDIO_ENDPOINTS: u32 = 0x022   /* 1.2 */;

/* AV_SYNC_DATA_BLOCK                                  1.2 */
pub const DP_AV_GRANULARITY: u32 = 0x023;
pub const DP_AG_FACTOR_MASK: u32 = (0xf << 0);
pub const DP_AG_FACTOR_3MS: u32 = (0 << 0);
pub const DP_AG_FACTOR_2MS: u32 = (1 << 0);
pub const DP_AG_FACTOR_1MS: u32 = (2 << 0);
pub const DP_AG_FACTOR_500US: u32 = (3 << 0);
pub const DP_AG_FACTOR_200US: u32 = (4 << 0);
pub const DP_AG_FACTOR_100US: u32 = (5 << 0);
pub const DP_AG_FACTOR_10US: u32 = (6 << 0);
pub const DP_AG_FACTOR_1US: u32 = (7 << 0);
pub const DP_VG_FACTOR_MASK: u32 = (0xf << 4);
pub const DP_VG_FACTOR_3MS: u32 = (0 << 4);
pub const DP_VG_FACTOR_2MS: u32 = (1 << 4);
pub const DP_VG_FACTOR_1MS: u32 = (2 << 4);
pub const DP_VG_FACTOR_500US: u32 = (3 << 4);
pub const DP_VG_FACTOR_200US: u32 = (4 << 4);
pub const DP_VG_FACTOR_100US: u32 = (5 << 4);

pub const DP_AUD_DEC_LAT0: u32 = 0x024;
pub const DP_AUD_DEC_LAT1: u32 = 0x025;

pub const DP_AUD_PP_LAT0: u32 = 0x026;
pub const DP_AUD_PP_LAT1: u32 = 0x027;

pub const DP_VID_INTER_LAT: u32 = 0x028;

pub const DP_VID_PROG_LAT: u32 = 0x029;

pub const DP_REP_LAT: u32 = 0x02a;

pub const DP_AUD_DEL_INS0: u32 = 0x02b;
pub const DP_AUD_DEL_INS1: u32 = 0x02c;
pub const DP_AUD_DEL_INS2: u32 = 0x02d;
/* End of AV_SYNC_DATA_BLOCK */

pub const DP_RECEIVER_ALPM_CAP: u32 = 0x02e   /* eDP 1.4 */;
pub const DP_ALPM_CAP: u32 = (1 << 0);
pub const DP_ALPM_PM_STATE_2A_SUPPORT: u32 = (1 << 1) /* eDP 1.5 */;
pub const DP_ALPM_AUX_LESS_CAP: u32 = (1 << 2) /* eDP 1.5 */;

pub const DP_SINK_DEVICE_AUX_FRAME_SYNC_CAP: u32 = 0x02f   /* eDP 1.4 */;
pub const DP_AUX_FRAME_SYNC_CAP: u32 = (1 << 0);

pub const DP_GUID: u32 = 0x030   /* 1.2 */;

pub const DP_DSC_SUPPORT: u32 = 0x060   /* DP 1.4 */;
pub const DP_DSC_DECOMPRESSION_IS_SUPPORTED: u32 = (1 << 0);
pub const DP_DSC_PASSTHROUGH_IS_SUPPORTED: u32 = (1 << 1);
pub const DP_DSC_DYNAMIC_PPS_UPDATE_SUPPORT_COMP_TO_COMP: u32 = (1 << 2);
pub const DP_DSC_DYNAMIC_PPS_UPDATE_SUPPORT_UNCOMP_TO_COMP: u32 = (1 << 3);

pub const DP_DSC_REV: u32 = 0x061;
pub const DP_DSC_MAJOR_MASK: u32 = (0xf << 0);
pub const DP_DSC_MINOR_MASK: u32 = (0xf << 4);
pub const DP_DSC_MAJOR_SHIFT: u32 = 0;
pub const DP_DSC_MINOR_SHIFT: u32 = 4;

pub const DP_DSC_RC_BUF_BLK_SIZE: u32 = 0x062;
pub const DP_DSC_RC_BUF_BLK_SIZE_1: u32 = 0x0;
pub const DP_DSC_RC_BUF_BLK_SIZE_4: u32 = 0x1;
pub const DP_DSC_RC_BUF_BLK_SIZE_16: u32 = 0x2;
pub const DP_DSC_RC_BUF_BLK_SIZE_64: u32 = 0x3;
pub const DP_DSC_THROUGHPUT_MODE_0_DELTA_SHIFT: u32 = 3 /* DP 2.1a, in units of 2 MPixels/sec */;
pub const DP_DSC_THROUGHPUT_MODE_0_DELTA_MASK: u32 = (0x1f << DP_DSC_THROUGHPUT_MODE_0_DELTA_SHIFT);

pub const DP_DSC_RC_BUF_SIZE: u32 = 0x063;

pub const DP_DSC_SLICE_CAP_1: u32 = 0x064;
pub const DP_DSC_1_PER_DP_DSC_SINK: u32 = (1 << 0);
pub const DP_DSC_2_PER_DP_DSC_SINK: u32 = (1 << 1);
pub const DP_DSC_4_PER_DP_DSC_SINK: u32 = (1 << 3);
pub const DP_DSC_6_PER_DP_DSC_SINK: u32 = (1 << 4);
pub const DP_DSC_8_PER_DP_DSC_SINK: u32 = (1 << 5);
pub const DP_DSC_10_PER_DP_DSC_SINK: u32 = (1 << 6);
pub const DP_DSC_12_PER_DP_DSC_SINK: u32 = (1 << 7);

pub const DP_DSC_LINE_BUF_BIT_DEPTH: u32 = 0x065;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_MASK: u32 = (0xf << 0);
pub const DP_DSC_LINE_BUF_BIT_DEPTH_9: u32 = 0x0;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_10: u32 = 0x1;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_11: u32 = 0x2;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_12: u32 = 0x3;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_13: u32 = 0x4;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_14: u32 = 0x5;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_15: u32 = 0x6;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_16: u32 = 0x7;
pub const DP_DSC_LINE_BUF_BIT_DEPTH_8: u32 = 0x8;

pub const DP_DSC_BLK_PREDICTION_SUPPORT: u32 = 0x066;
pub const DP_DSC_BLK_PREDICTION_IS_SUPPORTED: u32 = (1 << 0);
pub const DP_DSC_RGB_COLOR_CONV_BYPASS_SUPPORT: u32 = (1 << 1);

pub const DP_DSC_MAX_BITS_PER_PIXEL_LOW: u32 = 0x067   /* eDP 1.4 */;

pub const DP_DSC_MAX_BITS_PER_PIXEL_HI: u32 = 0x068   /* eDP 1.4 */;
pub const DP_DSC_MAX_BITS_PER_PIXEL_HI_MASK: u32 = (0x3 << 0);
pub const DP_DSC_MAX_BPP_DELTA_VERSION_MASK: u32 = (0x3 << 5)	/* eDP 1.5 & DP 2.0 */;
pub const DP_DSC_MAX_BPP_DELTA_AVAILABILITY: u32 = (1 << 7)	/* eDP 1.5 & DP 2.0 */;

pub const DP_DSC_DEC_COLOR_FORMAT_CAP: u32 = 0x069;
pub const DP_DSC_RGB: u32 = (1 << 0);
pub const DP_DSC_YCbCr444: u32 = (1 << 1);
pub const DP_DSC_YCbCr422_Simple: u32 = (1 << 2);
pub const DP_DSC_YCbCr422_Native: u32 = (1 << 3);
pub const DP_DSC_YCbCr420_Native: u32 = (1 << 4);

pub const DP_DSC_DEC_COLOR_DEPTH_CAP: u32 = 0x06A;
pub const DP_DSC_8_BPC: u32 = (1 << 1);
pub const DP_DSC_10_BPC: u32 = (1 << 2);
pub const DP_DSC_12_BPC: u32 = (1 << 3);

pub const DP_DSC_PEAK_THROUGHPUT: u32 = 0x06B;
pub const DP_DSC_THROUGHPUT_MODE_0_MASK: u32 = (0xf << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_SHIFT: u32 = 0;
pub const DP_DSC_THROUGHPUT_MODE_0_UNSUPPORTED: u32 = 0;
pub const DP_DSC_THROUGHPUT_MODE_0_340: u32 = (1 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_400: u32 = (2 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_450: u32 = (3 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_500: u32 = (4 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_550: u32 = (5 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_600: u32 = (6 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_650: u32 = (7 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_700: u32 = (8 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_750: u32 = (9 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_800: u32 = (10 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_850: u32 = (11 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_900: u32 = (12 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_950: u32 = (13 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_1000: u32 = (14 << 0);
pub const DP_DSC_THROUGHPUT_MODE_0_170: u32 = (15 << 0) /* 1.4a */;
pub const DP_DSC_THROUGHPUT_MODE_1_MASK: u32 = (0xf << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_SHIFT: u32 = 4;
pub const DP_DSC_THROUGHPUT_MODE_1_UNSUPPORTED: u32 = 0;
pub const DP_DSC_THROUGHPUT_MODE_1_340: u32 = (1 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_400: u32 = (2 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_450: u32 = (3 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_500: u32 = (4 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_550: u32 = (5 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_600: u32 = (6 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_650: u32 = (7 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_700: u32 = (8 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_750: u32 = (9 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_800: u32 = (10 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_850: u32 = (11 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_900: u32 = (12 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_950: u32 = (13 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_1000: u32 = (14 << 4);
pub const DP_DSC_THROUGHPUT_MODE_1_170: u32 = (15 << 4);

pub const DP_DSC_MAX_SLICE_WIDTH: u32 = 0x06C;
pub const DP_DSC_MIN_SLICE_WIDTH_VALUE: u32 = 2560;
pub const DP_DSC_SLICE_WIDTH_MULTIPLIER: u32 = 320;

pub const DP_DSC_SLICE_CAP_2: u32 = 0x06D;
pub const DP_DSC_16_PER_DP_DSC_SINK: u32 = (1 << 0);
pub const DP_DSC_20_PER_DP_DSC_SINK: u32 = (1 << 1);
pub const DP_DSC_24_PER_DP_DSC_SINK: u32 = (1 << 2);

pub const DP_DSC_MAX_BPP_DELTA_VERSION_1: u32 = 0x06E;
pub const DP_DSC_RGB_YCbCr444_MAX_BPP_DELTA_MASK: u32 = 0x1f;
pub const DP_DSC_NATIVE_YCbCr420_MAX_BPP_DELTA_MASK: u32 = 0xe0;

pub const DP_DSC_BPP_DELTA_444: u32 = 16;
pub const DP_DSC_BPP_DELTA_420: u32 = 12;
pub const DP_DSC_BPP_DELTA_SHIFT_420: u32 = 5;

pub const DP_DSC_BITS_PER_PIXEL_INC: u32 = 0x06F;
pub const DP_DSC_BITS_PER_PIXEL_1_16: u32 = 0x0;
pub const DP_DSC_BITS_PER_PIXEL_1_8: u32 = 0x1;
pub const DP_DSC_BITS_PER_PIXEL_1_4: u32 = 0x2;
pub const DP_DSC_BITS_PER_PIXEL_1_2: u32 = 0x3;
pub const DP_DSC_BITS_PER_PIXEL_1_1: u32 = 0x4;
pub const DP_DSC_BITS_PER_PIXEL_MASK: u32 = 0x7;
pub const DP_DSC_NATIVE_YCbCr422_MAX_BPP_DELTA_MASK: u32 = 0x78;
pub const DP_DSC_BPP_DELTA_NATIVE_SHIFT_422: u32 = 3;
pub const DP_DSC_BPP_DELTA_NATIVE_422: u32 = 16;

pub const DP_PSR_SUPPORT: u32 = 0x070   /* XXX 1.2? */;
pub const DP_PSR_IS_SUPPORTED: u32 = 1;
pub const DP_PSR2_IS_SUPPORTED: u32 = 2	    /* eDP 1.4 */;
pub const DP_PSR2_WITH_Y_COORD_IS_SUPPORTED: u32 = 3	    /* eDP 1.4a */;
pub const DP_PSR2_WITH_Y_COORD_ET_SUPPORTED: u32 = 4	    /* eDP 1.5, adopted eDP 1.4b SCR */;

pub const DP_PSR_CAPS: u32 = 0x071   /* XXX 1.2? */;
pub const DP_PSR_NO_TRAIN_ON_EXIT: u32 = 1;
pub const DP_PSR_SETUP_TIME_330: u32 = (0 << 1);
pub const DP_PSR_SETUP_TIME_275: u32 = (1 << 1);
pub const DP_PSR_SETUP_TIME_220: u32 = (2 << 1);
pub const DP_PSR_SETUP_TIME_165: u32 = (3 << 1);
pub const DP_PSR_SETUP_TIME_110: u32 = (4 << 1);
pub const DP_PSR_SETUP_TIME_55: u32 = (5 << 1);
pub const DP_PSR_SETUP_TIME_0: u32 = (6 << 1);
pub const DP_PSR_SETUP_TIME_MASK: u32 = (7 << 1);
pub const DP_PSR_SETUP_TIME_SHIFT: u32 = 1;
pub const DP_PSR2_SU_Y_COORDINATE_REQUIRED: u32 = (1 << 4)  /* eDP 1.4a */;
pub const DP_PSR2_SU_GRANULARITY_REQUIRED: u32 = (1 << 5)  /* eDP 1.4b */;
pub const DP_PSR2_SU_AUX_FRAME_SYNC_NOT_NEEDED: u32 = (1 << 6)/* eDP 1.5, adopted eDP 1.4b SCR */;

pub const DP_PSR2_SU_X_GRANULARITY: u32 = 0x072 /* eDP 1.4b */;
pub const DP_PSR2_SU_Y_GRANULARITY: u32 = 0x074 /* eDP 1.4b */;

/*
 * 0x80-0x8f describe downstream port capabilities, but there are two layouts
 * based on whether DP_DETAILED_CAP_INFO_AVAILABLE was set.  If it was not,
 * each port's descriptor is one byte wide.  If it was set, each port's is
 * four bytes wide, starting with the one byte from the base info.  As of
 * DP interop v1.1a only VGA defines additional detail.
 */

/* offset 0 */
pub const DP_DOWNSTREAM_PORT_0: u32 = 0x80;
pub const DP_DS_PORT_TYPE_MASK: u32 = (7 << 0);
pub const DP_DS_PORT_TYPE_DP: u32 = 0;
pub const DP_DS_PORT_TYPE_VGA: u32 = 1;
pub const DP_DS_PORT_TYPE_DVI: u32 = 2;
pub const DP_DS_PORT_TYPE_HDMI: u32 = 3;
pub const DP_DS_PORT_TYPE_NON_EDID: u32 = 4;
pub const DP_DS_PORT_TYPE_DP_DUALMODE: u32 = 5;
pub const DP_DS_PORT_TYPE_WIRELESS: u32 = 6;
pub const DP_DS_PORT_HPD: u32 = (1 << 3);
pub const DP_DS_NON_EDID_MASK: u32 = (0xf << 4);
pub const DP_DS_NON_EDID_720x480i_60: u32 = (1 << 4);
pub const DP_DS_NON_EDID_720x480i_50: u32 = (2 << 4);
pub const DP_DS_NON_EDID_1920x1080i_60: u32 = (3 << 4);
pub const DP_DS_NON_EDID_1920x1080i_50: u32 = (4 << 4);
pub const DP_DS_NON_EDID_1280x720_60: u32 = (5 << 4);
pub const DP_DS_NON_EDID_1280x720_50: u32 = (7 << 4);
/* offset 1 for VGA is maximum megapixels per second / 8 */
/* offset 1 for DVI/HDMI is maximum TMDS clock in Mbps / 2.5 */
/* offset 2 for VGA/DVI/HDMI */
pub const DP_DS_MAX_BPC_MASK: u32 = (3 << 0);
pub const DP_DS_8BPC: u32 = 0;
pub const DP_DS_10BPC: u32 = 1;
pub const DP_DS_12BPC: u32 = 2;
pub const DP_DS_16BPC: u32 = 3;
/* HDMI2.1 PCON FRL CONFIGURATION */
pub const DP_PCON_MAX_FRL_BW: u32 = (7 << 2);
pub const DP_PCON_MAX_0GBPS: u32 = (0 << 2);
pub const DP_PCON_MAX_9GBPS: u32 = (1 << 2);
pub const DP_PCON_MAX_18GBPS: u32 = (2 << 2);
pub const DP_PCON_MAX_24GBPS: u32 = (3 << 2);
pub const DP_PCON_MAX_32GBPS: u32 = (4 << 2);
pub const DP_PCON_MAX_40GBPS: u32 = (5 << 2);
pub const DP_PCON_MAX_48GBPS: u32 = (6 << 2);
pub const DP_PCON_SOURCE_CTL_MODE: u32 = (1 << 5);

/* offset 3 for DVI */
pub const DP_DS_DVI_DUAL_LINK: u32 = (1 << 1);
pub const DP_DS_DVI_HIGH_COLOR_DEPTH: u32 = (1 << 2);
/* offset 3 for HDMI */
pub const DP_DS_HDMI_FRAME_SEQ_TO_FRAME_PACK: u32 = (1 << 0);
pub const DP_DS_HDMI_YCBCR422_PASS_THROUGH: u32 = (1 << 1);
pub const DP_DS_HDMI_YCBCR420_PASS_THROUGH: u32 = (1 << 2);
pub const DP_DS_HDMI_YCBCR444_TO_422_CONV: u32 = (1 << 3);
pub const DP_DS_HDMI_YCBCR444_TO_420_CONV: u32 = (1 << 4);

/*
 * VESA DP-to-HDMI PCON Specification adds caps for colorspace
 * conversion in DFP cap DPCD 83h. Sec6.1 Table-3.
 * Based on the available support the source can enable
 * color conversion by writing into PROTOCOL_COVERTER_CONTROL_2
 * DPCD 3052h.
 */
pub const DP_DS_HDMI_BT601_RGB_YCBCR_CONV: u32 = (1 << 5);
pub const DP_DS_HDMI_BT709_RGB_YCBCR_CONV: u32 = (1 << 6);
pub const DP_DS_HDMI_BT2020_RGB_YCBCR_CONV: u32 = (1 << 7);

pub const DP_MAX_DOWNSTREAM_PORTS: u32 = 0x10;

/* DP Forward error Correction Registers */
pub const DP_FEC_CAPABILITY: u32 = 0x090    /* 1.4 */;
pub const DP_FEC_CAPABLE: u32 = (1 << 0);
pub const DP_FEC_UNCORR_BLK_ERROR_COUNT_CAP: u32 = (1 << 1);
pub const DP_FEC_CORR_BLK_ERROR_COUNT_CAP: u32 = (1 << 2);
pub const DP_FEC_BIT_ERROR_COUNT_CAP: u32 = (1 << 3);
pub const DP_FEC_CAPABILITY_1: u32 = 0x091   /* 2.0 */;

/* DP-HDMI2.1 PCON DSC ENCODER SUPPORT */
pub const DP_PCON_DSC_ENCODER_CAP_SIZE: u32 = 0xD	/* 0x92 through 0x9E */;
pub const DP_PCON_DSC_ENCODER: u32 = 0x092;
pub const DP_PCON_DSC_ENCODER_SUPPORTED: u32 = (1 << 0);
pub const DP_PCON_DSC_PPS_ENC_OVERRIDE: u32 = (1 << 1);

/* DP-HDMI2.1 PCON DSC Version */
pub const DP_PCON_DSC_VERSION: u32 = 0x093;
pub const DP_PCON_DSC_MAJOR_MASK: u32 = (0xF << 0);
pub const DP_PCON_DSC_MINOR_MASK: u32 = (0xF << 4);
pub const DP_PCON_DSC_MAJOR_SHIFT: u32 = 0;
pub const DP_PCON_DSC_MINOR_SHIFT: u32 = 4;

/* DP-HDMI2.1 PCON DSC RC Buffer block size */
pub const DP_PCON_DSC_RC_BUF_BLK_INFO: u32 = 0x094;
pub const DP_PCON_DSC_RC_BUF_BLK_SIZE: u32 = (0x3 << 0);
pub const DP_PCON_DSC_RC_BUF_BLK_1KB: u32 = 0;
pub const DP_PCON_DSC_RC_BUF_BLK_4KB: u32 = 1;
pub const DP_PCON_DSC_RC_BUF_BLK_16KB: u32 = 2;
pub const DP_PCON_DSC_RC_BUF_BLK_64KB: u32 = 3;

/* DP-HDMI2.1 PCON DSC RC Buffer size */
pub const DP_PCON_DSC_RC_BUF_SIZE: u32 = 0x095;

/* DP-HDMI2.1 PCON DSC Slice capabilities-1 */
pub const DP_PCON_DSC_SLICE_CAP_1: u32 = 0x096;
pub const DP_PCON_DSC_1_PER_DSC_ENC: u32 = (0x1 << 0);
pub const DP_PCON_DSC_2_PER_DSC_ENC: u32 = (0x1 << 1);
pub const DP_PCON_DSC_4_PER_DSC_ENC: u32 = (0x1 << 3);
pub const DP_PCON_DSC_6_PER_DSC_ENC: u32 = (0x1 << 4);
pub const DP_PCON_DSC_8_PER_DSC_ENC: u32 = (0x1 << 5);
pub const DP_PCON_DSC_10_PER_DSC_ENC: u32 = (0x1 << 6);
pub const DP_PCON_DSC_12_PER_DSC_ENC: u32 = (0x1 << 7);

pub const DP_PCON_DSC_BUF_BIT_DEPTH: u32 = 0x097;
pub const DP_PCON_DSC_BIT_DEPTH_MASK: u32 = (0xF << 0);
pub const DP_PCON_DSC_DEPTH_9_BITS: u32 = 0;
pub const DP_PCON_DSC_DEPTH_10_BITS: u32 = 1;
pub const DP_PCON_DSC_DEPTH_11_BITS: u32 = 2;
pub const DP_PCON_DSC_DEPTH_12_BITS: u32 = 3;
pub const DP_PCON_DSC_DEPTH_13_BITS: u32 = 4;
pub const DP_PCON_DSC_DEPTH_14_BITS: u32 = 5;
pub const DP_PCON_DSC_DEPTH_15_BITS: u32 = 6;
pub const DP_PCON_DSC_DEPTH_16_BITS: u32 = 7;
pub const DP_PCON_DSC_DEPTH_8_BITS: u32 = 8;

pub const DP_PCON_DSC_BLOCK_PREDICTION: u32 = 0x098;
pub const DP_PCON_DSC_BLOCK_PRED_SUPPORT: u32 = (0x1 << 0);

pub const DP_PCON_DSC_ENC_COLOR_FMT_CAP: u32 = 0x099;
pub const DP_PCON_DSC_ENC_RGB: u32 = (0x1 << 0);
pub const DP_PCON_DSC_ENC_YUV444: u32 = (0x1 << 1);
pub const DP_PCON_DSC_ENC_YUV422_S: u32 = (0x1 << 2);
pub const DP_PCON_DSC_ENC_YUV422_N: u32 = (0x1 << 3);
pub const DP_PCON_DSC_ENC_YUV420_N: u32 = (0x1 << 4);

pub const DP_PCON_DSC_ENC_COLOR_DEPTH_CAP: u32 = 0x09A;
pub const DP_PCON_DSC_ENC_8BPC: u32 = (0x1 << 1);
pub const DP_PCON_DSC_ENC_10BPC: u32 = (0x1 << 2);
pub const DP_PCON_DSC_ENC_12BPC: u32 = (0x1 << 3);

pub const DP_PCON_DSC_MAX_SLICE_WIDTH: u32 = 0x09B;

/* DP-HDMI2.1 PCON DSC Slice capabilities-2 */
pub const DP_PCON_DSC_SLICE_CAP_2: u32 = 0x09C;
pub const DP_PCON_DSC_16_PER_DSC_ENC: u32 = (0x1 << 0);
pub const DP_PCON_DSC_20_PER_DSC_ENC: u32 = (0x1 << 1);
pub const DP_PCON_DSC_24_PER_DSC_ENC: u32 = (0x1 << 2);

/* DP-HDMI2.1 PCON HDMI TX Encoder Bits/pixel increment */
pub const DP_PCON_DSC_BPP_INCR: u32 = 0x09E;
pub const DP_PCON_DSC_BPP_INCR_MASK: u32 = (0x7 << 0);
pub const DP_PCON_DSC_ONE_16TH_BPP: u32 = 0;
pub const DP_PCON_DSC_ONE_8TH_BPP: u32 = 1;
pub const DP_PCON_DSC_ONE_4TH_BPP: u32 = 2;
pub const DP_PCON_DSC_ONE_HALF_BPP: u32 = 3;
pub const DP_PCON_DSC_ONE_BPP: u32 = 4;

/* DP Extended DSC Capabilities */
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_0: u32 = 0x0a0   /* DP 1.4a SCR */;
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_1: u32 = 0x0a1;
pub const DP_DSC_BRANCH_MAX_LINE_WIDTH: u32 = 0x0a2;

/* DFP Capability Extension */
pub const DP_DFP_CAPABILITY_EXTENSION_SUPPORT: u32 = 0x0a3	/* 2.0 */;

pub const DP_PANEL_REPLAY_CAP_SUPPORT: u32 = 0x0b0  /* DP 2.0 */;
pub const DP_PANEL_REPLAY_SUPPORT: u32 = (1 << 0);
pub const DP_PANEL_REPLAY_SU_SUPPORT: u32 = (1 << 1);
pub const DP_PANEL_REPLAY_EARLY_TRANSPORT_SUPPORT: u32 = (1 << 2) /* eDP 1.5 */;

pub const DP_PANEL_REPLAY_CAP_SIZE: u32 = 7;

pub const DP_PANEL_REPLAY_CAP_CAPABILITY: u32 = 0xb1;
pub const DP_PANEL_REPLAY_DSC_DECODE_CAPABILITY_IN_PR_SHIFT: u32 = 1 /* DP 2.1a */;
pub const DP_PANEL_REPLAY_DSC_DECODE_CAPABILITY_IN_PR_MASK: u32 = (3 << DP_PANEL_REPLAY_DSC_DECODE_CAPABILITY_IN_PR_SHIFT);
pub const DP_DSC_DECODE_CAPABILITY_IN_PR_SUPPORTED: u32 = 0x00;
pub const DP_DSC_DECODE_CAPABILITY_IN_PR_FULL_FRAME_ONLY: u32 = 0x01;
pub const DP_DSC_DECODE_CAPABILITY_IN_PR_NOT_SUPPORTED: u32 = 0x02;
pub const DP_DSC_DECODE_CAPABILITY_IN_PR_RESERVED: u32 = 0x03;
pub const DP_PANEL_REPLAY_ASYNC_VIDEO_TIMING_NOT_SUPPORTED_IN_PR: u32 = (1 << 3);
pub const DP_PANEL_REPLAY_DSC_CRC_OF_MULTIPLE_SUS_SUPPORTED: u32 = (1 << 4);
pub const DP_PANEL_REPLAY_SU_GRANULARITY_REQUIRED: u32 = (1 << 5);
pub const DP_PANEL_REPLAY_SU_Y_GRANULARITY_EXTENDED_CAPABILITY_SUPPORTED: u32 = (1 << 6);
pub const DP_PANEL_REPLAY_LINK_OFF_SUPPORTED_IN_PR_AFTER_ADAPTIVE_SYNC_SDP: u32 = (1 << 7);

pub const DP_PANEL_REPLAY_CAP_X_GRANULARITY: u32 = 0xb2;
pub const DP_PANEL_REPLAY_FULL_LINE_GRANULARITY: u32 = 0xffff;

pub const DP_PANEL_REPLAY_CAP_Y_GRANULARITY: u32 = 0xb4;

/* Link Configuration */
pub const DP_LINK_BW_SET: u32 = 0x100;
pub const DP_LINK_RATE_TABLE: u32 = 0x00    /* eDP 1.4 */;
pub const DP_LINK_BW_1_62: u32 = 0x06;
pub const DP_LINK_BW_2_7: u32 = 0x0a;
pub const DP_LINK_BW_5_4: u32 = 0x14    /* 1.2 */;
pub const DP_LINK_BW_8_1: u32 = 0x1e    /* 1.4 */;
pub const DP_LINK_BW_10: u32 = 0x01    /* 2.0 128b/132b Link Layer */;
pub const DP_LINK_BW_13_5: u32 = 0x04    /* 2.0 128b/132b Link Layer */;
pub const DP_LINK_BW_20: u32 = 0x02    /* 2.0 128b/132b Link Layer */;

pub const DP_LANE_COUNT_SET: u32 = 0x101;
pub const DP_LANE_COUNT_MASK: u32 = 0x0f;
pub const DP_POST_LT_ADJ_REQ_GRANTED: u32 = (1 << 5) /* 1.3 */;
pub const DP_LANE_COUNT_ENHANCED_FRAME_EN: u32 = (1 << 7);

pub const DP_TRAINING_PATTERN_SET: u32 = 0x102;
pub const DP_TRAINING_PATTERN_DISABLE: u32 = 0;
pub const DP_TRAINING_PATTERN_1: u32 = 1;
pub const DP_TRAINING_PATTERN_2: u32 = 2;
pub const DP_TRAINING_PATTERN_2_CDS: u32 = 3	    /* 2.0 E11 */;
pub const DP_TRAINING_PATTERN_3: u32 = 3	    /* 1.2 */;
pub const DP_TRAINING_PATTERN_4: u32 = 7       /* 1.4 */;
pub const DP_TRAINING_PATTERN_MASK: u32 = 0x3;
pub const DP_TRAINING_PATTERN_MASK_1_4: u32 = 0xf;

/* DPCD 1.1 only. For DPCD >= 1.2 see per-lane DP_LINK_QUAL_LANEn_SET */
pub const DP_LINK_QUAL_PATTERN_11_DISABLE: u32 = (0 << 2);
pub const DP_LINK_QUAL_PATTERN_11_D10_2: u32 = (1 << 2);
pub const DP_LINK_QUAL_PATTERN_11_ERROR_RATE: u32 = (2 << 2);
pub const DP_LINK_QUAL_PATTERN_11_PRBS7: u32 = (3 << 2);
pub const DP_LINK_QUAL_PATTERN_11_MASK: u32 = (3 << 2);

pub const DP_RECOVERED_CLOCK_OUT_EN: u32 = (1 << 4);
pub const DP_LINK_SCRAMBLING_DISABLE: u32 = (1 << 5);

pub const DP_SYMBOL_ERROR_COUNT_BOTH: u32 = (0 << 6);
pub const DP_SYMBOL_ERROR_COUNT_DISPARITY: u32 = (1 << 6);
pub const DP_SYMBOL_ERROR_COUNT_SYMBOL: u32 = (2 << 6);
pub const DP_SYMBOL_ERROR_COUNT_MASK: u32 = (3 << 6);

pub const DP_TRAINING_LANE0_SET: u32 = 0x103;
pub const DP_TRAINING_LANE1_SET: u32 = 0x104;
pub const DP_TRAINING_LANE2_SET: u32 = 0x105;
pub const DP_TRAINING_LANE3_SET: u32 = 0x106;

pub const DP_TRAIN_VOLTAGE_SWING_MASK: u32 = 0x3;
pub const DP_TRAIN_VOLTAGE_SWING_SHIFT: u32 = 0;
pub const DP_TRAIN_MAX_SWING_REACHED: u32 = (1 << 2);
pub const DP_TRAIN_VOLTAGE_SWING_LEVEL_0: u32 = (0 << 0);
pub const DP_TRAIN_VOLTAGE_SWING_LEVEL_1: u32 = (1 << 0);
pub const DP_TRAIN_VOLTAGE_SWING_LEVEL_2: u32 = (2 << 0);
pub const DP_TRAIN_VOLTAGE_SWING_LEVEL_3: u32 = (3 << 0);

pub const DP_TRAIN_PRE_EMPHASIS_MASK: u32 = (3 << 3);
pub const DP_TRAIN_PRE_EMPH_LEVEL_0: u32 = (0 << 3);
pub const DP_TRAIN_PRE_EMPH_LEVEL_1: u32 = (1 << 3);
pub const DP_TRAIN_PRE_EMPH_LEVEL_2: u32 = (2 << 3);
pub const DP_TRAIN_PRE_EMPH_LEVEL_3: u32 = (3 << 3);

pub const DP_TRAIN_PRE_EMPHASIS_SHIFT: u32 = 3;
pub const DP_TRAIN_MAX_PRE_EMPHASIS_REACHED: u32 = (1 << 5);

pub const DP_TX_FFE_PRESET_VALUE_MASK: u32 = (0xf << 0) /* 2.0 128b/132b Link Layer */;

pub const DP_DOWNSPREAD_CTRL: u32 = 0x107;
pub const DP_SPREAD_AMP_0_5: u32 = (1 << 4);
pub const DP_FIXED_VTOTAL_AS_SDP_EN_IN_PR_ACTIVE: u32 = (1 << 6);
pub const DP_MSA_TIMING_PAR_IGNORE_EN: u32 = (1 << 7) /* eDP */;

pub const DP_MAIN_LINK_CHANNEL_CODING_SET: u32 = 0x108;
pub const DP_SET_ANSI_8B10B: u32 = (1 << 0);
pub const DP_SET_ANSI_128B132B: u32 = (1 << 1);

pub const DP_I2C_SPEED_CONTROL_STATUS: u32 = 0x109   /* DPI */;
/* bitmask as for DP_I2C_SPEED_CAP */

pub const DP_EDP_CONFIGURATION_SET: u32 = 0x10a   /* XXX 1.2? */;
pub const DP_ALTERNATE_SCRAMBLER_RESET_ENABLE: u32 = (1 << 0);
pub const DP_FRAMING_CHANGE_ENABLE: u32 = (1 << 1);
pub const DP_PANEL_SELF_TEST_ENABLE: u32 = (1 << 7);

pub const DP_LINK_QUAL_LANE0_SET: u32 = 0x10b   /* DPCD >= 1.2 */;
pub const DP_LINK_QUAL_LANE1_SET: u32 = 0x10c;
pub const DP_LINK_QUAL_LANE2_SET: u32 = 0x10d;
pub const DP_LINK_QUAL_LANE3_SET: u32 = 0x10e;
pub const DP_LINK_QUAL_PATTERN_DISABLE: u32 = 0;
pub const DP_LINK_QUAL_PATTERN_D10_2: u32 = 1;
pub const DP_LINK_QUAL_PATTERN_ERROR_RATE: u32 = 2;
pub const DP_LINK_QUAL_PATTERN_PRBS7: u32 = 3;
pub const DP_LINK_QUAL_PATTERN_80BIT_CUSTOM: u32 = 4;
pub const DP_LINK_QUAL_PATTERN_CP2520_PAT_1: u32 = 5;
pub const DP_LINK_QUAL_PATTERN_CP2520_PAT_2: u32 = 6;
pub const DP_LINK_QUAL_PATTERN_CP2520_PAT_3: u32 = 7;
/* DP 2.0 UHBR10, UHBR13.5, UHBR20 */
pub const DP_LINK_QUAL_PATTERN_128B132B_TPS1: u32 = 0x08;
pub const DP_LINK_QUAL_PATTERN_128B132B_TPS2: u32 = 0x10;
pub const DP_LINK_QUAL_PATTERN_PRSBS9: u32 = 0x18;
pub const DP_LINK_QUAL_PATTERN_PRSBS11: u32 = 0x20;
pub const DP_LINK_QUAL_PATTERN_PRSBS15: u32 = 0x28;
pub const DP_LINK_QUAL_PATTERN_PRSBS23: u32 = 0x30;
pub const DP_LINK_QUAL_PATTERN_PRSBS31: u32 = 0x38;
pub const DP_LINK_QUAL_PATTERN_CUSTOM: u32 = 0x40;
pub const DP_LINK_QUAL_PATTERN_SQUARE: u32 = 0x48;
pub const DP_LINK_QUAL_PATTERN_SQUARE_PRESHOOT_DISABLED: u32 = 0x49;
pub const DP_LINK_QUAL_PATTERN_SQUARE_DEEMPHASIS_DISABLED: u32 = 0x4a;
pub const DP_LINK_QUAL_PATTERN_SQUARE_PRESHOOT_DEEMPHASIS_DISABLED: u32 = 0x4b;

pub const DP_TRAINING_LANE0_1_SET2: u32 = 0x10f;
pub const DP_TRAINING_LANE2_3_SET2: u32 = 0x110;
pub const DP_LANE02_POST_CURSOR2_SET_MASK: u32 = (3 << 0);
pub const DP_LANE02_MAX_POST_CURSOR2_REACHED: u32 = (1 << 2);
pub const DP_LANE13_POST_CURSOR2_SET_MASK: u32 = (3 << 4);
pub const DP_LANE13_MAX_POST_CURSOR2_REACHED: u32 = (1 << 6);

pub const DP_MSTM_CTRL: u32 = 0x111   /* 1.2 */;
pub const DP_MST_EN: u32 = (1 << 0);
pub const DP_UP_REQ_EN: u32 = (1 << 1);
pub const DP_UPSTREAM_IS_SRC: u32 = (1 << 2);

pub const DP_AUDIO_DELAY0: u32 = 0x112   /* 1.2 */;
pub const DP_AUDIO_DELAY1: u32 = 0x113;
pub const DP_AUDIO_DELAY2: u32 = 0x114;

pub const DP_LINK_RATE_SET: u32 = 0x115   /* eDP 1.4 */;
pub const DP_LINK_RATE_SET_SHIFT: u32 = 0;
pub const DP_LINK_RATE_SET_MASK: u32 = (7 << 0);

pub const DP_RECEIVER_ALPM_CONFIG: u32 = 0x116   /* eDP 1.4 */;
pub const DP_ALPM_ENABLE: u32 = (1 << 0);
pub const DP_ALPM_LOCK_ERROR_IRQ_HPD_ENABLE: u32 = (1 << 1) /* eDP 1.5 */;
pub const DP_ALPM_MODE_AUX_LESS: u32 = (1 << 2) /* eDP 1.5 */;

pub const DP_SINK_DEVICE_AUX_FRAME_SYNC_CONF: u32 = 0x117   /* eDP 1.4 */;
pub const DP_AUX_FRAME_SYNC_ENABLE: u32 = (1 << 0);
pub const DP_IRQ_HPD_ENABLE: u32 = (1 << 1);

pub const DP_UPSTREAM_DEVICE_DP_PWR_NEED: u32 = 0x118   /* 1.2 */;
pub const DP_PWR_NOT_NEEDED: u32 = (1 << 0);

pub const DP_EXTENDED_DPRX_SLEEP_WAKE_TIMEOUT_GRANT: u32 = 0x119   /* 1.4a */;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_GRANTED: u32 = (1 << 0);

pub const PANEL_REPLAY_CONFIG3: u32 = 0x11a /* DP 2.1 */;
pub const DP_PR_AS_SDP_SETUP_TIME_MASK: u32 = (3 << 6);
pub const DP_PR_AS_SDP_SETUP_TIME_T1: u32 = (0 << 6);
pub const DP_PR_AS_SDP_SETUP_TIME_DYNAMIC: u32 = (1 << 6) /* DP 2.1 Table 2-227 */;
pub const DP_PR_AS_SDP_SETUP_TIME_T2: u32 = (2 << 6);

pub const DP_FEC_CONFIGURATION: u32 = 0x120    /* 1.4 */;
pub const DP_FEC_READY: u32 = (1 << 0);
pub const DP_FEC_ERR_COUNT_SEL_MASK: u32 = (7 << 1);
pub const DP_FEC_ERR_COUNT_DIS: u32 = (0 << 1);
pub const DP_FEC_UNCORR_BLK_ERROR_COUNT: u32 = (1 << 1);
pub const DP_FEC_CORR_BLK_ERROR_COUNT: u32 = (2 << 1);
pub const DP_FEC_BIT_ERROR_COUNT: u32 = (3 << 1);
pub const DP_FEC_LANE_SELECT_MASK: u32 = (3 << 4);
pub const DP_FEC_LANE_0_SELECT: u32 = (0 << 4);
pub const DP_FEC_LANE_1_SELECT: u32 = (1 << 4);
pub const DP_FEC_LANE_2_SELECT: u32 = (2 << 4);
pub const DP_FEC_LANE_3_SELECT: u32 = (3 << 4);

pub const DP_SDP_ERROR_DETECTION_CONFIGURATION: u32 = 0x121	/* DP 2.0 E11 */;
pub const DP_SDP_CRC16_128B132B_EN: u32 = BIT(0);

pub const DP_AUX_FRAME_SYNC_VALUE: u32 = 0x15c   /* eDP 1.4 */;
pub const DP_AUX_FRAME_SYNC_VALID: u32 = (1 << 0);

pub const DP_DSC_ENABLE: u32 = 0x160   /* DP 1.4 */;
pub const DP_DECOMPRESSION_EN: u32 = (1 << 0);
pub const DP_DSC_PASSTHROUGH_EN: u32 = (1 << 1);
pub const DP_DSC_CONFIGURATION: u32 = 0x161	/* DP 2.0 */;

pub const DP_PSR_EN_CFG: u32 = 0x170   /* XXX 1.2? */;
pub const DP_PSR_ENABLE: u32 = BIT(0);
pub const DP_PSR_MAIN_LINK_ACTIVE: u32 = BIT(1);
pub const DP_PSR_CRC_VERIFICATION: u32 = BIT(2);
pub const DP_PSR_FRAME_CAPTURE: u32 = BIT(3);
pub const DP_PSR_SU_REGION_SCANLINE_CAPTURE: u32 = BIT(4) /* eDP 1.4a */;
pub const DP_PSR_IRQ_HPD_WITH_CRC_ERRORS: u32 = BIT(5) /* eDP 1.4a */;
pub const DP_PSR_ENABLE_PSR2: u32 = BIT(6) /* eDP 1.4a */;
pub const DP_PSR_ENABLE_SU_REGION_ET: u32 = BIT(7) /* eDP 1.5 */;

pub const DP_ADAPTER_CTRL: u32 = 0x1a0;
pub const DP_ADAPTER_CTRL_FORCE_LOAD_SENSE: u32 = (1 << 0);

pub const DP_BRANCH_DEVICE_CTRL: u32 = 0x1a1;
pub const DP_BRANCH_DEVICE_IRQ_HPD: u32 = (1 << 0);

pub const PANEL_REPLAY_CONFIG: u32 = 0x1b0  /* DP 2.0 */;
pub const DP_PANEL_REPLAY_ENABLE: u32 = (1 << 0);
pub const DP_PANEL_REPLAY_VSC_SDP_CRC_EN: u32 = (1 << 1) /* eDP 1.5 */;
pub const DP_PANEL_REPLAY_UNRECOVERABLE_ERROR_EN: u32 = (1 << 3);
pub const DP_PANEL_REPLAY_RFB_STORAGE_ERROR_EN: u32 = (1 << 4);
pub const DP_PANEL_REPLAY_ACTIVE_FRAME_CRC_ERROR_EN: u32 = (1 << 5);
pub const DP_PANEL_REPLAY_SU_ENABLE: u32 = (1 << 6);
pub const DP_PANEL_REPLAY_ENABLE_SU_REGION_ET: u32 = (1 << 7) /* DP 2.1 */;

pub const PANEL_REPLAY_CONFIG2: u32 = 0x1b1 /* eDP 1.5 */;
pub const DP_PANEL_REPLAY_SINK_REFRESH_RATE_UNLOCK_GRANTED: u32 = (1 << 0);
pub const DP_PANEL_REPLAY_CRC_VERIFICATION: u32 = (1 << 1);
pub const DP_PANEL_REPLAY_SU_Y_GRANULARITY_EXTENDED_EN: u32 = (1 << 2);
pub const DP_PANEL_REPLAY_SU_Y_GRANULARITY_EXTENDED_VAL_SEL_SHIFT: u32 = 3;
pub const DP_PANEL_REPLAY_SU_Y_GRANULARITY_EXTENDED_VAL_SEL_MASK: u32 = (0xf << 3);
pub const DP_PANEL_REPLAY_SU_REGION_SCANLINE_CAPTURE: u32 = (1 << 7);

pub const DP_PAYLOAD_ALLOCATE_SET: u32 = 0x1c0;
pub const DP_PAYLOAD_ALLOCATE_START_TIME_SLOT: u32 = 0x1c1;
pub const DP_PAYLOAD_ALLOCATE_TIME_SLOT_COUNT: u32 = 0x1c2;

/* Link/Sink Device Status */
pub const DP_SINK_COUNT: u32 = 0x200;
/* prior to 1.2 bit 7 was reserved mbz */
pub const DP_GET_SINK_COUNT: u32 = (x)		    ((((x) & 0x80) >> 1) | ((x) & 0x3f));
pub const DP_SINK_CP_READY: u32 = (1 << 6);

pub const DP_DEVICE_SERVICE_IRQ_VECTOR: u32 = 0x201;
pub const DP_REMOTE_CONTROL_COMMAND_PENDING: u32 = (1 << 0);
pub const DP_AUTOMATED_TEST_REQUEST: u32 = (1 << 1);
pub const DP_CP_IRQ: u32 = (1 << 2);
pub const DP_MCCS_IRQ: u32 = (1 << 3);
pub const DP_DOWN_REP_MSG_RDY: u32 = (1 << 4) /* 1.2 MST */;
pub const DP_UP_REQ_MSG_RDY: u32 = (1 << 5) /* 1.2 MST */;
pub const DP_SINK_SPECIFIC_IRQ: u32 = (1 << 6);

pub const DP_LANE0_1_STATUS: u32 = 0x202;
pub const DP_LANE2_3_STATUS: u32 = 0x203;
pub const DP_LANE_CR_DONE: u32 = (1 << 0);
pub const DP_LANE_CHANNEL_EQ_DONE: u32 = (1 << 1);
pub const DP_LANE_SYMBOL_LOCKED: u32 = (1 << 2);

pub const DP_CHANNEL_EQ_BITS: u32 = (DP_LANE_CR_DONE |		;
			    DP_LANE_CHANNEL_EQ_DONE |	\
			    DP_LANE_SYMBOL_LOCKED)

pub const DP_LANE_ALIGN_STATUS_UPDATED: u32 = 0x204;
pub const DP_INTERLANE_ALIGN_DONE: u32 = (1 << 0);
pub const DP_POST_LT_ADJ_REQ_IN_PROGRESS: u32 = (1 << 1) /* 1.3 */;
pub const DP_128B132B_DPRX_EQ_INTERLANE_ALIGN_DONE: u32 = (1 << 2) /* 2.0 E11 */;
pub const DP_128B132B_DPRX_CDS_INTERLANE_ALIGN_DONE: u32 = (1 << 3) /* 2.0 E11 */;
pub const DP_128B132B_LT_FAILED: u32 = (1 << 4) /* 2.0 E11 */;
pub const DP_DOWNSTREAM_PORT_STATUS_CHANGED: u32 = (1 << 6);
pub const DP_LINK_STATUS_UPDATED: u32 = (1 << 7);

pub const DP_SINK_STATUS: u32 = 0x205;
pub const DP_RECEIVE_PORT_0_STATUS: u32 = (1 << 0);
pub const DP_RECEIVE_PORT_1_STATUS: u32 = (1 << 1);
pub const DP_STREAM_REGENERATION_STATUS: u32 = (1 << 2) /* 2.0 */;
pub const DP_INTRA_HOP_AUX_REPLY_INDICATION: u32 = (1 << 3) /* 2.0 */;

pub const DP_ADJUST_REQUEST_LANE0_1: u32 = 0x206;
pub const DP_ADJUST_REQUEST_LANE2_3: u32 = 0x207;
pub const DP_ADJUST_VOLTAGE_SWING_LANE0_MASK: u32 = 0x03;
pub const DP_ADJUST_VOLTAGE_SWING_LANE0_SHIFT: u32 = 0;
pub const DP_ADJUST_PRE_EMPHASIS_LANE0_MASK: u32 = 0x0c;
pub const DP_ADJUST_PRE_EMPHASIS_LANE0_SHIFT: u32 = 2;
pub const DP_ADJUST_VOLTAGE_SWING_LANE1_MASK: u32 = 0x30;
pub const DP_ADJUST_VOLTAGE_SWING_LANE1_SHIFT: u32 = 4;
pub const DP_ADJUST_PRE_EMPHASIS_LANE1_MASK: u32 = 0xc0;
pub const DP_ADJUST_PRE_EMPHASIS_LANE1_SHIFT: u32 = 6;

/* DP 2.0 128b/132b Link Layer */
pub const DP_ADJUST_TX_FFE_PRESET_LANE0_MASK: u32 = (0xf << 0);
pub const DP_ADJUST_TX_FFE_PRESET_LANE0_SHIFT: u32 = 0;
pub const DP_ADJUST_TX_FFE_PRESET_LANE1_MASK: u32 = (0xf << 4);
pub const DP_ADJUST_TX_FFE_PRESET_LANE1_SHIFT: u32 = 4;

pub const DP_ADJUST_REQUEST_POST_CURSOR2: u32 = 0x20c;
pub const DP_ADJUST_POST_CURSOR2_LANE0_MASK: u32 = 0x03;
pub const DP_ADJUST_POST_CURSOR2_LANE0_SHIFT: u32 = 0;
pub const DP_ADJUST_POST_CURSOR2_LANE1_MASK: u32 = 0x0c;
pub const DP_ADJUST_POST_CURSOR2_LANE1_SHIFT: u32 = 2;
pub const DP_ADJUST_POST_CURSOR2_LANE2_MASK: u32 = 0x30;
pub const DP_ADJUST_POST_CURSOR2_LANE2_SHIFT: u32 = 4;
pub const DP_ADJUST_POST_CURSOR2_LANE3_MASK: u32 = 0xc0;
pub const DP_ADJUST_POST_CURSOR2_LANE3_SHIFT: u32 = 6;

pub const DP_TEST_REQUEST: u32 = 0x218;
pub const DP_TEST_LINK_TRAINING: u32 = (1 << 0);
pub const DP_TEST_LINK_VIDEO_PATTERN: u32 = (1 << 1);
pub const DP_TEST_LINK_EDID_READ: u32 = (1 << 2);
pub const DP_TEST_LINK_PHY_TEST_PATTERN: u32 = (1 << 3) /* DPCD >= 1.1 */;
pub const DP_TEST_LINK_FAUX_PATTERN: u32 = (1 << 4) /* DPCD >= 1.2 */;
pub const DP_TEST_LINK_AUDIO_PATTERN: u32 = (1 << 5) /* DPCD >= 1.2 */;
pub const DP_TEST_LINK_AUDIO_DISABLED_VIDEO: u32 = (1 << 6) /* DPCD >= 1.2 */;

pub const DP_TEST_LINK_RATE: u32 = 0x219;
pub const DP_LINK_RATE_162: u32 = (0x6);
pub const DP_LINK_RATE_27: u32 = (0xa);

pub const DP_TEST_LANE_COUNT: u32 = 0x220;

pub const DP_TEST_PATTERN: u32 = 0x221;
pub const DP_NO_TEST_PATTERN: u32 = 0x0;
pub const DP_COLOR_RAMP: u32 = 0x1;
pub const DP_BLACK_AND_WHITE_VERTICAL_LINES: u32 = 0x2;
pub const DP_COLOR_SQUARE: u32 = 0x3;

pub const DP_TEST_H_TOTAL_HI: u32 = 0x222;
pub const DP_TEST_H_TOTAL_LO: u32 = 0x223;

pub const DP_TEST_V_TOTAL_HI: u32 = 0x224;
pub const DP_TEST_V_TOTAL_LO: u32 = 0x225;

pub const DP_TEST_H_START_HI: u32 = 0x226;
pub const DP_TEST_H_START_LO: u32 = 0x227;

pub const DP_TEST_V_START_HI: u32 = 0x228;
pub const DP_TEST_V_START_LO: u32 = 0x229;

pub const DP_TEST_HSYNC_HI: u32 = 0x22A;
pub const DP_TEST_HSYNC_POLARITY: u32 = (1 << 7);
pub const DP_TEST_HSYNC_WIDTH_HI_MASK: u32 = (127 << 0);
pub const DP_TEST_HSYNC_WIDTH_LO: u32 = 0x22B;

pub const DP_TEST_VSYNC_HI: u32 = 0x22C;
pub const DP_TEST_VSYNC_POLARITY: u32 = (1 << 7);
pub const DP_TEST_VSYNC_WIDTH_HI_MASK: u32 = (127 << 0);
pub const DP_TEST_VSYNC_WIDTH_LO: u32 = 0x22D;

pub const DP_TEST_H_WIDTH_HI: u32 = 0x22E;
pub const DP_TEST_H_WIDTH_LO: u32 = 0x22F;

pub const DP_TEST_V_HEIGHT_HI: u32 = 0x230;
pub const DP_TEST_V_HEIGHT_LO: u32 = 0x231;

pub const DP_TEST_MISC0: u32 = 0x232;
pub const DP_TEST_SYNC_CLOCK: u32 = (1 << 0);
pub const DP_TEST_COLOR_FORMAT_MASK: u32 = (3 << 1);
pub const DP_TEST_COLOR_FORMAT_SHIFT: u32 = 1;
pub const DP_COLOR_FORMAT_RGB: u32 = (0 << 1);
pub const DP_COLOR_FORMAT_YCbCr422: u32 = (1 << 1);
pub const DP_COLOR_FORMAT_YCbCr444: u32 = (2 << 1);
pub const DP_TEST_DYNAMIC_RANGE_VESA: u32 = (0 << 3);
pub const DP_TEST_DYNAMIC_RANGE_CEA: u32 = (1 << 3);
pub const DP_TEST_YCBCR_COEFFICIENTS: u32 = (1 << 4);
pub const DP_YCBCR_COEFFICIENTS_ITU601: u32 = (0 << 4);
pub const DP_YCBCR_COEFFICIENTS_ITU709: u32 = (1 << 4);
pub const DP_TEST_BIT_DEPTH_MASK: u32 = (7 << 5);
pub const DP_TEST_BIT_DEPTH_SHIFT: u32 = 5;
pub const DP_TEST_BIT_DEPTH_6: u32 = (0 << 5);
pub const DP_TEST_BIT_DEPTH_8: u32 = (1 << 5);
pub const DP_TEST_BIT_DEPTH_10: u32 = (2 << 5);
pub const DP_TEST_BIT_DEPTH_12: u32 = (3 << 5);
pub const DP_TEST_BIT_DEPTH_16: u32 = (4 << 5);

pub const DP_TEST_MISC1: u32 = 0x233;
pub const DP_TEST_REFRESH_DENOMINATOR: u32 = (1 << 0);
pub const DP_TEST_INTERLACED: u32 = (1 << 1);

pub const DP_TEST_REFRESH_RATE_NUMERATOR: u32 = 0x234;

pub const DP_TEST_MISC0: u32 = 0x232;

pub const DP_TEST_CRC_R_CR: u32 = 0x240;
pub const DP_TEST_CRC_G_Y: u32 = 0x242;
pub const DP_TEST_CRC_B_CB: u32 = 0x244;

pub const DP_TEST_SINK_MISC: u32 = 0x246;
pub const DP_TEST_CRC_SUPPORTED: u32 = (1 << 5);
pub const DP_TEST_COUNT_MASK: u32 = 0xf;

pub const DP_PHY_TEST_PATTERN: u32 = 0x248;
pub const DP_PHY_TEST_PATTERN_SEL_MASK: u32 = 0x7;
pub const DP_PHY_TEST_PATTERN_NONE: u32 = 0x0;
pub const DP_PHY_TEST_PATTERN_D10_2: u32 = 0x1;
pub const DP_PHY_TEST_PATTERN_ERROR_COUNT: u32 = 0x2;
pub const DP_PHY_TEST_PATTERN_PRBS7: u32 = 0x3;
pub const DP_PHY_TEST_PATTERN_80BIT_CUSTOM: u32 = 0x4;
pub const DP_PHY_TEST_PATTERN_CP2520: u32 = 0x5;

pub const DP_PHY_SQUARE_PATTERN: u32 = 0x249;

pub const DP_TEST_HBR2_SCRAMBLER_RESET: u32 = 0x24A;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_7_0: u32 = 0x250;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_15_8: u32 = 0x251;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_23_16: u32 = 0x252;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_31_24: u32 = 0x253;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_39_32: u32 = 0x254;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_47_40: u32 = 0x255;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_55_48: u32 = 0x256;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_63_56: u32 = 0x257;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_71_64: u32 = 0x258;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_79_72: u32 = 0x259;

pub const DP_TEST_RESPONSE: u32 = 0x260;
pub const DP_TEST_ACK: u32 = (1 << 0);
pub const DP_TEST_NAK: u32 = (1 << 1);
pub const DP_TEST_EDID_CHECKSUM_WRITE: u32 = (1 << 2);

pub const DP_TEST_EDID_CHECKSUM: u32 = 0x261;

pub const DP_TEST_SINK: u32 = 0x270;
pub const DP_TEST_SINK_START: u32 = (1 << 0);
pub const DP_TEST_AUDIO_MODE: u32 = 0x271;
pub const DP_TEST_AUDIO_PATTERN_TYPE: u32 = 0x272;
pub const DP_TEST_AUDIO_PERIOD_CH1: u32 = 0x273;
pub const DP_TEST_AUDIO_PERIOD_CH2: u32 = 0x274;
pub const DP_TEST_AUDIO_PERIOD_CH3: u32 = 0x275;
pub const DP_TEST_AUDIO_PERIOD_CH4: u32 = 0x276;
pub const DP_TEST_AUDIO_PERIOD_CH5: u32 = 0x277;
pub const DP_TEST_AUDIO_PERIOD_CH6: u32 = 0x278;
pub const DP_TEST_AUDIO_PERIOD_CH7: u32 = 0x279;
pub const DP_TEST_AUDIO_PERIOD_CH8: u32 = 0x27A;

pub const DP_FEC_STATUS: u32 = 0x280    /* 1.4 */;
pub const DP_FEC_DECODE_EN_DETECTED: u32 = (1 << 0);
pub const DP_FEC_DECODE_DIS_DETECTED: u32 = (1 << 1);

pub const DP_FEC_ERROR_COUNT_LSB: u32 = 0x0281    /* 1.4 */;

pub const DP_FEC_ERROR_COUNT_MSB: u32 = 0x0282    /* 1.4 */;
pub const DP_FEC_ERROR_COUNT_MASK: u32 = 0x7F;
pub const DP_FEC_ERR_COUNT_VALID: u32 = (1 << 7);

pub const DP_PAYLOAD_TABLE_UPDATE_STATUS: u32 = 0x2c0   /* 1.2 MST */;
pub const DP_PAYLOAD_TABLE_UPDATED: u32 = (1 << 0);
pub const DP_PAYLOAD_ACT_HANDLED: u32 = (1 << 1);

pub const DP_VC_PAYLOAD_ID_SLOT_1: u32 = 0x2c1   /* 1.2 MST */;
/* up to ID_SLOT_63 at 0x2ff */

/* Source Device-specific */
pub const DP_SOURCE_OUI: u32 = 0x300;

/* Sink Device-specific */
pub const DP_SINK_OUI: u32 = 0x400;

/* Branch Device-specific */
pub const DP_BRANCH_OUI: u32 = 0x500;
pub const DP_BRANCH_ID: u32 = 0x503;
pub const DP_BRANCH_REVISION_START: u32 = 0x509;
pub const DP_BRANCH_HW_REV: u32 = 0x509;
pub const DP_BRANCH_SW_REV: u32 = 0x50A;

/* Link/Sink Device Power Control */
pub const DP_SET_POWER: u32 = 0x600;
pub const DP_SET_POWER_D0: u32 = 0x1;
pub const DP_SET_POWER_D3: u32 = 0x2;
pub const DP_SET_POWER_MASK: u32 = 0x3;
pub const DP_SET_POWER_D3_AUX_ON: u32 = 0x5;

/* eDP-specific */
pub const DP_EDP_DPCD_REV: u32 = 0x700    /* eDP 1.2 */;
pub const DP_EDP_11: u32 = 0x00;
pub const DP_EDP_12: u32 = 0x01;
pub const DP_EDP_13: u32 = 0x02;
pub const DP_EDP_14: u32 = 0x03;
pub const DP_EDP_14a: u32 = 0x04    /* eDP 1.4a */;
pub const DP_EDP_14b: u32 = 0x05    /* eDP 1.4b */;
pub const DP_EDP_15: u32 = 0x06    /* eDP 1.5 */;

pub const DP_EDP_GENERAL_CAP_1: u32 = 0x701;
pub const DP_EDP_TCON_BACKLIGHT_ADJUSTMENT_CAP: u32 = (1 << 0);
pub const DP_EDP_BACKLIGHT_PIN_ENABLE_CAP: u32 = (1 << 1);
pub const DP_EDP_BACKLIGHT_AUX_ENABLE_CAP: u32 = (1 << 2);
pub const DP_EDP_PANEL_SELF_TEST_PIN_ENABLE_CAP: u32 = (1 << 3);
pub const DP_EDP_PANEL_SELF_TEST_AUX_ENABLE_CAP: u32 = (1 << 4);
pub const DP_EDP_FRC_ENABLE_CAP: u32 = (1 << 5);
pub const DP_EDP_COLOR_ENGINE_CAP: u32 = (1 << 6);
pub const DP_EDP_SET_POWER_CAP: u32 = (1 << 7);

pub const DP_EDP_BACKLIGHT_ADJUSTMENT_CAP: u32 = 0x702;
pub const DP_EDP_BACKLIGHT_BRIGHTNESS_PWM_PIN_CAP: u32 = (1 << 0);
pub const DP_EDP_BACKLIGHT_BRIGHTNESS_AUX_SET_CAP: u32 = (1 << 1);
pub const DP_EDP_BACKLIGHT_BRIGHTNESS_BYTE_COUNT: u32 = (1 << 2);
pub const DP_EDP_BACKLIGHT_AUX_PWM_PRODUCT_CAP: u32 = (1 << 3);
pub const DP_EDP_BACKLIGHT_FREQ_PWM_PIN_PASSTHRU_CAP: u32 = (1 << 4);
pub const DP_EDP_BACKLIGHT_FREQ_AUX_SET_CAP: u32 = (1 << 5);
pub const DP_EDP_DYNAMIC_BACKLIGHT_CAP: u32 = (1 << 6);
pub const DP_EDP_VBLANK_BACKLIGHT_UPDATE_CAP: u32 = (1 << 7);

pub const DP_EDP_GENERAL_CAP_2: u32 = 0x703;
pub const DP_EDP_OVERDRIVE_ENGINE_ENABLED: u32 = (1 << 0);
pub const DP_EDP_PANEL_LUMINANCE_CONTROL_CAPABLE: u32 = (1 << 4);
pub const DP_EDP_SMOOTH_BRIGHTNESS_CAPABLE: u32 = (1 << 6) /* eDP 2.0 */;

pub const DP_EDP_GENERAL_CAP_3: u32 = 0x704    /* eDP 1.4 */;
pub const DP_EDP_X_REGION_CAP_MASK: u32 = (0xf << 0);
pub const DP_EDP_X_REGION_CAP_SHIFT: u32 = 0;
pub const DP_EDP_Y_REGION_CAP_MASK: u32 = (0xf << 4);
pub const DP_EDP_Y_REGION_CAP_SHIFT: u32 = 4;

pub const DP_EDP_DISPLAY_CONTROL_REGISTER: u32 = 0x720;
pub const DP_EDP_BACKLIGHT_ENABLE: u32 = (1 << 0);
pub const DP_EDP_BLACK_VIDEO_ENABLE: u32 = (1 << 1);
pub const DP_EDP_FRC_ENABLE: u32 = (1 << 2);
pub const DP_EDP_COLOR_ENGINE_ENABLE: u32 = (1 << 3);
pub const DP_EDP_VBLANK_BACKLIGHT_UPDATE_ENABLE: u32 = (1 << 7);

pub const DP_EDP_BACKLIGHT_MODE_SET_REGISTER: u32 = 0x721;
pub const DP_EDP_BACKLIGHT_CONTROL_MODE_MASK: u32 = (3 << 0);
pub const DP_EDP_BACKLIGHT_CONTROL_MODE_PWM: u32 = (0 << 0);
pub const DP_EDP_BACKLIGHT_CONTROL_MODE_PRESET: u32 = (1 << 0);
pub const DP_EDP_BACKLIGHT_CONTROL_MODE_DPCD: u32 = (2 << 0);
pub const DP_EDP_BACKLIGHT_CONTROL_MODE_PRODUCT: u32 = (3 << 0);
pub const DP_EDP_BACKLIGHT_FREQ_PWM_PIN_PASSTHRU_ENABLE: u32 = (1 << 2);
pub const DP_EDP_BACKLIGHT_FREQ_AUX_SET_ENABLE: u32 = (1 << 3);
pub const DP_EDP_DYNAMIC_BACKLIGHT_ENABLE: u32 = (1 << 4);
pub const DP_EDP_REGIONAL_BACKLIGHT_ENABLE: u32 = (1 << 5);
pub const DP_EDP_UPDATE_REGION_BRIGHTNESS: u32 = (1 << 6) /* eDP 1.4 */;
pub const DP_EDP_PANEL_LUMINANCE_CONTROL_ENABLE: u32 = (1 << 7);

pub const DP_EDP_BACKLIGHT_BRIGHTNESS_MSB: u32 = 0x722;
pub const DP_EDP_BACKLIGHT_BRIGHTNESS_LSB: u32 = 0x723;

pub const DP_EDP_PWMGEN_BIT_COUNT: u32 = 0x724;
pub const DP_EDP_PWMGEN_BIT_COUNT_CAP_MIN: u32 = 0x725;
pub const DP_EDP_PWMGEN_BIT_COUNT_CAP_MAX: u32 = 0x726;
pub const DP_EDP_PWMGEN_BIT_COUNT_MASK: u32 = (0x1f << 0);

pub const DP_EDP_BACKLIGHT_CONTROL_STATUS: u32 = 0x727;

pub const DP_EDP_BACKLIGHT_FREQ_SET: u32 = 0x728;
pub const DP_EDP_BACKLIGHT_FREQ_BASE_KHZ: u32 = 27000;

pub const DP_EDP_BACKLIGHT_FREQ_CAP_MIN_MSB: u32 = 0x72a;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MIN_MID: u32 = 0x72b;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MIN_LSB: u32 = 0x72c;

pub const DP_EDP_BACKLIGHT_FREQ_CAP_MAX_MSB: u32 = 0x72d;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MAX_MID: u32 = 0x72e;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MAX_LSB: u32 = 0x72f;

pub const DP_EDP_DBC_MINIMUM_BRIGHTNESS_SET: u32 = 0x732;
pub const DP_EDP_DBC_MAXIMUM_BRIGHTNESS_SET: u32 = 0x733;
pub const DP_EDP_PANEL_TARGET_LUMINANCE_VALUE: u32 = 0x734;

pub const DP_EDP_REGIONAL_BACKLIGHT_BASE: u32 = 0x740    /* eDP 1.4 */;
pub const DP_EDP_REGIONAL_BACKLIGHT_0: u32 = 0x741    /* eDP 1.4 */;

pub const DP_EDP_MSO_LINK_CAPABILITIES: u32 = 0x7a4    /* eDP 1.4 */;
pub const DP_EDP_MSO_NUMBER_OF_LINKS_MASK: u32 = (7 << 0);
pub const DP_EDP_MSO_NUMBER_OF_LINKS_SHIFT: u32 = 0;
pub const DP_EDP_MSO_INDEPENDENT_LINK_BIT: u32 = (1 << 3);

/* Sideband MSG Buffers */
pub const DP_SIDEBAND_MSG_DOWN_REQ_BASE: u32 = 0x1000   /* 1.2 MST */;
pub const DP_SIDEBAND_MSG_UP_REP_BASE: u32 = 0x1200   /* 1.2 MST */;
pub const DP_SIDEBAND_MSG_DOWN_REP_BASE: u32 = 0x1400   /* 1.2 MST */;
pub const DP_SIDEBAND_MSG_UP_REQ_BASE: u32 = 0x1600   /* 1.2 MST */;

/* DPRX Event Status Indicator */
pub const DP_SINK_COUNT_ESI: u32 = 0x2002   /* same as 0x200 */;
pub const DP_DEVICE_SERVICE_IRQ_VECTOR_ESI0: u32 = 0x2003   /* same as 0x201 */;

pub const DP_DEVICE_SERVICE_IRQ_VECTOR_ESI1: u32 = 0x2004   /* 1.2 */;
pub const DP_RX_GTC_MSTR_REQ_STATUS_CHANGE: u32 = (1 << 0);
pub const DP_LOCK_ACQUISITION_REQUEST: u32 = (1 << 1);
pub const DP_CEC_IRQ: u32 = (1 << 2);

pub const DP_LINK_SERVICE_IRQ_VECTOR_ESI0: u32 = 0x2005   /* 1.2 */;
pub const RX_CAP_CHANGED: u32 = (1 << 0);
pub const LINK_STATUS_CHANGED: u32 = (1 << 1);
pub const STREAM_STATUS_CHANGED: u32 = (1 << 2);
pub const HDMI_LINK_STATUS_CHANGED: u32 = (1 << 3);
pub const CONNECTED_OFF_ENTRY_REQUESTED: u32 = (1 << 4);
pub const DP_TUNNELING_IRQ: u32 = (1 << 5);

pub const DP_PSR_ERROR_STATUS: u32 = 0x2006  /* XXX 1.2? */;
pub const DP_PSR_LINK_CRC_ERROR: u32 = (1 << 0);
pub const DP_PSR_RFB_STORAGE_ERROR: u32 = (1 << 1);
pub const DP_PSR_VSC_SDP_UNCORRECTABLE_ERROR: u32 = (1 << 2) /* eDP 1.4 */;

pub const DP_PSR_ESI: u32 = 0x2007  /* XXX 1.2? */;
pub const DP_PSR_CAPS_CHANGE: u32 = (1 << 0);

pub const DP_PSR_STATUS: u32 = 0x2008  /* XXX 1.2? */;
pub const DP_PSR_SINK_INACTIVE: u32 = 0;
pub const DP_PSR_SINK_ACTIVE_SRC_SYNCED: u32 = 1;
pub const DP_PSR_SINK_ACTIVE_RFB: u32 = 2;
pub const DP_PSR_SINK_ACTIVE_SINK_SYNCED: u32 = 3;
pub const DP_PSR_SINK_ACTIVE_RESYNC: u32 = 4;
pub const DP_PSR_SINK_INTERNAL_ERROR: u32 = 7;
pub const DP_PSR_SINK_STATE_MASK: u32 = 0x07;

pub const DP_SYNCHRONIZATION_LATENCY_IN_SINK: u32 = 0x2009 /* edp 1.4 */;
pub const DP_MAX_RESYNC_FRAME_COUNT_MASK: u32 = (0xf << 0);
pub const DP_MAX_RESYNC_FRAME_COUNT_SHIFT: u32 = 0;
pub const DP_LAST_ACTUAL_SYNCHRONIZATION_LATENCY_MASK: u32 = (0xf << 4);
pub const DP_LAST_ACTUAL_SYNCHRONIZATION_LATENCY_SHIFT: u32 = 4;

pub const DP_LAST_RECEIVED_PSR_SDP: u32 = 0x200a /* eDP 1.2 */;
pub const DP_PSR_STATE_BIT: u32 = (1 << 0) /* eDP 1.2 */;
pub const DP_UPDATE_RFB_BIT: u32 = (1 << 1) /* eDP 1.2 */;
pub const DP_CRC_VALID_BIT: u32 = (1 << 2) /* eDP 1.2 */;
pub const DP_SU_VALID: u32 = (1 << 3) /* eDP 1.4 */;
pub const DP_FIRST_SCAN_LINE_SU_REGION: u32 = (1 << 4) /* eDP 1.4 */;
pub const DP_LAST_SCAN_LINE_SU_REGION: u32 = (1 << 5) /* eDP 1.4 */;
pub const DP_Y_COORDINATE_VALID: u32 = (1 << 6) /* eDP 1.4a */;

pub const DP_RECEIVER_ALPM_STATUS: u32 = 0x200b  /* eDP 1.4 */;
pub const DP_ALPM_LOCK_TIMEOUT_ERROR: u32 = (1 << 0);

pub const DP_LANE0_1_STATUS_ESI: u32 = 0x200c /* status same as 0x202 */;
pub const DP_LANE2_3_STATUS_ESI: u32 = 0x200d /* status same as 0x203 */;
pub const DP_LANE_ALIGN_STATUS_UPDATED_ESI: u32 = 0x200e /* status same as 0x204 */;
pub const DP_SINK_STATUS_ESI: u32 = 0x200f /* status same as 0x205 */;

pub const DP_PANEL_REPLAY_ERROR_STATUS: u32 = 0x2020  /* DP 2.1*/;
pub const DP_PANEL_REPLAY_LINK_CRC_ERROR: u32 = (1 << 0);
pub const DP_PANEL_REPLAY_RFB_STORAGE_ERROR: u32 = (1 << 1);
pub const DP_PANEL_REPLAY_VSC_SDP_UNCORRECTABLE_ERROR: u32 = (1 << 2);

pub const DP_SINK_DEVICE_PR_AND_FRAME_LOCK_STATUS: u32 = 0x2022  /* DP 2.1 */;
pub const DP_SINK_DEVICE_PANEL_REPLAY_STATUS_MASK: u32 = (7 << 0);
pub const DP_SINK_FRAME_LOCKED_SHIFT: u32 = 3;
pub const DP_SINK_FRAME_LOCKED_MASK: u32 = (3 << 3);
pub const DP_SINK_FRAME_LOCKED_STATUS_VALID_SHIFT: u32 = 5;
pub const DP_SINK_FRAME_LOCKED_STATUS_VALID_MASK: u32 = (1 << 5);

/* Extended Receiver Capability: See DP_DPCD_REV for definitions */
pub const DP_DP13_DPCD_REV: u32 = 0x2200;

pub const DP_DPRX_FEATURE_ENUMERATION_LIST: u32 = 0x2210  /* DP 1.3 */;
pub const DP_GTC_CAP: u32 = (1 << 0)  /* DP 1.3 */;
pub const DP_SST_SPLIT_SDP_CAP: u32 = (1 << 1)  /* DP 1.4 */;
pub const DP_AV_SYNC_CAP: u32 = (1 << 2)  /* DP 1.3 */;
pub const DP_VSC_SDP_EXT_FOR_COLORIMETRY_SUPPORTED: u32 = (1 << 3)  /* DP 1.3 */;
pub const DP_VSC_EXT_VESA_SDP_SUPPORTED: u32 = (1 << 4)  /* DP 1.4 */;
pub const DP_VSC_EXT_VESA_SDP_CHAINING_SUPPORTED: u32 = (1 << 5)  /* DP 1.4 */;
pub const DP_VSC_EXT_CEA_SDP_SUPPORTED: u32 = (1 << 6)  /* DP 1.4 */;
pub const DP_VSC_EXT_CEA_SDP_CHAINING_SUPPORTED: u32 = (1 << 7)  /* DP 1.4 */;

pub const DP_EXTENDED_DPRX_SLEEP_WAKE_TIMEOUT_REQUEST: u32 = 0x2211  /* 1.4a */;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_MASK: u32 = 0xff;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_1_MS: u32 = 0x00;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_20_MS: u32 = 0x01;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_40_MS: u32 = 0x02;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_60_MS: u32 = 0x03;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_80_MS: u32 = 0x04;
pub const DP_DPRX_SLEEP_WAKE_TIMEOUT_PERIOD_100_MS: u32 = 0x05;

pub const DP_DPRX_FEATURE_ENUMERATION_LIST_CONT_1: u32 = 0x2214 /* 2.0 E11 */;
pub const DP_ADAPTIVE_SYNC_SDP_SUPPORTED: u32 = BIT(0);
pub const DP_AS_SDP_FIRST_HALF_LINE_OR_3840_PIXEL_CYCLE_WINDOW_NOT_SUPPORTED: u32 = BIT(1);
pub const DP_AS_SDP_FAVT_PAYLOAD_FIELDS_PARSING_SUPPORTED: u32 = BIT(2) /* 2.1 */;
pub const DP_VSC_EXT_SDP_FRAMEWORK_VERSION_1_SUPPORTED: u32 = BIT(4);

pub const DP_128B132B_SUPPORTED_LINK_RATES: u32 = 0x2215 /* 2.0 */;
pub const DP_UHBR10: u32 = (1 << 0);
pub const DP_UHBR20: u32 = (1 << 1);
pub const DP_UHBR13_5: u32 = (1 << 2);

pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL: u32 = 0x2216 /* 2.0 */;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_1MS_UNIT: u32 = (1 << 7);
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_MASK: u32 = 0x7f;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_400_US: u32 = 0x00;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_4_MS: u32 = 0x01;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_8_MS: u32 = 0x02;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_12_MS: u32 = 0x03;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_16_MS: u32 = 0x04;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_32_MS: u32 = 0x05;
pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_64_MS: u32 = 0x06;

pub const DP_TEST_264BIT_CUSTOM_PATTERN_7_0: u32 = 0x2230;
pub const DP_TEST_264BIT_CUSTOM_PATTERN_263_256: u32 = 0x2250;

/* DSC Extended Capability Branch Total DSC Resources */
pub const DP_DSC_SUPPORT_AND_DSC_DECODER_COUNT: u32 = 0x2260	/* 2.0 */;
pub const DP_DSC_DECODER_COUNT_MASK: u32 = (0b111 << 5);
pub const DP_DSC_DECODER_COUNT_SHIFT: u32 = 5;
pub const DP_DSC_MAX_SLICE_COUNT_AND_AGGREGATION_0: u32 = 0x2270	/* 2.0 */;
pub const DP_DSC_DECODER_0_MAXIMUM_SLICE_COUNT_MASK: u32 = (1 << 0);
pub const DP_DSC_DECODER_0_AGGREGATION_SUPPORT_MASK: u32 = (0b111 << 1);
pub const DP_DSC_DECODER_0_AGGREGATION_SUPPORT_SHIFT: u32 = 1;

/* Protocol Converter Extension */
/* HDMI CEC tunneling over AUX DP 1.3 section 5.3.3.3.1 DPCD 1.4+ */
pub const DP_CEC_TUNNELING_CAPABILITY: u32 = 0x3000;
pub const DP_CEC_TUNNELING_CAPABLE: u32 = (1 << 0);
pub const DP_CEC_SNOOPING_CAPABLE: u32 = (1 << 1);
pub const DP_CEC_MULTIPLE_LA_CAPABLE: u32 = (1 << 2);

pub const DP_CEC_TUNNELING_CONTROL: u32 = 0x3001;
pub const DP_CEC_TUNNELING_ENABLE: u32 = (1 << 0);
pub const DP_CEC_SNOOPING_ENABLE: u32 = (1 << 1);

pub const DP_CEC_RX_MESSAGE_INFO: u32 = 0x3002;
pub const DP_CEC_RX_MESSAGE_LEN_MASK: u32 = (0xf << 0);
pub const DP_CEC_RX_MESSAGE_LEN_SHIFT: u32 = 0;
pub const DP_CEC_RX_MESSAGE_HPD_STATE: u32 = (1 << 4);
pub const DP_CEC_RX_MESSAGE_HPD_LOST: u32 = (1 << 5);
pub const DP_CEC_RX_MESSAGE_ACKED: u32 = (1 << 6);
pub const DP_CEC_RX_MESSAGE_ENDED: u32 = (1 << 7);

pub const DP_CEC_TX_MESSAGE_INFO: u32 = 0x3003;
pub const DP_CEC_TX_MESSAGE_LEN_MASK: u32 = (0xf << 0);
pub const DP_CEC_TX_MESSAGE_LEN_SHIFT: u32 = 0;
pub const DP_CEC_TX_RETRY_COUNT_MASK: u32 = (0x7 << 4);
pub const DP_CEC_TX_RETRY_COUNT_SHIFT: u32 = 4;
pub const DP_CEC_TX_MESSAGE_SEND: u32 = (1 << 7);

pub const DP_CEC_TUNNELING_IRQ_FLAGS: u32 = 0x3004;
pub const DP_CEC_RX_MESSAGE_INFO_VALID: u32 = (1 << 0);
pub const DP_CEC_RX_MESSAGE_OVERFLOW: u32 = (1 << 1);
pub const DP_CEC_TX_MESSAGE_SENT: u32 = (1 << 4);
pub const DP_CEC_TX_LINE_ERROR: u32 = (1 << 5);
pub const DP_CEC_TX_ADDRESS_NACK_ERROR: u32 = (1 << 6);
pub const DP_CEC_TX_DATA_NACK_ERROR: u32 = (1 << 7);

pub const DP_CEC_LOGICAL_ADDRESS_MASK: u32 = 0x300E /* 0x300F word */;
pub const DP_CEC_LOGICAL_ADDRESS_0: u32 = (1 << 0);
pub const DP_CEC_LOGICAL_ADDRESS_1: u32 = (1 << 1);
pub const DP_CEC_LOGICAL_ADDRESS_2: u32 = (1 << 2);
pub const DP_CEC_LOGICAL_ADDRESS_3: u32 = (1 << 3);
pub const DP_CEC_LOGICAL_ADDRESS_4: u32 = (1 << 4);
pub const DP_CEC_LOGICAL_ADDRESS_5: u32 = (1 << 5);
pub const DP_CEC_LOGICAL_ADDRESS_6: u32 = (1 << 6);
pub const DP_CEC_LOGICAL_ADDRESS_7: u32 = (1 << 7);
pub const DP_CEC_LOGICAL_ADDRESS_MASK_2: u32 = 0x300F /* 0x300E word */;
pub const DP_CEC_LOGICAL_ADDRESS_8: u32 = (1 << 0);
pub const DP_CEC_LOGICAL_ADDRESS_9: u32 = (1 << 1);
pub const DP_CEC_LOGICAL_ADDRESS_10: u32 = (1 << 2);
pub const DP_CEC_LOGICAL_ADDRESS_11: u32 = (1 << 3);
pub const DP_CEC_LOGICAL_ADDRESS_12: u32 = (1 << 4);
pub const DP_CEC_LOGICAL_ADDRESS_13: u32 = (1 << 5);
pub const DP_CEC_LOGICAL_ADDRESS_14: u32 = (1 << 6);
pub const DP_CEC_LOGICAL_ADDRESS_15: u32 = (1 << 7);

pub const DP_CEC_RX_MESSAGE_BUFFER: u32 = 0x3010;
pub const DP_CEC_TX_MESSAGE_BUFFER: u32 = 0x3020;
pub const DP_CEC_MESSAGE_BUFFER_LENGTH: u32 = 0x10;

/* PCON CONFIGURE-1 FRL FOR HDMI SINK */
pub const DP_PCON_HDMI_LINK_CONFIG_1: u32 = 0x305A;
pub const DP_PCON_ENABLE_MAX_FRL_BW: u32 = (7 << 0);
pub const DP_PCON_ENABLE_MAX_BW_0GBPS: u32 = 0;
pub const DP_PCON_ENABLE_MAX_BW_9GBPS: u32 = 1;
pub const DP_PCON_ENABLE_MAX_BW_18GBPS: u32 = 2;
pub const DP_PCON_ENABLE_MAX_BW_24GBPS: u32 = 3;
pub const DP_PCON_ENABLE_MAX_BW_32GBPS: u32 = 4;
pub const DP_PCON_ENABLE_MAX_BW_40GBPS: u32 = 5;
pub const DP_PCON_ENABLE_MAX_BW_48GBPS: u32 = 6;
pub const DP_PCON_ENABLE_SOURCE_CTL_MODE: u32 = (1 << 3);
pub const DP_PCON_ENABLE_CONCURRENT_LINK: u32 = (1 << 4);
pub const DP_PCON_ENABLE_SEQUENTIAL_LINK: u32 = (0 << 4);
pub const DP_PCON_ENABLE_LINK_FRL_MODE: u32 = (1 << 5);
pub const DP_PCON_ENABLE_HPD_READY: u32 = (1 << 6);
pub const DP_PCON_ENABLE_HDMI_LINK: u32 = (1 << 7);

/* PCON CONFIGURE-2 FRL FOR HDMI SINK */
pub const DP_PCON_HDMI_LINK_CONFIG_2: u32 = 0x305B;
pub const DP_PCON_MAX_LINK_BW_MASK: u32 = (0x3F << 0);
pub const DP_PCON_FRL_BW_MASK_9GBPS: u32 = (1 << 0);
pub const DP_PCON_FRL_BW_MASK_18GBPS: u32 = (1 << 1);
pub const DP_PCON_FRL_BW_MASK_24GBPS: u32 = (1 << 2);
pub const DP_PCON_FRL_BW_MASK_32GBPS: u32 = (1 << 3);
pub const DP_PCON_FRL_BW_MASK_40GBPS: u32 = (1 << 4);
pub const DP_PCON_FRL_BW_MASK_48GBPS: u32 = (1 << 5);
pub const DP_PCON_FRL_LINK_TRAIN_EXTENDED: u32 = (1 << 6);
pub const DP_PCON_FRL_LINK_TRAIN_NORMAL: u32 = (0 << 6);

/* PCON HDMI LINK STATUS */
pub const DP_PCON_HDMI_TX_LINK_STATUS: u32 = 0x303B;
pub const DP_PCON_HDMI_TX_LINK_ACTIVE: u32 = (1 << 0);
pub const DP_PCON_FRL_READY: u32 = (1 << 1);

/* PCON HDMI POST FRL STATUS */
pub const DP_PCON_HDMI_POST_FRL_STATUS: u32 = 0x3036;
pub const DP_PCON_HDMI_LINK_MODE: u32 = (1 << 0);
pub const DP_PCON_HDMI_MODE_TMDS: u32 = 0;
pub const DP_PCON_HDMI_MODE_FRL: u32 = 1;
pub const DP_PCON_HDMI_FRL_TRAINED_BW: u32 = (0x3F << 1);
pub const DP_PCON_FRL_TRAINED_BW_9GBPS: u32 = (1 << 1);
pub const DP_PCON_FRL_TRAINED_BW_18GBPS: u32 = (1 << 2);
pub const DP_PCON_FRL_TRAINED_BW_24GBPS: u32 = (1 << 3);
pub const DP_PCON_FRL_TRAINED_BW_32GBPS: u32 = (1 << 4);
pub const DP_PCON_FRL_TRAINED_BW_40GBPS: u32 = (1 << 5);
pub const DP_PCON_FRL_TRAINED_BW_48GBPS: u32 = (1 << 6);

pub const DP_PROTOCOL_CONVERTER_CONTROL_0: u32 = 0x3050 /* DP 1.3 */;
pub const DP_HDMI_DVI_OUTPUT_CONFIG: u32 = (1 << 0) /* DP 1.3 */;
pub const DP_PROTOCOL_CONVERTER_CONTROL_1: u32 = 0x3051 /* DP 1.3 */;
pub const DP_CONVERSION_TO_YCBCR420_ENABLE: u32 = (1 << 0) /* DP 1.3 */;
pub const DP_HDMI_EDID_PROCESSING_DISABLE: u32 = (1 << 1) /* DP 1.4 */;
pub const DP_HDMI_AUTONOMOUS_SCRAMBLING_DISABLE: u32 = (1 << 2) /* DP 1.4 */;
pub const DP_HDMI_FORCE_SCRAMBLING: u32 = (1 << 3) /* DP 1.4 */;
pub const DP_PROTOCOL_CONVERTER_CONTROL_2: u32 = 0x3052 /* DP 1.3 */;
pub const DP_CONVERSION_TO_YCBCR422_ENABLE: u32 = (1 << 0) /* DP 1.3 */;
pub const DP_PCON_ENABLE_DSC_ENCODER: u32 = (1 << 1);
pub const DP_PCON_ENCODER_PPS_OVERRIDE_MASK: u32 = (0x3 << 2);
pub const DP_PCON_ENC_PPS_OVERRIDE_DISABLED: u32 = 0;
pub const DP_PCON_ENC_PPS_OVERRIDE_EN_PARAMS: u32 = 1;
pub const DP_PCON_ENC_PPS_OVERRIDE_EN_BUFFER: u32 = 2;
pub const DP_CONVERSION_RGB_YCBCR_MASK: u32 = (7 << 4);
pub const DP_CONVERSION_BT601_RGB_YCBCR_ENABLE: u32 = (1 << 4);
pub const DP_CONVERSION_BT709_RGB_YCBCR_ENABLE: u32 = (1 << 5);
pub const DP_CONVERSION_BT2020_RGB_YCBCR_ENABLE: u32 = (1 << 6);

/* PCON Downstream HDMI ERROR Status per Lane */
pub const DP_PCON_HDMI_ERROR_STATUS_LN0: u32 = 0x3037;
pub const DP_PCON_HDMI_ERROR_STATUS_LN1: u32 = 0x3038;
pub const DP_PCON_HDMI_ERROR_STATUS_LN2: u32 = 0x3039;
pub const DP_PCON_HDMI_ERROR_STATUS_LN3: u32 = 0x303A;
pub const DP_PCON_HDMI_ERROR_COUNT_MASK: u32 = (0x7 << 0);
pub const DP_PCON_HDMI_ERROR_COUNT_THREE_PLUS: u32 = (1 << 0);
pub const DP_PCON_HDMI_ERROR_COUNT_TEN_PLUS: u32 = (1 << 1);
pub const DP_PCON_HDMI_ERROR_COUNT_HUNDRED_PLUS: u32 = (1 << 2);

/* PCON HDMI CONFIG PPS Override Buffer
 * Valid Offsets to be added to Base : 0-127
 */
pub const DP_PCON_HDMI_PPS_OVERRIDE_BASE: u32 = 0x3100;

/* PCON HDMI CONFIG PPS Override Parameter: Slice height
 * Offset-0 8LSBs of the Slice height.
 * Offset-1 8MSBs of the Slice height.
 */
pub const DP_PCON_HDMI_PPS_OVRD_SLICE_HEIGHT: u32 = 0x3180;

/* PCON HDMI CONFIG PPS Override Parameter: Slice width
 * Offset-0 8LSBs of the Slice width.
 * Offset-1 8MSBs of the Slice width.
 */
pub const DP_PCON_HDMI_PPS_OVRD_SLICE_WIDTH: u32 = 0x3182;

/* PCON HDMI CONFIG PPS Override Parameter: bits_per_pixel
 * Offset-0 8LSBs of the bits_per_pixel.
 * Offset-1 2MSBs of the bits_per_pixel.
 */
pub const DP_PCON_HDMI_PPS_OVRD_BPP: u32 = 0x3184;

/* HDCP 1.3 and HDCP 2.2 */
pub const DP_AUX_HDCP_BKSV: u32 = 0x68000;
pub const DP_AUX_HDCP_RI_PRIME: u32 = 0x68005;
pub const DP_AUX_HDCP_AKSV: u32 = 0x68007;
pub const DP_AUX_HDCP_AN: u32 = 0x6800C;
// C function-like macro preserved: #define DP_AUX_HDCP_V_PRIME(h)		(0x68014 + h * 4)
pub const DP_AUX_HDCP_BCAPS: u32 = 0x68028;
pub const DP_BCAPS_REPEATER_PRESENT: u32 = BIT(1);
pub const DP_BCAPS_HDCP_CAPABLE: u32 = BIT(0);
pub const DP_AUX_HDCP_BSTATUS: u32 = 0x68029;
pub const DP_BSTATUS_REAUTH_REQ: u32 = BIT(3);
pub const DP_BSTATUS_LINK_FAILURE: u32 = BIT(2);
pub const DP_BSTATUS_R0_PRIME_READY: u32 = BIT(1);
pub const DP_BSTATUS_READY: u32 = BIT(0);
pub const DP_AUX_HDCP_BINFO: u32 = 0x6802A;
pub const DP_AUX_HDCP_KSV_FIFO: u32 = 0x6802C;
pub const DP_AUX_HDCP_AINFO: u32 = 0x6803B;

/* DP HDCP2.2 parameter offsets in DPCD address space */
pub const DP_HDCP_2_2_REG_RTX_OFFSET: u32 = 0x69000;
pub const DP_HDCP_2_2_REG_TXCAPS_OFFSET: u32 = 0x69008;
pub const DP_HDCP_2_2_REG_CERT_RX_OFFSET: u32 = 0x6900B;
pub const DP_HDCP_2_2_REG_RRX_OFFSET: u32 = 0x69215;
pub const DP_HDCP_2_2_REG_RX_CAPS_OFFSET: u32 = 0x6921D;
pub const DP_HDCP_2_2_REG_EKPUB_KM_OFFSET: u32 = 0x69220;
pub const DP_HDCP_2_2_REG_EKH_KM_WR_OFFSET: u32 = 0x692A0;
pub const DP_HDCP_2_2_REG_M_OFFSET: u32 = 0x692B0;
pub const DP_HDCP_2_2_REG_HPRIME_OFFSET: u32 = 0x692C0;
pub const DP_HDCP_2_2_REG_EKH_KM_RD_OFFSET: u32 = 0x692E0;
pub const DP_HDCP_2_2_REG_RN_OFFSET: u32 = 0x692F0;
pub const DP_HDCP_2_2_REG_LPRIME_OFFSET: u32 = 0x692F8;
pub const DP_HDCP_2_2_REG_EDKEY_KS_OFFSET: u32 = 0x69318;
pub const DP_HDCP_2_2_REG_RIV_OFFSET: u32 = 0x69328;
pub const DP_HDCP_2_2_REG_RXINFO_OFFSET: u32 = 0x69330;
pub const DP_HDCP_2_2_REG_SEQ_NUM_V_OFFSET: u32 = 0x69332;
pub const DP_HDCP_2_2_REG_VPRIME_OFFSET: u32 = 0x69335;
pub const DP_HDCP_2_2_REG_RECV_ID_LIST_OFFSET: u32 = 0x69345;
pub const DP_HDCP_2_2_REG_V_OFFSET: u32 = 0x693E0;
pub const DP_HDCP_2_2_REG_SEQ_NUM_M_OFFSET: u32 = 0x693F0;
pub const DP_HDCP_2_2_REG_K_OFFSET: u32 = 0x693F3;
pub const DP_HDCP_2_2_REG_STREAM_ID_TYPE_OFFSET: u32 = 0x693F5;
pub const DP_HDCP_2_2_REG_MPRIME_OFFSET: u32 = 0x69473;
pub const DP_HDCP_2_2_REG_RXSTATUS_OFFSET: u32 = 0x69493;
pub const DP_HDCP_2_2_REG_STREAM_TYPE_OFFSET: u32 = 0x69494;
pub const DP_HDCP_2_2_REG_DBG_OFFSET: u32 = 0x69518;

/* DP-tunneling */
pub const DP_TUNNELING_OUI: u32 = 0xe0000;
pub const DP_TUNNELING_OUI_BYTES: u32 = 3;

pub const DP_TUNNELING_DEV_ID: u32 = 0xe0003;
pub const DP_TUNNELING_DEV_ID_BYTES: u32 = 6;

pub const DP_TUNNELING_HW_REV: u32 = 0xe0009;
pub const DP_TUNNELING_HW_REV_MAJOR_SHIFT: u32 = 4;
pub const DP_TUNNELING_HW_REV_MAJOR_MASK: u32 = (0xf << DP_TUNNELING_HW_REV_MAJOR_SHIFT);
pub const DP_TUNNELING_HW_REV_MINOR_SHIFT: u32 = 0;
pub const DP_TUNNELING_HW_REV_MINOR_MASK: u32 = (0xf << DP_TUNNELING_HW_REV_MINOR_SHIFT);

pub const DP_TUNNELING_SW_REV_MAJOR: u32 = 0xe000a;
pub const DP_TUNNELING_SW_REV_MINOR: u32 = 0xe000b;

pub const DP_TUNNELING_CAPABILITIES: u32 = 0xe000d;
pub const DP_IN_BW_ALLOCATION_MODE_SUPPORT: u32 = (1 << 7);
pub const DP_PANEL_REPLAY_OPTIMIZATION_SUPPORT: u32 = (1 << 6);
pub const DP_TUNNELING_SUPPORT: u32 = (1 << 0);

pub const DP_IN_ADAPTER_INFO: u32 = 0xe000e;
pub const DP_IN_ADAPTER_NUMBER_BITS: u32 = 7;
pub const DP_IN_ADAPTER_NUMBER_MASK: u32 = ((1 << DP_IN_ADAPTER_NUMBER_BITS) - 1);

pub const DP_USB4_DRIVER_ID: u32 = 0xe000f;
pub const DP_USB4_DRIVER_ID_BITS: u32 = 4;
pub const DP_USB4_DRIVER_ID_MASK: u32 = ((1 << DP_USB4_DRIVER_ID_BITS) - 1);

pub const DP_USB4_DRIVER_BW_CAPABILITY: u32 = 0xe0020;
pub const DP_USB4_DRIVER_BW_ALLOCATION_MODE_SUPPORT: u32 = (1 << 7);

pub const DP_IN_ADAPTER_TUNNEL_INFORMATION: u32 = 0xe0021;
pub const DP_GROUP_ID_BITS: u32 = 3;
pub const DP_GROUP_ID_MASK: u32 = ((1 << DP_GROUP_ID_BITS) - 1);

pub const DP_BW_GRANULARITY: u32 = 0xe0022;
pub const DP_BW_GRANULARITY_MASK: u32 = 0x3;

pub const DP_ESTIMATED_BW: u32 = 0xe0023;
pub const DP_ALLOCATED_BW: u32 = 0xe0024;

pub const DP_TUNNELING_STATUS: u32 = 0xe0025;
pub const DP_BW_ALLOCATION_CAPABILITY_CHANGED: u32 = (1 << 3);
pub const DP_ESTIMATED_BW_CHANGED: u32 = (1 << 2);
pub const DP_BW_REQUEST_SUCCEEDED: u32 = (1 << 1);
pub const DP_BW_REQUEST_FAILED: u32 = (1 << 0);

pub const DP_TUNNELING_MAX_LINK_RATE: u32 = 0xe0028;

pub const DP_TUNNELING_MAX_LANE_COUNT: u32 = 0xe0029;
pub const DP_TUNNELING_MAX_LANE_COUNT_MASK: u32 = 0x1f;

pub const DP_TUNNELING_MAIN_LINK_CHANNEL_CODING: u32 = 0xe002b;
pub const DP_128B132B_DP_SUPPORTED: u32 = (1 << 0);

pub const DP_TUNNELING_128B132B_LINK_RATE: u32 = 0xe002c;
pub const DP_TUNNELING_13_5GBPS_PER_LANE_SUPPORT: u32 = (1 << 2);
pub const DP_TUNNELING_20GBPS_PER_LANE_SUPPORT: u32 = (1 << 1);
pub const DP_TUNNELING_10GBPS_PER_LANE_SUPPORT: u32 = (1 << 0);
pub const DP_TUNNELING_128B132B_LINK_RATE_MASK: u32 = (DP_TUNNELING_10GBPS_PER_LANE_SUPPORT | ;
							 DP_TUNNELING_13_5GBPS_PER_LANE_SUPPORT | \
							 DP_TUNNELING_20GBPS_PER_LANE_SUPPORT)
pub const DP_TUNNELING_128B132B_LL_LANE0_MAPPING_SUPPORT: u32 = (1 << 7);

pub const DP_DPTX_BW_ALLOCATION_MODE_CONTROL: u32 = 0xe0030;
pub const DP_DISPLAY_DRIVER_BW_ALLOCATION_MODE_ENABLE: u32 = (1 << 7);
pub const DP_UNMASK_BW_ALLOCATION_IRQ: u32 = (1 << 6);

pub const DP_REQUEST_BW: u32 = 0xe0031;
pub const MAX_DP_REQUEST_BW: u32 = 255;

/* LTTPR: Link Training (LT)-tunable PHY Repeaters */
pub const DP_LT_TUNABLE_PHY_REPEATER_FIELD_DATA_STRUCTURE_REV: u32 = 0xf0000 /* 1.3 */;
pub const DP_MAX_LINK_RATE_PHY_REPEATER: u32 = 0xf0001 /* 1.4a */;
pub const DP_PHY_REPEATER_CNT: u32 = 0xf0002 /* 1.3 */;
pub const DP_PHY_REPEATER_MODE: u32 = 0xf0003 /* 1.3 */;
pub const DP_MAX_LANE_COUNT_PHY_REPEATER: u32 = 0xf0004 /* 1.4a */;
pub const DP_Repeater_FEC_CAPABILITY: u32 = 0xf0004 /* 1.4 */;
pub const DP_PHY_REPEATER_EXTENDED_WAIT_TIMEOUT: u32 = 0xf0005 /* 1.4a */;
pub const DP_EXTENDED_WAKE_TIMEOUT_REQUEST_MASK: u32 = 0x7f;
pub const DP_EXTENDED_WAKE_TIMEOUT_GRANT: u32 = (1 << 7);
pub const DP_MAIN_LINK_CHANNEL_CODING_PHY_REPEATER: u32 = 0xf0006 /* 2.0 */;
pub const DP_PHY_REPEATER_128B132B_SUPPORTED: u32 = (1 << 0);
/* See DP_128B132B_SUPPORTED_LINK_RATES for values */
pub const DP_PHY_REPEATER_128B132B_RATES: u32 = 0xf0007 /* 2.0 */;
pub const DP_PHY_REPEATER_EQ_DONE: u32 = 0xf0008 /* 2.0 E11 */;

// C enum declaration follows; constants retain the original names and values.
	DP_PHY_DPRX,

	DP_PHY_LTTPR1,
	DP_PHY_LTTPR2,
	DP_PHY_LTTPR3,
	DP_PHY_LTTPR4,
	DP_PHY_LTTPR5,
	DP_PHY_LTTPR6,
	DP_PHY_LTTPR7,
	DP_PHY_LTTPR8,

pub const DP_MAX_LTTPR_COUNT: u32 = DP_PHY_LTTPR8;

// C function-like macro preserved: #define DP_PHY_LTTPR(i)					    (DP_PHY_LTTPR1 + (i))

pub const __DP_LTTPR1_BASE: u32 = 0xf0010 /* 1.3 */;
pub const __DP_LTTPR2_BASE: u32 = 0xf0060 /* 1.3 */;
// C function-like macro preserved: #define DP_LTTPR_BASE(dp_phy) \
	(__DP_LTTPR1_BASE + (__DP_LTTPR2_BASE - __DP_LTTPR1_BASE) * \
		((dp_phy) - DP_PHY_LTTPR1))

// C function-like macro preserved: #define DP_LTTPR_REG(dp_phy, lttpr1_reg) \
	(DP_LTTPR_BASE(dp_phy) - DP_LTTPR_BASE(DP_PHY_LTTPR1) + (lttpr1_reg))

pub const DP_TRAINING_PATTERN_SET_PHY_REPEATER1: u32 = 0xf0010 /* 1.3 */;
// C function-like macro preserved: #define DP_TRAINING_PATTERN_SET_PHY_REPEATER(dp_phy) \
	DP_LTTPR_REG(dp_phy, DP_TRAINING_PATTERN_SET_PHY_REPEATER1)

pub const DP_TRAINING_LANE0_SET_PHY_REPEATER1: u32 = 0xf0011 /* 1.3 */;
// C function-like macro preserved: #define DP_TRAINING_LANE0_SET_PHY_REPEATER(dp_phy) \
	DP_LTTPR_REG(dp_phy, DP_TRAINING_LANE0_SET_PHY_REPEATER1)

pub const DP_TRAINING_LANE1_SET_PHY_REPEATER1: u32 = 0xf0012 /* 1.3 */;
pub const DP_TRAINING_LANE2_SET_PHY_REPEATER1: u32 = 0xf0013 /* 1.3 */;
pub const DP_TRAINING_LANE3_SET_PHY_REPEATER1: u32 = 0xf0014 /* 1.3 */;
pub const DP_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER1: u32 = 0xf0020 /* 1.4a */;
// C function-like macro preserved: #define DP_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER(dp_phy)	\
	DP_LTTPR_REG(dp_phy, DP_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER1)

pub const DP_TRANSMITTER_CAPABILITY_PHY_REPEATER1: u32 = 0xf0021 /* 1.4a */;
pub const DP_VOLTAGE_SWING_LEVEL_3_SUPPORTED: u32 = BIT(0);
pub const DP_PRE_EMPHASIS_LEVEL_3_SUPPORTED: u32 = BIT(1);

pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER1: u32 = 0xf0022 /* 2.0 */;
// C function-like macro preserved: #define DP_128B132B_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER(dp_phy)	\
	DP_LTTPR_REG(dp_phy, DP_128B132B_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER1)
/* see DP_128B132B_TRAINING_AUX_RD_INTERVAL for values */

pub const DP_LANE0_1_STATUS_PHY_REPEATER1: u32 = 0xf0030 /* 1.3 */;
// C function-like macro preserved: #define DP_LANE0_1_STATUS_PHY_REPEATER(dp_phy) \
	DP_LTTPR_REG(dp_phy, DP_LANE0_1_STATUS_PHY_REPEATER1)

pub const DP_LANE2_3_STATUS_PHY_REPEATER1: u32 = 0xf0031 /* 1.3 */;

pub const DP_LANE_ALIGN_STATUS_UPDATED_PHY_REPEATER1: u32 = 0xf0032 /* 1.3 */;
pub const DP_ADJUST_REQUEST_LANE0_1_PHY_REPEATER1: u32 = 0xf0033 /* 1.3 */;
pub const DP_ADJUST_REQUEST_LANE2_3_PHY_REPEATER1: u32 = 0xf0034 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE0_PHY_REPEATER1: u32 = 0xf0035 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE1_PHY_REPEATER1: u32 = 0xf0037 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE2_PHY_REPEATER1: u32 = 0xf0039 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE3_PHY_REPEATER1: u32 = 0xf003b /* 1.3 */;

pub const DP_OUI_PHY_REPEATER1: u32 = 0xf003d /* 1.3 */;
// C function-like macro preserved: #define DP_OUI_PHY_REPEATER(dp_phy) \
	DP_LTTPR_REG(dp_phy, DP_OUI_PHY_REPEATER1)

pub const __DP_FEC1_BASE: u32 = 0xf0290 /* 1.4 */;
pub const __DP_FEC2_BASE: u32 = 0xf0298 /* 1.4 */;
// C function-like macro preserved: #define DP_FEC_BASE(dp_phy) \
	(__DP_FEC1_BASE + ((__DP_FEC2_BASE - __DP_FEC1_BASE) * \
			   ((dp_phy) - DP_PHY_LTTPR1)))

// C function-like macro preserved: #define DP_FEC_REG(dp_phy, fec1_reg) \
	(DP_FEC_BASE(dp_phy) - DP_FEC_BASE(DP_PHY_LTTPR1) + fec1_reg)

pub const DP_FEC_STATUS_PHY_REPEATER1: u32 = 0xf0290 /* 1.4 */;
// C function-like macro preserved: #define DP_FEC_STATUS_PHY_REPEATER(dp_phy) \
	DP_FEC_REG(dp_phy, DP_FEC_STATUS_PHY_REPEATER1)

pub const DP_FEC_ERROR_COUNT_PHY_REPEATER1: u32 = 0xf0291 /* 1.4 */;
pub const DP_FEC_CAPABILITY_PHY_REPEATER1: u32 = 0xf0294 /* 1.4a */;

pub const DP_LTTPR_MAX_ADD: u32 = 0xf02ff /* 1.4 */;

pub const DP_DPCD_MAX_ADD: u32 = 0xfffff /* 1.4 */;

/* Repeater modes */
pub const DP_PHY_REPEATER_MODE_TRANSPARENT: u32 = 0x55    /* 1.3 */;
pub const DP_PHY_REPEATER_MODE_NON_TRANSPARENT: u32 = 0xaa    /* 1.3 */;

/* DP HDCP message start offsets in DPCD address space */
pub const DP_HDCP_2_2_AKE_INIT_OFFSET: u32 = DP_HDCP_2_2_REG_RTX_OFFSET;
pub const DP_HDCP_2_2_AKE_SEND_CERT_OFFSET: u32 = DP_HDCP_2_2_REG_CERT_RX_OFFSET;
pub const DP_HDCP_2_2_AKE_NO_STORED_KM_OFFSET: u32 = DP_HDCP_2_2_REG_EKPUB_KM_OFFSET;
pub const DP_HDCP_2_2_AKE_STORED_KM_OFFSET: u32 = DP_HDCP_2_2_REG_EKH_KM_WR_OFFSET;
pub const DP_HDCP_2_2_AKE_SEND_HPRIME_OFFSET: u32 = DP_HDCP_2_2_REG_HPRIME_OFFSET;
pub const DP_HDCP_2_2_AKE_SEND_PAIRING_INFO_OFFSET: u32 = ;
						DP_HDCP_2_2_REG_EKH_KM_RD_OFFSET
pub const DP_HDCP_2_2_LC_INIT_OFFSET: u32 = DP_HDCP_2_2_REG_RN_OFFSET;
pub const DP_HDCP_2_2_LC_SEND_LPRIME_OFFSET: u32 = DP_HDCP_2_2_REG_LPRIME_OFFSET;
pub const DP_HDCP_2_2_SKE_SEND_EKS_OFFSET: u32 = DP_HDCP_2_2_REG_EDKEY_KS_OFFSET;
pub const DP_HDCP_2_2_REP_SEND_RECVID_LIST_OFFSET: u32 = DP_HDCP_2_2_REG_RXINFO_OFFSET;
pub const DP_HDCP_2_2_REP_SEND_ACK_OFFSET: u32 = DP_HDCP_2_2_REG_V_OFFSET;
pub const DP_HDCP_2_2_REP_STREAM_MANAGE_OFFSET: u32 = DP_HDCP_2_2_REG_SEQ_NUM_M_OFFSET;
pub const DP_HDCP_2_2_REP_STREAM_READY_OFFSET: u32 = DP_HDCP_2_2_REG_MPRIME_OFFSET;

pub const HDCP_2_2_DP_RXSTATUS_LEN: u32 = 1;
// C function-like macro preserved: #define HDCP_2_2_DP_RXSTATUS_READY(x)		((x) & BIT(0))
// C function-like macro preserved: #define HDCP_2_2_DP_RXSTATUS_H_PRIME(x)		((x) & BIT(1))
// C function-like macro preserved: #define HDCP_2_2_DP_RXSTATUS_PAIRING(x)		((x) & BIT(2))
// C function-like macro preserved: #define HDCP_2_2_DP_RXSTATUS_REAUTH_REQ(x)	((x) & BIT(3))
// C function-like macro preserved: #define HDCP_2_2_DP_RXSTATUS_LINK_FAILED(x)	((x) & BIT(4))

/* DP 1.2 Sideband message defines */
/* peer device type - DP 1.2a Table 2-92 */
pub const DP_PEER_DEVICE_NONE: u32 = 0x0;
pub const DP_PEER_DEVICE_SOURCE_OR_SST: u32 = 0x1;
pub const DP_PEER_DEVICE_MST_BRANCHING: u32 = 0x2;
pub const DP_PEER_DEVICE_SST_SINK: u32 = 0x3;
pub const DP_PEER_DEVICE_DP_LEGACY_CONV: u32 = 0x4;

/* DP 1.2 MST sideband request names DP 1.2a Table 2-80 */
pub const DP_GET_MSG_TRANSACTION_VERSION: u32 = 0x00 /* DP 1.3 */;
pub const DP_LINK_ADDRESS: u32 = 0x01;
pub const DP_CONNECTION_STATUS_NOTIFY: u32 = 0x02;
pub const DP_ENUM_PATH_RESOURCES: u32 = 0x10;
pub const DP_ALLOCATE_PAYLOAD: u32 = 0x11;
pub const DP_QUERY_PAYLOAD: u32 = 0x12;
pub const DP_RESOURCE_STATUS_NOTIFY: u32 = 0x13;
pub const DP_CLEAR_PAYLOAD_ID_TABLE: u32 = 0x14;
pub const DP_REMOTE_DPCD_READ: u32 = 0x20;
pub const DP_REMOTE_DPCD_WRITE: u32 = 0x21;
pub const DP_REMOTE_I2C_READ: u32 = 0x22;
pub const DP_REMOTE_I2C_WRITE: u32 = 0x23;
pub const DP_POWER_UP_PHY: u32 = 0x24;
pub const DP_POWER_DOWN_PHY: u32 = 0x25;
pub const DP_SINK_EVENT_NOTIFY: u32 = 0x30;
pub const DP_QUERY_STREAM_ENC_STATUS: u32 = 0x38;
pub const DP_QUERY_STREAM_ENC_STATUS_STATE_NO_EXIST: u32 = 0;
pub const DP_QUERY_STREAM_ENC_STATUS_STATE_INACTIVE: u32 = 1;
pub const DP_QUERY_STREAM_ENC_STATUS_STATE_ACTIVE: u32 = 2;

/* DP 1.2 MST sideband reply types */
pub const DP_SIDEBAND_REPLY_ACK: u32 = 0x00;
pub const DP_SIDEBAND_REPLY_NAK: u32 = 0x01;

/* DP 1.2 MST sideband nak reasons - table 2.84 */
pub const DP_NAK_WRITE_FAILURE: u32 = 0x01;
pub const DP_NAK_INVALID_READ: u32 = 0x02;
pub const DP_NAK_CRC_FAILURE: u32 = 0x03;
pub const DP_NAK_BAD_PARAM: u32 = 0x04;
pub const DP_NAK_DEFER: u32 = 0x05;
pub const DP_NAK_LINK_FAILURE: u32 = 0x06;
pub const DP_NAK_NO_RESOURCES: u32 = 0x07;
pub const DP_NAK_DPCD_FAIL: u32 = 0x08;
pub const DP_NAK_I2C_NAK: u32 = 0x09;
pub const DP_NAK_ALLOCATE_FAIL: u32 = 0x0a;

pub const MODE_I2C_START: u32 = 1;
pub const MODE_I2C_WRITE: u32 = 2;
pub const MODE_I2C_READ: u32 = 4;
pub const MODE_I2C_STOP: u32 = 8;

/* DP 1.2 MST PORTs - Section 2.5.1 v1.2a spec */
pub const DP_MST_PHYSICAL_PORT_0: u32 = 0;
pub const DP_MST_LOGICAL_PORT_0: u32 = 8;

pub const DP_LINK_CONSTANT_N_VALUE: u32 = 0x8000;
pub const DP_LINK_STATUS_SIZE: u32 = 6;

pub const DP_BRANCH_OUI_HEADER_SIZE: u32 = 0xc;
pub const DP_RECEIVER_CAP_SIZE: u32 = 0xf;
pub const DP_DSC_RECEIVER_CAP_SIZE: u32 = 0x10 /* DSC Capabilities 0x60 through 0x6F */;
pub const DP_DSC_BRANCH_CAP_SIZE: u32 = 3;
pub const EDP_PSR_RECEIVER_CAP_SIZE: u32 = 2;
pub const EDP_DISPLAY_CTL_CAP_SIZE: u32 = 5;
pub const DP_LTTPR_COMMON_CAP_SIZE: u32 = 8;
pub const DP_LTTPR_PHY_CAP_SIZE: u32 = 3;

pub const DP_SDP_AUDIO_TIMESTAMP: u32 = 0x01;
pub const DP_SDP_AUDIO_STREAM: u32 = 0x02;
pub const DP_SDP_EXTENSION: u32 = 0x04 /* DP 1.1 */;
pub const DP_SDP_AUDIO_COPYMANAGEMENT: u32 = 0x05 /* DP 1.2 */;
pub const DP_SDP_ISRC: u32 = 0x06 /* DP 1.2 */;
pub const DP_SDP_VSC: u32 = 0x07 /* DP 1.2 */;
pub const DP_SDP_ADAPTIVE_SYNC: u32 = 0x22 /* DP 1.4 */;
// C function-like macro preserved: #define DP_SDP_CAMERA_GENERIC(i)	(0x08 + (i)) /* 0-7, DP 1.3 */
pub const DP_SDP_PPS: u32 = 0x10 /* DP 1.4 */;
pub const DP_SDP_VSC_EXT_VESA: u32 = 0x20 /* DP 1.4 */;
pub const DP_SDP_VSC_EXT_CEA: u32 = 0x21 /* DP 1.4 */;

/* 0x80+ CEA-861 infoframe types */

pub const DP_SDP_AUDIO_INFOFRAME_HB2: u32 = 0x1b;

/**
 * struct dp_sdp_header - DP secondary data packet header
 * @HB0: Secondary Data Packet ID
 * @HB1: Secondary Data Packet Type
 * @HB2: Secondary Data Packet Specific header, Byte 0
 * @HB3: Secondary Data packet Specific header, Byte 1
 */
#[repr(C, packed)]
pub struct dp_sdp_header {
    pub HB0: u8, pub HB1: u8, pub HB2: u8, pub HB3: u8,
}
	u8 HB0;
	u8 HB1;
	u8 HB2;
	u8 HB3;
} __packed;

pub const EDP_SDP_HEADER_REVISION_MASK: u32 = 0x1F;
pub const EDP_SDP_HEADER_VALID_PAYLOAD_BYTES: u32 = 0x1F;
pub const DP_SDP_PPS_HEADER_PAYLOAD_BYTES_MINUS_1: u32 = 0x7F;

/**
 * struct dp_sdp - DP secondary data packet
 * @sdp_header: DP secondary data packet header
 * @db: DP secondaray data packet data blocks
 * VSC SDP Payload for PSR
 * db[0]: Stereo Interface
 * db[1]: 0 - PSR State; 1 - Update RFB; 2 - CRC Valid
 * db[2]: CRC value bits 7:0 of the R or Cr component
 * db[3]: CRC value bits 15:8 of the R or Cr component
 * db[4]: CRC value bits 7:0 of the G or Y component
 * db[5]: CRC value bits 15:8 of the G or Y component
 * db[6]: CRC value bits 7:0 of the B or Cb component
 * db[7]: CRC value bits 15:8 of the B or Cb component
 * db[8] - db[31]: Reserved
 * VSC SDP Payload for Pixel Encoding/Colorimetry Format
 * db[0] - db[15]: Reserved
 * db[16]: Pixel Encoding and Colorimetry Formats
 * db[17]: Dynamic Range and Component Bit Depth
 * db[18]: Content Type
 * db[19] - db[31]: Reserved
 */
#[repr(C, packed)]
pub struct dp_sdp {
    pub sdp_header: dp_sdp_header,
    pub db: [u8; 32],
}
	struct dp_sdp_header sdp_header;
	u8 db[32];
} __packed;

pub const EDP_VSC_PSR_STATE_ACTIVE: u32 = (1<<0);
pub const EDP_VSC_PSR_UPDATE_RFB: u32 = (1<<1);
pub const EDP_VSC_PSR_CRC_VALUES_VALID: u32 = (1<<2);

/**
 * enum dp_pixelformat - drm DP Pixel encoding formats
 *
 * This enum is used to indicate DP VSC SDP Pixel encoding formats.
 * It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
 * DB18]
 *
 * @DP_PIXELFORMAT_RGB: RGB pixel encoding format
 * @DP_PIXELFORMAT_YUV444: YCbCr 4:4:4 pixel encoding format
 * @DP_PIXELFORMAT_YUV422: YCbCr 4:2:2 pixel encoding format
 * @DP_PIXELFORMAT_YUV420: YCbCr 4:2:0 pixel encoding format
 * @DP_PIXELFORMAT_Y_ONLY: Y Only pixel encoding format
 * @DP_PIXELFORMAT_RAW: RAW pixel encoding format
 * @DP_PIXELFORMAT_RESERVED: Reserved pixel encoding format
 */
// C enum declaration follows; constants retain the original names and values.
pub const DP_PIXELFORMAT_RGB: u32 = 0;
pub const DP_PIXELFORMAT_YUV444: u32 = 0x1;
pub const DP_PIXELFORMAT_YUV422: u32 = 0x2;
pub const DP_PIXELFORMAT_YUV420: u32 = 0x3;
pub const DP_PIXELFORMAT_Y_ONLY: u32 = 0x4;
pub const DP_PIXELFORMAT_RAW: u32 = 0x5;
pub const DP_PIXELFORMAT_RESERVED: u32 = 0x6;

/**
 * enum dp_colorimetry - drm DP Colorimetry formats
 *
 * This enum is used to indicate DP VSC SDP Colorimetry formats.
 * It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
 * DB18] and a name of enum member follows enum drm_colorimetry definition.
 *
 * @DP_COLORIMETRY_DEFAULT: sRGB (IEC 61966-2-1) or
 *                          ITU-R BT.601 colorimetry format
 * @DP_COLORIMETRY_RGB_WIDE_FIXED: RGB wide gamut fixed point colorimetry format
 * @DP_COLORIMETRY_BT709_YCC: ITU-R BT.709 colorimetry format
 * @DP_COLORIMETRY_RGB_WIDE_FLOAT: RGB wide gamut floating point
 *                                 (scRGB (IEC 61966-2-2)) colorimetry format
 * @DP_COLORIMETRY_XVYCC_601: xvYCC601 colorimetry format
 * @DP_COLORIMETRY_OPRGB: OpRGB colorimetry format
 * @DP_COLORIMETRY_XVYCC_709: xvYCC709 colorimetry format
 * @DP_COLORIMETRY_DCI_P3_RGB: DCI-P3 (SMPTE RP 431-2) colorimetry format
 * @DP_COLORIMETRY_SYCC_601: sYCC601 colorimetry format
 * @DP_COLORIMETRY_RGB_CUSTOM: RGB Custom Color Profile colorimetry format
 * @DP_COLORIMETRY_OPYCC_601: opYCC601 colorimetry format
 * @DP_COLORIMETRY_BT2020_RGB: ITU-R BT.2020 R' G' B' colorimetry format
 * @DP_COLORIMETRY_BT2020_CYCC: ITU-R BT.2020 Y'c C'bc C'rc colorimetry format
 * @DP_COLORIMETRY_BT2020_YCC: ITU-R BT.2020 Y' C'b C'r colorimetry format
 */
// C enum declaration follows; constants retain the original names and values.
pub const DP_COLORIMETRY_DEFAULT: u32 = 0;
pub const DP_COLORIMETRY_RGB_WIDE_FIXED: u32 = 0x1;
pub const DP_COLORIMETRY_BT709_YCC: u32 = 0x1;
pub const DP_COLORIMETRY_RGB_WIDE_FLOAT: u32 = 0x2;
pub const DP_COLORIMETRY_XVYCC_601: u32 = 0x2;
pub const DP_COLORIMETRY_OPRGB: u32 = 0x3;
pub const DP_COLORIMETRY_XVYCC_709: u32 = 0x3;
pub const DP_COLORIMETRY_DCI_P3_RGB: u32 = 0x4;
pub const DP_COLORIMETRY_SYCC_601: u32 = 0x4;
pub const DP_COLORIMETRY_RGB_CUSTOM: u32 = 0x5;
pub const DP_COLORIMETRY_OPYCC_601: u32 = 0x5;
pub const DP_COLORIMETRY_BT2020_RGB: u32 = 0x6;
pub const DP_COLORIMETRY_BT2020_CYCC: u32 = 0x6;
pub const DP_COLORIMETRY_BT2020_YCC: u32 = 0x7;

/**
 * enum dp_dynamic_range - drm DP Dynamic Range
 *
 * This enum is used to indicate DP VSC SDP Dynamic Range.
 * It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
 * DB18]
 *
 * @DP_DYNAMIC_RANGE_VESA: VESA range
 * @DP_DYNAMIC_RANGE_CTA: CTA range
 */
// C enum declaration follows; constants retain the original names and values.
pub const DP_DYNAMIC_RANGE_VESA: u32 = 0;
pub const DP_DYNAMIC_RANGE_CTA: u32 = 1;

/**
 * enum dp_content_type - drm DP Content Type
 *
 * This enum is used to indicate DP VSC SDP Content Types.
 * It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
 * DB18]
 * CTA-861-G defines content types and expected processing by a sink device
 *
 * @DP_CONTENT_TYPE_NOT_DEFINED: Not defined type
 * @DP_CONTENT_TYPE_GRAPHICS: Graphics type
 * @DP_CONTENT_TYPE_PHOTO: Photo type
 * @DP_CONTENT_TYPE_VIDEO: Video type
 * @DP_CONTENT_TYPE_GAME: Game type
 */
// C enum declaration follows; constants retain the original names and values.
pub const DP_CONTENT_TYPE_NOT_DEFINED: u32 = 0x00;
pub const DP_CONTENT_TYPE_GRAPHICS: u32 = 0x01;
pub const DP_CONTENT_TYPE_PHOTO: u32 = 0x02;
pub const DP_CONTENT_TYPE_VIDEO: u32 = 0x03;
pub const DP_CONTENT_TYPE_GAME: u32 = 0x04;

// C enum declaration follows; constants retain the original names and values.
pub const DP_AS_SDP_AVT_DYNAMIC_VTOTAL: u32 = 0x00;
pub const DP_AS_SDP_AVT_FIXED_VTOTAL: u32 = 0x01;
pub const DP_AS_SDP_FAVT_TRR_NOT_REACHED: u32 = 0x02;
	DP_AS_SDP_FAVT_TRR_REACHED = 0x03

pub const DP_AS_SDP_OPERATION_MODE_MASK: u32 = GENMASK(1, 0);
pub const DP_AS_SDP_LENGTH_MASK: u32 = GENMASK(5, 0);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
