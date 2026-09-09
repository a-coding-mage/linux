/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below are present only when CONFIG_SMP is enabled in the
// C build. Preserve that build-time condition at the integration boundary.

#[cfg(feature = "CONFIG_SMP")]
use core::ffi::c_int;

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn uml_curr_cpu() -> c_int;

    pub fn arch_smp_send_reschedule(cpu: c_int);

    pub fn arch_send_call_function_single_ipi(cpu: c_int);

    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
}

#[cfg(feature = "CONFIG_SMP")]
macro_rules! raw_smp_processor_id {
    () => {
        unsafe { uml_curr_cpu() }
    };
}

#[cfg(feature = "CONFIG_SMP")]
pub(crate) use raw_smp_processor_id;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
