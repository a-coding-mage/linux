/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel translation. */
unsafe extern "C" {
    fn __pa32(addr: *const c_void) -> u32;
    fn __pa(addr: *const c_void) -> u64;
    fn __va(addr: core::ffi::c_ulong) -> *mut c_void;
}

/*
 * typedef dma32_t
 * Contains a 31 bit absolute address to a DMA capable piece of storage.
 *
 * For CIO, DMA addresses are always absolute addresses. These addresses tend
 * to be used in architectured memory blocks (like ORB, IDAW, MIDAW). Under
 * certain circumstances 31 bit wide addresses must be used because the
 * address must fit in 31 bits.
 *
 * This type is to be used when such fields can be modelled as 32 bit wide.
 */
pub type dma32_t = u32;

/*
 * typedef dma64_t
 * Contains a 64 bit absolute address to a DMA capable piece of storage.
 *
 * For CIO, DMA addresses are always absolute addresses. These addresses tend
 * to be used in architectured memory blocks (like ORB, IDAW, MIDAW).
 *
 * This type is to be used to model such 64 bit wide fields.
 */
pub type dma64_t = u64;

/*
 * Although DMA addresses should be obtained using the DMA API, in cases when
 * it is known that the first argument holds a virtual address that points to
 * DMA-able 31 bit addressable storage, then this function can be safely used.
 */
#[inline]
pub unsafe fn virt_to_dma32(ptr: *mut c_void) -> dma32_t {
    unsafe { __pa32(ptr.cast()) }
}

#[inline]
pub unsafe fn dma32_to_virt(addr: dma32_t) -> *mut c_void {
    unsafe { __va(addr as core::ffi::c_ulong) }
}

#[inline]
pub fn u32_to_dma32(addr: u32) -> dma32_t {
    addr
}

#[inline]
pub fn dma32_to_u32(addr: dma32_t) -> u32 {
    addr
}

#[inline]
pub fn dma32_add(a: dma32_t, b: u32) -> dma32_t {
    a.wrapping_add(b)
}

#[inline]
pub fn dma32_and(a: dma32_t, b: u32) -> dma32_t {
    a & b
}

/*
 * Although DMA addresses should be obtained using the DMA API, in cases when
 * it is known that the first argument holds a virtual address that points to
 * DMA-able storage, then this function can be safely used.
 */
#[inline]
pub unsafe fn virt_to_dma64(ptr: *mut c_void) -> dma64_t {
    unsafe { __pa(ptr.cast()) }
}

#[inline]
pub unsafe fn dma64_to_virt(addr: dma64_t) -> *mut c_void {
    unsafe { __va(addr as core::ffi::c_ulong) }
}

#[inline]
pub fn u64_to_dma64(addr: u64) -> dma64_t {
    addr
}

#[inline]
pub fn dma64_to_u64(addr: dma64_t) -> u64 {
    addr
}

#[inline]
pub fn dma64_add(a: dma64_t, b: u64) -> dma64_t {
    a.wrapping_add(b)
}

#[inline]
pub fn dma64_and(a: dma64_t, b: u64) -> dma64_t {
    a & b
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
