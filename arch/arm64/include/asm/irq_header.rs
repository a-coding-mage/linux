/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/cpumask.h> and <asm-generic/irq.h>.

extern "C" {
    pub fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: ::core::ffi::c_int);

    pub fn set_handle_irq(handle_irq: Option<unsafe extern "C" fn(*mut pt_regs)>)
        -> ::core::ffi::c_int;

    pub fn set_handle_fiq(handle_fiq: Option<unsafe extern "C" fn(*mut pt_regs)>);
}

// The C self-referential macros preserve the declaration names and have no
// additional Rust definition.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub unsafe fn nr_legacy_irqs() -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
