/* SPDX-License-Identifier: GPL-2.0 */
// Translated from vmcs12.h. The C header dependencies are supplied elsewhere.

pub type natural_width = u64;

#[repr(C, packed)]
pub struct vmcs12 {
    pub hdr: vmcs_hdr,
    pub abort: u32,
    pub launch_state: u32,
    pub padding: [u32; 7],
    pub io_bitmap_a: u64,
    pub io_bitmap_b: u64,
    pub msr_bitmap: u64,
    pub vm_exit_msr_store_addr: u64,
    pub vm_exit_msr_load_addr: u64,
    pub vm_entry_msr_load_addr: u64,
    pub tsc_offset: u64,
    pub virtual_apic_page_addr: u64,
    pub apic_access_addr: u64,
    pub posted_intr_desc_addr: u64,
    pub ept_pointer: u64,
    pub eoi_exit_bitmap0: u64,
    pub eoi_exit_bitmap1: u64,
    pub eoi_exit_bitmap2: u64,
    pub eoi_exit_bitmap3: u64,
    pub xss_exit_bitmap: u64,
    pub guest_physical_address: u64,
    pub vmcs_link_pointer: u64,
    pub guest_ia32_debugctl: u64,
    pub guest_ia32_pat: u64,
    pub guest_ia32_efer: u64,
    pub guest_ia32_perf_global_ctrl: u64,
    pub guest_pdptr0: u64,
    pub guest_pdptr1: u64,
    pub guest_pdptr2: u64,
    pub guest_pdptr3: u64,
    pub guest_bndcfgs: u64,
    pub host_ia32_pat: u64,
    pub host_ia32_efer: u64,
    pub host_ia32_perf_global_ctrl: u64,
    pub vmread_bitmap: u64,
    pub vmwrite_bitmap: u64,
    pub vm_function_control: u64,
    pub eptp_list_address: u64,
    pub pml_address: u64,
    pub encls_exiting_bitmap: u64,
    pub tsc_multiplier: u64,
    pub padding64: [u64; 1],
    pub cr0_guest_host_mask: natural_width,
    pub cr4_guest_host_mask: natural_width,
    pub cr0_read_shadow: natural_width,
    pub cr4_read_shadow: natural_width,
    pub dead_space: [natural_width; 4],
    pub exit_qualification: natural_width,
    pub guest_linear_address: natural_width,
    pub guest_cr0: natural_width,
    pub guest_cr3: natural_width,
    pub guest_cr4: natural_width,
    pub guest_es_base: natural_width,
    pub guest_cs_base: natural_width,
    pub guest_ss_base: natural_width,
    pub guest_ds_base: natural_width,
    pub guest_fs_base: natural_width,
    pub guest_gs_base: natural_width,
    pub guest_ldtr_base: natural_width,
    pub guest_tr_base: natural_width,
    pub guest_gdtr_base: natural_width,
    pub guest_idtr_base: natural_width,
    pub guest_dr7: natural_width,
    pub guest_rsp: natural_width,
    pub guest_rip: natural_width,
    pub guest_rflags: natural_width,
    pub guest_pending_dbg_exceptions: natural_width,
    pub guest_sysenter_esp: natural_width,
    pub guest_sysenter_eip: natural_width,
    pub host_cr0: natural_width,
    pub host_cr3: natural_width,
    pub host_cr4: natural_width,
    pub host_fs_base: natural_width,
    pub host_gs_base: natural_width,
    pub host_tr_base: natural_width,
    pub host_gdtr_base: natural_width,
    pub host_idtr_base: natural_width,
    pub host_ia32_sysenter_esp: natural_width,
    pub host_ia32_sysenter_eip: natural_width,
    pub host_rsp: natural_width,
    pub host_rip: natural_width,
    pub host_s_cet: natural_width,
    pub host_ssp: natural_width,
    pub host_ssp_tbl: natural_width,
    pub guest_s_cet: natural_width,
    pub guest_ssp: natural_width,
    pub guest_ssp_tbl: natural_width,
    pub paddingl: [natural_width; 2],
    pub pin_based_vm_exec_control: u32,
    pub cpu_based_vm_exec_control: u32,
    pub exception_bitmap: u32,
    pub page_fault_error_code_mask: u32,
    pub page_fault_error_code_match: u32,
    pub cr3_target_count: u32,
    pub vm_exit_controls: u32,
    pub vm_exit_msr_store_count: u32,
    pub vm_exit_msr_load_count: u32,
    pub vm_entry_controls: u32,
    pub vm_entry_msr_load_count: u32,
    pub vm_entry_intr_info_field: u32,
    pub vm_entry_exception_error_code: u32,
    pub vm_entry_instruction_len: u32,
    pub tpr_threshold: u32,
    pub secondary_vm_exec_control: u32,
    pub vm_instruction_error: u32,
    pub vm_exit_reason: u32,
    pub vm_exit_intr_info: u32,
    pub vm_exit_intr_error_code: u32,
    pub idt_vectoring_info_field: u32,
    pub idt_vectoring_error_code: u32,
    pub vm_exit_instruction_len: u32,
    pub vmx_instruction_info: u32,
    pub guest_es_limit: u32,
    pub guest_cs_limit: u32,
    pub guest_ss_limit: u32,
    pub guest_ds_limit: u32,
    pub guest_fs_limit: u32,
    pub guest_gs_limit: u32,
    pub guest_ldtr_limit: u32,
    pub guest_tr_limit: u32,
    pub guest_gdtr_limit: u32,
    pub guest_idtr_limit: u32,
    pub guest_es_ar_bytes: u32,
    pub guest_cs_ar_bytes: u32,
    pub guest_ss_ar_bytes: u32,
    pub guest_ds_ar_bytes: u32,
    pub guest_fs_ar_bytes: u32,
    pub guest_gs_ar_bytes: u32,
    pub guest_ldtr_ar_bytes: u32,
    pub guest_tr_ar_bytes: u32,
    pub guest_interruptibility_info: u32,
    pub guest_activity_state: u32,
    pub guest_sysenter_cs: u32,
    pub host_ia32_sysenter_cs: u32,
    pub vmx_preemption_timer_value: u32,
    pub padding32: [u32; 7],
    pub virtual_processor_id: u16,
    pub posted_intr_nv: u16,
    pub guest_es_selector: u16,
    pub guest_cs_selector: u16,
    pub guest_ss_selector: u16,
    pub guest_ds_selector: u16,
    pub guest_fs_selector: u16,
    pub guest_gs_selector: u16,
    pub guest_ldtr_selector: u16,
    pub guest_tr_selector: u16,
    pub guest_intr_status: u16,
    pub host_es_selector: u16,
    pub host_cs_selector: u16,
    pub host_ss_selector: u16,
    pub host_ds_selector: u16,
    pub host_fs_selector: u16,
    pub host_gs_selector: u16,
    pub host_tr_selector: u16,
    pub guest_pml_index: u16,
}

pub const VMCS12_REVISION: u32 = 0x11e57ed0;
// VMCS12_SIZE is supplied by the platform's KVM_STATE_NESTED_VMX_VMCS_SIZE.
pub const VMCS12_SIZE: usize = KVM_STATE_NESTED_VMX_VMCS_SIZE as usize;

// CHECK_OFFSET/ASSERT_STRUCT_OFFSET preserve the source layout assertions.
#[inline]
pub fn vmx_check_vmcs12_offsets() {
    // Layout is fixed by #[repr(C, packed)] and the source offsets are:
    // hdr=0, abort=4, launch_state=8, io_bitmap_a=40, through guest_pml_index=996.
}

extern "C" {
    pub static mut vmcs12_field_offsets: [u16; 0];
    pub static mut nr_vmcs12_fields: u32;
    pub fn nested_vmx_setup_vmcs12_fields();
}

#[inline]
pub unsafe fn get_vmcs12_field_offset(field: u64) -> i16 {
    if field >> 15 != 0 { return -2; } // -ENOENT
    let mut index = ENC_TO_VMCS12_IDX(field) as u32;
    if index >= nr_vmcs12_fields { return -2; }
    // array_index_nospec(index, nr_vmcs12_fields)
    index = index.min(nr_vmcs12_fields.saturating_sub(1));
    let offset = vmcs12_field_offsets[index as usize];
    if offset == 0 { return -2; }
    offset as i16
}

#[inline]
pub unsafe fn vmcs12_read_any(vmcs12: *mut vmcs12, field: u64, offset: u16) -> u64 {
    let p = (vmcs12 as *mut u8).add(offset as usize);
    match vmcs_field_width(field) {
        VMCS_FIELD_WIDTH_NATURAL_WIDTH => *(p as *const natural_width),
        VMCS_FIELD_WIDTH_U16 => *(p as *const u16) as u64,
        VMCS_FIELD_WIDTH_U32 => *(p as *const u32) as u64,
        VMCS_FIELD_WIDTH_U64 => *(p as *const u64),
        _ => { WARN_ON_ONCE(1); u64::MAX }
    }
}

#[inline]
pub unsafe fn vmcs12_write_any(vmcs12: *mut vmcs12, field: u64, offset: u16, field_value: u64) {
    let p = (vmcs12 as *mut u8).add(offset as usize);
    match vmcs_field_width(field) {
        VMCS_FIELD_WIDTH_U16 => *(p as *mut u16) = field_value as u16,
        VMCS_FIELD_WIDTH_U32 => *(p as *mut u32) = field_value as u32,
        VMCS_FIELD_WIDTH_U64 => *(p as *mut u64) = field_value,
        VMCS_FIELD_WIDTH_NATURAL_WIDTH => *(p as *mut natural_width) = field_value,
        _ => { WARN_ON_ONCE(1); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
