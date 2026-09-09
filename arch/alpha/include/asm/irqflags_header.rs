/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/pal.h in the original header.
unsafe extern "C" {
    fn rdps() -> u64;
    fn swpipl(ipl: u64) -> u64;
    fn barrier();
}

pub const IPL_MIN: u64 = 0;
pub const IPL_SW0: u64 = 1;
pub const IPL_SW1: u64 = 2;
pub const IPL_DEV0: u64 = 3;
pub const IPL_DEV1: u64 = 4;
pub const IPL_TIMER: u64 = 5;
pub const IPL_PERF: u64 = 6;
pub const IPL_POWERFAIL: u64 = 6;
pub const IPL_MCHECK: u64 = 7;
pub const IPL_MAX: u64 = 7;

// Under CONFIG_ALPHA_BROKEN_IRQ_MASK, IPL_MIN is instead __min_ipl.
#[cfg(CONFIG_ALPHA_BROKEN_IRQ_MASK)]
unsafe extern "C" {
    pub static mut __min_ipl: i32;
}

#[cfg(CONFIG_ALPHA_BROKEN_IRQ_MASK)]
#[allow(non_upper_case_globals)]
pub const IPL_MIN_CONFIGURED: i32 = 0; // Build-time replacement: __min_ipl.

#[inline]
pub unsafe fn getipl() -> u64 {
    unsafe { rdps() & 7 }
}

#[inline]
pub unsafe fn setipl(ipl: u64) {
    unsafe { swpipl(ipl); }
}

#[inline]
pub unsafe fn arch_local_save_flags() -> u64 {
    unsafe { getipl() }
}

#[inline]
pub unsafe fn arch_local_irq_disable() {
    unsafe {
        setipl(IPL_MAX);
        barrier();
    }
}

#[inline]
pub unsafe fn arch_local_irq_save() -> u64 {
    let flags = unsafe { swpipl(IPL_MAX) };
    unsafe { barrier(); }
    flags
}

#[inline]
pub unsafe fn arch_local_irq_enable() {
    unsafe {
        barrier();
        setipl(IPL_MIN);
    }
}

#[inline]
pub unsafe fn arch_local_irq_restore(flags: u64) {
    unsafe {
        barrier();
        setipl(flags & 7);
        barrier();
    }
}

#[inline]
pub unsafe fn arch_irqs_disabled_flags(flags: u64) -> bool {
    (flags & 7) == IPL_MAX
}

#[inline]
pub unsafe fn arch_irqs_disabled() -> bool {
    unsafe { arch_irqs_disabled_flags(getipl()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
