// SPDX-License-Identifier: GPL-2.0
/* Implement the default iomap interfaces. */

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;

#[cfg(not(have_arch_pio_size))]
const PIO_OFFSET: usize = 0x10000;
#[cfg(not(have_arch_pio_size))]
const PIO_MASK: usize = 0x0ffff;
#[cfg(not(have_arch_pio_size))]
const PIO_RESERVED: usize = 0x40000;

extern "C" {
    fn inb(port: usize) -> U8;
    fn inw(port: usize) -> U16;
    fn inl(port: usize) -> U32;
    fn outb(value: U8, port: usize);
    fn outw(value: U16, port: usize);
    fn outl(value: U32, port: usize);
    fn readb(addr: *const core::ffi::c_void) -> U8;
    fn readw(addr: *const core::ffi::c_void) -> U16;
    fn readl(addr: *const core::ffi::c_void) -> U32;
    fn readq(addr: *const core::ffi::c_void) -> U64;
    fn writeb(value: U8, addr: *mut core::ffi::c_void);
    fn writew(value: U16, addr: *mut core::ffi::c_void);
    fn writel(value: U32, addr: *mut core::ffi::c_void);
    fn writeq(value: U64, addr: *mut core::ffi::c_void);
    fn __raw_readb(addr: *const core::ffi::c_void) -> U8;
    fn __raw_readw(addr: *const core::ffi::c_void) -> U16;
    fn __raw_readl(addr: *const core::ffi::c_void) -> U32;
    fn __raw_writeb(value: U8, addr: *mut core::ffi::c_void);
    fn __raw_writew(value: U16, addr: *mut core::ffi::c_void);
    fn __raw_writel(value: U32, addr: *mut core::ffi::c_void);
    fn insb(port: usize, dst: *mut core::ffi::c_void, count: usize);
    fn insw(port: usize, dst: *mut core::ffi::c_void, count: usize);
    fn insl(port: usize, dst: *mut core::ffi::c_void, count: usize);
    fn outsb(port: usize, src: *const core::ffi::c_void, count: usize);
    fn outsw(port: usize, src: *const core::ffi::c_void, count: usize);
    fn outsl(port: usize, src: *const core::ffi::c_void, count: usize);
    fn iounmap(addr: *mut core::ffi::c_void);
    fn kmsan_check_memory(ptr: *const core::ffi::c_void, size: usize);
    fn kmsan_unpoison_memory(ptr: *mut core::ffi::c_void, size: usize);
    fn warn_bad_io_access(port: usize, access: *const core::ffi::c_char);
}

static mut COUNT: i32 = 10;

unsafe fn bad_io_access(port: usize, access: *const core::ffi::c_char) {
    if COUNT != 0 {
        COUNT -= 1;
        warn_bad_io_access(port, access);
    }
}

macro_rules! io_cond {
    ($addr:expr, $pio:expr, $mmio:expr) => {{
        let mut port = $addr as usize;
        if port >= PIO_RESERVED {
            $mmio
        } else if port > PIO_OFFSET {
            port &= PIO_MASK;
            $pio
        } else {
            unsafe { bad_io_access(port, stringify!($pio).as_ptr() as *const core::ffi::c_char) };
            Default::default()
        }
    }};
}

#[inline]
unsafe fn pio_read16be(port: usize) -> U16 { inw(port).swap_bytes() }
#[inline]
unsafe fn pio_read32be(port: usize) -> U32 { inl(port).swap_bytes() }
#[inline]
unsafe fn mmio_read16be(addr: *const core::ffi::c_void) -> U16 { readw(addr).swap_bytes() }
#[inline]
unsafe fn mmio_read32be(addr: *const core::ffi::c_void) -> U32 { readl(addr).swap_bytes() }
#[inline]
unsafe fn mmio_read64be(addr: *const core::ffi::c_void) -> U64 { readq(addr).swap_bytes() }
#[inline]
unsafe fn pio_write16be(value: U16, port: usize) { outw(value.swap_bytes(), port) }
#[inline]
unsafe fn pio_write32be(value: U32, port: usize) { outl(value.swap_bytes(), port) }
#[inline]
unsafe fn mmio_write16be(value: U16, addr: *mut core::ffi::c_void) { writew(value.swap_bytes(), addr) }
#[inline]
unsafe fn mmio_write32be(value: U32, addr: *mut core::ffi::c_void) { writel(value.swap_bytes(), addr) }
#[inline]
unsafe fn mmio_write64be(value: U64, addr: *mut core::ffi::c_void) { writeq(value.swap_bytes(), addr) }

pub unsafe fn ioread8(addr: *const core::ffi::c_void) -> U32 { io_cond!(addr, inb(port) as U32, readb(addr) as U32) }
pub unsafe fn ioread16(addr: *const core::ffi::c_void) -> U32 { io_cond!(addr, inw(port) as U32, readw(addr) as U32) }
pub unsafe fn ioread16be(addr: *const core::ffi::c_void) -> U32 { io_cond!(addr, pio_read16be(port) as U32, mmio_read16be(addr) as U32) }
pub unsafe fn ioread32(addr: *const core::ffi::c_void) -> U32 { io_cond!(addr, inl(port), readl(addr)) }
pub unsafe fn ioread32be(addr: *const core::ffi::c_void) -> U32 { io_cond!(addr, pio_read32be(port), mmio_read32be(addr)) }

#[cfg(target_pointer_width = "64")]
unsafe fn pio_read64_lo_hi(port: usize) -> U64 { (inl(port) as U64) | ((inl(port + 4) as U64) << 32) }
#[cfg(target_pointer_width = "64")]
unsafe fn pio_read64_hi_lo(port: usize) -> U64 { ((inl(port + 4) as U64) << 32) | inl(port) as U64 }
#[cfg(target_pointer_width = "64")]
unsafe fn pio_read64be_lo_hi(port: usize) -> U64 { (pio_read32be(port + 4) as U64) | ((pio_read32be(port) as U64) << 32) }
#[cfg(target_pointer_width = "64")]
unsafe fn pio_read64be_hi_lo(port: usize) -> U64 { (pio_read32be(port + 4) as U64) | ((pio_read32be(port) as U64) << 32) }

#[cfg(target_pointer_width = "64")]
pub unsafe fn __ioread64_lo_hi(addr: *const core::ffi::c_void) -> U64 { io_cond!(addr, pio_read64_lo_hi(port), readq(addr)) }
#[cfg(target_pointer_width = "64")]
pub unsafe fn __ioread64_hi_lo(addr: *const core::ffi::c_void) -> U64 { io_cond!(addr, pio_read64_hi_lo(port), readq(addr)) }
#[cfg(target_pointer_width = "64")]
pub unsafe fn __ioread64be_lo_hi(addr: *const core::ffi::c_void) -> U64 { io_cond!(addr, pio_read64be_lo_hi(port), mmio_read64be(addr)) }
#[cfg(target_pointer_width = "64")]
pub unsafe fn __ioread64be_hi_lo(addr: *const core::ffi::c_void) -> U64 { io_cond!(addr, pio_read64be_hi_lo(port), mmio_read64be(addr)) }

pub unsafe fn iowrite8(value: U8, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 1); io_cond!(addr, outb(value, port), writeb(value, addr)); }
pub unsafe fn iowrite16(value: U16, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 2); io_cond!(addr, outw(value, port), writew(value, addr)); }
pub unsafe fn iowrite16be(value: U16, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 2); io_cond!(addr, pio_write16be(value, port), mmio_write16be(value, addr)); }
pub unsafe fn iowrite32(value: U32, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 4); io_cond!(addr, outl(value, port), writel(value, addr)); }
pub unsafe fn iowrite32be(value: U32, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 4); io_cond!(addr, pio_write32be(value, port), mmio_write32be(value, addr)); }

#[cfg(target_pointer_width = "64")]
unsafe fn pio_write64_lo_hi(value: U64, port: usize) { outl(value as U32, port); outl((value >> 32) as U32, port + 4); }
#[cfg(target_pointer_width = "64")]
unsafe fn pio_write64_hi_lo(value: U64, port: usize) { outl((value >> 32) as U32, port + 4); outl(value as U32, port); }
#[cfg(target_pointer_width = "64")]
unsafe fn pio_write64be_lo_hi(value: U64, port: usize) { pio_write32be(value as U32, port + 4); pio_write32be((value >> 32) as U32, port); }
#[cfg(target_pointer_width = "64")]
unsafe fn pio_write64be_hi_lo(value: U64, port: usize) { pio_write32be((value >> 32) as U32, port); pio_write32be(value as U32, port + 4); }
#[cfg(target_pointer_width = "64")]
pub unsafe fn __iowrite64_lo_hi(value: U64, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 8); io_cond!(addr, pio_write64_lo_hi(value, port), writeq(value, addr)); }
#[cfg(target_pointer_width = "64")]
pub unsafe fn __iowrite64_hi_lo(value: U64, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 8); io_cond!(addr, pio_write64_hi_lo(value, port), writeq(value, addr)); }
#[cfg(target_pointer_width = "64")]
pub unsafe fn __iowrite64be_lo_hi(value: U64, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 8); io_cond!(addr, pio_write64be_lo_hi(value, port), mmio_write64be(value, addr)); }
#[cfg(target_pointer_width = "64")]
pub unsafe fn __iowrite64be_hi_lo(value: U64, addr: *mut core::ffi::c_void) { kmsan_check_memory(&value as *const _ as *const _, 8); io_cond!(addr, pio_write64be_hi_lo(value, port), mmio_write64be(value, addr)); }

unsafe fn mmio_insb(addr: *const core::ffi::c_void, dst: *mut U8, mut count: i32) { while { count -= 1; count >= 0 } { *dst.add((count as usize)) = __raw_readb(addr); } }
unsafe fn mmio_insw(addr: *const core::ffi::c_void, dst: *mut U16, mut count: i32) { while { count -= 1; count >= 0 } { *dst.add(count as usize) = __raw_readw(addr); } }
unsafe fn mmio_insl(addr: *const core::ffi::c_void, dst: *mut U32, mut count: i32) { while { count -= 1; count >= 0 } { *dst.add(count as usize) = __raw_readl(addr); } }
unsafe fn mmio_outsb(addr: *mut core::ffi::c_void, src: *const U8, mut count: i32) { while { count -= 1; count >= 0 } { __raw_writeb(*src.add(count as usize), addr); } }
unsafe fn mmio_outsw(addr: *mut core::ffi::c_void, src: *const U16, mut count: i32) { while { count -= 1; count >= 0 } { __raw_writew(*src.add(count as usize), addr); } }
unsafe fn mmio_outsl(addr: *mut core::ffi::c_void, src: *const U32, mut count: i32) { while { count -= 1; count >= 0 } { __raw_writel(*src.add(count as usize), addr); } }

pub unsafe fn ioread8_rep(addr: *const core::ffi::c_void, dst: *mut core::ffi::c_void, count: usize) { io_cond!(addr, insb(port, dst, count), mmio_insb(addr, dst as *mut U8, count as i32)); kmsan_unpoison_memory(dst, count); }
pub unsafe fn ioread16_rep(addr: *const core::ffi::c_void, dst: *mut core::ffi::c_void, count: usize) { io_cond!(addr, insw(port, dst, count), mmio_insw(addr, dst as *mut U16, count as i32)); kmsan_unpoison_memory(dst, count * 2); }
pub unsafe fn ioread32_rep(addr: *const core::ffi::c_void, dst: *mut core::ffi::c_void, count: usize) { io_cond!(addr, insl(port, dst, count), mmio_insl(addr, dst as *mut U32, count as i32)); kmsan_unpoison_memory(dst, count * 4); }
pub unsafe fn iowrite8_rep(addr: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize) { kmsan_check_memory(src, count); io_cond!(addr, outsb(port, src, count), mmio_outsb(addr, src as *const U8, count as i32)); }
pub unsafe fn iowrite16_rep(addr: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize) { kmsan_check_memory(src, count * 2); io_cond!(addr, outsw(port, src, count), mmio_outsw(addr, src as *const U16, count as i32)); }
pub unsafe fn iowrite32_rep(addr: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize) { kmsan_check_memory(src, count * 4); io_cond!(addr, outsl(port, src, count), mmio_outsl(addr, src as *const U32, count as i32)); }

#[cfg(has_ioport_map)]
pub unsafe fn ioport_map(port: usize, _nr: U32) -> *mut core::ffi::c_void { if port > PIO_MASK { core::ptr::null_mut() } else { (port + PIO_OFFSET) as *mut core::ffi::c_void } }
#[cfg(has_ioport_map)]
pub unsafe fn ioport_unmap(_addr: *mut core::ffi::c_void) {}

#[cfg(pci)]
pub unsafe fn pci_iounmap(_dev: *mut core::ffi::c_void, addr: *mut core::ffi::c_void) { io_cond!(addr, (), iounmap(addr)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
