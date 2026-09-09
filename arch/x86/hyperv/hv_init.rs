// SPDX-License-Identifier: GPL-2.0-only
/* X86 specific Hyper-V initialization code. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

// Kernel-provided types, constants, functions, globals, and macros are external
// dependencies supplied by the surrounding translation unit.
extern "C" {
    static mut hv_hypercall_pg: *mut c_void;
    static mut hv_ghcb_pg: *mut c_void;
    static mut hv_vp_assist_page: *mut *mut hv_vp_assist_page;
    static mut ms_hyperv: ms_hyperv_t;
    static mut hv_vp_index: *mut u32;
    static mut tsc_khz: u32;
    static mut x86_hyper_type: i32;
    static mut x86_init: x86_init_t;
    static mut hyperv_pcpu_input_arg: *mut *mut c_void;
    static mut hyperv_pcpu_output_arg: *mut *mut c_void;
    static mut panic_on_oops: bool;
    static mut cpu_online_mask: *mut c_void;
    static mut nr_cpu_ids: u32;

    fn hv_common_cpu_init(cpu: u32) -> i32;
    fn hv_common_cpu_die(cpu: u32);
    fn hv_common_init() -> i32;
    fn hv_common_free();
    fn hv_root_partition() -> bool;
    fn hv_isolation_type_snp() -> bool;
    fn hv_isolation_type_tdx() -> bool;
    fn hv_ghcb_negotiate_protocol() -> bool;
    fn hv_ghcb_terminate(a: u32, b: u32);
    fn hv_ivm_msr_write(msr: u32, value: u64);
    fn hv_generate_guest_id(version: u32) -> u64;
    fn hv_root_crash_init();
    fn hv_remap_tsc_clocksource();
    fn hv_sleep_notifiers_register();
    fn hv_apic_init();
    fn hv_get_partition_id();
    fn hv_query_ext_cap(arg: u32);
    fn get_vtl() -> u8;
    fn hv_vtl_early_init();
    fn hv_stimer_alloc(direct: bool) -> i32;
    fn hv_create_pci_msi_domain() -> *mut c_void;
    fn register_syscore(syscore: *const syscore);
    fn cpuhp_setup_state(state: i32, name: *const u8, init: unsafe extern "C" fn(u32) -> i32, die: unsafe extern "C" fn(u32) -> i32) -> i32;
    fn cpuhp_remove_state(state: i32);
    fn efi_enabled(feature: u32) -> bool;
    fn e820__reserve_resources_late();
    fn memremap(addr: u64, size: usize, flags: u64) -> *mut c_void;
    fn memunmap(addr: *mut c_void);
    fn this_cpu_ptr(p: *mut c_void) -> *mut *mut c_void;
    fn rdmsrq(msr: u32, value: *mut u64);
    fn wrmsrq(msr: u32, value: u64);
    fn hv_get_msr(msr: u32) -> u64;
    fn hv_set_msr(msr: u32, value: u64);
    fn apic_update_vector(cpu: u32, vector: u32, enabled: bool);
    fn apic_eoi();
    fn inc_irq_stat(stat: u32);
    fn schedule_delayed_work(work: *mut c_void, delay: u32);
    fn wmb();
    fn get_cpu() -> u32;
    fn put_cpu();
    fn cpumask_any_but(mask: *mut c_void, cpu: u32) -> u32;
    fn __vmalloc(size: usize, flags: u64) -> *mut c_void;
    fn __vmalloc_node_range(size: usize, align: usize, start: usize, end: usize, flags: u64, prot: u64, vm_flags: u64, node: i32, caller: *mut c_void) -> *mut c_void;
    fn vmalloc_to_pfn(addr: *mut c_void) -> u64;
    fn vmalloc_to_page(addr: *mut c_void) -> *mut c_void;
    fn set_memory_decrypted(addr: usize, pages: usize) -> i32;
    fn memset(dst: *mut c_void, value: i32, size: usize) -> *mut c_void;
    fn memcpy_to_page(page: *mut c_void, offset: usize, src: *const c_void, size: usize);
    fn BUG_ON(condition: bool);
    fn free_percpu(ptr: *mut c_void);
    fn alloc_percpu(size: usize) -> *mut c_void;
    fn kzalloc_objs(size: usize, count: u32) -> *mut *mut hv_vp_assist_page;
    fn kfree(ptr: *mut c_void);
    fn hv_do_hypercall(control: u64, input: *mut c_void, output: *mut u32) -> u64;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn hv_result_success(status: u64) -> bool;
}

#[repr(C)] pub struct hv_vp_assist_page { _private: [u8; 0] }
#[repr(C)] pub struct ms_hyperv_t { pub features: u64, pub misc_features: u64, pub hints: u64, pub priv_high: u64, pub paravisor_present: bool, pub vtl: u8, pub shared_gpa_boundary: u64 }
#[repr(C)] pub struct x86_timers_t { pub setup_percpu_clockev: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct x86_init_t { pub timers: x86_timers_t, pub pci: x86_pci_t, pub irqs: x86_irqs_t }
#[repr(C)] pub struct x86_pci_t { pub arch_init: Option<unsafe extern "C" fn() -> i32> }
#[repr(C)] pub struct x86_irqs_t { pub create_pci_msi_domain: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct syscore_ops { pub suspend: Option<unsafe extern "C" fn(*mut c_void) -> i32>, pub resume: Option<unsafe extern "C" fn(*mut c_void)> }
#[repr(C)] pub struct syscore { pub ops: *const syscore_ops }
#[repr(C)] pub union hv_vp_assist_msr_contents { pub as_uint64: u64, pub pfn: u64, pub enable: u64 }
#[repr(C)] pub union hv_x64_msr_hypercall_contents { pub as_uint64: u64, pub enable: u64, pub guest_physical_address: u64 }
#[repr(C)] pub union hv_reference_tsc_msr { pub as_uint64: u64, pub enable: u64 }
#[repr(C)] pub struct hv_reenlightenment_control { pub target_vp: u32, pub enabled: u32, pub _pad: u32 }
#[repr(C)] pub struct hv_tsc_emulation_control { pub enabled: u64 }
#[repr(C)] pub struct hv_tsc_emulation_status { pub inprogress: u64 }
#[repr(C)] pub struct pt_regs { pub ip: u64, pub ax: u64, pub sp: u64 }
#[repr(C)] pub struct hv_get_vp_from_apic_id_in { pub partition_id: u64, pub apic_ids: [u32; 1] }

static mut hv_hypercall_pg_saved: *mut c_void = core::ptr::null_mut();
static mut hv_reenlightenment_cb: Option<unsafe extern "C" fn()> = None;
static mut old_setup_percpu_clockev: Option<unsafe extern "C" fn()> = None;
static mut panic_reported: bool = false;

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn __hv_hyperfail(_: u64, _: u64, _: u64) -> u64 { u64::MAX }

#[cfg(target_arch = "x86_64")]
pub unsafe extern "C" fn hv_std_hypercall(control: u64, param1: u64, param2: u64) -> u64 {
    let mut status: u64;
    core::arch::asm!("call *{fnptr}", fnptr = in(reg) hv_std_hypercall as *const c_void,
        inlateout("rax") 0u64 => status, inlateout("rcx") control => _, inlateout("rdx") param1 => _, inlateout("r8") param2 => _, options(nostack));
    status
}

unsafe fn hv_set_hypercall_pg(ptr: *mut c_void) { hv_hypercall_pg = ptr; }

#[repr(C)] struct delayed_work { _private: [u8; 0] }
static mut hv_reenlightenment_work: delayed_work = delayed_work { _private: [] };

unsafe fn hyperv_init_ghcb() -> i32 {
    if !ms_hyperv.paravisor_present || !hv_isolation_type_snp() { return 0; }
    if hv_ghcb_pg.is_null() { return -22; }
    let mut gpa = 0; rdmsrq(0xC0010130, &mut gpa); gpa &= !ms_hyperv.shared_gpa_boundary;
    let va = memremap(gpa, 4096, 0); if va.is_null() { return -12; }
    *this_cpu_ptr(hv_ghcb_pg) = va; 0
}

unsafe extern "C" fn hv_cpu_init(cpu: u32) -> i32 { let ret = hv_common_cpu_init(cpu); if ret != 0 { return ret; } if !hv_vp_assist_page.is_null() { let p = hv_vp_assist_page.add(cpu as usize); if (*p).is_null() { **p = __vmalloc(4096, 0); } } apic_update_vector(cpu, 0xf1, true); hyperv_init_ghcb() }
unsafe extern "C" fn hv_cpu_die(cpu: u32) -> i32 { apic_update_vector(cpu, 0xf1, false); hv_common_cpu_die(cpu); 0 }

pub unsafe extern "C" fn hyperv_stop_tsc_emulation() { let mut s=0; rdmsrq(0x400000B0,&mut s); s &= !1; wrmsrq(0x400000B0,s); let mut f=0; rdmsrq(0x400000B1,&mut f); tsc_khz=(f/1000) as u32; }
unsafe fn hv_reenlightenment_available() -> bool { ms_hyperv.features & (1<<11) != 0 && ms_hyperv.misc_features & 1 != 0 && ms_hyperv.features & (1<<13) != 0 }
pub unsafe extern "C" fn set_hv_tscchange_cb(cb: Option<unsafe extern "C" fn()>) { if !hv_reenlightenment_available() || hv_vp_index.is_null() { return; } hv_reenlightenment_cb=cb; wmb(); let _ = get_cpu(); put_cpu(); }
pub unsafe extern "C" fn clear_hv_tscchange_cb() { if hv_reenlightenment_available() { hv_reenlightenment_cb=None; } }

unsafe fn hv_pci_init() -> i32 { if efi_enabled(1) { e820__reserve_resources_late(); 0 } else { 1 } }
unsafe fn hv_suspend(_: *mut c_void) -> i32 { if hv_root_partition() { return -1; } hv_hypercall_pg_saved=hv_hypercall_pg; hv_set_hypercall_pg(core::ptr::null_mut()); hv_cpu_die(0) }
unsafe fn hv_resume(_: *mut c_void) { let _=hv_cpu_init(0); hv_set_hypercall_pg(hv_hypercall_pg_saved); hv_hypercall_pg_saved=core::ptr::null_mut(); }
static hv_syscore_ops: syscore_ops = syscore_ops { suspend: Some(hv_suspend), resume: Some(hv_resume) };
static hv_syscore: syscore = syscore { ops: &hv_syscore_ops };
unsafe extern "C" fn hv_stimer_setup_percpu_clockev() { let _=hv_stimer_alloc(false); if let Some(f)=old_setup_percpu_clockev { f(); } }

pub unsafe extern "C" fn hyperv_init() {
    if x86_hyper_type != 2 || hv_common_init()!=0 { return; }
    hv_vp_assist_page=kzalloc_objs(core::mem::size_of::<*mut hv_vp_assist_page>(),nr_cpu_ids);
    if hv_vp_assist_page.is_null() && !hv_isolation_type_tdx() { hv_common_free(); return; }
    if ms_hyperv.paravisor_present && hv_isolation_type_snp() { if !hv_ghcb_negotiate_protocol() { hv_ghcb_terminate(0,0); } hv_ghcb_pg=alloc_percpu(core::mem::size_of::<*mut c_void>()); }
    if cpuhp_setup_state(0,"x86/hyperv_init:online\0".as_ptr(),hv_cpu_init,hv_cpu_die)<0 { free_percpu(hv_ghcb_pg); kfree(hv_vp_assist_page as *mut c_void); hv_common_free(); return; }
    let guest_id=hv_generate_guest_id(0); wrmsrq(0x40000000,guest_id); hv_ivm_msr_write(0x40000000,guest_id);
    if !(hv_isolation_type_tdx() && !ms_hyperv.paravisor_present) { hv_hypercall_pg=__vmalloc_node_range(4096,1,0,0,0,0,0,-1,core::ptr::null_mut()); if hv_hypercall_pg.is_null() { return; } hv_set_hypercall_pg(hv_hypercall_pg); }
    old_setup_percpu_clockev=x86_init.timers.setup_percpu_clockev; x86_init.timers.setup_percpu_clockev=Some(hv_stimer_setup_percpu_clockev); hv_apic_init(); register_syscore(&hv_syscore); hv_query_ext_cap(0); ms_hyperv.vtl=get_vtl(); if ms_hyperv.vtl>0 { hv_vtl_early_init(); }
}

pub unsafe extern "C" fn hyperv_cleanup() { wrmsrq(0x40000000,0); hv_ivm_msr_write(0x40000000,0); hv_hypercall_pg=core::ptr::null_mut(); }
pub unsafe extern "C" fn hyperv_report_panic(regs:*mut pt_regs, err:i64, in_die:bool) { if in_die && !panic_on_oops || panic_reported { return; } panic_reported=true; let mut guest=0; rdmsrq(0x40000000,&mut guest); wrmsrq(0x40000005,err as u64); wrmsrq(0x40000006,guest); wrmsrq(0x40000007,(*regs).ip); wrmsrq(0x40000008,(*regs).ax); wrmsrq(0x40000009,(*regs).sp); wrmsrq(0x4000000A,1); }
pub unsafe extern "C" fn hv_is_hyperv_initialized() -> bool { if x86_hyper_type!=2 { return false; } if hv_isolation_type_tdx() && !ms_hyperv.paravisor_present { return true; } !hv_hypercall_pg.is_null() }

pub unsafe extern "C" fn hv_apicid_to_vp_index(apic_id:u32) -> i32 { let mut flags=0; local_irq_save(&mut flags); let input=*this_cpu_ptr(hyperv_pcpu_input_arg) as *mut hv_get_vp_from_apic_id_in; (*input).partition_id=0xffff_ffff_ffff_ffff; (*input).apic_ids[0]=apic_id; let output=*this_cpu_ptr(hyperv_pcpu_output_arg) as *mut u32; let status=hv_do_hypercall((1<<32)|0x0001_0002,input as *mut c_void,output); let ret=*output; local_irq_restore(flags); if !hv_result_success(status) { return -22; } ret as i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
