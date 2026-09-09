/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Common arm64 stack unwinder code.
 *
 * See: arch/arm64/kernel/stacktrace.c for the reference implementation.
 *
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency intent: `linux/types.h` supplies the C integer and boolean types.

#[repr(C)]
pub struct stack_info {
    pub low: usize,
    pub high: usize,
}

/**
 * struct unwind_state - state used for robust unwinding.
 *
 * @fp:          The fp value in the frame record (or the real fp)
 * @pc:          The lr value in the frame record (or the real lr)
 *
 * @stack:       The stack currently being unwound.
 * @stacks:      An array of stacks which can be unwound.
 * @nr_stacks:   The number of stacks in @stacks.
 */
#[repr(C)]
pub struct unwind_state {
    pub fp: usize,
    pub pc: usize,

    pub stack: stack_info,
    pub stacks: *mut stack_info,
    pub nr_stacks: ::core::ffi::c_int,
}

pub unsafe fn stackinfo_get_unknown() -> stack_info {
    stack_info { low: 0, high: 0 }
}

pub unsafe fn stackinfo_on_stack(info: *const stack_info, sp: usize, size: usize) -> bool {
    if (*info).low == 0 {
        return false;
    }

    let end = sp.wrapping_add(size);
    if sp < (*info).low || end < sp || end > (*info).high {
        return false;
    }

    true
}

pub unsafe fn unwind_init_common(state: *mut unwind_state) {
    (*state).stack = stackinfo_get_unknown();
}

/**
 * unwind_find_stack() - Find the accessible stack which entirely contains an
 * object.
 *
 * @state: the current unwind state.
 * @sp:    the base address of the object.
 * @size:  the size of the object.
 *
 * Return: a pointer to the relevant stack_info if found; NULL otherwise.
 */
pub unsafe fn unwind_find_stack(
    state: *mut unwind_state,
    sp: usize,
    size: usize,
) -> *mut stack_info {
    let mut info = &mut (*state).stack as *mut stack_info;

    if stackinfo_on_stack(info, sp, size) {
        return info;
    }

    for i in 0..(*state).nr_stacks {
        info = (*state).stacks.add(i as usize);
        if stackinfo_on_stack(info, sp, size) {
            return info;
        }
    }

    core::ptr::null_mut()
}

/**
 * unwind_consume_stack() - Update stack boundaries so that future unwind steps
 * cannot consume this object again.
 *
 * @state: the current unwind state.
 * @info:  the stack_info of the stack containing the object.
 * @sp:    the base address of the object.
 * @size:  the size of the object.
 *
 * Return: 0 upon success, an error code otherwise.
 */
pub unsafe fn unwind_consume_stack(
    state: *mut unwind_state,
    info: *mut stack_info,
    sp: usize,
    size: usize,
) {
    let tmp = *info;

    /*
     * Stack transitions are strictly one-way, and once we've
     * transitioned from one stack to another, it's never valid to
     * unwind back to the old stack.
     *
     * Destroy the old stack info so that it cannot be found upon a
     * subsequent transition. If the stack has not changed, we'll
     * immediately restore the current stack info.
     *
     * Note that stacks can nest in several valid orders, e.g.
     *
     *   TASK -> IRQ -> OVERFLOW -> SDEI_NORMAL
     *   TASK -> SDEI_NORMAL -> SDEI_CRITICAL -> OVERFLOW
     *   HYP -> OVERFLOW
     *
     * ... so we do not check the specific order of stack
     * transitions.
     */
    *info = stackinfo_get_unknown();
    (*state).stack = tmp;

    /*
     * Future unwind steps can only consume stack above this frame record.
     * Update the current stack to start immediately above it.
     */
    (*state).stack.low = sp.wrapping_add(size);
}

// Dependency intent: `struct frame_record`, `EINVAL`, and `READ_ONCE` are
// supplied by the surrounding kernel translation.
/**
 * unwind_next_frame_record() - Unwind to the next frame record.
 *
 * @state:        the current unwind state.
 *
 * Return: 0 upon success, an error code otherwise.
 */
pub unsafe fn unwind_next_frame_record(state: *mut unwind_state) -> ::core::ffi::c_int {
    let mut info: *mut stack_info;
    let record: *mut frame_record;
    let fp = (*state).fp;

    if fp & 0x7 != 0 {
        return -EINVAL;
    }

    info = unwind_find_stack(state, fp, core::mem::size_of::<frame_record>());
    if info.is_null() {
        return -EINVAL;
    }

    unwind_consume_stack(state, info, fp, core::mem::size_of::<frame_record>());

    /*
     * Record this frame record's values.
     */
    record = fp as *mut frame_record;
    (*state).fp = core::ptr::read_volatile(core::ptr::addr_of!((*record).fp));
    (*state).pc = core::ptr::read_volatile(core::ptr::addr_of!((*record).lr));

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
