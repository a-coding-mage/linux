/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <abi/reg_ops.h>; its external register operations are
// referenced here as declarations supplied by the surrounding translation.
unsafe extern "C" {
    fn mfcr(reg: *const core::ffi::c_char) -> core::ffi::c_ulong;
    fn mtcr(reg: *const core::ffi::c_char, value: core::ffi::c_ulong);
}

#[inline]
pub unsafe fn arch_local_irq_save() -> core::ffi::c_ulong {
    let flags: core::ffi::c_ulong;

    flags = mfcr(c"psr".as_ptr());
    core::arch::asm!("psrclr ie", options(nostack));
    flags
}

#[inline]
pub unsafe fn arch_local_irq_enable() {
    core::arch::asm!("psrset ee, ie", options(nostack));
}

#[inline]
pub unsafe fn arch_local_irq_disable() {
    core::arch::asm!("psrclr ie", options(nostack));
}

#[inline]
pub unsafe fn arch_local_save_flags() -> core::ffi::c_ulong {
    mfcr(c"psr".as_ptr())
}

#[inline]
pub unsafe fn arch_local_irq_restore(flags: core::ffi::c_ulong) {
    mtcr(c"psr".as_ptr(), flags);
}

#[inline]
pub const fn arch_irqs_disabled_flags(flags: core::ffi::c_ulong) -> core::ffi::c_int {
    (!(flags & (1 << 6))) as core::ffi::c_int
}

// The C header also includes <asm-generic/irqflags.h>; its declarations are
// supplied by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
