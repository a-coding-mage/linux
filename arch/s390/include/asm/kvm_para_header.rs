/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definition for paravirtual devices on s390.
 *
 * Hypercalls for KVM on s390 use R2-R6 for parameters 1-5, R1 as the
 * hypercall number, R7 as parameter 6, and return the result in R2.  The
 * diagnose instruction is used with KVM hypercall number 0x500.
 */

// C dependencies: <uapi/asm/kvm_para.h> and <asm/diag.h>.
use core::ffi::{c_int, c_long, c_uint, c_ulong};

#[inline(always)]
unsafe fn __kvm_hypercall0(nr: c_ulong) -> c_long {
    let mut rc: c_long;
    core::arch::asm!(
        "diag 2,4,0x500",
        in("r1") nr,
        lateout("r2") rc,
        options(nostack)
    );
    rc
}

#[inline(always)]
unsafe fn __kvm_hypercall1(nr: c_ulong, arg1: c_ulong) -> c_long {
    let mut rc: c_long;
    core::arch::asm!(
        "diag 2,4,0x500",
        in("r1") nr,
        in("r2") arg1,
        lateout("r2") rc,
        options(nostack)
    );
    rc
}

#[inline(always)]
unsafe fn __kvm_hypercall2(nr: c_ulong, arg1: c_ulong, arg2: c_ulong) -> c_long {
    let mut rc: c_long;
    core::arch::asm!(
        "diag 2,4,0x500",
        in("r1") nr,
        in("r2") arg1,
        in("r3") arg2,
        lateout("r2") rc,
        options(nostack)
    );
    rc
}

#[inline(always)]
unsafe fn __kvm_hypercall3(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong) -> c_long {
    let mut rc: c_long;
    core::arch::asm!(
        "diag 2,4,0x500",
        in("r1") nr,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
        lateout("r2") rc,
        options(nostack)
    );
    rc
}

#[inline(always)]
unsafe fn __kvm_hypercall4(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong) -> c_long {
    let mut rc: c_long;
    core::arch::asm!(
        "diag 2,4,0x500",
        in("r1") nr,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
        in("r5") arg4,
        lateout("r2") rc,
        options(nostack)
    );
    rc
}

#[inline(always)]
unsafe fn __kvm_hypercall5(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_long {
    let mut rc: c_long;
    core::arch::asm!(
        "diag 2,4,0x500",
        in("r1") nr,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
        in("r5") arg4,
        in("r6") arg5,
        lateout("r2") rc,
        options(nostack)
    );
    rc
}

#[inline(always)]
unsafe fn __kvm_hypercall6(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong, arg6: c_ulong) -> c_long {
    let mut rc: c_long;
    core::arch::asm!(
        "diag 2,4,0x500",
        in("r1") nr,
        in("r2") arg1,
        in("r3") arg2,
        in("r4") arg3,
        in("r5") arg4,
        in("r6") arg5,
        in("r7") arg6,
        lateout("r2") rc,
        options(nostack)
    );
    rc
}

#[inline(always)]
fn kvm_hypercall0(nr: c_ulong) -> c_long {
    diag_stat_inc(DIAG_STAT_X500);
    unsafe { __kvm_hypercall0(nr) }
}

#[inline(always)]
fn kvm_hypercall1(nr: c_ulong, arg1: c_ulong) -> c_long {
    diag_stat_inc(DIAG_STAT_X500);
    unsafe { __kvm_hypercall1(nr, arg1) }
}

#[inline(always)]
fn kvm_hypercall2(nr: c_ulong, arg1: c_ulong, arg2: c_ulong) -> c_long {
    diag_stat_inc(DIAG_STAT_X500);
    unsafe { __kvm_hypercall2(nr, arg1, arg2) }
}

#[inline(always)]
fn kvm_hypercall3(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong) -> c_long {
    diag_stat_inc(DIAG_STAT_X500);
    unsafe { __kvm_hypercall3(nr, arg1, arg2, arg3) }
}

#[inline(always)]
fn kvm_hypercall4(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong) -> c_long {
    diag_stat_inc(DIAG_STAT_X500);
    unsafe { __kvm_hypercall4(nr, arg1, arg2, arg3, arg4) }
}

#[inline(always)]
fn kvm_hypercall5(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_long {
    diag_stat_inc(DIAG_STAT_X500);
    unsafe { __kvm_hypercall5(nr, arg1, arg2, arg3, arg4, arg5) }
}

#[inline(always)]
fn kvm_hypercall6(nr: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong, arg6: c_ulong) -> c_long {
    diag_stat_inc(DIAG_STAT_X500);
    unsafe { __kvm_hypercall6(nr, arg1, arg2, arg3, arg4, arg5, arg6) }
}

/* KVM on s390 is always paravirtualization enabled. */
#[inline(always)]
fn kvm_para_available() -> c_int {
    1
}

/* No feature bits are currently assigned for KVM on s390. */
#[inline(always)]
fn kvm_arch_para_features() -> c_uint {
    0
}

#[inline(always)]
fn kvm_arch_para_hints() -> c_uint {
    0
}

#[inline(always)]
fn kvm_check_and_clear_guest_paused() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
