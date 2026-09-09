/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license found in the
 * LICENSE file in the root directory of this source tree and the GPLv2 found
 * in the COPYING file in the root directory of this source tree.
 */

/* Dependencies: linux unaligned/compiler/swab/types and debug.h. */

pub type BYTE = u8;
pub type U8 = u8;
pub type S8 = i8;
pub type U16 = u16;
pub type S16 = i16;
pub type U32 = u32;
pub type S32 = i32;
pub type U64 = u64;
pub type S64 = i64;

#[inline]
pub const unsafe fn MEM_32bits() -> u32 { (core::mem::size_of::<usize>() == 4) as u32 }

#[inline]
pub const unsafe fn MEM_64bits() -> u32 { (core::mem::size_of::<usize>() == 8) as u32 }

#[inline]
pub const unsafe fn MEM_isLittleEndian() -> u32 { cfg!(target_endian = "little") as u32 }

#[inline]
pub unsafe fn MEM_read16(mem_ptr: *const core::ffi::c_void) -> U16 {
    core::ptr::read_unaligned(mem_ptr as *const U16)
}

#[inline]
pub unsafe fn MEM_read32(mem_ptr: *const core::ffi::c_void) -> U32 {
    core::ptr::read_unaligned(mem_ptr as *const U32)
}

#[inline]
pub unsafe fn MEM_read64(mem_ptr: *const core::ffi::c_void) -> U64 {
    core::ptr::read_unaligned(mem_ptr as *const U64)
}

#[inline]
pub unsafe fn MEM_readST(mem_ptr: *const core::ffi::c_void) -> usize {
    core::ptr::read_unaligned(mem_ptr as *const usize)
}

#[inline]
pub unsafe fn MEM_write16(mem_ptr: *mut core::ffi::c_void, value: U16) { core::ptr::write_unaligned(mem_ptr as *mut U16, value); }
#[inline]
pub unsafe fn MEM_write32(mem_ptr: *mut core::ffi::c_void, value: U32) { core::ptr::write_unaligned(mem_ptr as *mut U32, value); }
#[inline]
pub unsafe fn MEM_write64(mem_ptr: *mut core::ffi::c_void, value: U64) { core::ptr::write_unaligned(mem_ptr as *mut U64, value); }

#[inline]
pub unsafe fn MEM_readLE16(mem_ptr: *const core::ffi::c_void) -> U16 { u16::from_le(MEM_read16(mem_ptr)) }
#[inline]
pub unsafe fn MEM_writeLE16(mem_ptr: *mut core::ffi::c_void, val: U16) { MEM_write16(mem_ptr, u16::to_le(val)); }

#[inline]
pub unsafe fn MEM_readLE24(mem_ptr: *const core::ffi::c_void) -> U32 {
    MEM_readLE16(mem_ptr) as U32 | ((*((mem_ptr as *const BYTE).add(2))) as U32) << 16
}

#[inline]
pub unsafe fn MEM_writeLE24(mem_ptr: *mut core::ffi::c_void, val: U32) {
    MEM_writeLE16(mem_ptr, val as U16);
    *((mem_ptr as *mut BYTE).add(2)) = (val >> 16) as BYTE;
}

#[inline]
pub unsafe fn MEM_readLE32(mem_ptr: *const core::ffi::c_void) -> U32 { u32::from_le(MEM_read32(mem_ptr)) }
#[inline]
pub unsafe fn MEM_writeLE32(mem_ptr: *mut core::ffi::c_void, val: U32) { MEM_write32(mem_ptr, u32::to_le(val)); }
#[inline]
pub unsafe fn MEM_readLE64(mem_ptr: *const core::ffi::c_void) -> U64 { u64::from_le(MEM_read64(mem_ptr)) }
#[inline]
pub unsafe fn MEM_writeLE64(mem_ptr: *mut core::ffi::c_void, val: U64) { MEM_write64(mem_ptr, u64::to_le(val)); }

#[inline]
pub unsafe fn MEM_readLEST(mem_ptr: *const core::ffi::c_void) -> usize {
    if MEM_32bits() != 0 { MEM_readLE32(mem_ptr) as usize } else { MEM_readLE64(mem_ptr) as usize }
}
#[inline]
pub unsafe fn MEM_writeLEST(mem_ptr: *mut core::ffi::c_void, val: usize) {
    if MEM_32bits() != 0 { MEM_writeLE32(mem_ptr, val as U32) } else { MEM_writeLE64(mem_ptr, val as U64) }
}

#[inline]
pub unsafe fn MEM_readBE32(mem_ptr: *const core::ffi::c_void) -> U32 { u32::from_be(MEM_read32(mem_ptr)) }
#[inline]
pub unsafe fn MEM_writeBE32(mem_ptr: *mut core::ffi::c_void, val: U32) { MEM_write32(mem_ptr, u32::to_be(val)); }
#[inline]
pub unsafe fn MEM_readBE64(mem_ptr: *const core::ffi::c_void) -> U64 { u64::from_be(MEM_read64(mem_ptr)) }
#[inline]
pub unsafe fn MEM_writeBE64(mem_ptr: *mut core::ffi::c_void, val: U64) { MEM_write64(mem_ptr, u64::to_be(val)); }

#[inline]
pub unsafe fn MEM_readBEST(mem_ptr: *const core::ffi::c_void) -> usize {
    if MEM_32bits() != 0 { MEM_readBE32(mem_ptr) as usize } else { MEM_readBE64(mem_ptr) as usize }
}
#[inline]
pub unsafe fn MEM_writeBEST(mem_ptr: *mut core::ffi::c_void, val: usize) {
    if MEM_32bits() != 0 { MEM_writeBE32(mem_ptr, val as U32) } else { MEM_writeBE64(mem_ptr, val as U64) }
}

#[inline]
pub const unsafe fn MEM_swap32(input: U32) -> U32 { input.swap_bytes() }
#[inline]
pub const unsafe fn MEM_swap64(input: U64) -> U64 { input.swap_bytes() }
#[inline]
pub const unsafe fn MEM_swapST(input: usize) -> usize { input.swap_bytes() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
