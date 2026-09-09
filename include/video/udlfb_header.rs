/* SPDX-License-Identifier: GPL-2.0 */

/*
 * TODO: Propose standard fb.h ioctl for reporting damage,
 * using _IOWR() and one of the existing area structs from fb.h
 * Consider these ioctls deprecated, but they're still used by the
 * DisplayLink X server as yet - need both to be modified in tandem
 * when new ioctl(s) are ready.
 */
pub const DLFB_IOCTL_RETURN_EDID: u32 = 0xAD;
pub const DLFB_IOCTL_REPORT_DAMAGE: u32 = 0xAA;

#[repr(C)]
pub struct dloarea {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub x2: ::core::ffi::c_int,
    pub y2: ::core::ffi::c_int,
}

#[repr(C)]
pub struct urb_node {
    pub entry: list_head,
    pub dlfb: *mut dlfb_data,
    pub urb: *mut urb,
}

#[repr(C)]
pub struct urb_list {
    pub list: list_head,
    pub lock: spinlock_t,
    pub limit_sem: semaphore,
    pub available: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub size: usize,
}

#[repr(C)]
pub struct dlfb_data {
    pub udev: *mut usb_device,
    pub info: *mut fb_info,
    pub urbs: urb_list,
    pub backing_buffer: *mut ::core::ffi::c_char,
    pub fb_count: ::core::ffi::c_int,
    pub virtualized: bool, /* true when physical usb device not present */
    pub usb_active: atomic_t, /* 0 = update virtual buffer, but no usb traffic */
    pub lost_pixels: atomic_t, /* 1 = a render op failed. Need screen refresh */
    pub edid: *mut ::core::ffi::c_char, /* null until we read edid from hw or get from sysfs */
    pub edid_size: usize,
    pub sku_pixel_limit: ::core::ffi::c_int,
    pub base16: ::core::ffi::c_int,
    pub base8: ::core::ffi::c_int,
    pub pseudo_palette: [u32; 256],
    pub blank_mode: ::core::ffi::c_int, /* one of FB_BLANK_ */
    pub render_mutex: mutex,
    pub damage_x: ::core::ffi::c_int,
    pub damage_y: ::core::ffi::c_int,
    pub damage_x2: ::core::ffi::c_int,
    pub damage_y2: ::core::ffi::c_int,
    pub damage_lock: spinlock_t,
    pub damage_work: work_struct,
    pub ops: fb_ops,
    pub mmap_count: atomic_t,
    /* blit-only rendering path metrics, exposed through sysfs */
    pub bytes_rendered: atomic_t, /* raw pixel-bytes driver asked to render */
    pub bytes_identical: atomic_t, /* saved effort with backbuffer comparison */
    pub bytes_sent: atomic_t, /* to usb, after compression including overhead */
    pub cpu_kcycles_used: atomic_t, /* transpired during pixel processing */
    pub current_mode: fb_var_screeninfo,
    pub deferred_free: list_head,
}

pub const NR_USB_REQUEST_I2C_SUB_IO: u32 = 0x02;
pub const NR_USB_REQUEST_CHANNEL: u32 = 0x12;

/* -BULK_SIZE as per usb-skeleton. Can we get full page and avoid overhead? */
pub const BULK_SIZE: usize = 512;
/* PAGE_SIZE is supplied by the kernel environment. */
pub const MAX_TRANSFER: usize = PAGE_SIZE * 16 - BULK_SIZE;
pub const WRITES_IN_FLIGHT: usize = 4;

pub const MAX_VENDOR_DESCRIPTOR_SIZE: usize = 256;

pub const GET_URB_TIMEOUT: usize = HZ;
pub const FREE_URB_TIMEOUT: usize = HZ * 2;

pub const BPP: usize = 2;
pub const MAX_CMD_PIXELS: usize = 255;

pub const RLX_HEADER_BYTES: usize = 7;
pub const MIN_RLX_PIX_BYTES: usize = 4;
pub const MIN_RLX_CMD_BYTES: usize = RLX_HEADER_BYTES + MIN_RLX_PIX_BYTES;

pub const RLE_HEADER_BYTES: usize = 6;
pub const MIN_RLE_PIX_BYTES: usize = 3;
pub const MIN_RLE_CMD_BYTES: usize = RLE_HEADER_BYTES + MIN_RLE_PIX_BYTES;

pub const RAW_HEADER_BYTES: usize = 6;
pub const MIN_RAW_PIX_BYTES: usize = 2;
pub const MIN_RAW_CMD_BYTES: usize = RAW_HEADER_BYTES + MIN_RAW_PIX_BYTES;

/* optimal value for 720p video */
pub const DL_DEFIO_WRITE_DELAY: usize = msecs_to_jiffies(if HZ <= 300 { 4 } else { 10 });
/* "disable" with long delay */
pub const DL_DEFIO_WRITE_DISABLE: usize = HZ * 60;

/* remove these once align.h patch is taken into kernel */
#[inline]
pub const fn DL_ALIGN_UP(x: usize, a: usize) -> usize {
    (x + a - 1) / a * a
}

#[inline]
pub const fn DL_ALIGN_DOWN(x: usize, a: usize) -> usize {
    x / a * a
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
