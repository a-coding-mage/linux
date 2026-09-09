/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Convert a physical memory address into an IO memory address.
 * For us this is trivially a type cast.
 */
#[inline]
pub fn iomem(a: usize) -> *mut core::ffi::c_void {
    a as *mut core::ffi::c_void
}

/* Native-endian, volatile access used by non-MMU m68k and ColdFire. */
#[macro_export]
macro_rules! __raw_readb {
    ($addr:expr) => {{ unsafe { core::ptr::read_volatile(($addr) as *const u8) } }};
}
#[macro_export]
macro_rules! __raw_readw {
    ($addr:expr) => {{ unsafe { core::ptr::read_volatile(($addr) as *const u16) } }};
}
#[macro_export]
macro_rules! __raw_readl {
    ($addr:expr) => {{ unsafe { core::ptr::read_volatile(($addr) as *const u32) } }};
}
#[macro_export]
macro_rules! __raw_writeb {
    ($b:expr, $addr:expr) => {{ unsafe { core::ptr::write_volatile(($addr) as *mut u8, $b as u8) } }};
}
#[macro_export]
macro_rules! __raw_writew {
    ($b:expr, $addr:expr) => {{ unsafe { core::ptr::write_volatile(($addr) as *mut u16, $b as u16) } }};
}
#[macro_export]
macro_rules! __raw_writel {
    ($b:expr, $addr:expr) => {{ unsafe { core::ptr::write_volatile(($addr) as *mut u32, $b as u32) } }};
}

/* CONFIG_COLDFIRE supplies the platform byte-order and peripheral definitions. */

/* IOMEMBASE support: the original range constants are supplied by the platform. */
#[cfg(feature = "iomembase")]
#[inline]
unsafe fn __cf_internalio(addr: usize) -> bool {
    addr >= IOMEMBASE as usize && addr <= (IOMEMBASE as usize).wrapping_add(IOMEMSIZE as usize).wrapping_sub(1)
}

#[cfg(feature = "iomembase")]
#[inline]
unsafe fn cf_internalio(addr: *const core::ffi::c_void) -> bool {
    __cf_internalio(addr as usize)
}

/* swab16 and swab32 are supplied by the platform byte-order implementation. */
#[cfg(feature = "iomembase")]
#[inline]
pub unsafe fn readw(addr: *const core::ffi::c_void) -> u16 {
    if cf_internalio(addr) { __raw_readw!(addr) } else { swab16(__raw_readw!(addr)) }
}

#[cfg(feature = "iomembase")]
#[inline]
pub unsafe fn readl(addr: *const core::ffi::c_void) -> u32 {
    if cf_internalio(addr) { __raw_readl!(addr) } else { swab32(__raw_readl!(addr)) }
}

#[cfg(feature = "iomembase")]
#[inline]
pub unsafe fn writew(value: u16, addr: *mut core::ffi::c_void) {
    if cf_internalio(addr) { __raw_writew!(value, addr); } else { __raw_writew!(swab16(value), addr); }
}

#[cfg(feature = "iomembase")]
#[inline]
pub unsafe fn writel(value: u32, addr: *mut core::ffi::c_void) {
    if cf_internalio(addr) { __raw_writel!(value, addr); } else { __raw_writel!(swab32(value), addr); }
}

/* Without IOMEMBASE, the architecture accessors are direct raw accesses. */
#[cfg(not(feature = "iomembase"))]
#[inline] pub unsafe fn readb(addr: *const core::ffi::c_void) -> u8 { __raw_readb!(addr) }
#[cfg(not(feature = "iomembase"))]
#[inline] pub unsafe fn readw(addr: *const core::ffi::c_void) -> u16 { __raw_readw!(addr) }
#[cfg(not(feature = "iomembase"))]
#[inline] pub unsafe fn readl(addr: *const core::ffi::c_void) -> u32 { __raw_readl!(addr) }
#[cfg(not(feature = "iomembase"))]
#[inline] pub unsafe fn writeb(value: u8, addr: *mut core::ffi::c_void) { __raw_writeb!(value, addr); }
#[cfg(not(feature = "iomembase"))]
#[inline] pub unsafe fn writew(value: u16, addr: *mut core::ffi::c_void) { __raw_writew!(value, addr); }
#[cfg(not(feature = "iomembase"))]
#[inline] pub unsafe fn writel(value: u32, addr: *mut core::ffi::c_void) { __raw_writel!(value, addr); }

/* CONFIG_COLDFIRE: internal peripheral registers are big-endian. */
#[cfg(feature = "coldfire")]
pub use __raw_readb as mcf_read8;
#[cfg(feature = "coldfire")]
pub use __raw_readw as mcf_read16;
#[cfg(feature = "coldfire")]
pub use __raw_readl as mcf_read32;
#[cfg(feature = "coldfire")]
pub use __raw_writeb as mcf_write8;
#[cfg(feature = "coldfire")]
pub use __raw_writew as mcf_write16;
#[cfg(feature = "coldfire")]
pub use __raw_writel as mcf_write32;

/* CONFIG_PCI mappings (host physical, bus physical, and sizes). */
#[cfg(feature = "pci")]
pub const PCI_MEM_PA: u32 = 0xf0000000;
#[cfg(feature = "pci")]
pub const PCI_MEM_BA: u32 = 0xf0000000;
#[cfg(feature = "pci")]
pub const PCI_MEM_SIZE: u32 = 0x08000000;
#[cfg(feature = "pci")]
pub const PCI_MEM_MASK: u32 = PCI_MEM_SIZE - 1;
#[cfg(feature = "pci")]
pub const PCI_IO_PA: u32 = 0xf8000000;
#[cfg(feature = "pci")]
pub const PCI_IO_BA: u32 = 0x00000000;
#[cfg(feature = "pci")]
pub const PCI_IO_SIZE: u32 = 0x00010000;
#[cfg(feature = "pci")]
pub const PCI_IO_MASK: u32 = PCI_IO_SIZE - 1;
#[cfg(feature = "pci")]
pub const PCI_IOBASE: *mut core::ffi::c_void = PCI_IO_PA as *mut core::ffi::c_void;
#[cfg(feature = "pci")]
pub const PCI_SPACE_LIMIT: u32 = PCI_IO_MASK;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
