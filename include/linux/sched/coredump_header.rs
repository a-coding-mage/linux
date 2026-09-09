/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Task dumpability mode.  Gates core dump production and ptrace_attach()
 * authorization.  The numeric values are stable ABI (suid_dumpable
 * sysctl, prctl(PR_SET_DUMPABLE)); do not renumber.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum task_dumpable {
    TASK_DUMPABLE_OFF = 0,   /* no dump; ptrace needs CAP_SYS_PTRACE */
    TASK_DUMPABLE_OWNER = 1, /* default; dump and ptrace by uid match */
    TASK_DUMPABLE_ROOT = 2,  /* dump as root; ptrace needs CAP_SYS_PTRACE */
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub fn task_exec_state_set_dumpable(value: task_dumpable);
    pub fn task_exec_state_get_dumpable(task: *mut task_struct) -> task_dumpable;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
