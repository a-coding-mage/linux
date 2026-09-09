// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/UML translation unit:
// `task_struct` and `FS_BASE`.

/// Equivalent to the C `clear_flushed_tls` implementation.
pub unsafe fn clear_flushed_tls(_task: *mut task_struct) {}

/// Set the thread-local storage base for a task.
pub unsafe fn arch_set_tls(t: *mut task_struct, tls: usize) -> i32 {
    /*
     * If CLONE_SETTLS is set, we need to save the thread id
     * so it can be set during context switches.
     */
    (*t).thread.regs.regs.gp[FS_BASE / core::mem::size_of::<usize>()] = tls;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
