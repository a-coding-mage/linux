/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1999 Niibe Yutaka
 * Copyright (C) 2003 - 2007 Paul Mundt
 *
 * ASID handling idea taken from MIPS implementation.
 */

// C dependencies: <cpu/mmu_context.h>, <asm/tlbflush.h>, <linux/uaccess.h>,
// <linux/mm_types.h>, <asm/io.h>, <asm-generic/mm_hooks.h>.

/*
 * The MMU "context" consists of two things:
 *    (a) TLB cache version (or round, cycle whatever expression you like)
 *    (b) ASID (Address Space IDentifier)
 */
// When CONFIG_CPU_HAS_PTEAEX is enabled, use 0x0000ffff; otherwise use 0x000000ff.
#[cfg(CONFIG_CPU_HAS_PTEAEX)]
pub const MMU_CONTEXT_ASID_MASK: usize = 0x0000ffff;
#[cfg(not(CONFIG_CPU_HAS_PTEAEX))]
pub const MMU_CONTEXT_ASID_MASK: usize = 0x000000ff;

pub const MMU_CONTEXT_VERSION_MASK: usize = !MMU_CONTEXT_ASID_MASK;
pub const MMU_CONTEXT_FIRST_VERSION: usize = MMU_CONTEXT_ASID_MASK + 1;

/* Impossible ASID value, to differentiate from NO_CONTEXT. */
pub const MMU_NO_ASID: usize = MMU_CONTEXT_FIRST_VERSION;
pub const NO_CONTEXT: usize = 0;

/* C macro: asid_cache(cpu) (cpu_data[cpu].asid_cache) */

// Only meaningful when CONFIG_MMU is enabled.
#[cfg(CONFIG_MMU)]
pub const MMU_VPN_MASK: usize = 0xfffff000;

#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn get_mmu_context(mm: *mut mm_struct, cpu: core::ffi::c_uint) {
    let mut asid: usize = asid_cache(cpu);

    /* Check if we have old version of context. */
    if (((cpu_context(cpu, mm) ^ asid) & MMU_CONTEXT_VERSION_MASK) == 0) {
        /* It's up to date, do nothing */
        return;
    }

    /* It's old, we need to get new context with new version. */
    asid = asid.wrapping_add(1);
    if (asid & MMU_CONTEXT_ASID_MASK) == 0 {
        /*
         * We exhaust ASID of this version.
         * Flush all TLB and start new cycle.
         */
        local_flush_tlb_all();

        /*
         * Fix version; Note that we avoid version #0
         * to distinguish NO_CONTEXT.
         */
        if asid == 0 {
            asid = MMU_CONTEXT_FIRST_VERSION;
        }
    }

    asid_cache(cpu) = asid;
    cpu_context(cpu, mm) = asid;
}

/*
 * Initialize the context related info for a new mm_struct
 * instance.
 */
#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;

    for_each_online_cpu!(i, {
        cpu_context(i, mm) = NO_CONTEXT;
    });

    0
}

/*
 * After we have set current->mm to a new value, this activates
 * the context for the new mm so we see the new mappings.
 */
#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn activate_context(mm: *mut mm_struct, cpu: core::ffi::c_uint) {
    get_mmu_context(mm, cpu);
    set_asid(cpu_asid(cpu, mm));
}

#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
    let cpu: core::ffi::c_uint = smp_processor_id();

    if likely(prev != next) {
        cpumask_set_cpu(cpu, mm_cpumask(next));
        set_TTB((*next).pgd);
        activate_context(next, cpu);
    } else if !cpumask_test_and_set_cpu(cpu, mm_cpumask(next)) {
        activate_context(next, cpu);
    }
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_asid(_asid: usize) {}
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn get_asid() -> usize { 0 }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn cpu_asid(_cpu: core::ffi::c_uint, _mm: *mut mm_struct) -> usize { NO_CONTEXT }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn switch_and_save_asid(_asid: usize) -> usize { 0 }
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_TTB<T>(_pgd: *mut T) {}
#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn get_TTB() -> usize { 0 }

// <asm-generic/mmu_context.h> or <asm-generic/nommu_context.h> is required here.

// MMU control handlers for CONFIG_CPU_SH3 or CONFIG_CPU_SH4.
#[cfg(any(CONFIG_CPU_SH3, CONFIG_CPU_SH4))]
#[inline]
pub unsafe fn enable_mmu() {
    let cpu: core::ffi::c_uint = smp_processor_id();

    /* Enable MMU */
    __raw_writel(MMU_CONTROL_INIT, MMUCR);
    ctrl_barrier();

    if asid_cache(cpu) == NO_CONTEXT {
        asid_cache(cpu) = MMU_CONTEXT_FIRST_VERSION;
    }

    set_asid(asid_cache(cpu) & MMU_CONTEXT_ASID_MASK);
}

#[cfg(any(CONFIG_CPU_SH3, CONFIG_CPU_SH4))]
#[inline]
pub unsafe fn disable_mmu() {
    let mut cr: usize;

    cr = __raw_readl(MMUCR);
    cr &= !MMU_CONTROL_INIT;
    __raw_writel(cr, MMUCR);

    ctrl_barrier();
}

/*
 * MMU control handlers for processors lacking memory
 * management hardware.
 */
#[cfg(not(any(CONFIG_CPU_SH3, CONFIG_CPU_SH4)))]
#[inline]
pub unsafe fn enable_mmu() {}
#[cfg(not(any(CONFIG_CPU_SH3, CONFIG_CPU_SH4)))]
#[inline]
pub unsafe fn disable_mmu() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
