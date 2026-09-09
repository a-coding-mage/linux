// SPDX-License-Identifier: GPL-2.0-only
// Kernel-based Virtual Machine driver for Linux: AMD SVM support.
// C header translation; included dependencies and external symbols are supplied elsewhere.

pub const IOPM_SIZE: usize = PAGE_SIZE * 3;
pub const MSRPM_SIZE: usize = PAGE_SIZE * 2;

extern "C" {
    static mut gmet_enabled: bool; static mut npt_enabled: bool; static mut nrips: i32;
    static mut vgif: i32; static mut intercept_smi: bool; static mut vnmi: bool; static mut lbrv: i32;
    static mut tsc_aux_uret_slot: i32;
    static mut svm_x86_ops: kvm_x86_ops;
}

#[inline] pub unsafe fn vmcb_set_gpat(v:*mut vmcb,data:u64){(*v).save.g_pat=data;vmcb_mark_dirty(v,VMCB_NPT as c_int)}
#[inline] pub unsafe fn svm_vmgexit_set_return_code(s:*mut vcpu_svm,response:u64,data:u64){ghcb_set_sw_exit_info_1((*s).sev_es.ghcb,response);ghcb_set_sw_exit_info_2((*s).sev_es.ghcb,data)}
#[inline] pub unsafe fn svm_vmgexit_inject_exception(s:*mut vcpu_svm,vector:u8){svm_vmgexit_set_return_code(s,GHCB_HV_RESP_ISSUE_EXCEPTION,SVM_EVTINJ_VALID|SVM_EVTINJ_TYPE_EXEPT|vector as u64)}
#[inline] pub unsafe fn svm_vmgexit_bad_input(s:*mut vcpu_svm,suberror:u64){svm_vmgexit_set_return_code(s,GHCB_HV_RESP_MALFORMED_INPUT,suberror)}
#[inline] pub unsafe fn svm_vmgexit_success(s:*mut vcpu_svm,data:u64){svm_vmgexit_set_return_code(s,GHCB_HV_RESP_NO_ACTION,data)}
#[inline] pub unsafe fn svm_vmgexit_no_action(s:*mut vcpu_svm,data:u64){svm_vmgexit_success(s,data)}

extern "C" {
    fn nested_svm_init_msrpm_merge_offsets()->c_int; fn svm_copy_vmrun_state(*mut vmcb_save_area,*mut vmcb_save_area); fn svm_copy_vmloadsave_state(*mut vmcb,*mut vmcb); fn nested_svm_exit_handled(*mut vcpu_svm)->c_int; fn nested_svm_check_permissions(*mut kvm_vcpu)->c_int; fn nested_svm_check_cached_vmcb12(*mut kvm_vcpu)->c_int; fn nested_svm_check_exception(*mut vcpu_svm,c_uint,bool,u32)->c_int; fn nested_svm_exit_special(*mut vcpu_svm)->c_int; fn nested_svm_update_tsc_ratio_msr(*mut kvm_vcpu); fn svm_write_tsc_multiplier(*mut kvm_vcpu); fn svm_switch_vmcb(*mut vcpu_svm,*mut kvm_vmcb_info);
    fn avic_incomplete_ipi_interception(*mut kvm_vcpu)->c_int; fn avic_unaccelerated_access_interception(*mut kvm_vcpu)->c_int; fn avic_apicv_post_state_restore(*mut kvm_vcpu); fn avic_refresh_apicv_exec_ctrl(*mut kvm_vcpu); fn avic_vcpu_blocking(*mut kvm_vcpu); fn avic_vcpu_unblocking(*mut kvm_vcpu); fn avic_refresh_virtual_apic_mode(*mut kvm_vcpu);
    fn sev_vcpu_create(*mut kvm_vcpu)->c_int; fn sev_free_vcpu(*mut kvm_vcpu); fn sev_vm_init(*mut kvm); fn sev_vm_destroy(*mut kvm); fn sev_set_cpu_caps(); fn sev_hardware_setup(); fn sev_hardware_unsetup(); fn sev_cpu_init(*mut svm_cpu_data)->c_int; fn sev_dev_get_attr(u32,u64,*mut u64)->c_int; fn sev_handle_rmp_fault(*mut kvm_vcpu,gpa_t,u64); fn sev_snp_reload_vmsa(*mut kvm_vcpu);
}

#[inline] pub unsafe fn __sme_page_pa(page: *mut page) -> c_ulong { __sme_set(page_to_pfn(page) << PAGE_SHIFT) }
#[inline] pub unsafe fn __sme_pa_to_page(pa: c_ulong) -> *mut page { pfn_to_page(__sme_clr(pa) >> PAGE_SHIFT) }

pub const VMCB_INTERCEPTS: u32 = 0; pub const VMCB_PERM_MAP: u32 = 1; pub const VMCB_ASID: u32 = 2;
pub const VMCB_INTR: u32 = 3; pub const VMCB_NPT: u32 = 4; pub const VMCB_CR: u32 = 5;
pub const VMCB_DR: u32 = 6; pub const VMCB_DT: u32 = 7; pub const VMCB_SEG: u32 = 8;
pub const VMCB_CR2: u32 = 9; pub const VMCB_LBR: u32 = 10; pub const VMCB_AVIC: u32 = 11;
pub const VMCB_CET: u32 = 12; pub const VMCB_SW: u32 = 31;
pub const VMCB_ALL_CLEAN_MASK: u32 = (1<<VMCB_INTERCEPTS)|(1<<VMCB_PERM_MAP)|(1<<VMCB_ASID)|(1<<VMCB_INTR)|(1<<VMCB_NPT)|(1<<VMCB_CR)|(1<<VMCB_DR)|(1<<VMCB_DT)|(1<<VMCB_SEG)|(1<<VMCB_CR2)|(1<<VMCB_LBR)|(1<<VMCB_AVIC)|(1<<VMCB_CET)|(1<<VMCB_SW);
pub const VMCB_ALWAYS_DIRTY_MASK: u32 = (1<<VMCB_INTR)|(1<<VMCB_CR2);

#[cfg(CONFIG_KVM_AMD_SEV)]
#[repr(C)] pub struct kvm_sev_info { pub active: bool, pub es_active: bool, pub need_init: bool, pub asid: c_uint, pub handle: c_uint, pub fd: c_int, pub policy: c_ulong, pub pages_locked: c_ulong, pub regions_list: list_head, pub ap_jump_table: u64, pub vmsa_features: u64, pub ghcb_version: u16, pub enc_context_owner: *mut kvm, pub mirror_vms: list_head, pub mirror_entry: list_head, pub misc_cg: *mut misc_cg, pub migration_in_progress: atomic_t, pub snp_context: *mut c_void, pub guest_req_buf: *mut c_void, pub guest_resp_buf: *mut c_void, pub guest_req_mutex: mutex, pub have_run_cpus: cpumask_var_t, pub snp_certs_enabled: bool }

#[repr(C)] pub struct kvm_svm { pub kvm: kvm, pub avic_vm_id: u32, pub avic_logical_id_table: *mut u32, pub avic_physical_id_table: *mut u64, pub hnode: hlist_node, #[cfg(CONFIG_KVM_AMD_SEV)] pub sev_info: kvm_sev_info }
pub struct kvm_vcpu;
#[repr(C)] pub struct kvm_vmcb_info { pub ptr: *mut vmcb, pub pa: c_ulong, pub cpu: c_int, pub asid_generation: u64 }
#[repr(C)] pub struct vmcb_save_area_cached { pub es: vmcb_seg, pub cs: vmcb_seg, pub ss: vmcb_seg, pub ds: vmcb_seg, pub gdtr: vmcb_seg, pub idtr: vmcb_seg, pub cpl: u8, pub efer:u64, pub cr4:u64, pub cr3:u64, pub cr0:u64, pub dr7:u64, pub dr6:u64, pub rflags:u64, pub rip:u64, pub rsp:u64, pub s_cet:u64, pub ssp:u64, pub isst_addr:u64, pub rax:u64, pub cr2:u64, pub g_pat:u64, pub dbgctl:u64, pub br_from:u64, pub br_to:u64, pub last_excp_from:u64, pub last_excp_to:u64 }
#[repr(C)] pub struct vmcb_ctrl_area_cached { pub intercepts: [u32; MAX_INTERCEPT], pub pause_filter_thresh:u16, pub pause_filter_count:u16, pub iopm_base_pa:u64, pub msrpm_base_pa:u64, pub tsc_offset:u64, pub asid:u32, pub tlb_ctl:u8, pub erap_ctl:u8, pub int_ctl:u32, pub int_vector:u32, pub int_state:u32, pub exit_code:u64, pub exit_info_1:u64, pub exit_info_2:u64, pub exit_int_info:u32, pub exit_int_info_err:u32, pub misc_ctl:u64, pub event_inj:u32, pub event_inj_err:u32, pub next_rip:u64, pub nested_cr3:u64, pub misc_ctl2:u64, pub clean:u32, pub reserved_sw:[u8;32] }
#[repr(C)] pub struct svm_nested_state { pub vmcb02:kvm_vmcb_info, pub hsave_msr:u64, pub vm_cr_msr:u64, pub vmcb12_gpa:u64, pub last_vmcb12_gpa:u64, pub last_bus_lock_rip:u64, pub msrpm:*mut c_void, pub ctl:vmcb_ctrl_area_cached, pub save:vmcb_save_area_cached, pub initialized:bool, pub force_msr_bitmap_recalc:bool }
#[repr(C)] pub struct vcpu_sev_es_state { pub vmsa:*mut sev_es_save_area, pub ghcb:*mut ghcb, pub valid_bitmap:[u8;16], pub ghcb_map:kvm_host_map, pub received_first_sipi:bool, pub ap_reset_hold_type:c_uint, pub sw_scratch:u64, pub ghcb_sa:*mut c_void, pub ghcb_sa_len:u32, pub ghcb_sa_sync:bool, pub ghcb_sa_free:bool, pub psc: psc_state, pub ghcb_registered_gpa:u64, pub snp_vmsa_mutex:mutex, pub snp_pending_vmsa_gpa:gpa_t, pub snp_guest_vmsa_gpa:gpa_t, pub snp_ap_waiting_for_reset:bool, pub snp_has_guest_vmsa:bool }
#[repr(C)] pub struct psc_state { pub cur_idx:u16, pub end_idx:u16, pub batch_size:u16, pub is_2m:bool }
#[repr(C)] pub struct vcpu_svm { pub vcpu:kvm_vcpu, pub vmcb:*mut vmcb, pub vmcb01:kvm_vmcb_info, pub current_vmcb:*mut kvm_vmcb_info, pub asid:u32, pub sysenter_esp_hi:u32, pub sysenter_eip_hi:u32, pub tsc_aux:u64, pub msr_decfg:u64, pub next_rip:u64, pub spec_ctrl:u64, pub tsc_ratio_msr:u64, pub virt_spec_ctrl:u64, pub msrpm:*mut c_void, pub nmi_iret_rip: c_ulong, pub nested:svm_nested_state, pub nmi_masked:bool, pub awaiting_iret_completion:bool, pub nmi_singlestep:bool, pub nmi_singlestep_guest_rflags:u64, pub nmi_l1_to_l2:bool, pub soft_int_csbase:c_ulong, pub soft_int_old_rip:c_ulong, pub soft_int_next_rip:c_ulong, pub soft_int_injected:bool, pub ldr_reg:u32, pub dfr_reg:u32, pub avic_physical_id_entry:u64, pub ir_list:list_head, pub ir_list_lock:raw_spinlock_t, pub sev_es:vcpu_sev_es_state, pub guest_state_loaded:bool, pub avic_irq_window:bool, pub x2avic_msrs_intercepted:bool, pub lbr_msrs_intercepted:bool, pub guest_gif:bool }
#[repr(C)] pub struct svm_cpu_data { pub asid_generation:u64, pub max_asid:u32, pub next_asid:u32, pub min_asid:u32, pub bp_spec_reduce_set:bool, pub save_area:*mut vmcb, pub save_area_pa:c_ulong, pub sev_vmcbs:*mut *mut vmcb }

extern "C" { static mut svm_data: svm_cpu_data; }

#[inline] pub unsafe fn to_kvm_svm(kvm:*mut kvm)->*mut kvm_svm { container_of(kvm) }
#[inline] pub unsafe fn to_svm(vcpu:*mut kvm_vcpu)->*mut vcpu_svm { container_of(vcpu) }
#[inline] pub unsafe fn ghcb_gpa_is_registered(svm:*mut vcpu_svm,val:u64)->bool { (*svm).sev_es.ghcb_registered_gpa==val }
#[inline] pub unsafe fn vmcb_mark_all_dirty(v:*mut vmcb){(*v).control.clean=0}
#[inline] pub unsafe fn vmcb_mark_all_clean(v:*mut vmcb){(*v).control.clean=VMCB_ALL_CLEAN_MASK & !VMCB_ALWAYS_DIRTY_MASK}
#[inline] pub unsafe fn vmcb_mark_dirty(v:*mut vmcb,bit:c_int){(*v).control.clean &= !(1u32<<bit)}
pub const SVM_REGS_LAZY_LOAD_SET: u32 = BIT(VCPU_REG_PDPTR);
pub const SVM_MSRPM_BYTES_PER_RANGE: u32=2048; pub const SVM_BITS_PER_MSR:u32=2; pub const SVM_MSRS_PER_BYTE:u32=BITS_PER_BYTE/SVM_BITS_PER_MSR; pub const SVM_MSRS_PER_RANGE:u32=SVM_MSRPM_BYTES_PER_RANGE*SVM_MSRS_PER_BYTE; pub const SVM_MSRPM_OFFSET_MASK:u32=SVM_MSRS_PER_RANGE-1;
#[inline] pub fn svm_msrpm_bit_nr(msr:u32)->c_int { match msr & !SVM_MSRPM_OFFSET_MASK {0=>((msr&SVM_MSRPM_OFFSET_MASK)*SVM_BITS_PER_MSR) as c_int,0xc0000000=>((SVM_MSRS_PER_RANGE+((msr&SVM_MSRPM_OFFSET_MASK)))*SVM_BITS_PER_MSR) as c_int,0xc0010000=>((2*SVM_MSRS_PER_RANGE+(msr&SVM_MSRPM_OFFSET_MASK))*SVM_BITS_PER_MSR) as c_int,_=>-EINVAL} }

pub const DEBUGCTL_RESERVED_BITS:u64 = !DEBUGCTLMSR_LBR;
pub const NESTED_EXIT_HOST:c_int=0; pub const NESTED_EXIT_DONE:c_int=1; pub const NESTED_EXIT_CONTINUE:c_int=2;
pub const AVIC_REQUIRED_APICV_INHIBITS:u32 = BIT(APICV_INHIBIT_REASON_DISABLED)|BIT(APICV_INHIBIT_REASON_ABSENT)|BIT(APICV_INHIBIT_REASON_HYPERV)|BIT(APICV_INHIBIT_REASON_NESTED)|BIT(APICV_INHIBIT_REASON_IRQWIN)|BIT(APICV_INHIBIT_REASON_PIT_REINJ)|BIT(APICV_INHIBIT_REASON_BLOCKIRQ)|BIT(APICV_INHIBIT_REASON_SEV)|BIT(APICV_INHIBIT_REASON_PHYSICAL_ID_ALIASED)|BIT(APICV_INHIBIT_REASON_APIC_ID_MODIFIED)|BIT(APICV_INHIBIT_REASON_APIC_BASE_MODIFIED)|BIT(APICV_INHIBIT_REASON_LOGICAL_ID_ALIASED)|BIT(APICV_INHIBIT_REASON_PHYSICAL_ID_TOO_BIG);

extern "C" {
    fn nested_vmcb02_recalc_intercepts(svm:*mut vcpu_svm); fn svm_alloc_permissions_map(size:c_ulong,gfp_mask:gfp_t)->*mut c_void; fn svm_vcpu_free_msrpm(msrpm:*mut c_void);
    fn svm_set_intercept_for_msr(vcpu:*mut kvm_vcpu,msr:u32,ty:c_int,set:bool); fn nested_svm_vmexit(svm:*mut vcpu_svm);
    fn __svm_sev_es_vcpu_run(svm:*mut vcpu_svm,flags:c_uint,hostsa:*mut sev_es_save_area); fn __svm_vcpu_run(svm:*mut vcpu_svm,flags:c_uint);
}
#[inline] pub unsafe fn svm_vcpu_alloc_msrpm()->*mut c_void { svm_alloc_permissions_map(MSRPM_SIZE as c_ulong,GFP_KERNEL_ACCOUNT) }
#[inline] pub unsafe fn svm_disable_intercept_for_msr(v:*mut kvm_vcpu,m:u32,t:c_int){svm_set_intercept_for_msr(v,m,t,false)}
#[inline] pub unsafe fn svm_enable_intercept_for_msr(v:*mut kvm_vcpu,m:u32,t:c_int){svm_set_intercept_for_msr(v,m,t,true)}

// Remaining file-scope declarations retain the original external interfaces.
extern "C" {
    fn svm_set_efer(*mut kvm_vcpu,u64)->c_int; fn svm_set_cr0(*mut kvm_vcpu,c_ulong); fn svm_set_cr4(*mut kvm_vcpu,c_ulong); fn disable_nmi_singlestep(*mut vcpu_svm); fn svm_smi_blocked(*mut kvm_vcpu)->bool; fn svm_nmi_blocked(*mut kvm_vcpu)->bool; fn svm_interrupt_blocked(*mut kvm_vcpu)->bool; fn svm_set_gif(*mut vcpu_svm,bool); fn svm_invoke_exit_handler(*mut kvm_vcpu,u64)->c_int; fn svm_complete_interrupt_delivery(*mut kvm_vcpu,c_int,c_int,c_int); fn svm_skip_emulated_instruction(*mut kvm_vcpu)->c_int;
    fn avic_hardware_setup()->bool; fn avic_hardware_unsetup(); fn avic_vcpu_precreate(*mut kvm)->c_int; fn avic_vm_pre_destroy(*mut kvm); fn avic_vm_destroy(*mut kvm); fn avic_init_vmcb(*mut vcpu_svm,*mut vmcb); fn avic_init_vcpu(*mut vcpu_svm)->c_int; fn avic_vcpu_load(*mut kvm_vcpu,c_int); fn avic_vcpu_put(*mut kvm_vcpu); fn avic_ring_doorbell(*mut kvm_vcpu);
    fn pre_sev_run(*mut vcpu_svm,c_int)->c_int; fn sev_init_vmcb(*mut vcpu_svm,bool); fn sev_vcpu_after_set_cpuid(*mut vcpu_svm); fn sev_es_string_io(*mut vcpu_svm,c_int,c_uint,c_int)->c_int; fn sev_es_recalc_msr_intercepts(*mut kvm_vcpu); fn sev_vcpu_deliver_sipi_vector(*mut kvm_vcpu,u8); fn sev_es_unmap_ghcb(*mut vcpu_svm);
    fn enter_svm_guest_mode(*mut kvm_vcpu,u64,bool)->c_int; fn svm_leave_nested(*mut kvm_vcpu); fn svm_free_nested(*mut vcpu_svm); fn svm_allocate_nested(*mut vcpu_svm)->c_int; fn nested_svm_vmrun(*mut kvm_vcpu)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
