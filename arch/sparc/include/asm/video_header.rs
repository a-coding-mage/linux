/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/io.h, linux/types.h, asm/page.h, and asm-generic/video.h.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(CONFIG_SPARC32)]
#[inline]
pub unsafe fn pgprot_framebuffer(
    prot: pgprot_t,
    _vm_start: libc::c_ulong,
    _vm_end: libc::c_ulong,
    _offset: libc::c_ulong,
) -> pgprot_t {
    prot
}

#[cfg(CONFIG_VIDEO)]
unsafe extern "C" {
    pub fn video_is_primary_device(dev: *mut device) -> bool;
}

#[inline]
pub unsafe fn fb_memcpy_fromio(
    to: *mut libc::c_void,
    from: *const core::ffi::c_void,
    n: usize,
) {
    sbus_memcpy_fromio(to, from, n);
}

#[inline]
pub unsafe fn fb_memcpy_toio(
    to: *mut core::ffi::c_void,
    from: *const libc::c_void,
    n: usize,
) {
    sbus_memcpy_toio(to, from, n);
}

#[inline]
pub unsafe fn fb_memset_io(addr: *mut core::ffi::c_void, c: libc::c_int, n: usize) {
    sbus_memset_io(addr, c, n);
}

// C macro: #define fb_memset fb_memset_io
pub use fb_memset_io as fb_memset;

unsafe extern "C" {
    fn sbus_memcpy_fromio(to: *mut libc::c_void, from: *const core::ffi::c_void, n: usize);
    fn sbus_memcpy_toio(to: *mut core::ffi::c_void, from: *const libc::c_void, n: usize);
    fn sbus_memset_io(addr: *mut core::ffi::c_void, c: libc::c_int, n: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
