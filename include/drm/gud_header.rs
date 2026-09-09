/* SPDX-License-Identifier: MIT */
/* Copyright 2020 Noralf Trønnes */

/* Translated from the Linux GUD protocol header. */

#[repr(C, packed)]
pub struct gud_display_descriptor_req {
    pub magic: __le32,
    pub version: __u8,
    pub flags: __le32,
    pub compression: __u8,
    pub max_buffer_size: __le32,
    pub min_width: __le32,
    pub max_width: __le32,
    pub min_height: __le32,
    pub max_height: __le32,
}

pub const GUD_DISPLAY_MAGIC: u32 = 0x1d50614d;
pub const GUD_DISPLAY_FLAG_STATUS_ON_SET: u32 = 1 << 0;
pub const GUD_DISPLAY_FLAG_FULL_UPDATE: u32 = 1 << 1;
pub const GUD_COMPRESSION_LZ4: u8 = 1 << 0;

#[repr(C, packed)]
pub struct gud_property_req {
    pub prop: __le16,
    pub val: __le64,
}

#[repr(C, packed)]
pub struct gud_display_mode_req {
    pub clock: __le32,
    pub hdisplay: __le16,
    pub hsync_start: __le16,
    pub hsync_end: __le16,
    pub htotal: __le16,
    pub vdisplay: __le16,
    pub vsync_start: __le16,
    pub vsync_end: __le16,
    pub vtotal: __le16,
    pub flags: __le32,
}

pub const GUD_DISPLAY_MODE_FLAG_PHSYNC: u32 = 1 << 0;
pub const GUD_DISPLAY_MODE_FLAG_NHSYNC: u32 = 1 << 1;
pub const GUD_DISPLAY_MODE_FLAG_PVSYNC: u32 = 1 << 2;
pub const GUD_DISPLAY_MODE_FLAG_NVSYNC: u32 = 1 << 3;
pub const GUD_DISPLAY_MODE_FLAG_INTERLACE: u32 = 1 << 4;
pub const GUD_DISPLAY_MODE_FLAG_DBLSCAN: u32 = 1 << 5;
pub const GUD_DISPLAY_MODE_FLAG_CSYNC: u32 = 1 << 6;
pub const GUD_DISPLAY_MODE_FLAG_PCSYNC: u32 = 1 << 7;
pub const GUD_DISPLAY_MODE_FLAG_NCSYNC: u32 = 1 << 8;
pub const GUD_DISPLAY_MODE_FLAG_HSKEW: u32 = 1 << 9;
pub const GUD_DISPLAY_MODE_FLAG_DBLCLK: u32 = 1 << 12;
pub const GUD_DISPLAY_MODE_FLAG_CLKDIV2: u32 = 1 << 13;
pub const GUD_DISPLAY_MODE_FLAG_USER_MASK: u32 = GUD_DISPLAY_MODE_FLAG_PHSYNC | GUD_DISPLAY_MODE_FLAG_NHSYNC | GUD_DISPLAY_MODE_FLAG_PVSYNC | GUD_DISPLAY_MODE_FLAG_NVSYNC | GUD_DISPLAY_MODE_FLAG_INTERLACE | GUD_DISPLAY_MODE_FLAG_DBLSCAN | GUD_DISPLAY_MODE_FLAG_CSYNC | GUD_DISPLAY_MODE_FLAG_PCSYNC | GUD_DISPLAY_MODE_FLAG_NCSYNC | GUD_DISPLAY_MODE_FLAG_HSKEW | GUD_DISPLAY_MODE_FLAG_DBLCLK | GUD_DISPLAY_MODE_FLAG_CLKDIV2;
pub const GUD_DISPLAY_MODE_FLAG_PREFERRED: u32 = 1 << 10;

#[repr(C, packed)]
pub struct gud_connector_descriptor_req {
    pub connector_type: __u8,
    pub flags: __le32,
}

pub const GUD_CONNECTOR_TYPE_PANEL: u8 = 0;
pub const GUD_CONNECTOR_TYPE_VGA: u8 = 1;
pub const GUD_CONNECTOR_TYPE_COMPOSITE: u8 = 2;
pub const GUD_CONNECTOR_TYPE_SVIDEO: u8 = 3;
pub const GUD_CONNECTOR_TYPE_COMPONENT: u8 = 4;
pub const GUD_CONNECTOR_TYPE_DVI: u8 = 5;
pub const GUD_CONNECTOR_TYPE_DISPLAYPORT: u8 = 6;
pub const GUD_CONNECTOR_TYPE_HDMI: u8 = 7;
pub const GUD_CONNECTOR_FLAGS_POLL_STATUS: u32 = 1 << 0;
pub const GUD_CONNECTOR_FLAGS_INTERLACE: u32 = 1 << 1;
pub const GUD_CONNECTOR_FLAGS_DOUBLESCAN: u32 = 1 << 2;

#[repr(C, packed)]
pub struct gud_set_buffer_req {
    pub x: __le32,
    pub y: __le32,
    pub width: __le32,
    pub height: __le32,
    pub length: __le32,
    pub compression: __u8,
    pub compressed_length: __le32,
}

#[repr(C, packed)]
pub struct gud_state_req {
    pub mode: gud_display_mode_req,
    pub format: __u8,
    pub connector: __u8,
    pub properties: [gud_property_req; 0],
}

pub const GUD_PROPERTY_TV_LEFT_MARGIN: u16 = 1;
pub const GUD_PROPERTY_TV_RIGHT_MARGIN: u16 = 2;
pub const GUD_PROPERTY_TV_TOP_MARGIN: u16 = 3;
pub const GUD_PROPERTY_TV_BOTTOM_MARGIN: u16 = 4;
pub const GUD_PROPERTY_TV_MODE: u16 = 5;
pub const GUD_PROPERTY_TV_BRIGHTNESS: u16 = 6;
pub const GUD_PROPERTY_TV_CONTRAST: u16 = 7;
pub const GUD_PROPERTY_TV_FLICKER_REDUCTION: u16 = 8;
pub const GUD_PROPERTY_TV_OVERSCAN: u16 = 9;
pub const GUD_PROPERTY_TV_SATURATION: u16 = 10;
pub const GUD_PROPERTY_TV_HUE: u16 = 11;
pub const GUD_PROPERTY_BACKLIGHT_BRIGHTNESS: u16 = 12;
pub const GUD_PROPERTY_ROTATION: u16 = 50;
pub const GUD_ROTATION_0: u32 = 1 << 0;
pub const GUD_ROTATION_90: u32 = 1 << 1;
pub const GUD_ROTATION_180: u32 = 1 << 2;
pub const GUD_ROTATION_270: u32 = 1 << 3;
pub const GUD_ROTATION_REFLECT_X: u32 = 1 << 4;
pub const GUD_ROTATION_REFLECT_Y: u32 = 1 << 5;
pub const GUD_ROTATION_MASK: u32 = GUD_ROTATION_0 | GUD_ROTATION_90 | GUD_ROTATION_180 | GUD_ROTATION_270 | GUD_ROTATION_REFLECT_X | GUD_ROTATION_REFLECT_Y;

pub const GUD_REQ_GET_STATUS: u8 = 0x00;
pub const GUD_STATUS_OK: u8 = 0x00;
pub const GUD_STATUS_BUSY: u8 = 0x01;
pub const GUD_STATUS_REQUEST_NOT_SUPPORTED: u8 = 0x02;
pub const GUD_STATUS_PROTOCOL_ERROR: u8 = 0x03;
pub const GUD_STATUS_INVALID_PARAMETER: u8 = 0x04;
pub const GUD_STATUS_ERROR: u8 = 0x05;
pub const GUD_REQ_GET_DESCRIPTOR: u8 = 0x01;
pub const GUD_REQ_GET_FORMATS: u8 = 0x40;
pub const GUD_FORMATS_MAX_NUM: usize = 32;
pub const GUD_PIXEL_FORMAT_R1: u8 = 0x01;
pub const GUD_PIXEL_FORMAT_R8: u8 = 0x08;
pub const GUD_PIXEL_FORMAT_XRGB1111: u8 = 0x20;
pub const GUD_PIXEL_FORMAT_RGB332: u8 = 0x30;
pub const GUD_PIXEL_FORMAT_RGB565: u8 = 0x40;
pub const GUD_PIXEL_FORMAT_RGB888: u8 = 0x50;
pub const GUD_PIXEL_FORMAT_XRGB8888: u8 = 0x80;
pub const GUD_PIXEL_FORMAT_ARGB8888: u8 = 0x81;
pub const GUD_REQ_GET_PROPERTIES: u8 = 0x41;
pub const GUD_PROPERTIES_MAX_NUM: usize = 32;
pub const GUD_REQ_GET_CONNECTORS: u8 = 0x50;
pub const GUD_CONNECTORS_MAX_NUM: usize = 32;
pub const GUD_REQ_GET_CONNECTOR_PROPERTIES: u8 = 0x51;
pub const GUD_CONNECTOR_PROPERTIES_MAX_NUM: usize = 32;
pub const GUD_REQ_GET_CONNECTOR_TV_MODE_VALUES: u8 = 0x52;
pub const GUD_CONNECTOR_TV_MODE_NAME_LEN: usize = 16;
pub const GUD_CONNECTOR_TV_MODE_MAX_NUM: usize = 16;
pub const GUD_REQ_SET_CONNECTOR_FORCE_DETECT: u8 = 0x53;
pub const GUD_REQ_GET_CONNECTOR_STATUS: u8 = 0x54;
pub const GUD_CONNECTOR_STATUS_DISCONNECTED: u8 = 0x00;
pub const GUD_CONNECTOR_STATUS_CONNECTED: u8 = 0x01;
pub const GUD_CONNECTOR_STATUS_UNKNOWN: u8 = 0x02;
pub const GUD_CONNECTOR_STATUS_CONNECTED_MASK: u8 = 0x03;
pub const GUD_CONNECTOR_STATUS_CHANGED: u8 = 1 << 7;
pub const GUD_REQ_GET_CONNECTOR_MODES: u8 = 0x55;
pub const GUD_CONNECTOR_MAX_NUM_MODES: usize = 128;
pub const GUD_REQ_GET_CONNECTOR_EDID: u8 = 0x56;
pub const GUD_CONNECTOR_MAX_EDID_LEN: usize = 2048;
pub const GUD_REQ_SET_BUFFER: u8 = 0x60;
pub const GUD_REQ_SET_STATE_CHECK: u8 = 0x61;
pub const GUD_REQ_SET_STATE_COMMIT: u8 = 0x62;
pub const GUD_REQ_SET_CONTROLLER_ENABLE: u8 = 0x63;
pub const GUD_REQ_SET_DISPLAY_ENABLE: u8 = 0x64;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
