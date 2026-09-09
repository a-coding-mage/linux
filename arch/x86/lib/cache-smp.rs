// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

extern "C" {
    fn smp_call_function_single(
        cpu: i32,
        func: Option<unsafe extern "C" fn(*mut c_void)>,
        info: *mut c_void,
        wait: i32,
    );
    fn on_each_cpu(
        func: Option<unsafe extern "C" fn(*mut c_void)>,
        info: *mut c_void,
        wait: i32,
    );
    fn on_each_cpu_mask(
        cpus: *mut cpumask,
        func: Option<unsafe extern "C" fn(*mut c_void)>,
        info: *mut c_void,
        wait: i32,
    );
}

unsafe extern "C" fn __wbinvd(_dummy: *mut c_void) {
    core::arch::asm!("wbinvd", options(nostack, preserves_flags));
}

pub unsafe extern "C" fn wbinvd_on_cpu(cpu: i32) {
    smp_call_function_single(cpu, Some(__wbinvd), core::ptr::null_mut(), 1);
}

// EXPORT_SYMBOL_FOR_KVM(wbinvd_on_cpu);

pub unsafe extern "C" fn wbinvd_on_all_cpus() {
    on_each_cpu(Some(__wbinvd), core::ptr::null_mut(), 1);
}

// EXPORT_SYMBOL(wbinvd_on_all_cpus);

pub unsafe extern "C" fn wbinvd_on_cpus_mask(cpus: *mut cpumask) {
    on_each_cpu_mask(cpus, Some(__wbinvd), core::ptr::null_mut(), 1);
}

// EXPORT_SYMBOL_FOR_KVM(wbinvd_on_cpus_mask);

unsafe extern "C" fn __wbnoinvd(_dummy: *mut c_void) {
    core::arch::asm!("wbnoinvd", options(nostack, preserves_flags));
}

pub unsafe extern "C" fn wbnoinvd_on_all_cpus() {
    on_each_cpu(Some(__wbnoinvd), core::ptr::null_mut(), 1);
}

// EXPORT_SYMBOL_FOR_KVM(wbnoinvd_on_all_cpus);

pub unsafe extern "C" fn wbnoinvd_on_cpus_mask(cpus: *mut cpumask) {
    on_each_cpu_mask(cpus, Some(__wbnoinvd), core::ptr::null_mut(), 1);
}

// EXPORT_SYMBOL_FOR_KVM(wbnoinvd_on_cpus_mask);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
