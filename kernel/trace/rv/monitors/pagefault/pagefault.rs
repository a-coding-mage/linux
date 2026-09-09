// SPDX-License-Identifier: GPL-2.0
// Kernel and monitor dependencies supplied by the surrounding repository.

const MODULE_NAME: &str = "pagefault";

extern "C" {
    fn ltl_atom_set(mon: *mut ltl_monitor, atom: i32, value: bool);
    fn rt_or_dl_task(task: *mut task_struct) -> bool;
    fn ltl_atom_pulse(task: *mut task_struct, atom: i32, value: bool);
    fn rv_get_current() -> *mut task_struct;
    fn ltl_monitor_init() -> i32;
    fn rv_attach_trace_probe(
        monitor: *const core::ffi::c_char,
        event: *const core::ffi::c_void,
        handler: unsafe extern "C" fn(
            *mut core::ffi::c_void,
            usize,
            *mut pt_regs,
            usize,
        ),
    );
    fn rv_detach_trace_probe(
        monitor: *const core::ffi::c_char,
        event: *const core::ffi::c_void,
        handler: unsafe extern "C" fn(
            *mut core::ffi::c_void,
            usize,
            *mut pt_regs,
            usize,
        ),
    );
    fn ltl_monitor_destroy();
    fn rv_register_monitor(monitor: *mut rv_monitor, rv: *mut core::ffi::c_void) -> i32;
    fn rv_unregister_monitor(monitor: *mut rv_monitor);
    fn handle_task_newtask();
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ltl_monitor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rv_monitor {
    pub name: *const core::ffi::c_char,
    pub description: *const core::ffi::c_char,
    pub enable: Option<unsafe extern "C" fn() -> i32>,
    pub disable: Option<unsafe extern "C" fn()>,
}

const LTL_RT: i32 = 0;
const LTL_PAGEFAULT: i32 = 1;

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
    _task: *mut task_struct,
    mon: *mut ltl_monitor,
    task_creation: bool,
) {
    if task_creation {
        ltl_atom_set(mon, LTL_PAGEFAULT, false);
    }
}

unsafe extern "C" fn handle_page_fault(
    _data: *mut core::ffi::c_void,
    _address: usize,
    _regs: *mut pt_regs,
    _error_code: usize,
) {
    ltl_atom_pulse(rv_get_current(), LTL_PAGEFAULT, true);
}

unsafe extern "C" fn enable_pagefault() -> i32 {
    let retval = ltl_monitor_init();
    if retval != 0 {
        return retval;
    }

    rv_attach_trace_probe(
        b"rtapp_pagefault\0".as_ptr() as *const core::ffi::c_char,
        page_fault_kernel,
        handle_page_fault,
    );
    rv_attach_trace_probe(
        b"rtapp_pagefault\0".as_ptr() as *const core::ffi::c_char,
        page_fault_user,
        handle_page_fault,
    );

    0
}

unsafe extern "C" fn disable_pagefault() {
    rv_detach_trace_probe(
        b"rtapp_pagefault\0".as_ptr() as *const core::ffi::c_char,
        page_fault_kernel,
        handle_page_fault,
    );
    rv_detach_trace_probe(
        b"rtapp_pagefault\0".as_ptr() as *const core::ffi::c_char,
        page_fault_user,
        handle_page_fault,
    );

    ltl_monitor_destroy();
}

static mut rv_this: rv_monitor = rv_monitor {
    name: b"pagefault\0".as_ptr() as *const core::ffi::c_char,
    description: b"Monitor that RT tasks do not raise page faults\0".as_ptr()
        as *const core::ffi::c_char,
    enable: Some(enable_pagefault),
    disable: Some(disable_pagefault),
};

unsafe extern "C" fn register_pagefault() -> i32 {
    rv_register_monitor(&raw mut rv_this, &raw mut rv_rtapp)
}

unsafe extern "C" fn unregister_pagefault() {
    rv_unregister_monitor(&raw mut rv_this);
}

// module_init(register_pagefault);
// module_exit(unregister_pagefault);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Nam Cao <namcao@linutronix.de>");
// MODULE_DESCRIPTION("pagefault: Monitor that RT tasks do not raise page faults");

// Preserves the source condition: IS_ENABLED(CONFIG_RV_MONITORS_KUNIT_TEST).
#[cfg(feature = "CONFIG_RV_MONITORS_KUNIT_TEST")]
pub mod kunit {
    use super::*;

    #[repr(C)]
    pub struct rv_pagefault_ops {
        pub mon: *mut core::ffi::c_void,
        pub handle_page_fault: unsafe extern "C" fn(
            *mut core::ffi::c_void,
            usize,
            *mut pt_regs,
            usize,
        ),
        pub handle_task_newtask: unsafe extern "C" fn(),
    }

    #[no_mangle]
    pub static rv_pagefault_ops: rv_pagefault_ops = rv_pagefault_ops {
        mon: core::ptr::null_mut(),
        handle_page_fault,
        handle_task_newtask,
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
