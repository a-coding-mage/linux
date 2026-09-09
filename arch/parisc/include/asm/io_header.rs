/* SPDX-License-Identifier: GPL-2.0 */
// Translated from parisc/include/asm/io.h.

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn virt_to_phys<T>(a: *const T) -> ::core::ffi::c_ulong {
    __pa(a)
}

#[inline]
pub unsafe fn phys_to_virt<T>(a: ::core::ffi::c_ulong) -> *mut T {
    __va(a)
}

#[inline]
pub unsafe fn isa_bus_to_virt(_addr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    BUG();
    0
}

#[inline]
pub unsafe fn isa_virt_to_bus(_addr: *mut ::core::ffi::c_void) -> ::core::ffi::c_ulong {
    BUG();
    0
}

/* Memory mapped I/O.  The PA-specific inline assembly is retained as a
 * narrowly placed target-dependent operation. */
#[inline]
pub unsafe fn gsc_readb(addr: ::core::ffi::c_ulong) -> u8 {
    let mut ret: u8;
    ::core::arch::asm!("rsm  {sm}, {flags}\nldbx 0({addr}), {ret}\nmtsm {flags}",
        sm = const PSW_SM_D, flags = lateout(reg) _, ret = lateout(reg) ret,
        addr = in(reg) addr, options(nostack));
    ret
}

#[inline]
pub unsafe fn gsc_readw(addr: ::core::ffi::c_ulong) -> u16 {
    let mut ret: u16;
    ::core::arch::asm!("rsm  {sm}, {flags}\nldhx 0({addr}), {ret}\nmtsm {flags}",
        sm = const PSW_SM_D, flags = lateout(reg) _, ret = lateout(reg) ret,
        addr = in(reg) addr, options(nostack));
    ret
}

#[inline]
pub unsafe fn gsc_readl(addr: ::core::ffi::c_ulong) -> u32 {
    let mut ret: u32;
    ::core::arch::asm!("ldwax 0({addr}), {ret}", addr = in(reg) addr, ret = lateout(reg) ret);
    ret
}

#[inline]
pub unsafe fn gsc_readq(addr: ::core::ffi::c_ulong) -> u64 {
    #[cfg(CONFIG_64BIT)]
    { let mut ret: u64; ::core::arch::asm!("ldda 0({addr}), {ret}", addr=in(reg) addr, ret=lateout(reg) ret); ret }
    #[cfg(not(CONFIG_64BIT))]
    { ((gsc_readl(addr) as u64) << 32) | gsc_readl(addr.wrapping_add(4)) as u64 }
}

#[inline]
pub unsafe fn gsc_writeb(val: u8, addr: ::core::ffi::c_ulong) { ::core::arch::asm!("rsm {sm}, {flags}\nstbs {val}, 0({addr})\nmtsm {flags}", sm=const PSW_SM_D, flags=lateout(reg) _, val=in(reg) val, addr=in(reg) addr, options(nostack)); }
#[inline]
pub unsafe fn gsc_writew(val: u16, addr: ::core::ffi::c_ulong) { ::core::arch::asm!("rsm {sm}, {flags}\nsths {val}, 0({addr})\nmtsm {flags}", sm=const PSW_SM_D, flags=lateout(reg) _, val=in(reg) val, addr=in(reg) addr, options(nostack)); }
#[inline]
pub unsafe fn gsc_writel(val: u32, addr: ::core::ffi::c_ulong) { ::core::arch::asm!("stwas {val}, 0({addr})", val=in(reg) val, addr=in(reg) addr); }
#[inline]
pub unsafe fn gsc_writeq(val: u64, addr: ::core::ffi::c_ulong) {
    #[cfg(CONFIG_64BIT)]
    { ::core::arch::asm!("stda {val}, 0({addr})", val=in(reg) val, addr=in(reg) addr); }
    #[cfg(not(CONFIG_64BIT))]
    { gsc_writel((val >> 32) as u32, addr); gsc_writel(val as u32, addr.wrapping_add(4)); }
}

pub const _PAGE_IOREMAP: usize = _PAGE_PRESENT | _PAGE_RW | _PAGE_DIRTY | _PAGE_ACCESSED | _PAGE_NO_CACHE;
#[inline] pub unsafe fn ioremap_wc(addr: usize, size: usize) -> *mut ::core::ffi::c_void { ioremap_prot(addr, size, __pgprot(_PAGE_IOREMAP)) }

pub const IO_SPACE_LIMIT: u32 = 0x00ff_ffff;
#[inline] pub const fn F_EXTEND(x: u64) -> ::core::ffi::c_ulong { (x | 0xffff_ffff_0000_0000) as ::core::ffi::c_ulong }

extern "C" {
    #[cfg(CONFIG_PCI)]
    pub fn inb(addr: ::core::ffi::c_int) -> u8; #[cfg(CONFIG_PCI)] pub fn inw(addr: ::core::ffi::c_int) -> u16; #[cfg(CONFIG_PCI)] pub fn inl(addr: ::core::ffi::c_int) -> u32;
    #[cfg(CONFIG_PCI)] pub fn outb(val: u8, addr: ::core::ffi::c_int); #[cfg(CONFIG_PCI)] pub fn outw(val: u16, addr: ::core::ffi::c_int); #[cfg(CONFIG_PCI)] pub fn outl(val: u32, addr: ::core::ffi::c_int);
    pub fn eisa_in8(port: u16) -> u8; pub fn eisa_in16(port: u16) -> u16; pub fn eisa_in32(port: u16) -> u32;
    pub fn eisa_out8(data: u8, port: u16); pub fn eisa_out16(data: u16, port: u16); pub fn eisa_out32(data: u32, port: u16);
    pub fn insb(port: ::core::ffi::c_ulong, dst: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn insw(port: ::core::ffi::c_ulong, dst: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn insl(port: ::core::ffi::c_ulong, dst: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn outsb(port: ::core::ffi::c_ulong, src: *const ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn outsw(port: ::core::ffi::c_ulong, src: *const ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn outsl(port: ::core::ffi::c_ulong, src: *const ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn ioport_map(port: ::core::ffi::c_ulong, nr: u32) -> *mut ::core::ffi::c_void;
    pub fn ioport_unmap(addr: *mut ::core::ffi::c_void);
    pub fn ioread8(addr: *const ::core::ffi::c_void) -> u32; pub fn ioread16(addr: *const ::core::ffi::c_void) -> u32;
    pub fn ioread16be(addr: *const ::core::ffi::c_void) -> u32; pub fn ioread32(addr: *const ::core::ffi::c_void) -> u32; pub fn ioread32be(addr: *const ::core::ffi::c_void) -> u32;
    pub fn iowrite8(val: u8, addr: *mut ::core::ffi::c_void); pub fn iowrite16(val: u16, addr: *mut ::core::ffi::c_void); pub fn iowrite16be(val: u16, addr: *mut ::core::ffi::c_void);
    pub fn iowrite32(val: u32, addr: *mut ::core::ffi::c_void); pub fn iowrite32be(val: u32, addr: *mut ::core::ffi::c_void);
    pub fn ioread8_rep(port: *const ::core::ffi::c_void, buf: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn ioread16_rep(port: *const ::core::ffi::c_void, buf: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn ioread32_rep(port: *const ::core::ffi::c_void, buf: *mut ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn iowrite8_rep(port: *mut ::core::ffi::c_void, buf: *const ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn iowrite16_rep(port: *mut ::core::ffi::c_void, buf: *const ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn iowrite32_rep(port: *mut ::core::ffi::c_void, buf: *const ::core::ffi::c_void, count: ::core::ffi::c_ulong);
    pub fn devmem_is_allowed(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    #[cfg(CONFIG_64BIT)] pub fn ioread64(addr: *const ::core::ffi::c_void) -> u64;
    #[cfg(CONFIG_64BIT)] pub fn ioread64be(addr: *const ::core::ffi::c_void) -> u64;
    #[cfg(CONFIG_64BIT)] pub fn iowrite64(val: u64, addr: *mut ::core::ffi::c_void);
    #[cfg(CONFIG_64BIT)] pub fn iowrite64be(val: u64, addr: *mut ::core::ffi::c_void);
}

// The following identity macros in the C header are represented directly by
// the corresponding Rust items.  Port-space aliases retain their intent.
pub use eisa_in8 as inb_eisa;
pub use eisa_in16 as inw_eisa;
pub use eisa_in32 as inl_eisa;
pub use eisa_out8 as outb_eisa;
pub use eisa_out16 as outw_eisa;
pub use eisa_out32 as outl_eisa;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
