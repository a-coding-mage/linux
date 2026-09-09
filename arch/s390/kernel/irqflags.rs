// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by <asm/irqflags.h>.
unsafe extern "C" {
    fn __arch_local_save_flags() -> ::core::ffi::c_ulong;
    fn __arch_local_irq_save() -> ::core::ffi::c_ulong;
    fn __arch_local_irq_enable_external();
    fn __arch_local_irq_enable();
}

// noinstr unsigned long arch_local_save_flags(void)
#[no_mangle]
pub unsafe extern "C" fn arch_local_save_flags() -> ::core::ffi::c_ulong {
    unsafe { __arch_local_save_flags() }
}

// EXPORT_SYMBOL(arch_local_save_flags);

// noinstr unsigned long arch_local_irq_save(void)
#[no_mangle]
pub unsafe extern "C" fn arch_local_irq_save() -> ::core::ffi::c_ulong {
    unsafe { __arch_local_irq_save() }
}

// EXPORT_SYMBOL(arch_local_irq_save);

// noinstr void arch_local_irq_enable_external(void)
#[no_mangle]
pub unsafe extern "C" fn arch_local_irq_enable_external() {
    unsafe { __arch_local_irq_enable_external() }
}

// EXPORT_SYMBOL(arch_local_irq_enable_external);

// noinstr void arch_local_irq_enable(void)
#[no_mangle]
pub unsafe extern "C" fn arch_local_irq_enable() {
    unsafe { __arch_local_irq_enable() }
}

// EXPORT_SYMBOL(arch_local_irq_enable);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
