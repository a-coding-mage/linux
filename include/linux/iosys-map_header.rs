/* SPDX-License-Identifier: GPL-2.0-only */
/* Pointer abstraction for IO/system memory. */

use core::ffi::c_void;
use core::ptr;

/* C dependencies: linux/compiler_types.h, linux/io.h, linux/string.h. */

#[repr(C)]
pub union IosysMapAddress {
    pub vaddr_iomem: *mut c_void,
    pub vaddr: *mut c_void,
}

#[repr(C)]
pub struct iosys_map {
    pub address: IosysMapAddress,
    pub is_iomem: bool,
}

#[inline]
pub const fn iosys_map_init_vaddr(vaddr: *mut c_void) -> iosys_map {
    iosys_map { address: IosysMapAddress { vaddr }, is_iomem: false }
}

#[inline]
pub const fn iosys_map_init_vaddr_iomem(vaddr_iomem: *mut c_void) -> iosys_map {
    iosys_map { address: IosysMapAddress { vaddr_iomem }, is_iomem: true }
}

#[inline]
pub unsafe fn iosys_map_set_vaddr(map: *mut iosys_map, vaddr: *mut c_void) {
    (*map).address.vaddr = vaddr;
    (*map).is_iomem = false;
}

#[inline]
pub unsafe fn iosys_map_set_vaddr_iomem(map: *mut iosys_map, vaddr_iomem: *mut c_void) {
    (*map).address.vaddr_iomem = vaddr_iomem;
    (*map).is_iomem = true;
}

#[inline]
pub unsafe fn iosys_map_is_equal(lhs: *const iosys_map, rhs: *const iosys_map) -> bool {
    if (*lhs).is_iomem != (*rhs).is_iomem {
        false
    } else if (*lhs).is_iomem {
        (*lhs).address.vaddr_iomem == (*rhs).address.vaddr_iomem
    } else {
        (*lhs).address.vaddr == (*rhs).address.vaddr
    }
}

#[inline]
pub unsafe fn iosys_map_is_null(map: *const iosys_map) -> bool {
    if (*map).is_iomem { (*map).address.vaddr_iomem.is_null() } else { (*map).address.vaddr.is_null() }
}

#[inline]
pub unsafe fn iosys_map_is_set(map: *const iosys_map) -> bool { !iosys_map_is_null(map) }

#[inline]
pub unsafe fn iosys_map_clear(map: *mut iosys_map) {
    ptr::write_bytes(map, 0, 1);
}

extern "C" {
    pub fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    pub fn memcpy_toio(dst: *mut c_void, src: *const c_void, len: usize);
    pub fn memcpy_fromio(dst: *mut c_void, src: *const c_void, len: usize);
    pub fn memset(dst: *mut c_void, value: i32, len: usize) -> *mut c_void;
    pub fn memset_io(dst: *mut c_void, value: i32, len: usize);
    pub fn readb(addr: *const c_void) -> u8;
    pub fn readw(addr: *const c_void) -> u16;
    pub fn readl(addr: *const c_void) -> u32;
    pub fn readq(addr: *const c_void) -> u64;
    pub fn writeb(value: u8, addr: *mut c_void);
    pub fn writew(value: u16, addr: *mut c_void);
    pub fn writel(value: u32, addr: *mut c_void);
    pub fn writeq(value: u64, addr: *mut c_void);
}

#[inline]
pub unsafe fn iosys_map_memcpy_to(dst: *mut iosys_map, dst_offset: usize, src: *const c_void, len: usize) {
    if (*dst).is_iomem {
        memcpy_toio((*dst).address.vaddr_iomem.add(dst_offset), src, len);
    } else {
        memcpy((*dst).address.vaddr.add(dst_offset), src, len);
    }
}

#[inline]
pub unsafe fn iosys_map_memcpy_from(dst: *mut c_void, src: *const iosys_map, src_offset: usize, len: usize) {
    if (*src).is_iomem {
        memcpy_fromio(dst, (*src).address.vaddr_iomem.add(src_offset), len);
    } else {
        memcpy(dst, (*src).address.vaddr.add(src_offset), len);
    }
}

#[inline]
pub unsafe fn iosys_map_incr(map: *mut iosys_map, incr: usize) {
    if (*map).is_iomem { (*map).address.vaddr_iomem = (*map).address.vaddr_iomem.add(incr); }
    else { (*map).address.vaddr = (*map).address.vaddr.add(incr); }
}

#[inline]
pub unsafe fn iosys_map_memset(dst: *mut iosys_map, offset: usize, value: i32, len: usize) {
    if (*dst).is_iomem { memset_io((*dst).address.vaddr_iomem.add(offset), value, len); }
    else { memset((*dst).address.vaddr.add(offset), value, len); }
}

/* Rust equivalents of the C iosys_map_rd/iosys_map_wr operations. */
#[inline]
pub unsafe fn iosys_map_rd_u8(map: *const iosys_map, offset: usize) -> u8 {
    if (*map).is_iomem { readb((*map).address.vaddr_iomem.add(offset)) }
    else { ptr::read_volatile((*map).address.vaddr.add(offset) as *const u8) }
}
#[inline]
pub unsafe fn iosys_map_rd_u16(map: *const iosys_map, offset: usize) -> u16 {
    if (*map).is_iomem { readw((*map).address.vaddr_iomem.add(offset)) }
    else { ptr::read_volatile((*map).address.vaddr.add(offset) as *const u16) }
}
#[inline]
pub unsafe fn iosys_map_rd_u32(map: *const iosys_map, offset: usize) -> u32 {
    if (*map).is_iomem { readl((*map).address.vaddr_iomem.add(offset)) }
    else { ptr::read_volatile((*map).address.vaddr.add(offset) as *const u32) }
}
#[inline]
pub unsafe fn iosys_map_rd_u64(map: *const iosys_map, offset: usize) -> u64 {
    if (*map).is_iomem { readq((*map).address.vaddr_iomem.add(offset)) }
    else { ptr::read_volatile((*map).address.vaddr.add(offset) as *const u64) }
}

#[macro_export]
macro_rules! IOSYS_MAP_INIT_OFFSET {
    ($map:expr, $offset:expr) => {{ let mut copy_ = unsafe { *$map }; unsafe { $crate::iosys_map_incr(&mut copy_, $offset); } copy_ }};
}

#[macro_export]
macro_rules! IOSYS_MAP_INIT_VADDR { ($vaddr:expr) => { $crate::iosys_map_init_vaddr($vaddr as *mut core::ffi::c_void) }; }
#[macro_export]
macro_rules! IOSYS_MAP_INIT_VADDR_IOMEM { ($vaddr:expr) => { $crate::iosys_map_init_vaddr_iomem($vaddr as *mut core::ffi::c_void) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
