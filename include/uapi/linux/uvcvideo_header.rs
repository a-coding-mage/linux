/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
// Translation of linux/uvcvideo.h. C header guards and includes omitted.

/* Dynamic controls */

/* Data types for UVC control data */
pub const UVC_CTRL_DATA_TYPE_RAW: u32 = 0;
pub const UVC_CTRL_DATA_TYPE_SIGNED: u32 = 1;
pub const UVC_CTRL_DATA_TYPE_UNSIGNED: u32 = 2;
pub const UVC_CTRL_DATA_TYPE_BOOLEAN: u32 = 3;
pub const UVC_CTRL_DATA_TYPE_ENUM: u32 = 4;
pub const UVC_CTRL_DATA_TYPE_BITMASK: u32 = 5;
pub const UVC_CTRL_DATA_TYPE_RECT: u32 = 6;

/* Control flags */
pub const UVC_CTRL_FLAG_SET_CUR: u32 = 1 << 0;
pub const UVC_CTRL_FLAG_GET_CUR: u32 = 1 << 1;
pub const UVC_CTRL_FLAG_GET_MIN: u32 = 1 << 2;
pub const UVC_CTRL_FLAG_GET_MAX: u32 = 1 << 3;
pub const UVC_CTRL_FLAG_GET_RES: u32 = 1 << 4;
pub const UVC_CTRL_FLAG_GET_DEF: u32 = 1 << 5;
/* Control should be saved at suspend and restored at resume. */
pub const UVC_CTRL_FLAG_RESTORE: u32 = 1 << 6;
/* Control can be updated by the camera. */
pub const UVC_CTRL_FLAG_AUTO_UPDATE: u32 = 1 << 7;
/* Control supports asynchronous reporting */
pub const UVC_CTRL_FLAG_ASYNCHRONOUS: u32 = 1 << 8;

pub const UVC_CTRL_FLAG_GET_RANGE: u32 = UVC_CTRL_FLAG_GET_CUR
    | UVC_CTRL_FLAG_GET_MIN
    | UVC_CTRL_FLAG_GET_MAX
    | UVC_CTRL_FLAG_GET_RES
    | UVC_CTRL_FLAG_GET_DEF;

pub const UVC_MENU_NAME_LEN: usize = 32;

/* V4L2 driver-specific controls. V4L2_CID_USER_UVC_BASE is supplied externally. */
pub const V4L2_CID_UVC_REGION_OF_INTEREST_RECT: u32 = V4L2_CID_USER_UVC_BASE + 1;
pub const V4L2_CID_UVC_REGION_OF_INTEREST_AUTO: u32 = V4L2_CID_USER_UVC_BASE + 2;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_EXPOSURE: u32 = 1 << 0;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_IRIS: u32 = 1 << 1;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_WHITE_BALANCE: u32 = 1 << 2;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_FOCUS: u32 = 1 << 3;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_FACE_DETECT: u32 = 1 << 4;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_DETECT_AND_TRACK: u32 = 1 << 5;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_IMAGE_STABILIZATION: u32 = 1 << 6;
pub const V4L2_UVC_REGION_OF_INTEREST_AUTO_HIGHER_QUALITY: u32 = 1 << 7;

#[repr(C)]
pub struct uvc_menu_info {
    pub value: u32,
    pub name: [u8; UVC_MENU_NAME_LEN],
}

#[repr(C)]
pub struct uvc_xu_control_mapping {
    pub id: u32,
    pub name: [u8; 32],
    pub entity: [u8; 16],
    pub selector: u8,
    pub size: u8,
    pub offset: u8,
    pub v4l2_type: u32,
    pub data_type: u32,
    /* __user pointer */
    pub menu_info: *mut uvc_menu_info,
    pub menu_count: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
pub struct uvc_xu_control_query {
    pub unit: u8,
    pub selector: u8,
    /* Video Class-Specific Request Code, defined in linux/usb/video.h A.8. */
    pub query: u8,
    pub size: u16,
    /* __user pointer */
    pub data: *mut u8,
}

/* _IOWR('u', 0x20, struct uvc_xu_control_mapping) */
pub const UVCIOC_CTRL_MAP: u32 = _IOWR_UVC_CTRL_MAP;
/* _IOWR('u', 0x21, struct uvc_xu_control_query) */
pub const UVCIOC_CTRL_QUERY: u32 = _IOWR_UVC_CTRL_QUERY;

/* Metadata node */

/**
 * struct uvc_meta_buf - metadata buffer building block
 * @ns: system timestamp of the payload in nanoseconds
 * @sof: USB Frame Number
 * @length: length of the payload header
 * @flags: payload header flags
 * @buf: optional device-specific header data
 *
 * UVC metadata nodes fill buffers with possibly multiple instances of this
 * struct. The first two fields are added by the driver, they can be used for
 * clock synchronisation. The rest is an exact copy of a UVC payload header.
 * Only complete objects with complete buffers are included. Therefore it's
 * always sizeof(meta->ns) + sizeof(meta->sof) + meta->length bytes large.
 */
#[repr(C, packed)]
pub struct uvc_meta_buf {
    pub ns: u64,
    pub sof: u16,
    pub length: u8,
    pub flags: u8,
    pub buf: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
