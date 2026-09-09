/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of x86/include/asm/kvm_host.h. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* C headers are dependencies supplied by the surrounding kernel translation. */

pub const KVM_MAX_VCPUS: usize = 1024;
pub const KVM_VCPU_ID_RATIO: usize = 4;
pub const KVM_MAX_VCPU_IDS: usize = KVM_MAX_VCPUS * KVM_VCPU_ID_RATIO;
pub const KVM_INTERNAL_MEM_SLOTS: usize = 3;
pub const KVM_HALT_POLL_NS_DEFAULT: u64 = 200000;
pub const KVM_NR_DB_REGS: usize = 4;
pub const KVM_MAX_CPUID_ENTRIES: usize = 256;
pub const KVM_NR_VAR_MTRR: usize = 8;
pub const ASYNC_PF_PER_VCPU: usize = 64;
pub const PT64_ROOT_MAX_LEVEL: usize = 5;
pub const KVM_MMU_NUM_PREV_ROOTS: usize = 3;
pub const KVM_NESTED_RUN_PENDING: u8 = 1;
pub const KVM_NESTED_RUN_PENDING_UNTRUSTED: u8 = 2;
pub const HF_GUEST_MASK: u32 = 1 << 0;

pub const fn bit(n: u32) -> u64 { 1u64 << n }
pub const fn kvm_hpage_gfn_shift(x: u64) -> u64 { (x - 1) * 9 }
pub const fn kvm_hpage_shift(x: u64, page_shift: u64) -> u64 { page_shift + kvm_hpage_gfn_shift(x) }
pub const fn kvm_max<const A: usize, const B: usize>() -> usize { if A >= B { A } else { B } }

pub type hpa_t = u64; pub type gpa_t = u64; pub type gfn_t = u64; pub type gva_t = u64;
pub type kvm_pfn_t = u64; pub type cpumask_var_t = *mut core::ffi::c_void;
pub type atomic_t = i32; pub type atomic64_t = i64; pub type atomic_long_t = isize;

#[repr(C)] pub struct kvm_vcpu;
#[repr(C)] pub struct kvm;
#[repr(C)] pub struct kvm_mmu_page;
#[repr(C)] pub struct kvm_page_fault;
#[repr(C)] pub struct x86_exception;
#[repr(C)] pub struct x86_emulate_ctxt;
#[repr(C)] pub struct perf_event;
#[repr(C)] pub struct kvm_lapic;
#[repr(C)] pub struct kvm_x86_msr_filter;
#[repr(C)] pub struct kvm_x86_pmu_event_filter;
#[repr(C)] pub struct kvm_kernel_irqfd;
#[repr(C)] pub struct kvm_page_track_notifier_head;
#[repr(C)] pub struct kvm_async_pf;
#[repr(C)] pub struct kvm_segment;
#[repr(C)] pub struct desc_ptr;
#[repr(C)] pub struct kvm_enc_region;
#[repr(C)] pub struct kvm_gfn_range;
#[repr(C)] pub struct kvm_nested_state;
#[repr(C)] pub struct kvm_mmu_memory_cache;
#[repr(C)] pub struct fpu_guest;
#[repr(C)] pub struct gfn_to_pfn_cache;
#[repr(C)] pub struct gfn_to_hva_cache;

#[repr(C)] pub union kvm_mmu_page_role { pub word: u32, pub bits: kvm_mmu_page_role_bits }
#[repr(C)] pub struct kvm_mmu_page_role_bits {
    pub level: u32, pub has_4_byte_gpte: u32, pub quadrant: u32, pub direct: u32,
    pub access: u32, pub invalid: u32, pub efer_nx: u32, pub cr0_wp: u32,
    pub smap_andnot_wp: u32, pub ad_disabled: u32, pub guest_mode: u32,
    pub passthrough: u32, pub is_mirror: u32, pub cr4_smep: u32, pub reserved: u32,
    pub smm: u8,
}
#[repr(C)] pub union kvm_mmu_extended_role { pub word: u32, pub bits: kvm_mmu_extended_role_bits }
#[repr(C)] pub struct kvm_mmu_extended_role_bits { pub valid:u32, pub execonly:u32, pub cr4_pse:u32, pub cr4_pke:u32, pub cr4_smap:u32, pub cr4_la57:u32, pub efer_lma:u32, pub has_pferr_fetch:u32 }
#[repr(C)] pub union kvm_cpu_role { pub as_u64: u64, pub parts: kvm_cpu_role_parts }
#[repr(C)] pub struct kvm_cpu_role_parts { pub base:kvm_mmu_page_role, pub ext:kvm_mmu_extended_role }

#[repr(C)] pub struct kvm_caps { pub has_tsc_control:bool, pub max_guest_tsc_khz:u32, pub tsc_scaling_ratio_frac_bits:u8, pub max_tsc_scaling_ratio:u64, pub default_tsc_scaling_ratio:u64, pub has_bus_lock_exit:bool, pub has_notify_vmexit:bool, pub supported_vm_types:u32, pub supported_mce_cap:u64, pub supported_xcr0:u64, pub supported_xss:u64, pub supported_perf_cap:u64, pub supported_efer_bits:u64, pub supported_quirks:u64, pub inapplicable_quirks:u64 }
extern "C" { pub static mut kvm_caps: kvm_caps; }
#[repr(C)] pub struct kvm_host_values { pub maxphyaddr:u8, pub efer:u64, pub xcr0:u64, pub xss:u64, pub s_cet:u64, pub arch_capabilities:u64 }
extern "C" { pub static mut kvm_host: kvm_host_values; }

#[repr(C)] pub struct kvm_rmap_head { pub val: atomic_long_t }
#[repr(C)] pub struct kvm_pio_request { pub count:usize, pub in_:i32, pub port:i32, pub size:i32 }
#[repr(C)] pub struct kvm_page_format { pub rsvd_bits_mask:[[u64;PT64_ROOT_MAX_LEVEL];2], pub bad_mt_xwr:u64, pub pkru_mask:u32, pub permissions:[u16;16] }
#[repr(C)] pub struct kvm_mmu_root_info { pub pgd:gpa_t, pub hpa:hpa_t }
#[repr(C)] pub struct kvm_pagewalk { pub get_guest_pgd:Option<unsafe extern "C" fn(*mut kvm_vcpu)->usize>, pub get_pdptr:Option<unsafe extern "C" fn(*mut kvm_vcpu,i32)->u64>, pub inject_page_fault:Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut x86_exception,bool)>, pub gva_to_gpa:Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_pagewalk,gpa_t,u64,*mut x86_exception)->gpa_t>, pub cpu_role:kvm_cpu_role, pub fmt:kvm_page_format }
#[repr(C)] pub struct kvm_mmu { pub page_fault:Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_page_fault)->i32>, pub sync_spte:Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_mmu_page,i32)->i32>, pub w:*mut kvm_pagewalk, pub root:kvm_mmu_root_info, pub mirror_root_hpa:hpa_t, pub root_role:kvm_mmu_page_role, pub prev_roots:[kvm_mmu_root_info;KVM_MMU_NUM_PREV_ROOTS], pub pae_root:*mut u64, pub pml4_root:*mut u64, pub pml5_root:*mut u64, pub fmt:kvm_page_format }

#[repr(C)] pub struct kvm_pmc { pub type_:u32, pub idx:u8, pub is_paused:bool, pub intr:bool, pub counter:u64, pub emulated_counter:u64, pub eventsel:u64, pub eventsel_hw:u64, pub perf_event:*mut perf_event, pub vcpu:*mut kvm_vcpu, pub current_config:u64 }
pub const KVM_MAX_NR_GP_COUNTERS:usize=8; pub const KVM_MAX_NR_FIXED_COUNTERS:usize=3;
#[repr(C)] pub struct kvm_pmu { pub version:u8, pub nr_arch_gp_counters:u32, pub nr_arch_fixed_counters:u32, pub available_event_types:u32, pub fixed_ctr_ctrl:u64, pub fixed_ctr_ctrl_hw:u64, pub fixed_ctr_ctrl_rsvd:u64, pub global_ctrl:u64, pub global_status:u64, pub counter_bitmask:[u64;2], pub global_ctrl_rsvd:u64, pub global_status_rsvd:u64, pub reserved_bits:u64, pub raw_event_mask:u64, pub gp_counters:[kvm_pmc;KVM_MAX_NR_GP_COUNTERS], pub fixed_counters:[kvm_pmc;KVM_MAX_NR_FIXED_COUNTERS], pub reprogram_pmi:u64, pub all_valid_pmc_idx:u64, pub pmc_in_use:u64, pub pmc_counting_instructions:u64, pub pmc_counting_branches:u64, pub pmc_has_mode_specific_enables:u64, pub ds_area:u64, pub pebs_enable:u64, pub pebs_enable_rsvd:u64, pub pebs_data_cfg:u64, pub pebs_data_cfg_rsvd:u64, pub host_cross_mapped_mask:u64, pub need_cleanup:bool, pub event_count:u8 }

#[repr(C)] pub struct kvm_mtrr { pub var:[u64;KVM_NR_VAR_MTRR*2], pub fixed_64k:u64, pub fixed_16k:[u64;2], pub fixed_4k:[u64;8], pub deftype:u64 }
#[repr(C)] pub struct kvm_queued_exception { pub pending:bool, pub injected:bool, pub has_error_code:bool, pub vector:u8, pub error_code:u32, pub payload:usize, pub has_payload:bool }
#[repr(C)] pub struct kvm_msr_data { pub host_initiated:bool, pub index:u32, pub data:u64 }
#[repr(C)] pub struct kvm_lapic_irq { pub vector:u32, pub delivery_mode:u16, pub dest_mode:u16, pub level:bool, pub trig_mode:u16, pub shorthand:u32, pub dest_id:u32, pub msi_redir_hint:bool }
#[repr(C)] pub struct kvm_arch_memory_slot { pub rmap:[*mut kvm_rmap_head;4], pub lpage_info:[*mut core::ffi::c_void;3], pub gfn_write_track:*mut u16 }
#[repr(C)] pub struct kvm_arch { pub n_requested_mmu_pages:usize, pub n_max_mmu_pages:usize, pub indirect_shadow_pages:u32, pub mmu_valid_gen:u8, pub vm_type:u8, pub has_private_mem:bool, pub has_protected_state:bool, pub has_protected_eoi:bool, pub has_protected_pmu:bool, pub pre_fault_allowed:bool, pub shadow_mmio_value:u64, pub wall_clock:gpa_t, pub kvmclock_offset:i64, pub default_tsc_khz:u32, pub user_set_tsc:bool, pub backwards_tsc_observed:bool, pub bsp_vcpu_id:u32, pub irqchip_mode:u32, pub max_vcpu_ids:u32, pub disable_nx_huge_pages:bool, pub gfn_direct_bits:gfn_t, pub cpu_dirty_log_size:i32 }
#[repr(C)] pub struct kvm_arch_async_pf { pub token:u32, pub gfn:gfn_t, pub cr3:usize, pub direct_map:bool, pub error_code:u64 }

extern "C" {
    pub static mut allow_smaller_maxphyaddr:bool; pub static mut enable_apicv:bool; pub static mut enable_ipiv:bool; pub static mut enable_device_posted_irqs:bool;
    pub static mut kvm_x86_ops:kvm_x86_ops; pub static mut kvm_nested_ops:kvm_x86_nested_ops;
    pub fn kvm_arch_free_vm(kvm:*mut kvm); pub fn kvm_arch_async_page_not_present(vcpu:*mut kvm_vcpu, work:*mut kvm_async_pf)->bool; pub fn kvm_arch_async_page_present(vcpu:*mut kvm_vcpu, work:*mut kvm_async_pf); pub fn kvm_arch_async_page_ready(vcpu:*mut kvm_vcpu, work:*mut kvm_async_pf); pub fn kvm_arch_async_page_present_queued(vcpu:*mut kvm_vcpu); pub fn kvm_arch_can_dequeue_async_page_present(vcpu:*mut kvm_vcpu)->bool;
}
#[repr(C)] pub struct kvm_x86_ops { pub name:*const core::ffi::c_char, pub check_processor_compatibility:Option<unsafe extern "C" fn()->i32>, pub enable_virtualization_cpu:Option<unsafe extern "C" fn()->i32>, pub disable_virtualization_cpu:Option<unsafe extern "C" fn()>, pub vm_size:usize, pub vm_init:Option<unsafe extern "C" fn(*mut kvm)->i32>, pub vm_destroy:Option<unsafe extern "C" fn(*mut kvm)>, pub vcpu_create:Option<unsafe extern "C" fn(*mut kvm_vcpu)->i32>, pub vcpu_free:Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub vcpu_reset:Option<unsafe extern "C" fn(*mut kvm_vcpu,bool)>, pub vcpu_run:Option<unsafe extern "C" fn(*mut kvm_vcpu,u64)->i32>, pub handle_exit:Option<unsafe extern "C" fn(*mut kvm_vcpu,i32)->i32> }
#[repr(C)] pub struct kvm_x86_nested_ops { pub enabled:bool, pub leave_nested:Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub check_events:Option<unsafe extern "C" fn(*mut kvm_vcpu)->i32>, pub has_events:Option<unsafe extern "C" fn(*mut kvm_vcpu,bool)->bool>, pub triple_fault:Option<unsafe extern "C" fn(*mut kvm_vcpu)> }
#[repr(C)] pub struct kvm_x86_init_ops { pub hardware_setup:Option<unsafe extern "C" fn()->i32>, pub handle_intel_pt_intr:Option<unsafe extern "C" fn()->u32>, pub runtime_ops:*mut kvm_x86_ops, pub nested_ops:*mut kvm_x86_nested_ops }

pub const KVM_HANDLING_IRQ:u32=1; pub const KVM_HANDLING_NMI:u32=2;
pub const KVM_RUN_FORCE_IMMEDIATE_EXIT:u32=1; pub const KVM_RUN_LOAD_GUEST_DR6:u32=2; pub const KVM_RUN_LOAD_DEBUGCTL:u32=4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
