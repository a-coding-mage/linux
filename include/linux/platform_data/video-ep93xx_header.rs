/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations from the C header.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fb_info {
    _private: [u8; 0],
}

/* VideoAttributes flags */
pub const EP93XXFB_STATE_MACHINE_ENABLE: u32 = 1 << 0;
pub const EP93XXFB_PIXEL_CLOCK_ENABLE: u32 = 1 << 1;
pub const EP93XXFB_VSYNC_ENABLE: u32 = 1 << 2;
pub const EP93XXFB_PIXEL_DATA_ENABLE: u32 = 1 << 3;
pub const EP93XXFB_COMPOSITE_SYNC: u32 = 1 << 4;
pub const EP93XXFB_SYNC_VERT_HIGH: u32 = 1 << 5;
pub const EP93XXFB_SYNC_HORIZ_HIGH: u32 = 1 << 6;
pub const EP93XXFB_SYNC_BLANK_HIGH: u32 = 1 << 7;
pub const EP93XXFB_PCLK_FALLING: u32 = 1 << 8;
pub const EP93XXFB_ENABLE_AC: u32 = 1 << 9;
pub const EP93XXFB_ENABLE_LCD: u32 = 1 << 10;
pub const EP93XXFB_ENABLE_CCIR: u32 = 1 << 12;
pub const EP93XXFB_USE_PARALLEL_INTERFACE: u32 = 1 << 13;
pub const EP93XXFB_ENABLE_INTERRUPT: u32 = 1 << 14;
pub const EP93XXFB_USB_INTERLACE: u32 = 1 << 16;
pub const EP93XXFB_USE_EQUALIZATION: u32 = 1 << 17;
pub const EP93XXFB_USE_DOUBLE_HORZ: u32 = 1 << 18;
pub const EP93XXFB_USE_DOUBLE_VERT: u32 = 1 << 19;
pub const EP93XXFB_USE_BLANK_PIXEL: u32 = 1 << 20;
pub const EP93XXFB_USE_SDCSN0: u32 = 0 << 21;
pub const EP93XXFB_USE_SDCSN1: u32 = 1 << 21;
pub const EP93XXFB_USE_SDCSN2: u32 = 2 << 21;
pub const EP93XXFB_USE_SDCSN3: u32 = 3 << 21;

pub const EP93XXFB_ENABLE: u32 = EP93XXFB_STATE_MACHINE_ENABLE
    | EP93XXFB_PIXEL_CLOCK_ENABLE
    | EP93XXFB_VSYNC_ENABLE
    | EP93XXFB_PIXEL_DATA_ENABLE;

#[repr(C)]
pub struct ep93xxfb_mach_info {
    pub flags: ::core::ffi::c_uint,
    pub setup: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> ::core::ffi::c_int>,
    pub teardown: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
    pub blank: Option<unsafe extern "C" fn(
        blank_mode: ::core::ffi::c_int,
        info: *mut fb_info,
    )>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
