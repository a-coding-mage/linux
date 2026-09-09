/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This file contains common definitions for working with Enlightened VMCS which
 * are used both by Hyper-V on KVM and KVM on Hyper-V.
 *
 * C dependencies supplied by the surrounding translation unit:
 * hyperv/hvhdk.h, capabilities.h, and vmcs12.h.
 */

pub const KVM_EVMCS_VERSION: u32 = 1;

pub const EVMCS1_SUPPORTED_PINCTRL: u32 =
    PIN_BASED_ALWAYSON_WITHOUT_TRUE_MSR |
    PIN_BASED_EXT_INTR_MASK |
    PIN_BASED_NMI_EXITING |
    PIN_BASED_VIRTUAL_NMIS;

pub const EVMCS1_SUPPORTED_EXEC_CTRL: u32 =
    CPU_BASED_ALWAYSON_WITHOUT_TRUE_MSR |
    CPU_BASED_HLT_EXITING |
    CPU_BASED_CR3_LOAD_EXITING |
    CPU_BASED_CR3_STORE_EXITING |
    CPU_BASED_UNCOND_IO_EXITING |
    CPU_BASED_MOV_DR_EXITING |
    CPU_BASED_USE_TSC_OFFSETTING |
    CPU_BASED_MWAIT_EXITING |
    CPU_BASED_MONITOR_EXITING |
    CPU_BASED_INVLPG_EXITING |
    CPU_BASED_RDPMC_EXITING |
    CPU_BASED_INTR_WINDOW_EXITING |
    CPU_BASED_CR8_LOAD_EXITING |
    CPU_BASED_CR8_STORE_EXITING |
    CPU_BASED_RDTSC_EXITING |
    CPU_BASED_TPR_SHADOW |
    CPU_BASED_USE_IO_BITMAPS |
    CPU_BASED_MONITOR_TRAP_FLAG |
    CPU_BASED_USE_MSR_BITMAPS |
    CPU_BASED_NMI_WINDOW_EXITING |
    CPU_BASED_PAUSE_EXITING |
    CPU_BASED_ACTIVATE_SECONDARY_CONTROLS;

pub const EVMCS1_SUPPORTED_2NDEXEC: u32 =
    SECONDARY_EXEC_VIRTUALIZE_X2APIC_MODE |
    SECONDARY_EXEC_WBINVD_EXITING |
    SECONDARY_EXEC_ENABLE_VPID |
    SECONDARY_EXEC_ENABLE_EPT |
    SECONDARY_EXEC_UNRESTRICTED_GUEST |
    SECONDARY_EXEC_DESC |
    SECONDARY_EXEC_ENABLE_RDTSCP |
    SECONDARY_EXEC_ENABLE_INVPCID |
    SECONDARY_EXEC_ENABLE_XSAVES |
    SECONDARY_EXEC_RDSEED_EXITING |
    SECONDARY_EXEC_RDRAND_EXITING |
    SECONDARY_EXEC_TSC_SCALING |
    SECONDARY_EXEC_ENABLE_USR_WAIT_PAUSE |
    SECONDARY_EXEC_PT_USE_GPA |
    SECONDARY_EXEC_PT_CONCEAL_VMX |
    SECONDARY_EXEC_BUS_LOCK_DETECTION |
    SECONDARY_EXEC_NOTIFY_VM_EXITING |
    SECONDARY_EXEC_MODE_BASED_EPT_EXEC |
    SECONDARY_EXEC_ENCLS_EXITING;

pub const EVMCS1_SUPPORTED_3RDEXEC: u64 = 0;

pub const EVMCS1_SUPPORTED_VMEXIT_CTRL: u32 =
    VM_EXIT_ALWAYSON_WITHOUT_TRUE_MSR |
    VM_EXIT_SAVE_DEBUG_CONTROLS |
    VM_EXIT_ACK_INTR_ON_EXIT |
    VM_EXIT_HOST_ADDR_SPACE_SIZE |
    VM_EXIT_LOAD_IA32_PERF_GLOBAL_CTRL |
    VM_EXIT_SAVE_IA32_PAT |
    VM_EXIT_LOAD_IA32_PAT |
    VM_EXIT_SAVE_IA32_EFER |
    VM_EXIT_LOAD_IA32_EFER |
    VM_EXIT_CLEAR_BNDCFGS |
    VM_EXIT_PT_CONCEAL_PIP |
    VM_EXIT_CLEAR_IA32_RTIT_CTL;

pub const EVMCS1_SUPPORTED_VMENTRY_CTRL: u32 =
    VM_ENTRY_ALWAYSON_WITHOUT_TRUE_MSR |
    VM_ENTRY_LOAD_DEBUG_CONTROLS |
    VM_ENTRY_IA32E_MODE |
    VM_ENTRY_LOAD_IA32_PERF_GLOBAL_CTRL |
    VM_ENTRY_LOAD_IA32_PAT |
    VM_ENTRY_LOAD_IA32_EFER |
    VM_ENTRY_LOAD_BNDCFGS |
    VM_ENTRY_PT_CONCEAL_PIP |
    VM_ENTRY_LOAD_IA32_RTIT_CTL;

pub const EVMCS1_SUPPORTED_VMFUNC: u32 = 0;

#[repr(C)]
pub struct evmcs_field {
    pub offset: u16,
    pub clean_field: u16,
}

extern "C" {
    pub static vmcs_field_to_evmcs_1: *const evmcs_field;
    pub static nr_evmcs_1_fields: core::ffi::c_uint;
}

#[inline]
pub unsafe fn evmcs_field_offset(field: core::ffi::c_ulong, clean_field: *mut u16) -> i32 {
    let index: usize = ENC_TO_VMCS12_IDX(field) as usize;

    if index >= nr_evmcs_1_fields as usize {
        return -ENOENT;
    }

    let evmcs_field = &*vmcs_field_to_evmcs_1.add(index);

    /*
     * Use offset=0 to detect holes in eVMCS. This offset belongs to
     * 'revision_id' but this field has no encoding and is supposed to
     * be accessed directly.
     */
    if evmcs_field.offset == 0 {
        return -ENOENT;
    }

    if !clean_field.is_null() {
        *clean_field = evmcs_field.clean_field;
    }

    evmcs_field.offset as i32
}

#[inline]
pub unsafe fn evmcs_read_any(
    evmcs: *mut hv_enlightened_vmcs,
    field: core::ffi::c_ulong,
    offset: u16,
) -> u64 {
    /*
     * vmcs12_read_any() doesn't care whether the supplied structure
     * is 'struct vmcs12' or 'struct hv_enlightened_vmcs' as it takes
     * the exact offset of the required field, use it for convenience
     * here.
     */
    vmcs12_read_any(evmcs as *mut core::ffi::c_void, field, offset)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
