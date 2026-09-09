/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2007-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2007-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/* Translated from the MicroBlaze asm/io.h header. */

/* The original includes provide byte-order, page, Linux type, and page
 * definitions; those names are supplied by the surrounding translation. */

#[cfg(not(feature = "CONFIG_PCI"))]
pub const _IO_BASE: usize = 0;
#[cfg(not(feature = "CONFIG_PCI"))]
pub const _ISA_MEM_BASE: usize = 0;

#[cfg(feature = "CONFIG_PCI")]
pub struct pci_dev;

#[cfg(feature = "CONFIG_PCI")]
extern "C" {
    pub fn pci_iounmap(dev: *mut pci_dev, addr: *mut core::ffi::c_void);
    pub static mut isa_io_base: usize;
    pub static mut isa_mem_base: resource_size_t;
}

/* Under CONFIG_PCI, resource_size_t is supplied by Linux type definitions. */
#[cfg(feature = "CONFIG_PCI")]
pub type resource_size_t = u64;

#[cfg(not(feature = "CONFIG_PCI"))]
pub const PCI_IOBASE: *mut core::ffi::c_void = core::ptr::null_mut();
#[cfg(feature = "CONFIG_PCI")]
pub const PCI_IOBASE: *mut core::ffi::c_void = core::ptr::null_mut();
pub const IO_SPACE_LIMIT: u32 = 0xFFFF_FFFF;

extern "C" {
    pub fn iounmap(addr: *mut core::ffi::c_void);
    pub fn ioremap(address: phys_addr_t, size: usize) -> *mut core::ffi::c_void;
}

/* phys_addr_t is supplied by the Linux type definitions. */
pub type phys_addr_t = usize;

/* Big Endian */
#[macro_export]
macro_rules! out_be32 {
    ($a:expr, $v:expr) => { unsafe { __raw_writel($v, $a as *mut core::ffi::c_void) } };
}
#[macro_export]
macro_rules! out_be16 {
    ($a:expr, $v:expr) => { unsafe { __raw_writew($v, $a) } };
}
#[macro_export]
macro_rules! in_be32 {
    ($a:expr) => { unsafe { __raw_readl($a as *const core::ffi::c_void) } };
}
#[macro_export]
macro_rules! in_be16 {
    ($a:expr) => { unsafe { __raw_readw($a) } };
}
#[macro_export]
macro_rules! writel_be {
    ($v:expr, $a:expr) => { $crate::out_be32!($a as *mut u32, $v) };
}
#[macro_export]
macro_rules! readl_be {
    ($a:expr) => { $crate::in_be32!($a as *mut u32) };
}

/* Little endian */
#[macro_export]
macro_rules! out_le32 {
    ($a:expr, $v:expr) => { unsafe { __raw_writel(__cpu_to_le32($v), $a) } };
}
#[macro_export]
macro_rules! out_le16 {
    ($a:expr, $v:expr) => { unsafe { __raw_writew(__cpu_to_le16($v), $a) } };
}
#[macro_export]
macro_rules! in_le32 {
    ($a:expr) => { unsafe { __le32_to_cpu(__raw_readl($a)) } };
}
#[macro_export]
macro_rules! in_le16 {
    ($a:expr) => { unsafe { __le16_to_cpu(__raw_readw($a)) } };
}

/* Byte ops */
#[macro_export]
macro_rules! out_8 {
    ($a:expr, $v:expr) => { unsafe { __raw_writeb($v, $a) } };
}
#[macro_export]
macro_rules! in_8 {
    ($a:expr) => { unsafe { __raw_readb($a) } };
}

/* asm-generic/io.h declarations and definitions are supplied separately. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
