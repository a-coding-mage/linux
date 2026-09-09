/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/kstack_erase.h.
// The declarations below are supplied by the corresponding kernel headers.

pub const KSTACK_ERASE_POISON: isize = -0xBEEF;
pub const KSTACK_ERASE_SEARCH_DEPTH: usize = 128;

#[cfg(feature = "CONFIG_KSTACK_ERASE")]
extern "C" {
    pub type task_struct;

    pub fn end_of_stack(tsk: *const task_struct) -> *mut core::ffi::c_void;
    pub fn task_pt_regs(tsk: *const task_struct) -> *mut core::ffi::c_void;

    pub fn stackleak_erase();
    pub fn stackleak_erase_on_task_stack();
    pub fn stackleak_erase_off_task_stack();
    pub fn __sanitizer_cov_stack_depth();
}

#[cfg(feature = "CONFIG_KSTACK_ERASE")]
#[inline(always)]
pub unsafe fn stackleak_task_low_bound(tsk: *const task_struct) -> usize {
    // The lowest unsigned long on the task stack contains STACK_END_MAGIC,
    // which we must not corrupt.
    (end_of_stack(tsk) as usize).wrapping_add(core::mem::size_of::<usize>())
}

#[cfg(feature = "CONFIG_KSTACK_ERASE")]
#[inline(always)]
pub unsafe fn stackleak_task_high_bound(tsk: *const task_struct) -> usize {
    // The task's pt_regs lives at the top of the task stack and will be
    // overwritten by exception entry, so there's no need to erase them.
    task_pt_regs(tsk) as usize
}

#[cfg(feature = "CONFIG_KSTACK_ERASE")]
#[inline(always)]
pub unsafe fn stackleak_find_top_of_poison(low: usize, high: usize) -> usize {
    let depth: usize = KSTACK_ERASE_SEARCH_DEPTH / core::mem::size_of::<usize>();
    let mut poison_count: usize = 0;
    let mut poison_high: usize = high;
    let mut sp: usize = high;

    while sp > low && poison_count < depth {
        sp = sp.wrapping_sub(core::mem::size_of::<usize>());

        if core::ptr::read_volatile(sp as *const usize) == KSTACK_ERASE_POISON as usize {
            poison_count += 1;
        } else {
            poison_count = 0;
            poison_high = sp;
        }
    }

    poison_high
}

#[cfg(feature = "CONFIG_KSTACK_ERASE")]
#[inline]
pub unsafe fn stackleak_task_init(t: *mut task_struct) {
    // `task_struct` fields are supplied by linux/sched.h.
    (*t).lowest_stack = stackleak_task_low_bound(t);
    #[cfg(feature = "CONFIG_KSTACK_ERASE_METRICS")]
    {
        (*t).prev_lowest_stack = (*t).lowest_stack;
    }
}

#[cfg(not(feature = "CONFIG_KSTACK_ERASE"))]
#[inline]
pub unsafe fn stackleak_task_init(_t: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
