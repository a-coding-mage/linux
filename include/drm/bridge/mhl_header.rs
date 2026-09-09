/* SPDX-License-Identifier: GPL-2.0-only */
/* Defines for Mobile High-Definition Link (MHL) interface. */

// Dependency intent: Linux types such as __be16, __le16, and u8 are supplied externally.

pub const MHL_DCAP_DEV_STATE: u32 = 0;
pub const MHL_DCAP_MHL_VERSION: u32 = 1;
pub const MHL_DCAP_CAT: u32 = 2;
pub const MHL_DCAP_ADOPTER_ID_H: u32 = 3;
pub const MHL_DCAP_ADOPTER_ID_L: u32 = 4;
pub const MHL_DCAP_VID_LINK_MODE: u32 = 5;
pub const MHL_DCAP_AUD_LINK_MODE: u32 = 6;
pub const MHL_DCAP_VIDEO_TYPE: u32 = 7;
pub const MHL_DCAP_LOG_DEV_MAP: u32 = 8;
pub const MHL_DCAP_BANDWIDTH: u32 = 9;
pub const MHL_DCAP_FEATURE_FLAG: u32 = 10;
pub const MHL_DCAP_DEVICE_ID_H: u32 = 11;
pub const MHL_DCAP_DEVICE_ID_L: u32 = 12;
pub const MHL_DCAP_SCRATCHPAD_SIZE: u32 = 13;
pub const MHL_DCAP_INT_STAT_SIZE: u32 = 14;
pub const MHL_DCAP_RESERVED: u32 = 15;
pub const MHL_DCAP_SIZE: u32 = 16;

pub const MHL_DCAP_CAT_SINK: u32 = 0x01; pub const MHL_DCAP_CAT_SOURCE: u32 = 0x02;
pub const MHL_DCAP_CAT_POWER: u32 = 0x10;
pub const MHL_DCAP_CAT_PLIM: fn(u32) -> u32 = |x| x << 5;
pub const MHL_DCAP_VID_LINK_RGB444: u32 = 0x01; pub const MHL_DCAP_VID_LINK_YCBCR444: u32 = 0x02;
pub const MHL_DCAP_VID_LINK_YCBCR422: u32 = 0x04; pub const MHL_DCAP_VID_LINK_PPIXEL: u32 = 0x08;
pub const MHL_DCAP_VID_LINK_ISLANDS: u32 = 0x10; pub const MHL_DCAP_VID_LINK_VGA: u32 = 0x20;
pub const MHL_DCAP_VID_LINK_16BPP: u32 = 0x40;
pub const MHL_DCAP_AUD_LINK_2CH: u32 = 0x01; pub const MHL_DCAP_AUD_LINK_8CH: u32 = 0x02;
pub const MHL_DCAP_VT_GRAPHICS: u32 = 0x00; pub const MHL_DCAP_VT_PHOTO: u32 = 0x02;
pub const MHL_DCAP_VT_CINEMA: u32 = 0x04; pub const MHL_DCAP_VT_GAMES: u32 = 0x08; pub const MHL_DCAP_SUPP_VT: u32 = 0x80;
pub const MHL_DCAP_LD_DISPLAY: u32 = 0x01; pub const MHL_DCAP_LD_VIDEO: u32 = 0x02; pub const MHL_DCAP_LD_AUDIO: u32 = 0x04;
pub const MHL_DCAP_LD_MEDIA: u32 = 0x08; pub const MHL_DCAP_LD_TUNER: u32 = 0x10; pub const MHL_DCAP_LD_RECORD: u32 = 0x20;
pub const MHL_DCAP_LD_SPEAKER: u32 = 0x40; pub const MHL_DCAP_LD_GUI: u32 = 0x80; pub const MHL_DCAP_LD_ALL: u32 = 0xff;
pub const MHL_DCAP_FEATURE_RCP_SUPPORT: u32 = 0x01; pub const MHL_DCAP_FEATURE_RAP_SUPPORT: u32 = 0x02;
pub const MHL_DCAP_FEATURE_SP_SUPPORT: u32 = 0x04; pub const MHL_DCAP_FEATURE_UCP_SEND_SUPPOR: u32 = 0x08;
pub const MHL_DCAP_FEATURE_UCP_RECV_SUPPORT: u32 = 0x10; pub const MHL_DCAP_FEATURE_RBP_SUPPORT: u32 = 0x40;

pub const MHL_XDC_ECBUS_SPEEDS: u32 = 0; pub const MHL_XDC_TMDS_SPEEDS: u32 = 1; pub const MHL_XDC_ECBUS_ROLES: u32 = 2;
pub const MHL_XDC_LOG_DEV_MAPX: u32 = 3; pub const MHL_XDC_SIZE: u32 = 4;
pub const MHL_XDC_ECBUS_S_075: u32 = 1; pub const MHL_XDC_ECBUS_S_8BIT: u32 = 2; pub const MHL_XDC_ECBUS_S_12BIT: u32 = 4;
pub const MHL_XDC_ECBUS_D_150: u32 = 0x10; pub const MHL_XDC_ECBUS_D_8BIT: u32 = 0x20;
pub const MHL_XDC_TMDS_000: u32 = 0; pub const MHL_XDC_TMDS_150: u32 = 1; pub const MHL_XDC_TMDS_300: u32 = 2; pub const MHL_XDC_TMDS_600: u32 = 4;
pub const MHL_XDC_DEV_HOST: u32 = 1; pub const MHL_XDC_DEV_DEVICE: u32 = 2; pub const MHL_XDC_DEV_CHARGER: u32 = 4;
pub const MHL_XDC_HID_HOST: u32 = 8; pub const MHL_XDC_HID_DEVICE: u32 = 0x10; pub const MHL_XDC_LD_PHONE: u32 = 1;

pub const MHL_DST_CONNECTED_RDY: u32 = 0; pub const MHL_DST_LINK_MODE: u32 = 1; pub const MHL_DST_VERSION: u32 = 2; pub const MHL_DST_SIZE: u32 = 3;
pub const MHL_DST_OFFSET: u32 = 0x30;
pub const MHL_DST_REG_CONNECTED_RDY: u32 = MHL_DST_OFFSET + MHL_DST_CONNECTED_RDY;
pub const MHL_DST_REG_LINK_MODE: u32 = MHL_DST_OFFSET + MHL_DST_LINK_MODE;
pub const MHL_DST_REG_VERSION: u32 = MHL_DST_OFFSET + MHL_DST_VERSION;
pub const MHL_DST_CONN_DCAP_RDY: u32 = 1; pub const MHL_DST_CONN_XDEVCAPP_SUPP: u32 = 2; pub const MHL_DST_CONN_POW_STAT: u32 = 4; pub const MHL_DST_CONN_PLIM_STAT_MASK: u32 = 0x38;
pub const MHL_DST_LM_CLK_MODE_MASK: u32 = 7; pub const MHL_DST_LM_CLK_MODE_PACKED_PIXEL: u32 = 2; pub const MHL_DST_LM_CLK_MODE_NORMAL: u32 = 3; pub const MHL_DST_LM_PATH_EN_MASK: u32 = 8; pub const MHL_DST_LM_PATH_ENABLED: u32 = 8; pub const MHL_DST_LM_PATH_DISABLED: u32 = 0; pub const MHL_DST_LM_MUTED_MASK: u32 = 0x10;

pub const MHL_XDS_CURR_ECBUS_MODE: u32 = 0; pub const MHL_XDS_AVLINK_MODE_STATUS: u32 = 1; pub const MHL_XDS_AVLINK_MODE_CONTROL: u32 = 2; pub const MHL_XDS_MULTI_SINK_STATUS: u32 = 3; pub const MHL_XDS_SIZE: u32 = 4;
pub const MHL_XDS_OFFSET: u32 = 0x90;
pub const MHL_XDS_REG_CURR_ECBUS_MODE: u32 = 0x90; pub const MHL_XDS_REG_AVLINK_MODE_STATUS: u32 = 0x91; pub const MHL_XDS_REG_AVLINK_MODE_CONTROL: u32 = 0x92; pub const MHL_XDS_REG_MULTI_SINK_STATUS: u32 = 0x93;
pub const MHL_XDS_SLOT_MODE_8BIT: u32 = 0; pub const MHL_XDS_SLOT_MODE_6BIT: u32 = 1; pub const MHL_XDS_ECBUS_S: u32 = 4; pub const MHL_XDS_ECBUS_D: u32 = 8;
pub const MHL_XDS_LINK_CLOCK_75MHZ: u32 = 0; pub const MHL_XDS_LINK_CLOCK_150MHZ: u32 = 0x10; pub const MHL_XDS_LINK_CLOCK_300MHZ: u32 = 0x20; pub const MHL_XDS_LINK_CLOCK_600MHZ: u32 = 0x30;
pub const MHL_XDS_LINK_STATUS_NO_SIGNAL: u32 = 0; pub const MHL_XDS_LINK_STATUS_CRU_LOCKED: u32 = 1; pub const MHL_XDS_LINK_STATUS_TMDS_NORMAL: u32 = 2; pub const MHL_XDS_LINK_STATUS_TMDS_RESERVED: u32 = 3;
pub const MHL_XDS_LINK_RATE_1_5_GBPS: u32 = 0; pub const MHL_XDS_LINK_RATE_3_0_GBPS: u32 = 1; pub const MHL_XDS_LINK_RATE_6_0_GBPS: u32 = 2; pub const MHL_XDS_ATT_CAPABLE: u32 = 8;
pub const MHL_XDS_SINK_STATUS_1_HPD_LOW: u32 = 0; pub const MHL_XDS_SINK_STATUS_1_HPD_HIGH: u32 = 1; pub const MHL_XDS_SINK_STATUS_2_HPD_LOW: u32 = 0; pub const MHL_XDS_SINK_STATUS_2_HPD_HIGH: u32 = 4; pub const MHL_XDS_SINK_STATUS_3_HPD_LOW: u32 = 0; pub const MHL_XDS_SINK_STATUS_3_HPD_HIGH: u32 = 0x10; pub const MHL_XDS_SINK_STATUS_4_HPD_LOW: u32 = 0; pub const MHL_XDS_SINK_STATUS_4_HPD_HIGH: u32 = 0x40;

pub const MHL_INT_RCHANGE: u32 = 0; pub const MHL_INT_DCHANGE: u32 = 1; pub const MHL_INT_SIZE: u32 = 2; pub const MHL_INT_OFFSET: u32 = 0x20;
pub const MHL_INT_REG_RCHANGE: u32 = 0x20; pub const MHL_INT_REG_DCHANGE: u32 = 0x21;
pub const MHL_INT_RC_DCAP_CHG: u32 = 1; pub const MHL_INT_RC_DSCR_CHG: u32 = 2; pub const MHL_INT_RC_REQ_WRT: u32 = 4; pub const MHL_INT_RC_GRT_WRT: u32 = 8; pub const MHL_INT_RC_3D_REQ: u32 = 0x10; pub const MHL_INT_RC_FEAT_REQ: u32 = 0x20; pub const MHL_INT_RC_FEAT_COMPLETE: u32 = 0x40; pub const MHL_INT_DC_EDID_CHG: u32 = 2;

pub const MHL_ACK: u32 = 0x33; pub const MHL_NACK: u32 = 0x34; pub const MHL_ABORT: u32 = 0x35; pub const MHL_WRITE_STAT: u32 = 0xe0; pub const MHL_SET_INT: u32 = 0x60; pub const MHL_READ_DEVCAP_REG: u32 = 0x61; pub const MHL_GET_STATE: u32 = 0x62; pub const MHL_GET_VENDOR_ID: u32 = 0x63; pub const MHL_SET_HPD: u32 = 0x64; pub const MHL_CLR_HPD: u32 = 0x65; pub const MHL_SET_CAP_ID: u32 = 0x66; pub const MHL_GET_CAP_ID: u32 = 0x67; pub const MHL_MSC_MSG: u32 = 0x68; pub const MHL_GET_SC1_ERRORCODE: u32 = 0x69; pub const MHL_GET_DDC_ERRORCODE: u32 = 0x6a; pub const MHL_GET_MSC_ERRORCODE: u32 = 0x6b; pub const MHL_WRITE_BURST: u32 = 0x6c; pub const MHL_GET_SC3_ERRORCODE: u32 = 0x6d; pub const MHL_WRITE_XSTAT: u32 = 0x70; pub const MHL_READ_XDEVCAP_REG: u32 = 0x71; pub const MHL_READ_EDID_BLOCK: u32 = 0x72; pub const MHL_SEND_3D_REQ_OR_FEAT_REQ: u32 = 0x73; pub const MHL_READ_DEVCAP: u32 = 0x74; pub const MHL_READ_XDEVCAP: u32 = 0x75;

pub const MHL_MSC_MSG_RCP: u32 = 0x10; pub const MHL_MSC_MSG_RCPK: u32 = 0x11; pub const MHL_MSC_MSG_RCPE: u32 = 0x12; pub const MHL_MSC_MSG_RAP: u32 = 0x20; pub const MHL_MSC_MSG_RAPK: u32 = 0x21; pub const MHL_MSC_MSG_RBP: u32 = 0x22; pub const MHL_MSC_MSG_RBPK: u32 = 0x23; pub const MHL_MSC_MSG_RBPE: u32 = 0x24; pub const MHL_MSC_MSG_UCP: u32 = 0x30; pub const MHL_MSC_MSG_UCPK: u32 = 0x31; pub const MHL_MSC_MSG_UCPE: u32 = 0x32; pub const MHL_MSC_MSG_RUSB: u32 = 0x40; pub const MHL_MSC_MSG_RUSBK: u32 = 0x41; pub const MHL_MSC_MSG_RHID: u32 = 0x42; pub const MHL_MSC_MSG_RHIDK: u32 = 0x43; pub const MHL_MSC_MSG_ATT: u32 = 0x50; pub const MHL_MSC_MSG_ATTK: u32 = 0x51; pub const MHL_MSC_MSG_BIST_TRIGGER: u32 = 0x60; pub const MHL_MSC_MSG_BIST_REQUEST_STAT: u32 = 0x61; pub const MHL_MSC_MSG_BIST_READY: u32 = 0x62; pub const MHL_MSC_MSG_BIST_STOP: u32 = 0x63;

pub const MHL_RAP_POLL: u32 = 0; pub const MHL_RAP_CONTENT_ON: u32 = 0x10; pub const MHL_RAP_CONTENT_OFF: u32 = 0x11; pub const MHL_RAP_CBUS_MODE_DOWN: u32 = 0x20; pub const MHL_RAP_CBUS_MODE_UP: u32 = 0x21;
pub const MHL_RAPK_NO_ERR: u32 = 0; pub const MHL_RAPK_UNRECOGNIZED: u32 = 1; pub const MHL_RAPK_UNSUPPORTED: u32 = 2; pub const MHL_RAPK_BUSY: u32 = 3;
pub const MHL_RCP_KEY_RELEASED_MASK: u32 = 0x80; pub const MHL_RCP_KEY_ID_MASK: u32 = 0x7f;
pub const MHL_RCPE_STATUS_NO_ERROR: u32 = 0; pub const MHL_RCPE_STATUS_INEFFECTIVE_KEY_CODE: u32 = 1; pub const MHL_RCPE_STATUS_BUSY: u32 = 2;
pub const MHL_RBPE_STATUS_NO_ERROR: u32 = 0; pub const MHL_RBPE_STATUS_INEFFECTIVE_BUTTON_CODE: u32 = 1; pub const MHL_RBPE_STATUS_BUSY: u32 = 2;
pub const MHL_UCPE_STATUS_NO_ERROR: u32 = 0; pub const MHL_UCPE_STATUS_INEFFECTIVE_KEY_CODE: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mhl_burst_id { MHL_BURST_ID_3D_VIC = 0x10, MHL_BURST_ID_3D_DTD = 0x11, MHL_BURST_ID_HEV_VIC = 0x20, MHL_BURST_ID_HEV_DTDA = 0x21, MHL_BURST_ID_HEV_DTDB = 0x22, MHL_BURST_ID_VC_ASSIGN = 0x38, MHL_BURST_ID_VC_CONFIRM = 0x39, MHL_BURST_ID_AUD_DELAY = 0x40, MHL_BURST_ID_ADT_BURSTID = 0x41, MHL_BURST_ID_BIST_SETUP = 0x51, MHL_BURST_ID_BIST_RETURN_STAT = 0x52, MHL_BURST_ID_EMSC_SUPPORT = 0x61, MHL_BURST_ID_HID_PAYLOAD = 0x62, MHL_BURST_ID_BLK_RCV_BUFFER_INFO = 0x63, MHL_BURST_ID_BITS_PER_PIXEL_FMT = 0x64 }

#[repr(C, packed)] pub struct mhl_burst_blk_rcv_buffer_info { pub id: __be16, pub size: __le16 }
#[repr(C, packed)] pub struct mhl3_burst_header { pub id: __be16, pub checksum: u8, pub total_entries: u8, pub sequence_index: u8 }
#[repr(C, packed)] pub struct mhl_burst_bits_per_pixel_fmt { pub hdr: mhl3_burst_header, pub num_entries: u8, pub desc: [mhl_burst_bits_per_pixel_fmt_desc; 0] }
#[repr(C, packed)] pub struct mhl_burst_bits_per_pixel_fmt_desc { pub stream_id: u8, pub pixel_format: u8 }
#[repr(C, packed)] pub struct mhl_burst_emsc_support { pub hdr: mhl3_burst_header, pub num_entries: u8, pub burst_id: [__be16; 0] }
#[repr(C, packed)] pub struct mhl_burst_audio_descr { pub hdr: mhl3_burst_header, pub flags: u8, pub short_desc: [u8; 9] }

pub const MHL3_IEEE_OUI: u32 = 0x7ca61d; pub const MHL3_INFOFRAME_SIZE: u32 = 15;
#[repr(C)] #[derive(Copy, Clone)] pub enum mhl3_video_format { MHL3_VIDEO_FORMAT_NONE, MHL3_VIDEO_FORMAT_3D, MHL3_VIDEO_FORMAT_MULTI_VIEW, MHL3_VIDEO_FORMAT_DUAL_3D }
#[repr(C)] #[derive(Copy, Clone)] pub enum mhl3_3d_format_type { MHL3_3D_FORMAT_TYPE_FS, MHL3_3D_FORMAT_TYPE_TB, MHL3_3D_FORMAT_TYPE_LR, MHL3_3D_FORMAT_TYPE_FS_TB, MHL3_3D_FORMAT_TYPE_FS_LR, MHL3_3D_FORMAT_TYPE_TB_LR }
#[repr(C)] pub struct mhl3_infoframe { pub version: std::ffi::c_uchar, pub video_format: mhl3_video_format, pub format_type: mhl3_3d_format_type, pub sep_audio: bool, pub hev_format: std::ffi::c_int, pub av_delay: std::ffi::c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
