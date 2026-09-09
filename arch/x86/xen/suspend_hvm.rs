// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_int;

// Declarations supplied by the Xen and Linux dependencies.
extern "C" {
    fn xen_hvm_init_shared_info();
    fn xen_vcpu_restore();
    static mut xen_percpu_upcall: c_int;
    fn xen_set_upcall_vector(cpu: u32) -> c_int;
    fn xen_setup_callback_vector();
    fn xen_unplug_emulated_devices();
    fn xen_for_each_online_cpu(callback: unsafe extern "C" fn(u32));
}

unsafe extern "C" fn xen_hvm_restore_upcall_vector(cpu: u32) {
    // BUG_ON(xen_set_upcall_vector(cpu));
    if xen_set_upcall_vector(cpu) != 0 {
        core::hint::unreachable_unchecked();
    }
}

pub unsafe fn xen_hvm_post_suspend(suspend_cancelled: c_int) {
    if suspend_cancelled == 0 {
        xen_hvm_init_shared_info();
        xen_vcpu_restore();
    }
    if xen_percpu_upcall != 0 {
        // C macro: for_each_online_cpu(cpu)
        xen_for_each_online_cpu(xen_hvm_restore_upcall_vector);
    } else {
        xen_setup_callback_vector();
    }
    xen_unplug_emulated_devices();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
