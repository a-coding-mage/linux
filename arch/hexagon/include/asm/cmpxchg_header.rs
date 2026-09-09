/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * xchg/cmpxchg operations for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* _ASM_CMPXCHG_H */

/*
 * __arch_xchg - atomically exchange a register and a memory location
 * @x: value to swap
 * @ptr: pointer to memory
 * @size: size of the value
 *
 * Only 4 bytes supported currently.
 *
 * Note: there was an errata for V2 about .new's and memw_locked.
 */
#[inline]
pub unsafe fn __arch_xchg(x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize {
    let mut retval: usize;

    /* Can't seem to use printk or panic here, so just stop. */
    if size != 4 {
        loop {
            core::arch::asm!("brkpt;", options(nostack, preserves_flags));
        }
    }

    core::arch::asm!(
        "1: {retval} = memw_locked({ptr});",
        "memw_locked({ptr}, P0) = {x};",
        "if (!P0) jump 1b;",
        retval = out(reg) retval,
        ptr = in(reg) ptr,
        x = in(reg) x,
        options(nostack)
    );
    retval
}

/*
 * Atomically swap the contents of a register with memory. Should be atomic
 * between multiple CPU's and within interrupts on the same CPU.
 */
#[macro_export]
macro_rules! arch_xchg {
    ($ptr:expr, $v:expr) => {{
        let __xchg_ptr = $ptr;
        let __xchg_value = $v;
        $crate::__arch_xchg(
            __xchg_value as usize,
            __xchg_ptr as *mut core::ffi::c_void,
            core::mem::size_of_val(unsafe { &*__xchg_ptr }) as i32,
        ) as _
    }};
}

/*
 * see rt-mutex-design.txt; cmpxchg supposedly checks if *ptr == A and swaps.
 * looks just like atomic_cmpxchg on our arch currently with a bunch of
 * variable casting.
 */
#[macro_export]
macro_rules! arch_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr) => {{
        let __ptr = $ptr;
        let __old = $old;
        let __new = $new;
        let mut __oldval = 0 as _;

        unsafe {
            core::arch::asm!(
                "1: {oldval} = memw_locked({ptr});",
                "{{ P0 = cmp.eq({oldval},{old});",
                "if (!P0.new) jump:nt 2f; }}",
                "memw_locked({ptr},p0) = {new};",
                "if (!P0) jump 1b;",
                "2:",
                oldval = inout(reg) __oldval,
                ptr = in(reg) __ptr,
                old = in(reg) __old,
                new = in(reg) __new,
                options(nostack)
            );
        }
        __oldval
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
