/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <linux/io.h> are supplied externally.

#[repr(C)]
pub struct screen_info {
    _private: [u8; 0],
}

extern "C" {
    pub static mut vga_base: ::core::ffi::c_ulong;
    pub static mut vgacon_screen_info: screen_info;
}

#[inline]
pub unsafe fn VGA_MAP_MEM(x: ::core::ffi::c_ulong, _s: usize) -> ::core::ffi::c_ulong {
    vga_base + x
}

#[inline]
pub unsafe fn vga_readb(x: *const u8) -> u8 {
    ::core::ptr::read_volatile(x)
}

#[inline]
pub unsafe fn vga_writeb(x: u8, y: *mut u8) {
    ::core::ptr::write_volatile(y, x);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
