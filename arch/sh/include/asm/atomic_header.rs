/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard: __ASM_SH_ATOMIC_H
 *
 * When CONFIG_CPU_J2 is defined, this header includes the generic atomic
 * implementation.  Otherwise it uses the architecture-specific atomic
 * implementation selected below.
 */

#[cfg(CONFIG_CPU_J2)]
// Dependency supplied externally: asm-generic/atomic.h

#[cfg(not(CONFIG_CPU_J2))]
mod non_j2 {
    /*
     * Atomic operations that C can't guarantee us.  Useful for
     * resource counting etc..
     */

    // Dependencies supplied externally:
    // linux/compiler.h, linux/types.h, asm/cmpxchg.h, asm/barrier.h

    /*
     * #define arch_atomic_read(v) READ_ONCE((v)->counter)
     * #define arch_atomic_set(v,i) WRITE_ONCE((v)->counter, (i))
     *
     * READ_ONCE and WRITE_ONCE retain their external dependency and volatile
     * ordering semantics in the translated call sites.
     */
    #[inline(always)]
    pub unsafe fn arch_atomic_read<T>(v: *const T) -> T
    where
        T: Copy,
    {
        core::ptr::read_volatile(v)
    }

    #[inline(always)]
    pub unsafe fn arch_atomic_set<T>(v: *mut T, i: T) {
        core::ptr::write_volatile(v, i);
    }

    #[cfg(CONFIG_GUSA_RB)]
    // Dependency supplied externally: asm/atomic-grb.h

    #[cfg(all(not(CONFIG_GUSA_RB), CONFIG_CPU_SH4A))]
    // Dependency supplied externally: asm/atomic-llsc.h

    #[cfg(all(not(CONFIG_GUSA_RB), not(CONFIG_CPU_SH4A)))]
    // Dependency supplied externally: asm/atomic-irq.h
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
