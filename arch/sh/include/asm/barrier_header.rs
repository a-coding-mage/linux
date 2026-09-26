/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999, 2000  Niibe Yutaka  &  Kaz Kojima
 * Copyright (C) 2002 Paul Mundt
 */

/*
 * A brief note on ctrl_barrier(), the control register write barrier.
 *
 * Legacy SH cores typically require a sequence of 8 nops after
 * modification of a control register in order for the changes to take
 * effect. On newer cores (like the sh4a and sh5) this is accomplished
 * with icbi.
 *
 * Also note that on sh4a in the icbi case we can forego a synco for the
 * write barrier, as it's not necessary for control registers.
 *
 * Historically we have only done this type of barrier for the MMUCR, but
 * it's also necessary for the CCR, so we make it generic here instead.
 */

#[cfg(CONFIG_CPU_SH4A)]
#[inline(always)]
pub unsafe fn mb() {
    core::arch::asm!("synco", options(nostack, preserves_flags));
}

#[cfg(CONFIG_CPU_SH4A)]
#[inline(always)]
pub unsafe fn rmb() {
    mb();
}

#[cfg(CONFIG_CPU_SH4A)]
#[inline(always)]
pub unsafe fn wmb() {
    mb();
}

#[cfg(CONFIG_CPU_SH4A)]
#[macro_export]
macro_rules! ctrl_barrier {
    () => {{
        unsafe { $crate::__icbi($crate::PAGE_OFFSET) }
    }};
}

#[cfg(not(CONFIG_CPU_SH4A))]
#[cfg(all(CONFIG_CPU_J2, CONFIG_SMP))]
#[inline(always)]
pub unsafe fn __smp_mb() {
    let mut tmp: i32 = 0;
    core::arch::asm!(
        "cas.l {tmp},{tmp},@{ptr}",
        tmp = inout(reg) tmp,
        ptr = in(reg) (&mut tmp),
        options(nostack)
    );
}

#[cfg(not(CONFIG_CPU_SH4A))]
#[cfg(all(CONFIG_CPU_J2, CONFIG_SMP))]
#[inline(always)]
pub unsafe fn __smp_rmb() {
    __smp_mb();
}

#[cfg(not(CONFIG_CPU_SH4A))]
#[cfg(all(CONFIG_CPU_J2, CONFIG_SMP))]
#[inline(always)]
pub unsafe fn __smp_wmb() {
    __smp_mb();
}

#[cfg(not(CONFIG_CPU_SH4A))]
#[macro_export]
macro_rules! ctrl_barrier {
    () => {{
        unsafe {
            core::arch::asm!(
                "nop; nop; nop; nop; nop; nop; nop; nop",
                options(nostack, preserves_flags)
            );
        }
    }};
}

#[macro_export]
macro_rules! __smp_store_mb {
    ($var:expr, $value:expr) => {{
        let _ = $crate::xchg(&mut $var, $value);
    }};
}

/* The following symbols are supplied by the architecture's other headers. */
extern "Rust" {
    fn __icbi(address: usize);
    fn xchg<T>(var: *mut T, value: T) -> T;
    static PAGE_OFFSET: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
