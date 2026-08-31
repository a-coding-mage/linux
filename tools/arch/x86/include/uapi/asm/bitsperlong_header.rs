/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * C header guard omitted in Rust.
 *
 * Original dependency:
 * #include <asm-generic/bitsperlong.h>
 */

#[cfg(all(target_arch = "x86_64", not(target_pointer_width = "32")))]
pub const __BITS_PER_LONG: u32 = 64;

#[cfg(not(all(target_arch = "x86_64", not(target_pointer_width = "32"))))]
pub const __BITS_PER_LONG: u32 = 32;
