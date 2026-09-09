/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// <asm-generic/mm_hooks.h>, <asm/setup.h>, <asm/page.h>,
// <asm/cacheflush.h>, <asm/tlbflush.h>, <linux/errno.h>,
// <linux/sched.h>, <abi/ckmmu.h>

pub const ASID_MASK: u64 = (1u64 << CONFIG_CPU_ASID_BITS) - 1;

#[inline(always)]
pub unsafe fn cpu_asid(mm: *mut mm_struct) -> u64 {
    (*mm).context.asid.load(core::sync::atomic::Ordering::SeqCst) & ASID_MASK
}

#[inline(always)]
pub unsafe fn init_new_context(
    _tsk: *mut task_struct,
    mm: *mut mm_struct,
) -> i32 {
    (*mm)
        .context
        .asid
        .store(0, core::sync::atomic::Ordering::SeqCst);
    0
}

pub unsafe extern "C" fn check_and_switch_context(mm: *mut mm_struct, cpu: u32);

#[inline]
pub unsafe fn switch_mm(
    prev: *mut mm_struct,
    next: *mut mm_struct,
    tsk: *mut task_struct,
) {
    let cpu: u32 = smp_processor_id();

    if prev != next {
        check_and_switch_context(next, cpu);
    }

    setup_pgd((*next).pgd, (*next).context.asid.counter);

    flush_icache_deferred(next);
}

// The generic MMU context declarations are supplied by asm-generic/mmu_context.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
