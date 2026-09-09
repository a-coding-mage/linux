/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_PCI conditional from the original header. */

/* I/O size constraints */
pub const ZPCI_MAX_READ_SIZE: usize = 8;
pub const ZPCI_MAX_WRITE_SIZE: usize = 128;
pub const ZPCI_BOUNDARY_SIZE: u64 = 1 << 12;
pub const ZPCI_BOUNDARY_MASK: u64 = ZPCI_BOUNDARY_SIZE - 1;

/* I/O Map */
pub const ZPCI_IOMAP_SHIFT: u32 = 48;
pub const ZPCI_IOMAP_ADDR_SHIFT: u32 = 62;
pub const ZPCI_IOMAP_ADDR_BASE: u64 = 1u64 << ZPCI_IOMAP_ADDR_SHIFT;
pub const ZPCI_IOMAP_ADDR_MAX: u64 = (1u64 << (ZPCI_IOMAP_ADDR_SHIFT + 1)) - 1;
pub const ZPCI_IOMAP_ADDR_OFF_MASK: u64 = (1u64 << ZPCI_IOMAP_SHIFT) - 1;
pub const ZPCI_IOMAP_MAX_ENTRIES: u64 =
    1u64 << (ZPCI_IOMAP_ADDR_SHIFT - ZPCI_IOMAP_SHIFT);
pub const ZPCI_IOMAP_ADDR_IDX_MASK: u64 =
    (ZPCI_IOMAP_ADDR_BASE - 1) & !ZPCI_IOMAP_ADDR_OFF_MASK;

#[repr(C)]
pub struct ZpciIomapEntry {
    pub fh: u32,
    pub bar: u8,
    pub count: u16,
}

extern "C" {
    pub static mut zpci_iomap_start: *mut ZpciIomapEntry;
}

#[inline]
pub const fn ZPCI_ADDR(idx: u64) -> u64 {
    ZPCI_IOMAP_ADDR_BASE | (idx << ZPCI_IOMAP_SHIFT)
}

#[inline]
pub const fn ZPCI_IDX(addr: u64) -> u64 {
    (addr & ZPCI_IOMAP_ADDR_IDX_MASK) >> ZPCI_IOMAP_SHIFT
}

#[inline]
pub const fn ZPCI_OFFSET(addr: u64) -> u64 {
    addr & ZPCI_IOMAP_ADDR_OFF_MASK
}

#[inline]
pub const fn ZPCI_CREATE_REQ(handle: u64, space: u64, len: u64) -> u64 {
    (handle << 32) | (space << 16) | len
}

extern "C" {
    pub fn zpci_load(data: *mut u64, addr: *const core::ffi::c_void, length: usize) -> i32;
    pub fn zpci_store(addr: *const core::ffi::c_void, data: u64, length: usize) -> i32;
    pub fn zpci_write_block(
        dst: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        len: usize,
    ) -> i32;
    pub fn kmalloc(size: usize, flags: u32) -> *mut u8;
    pub fn kfree(ptr: *mut u8);
}

#[inline]
pub unsafe fn zpci_read_u64(addr: *const core::ffi::c_void) -> u64 {
    let mut data: u64 = 0;
    if zpci_load(&mut data, addr, 8) != 0 { data = u64::MAX; }
    data
}

#[inline]
pub unsafe fn zpci_read_u32(addr: *const core::ffi::c_void) -> u32 {
    zpci_read_u64(addr) as u32
}
#[inline]
pub unsafe fn zpci_read_u16(addr: *const core::ffi::c_void) -> u16 {
    let mut data = 0u64;
    if zpci_load(&mut data, addr, 2) != 0 { data = u64::MAX; }
    data as u16
}
#[inline]
pub unsafe fn zpci_read_u8(addr: *const core::ffi::c_void) -> u8 {
    let mut data = 0u64;
    if zpci_load(&mut data, addr, 1) != 0 { data = u64::MAX; }
    data as u8
}

#[inline]
pub unsafe fn zpci_write_u64(val: u64, addr: *const core::ffi::c_void) {
    zpci_store(addr, val, 8);
}
#[inline]
pub unsafe fn zpci_write_u32(val: u32, addr: *const core::ffi::c_void) {
    zpci_store(addr, val as u64, 4);
}
#[inline]
pub unsafe fn zpci_write_u16(val: u16, addr: *const core::ffi::c_void) {
    zpci_store(addr, val as u64, 2);
}
#[inline]
pub unsafe fn zpci_write_u8(val: u8, addr: *const core::ffi::c_void) {
    zpci_store(addr, val as u64, 1);
}

#[inline]
pub unsafe fn zpci_write_single(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> i32 {
    let val = match len {
        1 => *(src as *const u8) as u64,
        2 => *(src as *const u16) as u64,
        4 => *(src as *const u32) as u64,
        8 => *(src as *const u64),
        _ => 0,
    };
    zpci_store(dst, val, len)
}

#[inline]
pub unsafe fn zpci_read_single(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, len: usize) -> i32 {
    let mut data = 0u64;
    let cc = zpci_load(&mut data, src, len);
    if cc != 0 { return cc; }
    match len {
        1 => *(dst as *mut u8) = data as u8,
        2 => *(dst as *mut u16) = data as u16,
        4 => *(dst as *mut u32) = data as u32,
        8 => *(dst as *mut u64) = data,
        _ => {}
    }
    cc
}

#[inline]
pub fn zpci_get_max_io_size(src: u64, dst: u64, len: usize, max: usize) -> usize {
    let offset = (dst & ZPCI_BOUNDARY_MASK) as usize;
    let size = core::cmp::min(len, core::cmp::min((ZPCI_BOUNDARY_SIZE as usize) - offset, max));
    if src % 8 == 0 && dst % 8 == 0 && size % 8 == 0 { return size; }
    if size >= 8 { return 8; }
    if size == 0 { 0 } else { 1usize << (usize::BITS - 1 - size.leading_zeros()) }
}

#[inline]
pub unsafe fn zpci_memcpy_fromio(dst: *mut u8, src: *const u8, mut n: usize) -> i32 {
    let mut rc = 0;
    let mut dst = dst;
    let mut src = src;
    while n > 0 {
        let size = zpci_get_max_io_size(src as u64, dst as u64, n, ZPCI_MAX_READ_SIZE);
        rc = zpci_read_single(dst as *mut _, src as *const _, size);
        if rc != 0 { break; }
        src = src.add(size); dst = dst.add(size); n -= size;
    }
    rc
}

#[inline]
pub unsafe fn zpci_memcpy_toio(dst: *mut u8, src: *const u8, mut n: usize) -> i32 {
    if src.is_null() { return -22; }
    let mut dst = dst;
    let mut src = src;
    let mut rc = 0;
    while n > 0 {
        let size = zpci_get_max_io_size(dst as u64, src as u64, n, ZPCI_MAX_WRITE_SIZE);
        rc = if size > 8 { zpci_write_block(dst as *mut _, src as *const _, size) } else { zpci_write_single(dst as *mut _, src as *const _, size) };
        if rc != 0 { break; }
        src = src.add(size); dst = dst.add(size); n -= size;
    }
    rc
}

/* kmalloc/memset/kfree and GFP_KERNEL are supplied by the kernel dependencies. */

#[inline]
pub unsafe fn zpci_memset_io(dst: *mut u8, val: i32, count: usize) -> i32 {
    let src = kmalloc(count, 0 /* GFP_KERNEL */);
    if src.is_null() { return -12; }
    core::ptr::write_bytes(src, val as u8, count);
    let rc = zpci_memcpy_toio(dst, src, count);
    kfree(src);
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
