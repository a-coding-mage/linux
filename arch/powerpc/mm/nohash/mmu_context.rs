// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for handling the MMU on those
 * PowerPC implementations where the MMU is not using the hash
 * table, such as 8xx, 4xx, BookE's etc...
 *
 * Copyright 2008 Ben Herrenschmidt <benh@kernel.crashing.org>
 *                IBM Corp.
 *
 * Derived from previous arch/powerpc/mm/mmu_context.c
 * and arch/powerpc/include/asm/mmu_context.h
 */

// Includes and build-time configuration are supplied by the kernel crate.

/* Room for two PTE table pointers, usually the kernel and current user
 * pointer to their respective root page table (pgdir).
 */
pub static mut abatron_pteptrs: [*mut core::ffi::c_void; 2] = [core::ptr::null_mut(); 2];

// Configuration selects LAST_CONTEXT: 16 for PPC_8xx, 65535 for PPC_47x,
// and 255 otherwise.
const FIRST_CONTEXT: u32 = 1;
#[cfg(CONFIG_PPC_8xx)]
const LAST_CONTEXT: u32 = 16;
#[cfg(all(not(CONFIG_PPC_8xx), CONFIG_PPC_47x))]
const LAST_CONTEXT: u32 = 65535;
#[cfg(all(not(CONFIG_PPC_8xx), not(CONFIG_PPC_47x)))]
const LAST_CONTEXT: u32 = 255;

static mut next_context: u32 = 0;
static mut nr_free_contexts: u32 = 0;
static mut context_map: *mut libc::c_ulong = core::ptr::null_mut();
static mut stale_map: [*mut libc::c_ulong; NR_CPUS] = [core::ptr::null_mut(); NR_CPUS];
static mut context_mm: *mut *mut mm_struct = core::ptr::null_mut();
static mut context_lock: raw_spinlock_t = RAW_SPINLOCK_INIT;

const CTX_MAP_SIZE: usize = core::mem::size_of::<libc::c_ulong>()
    * (LAST_CONTEXT as usize / BITS_PER_LONG + 1);

/* Steal a context from a task that has one at the moment. */
unsafe fn steal_context_smp(mut id: u32) -> u32 {
    let mut mm: *mut mm_struct;
    let mut cpu: u32;
    let mut max = LAST_CONTEXT - FIRST_CONTEXT;
    let mut i: u32;

    while max != 0 {
        max -= 1;
        mm = *context_mm.add(id as usize);
        if (*mm).context.active != 0 {
            id += 1;
            if id > LAST_CONTEXT { id = FIRST_CONTEXT; }
            continue;
        }
        (*mm).context.id = MMU_NO_CONTEXT;
        for_each_cpu!(cpu, mm_cpumask(mm));
        {
            i = cpu_first_thread_sibling(cpu);
            while i <= cpu_last_thread_sibling(cpu) {
                if !stale_map[i as usize].is_null() {
                    __set_bit(id, stale_map[i as usize]);
                }
                i += 1;
            }
            cpu = i - 1;
        }
        return id;
    }
    raw_spin_unlock(&mut context_lock);
    cpu_relax();
    raw_spin_lock(&mut context_lock);
    MMU_NO_CONTEXT
}

unsafe fn steal_all_contexts() -> u32 {
    let cpu = smp_processor_id();
    let mut id = FIRST_CONTEXT;
    while id <= LAST_CONTEXT {
        let mm = *context_mm.add(id as usize);
        (*mm).context.id = MMU_NO_CONTEXT;
        if id != FIRST_CONTEXT {
            *context_mm.add(id as usize) = core::ptr::null_mut();
            __clear_bit(id, context_map);
        }
        if IS_ENABLED!(CONFIG_SMP) { __clear_bit(id, stale_map[cpu as usize]); }
        id += 1;
    }
    _tlbil_all();
    nr_free_contexts = LAST_CONTEXT - FIRST_CONTEXT;
    FIRST_CONTEXT
}

unsafe fn steal_context_up(id: u32) -> u32 {
    let cpu = smp_processor_id();
    let mm = *context_mm.add(id as usize);
    local_flush_tlb_mm(mm);
    (*mm).context.id = MMU_NO_CONTEXT;
    if IS_ENABLED!(CONFIG_SMP) { __clear_bit(id, stale_map[cpu as usize]); }
    id
}

unsafe fn set_context(id: u64, pgd: *mut pgd_t) {
    if IS_ENABLED!(CONFIG_PPC_8xx) {
        mtspr(SPRN_M_TWB, __pa(pgd));
        mtspr(SPRN_M_CASID, id - 1);
        mb();
    } else if kuap_is_disabled() {
        mtspr(SPRN_PID, id);
        isync();
    }
}

pub unsafe fn switch_mmu_context(prev: *mut mm_struct, next: *mut mm_struct,
                                 tsk: *mut task_struct) {
    let mut id: u32;
    let mut i: u32;
    let cpu = smp_processor_id();
    let map: *mut libc::c_ulong;

    raw_spin_lock(&mut context_lock);
    if IS_ENABLED!(CONFIG_SMP) {
        (*next).context.active += 1;
        if !prev.is_null() {
            WARN_ON((*prev).context.active < 1);
            (*prev).context.active -= 1;
        }
    }
    loop {
        id = (*next).context.id;
        if likely(id != MMU_NO_CONTEXT) { break; }
        id = next_context;
        if id > LAST_CONTEXT { id = FIRST_CONTEXT; }
        map = context_map;
        if nr_free_contexts == 0 {
            if num_online_cpus() > 1 {
                id = steal_context_smp(id);
                if id == MMU_NO_CONTEXT { continue; }
            } else if IS_ENABLED!(CONFIG_PPC_8xx) {
                id = steal_all_contexts();
            } else {
                id = steal_context_up(id);
            }
        } else {
            nr_free_contexts -= 1;
            while __test_and_set_bit(id, map) != 0 {
                id = find_next_zero_bit(map, LAST_CONTEXT + 1, id);
                if id > LAST_CONTEXT { id = FIRST_CONTEXT; }
            }
        }
        next_context = id + 1;
        *context_mm.add(id as usize) = next;
        (*next).context.id = id;
        break;
    }
    if IS_ENABLED!(CONFIG_SMP) && test_bit(id, stale_map[cpu as usize]) != 0 {
        local_flush_tlb_mm(next);
        i = cpu_first_thread_sibling(cpu);
        while i <= cpu_last_thread_sibling(cpu) {
            if !stale_map[i as usize].is_null() { __clear_bit(id, stale_map[i as usize]); }
            i += 1;
        }
    }
    if IS_ENABLED!(CONFIG_BDI_SWITCH) { abatron_pteptrs[1] = (*next).pgd as *mut core::ffi::c_void; }
    set_context(id as u64, (*next).pgd);
    #[cfg(all(CONFIG_BOOKE, CONFIG_PPC_KUAP))]
    { (*tsk).thread.pid = id; }
    raw_spin_unlock(&mut context_lock);
}

pub unsafe fn init_new_context(_t: *mut task_struct, mm: *mut mm_struct) -> i32 {
    (*mm).context.id = MMU_NO_CONTEXT;
    (*mm).context.active = 0;
    pte_frag_set(&mut (*mm).context, core::ptr::null_mut());
    0
}

pub unsafe fn destroy_context(mm: *mut mm_struct) {
    if (*mm).context.id == MMU_NO_CONTEXT { return; }
    WARN_ON((*mm).context.active != 0);
    let mut flags: libc::c_ulong = 0;
    raw_spin_lock_irqsave(&mut context_lock, &mut flags);
    let id = (*mm).context.id;
    if id != MMU_NO_CONTEXT {
        __clear_bit(id, context_map);
        (*mm).context.id = MMU_NO_CONTEXT;
        *context_mm.add(id as usize) = core::ptr::null_mut();
        nr_free_contexts += 1;
    }
    raw_spin_unlock_irqrestore(&mut context_lock, flags);
}

unsafe fn mmu_ctx_cpu_prepare(cpu: u32) -> i32 {
    if cpu == boot_cpuid { return 0; }
    stale_map[cpu as usize] = kzalloc(CTX_MAP_SIZE, GFP_KERNEL);
    0
}

unsafe fn mmu_ctx_cpu_dead(cpu: u32) -> i32 {
    #[cfg(CONFIG_HOTPLUG_CPU)]
    {
        if cpu == boot_cpuid { return 0; }
        kfree(stale_map[cpu as usize]);
        stale_map[cpu as usize] = core::ptr::null_mut();
        clear_tasks_mm_cpumask(cpu);
    }
    0
}

pub unsafe fn mmu_context_init() {
    init_mm.context.active = NR_CPUS;
    context_map = memblock_alloc_or_panic(CTX_MAP_SIZE, SMP_CACHE_BYTES);
    context_mm = memblock_alloc_or_panic(core::mem::size_of::<*mut core::ffi::c_void>() * (LAST_CONTEXT as usize + 1), SMP_CACHE_BYTES);
    if IS_ENABLED!(CONFIG_SMP) {
        stale_map[boot_cpuid as usize] = memblock_alloc_or_panic(CTX_MAP_SIZE, SMP_CACHE_BYTES);
        cpuhp_setup_state_nocalls(CPUHP_POWERPC_MMU_CTX_PREPARE, "powerpc/mmu/ctx:prepare", mmu_ctx_cpu_prepare, mmu_ctx_cpu_dead);
    }
    printk!(KERN_INFO, "MMU: Allocated %zu bytes of context maps for %d contexts\n",
        2 * CTX_MAP_SIZE + core::mem::size_of::<*mut core::ffi::c_void>() * (LAST_CONTEXT as usize + 1),
        LAST_CONTEXT - FIRST_CONTEXT + 1);
    *context_map = (1usize << FIRST_CONTEXT) as libc::c_ulong - 1;
    next_context = FIRST_CONTEXT;
    nr_free_contexts = LAST_CONTEXT - FIRST_CONTEXT + 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
