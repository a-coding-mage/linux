/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Switch a MMU context.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
pub unsafe fn asid_version_mask(cpu: core::ffi::c_uint) -> u64 {
    !(cpu_asid_mask(&cpu_data[cpu as usize]) as u64)
}

#[inline]
pub unsafe fn asid_first_version(cpu: core::ffi::c_uint) -> u64 {
    cpu_asid_mask(&cpu_data[cpu as usize]) as u64 + 1
}

#[inline]
pub unsafe fn cpu_context(cpu: core::ffi::c_uint, mm: *mut mm_struct) -> &mut u64 {
    &mut (*mm).context.asid[cpu as usize]
}

#[inline]
pub unsafe fn asid_cache(cpu: core::ffi::c_uint) -> &mut u64 {
    &mut cpu_data[cpu as usize].asid_cache
}

#[inline]
pub unsafe fn cpu_asid(cpu: core::ffi::c_uint, mm: *mut mm_struct) -> u64 {
    *cpu_context(cpu, mm) & cpu_asid_mask(&cpu_data[cpu as usize]) as u64
}

#[inline]
pub unsafe fn asid_valid(mm: *mut mm_struct, cpu: core::ffi::c_uint) -> i32 {
    if (*cpu_context(cpu, mm) ^ *asid_cache(cpu)) & asid_version_mask(cpu) != 0 {
        0
    } else {
        1
    }
}

#[inline]
pub unsafe fn enter_lazy_tlb(_mm: *mut mm_struct, _tsk: *mut task_struct) {}

/* Normal, classic get_new_mmu_context */
#[inline]
pub unsafe fn get_new_mmu_context(
    mm: *mut mm_struct,
    cpu: core::ffi::c_ulong,
    need_flush: *mut bool,
) {
    let mut asid = *asid_cache(cpu as core::ffi::c_uint);
    asid = asid.wrapping_add(1);

    if asid & cpu_asid_mask(&cpu_data[cpu as usize]) as u64 == 0 {
        *need_flush = true; /* start new asid cycle */
    }

    *cpu_context(cpu as core::ffi::c_uint, mm) = asid;
    *asid_cache(cpu as core::ffi::c_uint) = asid;
}

/*
 * Initialize the context related info for a new mm_struct
 * instance.
 */
#[inline]
pub unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    let mut i: i32 = 0;
    for_each_possible_cpu!(i);
    *cpu_context(i as core::ffi::c_uint, mm) = 0;
    0
}

#[inline]
pub unsafe fn atomic_update_pgd_asid(mut asid: core::ffi::c_ulong, mut pgdl: core::ffi::c_ulong) {
    core::arch::asm!(
        "csrwr {pgdl_val}, {pgdl_reg}",
        "csrwr {asid_val}, {asid_reg}",
        asid_val = inout(reg) asid,
        pgdl_val = inout(reg) pgdl,
        asid_reg = const LOONGARCH_CSR_ASID,
        pgdl_reg = const LOONGARCH_CSR_PGDL,
        options(nostack)
    );
}

#[inline]
pub unsafe fn switch_mm_irqs_off(
    _prev: *mut mm_struct,
    next: *mut mm_struct,
    _tsk: *mut task_struct,
) {
    let mut need_flush = false;
    let cpu = smp_processor_id();

    /* Check if our ASID is of an older version and thus invalid */
    if asid_valid(next, cpu) == 0 {
        get_new_mmu_context(next, cpu as core::ffi::c_ulong, &mut need_flush);
    }

    if next != &mut init_mm as *mut mm_struct {
        atomic_update_pgd_asid(cpu_asid(cpu, next) as core::ffi::c_ulong, (*next).pgd as core::ffi::c_ulong);
    } else {
        atomic_update_pgd_asid(cpu_asid(cpu, next) as core::ffi::c_ulong, invalid_pg_dir as core::ffi::c_ulong);
    }

    if need_flush {
        local_flush_tlb_user(); /* Flush tlb after update ASID */
    }

    /*
     * Mark current->active_mm as not "active" anymore.
     * We don't want to mislead possible IPI tlb flush routines.
     */
    cpumask_set_cpu(cpu, mm_cpumask(next));
}

#[inline]
pub unsafe fn switch_mm(prev: *mut mm_struct, next: *mut mm_struct, tsk: *mut task_struct) {
    let mut flags: core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    switch_mm_irqs_off(prev, next, tsk);
    local_irq_restore(flags);
}

/* Destroy context related info for an mm_struct that is about to be put to rest. */
#[inline]
pub unsafe fn destroy_context(_mm: *mut mm_struct) {}

#[inline]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    switch_mm(prev, next, current);
}

#[inline]
pub unsafe fn deactivate_mm(_task: *mut task_struct, _mm: *mut mm_struct) {}

/* If mm is currently active, we can't really drop it. Instead, we will get a new one for it. */
#[inline]
pub unsafe fn drop_mmu_context(mm: *mut mm_struct, cpu: core::ffi::c_uint) {
    let mut flags: core::ffi::c_ulong = 0;
    local_irq_save(&mut flags);

    let asid = read_csr_asid() & cpu_asid_mask(&current_cpu_data);
    if asid == cpu_asid(cpu, mm) {
        let mut need_flush = false;
        if (*current).mm.is_null() || (*current).mm == mm {
            get_new_mmu_context(mm, cpu as core::ffi::c_ulong, &mut need_flush);
            write_csr_asid(cpu_asid(cpu, mm));
            if need_flush {
                local_flush_tlb_user(); /* Flush tlb after update ASID */
            }
            local_irq_restore(flags);
            return;
        }
    }

    /* Will get a new context next time */
    *cpu_context(cpu, mm) = 0;
    cpumask_clear_cpu(cpu, mm_cpumask(mm));
    local_irq_restore(flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
