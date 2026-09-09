/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/cpumask.h>, <asm/machvec.h>, and <asm-generic/irq.h>.

/*
 * This is a special IRQ number for indicating that no IRQ has been
 * triggered and to simply ignore the IRQ dispatch. This is a special
 * case that can happen with IRQ auto-distribution when multiple CPUs
 * are woken up and signalled in parallel.
 */
pub const NO_IRQ_IGNORE: ::core::ffi::c_uint = !0u32;

/* PINT IRQs */
unsafe extern "C" {
    pub fn make_imask_irq(irq: ::core::ffi::c_uint);
}

pub fn generic_irq_demux(irq: ::core::ffi::c_int) -> ::core::ffi::c_int {
    irq
}

// C macro: sh_mv.mv_irq_demux(irq)
#[macro_export]
macro_rules! irq_demux {
    ($irq:expr) => {
        sh_mv.mv_irq_demux($irq)
    };
}

unsafe extern "C" {
    pub fn init_IRQ();
    pub fn migrate_irqs();

    // C declaration uses the asmlinkage calling convention.
    pub fn do_IRQ(
        irq: ::core::ffi::c_uint,
        regs: *mut crate::pt_regs,
    ) -> ::core::ffi::c_int;
}

// CONFIG_IRQSTACKS is a build-time condition from the C source.
#[cfg(feature = "CONFIG_IRQSTACKS")]
unsafe extern "C" {
    pub fn irq_ctx_init(cpu: ::core::ffi::c_int);
    pub fn irq_ctx_exit(cpu: ::core::ffi::c_int);
}

#[cfg(not(feature = "CONFIG_IRQSTACKS"))]
#[inline]
pub fn irq_ctx_init(_cpu: ::core::ffi::c_int) {}

#[cfg(not(feature = "CONFIG_IRQSTACKS"))]
#[inline]
pub fn irq_ctx_exit(_cpu: ::core::ffi::c_int) {}

// CONFIG_INTC_BALANCING is a build-time condition from the C source.
#[cfg(feature = "CONFIG_INTC_BALANCING")]
unsafe extern "C" {
    pub fn irq_lookup(irq: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn irq_finish(irq: ::core::ffi::c_uint);
}

#[cfg(not(feature = "CONFIG_INTC_BALANCING"))]
#[inline]
pub fn irq_lookup(irq: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    irq
}

#[cfg(not(feature = "CONFIG_INTC_BALANCING"))]
#[inline]
pub fn irq_finish(_irq: ::core::ffi::c_uint) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
