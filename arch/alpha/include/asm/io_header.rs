/* SPDX-License-Identifier: GPL-2.0 */
// Translation of alpha/include/asm/io.h.
// C includes, header guards, and build-time include selection are intentionally
// represented as dependency/configuration comments.

#[cfg(feature = "use_48_bit_kseg")]
pub const IDENT_ADDR: usize = 0xffff_8000_0000_0000;
#[cfg(not(feature = "use_48_bit_kseg"))]
pub const IDENT_ADDR: usize = 0xffff_fc00_0000_0000;

extern "C" {
    pub static mut __direct_map_base: usize;
    pub static mut __direct_map_size: usize;

    pub fn swpipl(level: usize) -> usize;
    pub fn setipl(flags: usize);
    pub fn mb();
    pub fn barrier();

    pub fn inb(port: usize) -> u8;
    pub fn inw(port: usize) -> u16;
    pub fn inl(port: usize) -> u32;
    pub fn outb(b: u8, port: usize);
    pub fn outw(b: u16, port: usize);
    pub fn outl(b: u32, port: usize);

    pub fn readb(addr: *const core::ffi::c_void) -> u8;
    pub fn readw(addr: *const core::ffi::c_void) -> u16;
    pub fn readl(addr: *const core::ffi::c_void) -> u32;
    pub fn readq(addr: *const core::ffi::c_void) -> u64;
    pub fn writeb(b: u8, addr: *mut core::ffi::c_void);
    pub fn writew(b: u16, addr: *mut core::ffi::c_void);
    pub fn writel(b: u32, addr: *mut core::ffi::c_void);
    pub fn writeq(b: u64, addr: *mut core::ffi::c_void);

    pub fn __raw_readb(addr: *const core::ffi::c_void) -> u8;
    pub fn __raw_readw(addr: *const core::ffi::c_void) -> u16;
    pub fn __raw_readl(addr: *const core::ffi::c_void) -> u32;
    pub fn __raw_readq(addr: *const core::ffi::c_void) -> u64;
    pub fn __raw_writeb(b: u8, addr: *mut core::ffi::c_void);
    pub fn __raw_writew(b: u16, addr: *mut core::ffi::c_void);
    pub fn __raw_writel(b: u32, addr: *mut core::ffi::c_void);
    pub fn __raw_writeq(b: u64, addr: *mut core::ffi::c_void);

    pub fn ioread8(addr: *const core::ffi::c_void) -> u32;
    pub fn ioread16(addr: *const core::ffi::c_void) -> u32;
    pub fn ioread32(addr: *const core::ffi::c_void) -> u32;
    pub fn ioread64(addr: *const core::ffi::c_void) -> u64;
    pub fn iowrite8(b: u8, addr: *mut core::ffi::c_void);
    pub fn iowrite16(b: u16, addr: *mut core::ffi::c_void);
    pub fn iowrite32(b: u32, addr: *mut core::ffi::c_void);
    pub fn iowrite64(b: u64, addr: *mut core::ffi::c_void);

    pub fn ioread8_rep(port: *const core::ffi::c_void, buf: *mut core::ffi::c_void, count: usize);
    pub fn ioread16_rep(port: *const core::ffi::c_void, buf: *mut core::ffi::c_void, count: usize);
    pub fn ioread32_rep(port: *const core::ffi::c_void, buf: *mut core::ffi::c_void, count: usize);
    pub fn iowrite8_rep(port: *mut core::ffi::c_void, buf: *const core::ffi::c_void, count: usize);
    pub fn iowrite16_rep(port: *mut core::ffi::c_void, buf: *const core::ffi::c_void, count: usize);
    pub fn iowrite32_rep(port: *mut core::ffi::c_void, buf: *const core::ffi::c_void, count: usize);

    pub fn memcpy_fromio(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: isize);
    pub fn memcpy_toio(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: isize);
    pub fn _memset_c_io(dst: *mut core::ffi::c_void, value: usize, len: isize);

    pub fn readb_relaxed(addr: *const core::ffi::c_void) -> u8;
    pub fn readw_relaxed(addr: *const core::ffi::c_void) -> u16;
    pub fn readl_relaxed(addr: *const core::ffi::c_void) -> u32;
    pub fn readq_relaxed(addr: *const core::ffi::c_void) -> u64;

    pub fn insb(port: usize, dst: *mut core::ffi::c_void, count: usize);
    pub fn insw(port: usize, dst: *mut core::ffi::c_void, count: usize);
    pub fn insl(port: usize, dst: *mut core::ffi::c_void, count: usize);
    pub fn outsb(port: usize, src: *const core::ffi::c_void, count: usize);
    pub fn outsw(port: usize, src: *const core::ffi::c_void, count: usize);
    pub fn outsl(port: usize, src: *const core::ffi::c_void, count: usize);
}

pub const IO_SPACE_LIMIT: usize = 0xffff;

#[inline(always)]
pub unsafe fn __set_hae(new_hae: usize) {
    // Atomic IPL manipulation, machine-vector HAE cache/register access, and
    // the required read-back are external Alpha platform operations.
    let flags = swpipl(31);
    barrier();
    // The Alpha machine-vector HAE cache/register are intentionally left as
    // external dependencies because their declaration is supplied elsewhere.
    let _ = new_hae;
    mb();
    setipl(flags);
    barrier();
}

#[inline(always)]
pub unsafe fn set_hae(new_hae: usize) { __set_hae(new_hae); }

#[inline(always)]
pub unsafe fn virt_to_phys(address: *const core::ffi::c_void) -> usize {
    #[cfg(feature = "use_48_bit_kseg")]
    { (address as usize).wrapping_sub(IDENT_ADDR) }
    #[cfg(not(feature = "use_48_bit_kseg"))]
    {
        // Sign extension from bit 41 and processor physical-address cropping
        // require the external Alpha HWRPB `pa_bits` value.
        let mut phys = address as usize;
        phys = ((phys << (64 - 41)) as isize >> (64 - 41)) as usize;
        phys
    }
}

#[inline(always)]
pub unsafe fn phys_to_virt(address: usize) -> *mut core::ffi::c_void {
    #[cfg(feature = "use_48_bit_kseg")]
    { address.wrapping_add(IDENT_ADDR) as *mut core::ffi::c_void }
    #[cfg(not(feature = "use_48_bit_kseg"))]
    { (IDENT_ADDR.wrapping_add(address & ((1usize << 41) - 1))) as *mut core::ffi::c_void }
}

#[inline(always)]
pub unsafe fn isa_virt_to_bus(address: *const core::ffi::c_void) -> usize {
    let phys = virt_to_phys(address);
    let bus = phys.wrapping_add(__direct_map_base);
    if phys <= __direct_map_size { bus } else { 0 }
}

#[inline(always)]
pub unsafe fn isa_bus_to_virt(mut address: usize) -> *mut core::ffi::c_void {
    address = address.wrapping_sub(__direct_map_base);
    let virt = phys_to_virt(address);
    if (address as isize) <= 0 { core::ptr::null_mut() } else { virt }
}

#[inline(always)]
pub unsafe fn ioport_map(port: usize, _size: u32) -> *mut core::ffi::c_void {
    // IO_CONCAT(__IO_PREFIX,ioportmap)(port)
    port as *mut core::ffi::c_void
}

#[inline(always)]
pub unsafe fn ioport_unmap(_addr: *mut core::ffi::c_void) {}

#[inline(always)]
pub unsafe fn ioremap(port: usize, _size: usize) -> *mut core::ffi::c_void {
    port as *mut core::ffi::c_void
}

#[inline(always)]
pub unsafe fn iounmap(_addr: *const core::ffi::c_void) {}

#[inline(always)]
pub unsafe fn __is_ioaddr(_addr: usize) -> i32 { 0 }

#[inline(always)]
pub unsafe fn __is_mmio(_addr: *const core::ffi::c_void) -> i32 { 0 }

#[inline(always)]
pub const fn rtc_port(x: usize) -> usize { 0x70 + x }
pub const RTC_ALWAYS_BCD: i32 = 0;

#[inline(always)]
pub unsafe fn memset_io(addr: *mut core::ffi::c_void, c: u8, len: isize) {
    _memset_c_io(addr, 0x0101_0101_0101_0101usize.wrapping_mul(c as usize), len);
}

#[inline(always)]
pub unsafe fn memsetw_io(addr: *mut core::ffi::c_void, c: u16, len: isize) {
    _memset_c_io(addr, 0x0001_0001_0001_0001usize.wrapping_mul(c as usize), len);
}

#[inline(always)]
pub unsafe fn ioread16be(p: *const core::ffi::c_void) -> u32 { ioread16(p) .swap_bytes() }
#[inline(always)]
pub unsafe fn ioread32be(p: *const core::ffi::c_void) -> u32 { ioread32(p).swap_bytes() }
#[inline(always)]
pub unsafe fn ioread64be(p: *const core::ffi::c_void) -> u64 { ioread64(p).swap_bytes() }
#[inline(always)]
pub unsafe fn iowrite16be(v: u16, p: *mut core::ffi::c_void) { iowrite16(v.swap_bytes(), p) }
#[inline(always)]
pub unsafe fn iowrite32be(v: u32, p: *mut core::ffi::c_void) { iowrite32(v.swap_bytes(), p) }
#[inline(always)]
pub unsafe fn iowrite64be(v: u64, p: *mut core::ffi::c_void) { iowrite64(v.swap_bytes(), p) }

// ioremap/iounmap, machine-vector remapping, relaxed accessors, endian-swapped
// accessors, and architecture-generic I/O declarations are supplied by the
// corresponding Alpha machine-vector and asm-generic dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
