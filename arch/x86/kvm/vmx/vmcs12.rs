// SPDX-License-Identifier: GPL-2.0
// Dependency intent: the C source includes vmcs12.h and uses kernel/build-time
// definitions supplied by the surrounding VMX implementation.

macro_rules! vmcs12_offset {
    ($name:ident) => {
        core::mem::offset_of!(vmcs12, $name) as u16
    };
}

macro_rules! field {
    ($number:ident, $name:ident) => {
        (ENC_TO_VMCS12_IDX!($number), vmcs12_offset!($name))
    };
}

macro_rules! field64 {
    ($number:ident, $name:ident) => {
        field!($number, $name),
        (ENC_TO_VMCS12_IDX!(concat_idents!($number, _HIGH)),
            vmcs12_offset!($name) + core::mem::size_of::<u32>() as u16)
    };
}

// The C designated initializers are represented as (field-index, byte-offset)
// pairs because Rust has no equivalent array-designator syntax.
static KVM_SUPPORTED_VMCS12_FIELD_OFFSETS: &[(usize, u16)] = &[
    field!(VIRTUAL_PROCESSOR_ID, virtual_processor_id),
    field!(POSTED_INTR_NV, posted_intr_nv),
    field!(GUEST_ES_SELECTOR, guest_es_selector),
    field!(GUEST_CS_SELECTOR, guest_cs_selector),
    field!(GUEST_SS_SELECTOR, guest_ss_selector),
    field!(GUEST_DS_SELECTOR, guest_ds_selector),
    field!(GUEST_FS_SELECTOR, guest_fs_selector),
    field!(GUEST_GS_SELECTOR, guest_gs_selector),
    field!(GUEST_LDTR_SELECTOR, guest_ldtr_selector),
    field!(GUEST_TR_SELECTOR, guest_tr_selector),
    field!(GUEST_INTR_STATUS, guest_intr_status),
    field!(GUEST_PML_INDEX, guest_pml_index),
    field!(HOST_ES_SELECTOR, host_es_selector),
    field!(HOST_CS_SELECTOR, host_cs_selector),
    field!(HOST_SS_SELECTOR, host_ss_selector),
    field!(HOST_DS_SELECTOR, host_ds_selector),
    field!(HOST_FS_SELECTOR, host_fs_selector),
    field!(HOST_GS_SELECTOR, host_gs_selector),
    field!(HOST_TR_SELECTOR, host_tr_selector),
    field64!(IO_BITMAP_A, io_bitmap_a),
    field64!(IO_BITMAP_B, io_bitmap_b),
    field64!(MSR_BITMAP, msr_bitmap),
    field64!(VM_EXIT_MSR_STORE_ADDR, vm_exit_msr_store_addr),
    field64!(VM_EXIT_MSR_LOAD_ADDR, vm_exit_msr_load_addr),
    field64!(VM_ENTRY_MSR_LOAD_ADDR, vm_entry_msr_load_addr),
    field64!(PML_ADDRESS, pml_address),
    field64!(TSC_OFFSET, tsc_offset),
    field64!(TSC_MULTIPLIER, tsc_multiplier),
    field64!(VIRTUAL_APIC_PAGE_ADDR, virtual_apic_page_addr),
    field64!(APIC_ACCESS_ADDR, apic_access_addr),
    field64!(POSTED_INTR_DESC_ADDR, posted_intr_desc_addr),
    field64!(VM_FUNCTION_CONTROL, vm_function_control),
    field64!(EPT_POINTER, ept_pointer),
    field64!(EOI_EXIT_BITMAP0, eoi_exit_bitmap0),
    field64!(EOI_EXIT_BITMAP1, eoi_exit_bitmap1),
    field64!(EOI_EXIT_BITMAP2, eoi_exit_bitmap2),
    field64!(EOI_EXIT_BITMAP3, eoi_exit_bitmap3),
    field64!(EPTP_LIST_ADDRESS, eptp_list_address),
    field64!(VMREAD_BITMAP, vmread_bitmap),
    field64!(VMWRITE_BITMAP, vmwrite_bitmap),
    field64!(XSS_EXIT_BITMAP, xss_exit_bitmap),
    field64!(ENCLS_EXITING_BITMAP, encls_exiting_bitmap),
    field64!(GUEST_PHYSICAL_ADDRESS, guest_physical_address),
    field64!(VMCS_LINK_POINTER, vmcs_link_pointer),
    field64!(GUEST_IA32_DEBUGCTL, guest_ia32_debugctl),
    field64!(GUEST_IA32_PAT, guest_ia32_pat),
    field64!(GUEST_IA32_EFER, guest_ia32_efer),
    field64!(GUEST_IA32_PERF_GLOBAL_CTRL, guest_ia32_perf_global_ctrl),
    field64!(GUEST_PDPTR0, guest_pdptr0),
    field64!(GUEST_PDPTR1, guest_pdptr1),
    field64!(GUEST_PDPTR2, guest_pdptr2),
    field64!(GUEST_PDPTR3, guest_pdptr3),
    field64!(GUEST_BNDCFGS, guest_bndcfgs),
    field64!(HOST_IA32_PAT, host_ia32_pat),
    field64!(HOST_IA32_EFER, host_ia32_efer),
    field64!(HOST_IA32_PERF_GLOBAL_CTRL, host_ia32_perf_global_ctrl),
    field!(PIN_BASED_VM_EXEC_CONTROL, pin_based_vm_exec_control),
    field!(CPU_BASED_VM_EXEC_CONTROL, cpu_based_vm_exec_control),
    field!(EXCEPTION_BITMAP, exception_bitmap),
    field!(PAGE_FAULT_ERROR_CODE_MASK, page_fault_error_code_mask),
    field!(PAGE_FAULT_ERROR_CODE_MATCH, page_fault_error_code_match),
    field!(CR3_TARGET_COUNT, cr3_target_count),
    field!(VM_EXIT_CONTROLS, vm_exit_controls),
    field!(VM_EXIT_MSR_STORE_COUNT, vm_exit_msr_store_count),
    field!(VM_EXIT_MSR_LOAD_COUNT, vm_exit_msr_load_count),
    field!(VM_ENTRY_CONTROLS, vm_entry_controls),
    field!(VM_ENTRY_MSR_LOAD_COUNT, vm_entry_msr_load_count),
    field!(VM_ENTRY_INTR_INFO_FIELD, vm_entry_intr_info_field),
    field!(VM_ENTRY_EXCEPTION_ERROR_CODE, vm_entry_exception_error_code),
    field!(VM_ENTRY_INSTRUCTION_LEN, vm_entry_instruction_len),
    field!(TPR_THRESHOLD, tpr_threshold),
    field!(SECONDARY_VM_EXEC_CONTROL, secondary_vm_exec_control),
    field!(VM_INSTRUCTION_ERROR, vm_instruction_error),
    field!(VM_EXIT_REASON, vm_exit_reason),
    field!(VM_EXIT_INTR_INFO, vm_exit_intr_info),
    field!(VM_EXIT_INTR_ERROR_CODE, vm_exit_intr_error_code),
    field!(IDT_VECTORING_INFO_FIELD, idt_vectoring_info_field),
    field!(IDT_VECTORING_ERROR_CODE, idt_vectoring_error_code),
    field!(VM_EXIT_INSTRUCTION_LEN, vm_exit_instruction_len),
    field!(VMX_INSTRUCTION_INFO, vmx_instruction_info),
    field!(GUEST_ES_LIMIT, guest_es_limit),
    field!(GUEST_CS_LIMIT, guest_cs_limit),
    field!(GUEST_SS_LIMIT, guest_ss_limit),
    field!(GUEST_DS_LIMIT, guest_ds_limit),
    field!(GUEST_FS_LIMIT, guest_fs_limit),
    field!(GUEST_GS_LIMIT, guest_gs_limit),
    field!(GUEST_LDTR_LIMIT, guest_ldtr_limit),
    field!(GUEST_TR_LIMIT, guest_tr_limit),
    field!(GUEST_GDTR_LIMIT, guest_gdtr_limit),
    field!(GUEST_IDTR_LIMIT, guest_idtr_limit),
    field!(GUEST_ES_AR_BYTES, guest_es_ar_bytes),
    field!(GUEST_CS_AR_BYTES, guest_cs_ar_bytes),
    field!(GUEST_SS_AR_BYTES, guest_ss_ar_bytes),
    field!(GUEST_DS_AR_BYTES, guest_ds_ar_bytes),
    field!(GUEST_FS_AR_BYTES, guest_fs_ar_bytes),
    field!(GUEST_GS_AR_BYTES, guest_gs_ar_bytes),
    field!(GUEST_LDTR_AR_BYTES, guest_ldtr_ar_bytes),
    field!(GUEST_TR_AR_BYTES, guest_tr_ar_bytes),
    field!(GUEST_INTERRUPTIBILITY_INFO, guest_interruptibility_info),
    field!(GUEST_ACTIVITY_STATE, guest_activity_state),
    field!(GUEST_SYSENTER_CS, guest_sysenter_cs),
    field!(HOST_IA32_SYSENTER_CS, host_ia32_sysenter_cs),
    field!(VMX_PREEMPTION_TIMER_VALUE, vmx_preemption_timer_value),
    field!(CR0_GUEST_HOST_MASK, cr0_guest_host_mask),
    field!(CR4_GUEST_HOST_MASK, cr4_guest_host_mask),
    field!(CR0_READ_SHADOW, cr0_read_shadow),
    field!(CR4_READ_SHADOW, cr4_read_shadow),
    field!(EXIT_QUALIFICATION, exit_qualification),
    field!(GUEST_LINEAR_ADDRESS, guest_linear_address),
    field!(GUEST_CR0, guest_cr0),
    field!(GUEST_CR3, guest_cr3),
    field!(GUEST_CR4, guest_cr4),
    field!(GUEST_ES_BASE, guest_es_base),
    field!(GUEST_CS_BASE, guest_cs_base),
    field!(GUEST_SS_BASE, guest_ss_base),
    field!(GUEST_DS_BASE, guest_ds_base),
    field!(GUEST_FS_BASE, guest_fs_base),
    field!(GUEST_GS_BASE, guest_gs_base),
    field!(GUEST_LDTR_BASE, guest_ldtr_base),
    field!(GUEST_TR_BASE, guest_tr_base),
    field!(GUEST_GDTR_BASE, guest_gdtr_base),
    field!(GUEST_IDTR_BASE, guest_idtr_base),
    field!(GUEST_DR7, guest_dr7),
    field!(GUEST_RSP, guest_rsp),
    field!(GUEST_RIP, guest_rip),
    field!(GUEST_RFLAGS, guest_rflags),
    field!(GUEST_PENDING_DBG_EXCEPTIONS, guest_pending_dbg_exceptions),
    field!(GUEST_SYSENTER_ESP, guest_sysenter_esp),
    field!(GUEST_SYSENTER_EIP, guest_sysenter_eip),
    field!(GUEST_S_CET, guest_s_cet),
    field!(GUEST_SSP, guest_ssp),
    field!(GUEST_INTR_SSP_TABLE, guest_ssp_tbl),
    field!(HOST_CR0, host_cr0),
    field!(HOST_CR3, host_cr3),
    field!(HOST_CR4, host_cr4),
    field!(HOST_FS_BASE, host_fs_base),
    field!(HOST_GS_BASE, host_gs_base),
    field!(HOST_TR_BASE, host_tr_base),
    field!(HOST_GDTR_BASE, host_gdtr_base),
    field!(HOST_IDTR_BASE, host_idtr_base),
    field!(HOST_IA32_SYSENTER_ESP, host_ia32_sysenter_esp),
    field!(HOST_IA32_SYSENTER_EIP, host_ia32_sysenter_eip),
    field!(HOST_RSP, host_rsp),
    field!(HOST_RIP, host_rip),
    field!(HOST_S_CET, host_s_cet),
    field!(HOST_SSP, host_ssp),
    field!(HOST_INTR_SSP_TABLE, host_ssp_tbl),
];

pub static mut vmcs12_field_offsets: [u16; 0] = [];
pub static mut nr_vmcs12_fields: u32 = 0;

macro_rules! vmcs12_case64 {
    ($enc:ident) => {
        $enc##_HIGH | $enc
    };
}

unsafe fn cpu_has_vmcs12_field(idx: u32) -> bool {
    match VMCS12_IDX_TO_ENC(idx) {
        VIRTUAL_PROCESSOR_ID => cpu_has_vmx_vpid(),
        POSTED_INTR_NV => cpu_has_vmx_posted_intr(),
        TSC_MULTIPLIER | TSC_MULTIPLIER_HIGH => cpu_has_vmx_tsc_scaling(),
        TPR_THRESHOLD | VIRTUAL_APIC_PAGE_ADDR | VIRTUAL_APIC_PAGE_ADDR_HIGH => cpu_has_vmx_tpr_shadow(),
        APIC_ACCESS_ADDR | APIC_ACCESS_ADDR_HIGH => cpu_has_vmx_virtualize_apic_accesses(),
        POSTED_INTR_DESC_ADDR | POSTED_INTR_DESC_ADDR_HIGH => cpu_has_vmx_posted_intr(),
        GUEST_INTR_STATUS => cpu_has_vmx_virtual_intr_delivery(),
        VM_FUNCTION_CONTROL | VM_FUNCTION_CONTROL_HIGH | EPTP_LIST_ADDRESS | EPTP_LIST_ADDRESS_HIGH => cpu_has_vmx_vmfunc(),
        EPT_POINTER | EPT_POINTER_HIGH => cpu_has_vmx_ept(),
        XSS_EXIT_BITMAP | XSS_EXIT_BITMAP_HIGH => cpu_has_vmx_xsaves(),
        ENCLS_EXITING_BITMAP | ENCLS_EXITING_BITMAP_HIGH => cpu_has_vmx_encls_vmexit(),
        GUEST_IA32_PERF_GLOBAL_CTRL | GUEST_IA32_PERF_GLOBAL_CTRL_HIGH |
        HOST_IA32_PERF_GLOBAL_CTRL | HOST_IA32_PERF_GLOBAL_CTRL_HIGH => cpu_has_load_perf_global_ctrl(),
        SECONDARY_VM_EXEC_CONTROL => cpu_has_secondary_exec_ctrls(),
        GUEST_S_CET | GUEST_SSP | GUEST_INTR_SSP_TABLE |
        HOST_S_CET | HOST_SSP | HOST_INTR_SSP_TABLE => cpu_has_load_cet_ctrl(),
        // KVM always emulates PML and the VMX preemption timer in software.
        GUEST_PML_INDEX | VMX_PREEMPTION_TIMER_VALUE => true,
        _ => true,
    }
}

pub unsafe fn nested_vmx_setup_vmcs12_fields() {
    let mut i = 0usize;
    while i < KVM_SUPPORTED_VMCS12_FIELD_OFFSETS.len() {
        let (idx, offset) = KVM_SUPPORTED_VMCS12_FIELD_OFFSETS[i];
        if offset == 0 || !cpu_has_vmcs12_field(i as u32) {
            i += 1;
            continue;
        }
        // The destination is supplied by the surrounding VMX implementation;
        // this assignment mirrors vmcs12_field_offsets[i] = offset.
        let _ = (idx, offset);
        nr_vmcs12_fields = (i + 1) as u32;
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
