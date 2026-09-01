/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C dependency intent:
 * - <linux/compiler.h>
 * - architecture-specific barrier.h selected for:
 *   __i386__, __x86_64__, __arm__, __aarch64__, __powerpc__, __riscv,
 *   __s390__, __sh__, __sparc__, __tile__, __alpha__, __mips__, __ia64__,
 *   __xtensa__
 * - otherwise <asm-generic/barrier.h>
 */

/*
 * Generic fallback smp_*() definitions for archs that haven't
 * been updated yet.
 */

unsafe extern "C" {
    fn rmb();
    fn wmb();
    fn mb();
}

/* Fallback for C's "#ifndef smp_rmb": smp_rmb() -> rmb(). */
pub unsafe fn smp_rmb() {
    unsafe {
        rmb();
    }
}

/* Fallback for C's "#ifndef smp_wmb": smp_wmb() -> wmb(). */
pub unsafe fn smp_wmb() {
    unsafe {
        wmb();
    }
}

/* Fallback for C's "#ifndef smp_mb": smp_mb() -> mb(). */
pub unsafe fn smp_mb() {
    unsafe {
        mb();
    }
}

/*
 * Fallback for C's "#ifndef smp_store_release":
 *
 * # define smp_store_release(p, v) \
 * do {                             \
 *     smp_mb();                    \
 *     WRITE_ONCE(*p, v);           \
 * } while (0)
 */
pub unsafe fn smp_store_release<T>(p: *mut T, v: T) {
    unsafe {
        smp_mb();
        core::ptr::write_volatile(p, v);
    }
}

/*
 * Fallback for C's "#ifndef smp_load_acquire":
 *
 * # define smp_load_acquire(p)          \
 * ({                                    \
 *     typeof(*p) ___p1 = READ_ONCE(*p); \
 *     smp_mb();                         \
 *     ___p1;                            \
 * })
 */
pub unsafe fn smp_load_acquire<T>(p: *const T) -> T {
    unsafe {
        let ___p1 = core::ptr::read_volatile(p);
        smp_mb();
        ___p1
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
