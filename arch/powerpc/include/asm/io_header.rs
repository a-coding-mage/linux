/* SPDX-License-Identifier: GPL-2.0-or-later */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* Translation of powerpc/include/asm/io.h.  Kernel-provided types and
 * functions referenced by this header remain external dependencies. */

pub const I8042_DATA_REG: usize = 0x60;
pub const FDC_BASE: usize = 0x3f0;
pub const SIO_CONFIG_RA: usize = 0x398;
pub const SIO_CONFIG_RD: usize = 0x399;

extern "C" {
    pub fn check_legacy_ioport(base_port: c_ulong) -> c_int;
    pub static mut isa_io_base: c_ulong;
    pub static mut pci_io_base: c_ulong;
    pub static mut pci_dram_offset: c_ulong;
    pub static mut isa_mem_base: resource_size_t;
    pub static mut isa_io_special: bool;
    pub fn _insb(addr: *const volatile u8, buf: *mut c_void, count: c_long);
    pub fn _outsb(addr: *mut volatile u8, buf: *const c_void, count: c_long);
    pub fn _insw(addr: *const volatile u16, buf: *mut c_void, count: c_long);
    pub fn _outsw(addr: *mut volatile u16, buf: *const c_void, count: c_long);
    pub fn _insl(addr: *const volatile u32, buf: *mut c_void, count: c_long);
    pub fn _outsl(addr: *mut volatile u32, buf: *const c_void, count: c_long);
    pub fn _memset_io(addr: *mut volatile c_void, c: c_int, n: c_ulong);
    pub fn _memcpy_fromio(dest: *mut c_void, src: *const volatile c_void, n: c_ulong);
    pub fn _memcpy_toio(dest: *mut volatile c_void, src: *const c_void, n: c_ulong);
    pub fn udelay(usecs: c_ulong);
    pub fn __pa(addr: c_ulong) -> c_ulong;
    pub fn __va(addr: c_ulong) -> *mut c_void;
}

pub type c_void = core::ffi::c_void;
pub type c_int = i32;
pub type c_long = isize;
pub type c_ulong = usize;
pub type resource_size_t = usize;
pub type phys_addr_t = usize;
pub type pgprot_t = usize;

#[inline(always)]
pub unsafe fn in_8(addr: *const volatile u8) -> u8 { core::ptr::read_volatile(addr as *const u8) }
#[inline(always)]
pub unsafe fn out_8(addr: *mut volatile u8, val: u8) { core::ptr::write_volatile(addr as *mut u8, val) }
#[inline(always)]
pub unsafe fn in_le16(addr: *const volatile u16) -> u16 { u16::from_le(core::ptr::read_volatile(addr as *const u16)) }
#[inline(always)]
pub unsafe fn in_le32(addr: *const volatile u32) -> u32 { u32::from_le(core::ptr::read_volatile(addr as *const u32)) }
#[inline(always)]
pub unsafe fn in_be16(addr: *const volatile u16) -> u16 { u16::from_be(core::ptr::read_volatile(addr as *const u16)) }
#[inline(always)]
pub unsafe fn in_be32(addr: *const volatile u32) -> u32 { u32::from_be(core::ptr::read_volatile(addr as *const u32)) }
#[inline(always)]
pub unsafe fn out_le16(addr: *mut volatile u16, val: u16) { core::ptr::write_volatile(addr as *mut u16, val.to_le()) }
#[inline(always)]
pub unsafe fn out_le32(addr: *mut volatile u32, val: u32) { core::ptr::write_volatile(addr as *mut u32, val.to_le()) }
#[inline(always)]
pub unsafe fn out_be16(addr: *mut volatile u16, val: u16) { core::ptr::write_volatile(addr as *mut u16, val.to_be()) }
#[inline(always)]
pub unsafe fn out_be32(addr: *mut volatile u32, val: u32) { core::ptr::write_volatile(addr as *mut u32, val.to_be()) }

#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn in_le64(addr: *const volatile u64) -> u64 { u64::from_le(core::ptr::read_volatile(addr as *const u64)) }
#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn in_be64(addr: *const volatile u64) -> u64 { u64::from_be(core::ptr::read_volatile(addr as *const u64)) }
#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn out_le64(addr: *mut volatile u64, val: u64) { core::ptr::write_volatile(addr as *mut u64, val.to_le()) }
#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn out_be64(addr: *mut volatile u64, val: u64) { core::ptr::write_volatile(addr as *mut u64, val.to_be()) }

#[inline(always)] pub unsafe fn readb(addr: *const volatile c_void) -> u8 { in_8(addr as *const volatile u8) }
#[inline(always)] pub unsafe fn readw(addr: *const volatile c_void) -> u16 { in_le16(addr as *const volatile u16) }
#[inline(always)] pub unsafe fn readl(addr: *const volatile c_void) -> u32 { in_le32(addr as *const volatile u32) }
#[inline(always)] pub unsafe fn readw_be(addr: *const volatile c_void) -> u16 { in_be16(addr as *const volatile u16) }
#[inline(always)] pub unsafe fn readl_be(addr: *const volatile c_void) -> u32 { in_be32(addr as *const volatile u32) }
#[inline(always)] pub unsafe fn writeb(val: u8, addr: *mut volatile c_void) { out_8(addr as *mut volatile u8, val) }
#[inline(always)] pub unsafe fn writew(val: u16, addr: *mut volatile c_void) { out_le16(addr as *mut volatile u16, val) }
#[inline(always)] pub unsafe fn writel(val: u32, addr: *mut volatile c_void) { out_le32(addr as *mut volatile u32, val) }
#[inline(always)] pub unsafe fn writew_be(val: u16, addr: *mut volatile c_void) { out_be16(addr as *mut volatile u16, val) }
#[inline(always)] pub unsafe fn writel_be(val: u32, addr: *mut volatile c_void) { out_be32(addr as *mut volatile u32, val) }

#[inline(always)] pub unsafe fn readsb(a: *const volatile c_void, b: *mut c_void, c: c_ulong) { _insb(a as _, b, c as _) }
#[inline(always)] pub unsafe fn readsw(a: *const volatile c_void, b: *mut c_void, c: c_ulong) { _insw(a as _, b, c as _) }
#[inline(always)] pub unsafe fn readsl(a: *const volatile c_void, b: *mut c_void, c: c_ulong) { _insl(a as _, b, c as _) }
#[inline(always)] pub unsafe fn writesb(a: *mut volatile c_void, b: *const c_void, c: c_ulong) { _outsb(a as _, b, c as _) }
#[inline(always)] pub unsafe fn writesw(a: *mut volatile c_void, b: *const c_void, c: c_ulong) { _outsw(a as _, b, c as _) }
#[inline(always)] pub unsafe fn writesl(a: *mut volatile c_void, b: *const c_void, c: c_ulong) { _outsl(a as _, b, c as _) }
#[inline(always)] pub unsafe fn memset_io(a: *mut volatile c_void, c: c_int, n: c_ulong) { _memset_io(a, c, n) }
#[inline(always)] pub unsafe fn memcpy_fromio(d: *mut c_void, s: *const volatile c_void, n: c_ulong) { _memcpy_fromio(d, s, n) }
#[inline(always)] pub unsafe fn memcpy_toio(d: *mut volatile c_void, s: *const c_void, n: c_ulong) { _memcpy_toio(d, s, n) }

#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn readq(a: *const volatile c_void) -> u64 { in_le64(a as _) }
#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn readq_be(a: *const volatile c_void) -> u64 { in_be64(a as _) }
#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn writeq(v: u64, a: *mut volatile c_void) { out_le64(a as _, v) }
#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn writeq_be(v: u64, a: *mut volatile c_void) { out_be64(a as _, v) }

#[inline(always)] pub unsafe fn iosync() { core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst) }
#[inline(always)] pub unsafe fn iobarrier_rw() { iosync() }
#[inline(always)] pub unsafe fn iobarrier_r() { iosync() }
#[inline(always)] pub unsafe fn iobarrier_w() { iosync() }

#[inline(always)] pub unsafe fn inb_p(port: c_ulong) -> u8 { inb(port) }
#[inline(always)] pub unsafe fn outb_p(val: u8, port: c_ulong) { udelay(1); outb(val, port) }
#[inline(always)] pub unsafe fn inw_p(port: c_ulong) -> u16 { inw(port) }
#[inline(always)] pub unsafe fn outw_p(val: u16, port: c_ulong) { udelay(1); outw(val, port) }
#[inline(always)] pub unsafe fn inl_p(port: c_ulong) -> u32 { inl(port) }
#[inline(always)] pub unsafe fn outl_p(val: u32, port: c_ulong) { udelay(1); outl(val, port) }

pub const IO_SPACE_LIMIT: c_ulong = !0;
pub const HAVE_ARCH_PIO_SIZE: usize = 1;
pub const PIO_OFFSET: c_ulong = 0;
/* FULL_IO_SIZE is supplied by the generic/kernel headers. */

extern "C" {
    pub fn ioremap(address: phys_addr_t, size: c_ulong) -> *mut volatile c_void;
    pub fn ioremap_wc(address: phys_addr_t, size: c_ulong) -> *mut volatile c_void;
    pub fn ioremap_wt(address: phys_addr_t, size: c_ulong) -> *mut volatile c_void;
    pub fn ioremap_coherent(address: phys_addr_t, size: c_ulong) -> *mut volatile c_void;
    pub fn ioremap_phb(address: phys_addr_t, size: c_ulong) -> *mut volatile c_void;
    pub fn early_ioremap_range(ea: c_ulong, pa: phys_addr_t, size: c_ulong, prot: pgprot_t) -> c_int;
    pub fn __ioremap_caller(address: phys_addr_t, size: c_ulong, prot: pgprot_t, caller: *mut c_void) -> *mut volatile c_void;
    pub fn iounmap(addr: *mut volatile c_void);
}

#[inline(always)] pub unsafe fn virt_to_phys(address: *const volatile c_void) -> c_ulong { __pa(address as c_ulong) }
#[inline(always)] pub unsafe fn phys_to_virt(address: c_ulong) -> *mut c_void { __va(address) }

#[cfg(target_pointer_width = "32")]
#[inline(always)] pub unsafe fn virt_to_bus(address: *mut volatile c_void) -> c_ulong { if address.is_null() { 0 } else { __pa(address as c_ulong) + pci_dram_offset } }
#[cfg(target_pointer_width = "32")]
#[inline(always)] pub unsafe fn bus_to_virt(address: c_ulong) -> *mut c_void { if address == 0 { core::ptr::null_mut() } else { __va(address - pci_dram_offset) } }

/* Source-level macro equivalents retained as declarative helpers. */
#[macro_export] macro_rules! setbits32 { ($a:expr, $v:expr) => { unsafe { $crate::out_be32($a, $crate::in_be32($a) | $v) } }; }
#[macro_export] macro_rules! clrbits32 { ($a:expr, $v:expr) => { unsafe { $crate::out_be32($a, $crate::in_be32($a) & !$v) } }; }
#[macro_export] macro_rules! setbits16 { ($a:expr, $v:expr) => { unsafe { $crate::out_be16($a, $crate::in_be16($a) | $v) } }; }
#[macro_export] macro_rules! clrbits16 { ($a:expr, $v:expr) => { unsafe { $crate::out_be16($a, $crate::in_be16($a) & !$v) } }; }
#[macro_export] macro_rules! setbits8 { ($a:expr, $v:expr) => { unsafe { $crate::out_8($a, $crate::in_8($a) | $v) } }; }
#[macro_export] macro_rules! clrbits8 { ($a:expr, $v:expr) => { unsafe { $crate::out_8($a, $crate::in_8($a) & !$v) } }; }
#[macro_export] macro_rules! clrsetbits { ($out:ident, $in:ident, $a:expr, $clear:expr, $set:expr) => { unsafe { $out($a, $in($a) & !$clear | $set) } }; }

#[cfg(target_pointer_width = "64")]
#[inline(always)] pub unsafe fn __raw_writeq_be(v: u64, addr: *mut volatile c_void) { __raw_writeq(v.to_be(), addr) }
#[cfg(target_pointer_width = "64")]
extern "C" { pub fn __raw_writeq(v: u64, addr: *mut volatile c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
