/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * g_uvc.h  --  USB Video Class Gadget driver API
 *
 * Copyright (C) 2009-2010 Laurent Pinchart <laurent.pinchart@ideasonboard.com>
 */

// C dependencies: <linux/ioctl.h>, <linux/types.h>, and <linux/usb/ch9.h>.

pub const UVC_EVENT_FIRST: u32 = V4L2_EVENT_PRIVATE_START + 0;
pub const UVC_EVENT_CONNECT: u32 = V4L2_EVENT_PRIVATE_START + 0;
pub const UVC_EVENT_DISCONNECT: u32 = V4L2_EVENT_PRIVATE_START + 1;
pub const UVC_EVENT_STREAMON: u32 = V4L2_EVENT_PRIVATE_START + 2;
pub const UVC_EVENT_STREAMOFF: u32 = V4L2_EVENT_PRIVATE_START + 3;
pub const UVC_EVENT_SETUP: u32 = V4L2_EVENT_PRIVATE_START + 4;
pub const UVC_EVENT_DATA: u32 = V4L2_EVENT_PRIVATE_START + 5;
pub const UVC_EVENT_LAST: u32 = V4L2_EVENT_PRIVATE_START + 5;

pub const UVC_STRING_CONTROL_IDX: u32 = 0;
pub const UVC_STRING_STREAMING_IDX: u32 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvc_request_data {
    pub length: i32,
    pub data: [u8; 60],
}

#[repr(C)]
pub union uvc_event {
    pub speed: usb_device_speed,
    pub req: usb_ctrlrequest,
    pub data: uvc_request_data,
}

// _IOW('U', 1, struct uvc_request_data), from <linux/ioctl.h>.
pub const UVCIOC_SEND_RESPONSE: u64 = _IOW(b'U' as u32, 1, core::mem::size_of::<uvc_request_data>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
