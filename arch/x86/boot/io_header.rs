/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding assembly/shared I/O definitions.

#[repr(C)]
pub struct port_io_ops {
    pub f_inb: unsafe extern "C" fn(port: u16) -> u8,
    pub f_outb: unsafe extern "C" fn(v: u8, port: u16),
    pub f_outw: unsafe extern "C" fn(v: u16, port: u16),
}

unsafe extern "C" {
    pub static mut pio_ops: port_io_ops;

    fn __inb(port: u16) -> u8;
    fn __outb(v: u8, port: u16);
    fn __outw(v: u16, port: u16);
}

/*
 * Use the normal I/O instructions by default.
 * TDX guests override these to use hypercalls.
 */
#[inline]
pub unsafe fn init_default_io_ops() {
    pio_ops.f_inb = __inb;
    pio_ops.f_outb = __outb;
    pio_ops.f_outw = __outw;
}

/*
 * Redirect port I/O operations via pio_ops callbacks.
 * TDX guests override these callbacks with TDX-specific helpers.
 *
 * These functions are the Rust equivalents of the C macros:
 *   #define inb  pio_ops.f_inb
 *   #define outb pio_ops.f_outb
 *   #define outw pio_ops.f_outw
 */
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    (pio_ops.f_inb)(port)
}

#[inline]
pub unsafe fn outb(v: u8, port: u16) {
    (pio_ops.f_outb)(v, port)
}

#[inline]
pub unsafe fn outw(v: u16, port: u16) {
    (pio_ops.f_outw)(v, port)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
