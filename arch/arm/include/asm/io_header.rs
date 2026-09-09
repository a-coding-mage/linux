/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/include/asm/io.h. */

/* Kernel includes and configuration conditions are supplied by the surrounding build. */

pub const MT_DEVICE: u32 = 0;
pub const MT_DEVICE_NONSHARED: u32 = 1;
pub const MT_DEVICE_CACHED: u32 = 2;
pub const MT_DEVICE_WC: u32 = 3;
pub const PCI_IO_VIRT_BASE: usize = 0xfee00000;
pub const PCI_IOBASE: *mut core::ffi::c_void = PCI_IO_VIRT_BASE as *mut core::ffi::c_void;

extern "C" {
    pub fn atomic_io_modify(reg: *mut core::ffi::c_void, mask: u32, set: u32);
    pub fn atomic_io_modify_relaxed(reg: *mut core::ffi::c_void, mask: u32, set: u32);
    pub fn __raw_writesb(addr: *mut core::ffi::c_void, data: *const core::ffi::c_void, bytelen: i32);
    pub fn __raw_writesw(addr: *mut core::ffi::c_void, data: *const core::ffi::c_void, wordlen: i32);
    pub fn __raw_writesl(addr: *mut core::ffi::c_void, data: *const core::ffi::c_void, longlen: i32);
    pub fn __raw_readsb(addr: *const core::ffi::c_void, data: *mut core::ffi::c_void, bytelen: i32);
    pub fn __raw_readsw(addr: *const core::ffi::c_void, data: *mut core::ffi::c_void, wordlen: i32);
    pub fn __raw_readsl(addr: *const core::ffi::c_void, data: *mut core::ffi::c_void, longlen: i32);
    pub fn __arm_ioremap_caller(addr: usize, size: usize, flags: u32, caller: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn __arm_ioremap_pfn(pfn: usize, offset: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn __arm_ioremap_exec(addr: usize, size: usize, cached: bool) -> *mut core::ffi::c_void;
    pub fn __arm_iomem_set_ro(ptr: *mut core::ffi::c_void, size: usize);
    pub fn __readwrite_bug(func: *const core::ffi::c_char);
    pub fn pci_remap_iospace(res: *const resource, phys_addr: usize) -> i32;
    pub fn pci_remap_cfgspace(res_cookie: usize, size: usize) -> *mut core::ffi::c_void;
    pub fn _memcpy_fromio(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
    pub fn _memcpy_toio(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
    pub fn _memset_io(dst: *mut core::ffi::c_void, value: i32, count: usize);
    pub fn ioremap(res_cookie: usize, size: usize) -> *mut core::ffi::c_void;
    pub fn ioremap_cache(res_cookie: usize, size: usize) -> *mut core::ffi::c_void;
    pub fn ioremap_wc(res_cookie: usize, size: usize) -> *mut core::ffi::c_void;
    pub fn iounmap(addr: *mut core::ffi::c_void);
    pub fn arch_memremap_wb(addr: usize, size: usize, flags: usize) -> *mut core::ffi::c_void;
    pub fn ioport_map(port: usize, nr: u32) -> *mut core::ffi::c_void;
    pub fn ioport_unmap(addr: *mut core::ffi::c_void);
    pub fn pci_iounmap(dev: *mut pci_dev, addr: *mut core::ffi::c_void);
    pub fn valid_phys_addr_range(addr: usize, size: usize) -> i32;
    pub fn valid_mmap_phys_addr_range(pfn: usize, size: usize) -> i32;
    pub fn arch_memremap_can_ram_remap(offset: usize, size: usize, flags: usize) -> bool;
    pub fn register_isa_ports(mmio: u32, io: u32, io_shift: u32);
}

#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }

#[inline] pub unsafe fn __typesafe_io(addr: usize) -> *mut core::ffi::c_void { addr as *mut core::ffi::c_void }
#[inline] pub unsafe fn __raw_writeb(val: u8, addr: *mut core::ffi::c_void) { core::ptr::write_volatile(addr as *mut u8, val); }
#[inline] pub unsafe fn __raw_readb(addr: *const core::ffi::c_void) -> u8 { core::ptr::read_volatile(addr as *const u8) }
#[inline] pub unsafe fn __raw_writew(val: u16, addr: *mut core::ffi::c_void) { core::ptr::write_volatile(addr as *mut u16, val); }
#[inline] pub unsafe fn __raw_readw(addr: *const core::ffi::c_void) -> u16 { core::ptr::read_volatile(addr as *const u16) }
#[inline] pub unsafe fn __raw_writel(val: u32, addr: *mut core::ffi::c_void) { core::ptr::write_volatile(addr as *mut u32, val); }
#[inline] pub unsafe fn __raw_readl(addr: *const core::ffi::c_void) -> u32 { core::ptr::read_volatile(addr as *const u32) }

#[inline] pub unsafe fn outb(v: u8, p: usize) { __raw_writeb(v, __typesafe_io(p)); }
#[inline] pub unsafe fn outw(v: u16, p: usize) { __raw_writew(v.to_le(), __typesafe_io(p)); }
#[inline] pub unsafe fn outl(v: u32, p: usize) { __raw_writel(v.to_le(), __typesafe_io(p)); }
#[inline] pub unsafe fn inb(p: usize) -> u8 { __raw_readb(__typesafe_io(p)) }
#[inline] pub unsafe fn inw(p: usize) -> u16 { u16::from_le(__raw_readw(__typesafe_io(p))) }
#[inline] pub unsafe fn inl(p: usize) -> u32 { u32::from_le(__raw_readl(__typesafe_io(p))) }

#[inline] pub unsafe fn readb_relaxed(c: *const core::ffi::c_void) -> u8 { __raw_readb(c) }
#[inline] pub unsafe fn readw_relaxed(c: *const core::ffi::c_void) -> u16 { u16::from_le(__raw_readw(c)) }
#[inline] pub unsafe fn readl_relaxed(c: *const core::ffi::c_void) -> u32 { u32::from_le(__raw_readl(c)) }
#[inline] pub unsafe fn writeb_relaxed(v: u8, c: *mut core::ffi::c_void) { __raw_writeb(v,c) }
#[inline] pub unsafe fn writew_relaxed(v: u16, c: *mut core::ffi::c_void) { __raw_writew(v.to_le(),c) }
#[inline] pub unsafe fn writel_relaxed(v: u32, c: *mut core::ffi::c_void) { __raw_writel(v.to_le(),c) }
#[inline] pub unsafe fn readb(c: *const core::ffi::c_void) -> u8 { readb_relaxed(c) }
#[inline] pub unsafe fn readw(c: *const core::ffi::c_void) -> u16 { readw_relaxed(c) }
#[inline] pub unsafe fn readl(c: *const core::ffi::c_void) -> u32 { readl_relaxed(c) }
#[inline] pub unsafe fn writeb(v: u8,c:*mut core::ffi::c_void) { writeb_relaxed(v,c) }
#[inline] pub unsafe fn writew(v: u16,c:*mut core::ffi::c_void) { writew_relaxed(v,c) }
#[inline] pub unsafe fn writel(v: u32,c:*mut core::ffi::c_void) { writel_relaxed(v,c) }

pub const IO_SPACE_LIMIT: usize = 0xfffff;
#[inline] pub unsafe fn __io(a: usize) -> *mut core::ffi::c_void { __typesafe_io(PCI_IO_VIRT_BASE + (a & IO_SPACE_LIMIT)) }
#[inline] pub unsafe fn ioread16be(p: *const core::ffi::c_void) -> u16 { u16::from_be(__raw_readw(p)) }
#[inline] pub unsafe fn ioread32be(p: *const core::ffi::c_void) -> u32 { u32::from_be(__raw_readl(p)) }
#[inline] pub unsafe fn iowrite16be(v: u16,p:*mut core::ffi::c_void) { __raw_writew(v.to_be(),p) }
#[inline] pub unsafe fn iowrite32be(v: u32,p:*mut core::ffi::c_void) { __raw_writel(v.to_be(),p) }

// CONFIG_ARM_DMA_MEM_BUFFERABLE, CONFIG_NEED_MACH_IO_H, CONFIG_PCI, CONFIG_PCMCIA,
// __LINUX_ARM_ARCH__, __ARMBE__, and CONFIG_MMU are build-time conditions from the C header.

#[inline] pub unsafe fn outsb(p: usize, d: *const core::ffi::c_void, l: i32) { __raw_writesb(__io(p), d, l) }
#[inline] pub unsafe fn outsw(p: usize, d: *const core::ffi::c_void, l: i32) { __raw_writesw(__io(p), d, l) }
#[inline] pub unsafe fn outsl(p: usize, d: *const core::ffi::c_void, l: i32) { __raw_writesl(__io(p), d, l) }
#[inline] pub unsafe fn insb(p: usize, d: *mut core::ffi::c_void, l: i32) { __raw_readsb(__io(p), d, l) }
#[inline] pub unsafe fn insw(p: usize, d: *mut core::ffi::c_void, l: i32) { __raw_readsw(__io(p), d, l) }
#[inline] pub unsafe fn insl(p: usize, d: *mut core::ffi::c_void, l: i32) { __raw_readsl(__io(p), d, l) }
#[inline] pub unsafe fn memset_io(dst: *mut core::ffi::c_void, c: u32, count: usize) { _memset_io(dst, c as i32, count) }
#[inline] pub unsafe fn memcpy_fromio(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize) { _memcpy_fromio(to, from, count) }
#[inline] pub unsafe fn memcpy_toio(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize) { _memcpy_toio(to, from, count) }
#[inline] pub unsafe fn ioremap_wt(res_cookie: usize, size: usize) -> *mut core::ffi::c_void { ioremap_wc(res_cookie, size) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
