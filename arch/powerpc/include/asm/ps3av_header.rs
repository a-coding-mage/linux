/* SPDX-License-Identifier: GPL-2.0-only */
/* PS3 AV backend support. */

/* C header dependencies are supplied by the surrounding translation unit. */

pub const PS3AV_VERSION: u32 = 0x205;

pub const PS3AV_CID_AV_INIT: u32 = 0x00000001;
pub const PS3AV_CID_AV_FIN: u32 = 0x00000002;
pub const PS3AV_CID_AV_GET_HW_CONF: u32 = 0x00000003;
pub const PS3AV_CID_AV_GET_MONITOR_INFO: u32 = 0x00000004;
pub const PS3AV_CID_AV_ENABLE_EVENT: u32 = 0x00000006;
pub const PS3AV_CID_AV_DISABLE_EVENT: u32 = 0x00000007;
pub const PS3AV_CID_AV_TV_MUTE: u32 = 0x0000000a;
pub const PS3AV_CID_AV_VIDEO_CS: u32 = 0x00010001;
pub const PS3AV_CID_AV_VIDEO_MUTE: u32 = 0x00010002;
pub const PS3AV_CID_AV_VIDEO_DISABLE_SIG: u32 = 0x00010003;
pub const PS3AV_CID_AV_AUDIO_PARAM: u32 = 0x00020001;
pub const PS3AV_CID_AV_AUDIO_MUTE: u32 = 0x00020002;
pub const PS3AV_CID_AV_HDMI_MODE: u32 = 0x00040001;
pub const PS3AV_CID_VIDEO_INIT: u32 = 0x01000001;
pub const PS3AV_CID_VIDEO_MODE: u32 = 0x01000002;
pub const PS3AV_CID_VIDEO_FORMAT: u32 = 0x01000004;
pub const PS3AV_CID_VIDEO_PITCH: u32 = 0x01000005;
pub const PS3AV_CID_AUDIO_INIT: u32 = 0x02000001;
pub const PS3AV_CID_AUDIO_MODE: u32 = 0x02000002;
pub const PS3AV_CID_AUDIO_MUTE: u32 = 0x02000003;
pub const PS3AV_CID_AUDIO_ACTIVE: u32 = 0x02000004;
pub const PS3AV_CID_AUDIO_INACTIVE: u32 = 0x02000005;
pub const PS3AV_CID_AUDIO_SPDIF_BIT: u32 = 0x02000006;
pub const PS3AV_CID_AUDIO_CTRL: u32 = 0x02000007;
pub const PS3AV_CID_EVENT_UNPLUGGED: u32 = 0x10000001;
pub const PS3AV_CID_EVENT_PLUGGED: u32 = 0x10000002;
pub const PS3AV_CID_EVENT_HDCP_DONE: u32 = 0x10000003;
pub const PS3AV_CID_EVENT_HDCP_FAIL: u32 = 0x10000004;
pub const PS3AV_CID_EVENT_HDCP_AUTH: u32 = 0x10000005;
pub const PS3AV_CID_EVENT_HDCP_ERROR: u32 = 0x10000006;
pub const PS3AV_CID_AVB_PARAM: u32 = 0x04000001;

pub const PS3AV_HDMI_MAX: usize = 2;
pub const PS3AV_AVMULTI_MAX: usize = 1;
pub const PS3AV_AV_PORT_MAX: usize = PS3AV_HDMI_MAX + PS3AV_AVMULTI_MAX;
pub const PS3AV_OPT_PORT_MAX: usize = 1;
pub const PS3AV_HEAD_MAX: usize = 2;
pub const PS3AV_AVB_NUM_VIDEO: usize = PS3AV_HEAD_MAX;
pub const PS3AV_AVB_NUM_AUDIO: usize = 0;
pub const PS3AV_AVB_NUM_AV_VIDEO: usize = PS3AV_AV_PORT_MAX;
pub const PS3AV_AVB_NUM_AV_AUDIO: usize = PS3AV_HDMI_MAX;
pub const PS3AV_MUTE_PORT_MAX: usize = 1;

pub const PS3AV_CMD_EVENT_BIT_UNPLUGGED: u32 = 1 << 0;
pub const PS3AV_CMD_EVENT_BIT_PLUGGED: u32 = 1 << 1;
pub const PS3AV_CMD_EVENT_BIT_HDCP_DONE: u32 = 1 << 2;
pub const PS3AV_CMD_EVENT_BIT_HDCP_FAIL: u32 = 1 << 3;
pub const PS3AV_CMD_EVENT_BIT_HDCP_REAUTH: u32 = 1 << 4;
pub const PS3AV_CMD_EVENT_BIT_HDCP_TOPOLOGY: u32 = 1 << 5;
pub const PS3AV_CMD_MUTE_OFF: u16 = 0;
pub const PS3AV_CMD_MUTE_ON: u16 = 1;
pub const PS3AV_CMD_AVPORT_HDMI_0: u16 = 0;
pub const PS3AV_CMD_AVPORT_HDMI_1: u16 = 1;
pub const PS3AV_CMD_AVPORT_AVMULTI_0: u16 = 0x10;
pub const PS3AV_CMD_AVPORT_SPDIF_0: u16 = 0x20;
pub const PS3AV_CMD_AVPORT_SPDIF_1: u16 = 0x21;
pub const PS3AV_CMD_AV_MCLK_128: u8 = 0;
pub const PS3AV_CMD_AV_MCLK_256: u8 = 1;
pub const PS3AV_CMD_AV_MCLK_512: u8 = 3;
pub const PS3AV_CMD_AV_INPUTLEN_16: u8 = 2;
pub const PS3AV_CMD_AV_INPUTLEN_20: u8 = 0xa;
pub const PS3AV_CMD_AV_INPUTLEN_24: u8 = 0xb;
pub const PS3AV_CMD_AV_LAYOUT_32: u8 = 1 << 0;
pub const PS3AV_CMD_AV_LAYOUT_44: u8 = 1 << 1;
pub const PS3AV_CMD_AV_LAYOUT_48: u8 = 1 << 2;
pub const PS3AV_CMD_AV_LAYOUT_88: u8 = 1 << 3;
pub const PS3AV_CMD_AV_LAYOUT_96: u8 = 1 << 4;
pub const PS3AV_CMD_AV_LAYOUT_176: u8 = 1 << 5;
pub const PS3AV_CMD_AV_LAYOUT_192: u8 = 1 << 6;
pub const PS3AV_CMD_AV_HDMI_MODE_NORMAL: u8 = 0xff;
pub const PS3AV_CMD_AV_HDMI_HDCP_OFF: u8 = 1;
pub const PS3AV_CMD_AV_HDMI_EDID_PASS: u8 = 0x80;
pub const PS3AV_CMD_AV_HDMI_DVI: u8 = 0x40;

pub const PS3AV_CMD_VIDEO_HEAD_A: u32 = 0;
pub const PS3AV_CMD_VIDEO_HEAD_B: u32 = 1;
pub const PS3AV_CMD_VIDEO_CS_NONE: u32 = 0;
pub const PS3AV_CMD_VIDEO_CS_RGB_8: u32 = 1;
pub const PS3AV_CMD_VIDEO_CS_YUV444_8: u32 = 2;
pub const PS3AV_CMD_VIDEO_CS_YUV422_8: u32 = 3;
pub const PS3AV_CMD_VIDEO_CS_XVYCC_8: u32 = 4;
pub const PS3AV_CMD_VIDEO_CS_RGB_10: u32 = 5;
pub const PS3AV_CMD_VIDEO_CS_YUV444_10: u32 = 6;
pub const PS3AV_CMD_VIDEO_CS_YUV422_10: u32 = 7;
pub const PS3AV_CMD_VIDEO_CS_XVYCC_10: u32 = 8;
pub const PS3AV_CMD_VIDEO_CS_RGB_12: u32 = 9;
pub const PS3AV_CMD_VIDEO_CS_YUV444_12: u32 = 0xa;
pub const PS3AV_CMD_VIDEO_CS_YUV422_12: u32 = 0xb;
pub const PS3AV_CMD_VIDEO_CS_XVYCC_12: u32 = 0xc;
pub const PS3AV_CMD_VIDEO_VID_NONE: u32 = 0;
pub const PS3AV_CMD_VIDEO_VID_480I: u32 = 1;
pub const PS3AV_CMD_VIDEO_VID_576I: u32 = 3;
pub const PS3AV_CMD_VIDEO_VID_480P: u32 = 5;
pub const PS3AV_CMD_VIDEO_VID_576P: u32 = 6;
pub const PS3AV_CMD_VIDEO_VID_1080I_60HZ: u32 = 7;
pub const PS3AV_CMD_VIDEO_VID_1080I_50HZ: u32 = 8;
pub const PS3AV_CMD_VIDEO_VID_720P_60HZ: u32 = 9;
pub const PS3AV_CMD_VIDEO_VID_720P_50HZ: u32 = 0xa;
pub const PS3AV_CMD_VIDEO_VID_1080P_60HZ: u32 = 0xb;
pub const PS3AV_CMD_VIDEO_VID_1080P_50HZ: u32 = 0xc;
pub const PS3AV_CMD_VIDEO_VID_WXGA: u32 = 0xd;
pub const PS3AV_CMD_VIDEO_VID_SXGA: u32 = 0xe;
pub const PS3AV_CMD_VIDEO_VID_WUXGA: u32 = 0xf;
pub const PS3AV_CMD_VIDEO_VID_480I_A: u32 = 0x10;
pub const PS3AV_CMD_VIDEO_FORMAT_BLACK: u32 = 0;
pub const PS3AV_CMD_VIDEO_FORMAT_ARGB_8BIT: u32 = 7;
pub const PS3AV_CMD_VIDEO_ORDER_RGB: u32 = 0;
pub const PS3AV_CMD_VIDEO_ORDER_BGR: u32 = 1;
pub const PS3AV_CMD_VIDEO_FMT_X8R8G8B8: u32 = 0;
pub const PS3AV_CMD_VIDEO_OUT_FORMAT_RGB_12BIT: u32 = 0;
pub const PS3AV_CMD_VIDEO_CL_CNV_ENABLE_LUT: u32 = 0;
pub const PS3AV_CMD_VIDEO_CL_CNV_DISABLE_LUT: u32 = 0x10;
pub const PS3AV_CMD_VIDEO_SYNC_VSYNC: u32 = 1;
pub const PS3AV_CMD_VIDEO_SYNC_CSYNC: u32 = 4;
pub const PS3AV_CMD_VIDEO_SYNC_HSYNC: u32 = 0x10;

pub const PS3AV_CMD_AUDIO_NUM_OF_CH_2: u32 = 0;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_3: u32 = 1;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_4: u32 = 2;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_5: u32 = 3;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_6: u32 = 4;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_7: u32 = 5;
pub const PS3AV_CMD_AUDIO_NUM_OF_CH_8: u32 = 6;
pub const PS3AV_CMD_AUDIO_FS_32K: u32 = 1;
pub const PS3AV_CMD_AUDIO_FS_44K: u32 = 2;
pub const PS3AV_CMD_AUDIO_FS_48K: u32 = 3;
pub const PS3AV_CMD_AUDIO_FS_88K: u32 = 4;
pub const PS3AV_CMD_AUDIO_FS_96K: u32 = 5;
pub const PS3AV_CMD_AUDIO_FS_176K: u32 = 6;
pub const PS3AV_CMD_AUDIO_FS_192K: u32 = 7;
pub const PS3AV_CMD_AUDIO_WORD_BITS_16: u32 = 1;
pub const PS3AV_CMD_AUDIO_WORD_BITS_20: u32 = 2;
pub const PS3AV_CMD_AUDIO_WORD_BITS_24: u32 = 3;
pub const PS3AV_CMD_AUDIO_FORMAT_PCM: u32 = 1;
pub const PS3AV_CMD_AUDIO_FORMAT_BITSTREAM: u32 = 0xff;
pub const PS3AV_CMD_AUDIO_SOURCE_SERIAL: u32 = 0;
pub const PS3AV_CMD_AUDIO_SOURCE_SPDIF: u32 = 1;
pub const PS3AV_CMD_AUDIO_SWAP_0: u32 = 0;
pub const PS3AV_CMD_AUDIO_SWAP_1: u32 = 0;
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_0: u32 = 0;
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_1: u32 = 1;
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_2: u32 = 2;
pub const PS3AV_CMD_AUDIO_MAP_OUTPUT_3: u32 = 3;
pub const PS3AV_CMD_AUDIO_LAYOUT_2CH: u32 = 0;
pub const PS3AV_CMD_AUDIO_LAYOUT_6CH: u32 = 0xb;
pub const PS3AV_CMD_AUDIO_LAYOUT_8CH: u32 = 0x1f;
pub const PS3AV_CMD_AUDIO_DOWNMIX_PERMITTED: u32 = 0;
pub const PS3AV_CMD_AUDIO_DOWNMIX_PROHIBITED: u32 = 1;
pub const PS3AV_CMD_AUDIO_PORT_HDMI_0: u32 = 1 << 0;
pub const PS3AV_CMD_AUDIO_PORT_HDMI_1: u32 = 1 << 1;
pub const PS3AV_CMD_AUDIO_PORT_AVMULTI_0: u32 = 1 << 10;
pub const PS3AV_CMD_AUDIO_PORT_SPDIF_0: u32 = 1 << 20;
pub const PS3AV_CMD_AUDIO_PORT_SPDIF_1: u32 = 1 << 21;
pub const PS3AV_CMD_AUDIO_CTRL_ID_DAC_RESET: u32 = 0;
pub const PS3AV_CMD_AUDIO_CTRL_ID_DAC_DE_EMPHASIS: u32 = 1;
pub const PS3AV_CMD_AUDIO_CTRL_ID_AVCLK: u32 = 2;
pub const PS3AV_CMD_AUDIO_CTRL_RESET_NEGATE: u32 = 0;
pub const PS3AV_CMD_AUDIO_CTRL_RESET_ASSERT: u32 = 1;
pub const PS3AV_CMD_AUDIO_CTRL_DE_EMPHASIS_OFF: u32 = 0;
pub const PS3AV_CMD_AUDIO_CTRL_DE_EMPHASIS_ON: u32 = 1;
pub const PS3AV_CMD_AUDIO_CTRL_AVCLK_22: u32 = 0;
pub const PS3AV_CMD_AUDIO_CTRL_AVCLK_18: u32 = 1;

pub const PS3AV_CMD_AV_VID_480I: u32 = 0;
pub const PS3AV_CMD_AV_VID_480P: u32 = 1;
pub const PS3AV_CMD_AV_VID_720P_60HZ: u32 = 2;
pub const PS3AV_CMD_AV_VID_1080I_60HZ: u32 = 3;
pub const PS3AV_CMD_AV_VID_1080P_60HZ: u32 = 4;
pub const PS3AV_CMD_AV_VID_576I: u32 = 5;
pub const PS3AV_CMD_AV_VID_576P: u32 = 6;
pub const PS3AV_CMD_AV_VID_720P_50HZ: u32 = 7;
pub const PS3AV_CMD_AV_VID_1080I_50HZ: u32 = 8;
pub const PS3AV_CMD_AV_VID_1080P_50HZ: u32 = 9;
pub const PS3AV_CMD_AV_VID_WXGA: u32 = 0xa;
pub const PS3AV_CMD_AV_VID_SXGA: u32 = 0xb;
pub const PS3AV_CMD_AV_VID_WUXGA: u32 = 0xc;
pub const PS3AV_CMD_AV_CS_RGB_8: u32 = 0;
pub const PS3AV_CMD_AV_CS_YUV444_8: u32 = 1;
pub const PS3AV_CMD_AV_CS_YUV422_8: u32 = 2;
pub const PS3AV_CMD_AV_CS_XVYCC_8: u32 = 3;
pub const PS3AV_CMD_AV_CS_RGB_10: u32 = 4;
pub const PS3AV_CMD_AV_CS_YUV444_10: u32 = 5;
pub const PS3AV_CMD_AV_CS_YUV422_10: u32 = 6;
pub const PS3AV_CMD_AV_CS_XVYCC_10: u32 = 7;
pub const PS3AV_CMD_AV_CS_RGB_12: u32 = 8;
pub const PS3AV_CMD_AV_CS_YUV444_12: u32 = 9;
pub const PS3AV_CMD_AV_CS_YUV422_12: u32 = 0xa;
pub const PS3AV_CMD_AV_CS_XVYCC_12: u32 = 0xb;
pub const PS3AV_CMD_AV_CS_8: u32 = 0;
pub const PS3AV_CMD_AV_CS_10: u32 = 1;
pub const PS3AV_CMD_AV_CS_12: u32 = 2;
pub const PS3AV_CMD_AV_DITHER_OFF: u32 = 0;
pub const PS3AV_CMD_AV_DITHER_ON: u32 = 1;
pub const PS3AV_CMD_AV_DITHER_8BIT: u32 = 0;
pub const PS3AV_CMD_AV_DITHER_10BIT: u32 = 2;
pub const PS3AV_CMD_AV_DITHER_12BIT: u32 = 4;
pub const PS3AV_CMD_AV_SUPER_WHITE_OFF: u32 = 0;
pub const PS3AV_CMD_AV_SUPER_WHITE_ON: u32 = 1;
pub const PS3AV_CMD_AV_ASPECT_16_9: u32 = 0;
pub const PS3AV_CMD_AV_ASPECT_4_3: u32 = 1;
pub const PS3AV_CMD_VIDEO_CS_RGB: u32 = 1;
pub const PS3AV_CMD_VIDEO_CS_YUV422: u32 = 2;
pub const PS3AV_CMD_VIDEO_CS_YUV444: u32 = 3;

pub const PS3AV_RESBIT_720X480P: u32 = 3;
pub const PS3AV_RESBIT_720X576P: u32 = 3;
pub const PS3AV_RESBIT_1280X720P: u32 = 4;
pub const PS3AV_RESBIT_1920X1080I: u32 = 8;
pub const PS3AV_RESBIT_1920X1080P: u32 = 0x4000;
pub const PS3AV_RES_MASK_60: u32 = PS3AV_RESBIT_720X480P | PS3AV_RESBIT_1280X720P | PS3AV_RESBIT_1920X1080I | PS3AV_RESBIT_1920X1080P;
pub const PS3AV_RES_MASK_50: u32 = PS3AV_RESBIT_720X576P | PS3AV_RESBIT_1280X720P | PS3AV_RESBIT_1920X1080I | PS3AV_RESBIT_1920X1080P;
pub const PS3AV_RESBIT_VGA: u32 = 1;
pub const PS3AV_RESBIT_WXGA: u32 = 2;
pub const PS3AV_RESBIT_SXGA: u32 = 4;
pub const PS3AV_RESBIT_WUXGA: u32 = 8;
pub const PS3AV_RES_MASK_VESA: u32 = PS3AV_RESBIT_WXGA | PS3AV_RESBIT_SXGA | PS3AV_RESBIT_WUXGA;
pub const PS3AV_MONITOR_TYPE_HDMI: u32 = 1;
pub const PS3AV_MONITOR_TYPE_DVI: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_mode_num;
pub const PS3AV_MODE_AUTO: u32 = 0;
pub const PS3AV_MODE_480I: u32 = 1;
pub const PS3AV_MODE_480P: u32 = 2;
pub const PS3AV_MODE_720P60: u32 = 3;
pub const PS3AV_MODE_1080I60: u32 = 4;
pub const PS3AV_MODE_1080P60: u32 = 5;
pub const PS3AV_MODE_576I: u32 = 6;
pub const PS3AV_MODE_576P: u32 = 7;
pub const PS3AV_MODE_720P50: u32 = 8;
pub const PS3AV_MODE_1080I50: u32 = 9;
pub const PS3AV_MODE_1080P50: u32 = 10;
pub const PS3AV_MODE_WXGA: u32 = 11;
pub const PS3AV_MODE_SXGA: u32 = 12;
pub const PS3AV_MODE_WUXGA: u32 = 13;
pub const PS3AV_MODE_MASK: u32 = 0xf;
pub const PS3AV_MODE_HDCP_OFF: u32 = 0x1000;
pub const PS3AV_MODE_DITHER: u32 = 0x800;
pub const PS3AV_MODE_COLOR: u32 = 0x400;
pub const PS3AV_MODE_WHITE: u32 = 0x200;
pub const PS3AV_MODE_FULL: u32 = 0x80;
pub const PS3AV_MODE_DVI: u32 = 0x40;
pub const PS3AV_MODE_RGB: u32 = 0x20;
pub const PS3AV_DEFAULT_HDMI_MODE_ID_REG_60: u32 = PS3AV_MODE_480P;
pub const PS3AV_DEFAULT_AVMULTI_MODE_ID_REG_60: u32 = PS3AV_MODE_480I;
pub const PS3AV_DEFAULT_HDMI_MODE_ID_REG_50: u32 = PS3AV_MODE_576P;
pub const PS3AV_DEFAULT_AVMULTI_MODE_ID_REG_50: u32 = PS3AV_MODE_576I;
pub const PS3AV_REGION_60: u32 = 1;
pub const PS3AV_REGION_50: u32 = 2;
pub const PS3AV_REGION_RGB: u32 = 0x10;
pub const PS3AV_HDR_SIZE: usize = 4;

#[inline]
pub unsafe fn get_status(buf: *const core::ffi::c_void) -> u32 {
    *((buf as *const u32).add(2))
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_send_hdr { pub version: u16, pub size: u16, pub cid: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_reply_hdr { pub version: u16, pub size: u16, pub cid: u32, pub status: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_init { pub send_hdr: ps3av_send_hdr, pub event_bit: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_fin { pub send_hdr: ps3av_send_hdr, pub reserved: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_get_hw_conf { pub send_hdr: ps3av_send_hdr, pub status: u32, pub num_of_hdmi: u16, pub num_of_avmulti: u16, pub num_of_spdif: u16, pub reserved: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_resolution { pub res_bits: u32, pub native: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_cs { pub rgb: u8, pub yuv444: u8, pub yuv422: u8, pub reserved: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_color { pub red_x: u16, pub red_y: u16, pub green_x: u16, pub green_y: u16, pub blue_x: u16, pub blue_y: u16, pub white_x: u16, pub white_y: u16, pub gamma: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_info_audio { pub r#type: u8, pub max_num_of_ch: u8, pub fs: u8, pub sbit: u8 }
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ps3av_info_monitor {
    pub avport: u8, pub monitor_id: [u8; 10], pub monitor_type: u8, pub monitor_name: [u8; 16],
    pub res_60: ps3av_info_resolution, pub res_50: ps3av_info_resolution, pub res_other: ps3av_info_resolution, pub res_vesa: ps3av_info_resolution,
    pub cs: ps3av_info_cs, pub color: ps3av_info_color, pub supported_ai: u8, pub speaker_info: u8, pub num_of_audio_block: u8,
    pub audio: [ps3av_info_audio; 0], pub reserved: [u8; 169],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_get_monitor_info { pub send_hdr: ps3av_send_hdr, pub avport: u16, pub reserved: u16, pub info: ps3av_info_monitor }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_event { pub send_hdr: ps3av_send_hdr, pub event_bit: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_video_cs { pub send_hdr: ps3av_send_hdr, pub avport: u16, pub av_vid: u16, pub av_cs_out: u16, pub av_cs_in: u16, pub dither: u8, pub bitlen_out: u8, pub super_white: u8, pub aspect: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_av_mute { pub avport: u16, pub mute: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_video_mute { pub send_hdr: ps3av_send_hdr, pub mute: [ps3av_av_mute; PS3AV_MUTE_PORT_MAX] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_video_disable_sig { pub send_hdr: ps3av_send_hdr, pub avport: u16, pub reserved: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pb1_bit { pub bits: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pb2_bit { pub bits: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pb5_bit { pub bits: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_audio_info_frame { pub pb1: pb1_bit, pub pb2: pb2_bit, pub pb3: u8, pub pb4: u8, pub pb5: pb5_bit }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_audio_param { pub send_hdr: ps3av_send_hdr, pub avport: u16, pub reserved: u16, pub mclk: u8, pub ns: [u8; 3], pub enable: u8, pub swaplr: u8, pub fifomap: u8, pub inputctrl: u8, pub inputlen: u8, pub layout: u8, pub info: ps3av_audio_info_frame, pub chstat: [u8; 5] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_audio_mute { pub send_hdr: ps3av_send_hdr, pub mute: [ps3av_av_mute; PS3AV_MUTE_PORT_MAX] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_hdmi_mode { pub send_hdr: ps3av_send_hdr, pub mode: u8, pub reserved0: u8, pub reserved1: u8, pub reserved2: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_av_tv_mute { pub send_hdr: ps3av_send_hdr, pub avport: u16, pub mute: u16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_init { pub send_hdr: ps3av_send_hdr, pub reserved: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_mode { pub send_hdr: ps3av_send_hdr, pub video_head: u32, pub reserved: u32, pub video_vid: u32, pub reserved1: u16, pub width: u16, pub reserved2: u16, pub height: u16, pub pitch: u32, pub video_out_format: u32, pub video_format: u32, pub reserved3: u8, pub video_cl_cnv: u8, pub video_order: u16, pub reserved4: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_format { pub send_hdr: ps3av_send_hdr, pub video_head: u32, pub video_format: u32, pub reserved: u8, pub video_cl_cnv: u8, pub video_order: u16 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_video_pitch { pub version: u16, pub size: u16, pub cid: u32, pub video_head: u32, pub pitch: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_init { pub send_hdr: ps3av_send_hdr, pub reserved: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_mode { pub send_hdr: ps3av_send_hdr, pub avport: u8, pub reserved0: [u8; 3], pub mask: u32, pub audio_num_of_ch: u32, pub audio_fs: u32, pub audio_word_bits: u32, pub audio_format: u32, pub audio_source: u32, pub audio_enable: [u8; 4], pub audio_swap: [u8; 4], pub audio_map: [u8; 4], pub audio_layout: u32, pub audio_downmix: u32, pub audio_downmix_level: u32, pub audio_cs_info: [u8; 8] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_audio_mute { pub avport: u8, pub reserved: [u8; 3], pub mute: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_mute { pub send_hdr: ps3av_send_hdr, pub mute: [ps3av_audio_mute; PS3AV_OPT_PORT_MAX] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_active { pub send_hdr: ps3av_send_hdr, pub audio_port: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_spdif_bit { pub version: u16, pub size: u16, pub cid: u32, pub avport: u8, pub reserved: [u8; 3], pub audio_port: u32, pub spdif_bit_data: [u32; 12] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_audio_ctrl { pub version: u16, pub size: u16, pub cid: u32, pub audio_ctrl_id: u32, pub audio_ctrl_data: [u32; 4] }

pub const PS3AV_PKT_AVB_PARAM_MAX_BUF_SIZE: usize = PS3AV_AVB_NUM_VIDEO * core::mem::size_of::<ps3av_pkt_video_mode>() + PS3AV_AVB_NUM_AUDIO * core::mem::size_of::<ps3av_pkt_audio_mode>() + PS3AV_AVB_NUM_AV_VIDEO * core::mem::size_of::<ps3av_pkt_av_video_cs>() + PS3AV_AVB_NUM_AV_AUDIO * core::mem::size_of::<ps3av_pkt_av_audio_param>();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3av_pkt_avb_param { pub send_hdr: ps3av_send_hdr, pub num_of_video_pkt: u16, pub num_of_audio_pkt: u16, pub num_of_av_video_pkt: u16, pub num_of_av_audio_pkt: u16, pub buf: [u8; PS3AV_PKT_AVB_PARAM_MAX_BUF_SIZE] }

extern "C" {
    pub static mut ps3av_mode_cs_info: u8;
}

pub const PS3AV_STATUS_SUCCESS: u32 = 0;
pub const PS3AV_STATUS_RECEIVE_VUART_ERROR: u32 = 1;
pub const PS3AV_STATUS_SYSCON_COMMUNICATE_FAIL: u32 = 2;
pub const PS3AV_STATUS_INVALID_COMMAND: u32 = 3;
pub const PS3AV_STATUS_INVALID_PORT: u32 = 4;
pub const PS3AV_STATUS_INVALID_VID: u32 = 5;
pub const PS3AV_STATUS_INVALID_COLOR_SPACE: u32 = 6;
pub const PS3AV_STATUS_INVALID_FS: u32 = 7;
pub const PS3AV_STATUS_INVALID_AUDIO_CH: u32 = 8;
pub const PS3AV_STATUS_UNSUPPORTED_VERSION: u32 = 9;
pub const PS3AV_STATUS_INVALID_SAMPLE_SIZE: u32 = 0xa;
pub const PS3AV_STATUS_FAILURE: u32 = 0xb;
pub const PS3AV_STATUS_UNSUPPORTED_COMMAND: u32 = 0xc;
pub const PS3AV_STATUS_BUFFER_OVERFLOW: u32 = 0xd;
pub const PS3AV_STATUS_INVALID_VIDEO_PARAM: u32 = 0xe;
pub const PS3AV_STATUS_NO_SEL: u32 = 0xf;
pub const PS3AV_STATUS_INVALID_AV_PARAM: u32 = 0x10;
pub const PS3AV_STATUS_INVALID_AUDIO_PARAM: u32 = 0x11;
pub const PS3AV_STATUS_UNSUPPORTED_HDMI_MODE: u32 = 0x12;
pub const PS3AV_STATUS_NO_SYNC_HEAD: u32 = 0x13;

extern "C" {
    pub fn ps3av_set_hdr(u32, u16, *mut ps3av_send_hdr);
    pub fn ps3av_do_pkt(u32, u16, usize, *mut ps3av_send_hdr) -> i32;
    pub fn ps3av_cmd_init() -> i32;
    pub fn ps3av_cmd_fin() -> i32;
    pub fn ps3av_cmd_av_video_mute(i32, *mut u32, u32) -> i32;
    pub fn ps3av_cmd_av_video_disable_sig(u32) -> i32;
    pub fn ps3av_cmd_av_tv_mute(u32, u32) -> i32;
    pub fn ps3av_cmd_enable_event() -> i32;
    pub fn ps3av_cmd_av_hdmi_mode(u8) -> i32;
    pub fn ps3av_cmd_set_av_video_cs(*mut core::ffi::c_void, u32, i32, i32, i32, u32) -> u32;
    pub fn ps3av_cmd_set_video_mode(*mut core::ffi::c_void, u32, i32, i32, u32) -> u32;
    pub fn ps3av_cmd_video_format_black(u32, u32, u32) -> i32;
    pub fn ps3av_cmd_av_audio_mute(i32, *mut u32, u32) -> i32;
    pub fn ps3av_cmd_set_av_audio_param(*mut core::ffi::c_void, u32, *const ps3av_pkt_audio_mode, u32) -> u32;
    pub fn ps3av_cmd_set_audio_mode(*mut ps3av_pkt_audio_mode, u32, u32, u32, u32, u32, u32);
    pub fn ps3av_cmd_audio_mode(*mut ps3av_pkt_audio_mode) -> i32;
    pub fn ps3av_cmd_audio_mute(i32, *mut u32, u32) -> i32;
    pub fn ps3av_cmd_audio_active(i32, u32) -> i32;
    pub fn ps3av_cmd_avb_param(*mut ps3av_pkt_avb_param, u32) -> i32;
    pub fn ps3av_cmd_av_get_hw_conf(*mut ps3av_pkt_av_get_hw_conf) -> i32;
    pub fn ps3av_cmd_video_get_monitor_info(*mut ps3av_pkt_av_get_monitor_info, u32) -> i32;
    pub fn ps3av_set_video_mode(i32) -> i32;
    pub fn ps3av_set_audio_mode(u32, u32, u32, u32, u32) -> i32;
    pub fn ps3av_get_auto_mode() -> i32;
    pub fn ps3av_get_mode() -> i32;
    pub fn ps3av_video_mode2res(u32, *mut u32, *mut u32) -> i32;
    pub fn ps3av_video_mute(i32) -> i32;
    pub fn ps3av_audio_mute(i32) -> i32;
    pub fn ps3av_audio_mute_analog(i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
