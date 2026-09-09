/*
 * Switch a MMU context.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1996, 1997, 1998, 1999 by Ralf Baechle
 * Copyright (C) 1999 Silicon Graphics, Inc.
 */

// C header dependencies are supplied by the surrounding translation unit.

#[inline(always)]
pub unsafe fn htw_set_pwbase(pgd: usize) {
    if cpu_has_htw {
        write_c0_pwbase(pgd);
        back_to_back_c0_hazard();
    }
}

extern "C" {
    pub fn tlbmiss_handler_setup_pgd(arg: c_ulong);
    pub static mut tlbmiss_handler_setup_pgd_end: c_char;
}

/* Note: This is also implemented with uasm in arch/mips/kvm/entry.c */
#[inline(always)]
pub unsafe fn TLBMISS_HANDLER_SETUP_PGD(pgd: usize) {
    tlbmiss_handler_setup_pgd(pgd as c_ulong);
    htw_set_pwbase(pgd);
}

// CONFIG_MIPS_PGD_C0_CONTEXT selects the first implementation in the C header.
#[cfg(CONFIG_MIPS_PGD_C0_CONTEXT)]
#[inline(always)]
pub unsafe fn TLBMISS_HANDLER_RESTORE() {
    write_c0_xcontext((smp_processor_id() as c_ulong) << SMP_CPUID_REGSHIFT);
}

#[cfg(CONFIG_MIPS_PGD_C0_CONTEXT)]
#[inline(always)]
pub unsafe fn TLBMISS_HANDLER_SETUP() {
    TLBMISS_HANDLER_SETUP_PGD(swapper_pg_dir);
    TLBMISS_HANDLER_RESTORE();
}

/*
 * For the fast tlb miss handlers, we keep a per cpu array of pointers
 * to the current pgd for each processor. Also, the proc. id is stuffed
 * into the context register.
 */
#[cfg(not(CONFIG_MIPS_PGD_C0_CONTEXT))]
extern "C" {
    pub static mut pgd_current: [c_ulong; 0];
}

#[cfg(not(CONFIG_MIPS_PGD_C0_CONTEXT))]
#[inline(always)]
pub unsafe fn TLBMISS_HANDLER_RESTORE() {
    write_c0_context((smp_processor_id() as c_ulong) << SMP_CPUID_REGSHIFT);
}

#[cfg(not(CONFIG_MIPS_PGD_C0_CONTEXT))]
#[inline(always)]
pub unsafe fn TLBMISS_HANDLER_SETUP() {
    TLBMISS_HANDLER_RESTORE();
    back_to_back_c0_hazard();
    TLBMISS_HANDLER_SETUP_PGD(swapper_pg_dir);
}

pub const MMID_KERNEL_WIRED: u32 = 0;

#[inline(always)]
pub unsafe fn asid_version_mask(cpu: c_uint) -> u64 {
    let asid_mask: c_ulong = cpu_asid_mask(&cpu_data[cpu as usize]);
    !((asid_mask | (asid_mask.wrapping_sub(1))) as u64)
}

#[inline(always)]
pub unsafe fn asid_first_version(cpu: c_uint) -> u64 {
    (!asid_version_mask(cpu)).wrapping_add(1)
}

#[inline(always)]
pub unsafe fn cpu_context(cpu: c_uint, mm: *const mm_struct) -> u64 {
    if cpu_has_mmid {
        atomic64_read(&(*mm).context.mmid)
    } else {
        (*mm).context.asid[cpu as usize]
    }
}

#[inline(always)]
pub unsafe fn set_cpu_context(cpu: c_uint, mm: *mut mm_struct, ctx: u64) {
    if cpu_has_mmid {
        atomic64_set(&mut (*mm).context.mmid, ctx);
    } else {
        (*mm).context.asid[cpu as usize] = ctx;
    }
}

#[inline(always)]
pub unsafe fn asid_cache(cpu: usize) -> c_ulong {
    cpu_data[cpu].asid_cache
}

#[inline(always)]
pub unsafe fn cpu_asid(cpu: usize, mm: *const mm_struct) -> u64 {
    cpu_context(cpu as c_uint, mm) & cpu_asid_mask(&cpu_data[cpu]) as u64
}

extern "C" {
    pub fn get_new_mmu_context(mm: *mut mm_struct);
    pub fn check_mmu_context(mm: *mut mm_struct);
    pub fn check_switch_mmu_context(mm: *mut mm_struct);
}

/* Initialize the context related info for a new mm_struct instance. */
#[inline(always)]
pub unsafe fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> c_int {
    let mut i: c_int;
    if cpu_has_mmid {
        set_cpu_context(0, mm, 0);
    } else {
        for_each_possible_cpu!(i, set_cpu_context(i as c_uint, mm, 0));
    }
    (*mm).context.bd_emupage_allocmap = core::ptr::null_mut();
    spin_lock_init(&mut (*mm).context.bd_emupage_lock);
    init_waitqueue_head(&mut (*mm).context.bd_emupage_queue);
    0
}

#[inline(always)]
pub unsafe fn switch_mm(prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct) {
    let cpu = smp_processor_id();
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    htw_stop();
    check_switch_mmu_context(next);
    /* Mark current->active_mm as not "active" anymore. */
    cpumask_clear_cpu(cpu, mm_cpumask(prev));
    cpumask_set_cpu(cpu, mm_cpumask(next));
    htw_start();
    local_irq_restore(flags);
}

/* Destroy context related info for an mm_struct about to be put to rest. */
#[inline(always)]
pub unsafe fn destroy_context(mm: *mut mm_struct) {
    dsemul_mm_cleanup(mm);
}

#[inline(always)]
pub unsafe fn drop_mmu_context(mm: *mut mm_struct) {
    let mut flags: c_ulong = 0;
    let cpu: c_uint;
    let mut old_mmid: u32;
    let ctx: u64;
    local_irq_save(&mut flags);
    cpu = smp_processor_id();
    ctx = cpu_context(cpu, mm);
    if ctx != 0 {
        if cpu_has_mmid {
            htw_stop();
            old_mmid = read_c0_memorymapid();
            write_c0_memorymapid(ctx & cpu_asid_mask(&cpu_data[cpu as usize]) as u64);
            mtc0_tlbw_hazard();
            ginvt_mmid();
            sync_ginv();
            write_c0_memorymapid(old_mmid);
            instruction_hazard();
            htw_start();
        } else if cpumask_test_cpu(cpu, mm_cpumask(mm)) {
            htw_stop();
            get_new_mmu_context(mm);
            write_c0_entryhi(cpu_asid(cpu as usize, mm));
            htw_start();
        } else {
            set_cpu_context(cpu, mm, 0);
        }
    }
    local_irq_restore(flags);
}

// asm-generic/mmu_context.h supplies additional generic declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
