/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Access to VGA videoram
 *
 *	(c) 1998 Martin Mares <mj@ucw.cz>
 */

// Dependency supplied by asm/io.h.
extern "C" {
    fn __is_ioaddr(addr: *const u16) -> bool;
    fn __raw_writew(val: u16, addr: *mut u16);
    fn __raw_readw(addr: *const u16) -> u16;
    fn memsetw_io(addr: *mut u16, val: u16, count: u32);
    fn memset16(addr: *mut u16, val: u16, count: u32);
    fn readb(addr: *mut u8) -> u8;
    fn writeb(val: u8, addr: *mut u8);
    fn ioremap(x: usize, s: usize) -> *mut core::ffi::c_void;
}

pub const VT_BUF_HAVE_RW: () = ();
pub const VT_BUF_HAVE_MEMSETW: () = ();
pub const VT_BUF_HAVE_MEMCPYW: () = ();
pub const VT_BUF_HAVE_MEMMOVEW: () = ();

pub unsafe fn scr_writew(val: u16, addr: *mut u16) {
    if __is_ioaddr(addr) {
        __raw_writew(val, addr);
    } else {
        core::ptr::write_volatile(addr, val);
    }
}

pub unsafe fn scr_readw(addr: *const u16) -> u16 {
    if __is_ioaddr(addr) {
        __raw_readw(addr)
    } else {
        core::ptr::read_volatile(addr)
    }
}

pub unsafe fn scr_memsetw(s: *mut u16, c: u16, count: u32) {
    if __is_ioaddr(s) {
        memsetw_io(s, c, count);
    } else {
        memset16(s, c, count / 2);
    }
}

/* Do not trust that the usage will be correct; analyze the arguments.  */
extern "C" {
    pub fn scr_memcpyw(d: *mut u16, s: *const u16, count: u32);
    pub fn scr_memmovew(d: *mut u16, s: *const u16, count: u32);
}

/* ??? These are currently only used for downloading character sets.  As
   such, they don't need memory barriers.  Is this all they are intended
   to be used for?  */
pub unsafe fn vga_readb(a: *mut core::ffi::c_void) -> u8 {
    readb(a as *mut u8)
}

pub unsafe fn vga_writeb(v: u8, a: *mut core::ffi::c_void) {
    writeb(v, a as *mut u8);
}

// CONFIG_VGA_HOSE is a build-time condition from the original header.
// Its enabled branch depends on linux/ioport.h and linux/pci.h declarations.
#[cfg(feature = "CONFIG_VGA_HOSE")]
pub unsafe fn __is_port_vga(a: usize) -> bool {
    a >= 0x3b0 && a < 0x3e0 && a != 0x3b3 && a != 0x3d3
}

#[cfg(feature = "CONFIG_VGA_HOSE")]
pub unsafe fn __is_mem_vga(a: usize) -> bool {
    a >= 0xa0000 && a <= 0xc0000
}

#[cfg(not(feature = "CONFIG_VGA_HOSE"))]
pub const pci_vga_hose: usize = 0;

#[cfg(not(feature = "CONFIG_VGA_HOSE"))]
pub const fn __is_port_vga(_a: usize) -> bool { false }

#[cfg(not(feature = "CONFIG_VGA_HOSE"))]
pub const fn __is_mem_vga(_a: usize) -> bool { false }

pub unsafe fn VGA_MAP_MEM(x: usize, s: usize) -> usize {
    ioremap(x, s) as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
