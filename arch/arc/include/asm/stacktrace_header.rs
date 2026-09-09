/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014-15 Synopsys, Inc. (www.synopsys.com)
 * Copyright (C) 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependency supplied by the surrounding kernel translation: linux/sched.h

/**
 * arc_unwind_core - Unwind the kernel mode stack for an execution context
 * @tsk:             NULL for current task, specific task otherwise
 * @regs:            pt_regs used to seed the unwinder {SP, FP, BLINK, PC}
 *                   If NULL, use pt_regs of @tsk (if !NULL) otherwise
 *                   use the current values of {SP, FP, BLINK, PC}
 * @consumer_fn:     Callback invoked for each frame unwound
 *                   Returns 0 to continue unwinding, -1 to stop
 * @arg:              Arg to callback
 *
 * Returns the address of first function in stack
 *
 * Semantics:
 *  - synchronous unwinding (e.g. dump_stack): @tsk  NULL, @regs  NULL
 *  - Asynchronous unwinding of sleeping task: @tsk !NULL, @regs  NULL
 *  - Asynchronous unwinding of intr/excp etc: @tsk !NULL, @regs !NULL
 */
// C attributes: notrace, noinline.
unsafe extern "C" {
    pub fn arc_unwind_core(
        tsk: *mut task_struct,
        regs: *mut pt_regs,
        consumer_fn: Option<unsafe extern "C" fn(unsigned_int: u32, arg: *mut core::ffi::c_void) -> i32>,
        arg: *mut core::ffi::c_void,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
