/* SPDX-License-Identifier: GPL-2.0 */

/*
 * include/linux/interrupt_rc.h - refcounted local processor interrupt
 * management.
 *
 * Since the implementation of this API currently depends on
 * local_irq_save()/local_irq_restore(), we split this into its own header to
 * make it easier to include without hitting circular header dependencies.
 *
 * C header dependencies: linux/irqflags.h, linux/preempt.h,
 * linux/processor.h, linux/smp.h
 */

#[cfg(not(feature = "module"))]
extern "C" {
    // DECLARE_PER_CPU(unsigned long, local_interrupt_disable_state);
    pub static mut local_interrupt_disable_state: ::core::ffi::c_ulong;
}

#[cfg(not(feature = "module"))]
#[inline(always)]
pub unsafe fn __local_interrupt_disable() {
    let mut flags: ::core::ffi::c_ulong = 0;

    // local_irq_save(flags);
    local_irq_save(&mut flags);
    // raw_cpu_write(local_interrupt_disable_state, flags);
    raw_cpu_write(&mut local_interrupt_disable_state, flags);
}

#[cfg(not(feature = "module"))]
#[inline(always)]
pub unsafe fn __local_interrupt_enable() {
    let flags: ::core::ffi::c_ulong =
        // raw_cpu_read(local_interrupt_disable_state)
        raw_cpu_read(&local_interrupt_disable_state);

    // local_irq_restore(flags);
    local_irq_restore(flags);
}

#[cfg(all(not(feature = "module"), not(feature = "instantiate_exported_interrupt_disable")))]
#[inline(always)]
pub unsafe fn _local_interrupt_disable() {
    __local_interrupt_disable();
}

#[cfg(all(not(feature = "module"), not(feature = "instantiate_exported_interrupt_disable")))]
#[inline(always)]
pub unsafe fn _local_interrupt_enable() {
    __local_interrupt_enable();
}

#[cfg(any(feature = "module", feature = "instantiate_exported_interrupt_disable"))]
extern "C" {
    pub fn _local_interrupt_disable();
    pub fn _local_interrupt_enable();
}

#[inline]
pub unsafe fn local_interrupt_disable() {
    let mut new_count: ::core::ffi::c_int;

    // WARN_ON_ONCE(in_nmi());
    WARN_ON_ONCE(in_nmi());

    new_count = hardirq_disable_enter();

    /* Interrupts can happen here, but it's OK, see __irq_exit_rcu(). */

    if (new_count & HARDIRQ_DISABLE_MASK) == HARDIRQ_DISABLE_OFFSET {
        _local_interrupt_disable();
    }
}

#[inline]
pub unsafe fn local_interrupt_enable() {
    let new_count: ::core::ffi::c_int;

    new_count = hardirq_disable_exit();

    if (new_count & HARDIRQ_DISABLE_MASK) == 0 {
        _local_interrupt_enable();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
