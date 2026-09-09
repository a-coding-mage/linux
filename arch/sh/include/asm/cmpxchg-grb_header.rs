/* SPDX-License-Identifier: GPL-2.0 */

// The original header uses SH-specific inline assembly.  The assembly is kept
// verbatim in Rust `asm!` blocks so that its register, ordering, and volatile
// memory semantics remain explicit.

#[inline]
pub unsafe fn xchg_u32(m: *mut core::ffi::c_void, val: usize) -> usize {
    let retval: usize;
    core::arch::asm!(
        ".align 2",
        "mova 1f, r0",
        "nop",
        "mov r15, r1",
        "mov #-4, r15",
        "mov.l @r1, r0",
        "mov.l r2, @r1",
        "1: mov r1, r15",
        inout("r1") m => _,
        inout("r2") val => _,
        lateout("r0") retval,
        options(nostack)
    );
    retval
}

#[inline]
pub unsafe fn xchg_u16(m: *mut core::ffi::c_void, val: usize) -> usize {
    let retval: usize;
    core::arch::asm!(
        ".align 2",
        "mova 1f, r0",
        "mov r15, r1",
        "mov #-6, r15",
        "mov.w @r1, r0",
        "extu.w r0, r0",
        "mov.w r2, @r1",
        "1: mov r1, r15",
        inout("r1") m => _,
        inout("r2") val => _,
        lateout("r0") retval,
        options(nostack)
    );
    retval
}

#[inline]
pub unsafe fn xchg_u8(m: *mut core::ffi::c_void, val: usize) -> usize {
    let retval: usize;
    core::arch::asm!(
        ".align 2",
        "mova 1f, r0",
        "mov r15, r1",
        "mov #-6, r15",
        "mov.b @r1, r0",
        "extu.b r0, r0",
        "mov.b r2, @r1",
        "1: mov r1, r15",
        inout("r1") m => _,
        inout("r2") val => _,
        lateout("r0") retval,
        options(nostack)
    );
    retval
}

#[inline]
pub unsafe fn __cmpxchg_u32(
    m: *const core::ffi::c_int,
    old: usize,
    new: usize,
) -> usize {
    let retval: usize;
    core::arch::asm!(
        ".align 2",
        "mova 1f, r0",
        "nop",
        "mov r15, r1",
        "mov #-8, r15",
        "mov.l @r3, r0",
        "cmp/eq r0, r1",
        "bf 1f",
        "mov.l r2, @r3",
        "1: mov r1, r15",
        inout("r1") old => _,
        inout("r2") new => _,
        in("r3") m,
        lateout("r0") retval,
        options(nostack)
    );
    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
