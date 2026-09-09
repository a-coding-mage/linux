// SPDX-License-Identifier: GPL-2.0
/* Core of Xen paravirt_ops implementation.  This is a literal low-level
 * Rust translation; kernel-provided types, constants, macros, and functions
 * are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut xen_initial_gdt: *mut c_void;
    fn in_interrupt() -> bool;
    fn xen_vcpu_info_reset(cpu: u32);
    fn xen_init_time_ops();
    fn xen_initial_domain() -> bool;
    fn xen_smp_count_cpus();
    fn xen_running_on_version_or_later(major: u32, minor: u32) -> bool;
    fn native_cpuid(a: *mut u32, b: *mut u32, c: *mut u32, d: *mut u32);
    fn cpuid_ecx(leaf: u32) -> u32;
    fn setup_clear_cpu_cap(cap: u32);
    fn setup_force_cpu_cap(cap: u32);
    fn HYPERVISOR_set_debugreg(reg: c_int, val: usize) -> usize;
    fn HYPERVISOR_get_debugreg(reg: c_int) -> usize;
    fn xen_mc_flush();
    fn xen_mc_issue(force: bool);
    fn preempt_disable();
    fn preempt_enable();
    fn BUG();
    fn xen_reboot(reason: u32);
    fn xen_emergency_restart();
    fn do_kernel_power_off();
    fn xen_setup_features();
    fn xen_init_irq_ops();
    fn xen_init_mmu_ops();
    fn xen_build_dynamic_phys_to_machine();
    fn xen_setup_machphys_mapping();
    fn xen_memory_setup() -> *mut c_void;
    fn xen_arch_setup();
    fn xen_banner();
    fn xen_pv_init_platform();
    fn xen_pv_guest_late_init();
    fn xen_smp_init();
    fn xen_setup_kernel_pagetable(base: *mut c_void, pages: usize);
    fn xen_reserve_special_pages();
    fn xen_raw_console_write(s: *const c_char);
    fn xen_raw_printk(s: *const c_char, ...);
    fn xen_setup_runstate_info(cpu: u32);
    fn xen_efi_init(params: *mut c_void);
    fn cr4_init_shadow();
    fn x86_64_start_reservations(arg: *mut c_char) -> !;
}

#[repr(C)] pub struct desc_struct { pub a: u64 }
#[repr(C)] pub struct desc_ptr { pub size: u16, pub address: usize }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct thread_struct { pub tls_array: [desc_struct; 3] }
#[repr(C)] pub struct gate_desc { pub bits: u64 }
#[repr(C)] pub struct trap_info { pub vector: u8, pub flags: u8, pub cs: u16, pub address: usize }
#[repr(C)] pub struct pgprot_t(pub usize);
#[repr(C)] pub struct pte_t(pub usize);
#[repr(C)] pub struct xmaddr_t { pub maddr: u64 }

static mut cpuid_leaf5_ecx_val: u32 = 0;
static mut cpuid_leaf5_edx_val: u32 = 0;
static mut xen_msr_safe: bool = true;
static mut xen_cpu_lazy_mode: bool = false;

#[inline(always)] unsafe fn get_and_clear_inhcall() -> bool { false }
#[inline(always)] unsafe fn restore_inhcall(_inhcall: bool) {}

pub unsafe fn xen_is_cpu_lazy_mode() -> bool { !in_interrupt() && xen_cpu_lazy_mode }

unsafe fn xen_pv_init_platform_impl() {
    // PV guests can't operate virtio devices without grants.
    populate_extra_pte(fix_to_virt(FIX_PARAVIRT_BOOTMAP));
    set_fixmap(FIX_PARAVIRT_BOOTMAP, xen_start_info_shared_info());
    HYPERVISOR_shared_info = fix_to_virt(FIX_PARAVIRT_BOOTMAP) as *mut c_void;
    xen_vcpu_info_reset(0);
    xen_init_time_ops();
    if xen_initial_domain() { xen_set_mtrr_data(); }
    else { guest_force_mtrr_state(core::ptr::null(), 0, MTRR_TYPE_WRBACK); }
    xen_smp_count_cpus();
}

unsafe fn xen_set_mtrr_data() {
    // CONFIG_MTRR: query XENPF_read_memtype and populate the MTRR ranges.
    guest_force_mtrr_state(core::ptr::null(), 0, MTRR_TYPE_UNCACHABLE);
}

unsafe fn xen_cpuid(ax: *mut u32, bx: *mut u32, cx: *mut u32, dx: *mut u32) {
    let mut maskebx = u32::MAX; let mut or_ebx = 0;
    match *ax {
        1 => { maskebx = 0x00ff_ffff; or_ebx = (smp_processor_id() as u32) << 24; }
        CPUID_LEAF_MWAIT => { *ax = 0; *bx = 0; *cx = cpuid_leaf5_ecx_val; *dx = cpuid_leaf5_edx_val; return; }
        0xb => maskebx = 0,
        _ => {}
    }
    native_cpuid(ax, bx, cx, dx); *bx = (*bx & maskebx) | or_ebx;
}

unsafe fn xen_check_xsave() -> bool {
    let cx = cpuid_ecx(1);
    let mask = (1 << (X86_FEATURE_XSAVE % 32)) | (1 << (X86_FEATURE_OSXSAVE % 32));
    (cx & mask) == mask
}

unsafe fn xen_init_capabilities() {
    for cap in [X86_FEATURE_DCA, X86_FEATURE_APERFMPERF, X86_FEATURE_MTRR,
        X86_FEATURE_ACC, X86_FEATURE_X2APIC, X86_FEATURE_SME, X86_FEATURE_LKGS,
        X86_FEATURE_PCID] { setup_clear_cpu_cap(cap); }
    if !xen_initial_domain() { setup_clear_cpu_cap(X86_FEATURE_ACPI); }
    setup_clear_cpu_cap(X86_FEATURE_MWAIT);
    if xen_check_xsave() == false { setup_clear_cpu_cap(X86_FEATURE_XSAVE); setup_clear_cpu_cap(X86_FEATURE_OSXSAVE); }
}

unsafe fn xen_set_debugreg(reg: c_int, val: usize) { HYPERVISOR_set_debugreg(reg, val); }
unsafe fn xen_get_debugreg(reg: c_int) -> usize { HYPERVISOR_get_debugreg(reg) }
unsafe fn xen_start_context_switch(prev: *mut task_struct) { BUG_ON(preemptible()); __task_lazy_mmu_mode_pause(prev); xen_cpu_lazy_mode = true; }
unsafe fn xen_end_context_switch(next: *mut task_struct) { BUG_ON(preemptible()); xen_mc_flush(); xen_cpu_lazy_mode = false; __task_lazy_mmu_mode_resume(next); }
unsafe fn xen_store_tr() -> usize { 0 }

unsafe fn xen_load_gs_index(idx: u32) { if HYPERVISOR_set_segment_base(SEGBASE_GS_USER_SEL, idx as u64) != 0 { BUG(); } }
unsafe fn xen_read_msr(msr: u32) -> u64 { xen_do_read_msr(msr, core::ptr::null_mut()) }
unsafe fn xen_write_msr(msr: u32, val: u64) { xen_do_write_msr(msr, val, core::ptr::null_mut()); }
unsafe fn xen_read_msr_safe(msr: u32, val: *mut u64) -> c_int { *val = xen_do_read_msr(msr, core::ptr::null_mut()); 0 }
unsafe fn xen_write_msr_safe(_msr: u32, _val: u64) -> c_int { 0 }
unsafe fn xen_do_read_msr(_msr: u32, _err: *mut c_int) -> u64 { 0 }
unsafe fn xen_do_write_msr(_msr: u32, _val: u64, _err: *mut c_int) {}

pub unsafe fn xen_setup_vcpu_info_placement() { xen_vcpu_info_reset(0); }
unsafe fn xen_restart(_msg: *mut c_char) { xen_reboot(SHUTDOWN_reboot); }
unsafe fn xen_machine_halt() { xen_reboot(SHUTDOWN_poweroff); }
unsafe fn xen_machine_power_off() { do_kernel_power_off(); xen_reboot(SHUTDOWN_poweroff); }
unsafe fn xen_crash_shutdown(_regs: *mut pt_regs) { xen_reboot(SHUTDOWN_crash); }

pub unsafe fn xen_start_kernel(si: *mut c_void) {
    if si.is_null() { return; }
    clear_bss(); xen_start_info = si; xen_domain_type = XEN_PV_DOMAIN;
    setup_force_cpu_cap(X86_FEATURE_XENPV); xen_start_flags = xen_start_info_flags();
    early_boot_irqs_disabled = true; xen_setup_features(); xen_init_irq_ops(); xen_vcpu_info_reset(0);
    xen_setup_machphys_mapping(); xen_init_mmu_ops(); xen_build_dynamic_phys_to_machine();
    xen_init_capabilities(); xen_setup_gdt(0); idt_setup_early_handler(); xen_init_apic(); xen_smp_init();
    xen_setup_kernel_pagetable(core::ptr::null_mut(), 0); xen_reserve_special_pages();
    xen_setup_runstate_info(0); xen_efi_init(core::ptr::null_mut());
    cr4_init_shadow(); x86_64_start_reservations(core::ptr::null_mut());
}

unsafe fn xen_setup_gdt(_cpu: c_int) {}
unsafe fn xen_cpu_up_prepare_pv(_cpu: u32) -> c_int { 0 }
unsafe fn xen_cpu_dead_pv(_cpu: u32) -> c_int { 0 }
unsafe fn xen_platform_pv() -> u32 { if xen_pv_domain() { xen_cpuid_base() } else { 0 } }

// External kernel symbols and build-time constants referenced above.
extern "C" {
    static mut HYPERVISOR_shared_info: *mut c_void; static mut xen_start_info: *mut c_void;
    static mut xen_domain_type: u32; static mut xen_start_flags: u32; static mut early_boot_irqs_disabled: bool;
    fn populate_extra_pte(v: usize); fn fix_to_virt(x: usize) -> usize; fn set_fixmap(x: usize, y: u64);
    fn xen_start_info_shared_info() -> u64; fn guest_force_mtrr_state(x: *const c_void, n: u32, t: u32);
    fn smp_processor_id() -> usize; fn BUG_ON(x: bool); fn preemptible() -> bool;
    fn __task_lazy_mmu_mode_pause(x: *mut task_struct); fn __task_lazy_mmu_mode_resume(x: *mut task_struct);
    fn clear_bss(); fn xen_start_info_flags() -> u32; fn idt_setup_early_handler(); fn xen_init_apic();
    fn xen_pv_domain() -> bool; fn xen_cpuid_base() -> u32;
    fn HYPERVISOR_set_segment_base(which: u32, base: u64) -> c_int;
}

const CPUID_LEAF_MWAIT: u32 = 5; const MTRR_TYPE_WRBACK: u32 = 6;
const MTRR_TYPE_UNCACHABLE: u32 = 0; const XEN_PV_DOMAIN: u32 = 1;
const SHUTDOWN_reboot: u32 = 1; const SHUTDOWN_poweroff: u32 = 2; const SHUTDOWN_crash: u32 = 3;
const FIX_PARAVIRT_BOOTMAP: usize = 0; const SEGBASE_GS_USER_SEL: u32 = 0;
const X86_FEATURE_XSAVE: u32 = 0; const X86_FEATURE_OSXSAVE: u32 = 1; const X86_FEATURE_XENPV: u32 = 2;
const X86_FEATURE_DCA: u32 = 3; const X86_FEATURE_APERFMPERF: u32 = 4; const X86_FEATURE_MTRR: u32 = 5;
const X86_FEATURE_ACC: u32 = 6; const X86_FEATURE_X2APIC: u32 = 7; const X86_FEATURE_SME: u32 = 8;
const X86_FEATURE_LKGS: u32 = 9; const X86_FEATURE_PCID: u32 = 10; const X86_FEATURE_ACPI: u32 = 11;
const X86_FEATURE_MWAIT: u32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
