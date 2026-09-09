/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/kernel.h>, <linux/ioport.h>, and <asm-generic/io.h>.

pub const IO_SPACE_LIMIT: u32 = 0xffff_ffff;

// C macros:
// #define memset_io(d,c,sz)     _memset_io(d,c,sz)
// #define memcpy_fromio(d,s,sz) _memcpy_fromio(d,s,sz)
// #define memcpy_toio(d,s,sz)   _memcpy_toio(d,s,sz)

/*
 * Bus number may be embedded in the higher bits of the physical address.
 * This is why we have no bus number argument to ioremap().
 */
unsafe extern "C" {
    pub fn ioremap(offset: phys_addr_t, size: usize) -> *mut core::ffi::c_void;
    pub fn iounmap(addr: *mut core::ffi::c_void);
}

pub unsafe fn _memset_io(dst: *mut core::ffi::c_void, c: i32, mut n: usize) {
    let mut d = dst as *mut u8;
    while n != 0 {
        writeb(c as u8, d as *mut core::ffi::c_void);
        d = d.add(1);
        n -= 1;
    }
}

pub unsafe fn _memcpy_fromio(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    mut n: usize,
) {
    let mut d = dst as *mut i8;
    let mut s = src as *const u8;
    while n != 0 {
        let tmp = readb(s as *const core::ffi::c_void);
        *d = tmp as i8;
        d = d.add(1);
        s = s.add(1);
        n -= 1;
    }
}

pub unsafe fn _memcpy_toio(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    mut n: usize,
) {
    let mut s = src as *const i8;
    let mut d = dst as *mut u8;
    while n != 0 {
        let tmp = *s as u8;
        writeb(tmp, d as *mut core::ffi::c_void);
        s = s.add(1);
        d = d.add(1);
        n -= 1;
    }
}

/*
 * SBus accessors.
 *
 * SBus has only one, memory mapped, I/O space.
 * We do not need to flip bytes for SBus of course.
 */
pub unsafe fn sbus_readb(addr: *const core::ffi::c_void) -> u8 {
    core::ptr::read_volatile(addr as *const u8)
}

pub unsafe fn sbus_readw(addr: *const core::ffi::c_void) -> u16 {
    core::ptr::read_volatile(addr as *const u16)
}

pub unsafe fn sbus_readl(addr: *const core::ffi::c_void) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}

pub unsafe fn sbus_writeb(b: u8, addr: *mut core::ffi::c_void) {
    core::ptr::write_volatile(addr as *mut u8, b);
}

pub unsafe fn sbus_writew(w: u16, addr: *mut core::ffi::c_void) {
    core::ptr::write_volatile(addr as *mut u16, w);
}

pub unsafe fn sbus_writel(l: u32, addr: *mut core::ffi::c_void) {
    core::ptr::write_volatile(addr as *mut u32, l);
}

pub unsafe fn sbus_memset_io(
    mut dst: *mut core::ffi::c_void,
    c: i32,
    mut n: usize,
) {
    while n != 0 {
        sbus_writeb(c as u8, dst);
        dst = (dst as *mut u8).add(1) as *mut core::ffi::c_void;
        n -= 1;
    }
}

pub unsafe fn sbus_memcpy_fromio(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    mut n: usize,
) {
    let mut d = dst as *mut i8;
    let mut s = src as *const u8;
    while n != 0 {
        let tmp = sbus_readb(s as *const core::ffi::c_void);
        *d = tmp as i8;
        d = d.add(1);
        s = s.add(1);
        n -= 1;
    }
}

pub unsafe fn sbus_memcpy_toio(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    mut n: usize,
) {
    let mut s = src as *const i8;
    let mut d = dst;
    while n != 0 {
        let tmp = *s as u8;
        sbus_writeb(tmp, d);
        s = s.add(1);
        d = (d as *mut u8).add(1) as *mut core::ffi::c_void;
        n -= 1;
    }
}

/* Create a virtual mapping cookie for an IO port range */
unsafe extern "C" {
    pub fn ioport_map(port: core::ffi::c_ulong, nr: u32) -> *mut core::ffi::c_void;
    pub fn ioport_unmap(addr: *mut core::ffi::c_void);
}

/* Create a virtual mapping cookie for a PCI BAR (memory or IO) */
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn pci_iounmap(dev: *mut pci_dev, addr: *mut core::ffi::c_void);
}

pub const fn sbus_can_dma_64bit() -> i32 {
    0 // actually, sparc_cpu_model==sun4d
}

pub const fn sbus_can_burst64() -> i32 {
    0 // actually, sparc_cpu_model==sun4d
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn sbus_set_sbus64(dev: *mut device, value: i32);
}

pub const __ARCH_HAS_NO_PAGE_ZERO_MAPPED: i32 = 1;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
