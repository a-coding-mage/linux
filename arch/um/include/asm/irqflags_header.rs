/* SPDX-License-Identifier: GPL-2.0 */

// Declarations provided by the UML signal implementation.
unsafe extern "C" {
    pub fn um_get_signals() -> core::ffi::c_int;
    pub fn um_set_signals(enable: core::ffi::c_int) -> core::ffi::c_int;
    pub fn block_signals();
    pub fn unblock_signals();
}

// C self-referential macro aliases:
// #define arch_local_save_flags arch_local_save_flags
#[inline]
pub unsafe fn arch_local_save_flags() -> core::ffi::c_ulong {
    um_get_signals() as core::ffi::c_ulong
}

// C self-referential macro alias:
// #define arch_local_irq_restore arch_local_irq_restore
#[inline]
pub unsafe fn arch_local_irq_restore(flags: core::ffi::c_ulong) {
    um_set_signals(flags as core::ffi::c_int);
}

// C self-referential macro alias:
// #define arch_local_irq_enable arch_local_irq_enable
#[inline]
pub unsafe fn arch_local_irq_enable() {
    unblock_signals();
}

// C self-referential macro alias:
// #define arch_local_irq_disable arch_local_irq_disable
#[inline]
pub unsafe fn arch_local_irq_disable() {
    block_signals();
}

pub const ARCH_IRQ_DISABLED: core::ffi::c_int = 0;

// The C header includes <asm-generic/irqflags.h>; its declarations and
// definitions are supplied by the surrounding translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
