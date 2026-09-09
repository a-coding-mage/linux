/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Access to VGA videoram.
 *
 * This header is active only for the kernel build (__KERNEL__).
 *
 * Dependency supplied externally: asm/io.h.
 */

/* CONFIG_VGA_CONSOLE: the following VGA console support is build-time conditional. */

pub const VT_BUF_HAVE_RW: bool = true;

/*
 * These are only needed for supporting VGA or MDA text mode, which use little
 * endian byte ordering. In other cases, native byte ordering may be used.
 */
#[inline]
pub unsafe fn scr_writew(val: u16, addr: *mut u16) {
    *addr = cpu_to_le16(val);
}

#[inline]
pub unsafe fn scr_readw(addr: *const u16) -> u16 {
    le16_to_cpu(*addr)
}

pub const VT_BUF_HAVE_MEMSETW: bool = true;

#[inline]
pub unsafe fn scr_memsetw(s: *mut u16, v: u16, n: u32) {
    memset16(s, cpu_to_le16(v), n / 2);
}

/*
 * On 64-bit PowerPC, VGA_MAP_MEM(x, s) maps the address with ioremap and
 * converts the result to unsigned long. On other PowerPC targets it is x.
 */
#[cfg(target_pointer_width = "64")]
#[inline]
pub unsafe fn vga_map_mem(x: usize, s: usize) -> usize {
    ioremap(x, s) as usize
}

#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn vga_map_mem(x: usize, _s: usize) -> usize {
    x
}

#[inline]
pub unsafe fn vga_readb(x: *const u8) -> u8 {
    *x
}

#[inline]
pub unsafe fn vga_writeb(x: u8, y: *mut u8) {
    *y = x;
}

/* External symbols supplied by asm/io.h. */
extern "C" {
    fn cpu_to_le16(val: u16) -> u16;
    fn le16_to_cpu(val: u16) -> u16;
    fn memset16(s: *mut u16, v: u16, n: u32);
    fn ioremap(x: usize, s: usize) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
