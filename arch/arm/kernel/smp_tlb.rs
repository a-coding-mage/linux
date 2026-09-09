// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/smp_tlb.c
 *
 *  Copyright (C) 2002 ARM Limited, All Rights Reserved.
 */

/* TLB operations. */
#[repr(C)]
pub struct tlb_args {
    pub ta_vma: *mut vm_area_struct,
    pub ta_start: c_ulong,
    pub ta_end: c_ulong,
}

extern "C" {
    fn local_flush_tlb_all();
    fn local_flush_tlb_mm(mm: *mut mm_struct);
    fn local_flush_tlb_page(vma: *mut vm_area_struct, address: c_ulong);
    fn local_flush_tlb_kernel_page(address: c_ulong);
    fn local_flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
    fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
    fn local_flush_bp_all();
    fn uaccess_save_and_enable() -> c_uint;
    fn uaccess_restore(flags: c_uint);
    fn tlb_ops_need_broadcast() -> bool;
    fn on_each_cpu(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn on_each_cpu_mask(mask: *const cpumask_t, func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn mm_cpumask(mm: *mut mm_struct) -> *const cpumask_t;
    fn __flush_tlb_all();
    fn __flush_tlb_mm(mm: *mut mm_struct);
    fn __flush_tlb_page(vma: *mut vm_area_struct, address: c_ulong);
    fn __flush_tlb_kernel_page(address: c_ulong);
    fn __flush_bp_all();
    fn smp_call_function(func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn smp_call_function_many(mask: *const cpumask_t, func: unsafe extern "C" fn(*mut c_void), info: *mut c_void, wait: c_int);
    fn get_cpu() -> c_int;
    fn put_cpu();
    fn a15_erratum_get_cpumask(cpu: c_int, mm: *mut mm_struct, mask: *mut cpumask_t);
    fn dmb();
    fn dsb(domain: c_int);
    fn read_cpuid_id() -> c_uint;
    fn read_cpuid(reg: c_uint) -> c_uint;
    fn erratum_a15_798181() -> bool;

    #[cfg(CONFIG_ARM_ERRATA_798181)]
    static mut erratum_a15_798181_handler: Option<unsafe extern "C" fn() -> bool>;
}

type c_ulong = usize;
type c_uint = u32;
type c_int = i32;
type c_void = core::ffi::c_void;

#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct cpumask_t { pub bits: [usize; 1] }
const CPU_BITS_NONE: usize = 0;
const CPUID_REVIDR: c_uint = 0;

unsafe extern "C" fn ipi_flush_tlb_all(_ignored: *mut c_void) { local_flush_tlb_all(); }

unsafe extern "C" fn ipi_flush_tlb_mm(arg: *mut c_void) {
    local_flush_tlb_mm(arg as *mut mm_struct);
}

unsafe extern "C" fn ipi_flush_tlb_page(arg: *mut c_void) {
    let ta = &*(arg as *mut tlb_args);
    let flags = uaccess_save_and_enable();
    local_flush_tlb_page(ta.ta_vma, ta.ta_start);
    uaccess_restore(flags);
}

unsafe extern "C" fn ipi_flush_tlb_kernel_page(arg: *mut c_void) {
    let ta = &*(arg as *mut tlb_args);
    local_flush_tlb_kernel_page(ta.ta_start);
}

unsafe extern "C" fn ipi_flush_tlb_range(arg: *mut c_void) {
    let ta = &*(arg as *mut tlb_args);
    let flags = uaccess_save_and_enable();
    local_flush_tlb_range(ta.ta_vma, ta.ta_start, ta.ta_end);
    uaccess_restore(flags);
}

unsafe extern "C" fn ipi_flush_tlb_kernel_range(arg: *mut c_void) {
    let ta = &*(arg as *mut tlb_args);
    local_flush_tlb_kernel_range(ta.ta_start, ta.ta_end);
}

unsafe extern "C" fn ipi_flush_bp_all(_ignored: *mut c_void) { local_flush_bp_all(); }

#[cfg(CONFIG_ARM_ERRATA_798181)]
unsafe extern "C" fn erratum_a15_798181_partial() -> bool {
    core::arch::asm!("mcr p15, 0, {0}, c8, c3, 1", in(reg) 0);
    dsb(0);
    false
}

#[cfg(CONFIG_ARM_ERRATA_798181)]
unsafe extern "C" fn erratum_a15_798181_broadcast() -> bool {
    core::arch::asm!("mcr p15, 0, {0}, c8, c3, 1", in(reg) 0);
    dsb(0);
    true
}

#[cfg(CONFIG_ARM_ERRATA_798181)]
pub unsafe extern "C" fn erratum_a15_798181_init() {
    let midr = read_cpuid_id();
    let revidr = read_cpuid(CPUID_REVIDR);
    if (midr & 0xff0ffff0) == 0x420f00f0 && midr <= 0x420f00f2 {
        erratum_a15_798181_handler = Some(erratum_a15_798181_broadcast);
    } else if (midr & 0xff0ffff0) == 0x410fc0f0 && midr < 0x412fc0f2 {
        erratum_a15_798181_handler = Some(erratum_a15_798181_broadcast);
    } else if (midr & 0xff0ffff0) == 0x410fc0f0 && midr < 0x412fc0f4 {
        if revidr & 0x10 != 0 { erratum_a15_798181_handler = Some(erratum_a15_798181_partial); }
        else { erratum_a15_798181_handler = Some(erratum_a15_798181_broadcast); }
    } else if (midr & 0xff0ffff0) == 0x410fc0f0 && midr < 0x413fc0f3 {
        if revidr & 0x210 == 0 { erratum_a15_798181_handler = Some(erratum_a15_798181_broadcast); }
        else if revidr & 0x10 != 0 { erratum_a15_798181_handler = Some(erratum_a15_798181_partial); }
    } else if (midr & 0xff0ffff0) == 0x410fc0f0 && midr < 0x414fc0f0 {
        if revidr & 0x200 == 0 { erratum_a15_798181_handler = Some(erratum_a15_798181_partial); }
    }
}

unsafe extern "C" fn ipi_flush_tlb_a15_erratum(_arg: *mut c_void) { dmb(); }

unsafe fn broadcast_tlb_a15_erratum() {
    if !erratum_a15_798181() { return; }
    smp_call_function(ipi_flush_tlb_a15_erratum, core::ptr::null_mut(), 1);
}

unsafe fn broadcast_tlb_mm_a15_erratum(mm: *mut mm_struct) {
    if !erratum_a15_798181() { return; }
    let this_cpu = get_cpu();
    let mut mask = cpumask_t { bits: [CPU_BITS_NONE] };
    a15_erratum_get_cpumask(this_cpu, mm, &mut mask);
    smp_call_function_many(&mask, ipi_flush_tlb_a15_erratum, core::ptr::null_mut(), 1);
    put_cpu();
}

pub unsafe extern "C" fn flush_tlb_all() {
    if tlb_ops_need_broadcast() { on_each_cpu(ipi_flush_tlb_all, core::ptr::null_mut(), 1); }
    else { __flush_tlb_all(); }
    broadcast_tlb_a15_erratum();
}

pub unsafe extern "C" fn flush_tlb_mm(mm: *mut mm_struct) {
    if tlb_ops_need_broadcast() { on_each_cpu_mask(mm_cpumask(mm), ipi_flush_tlb_mm, mm as *mut c_void, 1); }
    else { __flush_tlb_mm(mm); }
    broadcast_tlb_mm_a15_erratum(mm);
}

pub unsafe extern "C" fn flush_tlb_page(vma: *mut vm_area_struct, uaddr: c_ulong) {
    if tlb_ops_need_broadcast() {
        let mut ta = tlb_args { ta_vma: vma, ta_start: uaddr, ta_end: 0 };
        on_each_cpu_mask(mm_cpumask(*(vma as *mut *mut mm_struct)), ipi_flush_tlb_page, &mut ta as *mut _ as *mut c_void, 1);
    } else { __flush_tlb_page(vma, uaddr); }
    broadcast_tlb_mm_a15_erratum(*(vma as *mut *mut mm_struct));
}

pub unsafe extern "C" fn flush_tlb_kernel_page(kaddr: c_ulong) {
    if tlb_ops_need_broadcast() { let mut ta = tlb_args { ta_vma: core::ptr::null_mut(), ta_start: kaddr, ta_end: 0 }; on_each_cpu(ipi_flush_tlb_kernel_page, &mut ta as *mut _ as *mut c_void, 1); }
    else { __flush_tlb_kernel_page(kaddr); }
    broadcast_tlb_a15_erratum();
}

pub unsafe extern "C" fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) {
    if tlb_ops_need_broadcast() { let mut ta = tlb_args { ta_vma: vma, ta_start: start, ta_end: end }; on_each_cpu_mask(mm_cpumask(*(vma as *mut *mut mm_struct)), ipi_flush_tlb_range, &mut ta as *mut _ as *mut c_void, 1); }
    else { local_flush_tlb_range(vma, start, end); }
    broadcast_tlb_mm_a15_erratum(*(vma as *mut *mut mm_struct));
}

pub unsafe extern "C" fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    if tlb_ops_need_broadcast() { let mut ta = tlb_args { ta_vma: core::ptr::null_mut(), ta_start: start, ta_end: end }; on_each_cpu(ipi_flush_tlb_kernel_range, &mut ta as *mut _ as *mut c_void, 1); }
    else { local_flush_tlb_kernel_range(start, end); }
    broadcast_tlb_a15_erratum();
}

pub unsafe extern "C" fn flush_bp_all() {
    if tlb_ops_need_broadcast() { on_each_cpu(ipi_flush_bp_all, core::ptr::null_mut(), 1); }
    else { __flush_bp_all(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
