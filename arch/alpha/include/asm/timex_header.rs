/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm-alpha/timex.h
 *
 * ALPHA architecture timex specifications
 */

/*
 * Standard way to access the cycle counter.
 * Currently only used on SMP for scheduling.
 *
 * Only the low 32 bits are available as a continuously counting entity.
 * But this only means we'll force a reschedule every 8 seconds or so,
 * which isn't an evil thing.
 */

pub type cycles_t = u32;

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    let mut ret: cycles_t;
    core::arch::asm!("rpcc {0}", out(reg) ret);
    ret
}

// C macro: #define get_cycles get_cycles

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
