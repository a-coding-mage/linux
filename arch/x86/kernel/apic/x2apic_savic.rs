// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Secure AVIC Support (SEV-SNP Guests)
 *
 * Copyright (C) 2024 Advanced Micro Devices, Inc.
 *
 * Author: Neeraj Upadhyay <Neeraj.Upadhyay@amd.com>
 */

// C dependencies are supplied by the surrounding kernel translation unit.

#[repr(C, align(4096))]
struct secure_avic_page {
    regs: [u8; PAGE_SIZE],
}

static mut savic_page: *mut secure_avic_page = core::ptr::null_mut();

unsafe extern "C" {
    fn x2apic_enabled() -> bool;
    fn cc_platform_has(attr: u32) -> bool;
    fn per_cpu_ptr<T>(ptr: *mut T, cpu: u32) -> *mut T;
    fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn apic_set_vector(vector: u32, bitmap: *mut core::ffi::c_void);
    fn apic_clear_vector(vector: u32, bitmap: *mut core::ffi::c_void);
    fn savic_ghcb_msr_read(reg: u32) -> u32;
    fn apic_get_reg(ap: *mut core::ffi::c_void, reg: u32) -> u32;
    fn apic_get_reg64(ap: *mut core::ffi::c_void, reg: u32) -> u64;
    fn apic_set_reg(ap: *mut core::ffi::c_void, reg: u32, data: u32);
    fn apic_set_reg64(ap: *mut core::ffi::c_void, reg: u32, data: u64);
    fn savic_ghcb_msr_write(reg: u32, data: u64);
    fn native_apic_msr_write(reg: u32, data: u32);
    fn native_x2apic_icr_write(low: u32, high: u32);
    fn native_apic_msr_eoi();
    fn native_wrmsrq(msr: u32, value: u64);
    fn savic_unregister_gpa(gpa: *const core::ffi::c_void);
    fn __pa(ptr: *mut core::ffi::c_void) -> u64;
    fn savic_register_gpa(gpa: u64) -> es_result;
    fn sev_es_terminate(set: u32, reason: u32) -> !;
    fn alloc_percpu<T>() -> *mut T;
    fn apic_find_highest_vector(bitmap: *mut core::ffi::c_void) -> i32;
    fn apic_test_vector(vector: i32, bitmap: *mut core::ffi::c_void) -> bool;
    fn raw_smp_processor_id() -> u32;
    fn send_ipi(_: u32, _: u32, _: u32);
}

#[repr(C)]
enum es_result { ES_OK }

const SAVIC_ALLOWED_IRR: u32 = 0x204;
const SAVIC_NMI_REQ: u32 = 0x278;

unsafe fn get_reg_bitmap(cpu: u32, offset: u32) -> *mut core::ffi::c_void {
    (*per_cpu_ptr(savic_page, cpu)).regs.as_mut_ptr().add(offset as usize) as *mut _
}

unsafe fn update_vector(cpu: u32, offset: u32, vector: u32, set: bool) {
    let bitmap = get_reg_bitmap(cpu, offset);
    if set { apic_set_vector(vector, bitmap); } else { apic_clear_vector(vector, bitmap); }
}

unsafe fn savic_acpi_madt_oem_check(_: *mut i8, _: *mut i8) -> i32 {
    (x2apic_enabled() && cc_platform_has(CC_ATTR_SNP_SECURE_AVIC)) as i32
}

unsafe fn savic_read(reg: u32) -> u32 {
    let ap = this_cpu_ptr(savic_page) as *mut _ as *mut core::ffi::c_void;
    match reg {
        APIC_LVTT | APIC_TMICT | APIC_TMCCT | APIC_TDCR | APIC_LVTTHMR | APIC_LVTPC | APIC_LVT0 | APIC_LVT1 | APIC_LVTERR => savic_ghcb_msr_read(reg),
        APIC_ID | APIC_LVR | APIC_TASKPRI | APIC_ARBPRI | APIC_PROCPRI | APIC_LDR | APIC_SPIV | APIC_ESR | APIC_EFEAT | APIC_ECTRL | APIC_SEOI | APIC_IER => apic_get_reg(ap, reg),
        APIC_ICR => apic_get_reg64(ap, reg) as u32,
        _ if (APIC_ISR..=APIC_ISR + 0x70).contains(&reg) || (APIC_TMR..=APIC_TMR + 0x70).contains(&reg) => apic_get_reg(ap, reg),
        _ if (APIC_IRR..=APIC_IRR + 0x74).contains(&reg) => apic_get_reg(ap, reg),
        _ => 0,
    }
}

unsafe fn self_ipi_reg_write(vector: u32) { native_apic_msr_write(APIC_SELF_IPI, vector); }
unsafe fn send_ipi_dest(cpu: u32, vector: u32, nmi: bool) { if nmi { apic_set_reg(per_cpu_ptr(savic_page, cpu) as *mut _, SAVIC_NMI_REQ, 1); } else { update_vector(cpu, APIC_IRR, vector, true); } }
unsafe fn self_ipi(vector: u32, nmi: bool) { native_x2apic_icr_write(APIC_SELF_IPI | vector | if nmi { APIC_DM_NMI } else { 0 }, 0); }

unsafe fn savic_icr_write(icr_low: u32, icr_high: u32) {
    let dsh = icr_low & APIC_DEST_ALLBUT; let vector = icr_low & APIC_VECTOR_MASK; let nmi = (icr_low & APIC_DM_FIXED_MASK) == APIC_DM_NMI;
    match dsh { APIC_DEST_SELF => self_ipi(vector, nmi), APIC_DEST_ALLINC => { self_ipi(vector, nmi); send_ipi_allbut(vector, nmi); }, APIC_DEST_ALLBUT => send_ipi_allbut(vector, nmi), _ => send_ipi_dest(icr_high, vector, nmi) }
    let data = ((icr_high as u64) << 32) | icr_low as u64;
    if dsh != APIC_DEST_SELF { savic_ghcb_msr_write(APIC_ICR, data); }
    apic_set_reg64(this_cpu_ptr(savic_page) as *mut _, APIC_ICR, data);
}

unsafe fn send_ipi_allbut(vector: u32, nmi: bool) { let src = raw_smp_processor_id(); for cpu in 0..NR_CPUS { if cpu != src { send_ipi_dest(cpu, vector, nmi); } } }
unsafe fn savic_write(reg: u32, data: u32) { let ap = this_cpu_ptr(savic_page) as *mut _; match reg { APIC_LVTT|APIC_TMICT|APIC_TDCR|APIC_LVT0|APIC_LVT1|APIC_LVTTHMR|APIC_LVTPC|APIC_LVTERR => savic_ghcb_msr_write(reg,data as u64), APIC_ICR => savic_icr_write(data,0), APIC_SELF_IPI => self_ipi_reg_write(data), _ => apic_set_reg(ap,reg,data) } }

unsafe fn savic_setup() { let ap = this_cpu_ptr(savic_page) as *mut _; apic_set_reg(ap, APIC_ID, native_apic_msr_read(APIC_ID)); let gpa = __pa(ap); if savic_register_gpa(gpa) as u32 != ES_OK as u32 { sev_es_terminate(SEV_TERM_SET_LINUX, GHCB_TERM_SAVIC_FAIL); } native_wrmsrq(MSR_AMD64_SAVIC_CONTROL, gpa | MSR_AMD64_SAVIC_EN | MSR_AMD64_SAVIC_ALLOWEDNMI); }

unsafe fn savic_teardown() { native_wrmsrq(MSR_AMD64_SAVIC_CONTROL, 0); savic_unregister_gpa(core::ptr::null()); }
unsafe fn savic_probe() -> i32 {
    if !cc_platform_has(CC_ATTR_SNP_SECURE_AVIC) { return 0; }
    if !x2apic_mode { sev_es_terminate(SEV_TERM_SET_LINUX, GHCB_TERM_SAVIC_FAIL); }
    savic_page = alloc_percpu::<secure_avic_page>();
    if savic_page.is_null() { sev_es_terminate(SEV_TERM_SET_LINUX, GHCB_TERM_SAVIC_FAIL); }
    1
}

unsafe fn send_ipi_mask(mask: *const cpumask, vector: u32, excl_self: bool) {
    let this_cpu = raw_smp_processor_id();
    for cpu in 0..NR_CPUS { if (!excl_self || cpu != this_cpu) && cpumask_test_cpu(cpu, mask) { send_ipi(per_cpu(x86_cpu_to_apicid, cpu), vector, 0); } }
}
unsafe fn savic_send_ipi(cpu: i32, vector: i32) { send_ipi(per_cpu(x86_cpu_to_apicid, cpu as u32), vector as u32, 0); }
unsafe fn savic_send_ipi_mask(mask: *const cpumask, vector: i32) { send_ipi_mask(mask, vector as u32, false); }
unsafe fn savic_send_ipi_mask_allbutself(mask: *const cpumask, vector: i32) { send_ipi_mask(mask, vector as u32, true); }
unsafe fn savic_send_ipi_allbutself(vector: i32) { send_ipi(0, vector as u32, APIC_DEST_ALLBUT); }
unsafe fn savic_send_ipi_all(vector: i32) { send_ipi(0, vector as u32, APIC_DEST_ALLINC); }
unsafe fn savic_send_ipi_self(vector: i32) { self_ipi_reg_write(vector as u32); }
unsafe fn savic_update_vector(cpu: u32, vector: u32, set: bool) { update_vector(cpu, SAVIC_ALLOWED_IRR, vector, set); }
unsafe fn savic_eoi() { let cpu = raw_smp_processor_id(); let vec = apic_find_highest_vector(get_reg_bitmap(cpu, APIC_ISR)); if vec == -1 { return; } if apic_test_vector(vec, get_reg_bitmap(cpu, APIC_TMR)) { update_vector(cpu, APIC_ISR, vec as u32, false); savic_ghcb_msr_write(APIC_EOI, 0); } else { native_apic_msr_eoi(); } }

#[repr(C)]
struct apic_x2apic_savic {
    name: *const u8,
    probe: unsafe fn() -> i32,
    acpi_madt_oem_check: unsafe fn(*mut i8, *mut i8) -> i32,
    setup: unsafe fn(), teardown: unsafe fn(),
    dest_mode_logical: bool, disable_esr: u32,
    cpu_present_to_apicid: *const core::ffi::c_void,
    max_apic_id: u32, x2apic_set_max_apicid: bool,
    get_apic_id: *const core::ffi::c_void, calc_dest_apicid: *const core::ffi::c_void,
    send_IPI: unsafe fn(i32,i32), send_IPI_mask: unsafe fn(*const cpumask,i32),
    send_IPI_mask_allbutself: unsafe fn(*const cpumask,i32), send_IPI_allbutself: unsafe fn(i32),
    send_IPI_all: unsafe fn(i32), send_IPI_self: unsafe fn(i32), nmi_to_offline_cpu: bool,
    read: unsafe fn(u32)->u32, write: unsafe fn(u32,u32), eoi: unsafe fn(),
    icr_read: *const core::ffi::c_void, icr_write: unsafe fn(u32,u32),
    update_vector: unsafe fn(u32,u32,bool),
}

// C macros, globals, and APIC registration are provided by the surrounding kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
