/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated types header: `u16`.

/*
 * Low-level I/O routines.
 *
 * Copied from <file:arch/powerpc/include/asm/io.h> (which has no copyright)
 */

#[inline]
pub unsafe fn in_8(addr: *const core::ffi::c_uchar) -> i32 {
    let mut ret: i32;
    core::arch::asm!(
        "lbz%U1%X1 {0},{1}; twi 0,{0},0; isync",
        out(reg) ret,
        in(reg) addr,
        options(nostack)
    );
    ret
}

#[inline]
pub unsafe fn out_8(addr: *mut core::ffi::c_uchar, val: i32) {
    core::arch::asm!(
        "stb%U0%X0 {1},{0}; sync",
        inout(reg) addr => _,
        in(reg) val,
        options(nostack)
    );
}

#[inline]
pub unsafe fn in_le16(addr: *const u16) -> u32 {
    let mut ret: u32;
    core::arch::asm!(
        "lhbrx {0},0,{1}; twi 0,{0},0; isync",
        out(reg) ret,
        in(reg) addr,
        options(nostack)
    );
    ret
}

#[inline]
pub unsafe fn in_be16(addr: *const u16) -> u32 {
    let mut ret: u32;
    core::arch::asm!(
        "lhz%U1%X1 {0},{1}; twi 0,{0},0; isync",
        out(reg) ret,
        in(reg) addr,
        options(nostack)
    );
    ret
}

#[inline]
pub unsafe fn out_le16(addr: *mut u16, val: i32) {
    core::arch::asm!(
        "sthbrx {1},0,{2}; sync",
        inout(reg) addr => _,
        in(reg) val,
        in(reg) addr,
        options(nostack)
    );
}

#[inline]
pub unsafe fn out_be16(addr: *mut u16, val: i32) {
    core::arch::asm!(
        "sth%U0%X0 {1},{0}; sync",
        inout(reg) addr => _,
        in(reg) val,
        options(nostack)
    );
}

#[inline]
pub unsafe fn in_le32(addr: *const u32) -> u32 {
    let mut ret: u32;
    core::arch::asm!(
        "lwbrx {0},0,{1}; twi 0,{0},0; isync",
        out(reg) ret,
        in(reg) addr,
        options(nostack)
    );
    ret
}

#[inline]
pub unsafe fn in_be32(addr: *const u32) -> u32 {
    let mut ret: u32;
    core::arch::asm!(
        "lwz%U1%X1 {0},{1}; twi 0,{0},0; isync",
        out(reg) ret,
        in(reg) addr,
        options(nostack)
    );
    ret
}

#[inline]
pub unsafe fn out_le32(addr: *mut u32, val: i32) {
    core::arch::asm!(
        "stwbrx {1},0,{2}; sync",
        inout(reg) addr => _,
        in(reg) val,
        in(reg) addr,
        options(nostack)
    );
}

#[inline]
pub unsafe fn out_be32(addr: *mut u32, val: i32) {
    core::arch::asm!(
        "stw%U0%X0 {1},{0}; sync",
        inout(reg) addr => _,
        in(reg) val,
        options(nostack)
    );
}

#[inline]
pub unsafe fn sync() {
    core::arch::asm!("sync", options(nostack));
}

#[inline]
pub unsafe fn eieio() {
    core::arch::asm!("eieio", options(nostack));
}

#[inline]
pub unsafe fn barrier() {
    core::arch::asm!("", options(nostack));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
