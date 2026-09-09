/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Various scheduler/task debugging interfaces:
 */

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pid_namespace {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dump_cpu_task(cpu: ::core::ffi::c_int);

    /*
     * Only dump TASK_* tasks. (0 for all tasks)
     */
    pub fn show_state_filter(state_filter: ::core::ffi::c_uint);
}

#[inline]
pub unsafe fn show_state() {
    unsafe {
        show_state_filter(0);
    }
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn show_regs(regs: *mut pt_regs);
}

/*
 * TASK is a pointer to the task whose backtrace we want to see (or NULL for current
 * task), SP is the stack pointer of the first frame that should be shown in the back
 * trace (or NULL if the entire call-chain of the task should be shown).
 */
unsafe extern "C" {
    pub fn show_stack(
        task: *mut task_struct,
        sp: *mut ::core::ffi::c_ulong,
        loglvl: *const ::core::ffi::c_char,
    );

    pub fn sched_show_task(p: *mut task_struct);
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn proc_sched_show_task(
        p: *mut task_struct,
        ns: *mut pid_namespace,
        m: *mut seq_file,
    );
    pub fn proc_sched_set_task(p: *mut task_struct);
}

/* Attach to any functions which should be ignored in wchan output. */
/* C macro: __sched __section(".sched.text") */

/* Linker adds these: start and end of __sched functions */
unsafe extern "C" {
    pub static mut __sched_text_start: ::core::ffi::c_char;
    pub static mut __sched_text_end: ::core::ffi::c_char;
}

/* Is this address in the __sched functions? */
unsafe extern "C" {
    pub fn in_sched_functions(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
