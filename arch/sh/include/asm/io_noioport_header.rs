/* SPDX-License-Identifier: GPL-2.0 */

// The original header guard and include context are intentionally omitted.

pub unsafe fn inb(addr: usize) -> u8 {
    BUG();
    u8::MAX
}

pub unsafe fn inw(addr: usize) -> u16 {
    BUG();
    u16::MAX
}

pub unsafe fn inl(addr: usize) -> u32 {
    BUG();
    u32::MAX
}

pub unsafe fn outb(x: u8, port: usize) {
    BUG();
}

pub unsafe fn outw(x: u16, port: usize) {
    BUG();
}

pub unsafe fn outl(x: u32, port: usize) {
    BUG();
}

pub unsafe fn ioport_map(port: usize, size: u32) -> *mut core::ffi::c_void {
    BUG();
    core::ptr::null_mut()
}

pub unsafe fn ioport_unmap(addr: *mut core::ffi::c_void) {
    BUG();
}

pub unsafe fn insb(port: usize, dst: *mut core::ffi::c_void, count: usize) {
    BUG();
}

pub unsafe fn insw(port: usize, dst: *mut core::ffi::c_void, count: usize) {
    BUG();
}

pub unsafe fn insl(port: usize, dst: *mut core::ffi::c_void, count: usize) {
    BUG();
}

pub unsafe fn outsb(port: usize, src: *const core::ffi::c_void, count: usize) {
    BUG();
}

pub unsafe fn outsw(port: usize, src: *const core::ffi::c_void, count: usize) {
    BUG();
}

pub unsafe fn outsl(port: usize, src: *const core::ffi::c_void, count: usize) {
    BUG();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
