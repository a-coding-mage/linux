/*
 * Copyright (C) 2014 Stefan Kristiansson <stefan.kristiansson@saunalahti.fi>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Dependency declarations supplied by the surrounding kernel translation:
// linux/cpumask.h, asm/spr.h, and asm/spr_defs.h.

/// Opaque C `struct cpumask` supplied by the surrounding translation.
#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

/// Equivalent of the C macro `raw_smp_processor_id()`.
#[macro_export]
macro_rules! raw_smp_processor_id {
    () => {
        current_thread_info().cpu
    };
}

/// Equivalent of the C macro `hard_smp_processor_id()`.
#[macro_export]
macro_rules! hard_smp_processor_id {
    () => {
        mfspr(SPR_COREID)
    };
}

extern "C" {
    pub fn smp_init_cpus();

    pub fn arch_send_call_function_single_ipi(cpu: ::core::ffi::c_int);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);

    pub fn set_smp_cross_call(
        callback: Option<unsafe extern "C" fn(*const cpumask, ::core::ffi::c_uint)>,
        irq: ::core::ffi::c_uint,
    );
    pub fn handle_IPI(ipi_msg: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
