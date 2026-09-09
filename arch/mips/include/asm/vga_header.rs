/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Access to VGA videoram
 *
 *	(c) 1998 Martin Mares <mj@ucw.cz>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/string.h, asm/addrspace.h, and asm/byteorder.h.

/*
 *	On the PC, we can just recalculate addresses and then
 *	access the videoram directly without any black magic.
 */

#[macro_export]
macro_rules! VGA_MAP_MEM {
    ($x:expr, $s:expr) => {
        CKSEG1ADDR(0x10000000usize.wrapping_add(($x) as usize))
    };
}

#[macro_export]
macro_rules! vga_readb {
    ($x:expr) => {
        *($x)
    };
}

#[macro_export]
macro_rules! vga_writeb {
    ($x:expr, $y:expr) => {
        *($y) = $x
    };
}

pub const VT_BUF_HAVE_RW: bool = true;

/*
 *  These are only needed for supporting VGA or MDA text mode, which use little
 *  endian byte ordering.
 *  In other cases, we can optimize by using native byte ordering and
 *  <linux/vt_buffer.h> has already done the right job for us.
 */

extern "C" {
    fn cpu_to_le16(val: u16) -> u16;
    fn le16_to_cpu(val: u16) -> u16;
    fn memset16(s: *mut u16, v: u16, count: usize);
}

#[inline]
pub unsafe fn scr_writew(val: u16, addr: *mut u16) {
    core::ptr::write_volatile(addr, cpu_to_le16(val));
}

#[inline]
pub unsafe fn scr_readw(addr: *const u16) -> u16 {
    le16_to_cpu(core::ptr::read_volatile(addr))
}

#[inline]
pub unsafe fn scr_memsetw(s: *mut u16, v: u16, count: u32) {
    memset16(s, cpu_to_le16(v), (count / 2) as usize);
}

pub const VT_BUF_HAVE_MEMSETW: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
