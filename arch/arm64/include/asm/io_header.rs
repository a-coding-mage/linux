/* SPDX-License-Identifier: GPL-2.0-only */
/* Based on arch/arm/include/asm/io.h */

// Dependencies supplied by the surrounding kernel translation.

#[inline(always)]
pub unsafe fn __raw_writeb(val: u8, addr: *mut core::ffi::c_void) {
    core::arch::asm!("strb {val:w}, [{addr}]", val = in(reg) val, addr = in(reg) addr, options(nostack));
}

#[inline(always)]
pub unsafe fn __raw_writew(val: u16, addr: *mut core::ffi::c_void) {
    core::arch::asm!("strh {val:w}, [{addr}]", val = in(reg) val, addr = in(reg) addr, options(nostack));
}

#[inline(always)]
pub unsafe fn __raw_writel(val: u32, addr: *mut core::ffi::c_void) {
    core::arch::asm!("str {val:w}, [{addr}]", val = in(reg) val, addr = in(reg) addr, options(nostack));
}

#[inline(always)]
pub unsafe fn __raw_writeq(val: u64, addr: *mut core::ffi::c_void) {
    core::arch::asm!("str {val}, [{addr}]", val = in(reg) val, addr = in(reg) addr, options(nostack));
}

#[inline(always)]
pub unsafe fn __raw_readb(addr: *const core::ffi::c_void) -> u8 {
    let val: u8;
    core::arch::asm!("ldrb {val:w}, [{addr}]", val = out(reg) val, addr = in(reg) addr, options(nostack));
    val
}

#[inline(always)]
pub unsafe fn __raw_readw(addr: *const core::ffi::c_void) -> u16 {
    let val: u16;
    core::arch::asm!("ldrh {val:w}, [{addr}]", val = out(reg) val, addr = in(reg) addr, options(nostack));
    val
}

#[inline(always)]
pub unsafe fn __raw_readl(addr: *const core::ffi::c_void) -> u32 {
    let val: u32;
    core::arch::asm!("ldr {val:w}, [{addr}]", val = out(reg) val, addr = in(reg) addr, options(nostack));
    val
}

#[inline(always)]
pub unsafe fn __raw_readq(addr: *const core::ffi::c_void) -> u64 {
    let val: u64;
    core::arch::asm!("ldr {val}, [{addr}]", val = out(reg) val, addr = in(reg) addr, options(nostack));
    val
}

#[inline(always)]
pub unsafe fn __io_ar(v: usize) {
    dma_rmb();
    let mut tmp: usize;
    core::arch::asm!("eor {tmp}, {v}, {v}\ncbnz {tmp}, .", tmp = out(reg) tmp, v = in(reg) v, options(nostack));
}

#[inline(always)]
pub unsafe fn __io_bw() { dma_wmb(); }
#[inline(always)]
pub unsafe fn __io_br(_v: usize) {}
#[inline(always)]
pub unsafe fn __io_aw(_v: usize) {}
#[inline(always)]
pub unsafe fn __iormb(v: usize) { __io_ar(v); }
#[inline(always)]
pub unsafe fn __iowmb() { __io_bw(); }
#[inline(always)]
pub unsafe fn __iomb() { dma_mb(); }

pub const ARCH_HAS_DEV_PORT: bool = true;
pub const IO_SPACE_LIMIT: usize = PCI_IO_SIZE - 1;
pub const PCI_IOBASE: *mut core::ffi::c_void = PCI_IO_START as *mut core::ffi::c_void;

#[inline(always)]
pub unsafe fn __const_memcpy_toio_aligned32(to: *mut u32, from: *const u32, count: usize) {
    match count { 8 | 4 | 2 => for i in 0..count { __raw_writel(*from.add(i), to.add(i) as *mut core::ffi::c_void); }, 1 => __raw_writel(*from, to as *mut core::ffi::c_void), _ => core::hint::unreachable_unchecked() }
}

#[inline(always)]
pub unsafe fn __iowrite32_copy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize) {
    if count == 8 || count == 4 || count == 2 || count == 1 { __const_memcpy_toio_aligned32(to as *mut u32, from as *const u32, count); dgh(); } else { __iowrite32_copy_full(to, from, count); }
}

#[inline(always)]
pub unsafe fn __const_memcpy_toio_aligned64(to: *mut u64, from: *const u64, count: usize) {
    match count { 8 | 4 | 2 => for i in 0..count { __raw_writeq(*from.add(i), to.add(i) as *mut core::ffi::c_void); }, 1 => __raw_writeq(*from, to as *mut core::ffi::c_void), _ => core::hint::unreachable_unchecked() }
}

#[inline(always)]
pub unsafe fn __iowrite64_copy(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize) {
    if count == 8 || count == 4 || count == 2 || count == 1 { __const_memcpy_toio_aligned64(to as *mut u64, from as *const u64, count); dgh(); } else { __iowrite64_copy_full(to, from, count); }
}

extern "C" {
    pub fn dma_rmb();
    pub fn dma_wmb();
    pub fn dma_mb();
    pub fn dgh();
    pub fn __iowrite32_copy_full(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
    pub fn __iowrite64_copy_full(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, count: usize);
    pub fn arm64_ioremap_prot_hook_register(hook: Option<unsafe extern "C" fn(usize, usize, *mut pgprot_t) -> i32>) -> i32;
    pub fn __ioremap_prot(phys: usize, size: usize, prot: pgprot_t) -> *mut core::ffi::c_void;
    pub fn valid_phys_addr_range(addr: usize, size: usize) -> i32;
    pub fn valid_mmap_phys_addr_range(pfn: usize, size: usize) -> i32;
    pub fn arm64_memremap_can_ram_remap(offset: usize, size: usize, flags: usize) -> bool;
}

pub type ioremap_prot_hook_t = unsafe extern "C" fn(usize, usize, *mut pgprot_t) -> i32;

#[repr(C)]
pub struct pgprot_t(pub usize);

#[inline]
pub unsafe fn ioremap_prot(phys: usize, size: usize, user_prot: pgprot_t) -> *mut core::ffi::c_void {
    __ioremap_prot(phys, size, user_prot)
}

#[inline]
pub unsafe fn ioremap(phys: usize, size: usize) -> *mut core::ffi::c_void { __ioremap_prot(phys, size, pgprot_t(PROT_DEVICE_NGNRE)) }
#[inline]
pub unsafe fn ioremap_wc(phys: usize, size: usize) -> *mut core::ffi::c_void { __ioremap_prot(phys, size, pgprot_t(PROT_NORMAL_NC)) }
#[inline]
pub unsafe fn ioremap_np(phys: usize, size: usize) -> *mut core::ffi::c_void { __ioremap_prot(phys, size, pgprot_t(PROT_DEVICE_NGNRNE)) }
#[inline]
pub unsafe fn ioremap_encrypted(phys: usize, size: usize) -> *mut core::ffi::c_void { __ioremap_prot(phys, size, PAGE_KERNEL) }

#[inline]
pub unsafe fn ioread16be(p: *const core::ffi::c_void) -> u16 { let v = __raw_readw(p); __iormb(v as usize); u16::from_be(v) }
#[inline]
pub unsafe fn ioread32be(p: *const core::ffi::c_void) -> u32 { let v = __raw_readl(p); __iormb(v as usize); u32::from_be(v) }
#[inline]
pub unsafe fn ioread64be(p: *const core::ffi::c_void) -> u64 { let v = __raw_readq(p); __iormb(v as usize); u64::from_be(v) }
#[inline]
pub unsafe fn iowrite16be(v: u16, p: *mut core::ffi::c_void) { __iowmb(); __raw_writew(v.to_be(), p); }
#[inline]
pub unsafe fn iowrite32be(v: u32, p: *mut core::ffi::c_void) { __iowmb(); __raw_writel(v.to_be(), p); }
#[inline]
pub unsafe fn iowrite64be(v: u64, p: *mut core::ffi::c_void) { __iowmb(); __raw_writeq(v.to_be(), p); }

#[inline]
pub unsafe fn ioremap_cache(addr: usize, size: usize) -> *mut core::ffi::c_void { __ioremap_prot(addr, size, pgprot_t(PROT_NORMAL)) }

pub const ARCH_HAS_VALID_PHYS_ADDR_RANGE: bool = true;

#[inline]
pub unsafe fn arm64_is_protected_mmio(phys_addr: usize, size: usize) -> bool {
    if is_realm_world() { arm64_rsi_is_protected(phys_addr, size) } else { false }
}

extern "C" {
    fn is_realm_world() -> bool;
    fn arm64_rsi_is_protected(phys_addr: usize, size: usize) -> bool;
}

// External constants and types supplied by other translated headers.
extern "C" {
    static PCI_IO_SIZE: usize;
    static PCI_IO_START: usize;
    static PROT_DEVICE_NGNRE: usize;
    static PROT_NORMAL_NC: usize;
    static PROT_DEVICE_NGNRNE: usize;
    static PAGE_KERNEL: pgprot_t;
    static PROT_NORMAL: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
