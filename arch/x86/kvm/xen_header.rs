// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright © 2019 Oracle and/or its affiliates. All rights reserved.
 * Copyright © 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * KVM Xen emulation
 */

// Translated from the C header. External kernel types, constants, and helpers
// are supplied by the surrounding kernel bindings.

#[cfg(CONFIG_KVM_XEN)]
extern "C" {
    pub static mut kvm_xen_enabled: static_key_false_deferred;

    pub fn __kvm_xen_has_interrupt(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_xen_inject_pending_events(vcpu: *mut kvm_vcpu);
    pub fn kvm_xen_inject_vcpu_vector(vcpu: *mut kvm_vcpu);
    pub fn kvm_xen_vcpu_set_attr(vcpu: *mut kvm_vcpu, data: *mut kvm_xen_vcpu_attr) -> i32;
    pub fn kvm_xen_vcpu_get_attr(vcpu: *mut kvm_vcpu, data: *mut kvm_xen_vcpu_attr) -> i32;
    pub fn kvm_xen_hvm_set_attr(kvm: *mut kvm, data: *mut kvm_xen_hvm_attr) -> i32;
    pub fn kvm_xen_hvm_get_attr(kvm: *mut kvm, data: *mut kvm_xen_hvm_attr) -> i32;
    pub fn kvm_xen_hvm_evtchn_send(kvm: *mut kvm, evt: *mut kvm_irq_routing_xen_evtchn) -> i32;
    pub fn kvm_xen_write_hypercall_page(vcpu: *mut kvm_vcpu, data: u64) -> i32;
    pub fn kvm_xen_hvm_config(kvm: *mut kvm, xhc: *mut kvm_xen_hvm_config) -> i32;
    pub fn kvm_xen_init_vm(kvm: *mut kvm);
    pub fn kvm_xen_destroy_vm(kvm: *mut kvm);
    pub fn kvm_xen_init_vcpu(vcpu: *mut kvm_vcpu);
    pub fn kvm_xen_destroy_vcpu(vcpu: *mut kvm_vcpu);
    pub fn kvm_xen_set_evtchn_fast(xe: *mut kvm_xen_evtchn, kvm: *mut kvm) -> i32;
    pub fn kvm_xen_setup_evtchn(kvm: *mut kvm, e: *mut kvm_kernel_irq_routing_entry,
                                ue: *const kvm_irq_routing_entry) -> i32;
    pub fn kvm_xen_inject_timer_irqs(vcpu: *mut kvm_vcpu);
}

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_sw_enable_lapic(vcpu: *mut kvm_vcpu) {
    if static_branch_unlikely(&(*kvm_xen_enabled).key) && (*vcpu).arch.xen.vcpu_info_cache.active
        && (*vcpu).arch.xen.upcall_vector != 0 && __kvm_xen_has_interrupt(vcpu) != 0 {
        kvm_xen_inject_vcpu_vector(vcpu);
    }
}

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_msr_enabled(kvm: *mut kvm) -> bool {
    static_branch_unlikely(&(*kvm_xen_enabled).key) && (*kvm).arch.xen.hvm_config.msr != 0
}

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_is_hypercall_page_msr(kvm: *mut kvm, msr: u32) -> bool {
    if !static_branch_unlikely(&(*kvm_xen_enabled).key) { return false; }
    msr != 0 && msr == (*kvm).arch.xen.hvm_config.msr
}

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_hypercall_enabled(kvm: *mut kvm) -> bool {
    static_branch_unlikely(&(*kvm_xen_enabled).key)
        && ((*kvm).arch.xen.hvm_config.flags & KVM_XEN_HVM_CONFIG_INTERCEPT_HCALL) != 0
}

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_has_interrupt(vcpu: *mut kvm_vcpu) -> i32 {
    if static_branch_unlikely(&(*kvm_xen_enabled).key) && (*vcpu).arch.xen.vcpu_info_cache.active
        && (*(*vcpu).kvm).arch.xen.upcall_vector != 0 { return __kvm_xen_has_interrupt(vcpu); }
    0
}

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_has_pending_events(vcpu: *mut kvm_vcpu) -> bool {
    static_branch_unlikely(&(*kvm_xen_enabled).key) && (*vcpu).arch.xen.evtchn_pending_sel != 0
}

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_timer_enabled(vcpu: *mut kvm_vcpu) -> bool { (*vcpu).arch.xen.timer_virq != 0 }

#[cfg(CONFIG_KVM_XEN)]
pub unsafe fn kvm_xen_has_pending_timer(vcpu: *mut kvm_vcpu) -> i32 {
    if kvm_xen_hypercall_enabled((*vcpu).kvm) && kvm_xen_timer_enabled(vcpu) {
        return atomic_read(&(*vcpu).arch.xen.timer_pending);
    }
    0
}

#[cfg(not(CONFIG_KVM_XEN))]
pub unsafe fn kvm_xen_write_hypercall_page(_: *mut kvm_vcpu, _: u64) -> i32 { 1 }
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_init_vm(_: *mut kvm) {}
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_destroy_vm(_: *mut kvm) {}
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_init_vcpu(_: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_destroy_vcpu(_: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_sw_enable_lapic(_: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_msr_enabled(_: *mut kvm) -> bool { false }
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_is_hypercall_page_msr(_: *mut kvm, _: u32) -> bool { false }
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_hypercall_enabled(_: *mut kvm) -> bool { false }
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_has_interrupt(_: *mut kvm_vcpu) -> i32 { 0 }
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_inject_pending_events(_: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_has_pending_events(_: *mut kvm_vcpu) -> bool { false }
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_has_pending_timer(_: *mut kvm_vcpu) -> i32 { 0 }
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_inject_timer_irqs(_: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_XEN))] pub unsafe fn kvm_xen_timer_enabled(_: *mut kvm_vcpu) -> bool { false }

extern "C" {
    pub fn kvm_xen_hypercall(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_xen_update_runstate(vcpu: *mut kvm_vcpu, state: i32);
}

pub unsafe fn kvm_xen_runstate_set_running(vcpu: *mut kvm_vcpu) {
    kvm_xen_update_runstate(vcpu, RUNSTATE_running);
}
pub unsafe fn kvm_xen_runstate_set_preempted(vcpu: *mut kvm_vcpu) {
    if WARN_ON_ONCE(!(*vcpu).preempted) { return; }
    kvm_xen_update_runstate(vcpu, RUNSTATE_runnable);
}

#[repr(C)]
pub struct compat_arch_vcpu_info { pub cr2: c_uint, pub pad: [c_uint; 5] }
#[repr(C)]
pub struct compat_vcpu_info {
    pub evtchn_upcall_pending: u8, pub evtchn_upcall_mask: u8, pub pad: u16,
    pub evtchn_pending_sel: u32, pub arch: compat_arch_vcpu_info,
    pub time: pvclock_vcpu_time_info,
}
#[repr(C)]
pub struct compat_arch_shared_info {
    pub max_pfn: c_uint, pub pfn_to_mfn_frame_list_list: c_uint, pub nmi_reason: c_uint,
    pub p2m_cr3: c_uint, pub p2m_vaddr: c_uint, pub p2m_generation: c_uint, pub wc_sec_hi: u32,
}
#[repr(C)]
pub struct compat_shared_info {
    pub vcpu_info: [compat_vcpu_info; MAX_VIRT_CPUS], pub evtchn_pending: [u32; 32],
    pub evtchn_mask: [u32; 32], pub wc: pvclock_wall_clock, pub arch: compat_arch_shared_info,
}
pub const COMPAT_EVTCHN_2L_NR_CHANNELS: usize = 8 * core::mem::size_of::<[u32; 32]>();

#[repr(C, packed)]
pub struct compat_vcpu_runstate_info { pub state: i32, pub state_entry_time: u64, pub time: [u64; 4] }
#[repr(C)]
pub struct compat_sched_poll { pub ports: u32, pub nr_ports: c_uint, pub timeout: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
