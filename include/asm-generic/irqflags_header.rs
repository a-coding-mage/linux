/* SPDX-License-Identifier: GPL-2.0 */

// All architectures should implement at least the first two functions;
// usually inline assembly will be the best way.
//
// The C preprocessor conditions are preserved here as intent: architecture-
// specific definitions may override these generic defaults.
pub const ARCH_IRQ_DISABLED: ::kernel::ffi::c_int = 0;
pub const ARCH_IRQ_ENABLED: ::kernel::ffi::c_int = 1;

// read interrupt enabled status
unsafe extern "C" {
    pub fn arch_local_save_flags() -> ::kernel::ffi::c_ulong;
}

// set interrupt enabled status
unsafe extern "C" {
    pub fn arch_local_irq_restore(flags: ::kernel::ffi::c_ulong);
}

// get status and disable interrupts
#[inline]
pub unsafe fn arch_local_irq_save() -> ::kernel::ffi::c_ulong {
    let flags: ::kernel::ffi::c_ulong = unsafe { arch_local_save_flags() };
    unsafe { arch_local_irq_restore(ARCH_IRQ_DISABLED as ::kernel::ffi::c_ulong) };
    flags
}

// test flags
#[inline]
pub unsafe fn arch_irqs_disabled_flags(flags: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int {
    (flags == ARCH_IRQ_DISABLED as ::kernel::ffi::c_ulong) as ::kernel::ffi::c_int
}

// unconditionally enable interrupts
#[inline]
pub unsafe fn arch_local_irq_enable() {
    unsafe { arch_local_irq_restore(ARCH_IRQ_ENABLED as ::kernel::ffi::c_ulong) };
}

// unconditionally disable interrupts
#[inline]
pub unsafe fn arch_local_irq_disable() {
    unsafe { arch_local_irq_restore(ARCH_IRQ_DISABLED as ::kernel::ffi::c_ulong) };
}

// test hardware interrupt enable bit
#[inline]
pub unsafe fn arch_irqs_disabled() -> ::kernel::ffi::c_int {
    unsafe { arch_irqs_disabled_flags(arch_local_save_flags()) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
