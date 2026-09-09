// SPDX-License-Identifier: GPL-2.0
/*
 * Alpha TLB shootdown helpers
 *
 * Copyright (C) 2025 Magnus Lindholm <linmag7@gmail.com>
 *
 * Alpha-specific TLB flush helpers that cannot be expressed purely
 * as inline functions.
 *
 * These helpers provide combined MM context handling (ASN rollover)
 * and immediate TLB invalidation for page migration and memory
 * compaction paths, where lazy shootdowns are insufficient.
 */

// Declarations supplied by the Linux MM, SMP, scheduler, and Alpha headers.
extern "C" {
    static mut current: *mut task_struct;
    static mut cpu_data: *mut cpu_data_t;
    fn smp_processor_id() -> i32;
    fn flush_tlb_current(mm: *mut mm_struct);
    fn flush_tlb_other(mm: *mut mm_struct);
    fn __load_new_mm_context(mm: *mut mm_struct);
    fn tbi(tbi_type: i32, addr: usize);
    fn preempt_disable();
    fn preempt_enable();
    fn on_each_cpu(func: unsafe extern "C" fn(*mut core::ffi::c_void), info: *mut core::ffi::c_void, wait: i32);
    fn cpu_online(cpu: i32) -> i32;
    fn atomic_read(v: *const atomic_t) -> i32;
}

#[repr(C)]
struct task_struct {
    active_mm: *mut mm_struct,
}

#[repr(C)]
struct vm_area_struct {
    vm_mm: *mut mm_struct,
    vm_flags: usize,
}

#[repr(C)]
struct atomic_t {
    counter: i32,
}

#[repr(C)]
struct cpu_data_t {
    asn_lock: bool,
}

#[repr(C)]
struct mm_struct {
    mm_users: atomic_t,
    context: *mut usize,
}

const VM_EXEC: usize = 0x0000_0004;
const NR_CPUS: i32 = 1;

#[cfg(not(feature = "CONFIG_SMP"))]
#[no_mangle]
pub unsafe extern "C" fn migrate_flush_tlb_page(vma: *mut vm_area_struct, addr: usize) {
    let mm = (*vma).vm_mm;
    let tbi_type: i32 = if ((*vma).vm_flags & VM_EXEC) != 0 { 3 } else { 2 };

    /*
     * First do the mm-context side:
     * If we're currently running this mm, reload a fresh context ASN.
     * Otherwise, mark context invalid.
     *
     * On UP, this is mostly about matching the SMP semantics and ensuring
     * exec/i-cache tagging assumptions hold when compaction migrates pages.
     */
    if mm == (*current).active_mm {
        flush_tlb_current(mm);
    } else {
        flush_tlb_other(mm);
    }

    /* Then do the immediate translation kill for this VA. */
    tbi(tbi_type, addr);
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
struct tlb_mm_and_addr {
    mm: *mut mm_struct,
    addr: usize,
    tbi_type: i32, // 2 = DTB, 3 = ITB+DTB
}

#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" fn ipi_flush_mm_and_page(x: *mut core::ffi::c_void) {
    let d = x as *mut tlb_mm_and_addr;

    /* Part 1: mm context side (Alpha uses ASN/context as a key mechanism). */
    if (*d).mm == (*current).active_mm && !asn_locked() {
        __load_new_mm_context((*d).mm);
    } else {
        flush_tlb_other((*d).mm);
    }

    /* Part 2: immediate per-VA invalidation on this CPU. */
    tbi((*d).tbi_type, (*d).addr);
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn asn_locked() -> bool {
    (*cpu_data.add(smp_processor_id() as usize)).asn_lock
}

#[cfg(feature = "CONFIG_SMP")]
#[no_mangle]
pub unsafe extern "C" fn migrate_flush_tlb_page(vma: *mut vm_area_struct, addr: usize) {
    let mm = (*vma).vm_mm;
    let mut d = tlb_mm_and_addr {
        mm,
        addr,
        tbi_type: if ((*vma).vm_flags & VM_EXEC) != 0 { 3 } else { 2 },
    };

    /* One synchronous rendezvous: every CPU runs ipi_flush_mm_and_page(). */
    preempt_disable();
    on_each_cpu(ipi_flush_mm_and_page, &mut d as *mut _ as *mut core::ffi::c_void, 1);

    /* Mimic flush_tlb_mm()'s mm_users<=1 optimization. */
    if atomic_read(&(*mm).mm_users) <= 1 {
        let this_cpu = smp_processor_id();
        for cpu in 0..NR_CPUS {
            if cpu_online(cpu) == 0 || cpu == this_cpu {
                continue;
            }
            let context = (*mm).context.add(cpu as usize);
            if core::ptr::read_volatile(context) != 0 {
                core::ptr::write_volatile(context, 0);
            }
        }
    }
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
