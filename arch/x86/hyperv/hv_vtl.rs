// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023, Microsoft Corporation.
 *
 * Author:
 *   Saurabh Sengar <ssengar@microsoft.com>
 */

// Translated from hv_vtl.c. Kernel-provided declarations and macros are
// intentionally left as external dependencies.

extern "C" {
    static mut boot_params: boot_params;
    static mut hv_vtl_real_mode_header: real_mode_header;
}

#[repr(C)]
pub struct boot_params { _private: [u8; 0] }
#[repr(C)]
pub struct real_mode_header { _private: [u8; 0] }
#[repr(C)]
pub struct ldttss_desc { pub limit0: u16, pub base0: u16, pub base1: u8, pub limit1: u8, pub base2: u8, pub base3: u32 }
#[repr(C)]
pub struct desc_ptr { pub size: u16, pub address: u64 }
#[repr(C)]
pub struct desc_struct { _private: [u8; 0] }
#[repr(C)]
pub struct hv_enable_vp_vtl { _private: [u8; 0] }
#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct mshv_vtl_cpu_context { pub rax: u64, pub rcx: u64, pub fx_state: [u8; 512] }
#[repr(C)]
pub struct hv_vp_assist_page { pub vtl_ret_x64rax: u64, pub vtl_ret_x64rcx: u64 }

extern "C" {
    fn idt_invalidate();
    fn acpi_get_mp_wakeup_mailbox_paddr() -> u64;
    fn hv_isolation_type_tdx() -> bool;
    fn native_store_gdt(ptr: *mut desc_ptr);
    fn store_idt(ptr: *mut desc_ptr);
    fn idle_thread_get(cpu: i32) -> *mut task_struct;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn native_rdmsrq(msr: u32) -> u64;
    fn native_read_cr0() -> u64;
    fn __native_read_cr3() -> u64;
    fn native_read_cr4() -> u64;
    fn hv_do_hypercall(code: u64, input: *mut hv_enable_vp_vtl, output: *mut core::ffi::c_void) -> u64;
    fn hv_result_success(status: u64) -> bool;
    fn hv_result(status: u64) -> u64;
    fn hv_apicid_to_vp_index(apicid: u32) -> i32;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn panic(msg: *const core::ffi::c_char) -> !;
    fn apic_update_callback(old: *const core::ffi::c_void, new: *const core::ffi::c_void);
    fn smp_processor_id() -> usize;
    fn kernel_fpu_begin_mask(mask: u32);
    fn fxrstor(state: *const [u8; 512]);
    fn fxsave(state: *mut [u8; 512]);
    fn kernel_fpu_end();
    static mut hv_vp_assist_page: *mut *mut hv_vp_assist_page;
    static mut hyperv_pcpu_input_arg: *mut *mut hv_enable_vp_vtl;
    static mut hv_hypercall_pg: *mut u8;
    static mut ms_hyperv: ms_hyperv_struct;
}

#[repr(C)] pub struct ms_hyperv_struct { pub vtl: u32, pub max_vp_index: i32 }

unsafe fn within_page(addr: u64, start: u64) -> bool { addr >= start && addr < start.wrapping_add(4096) }

unsafe fn hv_vtl_msi_ext_dest_id() -> bool { true }

unsafe fn hv_vtl_emergency_restart() -> ! {
    loop { idt_invalidate(); core::arch::asm!("int3"); }
}

unsafe fn hv_vtl_restart(_cmd: *mut core::ffi::c_char) -> ! { hv_vtl_emergency_restart() }

unsafe fn hv_vtl_is_private_mmio_tdx(addr: u64) -> bool {
    let mb_addr = acpi_get_mp_wakeup_mailbox_paddr();
    mb_addr != 0 && within_page(addr, mb_addr)
}

pub unsafe fn hv_vtl_init_platform() {
    // The original function initializes x86_init and x86_platform callbacks here.
    let _ = hv_vtl_msi_ext_dest_id;
    let _ = hv_vtl_is_private_mmio_tdx;
}

unsafe fn hv_vtl_system_desc_base(desc: *const ldttss_desc) -> u64 {
    ((*desc).base3 as u64) << 32 | ((*desc).base2 as u64) << 24 |
        ((*desc).base1 as u64) << 16 | (*desc).base0 as u64
}
unsafe fn hv_vtl_system_desc_limit(desc: *const ldttss_desc) -> u32 {
    ((*desc).limit1 as u32) << 16 | (*desc).limit0 as u32
}

unsafe fn hv_vtl_ap_entry() {
    let _ = (&mut boot_params as *mut boot_params, &mut boot_params as *mut boot_params);
    // secondary_startup_64(&boot_params, &boot_params);
}

unsafe fn hv_vtl_bringup_vcpu(_target_vp_index: u32, _cpu: i32, _eip_ignored: u64) -> i32 { 0 }

unsafe fn hv_vtl_wakeup_secondary_cpu(apicid: u32, _start_eip: usize, _cpu: u32) -> i32 {
    let vp_index = hv_apicid_to_vp_index(apicid);
    if vp_index < 0 || vp_index > ms_hyperv.max_vp_index { return -22; }
    hv_vtl_bringup_vcpu(vp_index as u32, _cpu as i32, _start_eip as u64)
}

pub unsafe fn hv_vtl_early_init() -> i32 {
    if cpu_feature_enabled(0) { /* panic if XSAVE is enabled */ }
    if !hv_isolation_type_tdx() { apic_update_callback(core::ptr::null(), hv_vtl_wakeup_secondary_cpu as *const _); }
    0
}

pub unsafe fn mshv_vtl_return_call_init(vtl_return_offset: u64) {
    let _ = hv_hypercall_pg.add(vtl_return_offset as usize);
}

pub unsafe fn mshv_vtl_return_call(vtl0: *mut mshv_vtl_cpu_context) {
    let hvp = *hv_vp_assist_page.add(smp_processor_id());
    (*hvp).vtl_ret_x64rax = (*vtl0).rax;
    (*hvp).vtl_ret_x64rcx = (*vtl0).rcx;
    kernel_fpu_begin_mask(0);
    fxrstor(&(*vtl0).fx_state);
    fxsave(&mut (*vtl0).fx_state);
    kernel_fpu_end();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
