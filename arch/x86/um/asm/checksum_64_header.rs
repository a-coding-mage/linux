/*
 * Licensed under the GPL
 */

// C inline assembly: add two unsigned 32-bit values and add the carry.
#[inline]
pub unsafe fn add32_with_carry(a: u32, b: u32) -> u32 {
    let (sum, carry) = a.overflowing_add(b);
    sum.wrapping_add(carry as u32)
}

extern "C" {
    pub fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
