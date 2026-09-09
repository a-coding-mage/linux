/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the SH LL/SC futex helper.  The `__user` qualifier is a
// kernel address-space annotation and is represented by a raw pointer here.
#[inline]
pub unsafe fn atomic_futex_op_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    let mut err: i32 = 0;

    // The following is the SH inline assembly from the original header.
    // CONFIG_MMU controls the fixup and exception-table section emitted by
    // the kernel build; preserve that intent for the eventual SH backend.
    core::arch::asm!(
        "synco",
        "1:",
        "movli.l @r2, r0",
        "mov r0, r1",
        "cmp/eq r1, r4",
        "bf 2f",
        "mov r5, r0",
        "movco.l r0, @r2",
        "bf 1b",
        "2:",
        "synco",
        in("r2") uaddr,
        in("r4") oldval,
        in("r5") newval,
        lateout("r1") *uval,
        inout("r0") err,
        options(nostack),
    );

    if err != 0 {
        return err;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
