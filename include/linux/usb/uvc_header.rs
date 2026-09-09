/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  v4l2 uvc internal API header
 *
 *  Some commonly needed functions for uvc drivers
 */

/* GUIDs. The C initializer macros are represented as typed byte arrays. */
pub const UVC_GUID_UVC_CAMERA: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
pub const UVC_GUID_UVC_OUTPUT: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2];
pub const UVC_GUID_UVC_MEDIA_TRANSPORT_INPUT: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3];
pub const UVC_GUID_UVC_PROCESSING: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1];
pub const UVC_GUID_UVC_SELECTOR: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2];
pub const UVC_GUID_EXT_GPIO_CONTROLLER: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,3];
pub const UVC_GUID_CHROMEOS_XU: [u8; 16] = [0x24,0xe9,0xd7,0x74,0xc9,0x49,0x45,0x4a,0x98,0xa3,0xc8,0x07,0x7e,0x05,0x1c,0xa3];
pub const UVC_GUID_MSXU_1_5: [u8; 16] = [0xdc,0x95,0x3f,0x0f,0x32,0x26,0x4e,0x4c,0x92,0xc9,0xa0,0x47,0x82,0xf4,0x3b,0xc8];
pub const UVC_GUID_LOGITECH_MOTOR_CONTROL_V1: [u8; 16] = [0x82,0x06,0x61,0x63,0x70,0x50,0xab,0x49,0xb8,0xcc,0xb3,0x85,0x5e,0x8d,0x22,0x56];
pub const UVC_GUID_LOGITECH_PERIPHERAL: [u8; 16] = [0x21,0x2d,0xe5,0xff,0x30,0x80,0x2c,0x4e,0x82,0xd9,0xf5,0x87,0xd0,0x05,0x40,0xbd];
pub const UVC_GUID_LOGITECH_USER_HW_CONTROL_V1: [u8; 16] = [0x82,0x06,0x61,0x63,0x70,0x50,0xab,0x49,0xb8,0xcc,0xb3,0x85,0x5e,0x8d,0x22,0x1f];

pub const UVC_MSXU_CONTROL_FOCUS: u32 = 0x01;
pub const UVC_MSXU_CONTROL_EXPOSURE: u32 = 0x02;
pub const UVC_MSXU_CONTROL_EVCOMPENSATION: u32 = 0x03;
pub const UVC_MSXU_CONTROL_WHITEBALANCE: u32 = 0x04;
pub const UVC_MSXU_CONTROL_FACE_AUTHENTICATION: u32 = 0x06;
pub const UVC_MSXU_CONTROL_CAMERA_EXTRINSICS: u32 = 0x07;
pub const UVC_MSXU_CONTROL_CAMERA_INTRINSICS: u32 = 0x08;
pub const UVC_MSXU_CONTROL_METADATA: u32 = 0x09;
pub const UVC_MSXU_CONTROL_IR_TORCH: u32 = 0x0a;
pub const UVC_MSXU_CONTROL_DIGITALWINDOW: u32 = 0x0b;
pub const UVC_MSXU_CONTROL_DIGITALWINDOW_CONFIG: u32 = 0x0c;
pub const UVC_MSXU_CONTROL_VIDEO_HDR: u32 = 0x0d;
pub const UVC_MSXU_CONTROL_FRAMERATE_THROTTLE: u32 = 0x0e;
pub const UVC_MSXU_CONTROL_FIELDOFVIEW2_CONFIG: u32 = 0x0f;
pub const UVC_MSXU_CONTROL_FIELDOFVIEW2: u32 = 0x10;
pub const UVC_CROSXU_CONTROL_IQ_PROFILE: u32 = 0x04;

const fn guid4(a: [u8; 4]) -> [u8; 16] { [a[0],a[1],a[2],a[3],0,0,0x10,0,0x80,0,0,0xaa,0,0x38,0x9b,0x71] }
pub const UVC_GUID_FORMAT_MJPEG: [u8;16] = guid4(*b"MJPG");
pub const UVC_GUID_FORMAT_YUY2: [u8;16] = guid4(*b"YUY2");
pub const UVC_GUID_FORMAT_YUY2_ISIGHT: [u8;16] = ['Y' as u8,'U' as u8,'Y' as u8,'2' as u8,0,0,0x10,0,0x80,0,0,0,0,0x38,0x9b,0x71];
pub const UVC_GUID_FORMAT_NV12: [u8;16] = guid4(*b"NV12");
pub const UVC_GUID_FORMAT_YV12: [u8;16] = guid4(*b"YV12");
pub const UVC_GUID_FORMAT_I420: [u8;16] = guid4(*b"I420");
pub const UVC_GUID_FORMAT_UYVY: [u8;16] = guid4(*b"UYVY");
pub const UVC_GUID_FORMAT_Y800: [u8;16] = guid4(*b"Y800");
pub const UVC_GUID_FORMAT_Y8: [u8;16] = guid4(*b"Y8  ");
pub const UVC_GUID_FORMAT_Y10: [u8;16] = guid4(*b"Y10 ");
pub const UVC_GUID_FORMAT_Y12: [u8;16] = guid4(*b"Y12 ");
pub const UVC_GUID_FORMAT_Y16: [u8;16] = guid4(*b"Y16 ");
pub const UVC_GUID_FORMAT_BY8: [u8;16] = guid4(*b"BY8 ");
pub const UVC_GUID_FORMAT_BA81: [u8;16] = guid4(*b"BA81");
pub const UVC_GUID_FORMAT_GBRG: [u8;16] = guid4(*b"GBRG");
pub const UVC_GUID_FORMAT_GRBG: [u8;16] = guid4(*b"GRBG");
pub const UVC_GUID_FORMAT_RGGB: [u8;16] = guid4(*b"RGGB");
pub const UVC_GUID_FORMAT_BG16: [u8;16] = guid4(*b"BG16");
pub const UVC_GUID_FORMAT_GB16: [u8;16] = guid4(*b"GB16");
pub const UVC_GUID_FORMAT_RG16: [u8;16] = guid4(*b"RG16");
pub const UVC_GUID_FORMAT_GR16: [u8;16] = guid4(*b"GR16");
pub const UVC_GUID_FORMAT_RGBP: [u8;16] = guid4(*b"RGBP");
pub const UVC_GUID_FORMAT_M420: [u8;16] = guid4(*b"M420");
pub const UVC_GUID_FORMAT_P010: [u8;16] = guid4(*b"P010");
pub const UVC_GUID_FORMAT_H264: [u8;16] = guid4(*b"H264");
pub const UVC_GUID_FORMAT_H265: [u8;16] = guid4(*b"H265");
pub const UVC_GUID_FORMAT_Y8I: [u8;16] = guid4(*b"Y8I ");
pub const UVC_GUID_FORMAT_Y12I: [u8;16] = guid4(*b"Y12I");
pub const UVC_GUID_FORMAT_Y16I: [u8;16] = guid4(*b"Y16I");
pub const UVC_GUID_FORMAT_Z16: [u8;16] = guid4(*b"Z16 ");
pub const UVC_GUID_FORMAT_RW10: [u8;16] = guid4(*b"RW10");
pub const UVC_GUID_FORMAT_CNF4: [u8;16] = guid4(*b"C   ");
pub const UVC_GUID_FORMAT_HEVC: [u8;16] = guid4(*b"HEVC");

pub const UVC_GUID_FORMAT_BGR3: [u8;16] = [0x7d,0xeb,0x36,0xe4,0x4f,0x52,0xce,0x11,0x9f,0x53,0,0x20,0xaf,0x0b,0xa7,0x70];
pub const UVC_GUID_FORMAT_BGR4: [u8;16] = [0x7e,0xeb,0x36,0xe4,0x4f,0x52,0xce,0x11,0x9f,0x53,0,0x20,0xaf,0x0b,0xa7,0x70];
pub const UVC_GUID_FORMAT_INVZ: [u8;16] = [b'I',b'N',b'V',b'Z',0x90,0x2d,0x58,0x4a,0x92,0x0b,0x77,0x3f,0x1f,0x2c,0x55,0x6b];
pub const UVC_GUID_FORMAT_INZI: [u8;16] = [b'I',b'N',b'Z',b'I',0x66,0x1a,0x42,0xa2,0x90,0x65,0xd0,0x18,0x14,0xa8,0xef,0x8a];
pub const UVC_GUID_FORMAT_INVI: [u8;16] = [b'I',b'N',b'V',b'I',0xdb,0x57,0x49,0x5e,0x8e,0x3f,0xf4,0x79,0x53,0x2b,0x94,0x6f];
pub const UVC_GUID_FORMAT_D3DFMT_L8: [u8;16] = [0x32,0,0,0,0,0,0x10,0,0x80,0,0,0xaa,0,0x38,0x9b,0x71];
pub const UVC_GUID_FORMAT_D3DFMT_R5G6B5: [u8;16] = [0x7b,0xeb,0x36,0xe4,0x4f,0x52,0xce,0x11,0x9f,0x53,0,0x20,0xaf,0x0b,0xa7,0x70];
pub const UVC_GUID_FORMAT_KSMEDIA_L8_IR: [u8;16] = [0x32,0,0,0,2,0,0x10,0,0x80,0,0,0xaa,0,0x38,0x9b,0x71];

#[repr(C)]
pub struct uvc_format_desc { pub guid: [u8; 16], pub fcc: u32 }

unsafe extern "C" { pub fn uvc_format_by_guid(guid: *const u8) -> *const uvc_format_desc; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
