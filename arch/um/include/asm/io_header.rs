/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by linux/types.h.
// Dependency provided by asm-generic/logic_io.h.

/* get emulated iomem (if desired) */

// Equivalent of the C preprocessor guard: define ioremap only when it is not
// supplied by another dependency.
#[inline]
pub unsafe fn ioremap(offset: phys_addr_t, size: usize) -> *mut core::ffi::c_void {
    let _ = offset;
    let _ = size;
    core::ptr::null_mut()
}

// Equivalent of the C preprocessor guard: define iounmap only when it is not
// supplied by another dependency.
#[inline]
pub unsafe fn iounmap(addr: *mut core::ffi::c_void) {
    let _ = addr;
}

// Declarations supplied by asm-generic/io.h are intentionally not copied here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
