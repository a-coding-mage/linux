// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies:
// linux/sched/signal.h, linux/sched/task.h, linux/sched/mm.h,
// linux/spinlock.h, linux/slab.h, linux/oom.h, linux/reboot.h,
// kern_util.h, os.h, skas.h

extern "C" {
    static mut tasklist_lock: crate::spinlock_t;
    static mut kmalloc_ok: i32;

    fn do_uml_exitcalls();
    fn os_kill_ptraced_process(pid: i32, reap: i32);
    fn reboot_skas();
    fn halt_skas();
    fn register_sys_off_handler(
        mode: i32,
        priority: i32,
        handler: unsafe extern "C" fn(*mut crate::sys_off_data) -> i32,
        data: *mut core::ffi::c_void,
    );
    fn read_lock(lock: *mut crate::spinlock_t);
    fn read_unlock(lock: *mut crate::spinlock_t);
    fn find_lock_task_mm(task: *mut crate::task_struct) -> *mut crate::task_struct;
    fn task_unlock(task: *mut crate::task_struct);
}

// Exported kernel symbol.
#[no_mangle]
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = None;

unsafe fn kill_off_processes() {
    let mut p: *mut crate::task_struct;
    let mut pid: i32;

    read_lock(core::ptr::addr_of_mut!(tasklist_lock));
    // for_each_process(p)
    for_each_process!(p => {
        let t: *mut crate::task_struct;

        t = find_lock_task_mm(p);
        if t.is_null() {
            continue;
        }
        pid = (*(*t).mm).context.id.pid;
        task_unlock(t);
        os_kill_ptraced_process(pid, 1);
    });
    read_unlock(core::ptr::addr_of_mut!(tasklist_lock));
}

#[no_mangle]
pub unsafe extern "C" fn uml_cleanup() {
    kmalloc_ok = 0;
    do_uml_exitcalls();
    kill_off_processes();
}

#[no_mangle]
pub unsafe extern "C" fn machine_restart(_unused: *mut core::ffi::c_char) {
    uml_cleanup();
    reboot_skas();
}

#[no_mangle]
pub unsafe extern "C" fn machine_power_off() {
    uml_cleanup();
    halt_skas();
}

#[no_mangle]
pub unsafe extern "C" fn machine_halt() {
    machine_power_off();
}

unsafe extern "C" fn sys_power_off_handler(data: *mut crate::sys_off_data) -> i32 {
    let _ = data;
    machine_power_off();
    0
}

unsafe fn register_power_off() -> i32 {
    register_sys_off_handler(
        crate::SYS_OFF_MODE_POWER_OFF,
        crate::SYS_OFF_PRIO_DEFAULT,
        sys_power_off_handler,
        core::ptr::null_mut(),
    );
    0
}

// __initcall(register_power_off)
crate::__initcall!(register_power_off);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
