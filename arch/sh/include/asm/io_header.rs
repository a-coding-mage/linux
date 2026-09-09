/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Convention:
 * read{b,w,l,q}/write{b,w,l,q} are for PCI; in{b,w,l}/out{b,w,l} are for ISA.
 * Pausing and string variants are also provided.  The original header includes
 * architecture and generic I/O headers; those dependencies are supplied elsewhere.
 */

pub type U8 = u8;
pub type U16 = u16;
pub type U32 = u32;
pub type U64 = u64;

extern "C" {
    pub fn __chk_io_ptr<T>(addr: *const T);
    pub fn ioswabb(v: u8) -> u8;
    pub fn ioswabw(v: u16) -> u16;
    pub fn ioswabl(v: u32) -> u32;
    pub fn ioswabq(v: u64) -> u64;
    pub fn rmb();
    pub fn wmb();
    pub fn barrier();
    pub fn jump_to_uncached();
    pub fn back_to_cached();
}

#[inline]
pub unsafe fn __raw_writeb(v: u8, a: *mut u8) { __chk_io_ptr(a); core::ptr::write_volatile(a, v); }
#[inline]
pub unsafe fn __raw_writew(v: u16, a: *mut u16) { __chk_io_ptr(a); core::ptr::write_volatile(a, v); }
#[inline]
pub unsafe fn __raw_writel(v: u32, a: *mut u32) { __chk_io_ptr(a); core::ptr::write_volatile(a, v); }
#[inline]
pub unsafe fn __raw_writeq(v: u64, a: *mut u64) { __chk_io_ptr(a); core::ptr::write_volatile(a, v); }
#[inline]
pub unsafe fn __raw_readb(a: *const u8) -> u8 { __chk_io_ptr(a); core::ptr::read_volatile(a) }
#[inline]
pub unsafe fn __raw_readw(a: *const u16) -> u16 { __chk_io_ptr(a); core::ptr::read_volatile(a) }
#[inline]
pub unsafe fn __raw_readl(a: *const u32) -> u32 { __chk_io_ptr(a); core::ptr::read_volatile(a) }
#[inline]
pub unsafe fn __raw_readq(a: *const u64) -> u64 { __chk_io_ptr(a); core::ptr::read_volatile(a) }

#[inline] pub unsafe fn readb_relaxed(a: *const u8) -> u8 { ioswabb(__raw_readb(a)) }
#[inline] pub unsafe fn readw_relaxed(a: *const u16) -> u16 { ioswabw(__raw_readw(a)) }
#[inline] pub unsafe fn readl_relaxed(a: *const u32) -> u32 { ioswabl(__raw_readl(a)) }
#[inline] pub unsafe fn readq_relaxed(a: *const u64) -> u64 { ioswabq(__raw_readq(a)) }
#[inline] pub unsafe fn writeb_relaxed(v: u8, a: *mut u8) { __raw_writeb(ioswabb(v), a) }
#[inline] pub unsafe fn writew_relaxed(v: u16, a: *mut u16) { __raw_writew(ioswabw(v), a) }
#[inline] pub unsafe fn writel_relaxed(v: u32, a: *mut u32) { __raw_writel(ioswabl(v), a) }
#[inline] pub unsafe fn writeq_relaxed(v: u64, a: *mut u64) { __raw_writeq(ioswabq(v), a) }
#[inline] pub unsafe fn readb(a: *const u8) -> u8 { let r = readb_relaxed(a); rmb(); r }
#[inline] pub unsafe fn readw(a: *const u16) -> u16 { let r = readw_relaxed(a); rmb(); r }
#[inline] pub unsafe fn readl(a: *const u32) -> u32 { let r = readl_relaxed(a); rmb(); r }
#[inline] pub unsafe fn readq(a: *const u64) -> u64 { let r = readq_relaxed(a); rmb(); r }
#[inline] pub unsafe fn writeb(v: u8, a: *mut u8) { wmb(); writeb_relaxed(v, a) }
#[inline] pub unsafe fn writew(v: u16, a: *mut u16) { wmb(); writew_relaxed(v, a) }
#[inline] pub unsafe fn writel(v: u32, a: *mut u32) { wmb(); writel_relaxed(v, a) }
#[inline] pub unsafe fn writeq(v: u64, a: *mut u64) { wmb(); writeq_relaxed(v, a) }

pub unsafe fn readsb(p: *const u8, d: *mut u8, l: usize) { for i in 0..l { *d.add(i) = __raw_readb(p); } }
pub unsafe fn readsw(p: *const u16, d: *mut u16, l: usize) { for i in 0..l { *d.add(i) = __raw_readw(p); } }
pub unsafe fn readsl(p: *const u32, d: *mut u32, l: usize) { for i in 0..l { *d.add(i) = __raw_readl(p); } }
pub unsafe fn writesb(p: *mut u8, d: *const u8, l: usize) { for i in 0..l { __raw_writeb(*d.add(i), p); } }
pub unsafe fn writesw(p: *mut u16, d: *const u16, l: usize) { for i in 0..l { __raw_writew(*d.add(i), p); } }
pub unsafe fn writesl(p: *mut u32, d: *const u32, l: usize) { for i in 0..l { __raw_writel(*d.add(i), p); } }

#[inline] pub unsafe fn readb_uncached(addr: usize) -> u8 { jump_to_uncached(); let r = __raw_readb(addr as *const u8); back_to_cached(); r }
#[inline] pub unsafe fn readw_uncached(addr: usize) -> u16 { jump_to_uncached(); let r = __raw_readw(addr as *const u16); back_to_cached(); r }
#[inline] pub unsafe fn readl_uncached(addr: usize) -> u32 { jump_to_uncached(); let r = __raw_readl(addr as *const u32); back_to_cached(); r }
#[inline] pub unsafe fn readq_uncached(addr: usize) -> u64 { jump_to_uncached(); let r = __raw_readq(addr as *const u64); back_to_cached(); r }
#[inline] pub unsafe fn writeb_uncached(v: u8, addr: usize) { jump_to_uncached(); __raw_writeb(v, addr as *mut u8); back_to_cached(); }
#[inline] pub unsafe fn writew_uncached(v: u16, addr: usize) { jump_to_uncached(); __raw_writew(v, addr as *mut u16); back_to_cached(); }
#[inline] pub unsafe fn writel_uncached(v: u32, addr: usize) { jump_to_uncached(); __raw_writel(v, addr as *mut u32); back_to_cached(); }
#[inline] pub unsafe fn writeq_uncached(v: u64, addr: usize) { jump_to_uncached(); __raw_writeq(v, addr as *mut u64); back_to_cached(); }

extern "C" {
    pub fn __raw_writesl(addr: *mut u32, data: *const u32, longlen: i32);
    pub fn __raw_readsl(addr: *const u32, data: *mut u32, longlen: i32);
    pub fn ioport_map(port: usize, nr: u32) -> *mut u8;
    pub static mut sh_io_port_base: usize;
    pub fn __ioport_map(addr: usize, size: u32) -> *mut u8;
    pub fn memcpy_fromio(to: *mut core::ffi::c_void, from: *const u8, n: usize);
    pub fn memcpy_toio(to: *mut u8, from: *const core::ffi::c_void, n: usize);
    pub fn memset_io(s: *mut u8, c: i32, n: usize);
    pub fn peek_real_address_q(addr: u64) -> u64;
    pub fn poke_real_address_q(addr: u64, val: u64) -> u64;
    pub fn valid_phys_addr_range(addr: usize, size: usize) -> i32;
    pub fn valid_mmap_phys_addr_range(pfn: usize, size: usize) -> i32;
}

pub const IO_SPACE_LIMIT: u64 = 0xffff_ffff;
pub const ARCH_HAS_VALID_PHYS_ADDR_RANGE: bool = true;

/* CONFIG_HAS_IOPORT_MAP and CONFIG_MMU are build-time conditions from the C header. */
#[inline] pub unsafe fn __set_io_port_base(pbase: usize) { sh_io_port_base = pbase; barrier(); }
#[inline] pub unsafe fn inb(addr: usize) -> u8 { *((sh_io_port_base + addr) as *const u8) }
#[inline] pub unsafe fn inw(addr: usize) -> u16 { *((sh_io_port_base + addr) as *const u16) }
#[inline] pub unsafe fn inl(addr: usize) -> u32 { *((sh_io_port_base + addr) as *const u32) }
#[inline] pub unsafe fn outb(x: u8, addr: usize) { *((sh_io_port_base + addr) as *mut u8) = x; }
#[inline] pub unsafe fn outw(x: u16, addr: usize) { *((sh_io_port_base + addr) as *mut u16) = x; }
#[inline] pub unsafe fn outl(x: u32, addr: usize) { *((sh_io_port_base + addr) as *mut u32) = x; }
#[inline] pub unsafe fn inb_p(addr: usize) -> u8 { inb(addr) }
#[inline] pub unsafe fn inw_p(addr: usize) -> u16 { inw(addr) }
#[inline] pub unsafe fn inl_p(addr: usize) -> u32 { inl(addr) }
#[inline] pub unsafe fn outb_p(x: u8, addr: usize) { outb(x, addr) }
#[inline] pub unsafe fn outw_p(x: u16, addr: usize) { outw(x, addr) }
#[inline] pub unsafe fn outl_p(x: u32, addr: usize) { outl(x, addr) }

pub const fn virt_to_phys(address: usize) -> usize { address }
pub const fn phys_to_virt(address: usize) -> usize { address }
/* Under CONFIG_MMU these map to __pa(address) and __va(address), respectively. */

/* CONFIG_MMU mapping declarations: ioremap_prot, iounmap, PAGE_KERNEL, and
 * PAGE_KERNEL_NOCACHE are supplied by the architecture/generic dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
