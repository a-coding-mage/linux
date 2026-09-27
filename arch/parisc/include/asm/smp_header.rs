/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub fn init_per_cpu(cpuid: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
}

/* Equivalent of CONFIG_SMP. */
#[cfg(CONFIG_SMP)]
pub const PDC_OS_BOOT_RENDEZVOUS: usize = 0x10;
#[cfg(CONFIG_SMP)]
pub const PDC_OS_BOOT_RENDEZVOUS_HI: usize = 0x28;

#[cfg(CONFIG_SMP)]
pub type address_t = ::kernel::ffi::c_ulong;

#[cfg(CONFIG_SMP)]
pub type cpumask = ::kernel::ffi::c_void;

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn smp_send_all_nop();
    pub fn arch_send_call_function_single_ipi(cpu: ::kernel::ffi::c_int);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub const fn cpu_number_map(cpu: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int {
    cpu
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub const fn cpu_logical_map(cpu: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int {
    cpu
}

#[cfg(CONFIG_SMP)]
#[repr(C)]
pub struct thread_info {
    pub cpu: ::kernel::ffi::c_int,
}

#[cfg(CONFIG_SMP)]
extern "C" {
    pub fn current_thread_info() -> *mut thread_info;
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub unsafe fn raw_smp_processor_id() -> ::kernel::ffi::c_int {
    (*current_thread_info()).cpu
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub fn smp_send_all_nop() {}

pub const NO_PROC_ID: u8 = 0xFF; /* No processor magic marker */
pub const ANY_PROC_ID: u8 = 0xFF; /* Any processor magic marker */

extern "C" {
    pub fn __cpu_disable() -> ::kernel::ffi::c_int;
    pub fn __cpu_die(cpu: ::kernel::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
