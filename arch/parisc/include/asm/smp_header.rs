/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub fn init_per_cpu(cpuid: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

/* Equivalent of CONFIG_SMP. */
#[cfg(feature = "CONFIG_SMP")]
pub const PDC_OS_BOOT_RENDEZVOUS: usize = 0x10;
#[cfg(feature = "CONFIG_SMP")]
pub const PDC_OS_BOOT_RENDEZVOUS_HI: usize = 0x28;

#[cfg(feature = "CONFIG_SMP")]
pub type address_t = ::core::ffi::c_ulong;

#[cfg(feature = "CONFIG_SMP")]
pub type cpumask = ::core::ffi::c_void;

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn smp_send_all_nop();
    pub fn arch_send_call_function_single_ipi(cpu: ::core::ffi::c_int);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub const fn cpu_number_map(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    cpu
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub const fn cpu_logical_map(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    cpu
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct thread_info {
    pub cpu: ::core::ffi::c_int,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn current_thread_info() -> *mut thread_info;
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn raw_smp_processor_id() -> ::core::ffi::c_int {
    (*current_thread_info()).cpu
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub fn smp_send_all_nop() {}

pub const NO_PROC_ID: u8 = 0xFF; /* No processor magic marker */
pub const ANY_PROC_ID: u8 = 0xFF; /* Any processor magic marker */

extern "C" {
    pub fn __cpu_disable() -> ::core::ffi::c_int;
    pub fn __cpu_die(cpu: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
