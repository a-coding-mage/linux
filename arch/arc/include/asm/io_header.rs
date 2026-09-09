/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux dependencies supplied by the surrounding translation.
// The CONFIG_ISA_ARCV2 conditional selects the barrier implementation.

extern "C" {
    pub fn ioremap(paddr: usize, size: usize) -> *mut core::ffi::c_void;
    pub fn ioremap_prot(paddr: usize, size: usize, prot: usize) -> *mut core::ffi::c_void;
    pub fn iounmap(addr: *mut core::ffi::c_void);
    pub fn rmb();
    pub fn wmb();
}

#[inline]
pub unsafe fn ioport_map(port: usize, _nr: u32) -> *mut core::ffi::c_void {
    port as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn ioport_unmap(_addr: *mut core::ffi::c_void) {}

#[inline]
unsafe fn __iormb() {
    // CONFIG_ISA_ARCV2: rmb(); otherwise this is an empty barrier.
    #[cfg(CONFIG_ISA_ARCV2)]
    { rmb(); }
}

#[inline]
unsafe fn __iowmb() {
    // CONFIG_ISA_ARCV2: wmb(); otherwise this is an empty barrier.
    #[cfg(CONFIG_ISA_ARCV2)]
    { wmb(); }
}

#[inline]
pub unsafe fn __raw_readb(addr: *const core::ffi::c_void) -> u8 {
    core::ptr::read_volatile(addr as *const u8)
}

#[inline]
pub unsafe fn __raw_readw(addr: *const core::ffi::c_void) -> u16 {
    core::ptr::read_volatile(addr as *const u16)
}

#[inline]
pub unsafe fn __raw_readl(addr: *const core::ffi::c_void) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}

#[inline]
pub unsafe fn __raw_writeb(value: u8, addr: *mut core::ffi::c_void) {
    core::ptr::write_volatile(addr as *mut u8, value);
}

#[inline]
pub unsafe fn __raw_writew(value: u16, addr: *mut core::ffi::c_void) {
    core::ptr::write_volatile(addr as *mut u16, value);
}

#[inline]
pub unsafe fn __raw_writel(value: u32, addr: *mut core::ffi::c_void) {
    core::ptr::write_volatile(addr as *mut u32, value);
}

#[inline]
pub unsafe fn __raw_readsb(addr: *const core::ffi::c_void, ptr: *mut u8, count: u32) {
    let mut i = 0;
    while i < count { *ptr.add(i as usize) = __raw_readb(addr); i += 1; }
}

#[inline]
pub unsafe fn __raw_readsw(addr: *const core::ffi::c_void, ptr: *mut u16, count: u32) {
    let mut i = 0;
    while i < count { *ptr.add(i as usize) = __raw_readw(addr); i += 1; }
}

#[inline]
pub unsafe fn __raw_readsl(addr: *const core::ffi::c_void, ptr: *mut u32, count: u32) {
    let mut i = 0;
    while i < count { *ptr.add(i as usize) = __raw_readl(addr); i += 1; }
}

#[inline]
pub unsafe fn __raw_writesb(addr: *mut core::ffi::c_void, ptr: *const u8, count: u32) {
    let mut i = 0;
    while i < count { __raw_writeb(*ptr.add(i as usize), addr); i += 1; }
}

#[inline]
pub unsafe fn __raw_writesw(addr: *mut core::ffi::c_void, ptr: *const u16, count: u32) {
    let mut i = 0;
    while i < count { __raw_writew(*ptr.add(i as usize), addr); i += 1; }
}

#[inline]
pub unsafe fn __raw_writesl(addr: *mut core::ffi::c_void, ptr: *const u32, count: u32) {
    let mut i = 0;
    while i < count { __raw_writel(*ptr.add(i as usize), addr); i += 1; }
}

#[inline]
pub unsafe fn ioread16be(p: *const core::ffi::c_void) -> u16 {
    let v = u16::from_be(__raw_readw(p)); __iormb(); v
}

#[inline]
pub unsafe fn ioread32be(p: *const core::ffi::c_void) -> u32 {
    let v = u32::from_be(__raw_readl(p)); __iormb(); v
}

#[inline]
pub unsafe fn iowrite16be(v: u16, p: *mut core::ffi::c_void) {
    __iowmb(); __raw_writew(v.to_be(), p);
}

#[inline]
pub unsafe fn iowrite32be(v: u32, p: *mut core::ffi::c_void) {
    __iowmb(); __raw_writel(v.to_be(), p);
}

#[inline] pub unsafe fn readb_relaxed(c: *const core::ffi::c_void) -> u8 { __raw_readb(c) }
#[inline] pub unsafe fn readw_relaxed(c: *const core::ffi::c_void) -> u16 { u16::from_le(__raw_readw(c)) }
#[inline] pub unsafe fn readl_relaxed(c: *const core::ffi::c_void) -> u32 { u32::from_le(__raw_readl(c)) }
#[inline] pub unsafe fn writeb_relaxed(v: u8, c: *mut core::ffi::c_void) { __raw_writeb(v, c) }
#[inline] pub unsafe fn writew_relaxed(v: u16, c: *mut core::ffi::c_void) { __raw_writew(v.to_le(), c) }
#[inline] pub unsafe fn writel_relaxed(v: u32, c: *mut core::ffi::c_void) { __raw_writel(v.to_le(), c) }

#[inline] pub unsafe fn readb(c: *const core::ffi::c_void) -> u8 { let v = readb_relaxed(c); __iormb(); v }
#[inline] pub unsafe fn readw(c: *const core::ffi::c_void) -> u16 { let v = readw_relaxed(c); __iormb(); v }
#[inline] pub unsafe fn readl(c: *const core::ffi::c_void) -> u32 { let v = readl_relaxed(c); __iormb(); v }
#[inline] pub unsafe fn readsb(p: *const core::ffi::c_void, d: *mut u8, l: u32) { __raw_readsb(p, d, l); __iormb(); }
#[inline] pub unsafe fn readsw(p: *const core::ffi::c_void, d: *mut u16, l: u32) { __raw_readsw(p, d, l); __iormb(); }
#[inline] pub unsafe fn readsl(p: *const core::ffi::c_void, d: *mut u32, l: u32) { __raw_readsl(p, d, l); __iormb(); }
#[inline] pub unsafe fn writeb(v: u8, c: *mut core::ffi::c_void) { __iowmb(); writeb_relaxed(v, c); }
#[inline] pub unsafe fn writew(v: u16, c: *mut core::ffi::c_void) { __iowmb(); writew_relaxed(v, c); }
#[inline] pub unsafe fn writel(v: u32, c: *mut core::ffi::c_void) { __iowmb(); writel_relaxed(v, c); }
#[inline] pub unsafe fn writesb(p: *mut core::ffi::c_void, d: *const u8, l: u32) { __iowmb(); __raw_writesb(p, d, l); }
#[inline] pub unsafe fn writesw(p: *mut core::ffi::c_void, d: *const u16, l: u32) { __iowmb(); __raw_writesw(p, d, l); }
#[inline] pub unsafe fn writesl(p: *mut core::ffi::c_void, d: *const u32, l: u32) { __iowmb(); __raw_writesl(p, d, l); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
