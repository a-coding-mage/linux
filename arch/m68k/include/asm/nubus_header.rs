/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <asm/nubus.h>.
// Dependencies supplied by asm/raw_io.h and asm/kmap.h remain external.

use core::ffi::c_void;
use core::ffi::c_ulong;
use core::ffi::c_uint;

unsafe extern "C" {
    fn raw_inb(addr: c_ulong) -> u8;
    fn raw_inw(addr: c_ulong) -> u16;
    fn raw_inl(addr: c_ulong) -> u32;

    fn raw_outb(value: u8, addr: c_ulong);
    fn raw_outw(value: u16, addr: c_ulong);
    fn raw_outl(value: u32, addr: c_ulong);

    fn memset(dest: *mut c_void, value: i32, count: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, count: usize) -> *mut c_void;

    fn __ioremap(physaddr: c_ulong, size: c_ulong, flags: c_uint) -> *mut c_void;
    fn iounmap(addr: *mut c_void);

    static IOMAP_NOCACHE_SER: c_uint;
    static IOMAP_NOCACHE_NONSER: c_uint;
    static IOMAP_WRITETHROUGH: c_uint;
    static IOMAP_FULL_CACHING: c_uint;
}

macro_rules! nubus_readb { ($addr:expr) => { raw_inb($addr) }; }
macro_rules! nubus_readw { ($addr:expr) => { raw_inw($addr) }; }
macro_rules! nubus_readl { ($addr:expr) => { raw_inl($addr) }; }

macro_rules! nubus_writeb { ($value:expr, $addr:expr) => { raw_outb($value, $addr) }; }
macro_rules! nubus_writew { ($value:expr, $addr:expr) => { raw_outw($value, $addr) }; }
macro_rules! nubus_writel { ($value:expr, $addr:expr) => { raw_outl($value, $addr) }; }

macro_rules! nubus_memset_io {
    ($a:expr, $b:expr, $c:expr) => {
        memset(($a as *mut c_void), $b, $c)
    };
}
macro_rules! nubus_memcpy_fromio {
    ($a:expr, $b:expr, $c:expr) => {
        memcpy(($a as *mut c_void), ($b as *const c_void), $c)
    };
}
macro_rules! nubus_memcpy_toio {
    ($a:expr, $b:expr, $c:expr) => {
        memcpy(($a as *mut c_void), ($b as *const c_void), $c)
    };
}

unsafe fn nubus_remap_nocache_ser(physaddr: c_ulong, size: c_ulong) -> *mut c_void {
    __ioremap(physaddr, size, IOMAP_NOCACHE_SER)
}

unsafe fn nubus_remap_nocache_nonser(physaddr: c_ulong, size: c_ulong) -> *mut c_void {
    __ioremap(physaddr, size, IOMAP_NOCACHE_NONSER)
}

unsafe fn nbus_remap_writethrough(physaddr: c_ulong, size: c_ulong) -> *mut c_void {
    __ioremap(physaddr, size, IOMAP_WRITETHROUGH)
}

unsafe fn nubus_remap_fullcache(physaddr: c_ulong, size: c_ulong) -> *mut c_void {
    __ioremap(physaddr, size, IOMAP_FULL_CACHING)
}

macro_rules! nubus_unmap { ($addr:expr) => { iounmap($addr) }; }
macro_rules! nubus_iounmap { ($addr:expr) => { iounmap($addr) }; }
macro_rules! nubus_ioremap {
    ($physaddr:expr, $size:expr) => { nubus_remap_nocache_ser($physaddr, $size) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
