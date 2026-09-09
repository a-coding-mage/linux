// SPDX-License-Identifier: GPL-2.0
/*
 * Resource Director Technology (RDT)
 *
 * Pseudo-locking support built on top of Cache Allocation Technology (CAT)
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Author: Reinette Chatre <reinette.chatre@intel.com>
 */

// C includes and build-time configuration are supplied by the surrounding kernel.

/* The bits needed to disable hardware prefetching vary by platform. */
static mut prefetch_disable_bits: u64 = 0;

pub unsafe fn resctrl_arch_get_prefetch_disable_bits() -> u64 {
    prefetch_disable_bits = 0;

    if boot_cpu_data.x86_vendor != X86_VENDOR_INTEL || boot_cpu_data.x86 != 6 {
        return 0;
    }

    match boot_cpu_data.x86_vfm {
        INTEL_BROADWELL_X => {
            /*
             * SDM defines bits of MSR_MISC_FEATURE_CONTROL register as:
             * 0    L2 Hardware Prefetcher Disable (R/W)
             * 1    L2 Adjacent Cache Line Prefetcher Disable (R/W)
             * 2    DCU Hardware Prefetcher Disable (R/W)
             * 3    DCU IP Prefetcher Disable (R/W)
             * 63:4 Reserved
             */
            prefetch_disable_bits = 0xF;
        }
        INTEL_ATOM_GOLDMONT | INTEL_ATOM_GOLDMONT_PLUS => {
            /*
             * SDM defines bits of MSR_MISC_FEATURE_CONTROL register as:
             * 0     L2 Hardware Prefetcher Disable (R/W)
             * 1     Reserved
             * 2     DCU Hardware Prefetcher Disable (R/W)
             * 63:3  Reserved
             */
            prefetch_disable_bits = 0x5;
        }
        _ => {}
    }

    prefetch_disable_bits
}

pub unsafe fn resctrl_arch_pseudo_lock_fn(_plr: *mut core::ffi::c_void) -> i32 {
    let plr = _plr as *mut pseudo_lock_region;
    let mut rmid_p: u32;
    let mut closid_p: u32;
    let mut i: usize;
    let saved_msr: u64;
    let mut line_size: u32;
    let mut size: u32;
    let mut mem_r: *mut core::ffi::c_void;

    wbinvd();
    local_irq_disable();
    saved_msr = native_rdmsrq(MSR_MISC_FEATURE_CONTROL);
    native_wrmsrq(MSR_MISC_FEATURE_CONTROL, prefetch_disable_bits);
    closid_p = this_cpu_read(pqr_state.cur_closid);
    rmid_p = this_cpu_read(pqr_state.cur_rmid);
    mem_r = (*plr).kmem;
    size = (*plr).size;
    line_size = (*plr).line_size;

    native_wrmsr(MSR_IA32_PQR_ASSOC, rmid_p, (*plr).closid);
    i = 0;
    while i < size as usize {
        rmb();
        core::arch::asm!("mov ({mem},{off},1), %eax", mem = in(reg) mem_r, off = in(reg) i, out("eax") _, options(nostack));
        i += PAGE_SIZE as usize;
    }
    i = 0;
    while i < size as usize {
        rmb();
        core::arch::asm!("mov ({mem},{off},1), %eax", mem = in(reg) mem_r, off = in(reg) i, out("eax") _, options(nostack));
        i += line_size as usize;
    }
    native_wrmsr(MSR_IA32_PQR_ASSOC, rmid_p, closid_p);
    wrmsrq(MSR_MISC_FEATURE_CONTROL, saved_msr);
    local_irq_enable();
    (*plr).thread_done = 1;
    wake_up_interruptible(&mut (*plr).lock_thread_wq);
    0
}

pub unsafe fn resctrl_arch_measure_cycles_lat_fn(_plr: *mut core::ffi::c_void) -> i32 {
    let plr = _plr as *mut pseudo_lock_region;
    let mut i: usize;
    let mut start: u64;
    let mut end: u64;
    let mut mem_r: *mut core::ffi::c_void;
    let mut saved: u64 = 0;

    local_irq_disable();
    rdmsrq(MSR_MISC_FEATURE_CONTROL, &mut saved);
    wrmsrq(MSR_MISC_FEATURE_CONTROL, prefetch_disable_bits);
    mem_r = READ_ONCE((*plr).kmem);
    start = rdtsc_ordered();
    i = 0;
    while i < (*plr).size as usize {
        start = rdtsc_ordered();
        core::arch::asm!("mov ({mem},{off},1), %eax", mem = in(reg) mem_r, off = in(reg) i, out("eax") _, options(nostack));
        end = rdtsc_ordered();
        trace_pseudo_lock_mem_latency((end.wrapping_sub(start)) as u32);
        i += 32;
    }
    wrmsrq(MSR_MISC_FEATURE_CONTROL, saved);
    local_irq_enable();
    (*plr).thread_done = 1;
    wake_up_interruptible(&mut (*plr).lock_thread_wq);
    0
}

static mut perf_miss_attr: perf_event_attr = perf_event_attr {
    type_: PERF_TYPE_RAW,
    size: core::mem::size_of::<perf_event_attr>(),
    pinned: 1,
    disabled: 0,
    exclude_user: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut perf_hit_attr: perf_event_attr = perf_event_attr {
    type_: PERF_TYPE_RAW,
    size: core::mem::size_of::<perf_event_attr>(),
    pinned: 1,
    disabled: 0,
    exclude_user: 1,
    ..unsafe { core::mem::zeroed() }
};

#[repr(C)]
struct residency_counts {
    miss_before: u64,
    hits_before: u64,
    miss_after: u64,
    hits_after: u64,
}

unsafe fn measure_residency_fn(miss_attr: *mut perf_event_attr, hit_attr: *mut perf_event_attr, plr: *mut pseudo_lock_region, counts: *mut residency_counts) -> i32 {
    let mut hits_before: u64 = 0;
    let mut hits_after: u64 = 0;
    let mut miss_before: u64 = 0;
    let mut miss_after: u64 = 0;
    let miss_event = perf_event_create_kernel_counter(miss_attr, (*plr).cpu, core::ptr::null_mut(), None, core::ptr::null_mut());
    if IS_ERR(miss_event) { return 0; }
    let hit_event = perf_event_create_kernel_counter(hit_attr, (*plr).cpu, core::ptr::null_mut(), None, core::ptr::null_mut());
    if IS_ERR(hit_event) { perf_event_release_kernel(miss_event); return 0; }
    local_irq_disable();
    let mut tmp = 0u64;
    if perf_event_read_local(miss_event, &mut tmp, core::ptr::null_mut(), core::ptr::null_mut()) != 0 || perf_event_read_local(hit_event, &mut tmp, core::ptr::null_mut(), core::ptr::null_mut()) != 0 {
        local_irq_enable();
        perf_event_release_kernel(hit_event);
        perf_event_release_kernel(miss_event);
        return 0;
    }
    let mut saved = 0u64;
    rdmsrq(MSR_MISC_FEATURE_CONTROL, &mut saved);
    wrmsrq(MSR_MISC_FEATURE_CONTROL, prefetch_disable_bits);
    let miss_pmcnum = x86_perf_rdpmc_index(miss_event);
    let hit_pmcnum = x86_perf_rdpmc_index(hit_event);
    let line_size = READ_ONCE((*plr).line_size);
    let mem_r = READ_ONCE((*plr).kmem);
    let size = READ_ONCE((*plr).size);
    hits_before = rdpmc(hit_pmcnum); miss_before = rdpmc(miss_pmcnum);
    rmb();
    hits_before = rdpmc(hit_pmcnum); miss_before = rdpmc(miss_pmcnum);
    rmb();
    let mut i = 0usize;
    while i < size as usize {
        rmb();
        core::arch::asm!("mov ({mem},{off},1), %eax", mem = in(reg) mem_r, off = in(reg) i, out("eax") _, options(nostack));
        i += line_size as usize;
    }
    rmb();
    hits_after = rdpmc(hit_pmcnum); miss_after = rdpmc(miss_pmcnum);
    rmb();
    wrmsrq(MSR_MISC_FEATURE_CONTROL, saved);
    local_irq_enable();
    perf_event_release_kernel(hit_event);
    perf_event_release_kernel(miss_event);
    (*counts).miss_before = miss_before; (*counts).hits_before = hits_before;
    (*counts).miss_after = miss_after; (*counts).hits_after = hits_after;
    0
}

pub unsafe fn resctrl_arch_measure_l2_residency(_plr: *mut core::ffi::c_void) -> i32 {
    let plr = _plr as *mut pseudo_lock_region;
    let mut counts = residency_counts { miss_before: 0, hits_before: 0, miss_after: 0, hits_after: 0 };
    match boot_cpu_data.x86_vfm {
        INTEL_ATOM_GOLDMONT | INTEL_ATOM_GOLDMONT_PLUS => {
            perf_miss_attr.config = X86_CONFIG(.event = 0xd1, .umask = 0x10);
            perf_hit_attr.config = X86_CONFIG(.event = 0xd1, .umask = 0x2);
            measure_residency_fn(&mut perf_miss_attr, &mut perf_hit_attr, plr, &mut counts);
            trace_pseudo_lock_l2(counts.hits_after.wrapping_sub(counts.hits_before), counts.miss_after.wrapping_sub(counts.miss_before));
        }
        _ => {}
    }
    (*plr).thread_done = 1;
    wake_up_interruptible(&mut (*plr).lock_thread_wq);
    0
}

pub unsafe fn resctrl_arch_measure_l3_residency(_plr: *mut core::ffi::c_void) -> i32 {
    let plr = _plr as *mut pseudo_lock_region;
    let mut counts = residency_counts { miss_before: 0, hits_before: 0, miss_after: 0, hits_after: 0 };
    match boot_cpu_data.x86_vfm {
        INTEL_BROADWELL_X => {
            perf_hit_attr.config = X86_CONFIG(.event = 0x2e, .umask = 0x4f);
            perf_miss_attr.config = X86_CONFIG(.event = 0x2e, .umask = 0x41);
            measure_residency_fn(&mut perf_miss_attr, &mut perf_hit_attr, plr, &mut counts);
            counts.miss_after = counts.miss_after.wrapping_sub(counts.miss_before);
            counts.hits_after = counts.hits_after.wrapping_sub(counts.hits_before);
            counts.hits_after = counts.hits_after.wrapping_sub(core::cmp::min(counts.miss_after, counts.hits_after));
            trace_pseudo_lock_l3(counts.hits_after, counts.miss_after);
        }
        _ => {}
    }
    (*plr).thread_done = 1;
    wake_up_interruptible(&mut (*plr).lock_thread_wq);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
