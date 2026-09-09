/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

/* External kernel types and symbols supplied by the surrounding translation. */
pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;

pub enum hv_guest_mapping_flush_list {}
pub enum pt_regs {}
pub enum hv_vp_assist_page {}
pub enum irq_domain {}
pub enum irq_data {}
pub enum hv_interrupt_entry {}
pub struct fxregs_state {
    _private: [u8; 0],
}

/* Hyper-V always provides a single IO-APIC at this MMIO address. */
pub const HV_IOAPIC_BASE_ADDRESS: u32 = 0xfec00000;

pub const HV_VTL_NORMAL: u32 = 0x0;
pub const HV_VTL_SECURE: u32 = 0x1;
pub const HV_VTL_MGMT: u32 = 0x2;

pub enum hv_ghcb {}

/* DECLARE_STATIC_KEY_FALSE(isolation_type_snp); */
/* DECLARE_STATIC_KEY_FALSE(isolation_type_tdx); */

pub type hyperv_fill_flush_list_func = unsafe extern "C" fn(
    flush: *mut hv_guest_mapping_flush_list,
    data: *mut c_void,
) -> ::core::ffi::c_int;

unsafe extern "C" {
    pub fn hyperv_vector_handler(regs: *mut pt_regs);
}

#[inline]
pub const unsafe fn hv_get_nmi_reason() -> u8 {
    0
}

unsafe extern "C" {
    pub fn hv_tdx_hypercall(control: u64, param1: u64, param2: u64) -> u64;
    pub fn hv_snp_hypercall(control: u64, param1: u64, param2: u64) -> u64;
    pub fn hv_std_hypercall(control: u64, param1: u64, param2: u64) -> u64;
}

/* CONFIG_HYPERV */
unsafe extern "C" {
    pub static mut hv_hypercall_pg: *mut c_void;
    pub static mut hv_ghcb_pg: *mut *mut hv_ghcb;
    pub fn hv_isolation_type_snp() -> bool;
    pub fn hv_isolation_type_tdx() -> bool;
}

/* CONFIG_X86_64: DECLARE_STATIC_CALL(hv_hypercall, hv_std_hypercall); */

pub const HV_AP_INIT_GPAT_DEFAULT: u64 = 0x0007040600070406;
pub const HV_AP_SEGMENT_LIMIT: u32 = 0xffff_ffff;

/* If the hypercall has no input or output parameters, the GPA is ignored. */
#[inline]
pub unsafe fn hv_do_hypercall(
    control: u64,
    input: *mut c_void,
    output: *mut c_void,
) -> u64 {
    let input_address = if !input.is_null() { input as u64 } else { 0 };
    let output_address = if !output.is_null() { output as u64 } else { 0 };

    /* CONFIG_X86_64 */
    hv_std_hypercall(control, input_address, output_address)
}

/* Fast hypercall with 8 bytes of input and no output. */
#[inline]
pub unsafe fn _hv_do_fast_hypercall8(control: u64, input1: u64) -> u64 {
    hv_std_hypercall(control, input1, 0)
}

#[inline]
pub unsafe fn hv_do_fast_hypercall8(code: u16, input1: u64) -> u64 {
    let control = (code as u64) | HV_HYPERCALL_FAST_BIT;
    _hv_do_fast_hypercall8(control, input1)
}

/* Fast hypercall with 16 bytes of input. */
#[inline]
pub unsafe fn _hv_do_fast_hypercall16(control: u64, input1: u64, input2: u64) -> u64 {
    hv_std_hypercall(control, input1, input2)
}

#[inline]
pub unsafe fn hv_do_fast_hypercall16(code: u16, input1: u64, input2: u64) -> u64 {
    let control = (code as u64) | HV_HYPERCALL_FAST_BIT;
    _hv_do_fast_hypercall16(control, input1, input2)
}

unsafe extern "C" {
    pub static mut hv_vp_assist_page: *mut *mut hv_vp_assist_page;
}

#[inline]
pub unsafe fn hv_get_vp_assist_page(cpu: u32) -> *mut hv_vp_assist_page {
    if hv_vp_assist_page.is_null() {
        return core::ptr::null_mut();
    }
    *hv_vp_assist_page.add(cpu as usize)
}

unsafe extern "C" {
    pub fn hyperv_init();
    pub fn hyperv_setup_mmu_ops();
    pub fn set_hv_tscchange_cb(cb: Option<unsafe extern "C" fn()>);
    pub fn clear_hv_tscchange_cb();
    pub fn hyperv_stop_tsc_emulation();
    pub fn hyperv_flush_guest_mapping(as_: u64) -> ::core::ffi::c_int;
    pub fn hyperv_flush_guest_mapping_range(
        as_: u64,
        fill_func: hyperv_fill_flush_list_func,
        data: *mut c_void,
    ) -> ::core::ffi::c_int;
    pub fn hyperv_fill_flush_guest_mapping_list(
        flush: *mut hv_guest_mapping_flush_list,
        start_gfn: u64,
        end_gfn: u64,
    ) -> ::core::ffi::c_int;
    pub fn hv_sleep_notifiers_register();
    pub fn hv_machine_power_off();
    pub fn hv_apic_init();
    pub fn hv_init_spinlocks();
    pub fn hv_vcpu_is_preempted(vcpu: ::core::ffi::c_int) -> bool;
    pub fn hv_create_pci_msi_domain() -> *mut irq_domain;
    pub fn hv_map_msi_interrupt(data: *mut irq_data, out_entry: *mut hv_interrupt_entry) -> ::core::ffi::c_int;
    pub fn hv_map_ioapic_interrupt(ioapic_id: ::core::ffi::c_int, level: bool, vcpu: ::core::ffi::c_int, vector: ::core::ffi::c_int, entry: *mut hv_interrupt_entry) -> ::core::ffi::c_int;
    pub fn hv_unmap_ioapic_interrupt(ioapic_id: ::core::ffi::c_int, entry: *mut hv_interrupt_entry) -> ::core::ffi::c_int;
}

/* CONFIG_AMD_MEM_ENCRYPT */
unsafe extern "C" {
    pub fn hv_ghcb_negotiate_protocol() -> bool;
    pub fn hv_ghcb_terminate(set: u32, reason: u32) -> !;
    pub fn hv_snp_boot_ap(apic_id: u32, start_ip: usize, cpu: u32) -> ::core::ffi::c_int;
}

/* CONFIG_AMD_MEM_ENCRYPT || CONFIG_INTEL_TDX_GUEST */
unsafe extern "C" {
    pub fn hv_vtom_init();
    pub fn hv_ivm_msr_write(msr: u64, value: u64);
    pub fn hv_ivm_msr_read(msr: u64, value: *mut u64);
}

#[inline]
pub const fn hv_is_synic_msr(reg: u32) -> bool {
    (reg >= HV_X64_MSR_SCONTROL) && (reg <= HV_X64_MSR_SINT15)
}

#[inline]
pub const fn hv_is_sint_msr(reg: u32) -> bool {
    (reg >= HV_X64_MSR_SINT0) && (reg <= HV_X64_MSR_SINT15)
}

unsafe extern "C" {
    pub fn hv_get_msr(reg: u32) -> u64;
    pub fn hv_set_msr(reg: u32, value: u64);
    pub fn hv_get_non_nested_msr(reg: u32) -> u64;
    pub fn hv_set_non_nested_msr(reg: u32, value: u64);
    pub fn native_rdmsrq(reg: u32) -> u64;
    pub fn hv_apicid_to_vp_index(apic_id: u32) -> ::core::ffi::c_int;
}

#[inline(always)]
pub unsafe fn hv_raw_get_msr(reg: u32) -> u64 {
    native_rdmsrq(reg)
}

/* CONFIG_MSHV_ROOT && CONFIG_CRASH_DUMP */
unsafe extern "C" {
    pub fn hv_root_crash_init();
    pub fn hv_crash_asm32();
    pub fn hv_crash_asm64();
    pub fn hv_crash_asm_end();
}

#[repr(C)]
pub union mshv_vtl_gp_regs {
    pub named: mshv_vtl_named_regs,
    pub gp_regs: [u64; 16],
}

#[repr(C)]
pub struct mshv_vtl_named_regs {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub cr2: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

#[repr(C)]
pub struct mshv_vtl_cpu_context {
    pub gp_regs: mshv_vtl_gp_regs,
    pub fx_state: fxregs_state,
}

/* CONFIG_HYPERV_VTL_MODE */
unsafe extern "C" {
    pub fn hv_vtl_init_platform();
    pub fn hv_vtl_early_init() -> ::core::ffi::c_int;
    pub fn mshv_vtl_return_call(vtl0: *mut mshv_vtl_cpu_context);
    pub fn mshv_vtl_return_call_init(vtl_return_offset: u64);
    pub fn mshv_vtl_return_hypercall();
    pub fn __mshv_vtl_return_call(vtl0: *mut mshv_vtl_cpu_context);
}

/* <asm-generic/mshyperv.h> supplies additional declarations. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
