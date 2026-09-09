/* SPDX-License-Identifier: GPL-2.0 */

/*
 * These are the "generic" interfaces for doing new-style
 * memory-mapped or PIO accesses. Architectures may do
 * their own arch-optimized versions, these just act as
 * wrappers around the old-style IO register access functions:
 * read[bwl]/write[bwl]/in[bwl]/out[bwl]
 *
 * Don't include this directly, include it from <asm/io.h>.
 */

/*
 * Read/write from/to an (offsettable) iomem cookie. It might be a PIO
 * access or a MMIO access, these functions don't care. The info is
 * encoded in the hardware mapping set up by the mapping functions
 * (or the cookie itself, depending on implementation and hw).
 *
 * The generic routines just encode the PIO/MMIO as part of the
 * cookie, and coldly assume that the MMIO IO mappings are not
 * in the low address range. Architectures for which this is not
 * true can't use this generic implementation.
 */
extern "C" {
    pub fn ioread8(addr: *const core::ffi::c_void) -> u32;
    pub fn ioread16(addr: *const core::ffi::c_void) -> u32;
    pub fn ioread16be(addr: *const core::ffi::c_void) -> u32;
    pub fn ioread32(addr: *const core::ffi::c_void) -> u32;
    pub fn ioread32be(addr: *const core::ffi::c_void) -> u32;

    pub fn __ioread64_lo_hi(addr: *const core::ffi::c_void) -> u64;
    pub fn __ioread64_hi_lo(addr: *const core::ffi::c_void) -> u64;
    pub fn __ioread64be_lo_hi(addr: *const core::ffi::c_void) -> u64;
    pub fn __ioread64be_hi_lo(addr: *const core::ffi::c_void) -> u64;

    pub fn iowrite8(value: u8, addr: *mut core::ffi::c_void);
    pub fn iowrite16(value: u16, addr: *mut core::ffi::c_void);
    pub fn iowrite16be(value: u16, addr: *mut core::ffi::c_void);
    pub fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    pub fn iowrite32be(value: u32, addr: *mut core::ffi::c_void);

    pub fn __iowrite64_lo_hi(value: u64, addr: *mut core::ffi::c_void);
    pub fn __iowrite64_hi_lo(value: u64, addr: *mut core::ffi::c_void);
    pub fn __iowrite64be_lo_hi(value: u64, addr: *mut core::ffi::c_void);
    pub fn __iowrite64be_hi_lo(value: u64, addr: *mut core::ffi::c_void);

    /* "String" versions use native byte ordering and do not update the port address. */
    pub fn ioread8_rep(port: *const core::ffi::c_void, buf: *mut core::ffi::c_void, count: libc::c_ulong);
    pub fn ioread16_rep(port: *const core::ffi::c_void, buf: *mut core::ffi::c_void, count: libc::c_ulong);
    pub fn ioread32_rep(port: *const core::ffi::c_void, buf: *mut core::ffi::c_void, count: libc::c_ulong);

    pub fn iowrite8_rep(port: *mut core::ffi::c_void, buf: *const core::ffi::c_void, count: libc::c_ulong);
    pub fn iowrite16_rep(port: *mut core::ffi::c_void, buf: *const core::ffi::c_void, count: libc::c_ulong);
    pub fn iowrite32_rep(port: *mut core::ffi::c_void, buf: *const core::ffi::c_void, count: libc::c_ulong);

    /* CONFIG_HAS_IOPORT_MAP */
    pub fn ioport_map(port: libc::c_ulong, nr: u32) -> *mut core::ffi::c_void;
    pub fn ioport_unmap(addr: *mut core::ffi::c_void);
}

/* ioremap_wc and ioremap_wt default to ioremap when not otherwise defined. */

/* See the comment in asm-generic/io.h about ioremap_np(). */
#[inline]
pub unsafe fn ioremap_np(_offset: phys_addr_t, _size: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
