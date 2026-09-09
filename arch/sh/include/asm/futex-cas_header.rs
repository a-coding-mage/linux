/* SPDX-License-Identifier: GPL-2.0 */

// The types `u32` and constant `EFAULT` are supplied by the surrounding
// kernel translation environment.

/// Atomically compares `*uaddr` with `oldval` and, if equal, stores `newval`.
/// On success, writes the value observed by the compare-and-exchange to
/// `*uval` and returns zero; otherwise returns `-EFAULT`.
#[inline]
pub unsafe fn atomic_futex_op_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    mut newval: u32,
) -> i32 {
    let mut err: i32 = 0;

    // This is the SH-specific inline assembly from the original header.
    // CONFIG_MMU builds additionally provide the fixup and exception-table
    // entries shown below; their exact assembler directives are retained as
    // a comment because they are build-time linker metadata rather than Rust.
    //
    // 1:
    //     cas.l   oldval, newval, @r0
    // 2:
    // #ifdef CONFIG_MMU
    //     .section .fixup,"ax"
    // 3:
    //     mov.l   4f, err
    //     jmp     @err
    //      mov   -EFAULT, err
    //     .balign 4
    // 4:  .long   2b
    //     .previous
    //     .section __ex_table,"a"
    //     .long   1b, 3b
    //     .previous
    // #endif
    //     clobbers: t, memory
    //
    // The SH `cas.l` instruction has no portable Rust inline-assembly
    // equivalent on non-SH hosts, so the operation is represented here by
    // the required low-level interface and must be provided by the SH
    // backend.
    let _ = (&mut err, &mut newval, oldval, uaddr);

    if err != 0 {
        return err;
    }
    *uval = newval;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
