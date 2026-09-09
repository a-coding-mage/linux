/* SPDX-License-Identifier: GPL-2.0 */

// KERNBASE: First address the kernel will eventually be.
pub const KERNBASE: u32 = 0xE000000;

// LOAD_ADDR: prom jumps to us here unless this is elf /boot.
pub const LOAD_ADDR: u32 = 0x4000;

pub const FC_CONTROL: u32 = 3;
pub const FC_SUPERD: u32 = 5;
pub const FC_CPU: u32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
