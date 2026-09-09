/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from vmx.h; declarations from included headers are external dependencies. */

// CONFIG_X86_64: MAX_NR_USER_RETURN_MSRS = 7; otherwise 4.
pub const MAX_NR_USER_RETURN_MSRS: usize = 7;
pub const MAX_NR_LOADSTORE_MSRS: usize = 8;

#[repr(C)] pub struct vmx_msrs { pub nr: ::core::ffi::c_uint, pub val: [vmx_msr_entry; MAX_NR_LOADSTORE_MSRS] }
#[repr(C)] pub struct vmx_uret_msr { pub load_into_hardware: bool, pub data: u64, pub mask: u64 }

#[repr(C)] pub enum segment_cache_field { SEG_FIELD_SEL=0, SEG_FIELD_BASE=1, SEG_FIELD_LIMIT=2, SEG_FIELD_AR=3, SEG_FIELD_NR=4 }
pub const RTIT_ADDR_RANGE: usize = 4;
#[repr(C)] pub struct pt_ctx { pub ctl:u64, pub status:u64, pub output_base:u64, pub output_mask:u64, pub cr3_match:u64, pub addr_a:[u64;4], pub addr_b:[u64;4] }
#[repr(C)] pub struct pt_desc { pub ctl_bitmask:u64, pub num_address_ranges:u32, pub caps:[u32; PT_CPUID_REGS_NUM * PT_CPUID_LEAVES], pub host:pt_ctx, pub guest:pt_ctx }

#[repr(C)] pub struct nested_vmx {
    pub vmxon:bool, pub vmxon_ptr:gpa_t, pub pml_full:bool, pub current_vmptr:gpa_t,
    pub cached_vmcs12:*mut vmcs12, pub cached_shadow_vmcs12:*mut vmcs12,
    pub shadow_vmcs12_cache:gfn_to_hva_cache, pub vmcs12_cache:gfn_to_hva_cache,
    pub need_vmcs12_to_shadow_sync:bool, pub dirty_vmcs12:bool, pub force_msr_bitmap_recalc:bool,
    pub need_sync_vmcs02_to_vmcs12_rare:bool, pub vmcs02_initialized:bool, pub enlightened_vmcs_enabled:bool,
    pub mtf_pending:bool, pub vmcs02:loaded_vmcs, pub apic_access_page_map:kvm_host_map,
    pub virtual_apic_map:kvm_host_map, pub pi_desc_map:kvm_host_map, pub pi_desc:*mut pi_desc,
    pub pi_pending:bool, pub posted_intr_nv:u16, pub preemption_timer:hrtimer,
    pub preemption_timer_deadline:u64, pub has_preemption_timer_deadline:bool, pub preemption_timer_expired:bool,
    pub pre_vmenter_cr3:ulong, pub pre_vmenter_debugctl:u64, pub pre_vmenter_bndcfgs:u64,
    pub pre_vmenter_s_cet:u64, pub pre_vmenter_ssp:u64, pub pre_vmenter_ssp_tbl:u64,
    pub vpid02:u16, pub last_vpid:u16, pub tsc_autostore_slot:i32, pub msrs:nested_vmx_msrs,
    pub smm: nested_vmx_smm,
}
#[repr(C)] pub struct nested_vmx_smm { pub vmxon:bool, pub guest_mode:bool }

#[repr(C)] pub struct vcpu_vmx {
    pub vcpu:kvm_vcpu, pub vt:vcpu_vt, pub fail:u8, pub x2apic_msr_bitmap_mode:u8,
    pub idt_vectoring_info:u32, pub rflags:ulong, pub guest_uret_msrs:[vmx_uret_msr;MAX_NR_USER_RETURN_MSRS],
    pub guest_uret_msrs_loaded:bool, pub msr_guest_kernel_gs_base:u64, pub spec_ctrl:u64,
    pub msr_ia32_umwait_control:u32, pub vmcs01:loaded_vmcs, pub loaded_vmcs:*mut loaded_vmcs,
    pub msr_autoload:msr_autoload, pub msr_autostore:vmx_msrs, pub rmode:rmode_state,
    pub segment_cache:segment_cache, pub vpid:i32, pub nested:nested_vmx, pub ple_window:u32,
    pub ple_window_dirty:bool, pub pml_pg:*mut page, pub hv_deadline_tsc:u64,
    pub msr_ia32_feature_control:u64, pub msr_ia32_feature_control_valid_bits:u64,
    pub msr_ia32_sgxlepubkeyhash:[u64;4], pub msr_ia32_mcu_opt_ctrl:u64, pub disable_fb_clear:bool,
    pub pt_desc:pt_desc, pub lbr_desc:lbr_desc, pub ve_info:*mut vmx_ve_information,
}
#[repr(C)] pub struct msr_autoload { pub guest:vmx_msrs, pub host:vmx_msrs }
#[repr(C)] pub struct rmode_state { pub vm86_active:i32, pub save_rflags:ulong, pub segs:[kvm_segment;8] }
#[repr(C)] pub struct kvm_save_segment { pub selector:u16, pub base:ulong, pub limit:u32, pub ar:u32 }
#[repr(C)] pub struct segment_cache { pub bitmask:u32, pub seg:[kvm_save_segment;8] }
#[repr(C)] pub struct kvm_vmx { pub kvm:kvm, pub tss_addr:u32, pub ept_identity_pagetable_done:bool, pub ept_identity_map_addr:gpa_t, pub pid_table:*mut u64 }

extern "C" {
    pub fn vmx_vcpu_load_vmcs(vcpu:*mut kvm_vcpu,cpu:i32); pub fn allocate_vpid()->i32; pub fn free_vpid(vpid:i32);
    pub fn vmx_set_constant_host_state(vmx:*mut vcpu_vmx); pub fn vmx_prepare_switch_to_guest(vcpu:*mut kvm_vcpu);
    pub fn vmx_set_host_fs_gs(host:*mut vmcs_host_state,fs_sel:u16,gs_sel:u16,fs_base:ulong,gs_base:ulong);
    pub fn vmx_get_cpl(vcpu:*mut kvm_vcpu)->i32; pub fn vmx_get_cpl_no_cache(vcpu:*mut kvm_vcpu)->i32;
    pub fn vmx_emulation_required(vcpu:*mut kvm_vcpu)->bool; pub fn vmx_get_rflags(vcpu:*mut kvm_vcpu)->ulong; pub fn vmx_set_rflags(vcpu:*mut kvm_vcpu,rflags:ulong);
    pub fn vmx_get_interrupt_shadow(vcpu:*mut kvm_vcpu)->u32; pub fn vmx_set_interrupt_shadow(vcpu:*mut kvm_vcpu,mask:i32); pub fn vmx_set_efer(vcpu:*mut kvm_vcpu,efer:u64)->i32;
    pub fn vmx_set_cr0(vcpu:*mut kvm_vcpu,cr0:ulong); pub fn vmx_set_cr4(vcpu:*mut kvm_vcpu,cr4:ulong); pub fn set_cr4_guest_host_mask(vmx:*mut vcpu_vmx);
    pub fn ept_save_pdptrs(vcpu:*mut kvm_vcpu); pub fn vmx_get_segment(vcpu:*mut kvm_vcpu,var:*mut kvm_segment,seg:i32); pub fn __vmx_set_segment(vcpu:*mut kvm_vcpu,var:*mut kvm_segment,seg:i32);
    pub fn vmx_guest_inject_ac(vcpu:*mut kvm_vcpu)->bool; pub fn vmx_update_exception_bitmap(vcpu:*mut kvm_vcpu); pub fn vmx_nmi_blocked(vcpu:*mut kvm_vcpu)->bool; pub fn __vmx_interrupt_blocked(vcpu:*mut kvm_vcpu)->bool; pub fn vmx_interrupt_blocked(vcpu:*mut kvm_vcpu)->bool;
    pub fn vmx_get_nmi_mask(vcpu:*mut kvm_vcpu)->bool; pub fn vmx_set_nmi_mask(vcpu:*mut kvm_vcpu,masked:bool); pub fn vmx_set_virtual_apic_mode(vcpu:*mut kvm_vcpu);
    pub fn vmx_find_uret_msr(vmx:*mut vcpu_vmx,msr:u32)->*mut vmx_uret_msr; pub fn pt_update_intercept_for_msr(vcpu:*mut kvm_vcpu); pub fn vmx_update_host_rsp(vmx:*mut vcpu_vmx,host_rsp:ulong);
    pub fn __vmx_vcpu_enter_flags(vmx:*mut vcpu_vmx)->u32; pub fn __vmx_vcpu_run(vmx:*mut vcpu_vmx,flags:u32)->bool; pub fn vmx_ept_load_pdptrs(vcpu:*mut kvm_vcpu);
    pub fn vmx_set_intercept_for_msr(vcpu:*mut kvm_vcpu,msr:u32,ty:i32,set:bool); pub fn vmx_get_l2_tsc_offset(vcpu:*mut kvm_vcpu)->u64; pub fn vmx_get_l2_tsc_multiplier(vcpu:*mut kvm_vcpu)->u64;
    pub fn vmx_get_untagged_addr(vcpu:*mut kvm_vcpu,gva:gva_t,flags:u32)->gva_t; pub fn vmx_update_cpu_dirty_logging(vcpu:*mut kvm_vcpu);
    pub fn vmx_get_supported_debugctl(vcpu:*mut kvm_vcpu,host_initiated:bool)->u64; pub fn vmx_is_valid_debugctl(vcpu:*mut kvm_vcpu,data:u64,host_initiated:bool)->bool;
}

pub const VMX_HOST_OWNED_DEBUGCTL_BITS:u64 = DEBUGCTLMSR_FREEZE_IN_SMM;
pub const PML_LOG_NR_ENTRIES:usize=512; pub const PML_HEAD_INDEX:usize=PML_LOG_NR_ENTRIES-1;
pub const KVM_REQUIRED_VMX_SECONDARY_VM_EXEC_CONTROL:u64=0; pub const KVM_REQUIRED_VMX_TERTIARY_VM_EXEC_CONTROL:u64=0;
pub const KVM_REQUIRED_VMX_PIN_BASED_VM_EXEC_CONTROL:u32=PIN_BASED_EXT_INTR_MASK|PIN_BASED_NMI_EXITING;
pub const KVM_REQUIRED_VMX_CPU_BASED_VM_EXEC_CONTROL:u32=CPU_BASED_HLT_EXITING|CPU_BASED_CR3_LOAD_EXITING|CPU_BASED_CR3_STORE_EXITING|CPU_BASED_UNCOND_IO_EXITING|CPU_BASED_MOV_DR_EXITING|CPU_BASED_USE_TSC_OFFSETTING|CPU_BASED_MWAIT_EXITING|CPU_BASED_MONITOR_EXITING|CPU_BASED_INVLPG_EXITING|CPU_BASED_RDPMC_EXITING|CPU_BASED_INTR_WINDOW_EXITING;

pub unsafe fn vmx_disable_intercept_for_msr(v:*mut kvm_vcpu,m:u32,t:i32){vmx_set_intercept_for_msr(v,m,t,false)}
pub unsafe fn vmx_enable_intercept_for_msr(v:*mut kvm_vcpu,m:u32,t:i32){vmx_set_intercept_for_msr(v,m,t,true)}
pub unsafe fn vmx_get_rvi()->u8 {(vmcs_read16(GUEST_INTR_STATUS)&0xff) as u8}
pub unsafe fn vmx_l1_guest_owned_cr0_bits()->ulong { let mut bits=KVM_POSSIBLE_CR0_GUEST_BITS; if !enable_ept {bits &= !X86_CR0_WP;} bits }
pub unsafe fn vmx_get_instr_info_reg(x:u32)->i32 {((x>>3)&0xf) as i32}
pub unsafe fn vmx_get_instr_info_reg2(x:u32)->i32 {((x>>28)&0xf) as i32}

pub unsafe fn vmx_guest_debugctl_write(vcpu:*mut kvm_vcpu,val:u64){
    WARN_ON_ONCE(val & VMX_HOST_OWNED_DEBUGCTL_BITS);
    let val=val | (*vcpu).arch.host_debugctl & VMX_HOST_OWNED_DEBUGCTL_BITS;
    vmcs_write64(GUEST_IA32_DEBUGCTL,val);
}
pub unsafe fn vmx_guest_debugctl_read()->u64 { vmcs_read64(GUEST_IA32_DEBUGCTL) & !VMX_HOST_OWNED_DEBUGCTL_BITS }
pub unsafe fn vmx_reload_guest_debugctl(vcpu:*mut kvm_vcpu){
    let val=vmcs_read64(GUEST_IA32_DEBUGCTL);
    if !((val ^ (*vcpu).arch.host_debugctl) & VMX_HOST_OWNED_DEBUGCTL_BITS != 0) { return; }
    vmx_guest_debugctl_write(vcpu,val & !VMX_HOST_OWNED_DEBUGCTL_BITS);
}

pub unsafe fn to_vt(vcpu:*mut kvm_vcpu)->*mut vcpu_vt { &mut (*(vcpu as *mut vcpu_vmx)).vt }
pub unsafe fn vt_to_vcpu(vt:*mut vcpu_vt)->*mut kvm_vcpu { &mut (*(vt as *mut vcpu_vmx)).vcpu }
pub unsafe fn vmx_get_exit_reason(vcpu:*mut kvm_vcpu)->vmx_exit_reason { (*to_vt(vcpu)).exit_reason }
pub unsafe fn vmx_get_exit_qual(vcpu:*mut kvm_vcpu)->ulong {
    let vt=to_vt(vcpu); if !kvm_register_test_and_mark_available(vcpu,VCPU_REG_EXIT_INFO_1) && !WARN_ON_ONCE(is_td_vcpu(vcpu)){(*vt).exit_qualification=vmcs_readl(EXIT_QUALIFICATION);} (*vt).exit_qualification
}
pub unsafe fn vmx_get_intr_info(vcpu:*mut kvm_vcpu)->u32 {
    let vt=to_vt(vcpu); if !kvm_register_test_and_mark_available(vcpu,VCPU_REG_EXIT_INFO_2) && !WARN_ON_ONCE(is_td_vcpu(vcpu)){(*vt).exit_intr_info=vmcs_read32(VM_EXIT_INTR_INFO);} (*vt).exit_intr_info
}
pub unsafe fn to_kvm_vmx(kvm:*mut kvm)->*mut kvm_vmx { kvm as *mut kvm_vmx }
pub unsafe fn to_vmx(vcpu:*mut kvm_vcpu)->*mut vcpu_vmx { vcpu as *mut vcpu_vmx }
pub unsafe fn vmx_has_waitpkg(vmx:*mut vcpu_vmx)->bool { secondary_exec_controls_get(vmx) & SECONDARY_EXEC_ENABLE_USR_WAIT_PAUSE != 0 }
pub unsafe fn vmx_need_pf_intercept(vcpu:*mut kvm_vcpu)->bool { if !enable_ept {return true;} allow_smaller_maxphyaddr && cpuid_maxphyaddr(vcpu)<kvm_host.maxphyaddr }
pub unsafe fn is_unrestricted_guest(vcpu:*mut kvm_vcpu)->bool { enable_unrestricted_guest && (!is_guest_mode(vcpu) || secondary_exec_controls_get(to_vmx(vcpu)) & SECONDARY_EXEC_UNRESTRICTED_GUEST != 0) }
pub unsafe fn vmx_guest_state_valid(vcpu:*mut kvm_vcpu)->bool { is_unrestricted_guest(vcpu) || __vmx_guest_state_valid(vcpu) }

extern "C" {
 pub fn intel_pmu_cross_mapped_check(pmu:*mut kvm_pmu); pub fn intel_pmu_create_guest_lbr_event(vcpu:*mut kvm_vcpu)->i32; pub fn vmx_passthrough_lbr_msrs(vcpu:*mut kvm_vcpu);
 pub fn alloc_vmcs_cpu(shadow:bool,cpu:i32,flags:gfp_t)->*mut vmcs; pub fn free_vmcs(v:*mut vmcs); pub fn alloc_loaded_vmcs(v:*mut loaded_vmcs)->i32; pub fn free_loaded_vmcs(v:*mut loaded_vmcs);
 pub fn __vmx_guest_state_valid(vcpu:*mut kvm_vcpu)->bool; pub fn dump_vmcs(vcpu:*mut kvm_vcpu);
}
pub unsafe fn alloc_vmcs(shadow:bool)->*mut vmcs { alloc_vmcs_cpu(shadow,raw_smp_processor_id(),GFP_KERNEL_ACCOUNT) }

// The remaining C macros are represented by their source-level control expressions;
// exact included-header types and configuration symbols are supplied by dependencies.
pub const VMX_REGS_LAZY_LOAD_SET:u64 = BIT(VCPU_REGS_RSP)|BIT(VCPU_REG_RIP)|BIT(VCPU_REG_RFLAGS)|BIT(VCPU_REG_PDPTR)|BIT(VCPU_REG_SEGMENTS)|BIT(VCPU_REG_CR0)|BIT(VCPU_REG_CR3)|BIT(VCPU_REG_CR4)|BIT(VCPU_REG_EXIT_INFO_1)|BIT(VCPU_REG_EXIT_INFO_2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
