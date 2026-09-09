/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *\tinclude/asm-mips/mach-generic/ioremap.h
 */

// Dependency supplied by the surrounding translation unit: `phys_addr_t`.

#[inline]
unsafe fn plat_ioremap(
    offset: phys_addr_t,
    size: usize,
    flags: usize,
) -> *mut core::ffi::c_void {
    let _ = (offset, size, flags);
    core::ptr::null_mut()
}

#[inline]
unsafe fn plat_iounmap(addr: *const core::ffi::c_void) -> i32 {
    let _ = addr;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
