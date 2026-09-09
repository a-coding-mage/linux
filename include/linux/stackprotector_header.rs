/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/compiler.h, linux/sched.h, and linux/random.h

/*
 * On 64-bit architectures, protect against non-terminated C string overflows
 * by zeroing out the first byte of the canary; this leaves 56 bits of entropy.
 */
#[cfg(all(target_pointer_width = "64", target_endian = "little"))]
pub const CANARY_MASK: usize = 0xffffffffffffff00usize;

#[cfg(all(target_pointer_width = "64", target_endian = "big"))]
pub const CANARY_MASK: usize = 0x00ffffffffffffffusize;

/* 32 bits. */
#[cfg(target_pointer_width = "32")]
pub const CANARY_MASK: usize = 0xffffffffusize;

extern "C" {
    pub fn get_random_long() -> usize;
}

#[inline]
pub unsafe fn get_random_canary() -> usize {
    get_random_long() & CANARY_MASK
}

// When CONFIG_STACKPROTECTOR or CONFIG_ARM64_PTR_AUTH is enabled, the
// architecture-specific asm/stackprotector.h declarations are supplied by the
// surrounding translation.
#[cfg(not(any(feature = "CONFIG_STACKPROTECTOR", feature = "CONFIG_ARM64_PTR_AUTH")))]
#[inline]
pub fn boot_init_stack_canary() {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
