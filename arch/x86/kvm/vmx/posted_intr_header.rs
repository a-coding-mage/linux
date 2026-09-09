/* SPDX-License-Identifier: GPL-2.0 */
// Translated from x86/kvm/vmx/posted_intr.h.
// C header dependencies are supplied by the surrounding kernel translation.

unsafe extern "C" {
    pub fn vmx_vcpu_pi_load(vcpu: *mut kvm_vcpu, cpu: ::core::ffi::c_int);
    pub fn vmx_vcpu_pi_put(vcpu: *mut kvm_vcpu);
    pub fn pi_wakeup_handler();
    // C __init annotation: initialization-only function.
    pub fn pi_init_cpu(cpu: ::core::ffi::c_int);
    pub fn pi_apicv_pre_state_restore(vcpu: *mut kvm_vcpu);
    pub fn pi_has_pending_interrupt(vcpu: *mut kvm_vcpu) -> bool;
    pub fn vmx_pi_update_irte(
        irqfd: *mut kvm_kernel_irqfd,
        kvm: *mut kvm,
        host_irq: u32,
        guest_irq: u32,
        vcpu: *mut kvm_vcpu,
        vector: u32,
    ) -> ::core::ffi::c_int;
    pub fn vmx_pi_start_bypass(kvm: *mut kvm);

    pub fn find_last_bit(addr: *const ::core::ffi::c_ulong, size: usize) -> usize;
}

#[inline]
pub unsafe fn pi_find_highest_vector(pi_desc: *mut pi_desc) -> ::core::ffi::c_int {
    let vec: usize = unsafe { find_last_bit((*pi_desc).pir.as_ptr() as *const _, 256) };
    if vec < 256 {
        vec as ::core::ffi::c_int
    } else {
        -1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
