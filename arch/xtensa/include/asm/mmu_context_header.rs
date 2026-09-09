/*
 * Switch an MMU context.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2013 Tensilica Inc.
 */

// The CONFIG_MMU-disabled case includes <asm/nommu_context.h>.
// The declarations below are the CONFIG_MMU case.

// XCHAL_HAVE_TLBS must equal 1: Linux must have an MMU.

extern "C" {
    static mut asid_cache: [::core::ffi::c_ulong; 0];
    fn init_mmu();
    fn init_kio();
    fn local_flush_tlb_all();
    fn invalidate_page_directory();
    fn __invalidate_icache_all();
    fn smp_processor_id() -> u32;
    fn for_each_possible_cpu(cpu: *mut i32);
}

// DECLARE_PER_CPU(unsigned long, asid_cache);
// #define cpu_asid_cache(cpu) per_cpu(asid_cache, cpu)
// The per-CPU accessors and the `mm_struct`, `task_struct`, and context types
// are supplied by the corresponding kernel dependencies.

/*
 * NO_CONTEXT is the invalid ASID value that we don't ever assign to
 * any user or kernel context.  We use the reserved values in the
 * ASID_INSERT macro below.
 *
 * 0 invalid
 * 1 kernel
 * 2 reserved
 * 3 reserved
 * 4...255 available
 */

pub const NO_CONTEXT: u32 = 0;
pub const ASID_USER_FIRST: u32 = 4;
// XCHAL_MMU_ASID_BITS is supplied by the target configuration.
pub const ASID_MASK: u32 = (1u32 << XCHAL_MMU_ASID_BITS) - 1;

#[inline]
pub const fn asid_insert(x: u32) -> u32 {
    0x03020001u32 | ((x & ASID_MASK) << 8)
}

#[inline]
unsafe fn set_rasid_register(val: ::core::ffi::c_ulong) {
    ::core::arch::asm!(
        "wsr {0}, rasid",
        "isync",
        in("a2") val,
        options(nostack)
    );
}

#[inline]
unsafe fn get_rasid_register() -> ::core::ffi::c_ulong {
    let tmp: ::core::ffi::c_ulong;
    ::core::arch::asm!("rsr {0}, rasid", out("a2") tmp, options(nostack));
    tmp
}

#[inline]
unsafe fn get_new_mmu_context(mm: *mut mm_struct, cpu: u32) {
    let mut asid = cpu_asid_cache(cpu);
    asid = asid.wrapping_add(1);
    if (asid & ASID_MASK) == 0 {
        /*
         * Start new asid cycle; continue counting with next
         * incarnation bits; skipping over 0, 1, 2, 3.
         */
        local_flush_tlb_all();
        asid = asid.wrapping_add(ASID_USER_FIRST);
    }
    cpu_asid_cache_set(cpu, asid);
    (*mm).context.asid[cpu as usize] = asid;
    (*mm).context.cpu = cpu as i32;
}

#[inline]
unsafe fn get_mmu_context(mm: *mut mm_struct, cpu: u32) {
    /* Check if our ASID is of an older version and thus invalid. */
    if !mm.is_null() {
        let asid = (*mm).context.asid[cpu as usize];
        if asid == NO_CONTEXT || ((asid ^ cpu_asid_cache(cpu)) & !ASID_MASK) != 0 {
            get_new_mmu_context(mm, cpu);
        }
    }
}

#[inline]
unsafe fn activate_context(mm: *mut mm_struct, cpu: u32) {
    get_mmu_context(mm, cpu);
    set_rasid_register(asid_insert((*mm).context.asid[cpu as usize] as u32) as _);
    invalidate_page_directory();
}

/*
 * Initialize the context related info for a new mm_struct
 * instance.  Valid cpu values are 0..(NR_CPUS-1), so initializing
 * to -1 says the process has never run on any core.
 */

#[inline]
unsafe fn init_new_context(_tsk: *mut task_struct, mm: *mut mm_struct) -> i32 {
    let mut cpu: i32 = 0;
    // for_each_possible_cpu(cpu)
    while cpu < NR_CPUS {
        (*mm).context.asid[cpu as usize] = NO_CONTEXT;
        cpu += 1;
    }
    (*mm).context.cpu = -1;
    0
}

#[inline]
unsafe fn switch_mm(prev: *mut mm_struct, next: *mut mm_struct, _tsk: *mut task_struct) {
    let cpu = smp_processor_id();
    let migrated = (*next).context.cpu != cpu as i32;
    /* Flush the icache if we migrated to a new core. */
    if migrated {
        __invalidate_icache_all();
        (*next).context.cpu = cpu as i32;
    }
    if migrated || prev != next {
        activate_context(next, cpu);
    }
}

/* Destroy context related info for an mm_struct about to be put to rest. */
#[inline]
unsafe fn destroy_context(_mm: *mut mm_struct) {
    invalidate_page_directory();
}

// <asm-generic/mmu_context.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
