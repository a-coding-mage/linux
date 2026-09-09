/* SPDX-License-Identifier: GPL-2.0 */

/* These are here in an effort to more fully work around Spitfire Errata
 * #51.  Essentially, if a memory barrier occurs soon after a mispredicted
 * branch, the chip can stop executing instructions until a trap occurs.
 * Therefore, if interrupts are disabled, the chip can hang forever.
 *
 * It used to be believed that the memory barrier had to be right in the
 * delay slot, but a case has been traced recently wherein the memory barrier
 * was one instruction after the branch delay slot and the chip still hung.
 * The offending sequence was the following in sym_wakeup_done() of the
 * sym53c8xx_2 driver:
 *
 *\tcall\tsym_ccb_from_dsa, 0
 *\t movge\t%icc, 0, %l0
 *\t brz,pn\t%o0, .LL1303
 *\t mov\t%o0, %l2
 *\t membar\t#LoadLoad
 *
 * The branch has to be mispredicted for the bug to occur.  Therefore, we put
 * the memory barrier explicitly into a "branch always, predicted taken"
 * delay slot to avoid the problem case.
 */
#[inline(always)]
pub unsafe fn membar_safe(type_: &str) {
    core::arch::asm!(
        "ba,pt %xcc, 1f",
        " membar {0}",
        "1:",
        in(reg) type_,
        options(nostack, preserves_flags)
    );
}

/* The kernel always executes in TSO memory model these days,
 * and furthermore most sparc64 chips implement more stringent
 * memory ordering than required by the specifications.
 */
#[inline(always)]
pub unsafe fn mb() {
    membar_safe("#StoreLoad");
}

#[inline(always)]
pub unsafe fn rmb() {
    core::arch::asm!("", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn wmb() {
    core::arch::asm!("", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __smp_store_release<T>(p: *mut T, v: T) {
    compiletime_assert_atomic_type!(*p);
    barrier!();
    WRITE_ONCE!(*p, v);
}

#[inline(always)]
pub unsafe fn __smp_load_acquire<T: Copy>(p: *const T) -> T {
    let ___p1: T = READ_ONCE!(*p);
    compiletime_assert_atomic_type!(*p);
    barrier!();
    ___p1
}

#[inline(always)]
pub unsafe fn __smp_mb__before_atomic() {
    barrier!();
}

#[inline(always)]
pub unsafe fn __smp_mb__after_atomic() {
    barrier!();
}

/* Dependency: <asm-generic/barrier.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
