/* SPDX-License-Identifier: GPL-2.0 */
/* This file is meant to be included multiple times by other headers. */

// DEF_PCI_AC_RET / DEF_PCI_AC_NORET are supplied by the including headers.
// Their invocations below declare the corresponding low-level I/O interfaces.

unsafe extern "C" {
    pub fn inb(port: usize) -> u8;
    pub fn inw(port: usize) -> u16;
    pub fn inl(port: usize) -> u32;

    pub fn outb(val: u8, port: usize);
    pub fn outw(val: u16, port: usize);
    pub fn outl(val: u32, port: usize);

    pub fn insb(p: usize, b: *mut core::ffi::c_void, c: usize);
    pub fn insw(p: usize, b: *mut core::ffi::c_void, c: usize);
    pub fn insl(p: usize, b: *mut core::ffi::c_void, c: usize);

    pub fn outsb(p: usize, b: *const core::ffi::c_void, c: usize);
    pub fn outsw(p: usize, b: *const core::ffi::c_void, c: usize);
    pub fn outsl(p: usize, b: *const core::ffi::c_void, c: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
