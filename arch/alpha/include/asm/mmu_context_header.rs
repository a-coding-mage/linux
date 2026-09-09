/* SPDX-License-Identifier: GPL-2.0 */

/*
 * get a new mmu context..
 *
 * Copyright (C) 1996, Linus Torvalds
 */

/* Dependencies supplied by the corresponding Linux/Alpha headers. */

/*
 * Force a context reload. This is needed when we change the page
 * table pointer or when we update the ASN of the current process.
 */

pub unsafe extern "C" fn __reload_thread(pcb: *mut pcb_struct) -> ::core::primitive::usize {
    let mut a0: ::core::primitive::usize = virt_to_phys(pcb);
    let mut v0: ::core::primitive::usize;
    ::core::arch::asm!(
        "call_pal {pal} #__reload_thread",
        pal = const PAL_swpctx,
        inout("$16") a0,
        lateout("$0") v0,
        clobber_abi("C"),
    );
    v0
}

/* The maximum ASN values supported by the processor variants. */
pub const EV4_MAX_ASN: ::core::primitive::usize = 63;
pub const EV5_MAX_ASN: ::core::primitive::usize = 127;
pub const EV6_MAX_ASN: ::core::primitive::usize = 255;

/* CONFIG_ALPHA_GENERIC selects alpha_mv.max_asn at build time. */
#[cfg(CONFIG_ALPHA_GENERIC)]
pub const MAX_ASN: ::core::primitive::usize = alpha_mv.max_asn;
#[cfg(all(not(CONFIG_ALPHA_GENERIC), CONFIG_ALPHA_EV56))]
pub const MAX_ASN: ::core::primitive::usize = EV5_MAX_ASN;
#[cfg(all(not(CONFIG_ALPHA_GENERIC), not(CONFIG_ALPHA_EV56)))]
pub const MAX_ASN: ::core::primitive::usize = EV6_MAX_ASN;

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn cpu_last_asn(cpuid: ::core::primitive::usize) -> ::core::primitive::usize {
    cpu_data[cpuid].last_asn
}
#[cfg(not(CONFIG_SMP))]
unsafe extern "C" {
    pub static mut last_asn: ::core::primitive::usize;
}
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn cpu_last_asn(_cpuid: ::core::primitive::usize) -> ::core::primitive::usize {
    last_asn
}

pub const WIDTH_HARDWARE_ASN: ::core::primitive::usize = 8;
pub const ASN_FIRST_VERSION: ::core::primitive::usize = 1usize << WIDTH_HARDWARE_ASN;
pub const HARDWARE_ASN_MASK: ::core::primitive::usize = (1usize << WIDTH_HARDWARE_ASN) - 1;

#[inline]
pub unsafe fn __get_new_mm_context(mm: *mut mm_struct, cpu: ::core::primitive::isize) -> ::core::primitive::usize {
    let asn = cpu_last_asn(cpu as usize);
    let mut next = asn.wrapping_add(1);
    if (asn & HARDWARE_ASN_MASK) >= MAX_ASN {
        tbiap();
        imb();
        next = (asn & !HARDWARE_ASN_MASK).wrapping_add(ASN_FIRST_VERSION);
    }
    cpu_last_asn_set(cpu as usize, next);
    let _ = mm;
    next
}

#[cfg(CONFIG_SMP)]
#[inline]
unsafe fn cpu_last_asn_set(cpuid: usize, value: usize) { cpu_data[cpuid].last_asn = value; }
#[cfg(not(CONFIG_SMP))]
#[inline]
unsafe fn cpu_last_asn_set(_cpuid: usize, value: usize) { last_asn = value; }

#[inline]
pub unsafe fn ev5_switch_mm(prev_mm: *mut mm_struct, next_mm: *mut mm_struct, next: *mut task_struct) {
    let _ = prev_mm;
    let cpu = smp_processor_id();
    #[cfg(CONFIG_SMP)]
    { cpu_data[cpu].asn_lock = 1; barrier(); }
    let asn = cpu_last_asn(cpu);
    let mut mmc = (*next_mm).context[cpu];
    if (mmc ^ asn) & !HARDWARE_ASN_MASK != 0 {
        mmc = __get_new_mm_context(next_mm, cpu as isize);
        (*next_mm).context[cpu] = mmc;
    }
    #[cfg(CONFIG_SMP)]
    if (mmc ^ asn) & !HARDWARE_ASN_MASK == 0 { cpu_data[cpu].need_new_asn = 1; }
    task_thread_info(next).pcb.asn = mmc & HARDWARE_ASN_MASK;
}

unsafe extern "C" {
    pub fn __load_new_mm_context(mm: *mut mm_struct);
    pub fn do_page_fault(address: usize, mmcsr: usize, cause: isize, regs: *mut pt_regs);
}

#[cfg(CONFIG_SMP)]
#[inline]
pub unsafe fn check_mmu_context() {
    let cpu = smp_processor_id();
    cpu_data[cpu].asn_lock = 0;
    barrier();
    if cpu_data[cpu].need_new_asn {
        let mm = (*current).active_mm;
        cpu_data[cpu].need_new_asn = 0;
        if !(*mm).context[cpu] { __load_new_mm_context(mm); }
    }
}
#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn check_mmu_context() {}

#[inline]
pub unsafe fn ev5_activate_mm(prev_mm: *mut mm_struct, next_mm: *mut mm_struct) {
    let _ = prev_mm;
    __load_new_mm_context(next_mm);
}

#[inline]
pub unsafe fn switch_mm(a: *mut mm_struct, b: *mut mm_struct, c: *mut task_struct) { ev5_switch_mm(a, b, c); }
#[inline]
pub unsafe fn activate_mm(x: *mut mm_struct, y: *mut mm_struct) { ev5_activate_mm(x, y); }

#[inline]
pub unsafe fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> ::core::primitive::isize {
    for i in for_each_online_cpu() { (*mm).context[i] = 0; }
    if tsk != current {
        task_thread_info(tsk).pcb.ptbr = (((*mm).pgd as usize).wrapping_sub(IDENT_ADDR)) >> PAGE_SHIFT;
    }
    0
}

#[inline]
pub unsafe fn enter_lazy_tlb(mm: *mut mm_struct, tsk: *mut task_struct) {
    task_thread_info(tsk).pcb.ptbr = (((*mm).pgd as usize).wrapping_sub(IDENT_ADDR)) >> PAGE_SHIFT;
}

/* asm-generic/mmu_context.h supplies the remaining generic context items. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
