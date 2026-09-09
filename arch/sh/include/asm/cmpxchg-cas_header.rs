/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted; Rust items are subject to normal module/item
// scoping instead.

/// Compare-and-exchange a 32-bit value using the SuperH `cas.l` instruction.
#[inline]
pub unsafe fn __cmpxchg_u32(m: *mut core::ffi::c_void, old: usize, mut new: usize) -> usize {
    // `u32` and the external declarations supplying it are dependencies of
    // the translated header.  The original inline assembly is retained here
    // as the direct low-level operation.
    core::arch::asm!(
        "cas.l {old}, {new}, @r0",
        old = in(reg) old,
        new = inout(reg) new,
        in("r0") m,
        lateout("t") _,
        options(nostack),
    );
    new
}

#[inline]
pub unsafe fn xchg_u32(m: *mut core::ffi::c_void, val: usize) -> usize {
    let mut old: usize;
    loop {
        old = core::ptr::read_volatile(m as *const usize);
        if __cmpxchg_u32(m, old, val) == old {
            break;
        }
    }
    old
}

// Dependency corresponding to: #include <asm/cmpxchg-xchg.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
