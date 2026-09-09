/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the SuperH load-linked/store-conditional implementation.

#[inline]
pub unsafe fn xchg_u32(m: *mut u32, val: usize) -> usize {
    let mut retval: usize;
    let mut tmp: usize;

    core::arch::asm!(
        "1:",
        "movli.l @{{m}}, {{tmp}} ! xchg_u32",
        "mov {{tmp}}, {{retval}}",
        "mov {{val}}, {{tmp}}",
        "movco.l {{tmp}}, @{{m}}",
        "bf 1b",
        "synco",
        m = in(reg) m,
        val = in(reg) val,
        tmp = out(reg) tmp,
        retval = out(reg) retval,
        options(nostack)
    );

    retval
}

#[inline]
pub unsafe fn __cmpxchg_u32(m: *mut u32, old: usize, new: usize) -> usize {
    let mut retval: usize;
    let mut tmp: usize;

    core::arch::asm!(
        "1:",
        "movli.l @{{m}}, {{tmp}} ! __cmpxchg_u32",
        "mov {{tmp}}, {{retval}}",
        "cmp/eq {{retval}}, {{old}}",
        "bf 2f",
        "mov {{new}}, {{tmp}}",
        "2:",
        "movco.l {{tmp}}, @{{m}}",
        "bf 1b",
        "synco",
        m = in(reg) m,
        old = in(reg) old,
        new = in(reg) new,
        tmp = out(reg) tmp,
        retval = out(reg) retval,
        options(nostack)
    );

    retval
}

// Dependency supplied by the corresponding architecture translation:
// #include <asm/cmpxchg-xchg.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
