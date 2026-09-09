/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/types.h>; the required integer types are
// represented by Rust's fixed-width unsigned integer types below.

#[inline(always)]
unsafe fn __outb(value: u8, port: u16) {
    core::arch::asm!("out dx, al", in("al") value, in("dx") port);
}

#[inline(always)]
unsafe fn __inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", out("al") value, in("dx") port);
    value
}

#[inline(always)]
unsafe fn __outw(value: u16, port: u16) {
    core::arch::asm!("out dx, ax", in("ax") value, in("dx") port);
}

#[inline(always)]
unsafe fn __inw(port: u16) -> u16 {
    let value: u16;
    core::arch::asm!("in ax, dx", out("ax") value, in("dx") port);
    value
}

#[inline(always)]
unsafe fn __outl(value: u32, port: u16) {
    core::arch::asm!("out dx, eax", in("eax") value, in("dx") port);
}

#[inline(always)]
unsafe fn __inl(port: u16) -> u32 {
    let value: u32;
    core::arch::asm!("in eax, dx", out("eax") value, in("dx") port);
    value
}

pub use __inb as inb;
pub use __inw as inw;
pub use __inl as inl;
pub use __outb as outb;
pub use __outw as outw;
pub use __outl as outl;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
