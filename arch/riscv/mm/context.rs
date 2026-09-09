// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

#[cfg(CONFIG_MMU)]
static mut USE_ASID_ALLOCATOR: bool = false;

#[cfg(CONFIG_MMU)]
static mut NUM_ASIDS: usize = 0;

#[cfg(CONFIG_MMU)]
static mut CURRENT_VERSION: isize = 0;

#[cfg(CONFIG_MMU)]
static mut CONTEXT_TLB_FLUSH_PENDING: Cpumask = Cpumask { bits: 0 };

#[cfg(CONFIG_MMU)]
static mut CONTEXT_ASID_MAP: *mut usize = core::ptr::null_mut();

#[cfg(CONFIG_MMU)]
static mut ACTIVE_CONTEXT: [isize; 1] = [0; 1];

#[cfg(CONFIG_MMU)]
static mut RESERVED_CONTEXT: [usize; 1] = [0; 1];

#[repr(C)]
pub struct Cpumask {
    pub bits: usize,
}

#[repr(C)]
pub struct MmContext {
    pub id: isize,
    pub icache_stale_mask: Cpumask,
}

#[repr(C)]
pub struct MmStruct {
    pub context: MmContext,
    pub pgd: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

#[cfg(CONFIG_MMU)]
unsafe fn check_update_reserved_context(cntx: usize, newcntx: usize) -> bool {
    let mut hit = false;
    for cpu in for_each_possible_cpu() {
        if RESERVED_CONTEXT[cpu] == cntx {
            hit = true;
            RESERVED_CONTEXT[cpu] = newcntx;
        }
    }
    hit
}

#[cfg(CONFIG_MMU)]
unsafe fn __flush_context() {
    bitmap_zero(CONTEXT_ASID_MAP, NUM_ASIDS);

    for cpu in for_each_possible_cpu() {
        let mut cntx = atomic_long_xchg_relaxed(&mut ACTIVE_CONTEXT[cpu], 0);
        if cntx == 0 {
            cntx = RESERVED_CONTEXT[cpu] as isize;
        }
        __set_bit(cntx2asid(cntx as usize), CONTEXT_ASID_MAP);
        RESERVED_CONTEXT[cpu] = cntx as usize;
    }

    __set_bit(0, CONTEXT_ASID_MAP);
    cpumask_setall(&mut CONTEXT_TLB_FLUSH_PENDING);
}

#[cfg(CONFIG_MMU)]
unsafe fn __new_context(mm: *mut MmStruct) -> usize {
    static mut CUR_IDX: usize = 1;
    let mut cntx = atomic_long_read(&(*mm).context.id) as usize;
    let mut asid: usize;
    let mut ver = atomic_long_read(&CURRENT_VERSION) as usize;

    if cntx != 0 {
        let newcntx = ver | cntx2asid(cntx);
        if check_update_reserved_context(cntx, newcntx) {
            return newcntx;
        }
        if !__test_and_set_bit(cntx2asid(cntx), CONTEXT_ASID_MAP) {
            return newcntx;
        }
    }

    asid = find_next_zero_bit(CONTEXT_ASID_MAP, NUM_ASIDS, CUR_IDX);
    if asid == NUM_ASIDS {
        ver = atomic_long_add_return_relaxed(1usize << SATP_ASID_BITS, &mut CURRENT_VERSION) as usize;
        __flush_context();
        asid = find_next_zero_bit(CONTEXT_ASID_MAP, NUM_ASIDS, 1);
    }

    __set_bit(asid, CONTEXT_ASID_MAP);
    CUR_IDX = asid;
    asid | ver
}

#[cfg(CONFIG_MMU)]
unsafe fn set_mm_asid(mm: *mut MmStruct, cpu: usize) {
    let mut need_flush_tlb = false;
    let mut cntx = atomic_long_read(&(*mm).context.id) as usize;
    let old_active_cntx = atomic_long_read(&ACTIVE_CONTEXT[cpu]) as usize;

    if old_active_cntx != 0
        && cntx2version(cntx) == atomic_long_read(&CURRENT_VERSION) as usize
        && atomic_long_cmpxchg_relaxed(&mut ACTIVE_CONTEXT[cpu], old_active_cntx as isize, cntx as isize) != 0
    {
        csr_write(CSR_SATP, virt_to_pfn((*mm).pgd) | (cntx2asid(cntx) << SATP_ASID_SHIFT) | satp_mode());
        return;
    }

    raw_spin_lock_irqsave();
    cntx = atomic_long_read(&(*mm).context.id) as usize;
    if cntx2version(cntx) != atomic_long_read(&CURRENT_VERSION) as usize {
        cntx = __new_context(mm);
        atomic_long_set(&mut (*mm).context.id, cntx as isize);
    }
    if cpumask_test_and_clear_cpu(cpu, &mut CONTEXT_TLB_FLUSH_PENDING) {
        need_flush_tlb = true;
    }
    atomic_long_set(&mut ACTIVE_CONTEXT[cpu], cntx as isize);
    raw_spin_unlock_irqrestore();

    csr_write(CSR_SATP, virt_to_pfn((*mm).pgd) | (cntx2asid(cntx) << SATP_ASID_SHIFT) | satp_mode());
    if need_flush_tlb {
        local_flush_tlb_all();
    }
}

#[cfg(CONFIG_MMU)]
unsafe fn set_mm_noasid(mm: *mut MmStruct) {
    csr_write(CSR_SATP, virt_to_pfn((*mm).pgd) | satp_mode());
    local_flush_tlb_all_asid(0);
}

#[cfg(CONFIG_MMU)]
unsafe fn set_mm(prev: *mut MmStruct, next: *mut MmStruct, cpu: usize) {
    cpumask_set_cpu(cpu, mm_cpumask(next));
    if static_branch_unlikely(&USE_ASID_ALLOCATOR) {
        set_mm_asid(next, cpu);
    } else {
        cpumask_clear_cpu(cpu, mm_cpumask(prev));
        set_mm_noasid(next);
    }
}

#[cfg(not(CONFIG_MMU))]
unsafe fn set_mm(_prev: *mut MmStruct, _next: *mut MmStruct, _cpu: usize) {}

unsafe fn flush_icache_deferred(mm: *mut MmStruct, cpu: usize, task: *mut TaskStruct) {
    // CONFIG_SMP controls this block in the original source.
    #[cfg(CONFIG_SMP)]
    if cpumask_test_and_clear_cpu(cpu, &mut (*mm).context.icache_stale_mask) {
        smp_mb();
        if task.is_null() || !switch_to_should_flush_icache(task) {
            local_flush_icache_all();
        }
    }
}

pub unsafe fn switch_mm(prev: *mut MmStruct, next: *mut MmStruct, task: *mut TaskStruct) {
    if prev == next {
        return;
    }

    membarrier_arch_switch_mm(prev, next, task);
    let cpu = smp_processor_id();
    set_mm(prev, next, cpu);
    flush_icache_deferred(next, cpu, task);
}

// External kernel symbols and low-level helpers referenced above.
extern "C" {
    fn for_each_possible_cpu() -> core::ops::Range<usize>;
    fn bitmap_zero(map: *mut usize, nbits: usize);
    fn __set_bit(bit: usize, map: *mut usize);
    fn __test_and_set_bit(bit: usize, map: *mut usize) -> bool;
    fn find_next_zero_bit(map: *mut usize, size: usize, offset: usize) -> usize;
    fn atomic_long_read(value: *const isize) -> isize;
    fn atomic_long_set(value: *mut isize, new: isize);
    fn atomic_long_xchg_relaxed(value: *mut isize, new: isize) -> isize;
    fn atomic_long_cmpxchg_relaxed(value: *mut isize, old: isize, new: isize) -> isize;
    fn atomic_long_add_return_relaxed(value: usize, ptr: *mut isize) -> isize;
    fn cntx2asid(cntx: usize) -> usize;
    fn cntx2version(cntx: usize) -> usize;
    fn cpumask_setall(mask: *mut Cpumask);
    fn cpumask_set_cpu(cpu: usize, mask: *mut Cpumask);
    fn cpumask_clear_cpu(cpu: usize, mask: *mut Cpumask);
    fn cpumask_test_and_clear_cpu(cpu: usize, mask: *mut Cpumask) -> bool;
    fn mm_cpumask(mm: *mut MmStruct) -> *mut Cpumask;
    fn static_branch_unlikely(key: *const bool) -> bool;
    fn csr_write(reg: usize, value: usize);
    fn virt_to_pfn(pgd: *mut core::ffi::c_void) -> usize;
    fn satp_mode() -> usize;
    fn local_flush_tlb_all();
    fn local_flush_tlb_all_asid(asid: usize);
    fn raw_spin_lock_irqsave();
    fn raw_spin_unlock_irqrestore();
    fn smp_mb();
    fn switch_to_should_flush_icache(task: *mut TaskStruct) -> bool;
    fn local_flush_icache_all();
    fn membarrier_arch_switch_mm(prev: *mut MmStruct, next: *mut MmStruct, task: *mut TaskStruct);
    fn smp_processor_id() -> usize;
}

const SATP_ASID_BITS: usize = 16;
const SATP_ASID_SHIFT: usize = 44;
const CSR_SATP: usize = 0x180;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
