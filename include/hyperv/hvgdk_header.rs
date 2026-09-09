/* SPDX-License-Identifier: GPL-2.0 */
/* Type definitions for the Microsoft Hypervisor. */

// Dependencies supplied by the original headers: hvgdk_mini.h, hvgdk_ext.h.

pub const HV_LINUX_VENDOR_ID: u16 = 0x8100;

#[repr(C, packed)]
pub struct hv_enlightened_vmcs {
    pub revision_id: u32, pub abort: u32,
    pub host_es_selector: u16, pub host_cs_selector: u16, pub host_ss_selector: u16,
    pub host_ds_selector: u16, pub host_fs_selector: u16, pub host_gs_selector: u16,
    pub host_tr_selector: u16, pub padding16_1: u16,
    pub host_ia32_pat: u64, pub host_ia32_efer: u64,
    pub host_cr0: u64, pub host_cr3: u64, pub host_cr4: u64,
    pub host_ia32_sysenter_esp: u64, pub host_ia32_sysenter_eip: u64, pub host_rip: u64,
    pub host_ia32_sysenter_cs: u32,
    pub pin_based_vm_exec_control: u32, pub vm_exit_controls: u32, pub secondary_vm_exec_control: u32,
    pub io_bitmap_a: u64, pub io_bitmap_b: u64, pub msr_bitmap: u64,
    pub guest_es_selector: u16, pub guest_cs_selector: u16, pub guest_ss_selector: u16,
    pub guest_ds_selector: u16, pub guest_fs_selector: u16, pub guest_gs_selector: u16,
    pub guest_ldtr_selector: u16, pub guest_tr_selector: u16,
    pub guest_es_limit: u32, pub guest_cs_limit: u32, pub guest_ss_limit: u32,
    pub guest_ds_limit: u32, pub guest_fs_limit: u32, pub guest_gs_limit: u32,
    pub guest_ldtr_limit: u32, pub guest_tr_limit: u32, pub guest_gdtr_limit: u32, pub guest_idtr_limit: u32,
    pub guest_es_ar_bytes: u32, pub guest_cs_ar_bytes: u32, pub guest_ss_ar_bytes: u32,
    pub guest_ds_ar_bytes: u32, pub guest_fs_ar_bytes: u32, pub guest_gs_ar_bytes: u32,
    pub guest_ldtr_ar_bytes: u32, pub guest_tr_ar_bytes: u32,
    pub guest_es_base: u64, pub guest_cs_base: u64, pub guest_ss_base: u64, pub guest_ds_base: u64,
    pub guest_fs_base: u64, pub guest_gs_base: u64, pub guest_ldtr_base: u64, pub guest_tr_base: u64,
    pub guest_gdtr_base: u64, pub guest_idtr_base: u64, pub padding64_1: [u64; 3],
    pub vm_exit_msr_store_addr: u64, pub vm_exit_msr_load_addr: u64, pub vm_entry_msr_load_addr: u64,
    pub cr3_target_value0: u64, pub cr3_target_value1: u64, pub cr3_target_value2: u64, pub cr3_target_value3: u64,
    pub page_fault_error_code_mask: u32, pub page_fault_error_code_match: u32,
    pub cr3_target_count: u32, pub vm_exit_msr_store_count: u32, pub vm_exit_msr_load_count: u32, pub vm_entry_msr_load_count: u32,
    pub tsc_offset: u64, pub virtual_apic_page_addr: u64, pub vmcs_link_pointer: u64,
    pub guest_ia32_debugctl: u64, pub guest_ia32_pat: u64, pub guest_ia32_efer: u64,
    pub guest_pdptr0: u64, pub guest_pdptr1: u64, pub guest_pdptr2: u64, pub guest_pdptr3: u64,
    pub guest_pending_dbg_exceptions: u64, pub guest_sysenter_esp: u64, pub guest_sysenter_eip: u64,
    pub guest_activity_state: u32, pub guest_sysenter_cs: u32,
    pub cr0_guest_host_mask: u64, pub cr4_guest_host_mask: u64, pub cr0_read_shadow: u64, pub cr4_read_shadow: u64,
    pub guest_cr0: u64, pub guest_cr3: u64, pub guest_cr4: u64, pub guest_dr7: u64,
    pub host_fs_base: u64, pub host_gs_base: u64, pub host_tr_base: u64, pub host_gdtr_base: u64,
    pub host_idtr_base: u64, pub host_rsp: u64, pub ept_pointer: u64,
    pub virtual_processor_id: u16, pub padding16_2: [u16; 3], pub padding64_2: [u64; 5], pub guest_physical_address: u64,
    pub vm_instruction_error: u32, pub vm_exit_reason: u32, pub vm_exit_intr_info: u32, pub vm_exit_intr_error_code: u32,
    pub idt_vectoring_info_field: u32, pub idt_vectoring_error_code: u32, pub vm_exit_instruction_len: u32, pub vmx_instruction_info: u32,
    pub exit_qualification: u64, pub exit_io_instruction_ecx: u64, pub exit_io_instruction_esi: u64,
    pub exit_io_instruction_edi: u64, pub exit_io_instruction_eip: u64,
    pub guest_linear_address: u64, pub guest_rsp: u64, pub guest_rflags: u64,
    pub guest_interruptibility_info: u32, pub cpu_based_vm_exec_control: u32, pub exception_bitmap: u32,
    pub vm_entry_controls: u32, pub vm_entry_intr_info_field: u32, pub vm_entry_exception_error_code: u32,
    pub vm_entry_instruction_len: u32, pub tpr_threshold: u32, pub guest_rip: u64,
    pub hv_clean_fields: u32, pub padding32_1: u32, pub hv_synthetic_controls: u32,
    // C bitfields: nested_flush_hypercall:1, msr_bitmap:1, reserved:30.
    pub hv_enlightenments_control: u32,
    pub hv_vp_id: u32, pub padding32_2: u32, pub hv_vm_id: u64, pub partition_assist_page: u64,
    pub padding64_4: [u64; 4], pub guest_bndcfgs: u64, pub guest_ia32_perf_global_ctrl: u64,
    pub guest_ia32_s_cet: u64, pub guest_ssp: u64, pub guest_ia32_int_ssp_table_addr: u64,
    pub guest_ia32_lbr_ctl: u64, pub padding64_5: [u64; 2], pub xss_exit_bitmap: u64,
    pub encls_exiting_bitmap: u64, pub host_ia32_perf_global_ctrl: u64, pub tsc_multiplier: u64,
    pub host_ia32_s_cet: u64, pub host_ssp: u64, pub host_ia32_int_ssp_table_addr: u64, pub padding64_6: u64,
}

pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_NONE: u32 = 0;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_IO_BITMAP: u32 = 1 << 0;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_MSR_BITMAP: u32 = 1 << 1;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CONTROL_GRP2: u32 = 1 << 2;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CONTROL_GRP1: u32 = 1 << 3;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CONTROL_PROC: u32 = 1 << 4;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CONTROL_EVENT: u32 = 1 << 5;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CONTROL_ENTRY: u32 = 1 << 6;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CONTROL_EXCPN: u32 = 1 << 7;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CRDR: u32 = 1 << 8;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_CONTROL_XLAT: u32 = 1 << 9;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_GUEST_BASIC: u32 = 1 << 10;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_GUEST_GRP1: u32 = 1 << 11;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_GUEST_GRP2: u32 = 1 << 12;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_HOST_POINTER: u32 = 1 << 13;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_HOST_GRP1: u32 = 1 << 14;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_ENLIGHTENMENTSCONTROL: u32 = 1 << 15;
pub const HV_VMX_ENLIGHTENED_CLEAN_FIELD_ALL: u32 = 0xffff;
pub const HV_VMX_SYNTHETIC_EXIT_REASON_TRAP_AFTER_FLUSH: u32 = 0x10000031;

#[repr(C, packed)]
pub struct hv_vmcb_enlightenments {
    // C bitfields: nested_flush_hypercall:1, msr_bitmap:1, enlightened_npt_tlb:1, reserved:29.
    pub hv_enlightenments_control: u32,
    pub hv_vp_id: u32, pub hv_vm_id: u64, pub partition_assist_page: u64, pub reserved: u64,
}

pub const HV_VMCB_NESTED_ENLIGHTENMENTS: u32 = 31;
pub const HV_SVM_EXITCODE_ENL: u64 = 0xf0000000u64;
pub const HV_SVM_ENL_EXITCODE_TRAP_AFTER_FLUSH: u32 = 1;

#[repr(C)]
pub struct hv_partition_assist_pg { pub tlb_lock_count: u32 }

#[repr(C)]
pub union hv_connection_id {
    pub asu32: u32,
    // C bitfields: id:24, reserved:8.
    pub u: hv_connection_id_bits,
}

#[repr(C, packed)]
pub struct hv_connection_id_bits { pub id_reserved: u32 }

#[repr(C, packed)]
pub struct hv_input_unmap_gpa_pages {
    pub target_partition_id: u64, pub target_gpa_base: u64, pub unmap_flags: u32, pub padding: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
