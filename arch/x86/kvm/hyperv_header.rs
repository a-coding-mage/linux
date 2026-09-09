/* SPDX-License-Identifier: GPL-2.0-only */
/* KVM Microsoft Hyper-V emulation; translated from hyperv.h. */

/* External kernel types, constants, and helpers are supplied by the surrounding translation unit. */

#[cfg(CONFIG_KVM_HYPERV)]
#[repr(C)]
pub struct kvm_vcpu_hv_stimer {
    pub timer: hrtimer,
    pub index: i32,
    pub config: hv_stimer_config,
    pub count: u64,
    pub exp_time: u64,
    pub msg: hv_message,
    pub msg_pending: bool,
}

#[cfg(CONFIG_KVM_HYPERV)]
#[repr(C)]
pub struct kvm_vcpu_hv_synic {
    pub version: u64,
    pub control: u64,
    pub msg_page: u64,
    pub evt_page: u64,
    pub sint: [atomic64_t; HV_SYNIC_SINT_COUNT as usize],
    pub sint_to_gsi: [atomic_t; HV_SYNIC_SINT_COUNT as usize],
    pub auto_eoi_bitmap: [usize; 256 / (usize::BITS as usize)],
    pub vec_bitmap: [usize; 256 / (usize::BITS as usize)],
    pub active: bool,
    pub dont_zero_synic_pages: bool,
}

pub const KVM_HV_TLB_FLUSH_FIFO_SIZE: usize = 16;
pub const KVM_HV_TLB_FLUSHALL_ENTRY: u64 = u64::MAX;

#[repr(u32)]
pub enum hv_tlb_flush_fifos {
    HV_L1_TLB_FLUSH_FIFO,
    HV_L2_TLB_FLUSH_FIFO,
    HV_NR_TLB_FLUSH_FIFOS,
}

#[cfg(CONFIG_KVM_HYPERV)]
#[repr(C)]
pub struct kvm_vcpu_hv_tlb_flush_fifo {
    pub write_lock: spinlock_t,
    pub entries: kfifo,
}

#[cfg(CONFIG_KVM_HYPERV)]
#[repr(C)]
pub struct kvm_vcpu_hv {
    pub vcpu: *mut kvm_vcpu,
    pub vp_index: u32,
    pub hv_vapic: u64,
    pub runtime_offset: i64,
    pub synic: kvm_vcpu_hv_synic,
    pub exit: kvm_hyperv_exit,
    pub stimer: [kvm_vcpu_hv_stimer; HV_SYNIC_STIMER_COUNT as usize],
    pub stimer_pending_bitmap: [usize; (HV_SYNIC_STIMER_COUNT as usize + usize::BITS as usize - 1) / usize::BITS as usize],
    pub enforce_cpuid: bool,
    pub cpuid_cache: kvm_vcpu_hv_cpuid_cache,
    pub tlb_flush_fifo: [kvm_vcpu_hv_tlb_flush_fifo; HV_NR_TLB_FLUSH_FIFOS as usize],
    pub sparse_banks: [u64; HV_MAX_SPARSE_VCPU_BANKS as usize],
    pub vcpu_mask: [usize; (KVM_MAX_VCPUS as usize + usize::BITS as usize - 1) / usize::BITS as usize],
    pub vp_assist_page: hv_vp_assist_page,
    pub nested: kvm_vcpu_hv_nested,
}

#[repr(C)]
pub struct kvm_vcpu_hv_cpuid_cache {
    pub features_eax: u32,
    pub features_ebx: u32,
    pub features_edx: u32,
    pub enlightenments_eax: u32,
    pub enlightenments_ebx: u32,
    pub syndbg_cap_eax: u32,
    pub nested_eax: u32,
    pub nested_ebx: u32,
}

#[repr(C)]
pub struct kvm_vcpu_hv_nested { pub pa_page_gpa: u64, pub vm_id: u64, pub vp_id: u32 }

pub const HYPERV_CPUID_SIGNATURE_EAX: u32 = 0x3123_7648;
pub const HYPERV_CPUID_SYNDBG_VENDOR_AND_MAX_FUNCTIONS: u32 = 0x4000_0080;
pub const HYPERV_CPUID_SYNDBG_INTERFACE: u32 = 0x4000_0081;
pub const HYPERV_CPUID_SYNDBG_PLATFORM_CAPABILITIES: u32 = 0x4000_0082;
pub const HV_X64_SYNDBG_CAP_ALLOW_KERNEL_DEBUGGING: u32 = 1 << 1;
pub const HV_X64_MSR_SYNDBG_CONTROL: u32 = 0x4000_00f1;
pub const HV_X64_MSR_SYNDBG_STATUS: u32 = 0x4000_00f2;
pub const HV_X64_MSR_SYNDBG_SEND_BUFFER: u32 = 0x4000_00f3;
pub const HV_X64_MSR_SYNDBG_RECV_BUFFER: u32 = 0x4000_00f4;
pub const HV_X64_MSR_SYNDBG_PENDING_BUFFER: u32 = 0x4000_00f5;
pub const HV_X64_MSR_SYNDBG_OPTIONS: u32 = 0x4000_00ff;
pub const HV_X64_SYNDBG_OPTION_USE_HCALLS: u32 = 1 << 2;

#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn to_kvm_hv(kvm: *mut kvm) -> *mut kvm_hv { &mut (*kvm).arch.hyperv }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn to_hv_vcpu_safe(vcpu: *mut kvm_vcpu) -> *mut kvm_vcpu_hv { smp_load_acquire(&(*vcpu).arch.hyperv) }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn to_hv_vcpu(vcpu: *mut kvm_vcpu) -> *mut kvm_vcpu_hv { (*vcpu).arch.hyperv }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn to_hv_synic(vcpu: *mut kvm_vcpu) -> *mut kvm_vcpu_hv_synic { &mut (*to_hv_vcpu(vcpu)).synic }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn hv_synic_to_vcpu(synic: *mut kvm_vcpu_hv_synic) -> *mut kvm_vcpu { container_of!(synic, kvm_vcpu_hv, synic).vcpu }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn to_hv_syndbg(vcpu: *mut kvm_vcpu) -> *mut kvm_hv_syndbg { &mut (*(*vcpu).kvm).arch.hyperv.hv_syndbg }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_get_vpindex(vcpu: *mut kvm_vcpu) -> u32 { let hv = to_hv_vcpu_safe(vcpu); if !hv.is_null() { (*hv).vp_index } else { (*vcpu).vcpu_idx } }

extern "C" {
    pub fn kvm_hv_set_msr_common(vcpu: *mut kvm_vcpu, msr: u32, data: u64, host: bool) -> i32;
    pub fn kvm_hv_get_msr_common(vcpu: *mut kvm_vcpu, msr: u32, pdata: *mut u64, host: bool) -> i32;
    pub fn kvm_hv_hypercall(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_hv_irq_routing_update(kvm: *mut kvm);
    pub fn kvm_hv_synic_set_irq(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, irq_source_id: i32, level: i32, line_status: bool) -> i32;
    pub fn kvm_hv_synic_send_eoi(vcpu: *mut kvm_vcpu, vector: i32);
    pub fn kvm_hv_activate_synic(vcpu: *mut kvm_vcpu, dont_zero_synic_pages: bool) -> i32;
    pub fn kvm_hv_vcpu_uninit(vcpu: *mut kvm_vcpu);
    pub fn kvm_hv_assist_page_enabled(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_hv_get_assist_page(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_hv_process_stimers(vcpu: *mut kvm_vcpu);
    pub fn kvm_hv_setup_tsc_page(kvm: *mut kvm, hv_clock: *mut pvclock_vcpu_time_info);
    pub fn kvm_hv_request_tsc_page_update(kvm: *mut kvm);
    pub fn kvm_hv_xsaves_xsavec_maybe_warn(vcpu: *mut kvm_vcpu);
    pub fn kvm_hv_init_vm(kvm: *mut kvm);
    pub fn kvm_hv_destroy_vm(kvm: *mut kvm);
    pub fn kvm_hv_vcpu_init(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_hv_set_cpuid(vcpu: *mut kvm_vcpu, hyperv_enabled: bool);
    pub fn kvm_hv_set_enforce_cpuid(vcpu: *mut kvm_vcpu, enforce: bool) -> i32;
    pub fn kvm_vm_ioctl_hv_eventfd(kvm: *mut kvm, args: *mut kvm_hyperv_eventfd) -> i32;
    pub fn kvm_get_hv_cpuid(vcpu: *mut kvm_vcpu, cpuid: *mut kvm_cpuid2, entries: *mut kvm_cpuid_entry2) -> i32;
    pub fn kvm_hv_vcpu_flush_tlb(vcpu: *mut kvm_vcpu) -> i32;
}

#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn to_hv_stimer(vcpu: *mut kvm_vcpu, timer_index: usize) -> *mut kvm_vcpu_hv_stimer { &mut (*to_hv_vcpu(vcpu)).stimer[timer_index] }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn hv_stimer_to_vcpu(stimer: *mut kvm_vcpu_hv_stimer) -> *mut kvm_vcpu {
    container_of!((stimer as *mut u8).offset(-((*stimer).index as isize * core::mem::size_of::<kvm_vcpu_hv_stimer>() as isize)), kvm_vcpu_hv, stimer).vcpu
}
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_has_stimer_pending(vcpu: *mut kvm_vcpu) -> bool {
    let hv = to_hv_vcpu_safe(vcpu);
    !hv.is_null() && !bitmap_empty!((*hv).stimer_pending_bitmap, HV_SYNIC_STIMER_COUNT)
}
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_synic_has_vector(vcpu: *mut kvm_vcpu, vector: usize) -> bool { !to_hv_vcpu(vcpu).is_null() && test_bit!(vector, (*to_hv_synic(vcpu)).vec_bitmap) }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_synic_auto_eoi_set(vcpu: *mut kvm_vcpu, vector: usize) -> bool { !to_hv_vcpu(vcpu).is_null() && test_bit!(vector, (*to_hv_synic(vcpu)).auto_eoi_bitmap) }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_hypercall_enabled(vcpu: *mut kvm_vcpu) -> bool { (*vcpu).arch.hyperv_enabled && (*to_kvm_hv((*vcpu).kvm)).hv_guest_os_id != 0 }
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_invtsc_suppressed(vcpu: *mut kvm_vcpu) -> bool {
    let hv = to_hv_vcpu(vcpu);
    !hv.is_null() && ((*hv).cpuid_cache.features_eax & HV_ACCESS_TSC_INVARIANT) != 0 && ((*to_kvm_hv((*vcpu).kvm)).hv_invtsc_control & HV_EXPOSE_INVARIANT_TSC) == 0
}
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_verify_vp_assist(vcpu: *mut kvm_vcpu) -> i32 {
    if to_hv_vcpu(vcpu).is_null() || !kvm_hv_assist_page_enabled(vcpu) { 0 } else { kvm_hv_get_assist_page(vcpu) }
}
#[cfg(CONFIG_KVM_HYPERV)]
pub unsafe fn kvm_hv_nested_transtion_tlb_flush(vcpu: *mut kvm_vcpu, tdp_enabled: bool) { if !to_hv_vcpu(vcpu).is_null() && tdp_enabled { kvm_make_request!(KVM_REQ_HV_TLB_FLUSH, vcpu); } }

#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_setup_tsc_page(_kvm: *mut kvm, _hv_clock: *mut pvclock_vcpu_time_info) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_request_tsc_page_update(_kvm: *mut kvm) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_xsaves_xsavec_maybe_warn(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_init_vm(_kvm: *mut kvm) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_destroy_vm(_kvm: *mut kvm) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_vcpu_init(_vcpu: *mut kvm_vcpu) -> i32 { 0 }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_vcpu_uninit(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_hypercall_enabled(_vcpu: *mut kvm_vcpu) -> bool { false }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_hypercall(_vcpu: *mut kvm_vcpu) -> i32 { HV_STATUS_ACCESS_DENIED }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_vcpu_purge_flush_tlb(_vcpu: *mut kvm_vcpu) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_synic_has_vector(_vcpu: *mut kvm_vcpu, _vector: i32) -> bool { false }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_synic_auto_eoi_set(_vcpu: *mut kvm_vcpu, _vector: i32) -> bool { false }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_synic_send_eoi(_vcpu: *mut kvm_vcpu, _vector: i32) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_invtsc_suppressed(_vcpu: *mut kvm_vcpu) -> bool { false }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_set_cpuid(_vcpu: *mut kvm_vcpu, _hyperv_enabled: bool) {}
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_has_stimer_pending(_vcpu: *mut kvm_vcpu) -> bool { false }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_verify_vp_assist(_vcpu: *mut kvm_vcpu) -> i32 { 0 }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_get_vpindex(vcpu: *mut kvm_vcpu) -> u32 { (*vcpu).vcpu_idx }
#[cfg(not(CONFIG_KVM_HYPERV))]
pub unsafe fn kvm_hv_nested_transtion_tlb_flush(_vcpu: *mut kvm_vcpu, _tdp_enabled: bool) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
