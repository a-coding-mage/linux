// SPDX-License-Identifier: GPL-2.0
/*
 * This code fills the used part of the kernel stack with a poison value
 * before returning to userspace. It's part of the STACKLEAK feature
 * ported from grsecurity/PaX.
 *
 * Author: Alexander Popov <alex.popov@linux.com>
 *
 * KSTACK_ERASE reduces the information which kernel stack leak bugs can
 * reveal and blocks some uninitialized stack variable attacks.
 */

// C dependencies supplied by the kernel build are intentionally external.

#[cfg(feature = "CONFIG_KSTACK_ERASE_RUNTIME_DISABLE")]
static mut STACK_ERASING_BYPASS: bool = false;

#[cfg(all(feature = "CONFIG_KSTACK_ERASE_RUNTIME_DISABLE", feature = "CONFIG_SYSCTL"))]
unsafe fn stack_erasing_sysctl(
    table: *const ctl_table,
    write: i32,
    buffer: *mut core::ffi::c_void,
    lenp: *mut usize,
    ppos: *mut i64,
) -> i32 {
    let mut ret: i32 = 0;
    let mut state: i32 = if !static_branch_unlikely(&STACK_ERASING_BYPASS) { 1 } else { 0 };
    let prev_state = state;
    let mut table_copy = *table;

    table_copy.data = (&mut state as *mut i32).cast();
    ret = proc_dointvec_minmax(&mut table_copy, write, buffer, lenp, ppos);
    state = if state != 0 { 1 } else { 0 };
    if ret != 0 || write == 0 || state == prev_state {
        return ret;
    }

    if state != 0 {
        static_branch_disable(&mut STACK_ERASING_BYPASS);
    } else {
        static_branch_enable(&mut STACK_ERASING_BYPASS);
    }

    pr_warn!("stackleak: kernel stack erasing is {}\n", str_enabled_disabled(state));
    ret
}

#[cfg(all(feature = "CONFIG_KSTACK_ERASE_RUNTIME_DISABLE", feature = "CONFIG_SYSCTL"))]
static STACKLEAK_SYSCTLS: [ctl_table; 1] = [ctl_table {
    procname: "stack_erasing\0".as_ptr(),
    data: core::ptr::null_mut(),
    maxlen: core::mem::size_of::<i32>(),
    mode: 0o600,
    proc_handler: Some(stack_erasing_sysctl),
    extra1: SYSCTL_ZERO,
    extra2: SYSCTL_ONE,
}];

#[cfg(all(feature = "CONFIG_KSTACK_ERASE_RUNTIME_DISABLE", feature = "CONFIG_SYSCTL"))]
unsafe fn stackleak_sysctls_init() -> i32 {
    register_sysctl_init("kernel\0".as_ptr(), STACKLEAK_SYSCTLS.as_ptr());
    0
}

#[cfg(feature = "CONFIG_KSTACK_ERASE_RUNTIME_DISABLE")]
#[inline]
unsafe fn skip_erasing() -> bool {
    static_branch_unlikely(&STACK_ERASING_BYPASS)
}

#[cfg(not(feature = "CONFIG_KSTACK_ERASE_RUNTIME_DISABLE"))]
#[inline]
unsafe fn skip_erasing() -> bool { false }

#[cfg(not(feature = "__stackleak_poison"))]
#[inline(always)]
unsafe fn __stackleak_poison(mut erase_low: usize, erase_high: usize, poison: usize) {
    while erase_low < erase_high {
        *(erase_low as *mut usize) = poison;
        erase_low = erase_low.wrapping_add(core::mem::size_of::<usize>());
    }
}

#[inline(always)]
unsafe fn __stackleak_erase(on_task_stack: bool) {
    let task_stack_low = stackleak_task_low_bound(current);
    let task_stack_high = stackleak_task_high_bound(current);
    let erase_low = stackleak_find_top_of_poison(task_stack_low, (*current).lowest_stack);

    #[cfg(feature = "CONFIG_KSTACK_ERASE_METRICS")]
    { (*current).prev_lowest_stack = erase_low; }

    let erase_high = if on_task_stack { current_stack_pointer } else { task_stack_high };
    __stackleak_poison(erase_low, erase_high, KSTACK_ERASE_POISON);
    (*current).lowest_stack = task_stack_high;
}

pub unsafe fn stackleak_erase() {
    if skip_erasing() { return; }
    __stackleak_erase(on_thread_stack());
}

pub unsafe fn stackleak_erase_on_task_stack() {
    if skip_erasing() { return; }
    __stackleak_erase(true);
}

pub unsafe fn stackleak_erase_off_task_stack() {
    if skip_erasing() { return; }
    __stackleak_erase(false);
}

pub unsafe fn __sanitizer_cov_stack_depth() {
    let mut sp = current_stack_pointer;

    // Having CONFIG_KSTACK_ERASE_TRACK_MIN_SIZE larger than
    // KSTACK_ERASE_SEARCH_DEPTH makes the poison search unreliable.
    const _: () = assert!(CONFIG_KSTACK_ERASE_TRACK_MIN_SIZE <= KSTACK_ERASE_SEARCH_DEPTH);

    sp = (sp + core::mem::size_of::<usize>() - 1) & !(core::mem::size_of::<usize>() - 1);
    if sp < (*current).lowest_stack && sp >= stackleak_task_low_bound(current) {
        (*current).lowest_stack = sp;
    }
}

// External kernel declarations and macros referenced above are supplied by
// the corresponding kernel headers and build environment.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
