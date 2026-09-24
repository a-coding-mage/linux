// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel implementation. The original dependencies
// are supplied by the surrounding kernel environment.

use core::ffi::c_ulong;
use core::mem::size_of;

/// Counts the set bits, returning `None` at the original BUG_ON threshold.
///
/// The callback receives only aligned, whole words and an unsigned bit count.
/// Its result is widened without passing through a signed integer.
///
/// # Safety
/// `ptr` must be readable for `bytes` bytes, except that an input reaching the
/// original BUG_ON needs only its leading unaligned bytes to be readable.
/// Zero bytes access no memory, so `ptr` may be null in that case. An aligned
/// pointer at the BUG threshold also needs no readable allocation.
/// `weight` must implement bitmap_weight for the supplied words without mutation.
pub(crate) unsafe fn memweight_with(
    mut ptr: *const u8,
    mut bytes: usize,
    mut weight: impl FnMut(*const c_ulong, u32) -> u32,
) -> Option<usize> {
    let mut result = 0usize;
    const WORD: usize = size_of::<c_ulong>();
    const BITS: usize = WORD * 8;
    while bytes != 0 && (ptr as usize) % WORD != 0 {
        // SAFETY: The caller supplies readable leading bytes.
        result = result.wrapping_add(unsafe { *ptr }.count_ones() as usize);
        bytes -= 1;
        // SAFETY: Advance within the caller's readable region (or one past it).
        ptr = unsafe { ptr.add(1) };
    }
    let longs = bytes / WORD;
    if longs != 0 {
        if longs >= (i32::MAX as usize) / BITS {
            return None;
        }
        result = result.wrapping_add(weight(ptr.cast(), (longs * BITS) as u32) as usize);
        bytes -= longs * WORD;
        // SAFETY: These whole words are within the caller's region.
        ptr = unsafe { ptr.add(longs * WORD) };
    }
    // Keep the trailing bytes separate: a partial native word has different
    // bitmap bit numbering on big-endian machines.
    while bytes != 0 {
        // SAFETY: The remaining bytes are readable by the caller's contract.
        result = result.wrapping_add(unsafe { *ptr }.count_ones() as usize);
        bytes -= 1;
        // SAFETY: Advance within the region or to its one-past pointer.
        ptr = unsafe { ptr.add(1) };
    }
    Some(result)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
