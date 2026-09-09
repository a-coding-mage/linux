/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency supplied by the translated processor header.
// `xtensa_get_sr` reads the named Xtensa special register.

#[inline(always)]
pub unsafe fn xip_irqpending() -> u32 {
    xtensa_get_sr("interrupt") & xtensa_get_sr("intenable")
}

#[inline(always)]
pub unsafe fn xip_currtime() -> u32 {
    xtensa_get_sr("ccount")
}

#[inline(always)]
pub unsafe fn xip_elapsed_since(x: u32) -> u32 {
    // Should work up to 1GHz.
    xtensa_get_sr("ccount").wrapping_sub(x) / 1000
}

#[inline(always)]
pub unsafe fn xip_cpu_idle() {
    core::arch::asm!("waiti 0", options(nomem, nostack, preserves_flags));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
