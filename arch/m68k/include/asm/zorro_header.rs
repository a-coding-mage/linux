/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by asm/raw_io.h and asm/kmap.h remain external.

macro_rules! z_readb {
    ($($arg:tt)*) => { raw_inb!($($arg)*) };
}
macro_rules! z_readw {
    ($($arg:tt)*) => { raw_inw!($($arg)*) };
}
macro_rules! z_readl {
    ($($arg:tt)*) => { raw_inl!($($arg)*) };
}

macro_rules! z_writeb {
    ($($arg:tt)*) => { raw_outb!($($arg)*) };
}
macro_rules! z_writew {
    ($($arg:tt)*) => { raw_outw!($($arg)*) };
}
macro_rules! z_writel {
    ($($arg:tt)*) => { raw_outl!($($arg)*) };
}

macro_rules! z_memset_io {
    ($a:expr, $b:expr, $c:expr) => {
        memset(($a as *mut core::ffi::c_void), $b, $c)
    };
}
macro_rules! z_memcpy_fromio {
    ($a:expr, $b:expr, $c:expr) => {
        memcpy($a, ($b as *const core::ffi::c_void), $c)
    };
}
macro_rules! z_memcpy_toio {
    ($a:expr, $b:expr, $c:expr) => {
        memcpy(($a as *mut core::ffi::c_void), $b, $c)
    };
}

#[inline]
pub unsafe fn z_remap_nocache_ser(
    physaddr: c_ulong,
    size: c_ulong,
) -> *mut core::ffi::c_void {
    __ioremap(physaddr, size, IOMAP_NOCACHE_SER)
}

#[inline]
pub unsafe fn z_remap_nocache_nonser(
    physaddr: c_ulong,
    size: c_ulong,
) -> *mut core::ffi::c_void {
    __ioremap(physaddr, size, IOMAP_NOCACHE_NONSER)
}

#[inline]
pub unsafe fn z_remap_writethrough(
    physaddr: c_ulong,
    size: c_ulong,
) -> *mut core::ffi::c_void {
    __ioremap(physaddr, size, IOMAP_WRITETHROUGH)
}

#[inline]
pub unsafe fn z_remap_fullcache(
    physaddr: c_ulong,
    size: c_ulong,
) -> *mut core::ffi::c_void {
    __ioremap(physaddr, size, IOMAP_FULL_CACHING)
}

macro_rules! z_unmap {
    ($($arg:tt)*) => { iounmap!($($arg)*) };
}
macro_rules! z_iounmap {
    ($($arg:tt)*) => { iounmap!($($arg)*) };
}
macro_rules! z_ioremap {
    ($($arg:tt)*) => { z_remap_nocache_ser($($arg)*) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
