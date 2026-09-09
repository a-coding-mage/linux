// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel, RV infrastructure, and wakeup headers.

const MODULE_NAME: &str = "wakeup";

#[cfg(not(any()))]
const __NR_FUTEX: i64 = -0;
#[cfg(not(any()))]
const __NR_FUTEX_TIME64: i64 = -1;

extern "C" {
    fn ltl_atom_set(mon: *mut ltl_monitor, atom: i32, value: bool);
    fn ltl_atom_pulse(task: *mut task_struct, atom: i32, value: bool);
    fn ltl_atom_update(task: *mut task_struct, atom: i32, value: bool);
    fn rt_or_dl_task(task: *mut task_struct) -> bool;
    fn in_task() -> bool;
    fn in_serving_softirq() -> bool;
    fn syscall_get_arguments(task: *mut task_struct, regs: *mut pt_regs, args: *mut c_ulong);
    fn ltl_monitor_init() -> c_int;
    fn ltl_monitor_destroy();
    fn rv_attach_trace_probe(name: *const c_char, probe: *const c_void, handler: *const c_void);
    fn rv_detach_trace_probe(name: *const c_char, probe: *const c_void, handler: *const c_void);
    fn rv_register_monitor(monitor: *mut rv_monitor, parent: *mut rv_monitor) -> c_int;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
}

#[repr(C)]
pub struct task_struct {
    pub flags: c_ulong,
    pub prio: c_int,
}

#[repr(C)]
pub struct pt_regs;

#[repr(C)]
pub struct ltl_monitor;

#[repr(C)]
pub struct rv_monitor {
    pub name: *const c_char,
    pub description: *const c_char,
    pub enable: Option<unsafe extern "C" fn() -> c_int>,
    pub disable: Option<unsafe extern "C" fn()>,
}

type c_int = i32;
type c_ulong = usize;
type c_char = i8;
type c_void = core::ffi::c_void;

const LTL_RT: i32 = 0;
const LTL_WOKEN_BY_LOWER_PRIO: i32 = 1;
const LTL_WOKEN_BY_SOFTIRQ: i32 = 2;
const LTL_BLOCK_ON_RT_MUTEX: i32 = 3;
const LTL_FUTEX_LOCK_PI: i32 = 4;
const LTL_USER_THREAD: i32 = 5;
const PF_KTHREAD: c_ulong = 0x0020_0000;
const LCB_F_RT: c_uint = 1;
const FUTEX_CMD_MASK: c_ulong = 0x7f;
const FUTEX_LOCK_PI: c_ulong = 6;
const FUTEX_LOCK_PI2: c_ulong = 13;
type c_uint = u32;

extern "C" {
    static mut current: *mut task_struct;
    static mut sched_waking: c_void;
    static mut contention_begin: c_void;
    static mut contention_end: c_void;
    static mut sys_enter: c_void;
    static mut sys_exit: c_void;
    static mut rv_rtapp: rv_monitor;
}

unsafe extern "C" fn ltl_atoms_fetch(task: *mut task_struct, mon: *mut ltl_monitor) {
    /*
     * This includes "actual" real-time tasks and also PI-boosted
     * tasks. A task being PI-boosted means it is blocking an "actual"
     * real-task, therefore it should also obey the monitor's rule,
     * otherwise the "actual" real-task may be delayed.
     */
    ltl_atom_set(mon, LTL_RT, rt_or_dl_task(task));
}

unsafe extern "C" fn ltl_atoms_init(
    task: *mut task_struct,
    mon: *mut ltl_monitor,
    task_creation: bool,
) {
    ltl_atom_set(mon, LTL_WOKEN_BY_LOWER_PRIO, false);
    ltl_atom_set(mon, LTL_WOKEN_BY_SOFTIRQ, false);

    if task_creation {
        ltl_atom_set(mon, LTL_BLOCK_ON_RT_MUTEX, false);
        ltl_atom_set(mon, LTL_FUTEX_LOCK_PI, false);
    }

    ltl_atom_set(mon, LTL_USER_THREAD, ((*task).flags & PF_KTHREAD) == 0);
}

unsafe extern "C" fn handle_sched_waking(_data: *mut c_void, task: *mut task_struct) {
    if in_task() {
        if (*current).prio > (*task).prio {
            ltl_atom_pulse(task, LTL_WOKEN_BY_LOWER_PRIO, true);
        }
    } else if in_serving_softirq() {
        ltl_atom_pulse(task, LTL_WOKEN_BY_SOFTIRQ, true);
    }
}

unsafe extern "C" fn handle_contention_begin(
    _data: *mut c_void,
    _lock: *mut c_void,
    flags: c_uint,
) {
    if flags & LCB_F_RT != 0 {
        ltl_atom_update(current, LTL_BLOCK_ON_RT_MUTEX, true);
    }
}

unsafe extern "C" fn handle_contention_end(
    _data: *mut c_void,
    _lock: *mut c_void,
    _ret: c_int,
) {
    ltl_atom_update(current, LTL_BLOCK_ON_RT_MUTEX, false);
}

unsafe extern "C" fn handle_sys_enter(_data: *mut c_void, regs: *mut pt_regs, id: i64) {
    let mut args = [0usize; 6];
    let mut op: c_ulong;
    let mut cmd: c_ulong;

    match id {
        __NR_FUTEX | __NR_FUTEX_TIME64 => {
            syscall_get_arguments(current, regs, args.as_mut_ptr());
            op = args[1];
            cmd = op & FUTEX_CMD_MASK;
            match cmd {
                FUTEX_LOCK_PI | FUTEX_LOCK_PI2 => {
                    ltl_atom_update(current, LTL_FUTEX_LOCK_PI, true);
                }
                _ => {}
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn handle_sys_exit(
    _data: *mut c_void,
    _regs: *mut pt_regs,
    _ret: i64,
) {
    ltl_atom_update(current, LTL_FUTEX_LOCK_PI, false);
}

unsafe extern "C" fn enable_wakeup() -> c_int {
    let retval = ltl_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut sched_waking, handle_sched_waking as *const c_void);
    rv_attach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut contention_begin, handle_contention_begin as *const c_void);
    rv_attach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut contention_end, handle_contention_end as *const c_void);
    rv_attach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut sys_enter, handle_sys_enter as *const c_void);
    rv_attach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut sys_exit, handle_sys_exit as *const c_void);

    0
}

unsafe extern "C" fn disable_wakeup() {
    rv_detach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut sched_waking, handle_sched_waking as *const c_void);
    rv_detach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut contention_begin, handle_contention_begin as *const c_void);
    rv_detach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut contention_end, handle_contention_end as *const c_void);
    rv_detach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut sys_enter, handle_sys_enter as *const c_void);
    rv_detach_trace_probe(b"rtapp_wakeup\0".as_ptr() as *const c_char, &raw mut sys_exit, handle_sys_exit as *const c_void);
    ltl_monitor_destroy();
}

static mut rv_wakeup: rv_monitor = rv_monitor {
    name: b"wakeup\0".as_ptr() as *const c_char,
    description: b"Monitor that real-time tasks are not woken by lower-priority tasks\0".as_ptr() as *const c_char,
    enable: Some(enable_wakeup),
    disable: Some(disable_wakeup),
};

unsafe extern "C" fn register_wakeup() -> c_int {
    rv_register_monitor(&raw mut rv_wakeup, &raw mut rv_rtapp)
}

unsafe extern "C" fn unregister_wakeup() {
    rv_unregister_monitor(&raw mut rv_wakeup);
}

// module_init(register_wakeup);
// module_exit(unregister_wakeup);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Nam Cao <namcao@linutronix.de>");
// MODULE_DESCRIPTION("Monitor that real-time tasks are not woken by lower-priority tasks");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
