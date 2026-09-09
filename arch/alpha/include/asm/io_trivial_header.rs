/* SPDX-License-Identifier: GPL-2.0 */
/* Trivial implementations of basic I/O routines. Assumes that all of the
 * hard work has been done by ioremap and ioportmap, and that access to I/O
 * space is linear.
 *
 * The original header may be included multiple times. Its IO_CONCAT prefix
 * and build-time conditions are supplied by the surrounding translation.
 */

/* Original condition: IO_CONCAT(__IO_PREFIX, trivial_io_bw). */
#[inline]
pub unsafe fn ioread8(a: *const core::ffi::c_void) -> u8 {
    core::ptr::read_volatile(a as *const u8)
}

#[inline]
pub unsafe fn ioread16(a: *const core::ffi::c_void) -> u16 {
    __kernel_ldwu(core::ptr::read_volatile(a as *const u16))
}

#[inline]
pub unsafe fn iowrite8(b: u8, a: *mut core::ffi::c_void) {
    __kernel_stb(b, a as *mut u8);
}

#[inline]
pub unsafe fn iowrite16(b: u16, a: *mut core::ffi::c_void) {
    __kernel_stw(b, a as *mut u16);
}

/* Original condition: IO_CONCAT(__IO_PREFIX, trivial_io_lq). */
#[inline]
pub unsafe fn ioread32(a: *const core::ffi::c_void) -> u32 {
    core::ptr::read_volatile(a as *const u32)
}

#[inline]
pub unsafe fn iowrite32(b: u32, a: *mut core::ffi::c_void) {
    core::ptr::write_volatile(a as *mut u32, b);
}

#[inline]
pub unsafe fn ioread64(a: *const core::ffi::c_void) -> u64 {
    core::ptr::read_volatile(a as *const u64)
}

#[inline]
pub unsafe fn iowrite64(b: u64, a: *mut core::ffi::c_void) {
    core::ptr::write_volatile(a as *mut u64, b);
}

/* Original condition: IO_CONCAT(__IO_PREFIX, trivial_rw_bw) == 1. */
#[inline]
pub unsafe fn readb(a: *const core::ffi::c_void) -> u8 {
    core::ptr::read_volatile(a as *const u8)
}

#[inline]
pub unsafe fn readw(a: *const core::ffi::c_void) -> u16 {
    __kernel_ldwu(core::ptr::read_volatile(a as *const u16))
}

#[inline]
pub unsafe fn writeb(b: u8, a: *mut core::ffi::c_void) {
    __kernel_stb(b, a as *mut u8);
}

#[inline]
pub unsafe fn writew(b: u16, a: *mut core::ffi::c_void) {
    __kernel_stw(b, a as *mut u16);
}

/* Original condition: IO_CONCAT(__IO_PREFIX, trivial_rw_bw) == 2. */
#[inline]
pub unsafe fn readb_via_io(a: *const core::ffi::c_void) -> u8 {
    ioread8(a)
}

#[inline]
pub unsafe fn readw_via_io(a: *const core::ffi::c_void) -> u16 {
    ioread16(a)
}

#[inline]
pub unsafe fn writeb_via_io(b: u8, a: *mut core::ffi::c_void) {
    iowrite8(b, a);
}

#[inline]
pub unsafe fn writew_via_io(b: u16, a: *mut core::ffi::c_void) {
    iowrite16(b, a);
}

/* Original condition: IO_CONCAT(__IO_PREFIX, trivial_rw_lq) == 1. */
#[inline]
pub unsafe fn readl(a: *const core::ffi::c_void) -> u32 {
    core::ptr::read_volatile(a as *const u32)
}

#[inline]
pub unsafe fn readq(a: *const core::ffi::c_void) -> u64 {
    core::ptr::read_volatile(a as *const u64)
}

#[inline]
pub unsafe fn writel(b: u32, a: *mut core::ffi::c_void) {
    core::ptr::write_volatile(a as *mut u32, b);
}

#[inline]
pub unsafe fn writeq(b: u64, a: *mut core::ffi::c_void) {
    core::ptr::write_volatile(a as *mut u64, b);
}

/* Original condition: IO_CONCAT(__IO_PREFIX, trivial_iounmap). */
#[inline]
pub unsafe fn iounmap(_a: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
