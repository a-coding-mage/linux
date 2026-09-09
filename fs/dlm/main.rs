// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2007 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C dependencies: linux/module.h, dlm_internal.h, lockspace.h, lock.h,
// user.h, memory.h, config.h, midcomms.h, and trace/events/dlm.h.
// CREATE_TRACE_POINTS

#[repr(C)]
pub struct workqueue_struct {
    _private: [u8; 0],
}

extern "C" {
    fn dlm_memory_init() -> i32;
    fn dlm_memory_exit();
    fn dlm_midcomms_init();
    fn dlm_midcomms_exit();
    fn dlm_lockspace_init() -> i32;
    fn dlm_lockspace_exit();
    fn dlm_config_init() -> i32;
    fn dlm_config_exit();
    fn dlm_register_debugfs();
    fn dlm_unregister_debugfs();
    fn dlm_user_init() -> i32;
    fn dlm_user_exit();
    fn dlm_plock_init() -> i32;
    fn dlm_plock_exit();
    fn alloc_workqueue(name: *const core::ffi::c_char, flags: u32, max_active: u32)
        -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn printk(format: *const core::ffi::c_char, ...);
}

// WQ_PERCPU and -ENOMEM are supplied by the kernel headers.
extern "C" {
    static WQ_PERCPU: u32;
}

#[no_mangle]
pub static mut dlm_wq: *mut workqueue_struct = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn init_dlm() -> i32 {
    let mut error: i32;

    error = dlm_memory_init();
    if error != 0 {
        return error;
    }

    dlm_midcomms_init();

    error = dlm_lockspace_init();
    if error != 0 {
        dlm_midcomms_exit();
        dlm_memory_exit();
        return error;
    }

    error = dlm_config_init();
    if error != 0 {
        dlm_lockspace_exit();
        dlm_midcomms_exit();
        dlm_memory_exit();
        return error;
    }

    dlm_register_debugfs();

    error = dlm_user_init();
    if error != 0 {
        dlm_unregister_debugfs();
        dlm_config_exit();
        dlm_lockspace_exit();
        dlm_midcomms_exit();
        dlm_memory_exit();
        return error;
    }

    error = dlm_plock_init();
    if error != 0 {
        dlm_user_exit();
        dlm_unregister_debugfs();
        dlm_config_exit();
        dlm_lockspace_exit();
        dlm_midcomms_exit();
        dlm_memory_exit();
        return error;
    }

    dlm_wq = alloc_workqueue(b"dlm_wq\0".as_ptr() as *const core::ffi::c_char, WQ_PERCPU, 0);
    if dlm_wq.is_null() {
        error = -12; // -ENOMEM
        dlm_plock_exit();
        dlm_user_exit();
        dlm_unregister_debugfs();
        dlm_config_exit();
        dlm_lockspace_exit();
        dlm_midcomms_exit();
        dlm_memory_exit();
        return error;
    }

    printk(b"DLM installed\n\0".as_ptr() as *const core::ffi::c_char);

    0
}

#[no_mangle]
pub unsafe extern "C" fn exit_dlm() {
    /* be sure every pending work e.g. freeing is done */
    destroy_workqueue(dlm_wq);
    dlm_plock_exit();
    dlm_user_exit();
    dlm_config_exit();
    dlm_lockspace_exit();
    dlm_midcomms_exit();
    dlm_unregister_debugfs();
    dlm_memory_exit();
}

// module_init(init_dlm);
// module_exit(exit_dlm);
// MODULE_DESCRIPTION("Distributed Lock Manager");
// MODULE_AUTHOR("Red Hat, Inc.");
// MODULE_LICENSE("GPL");
// EXPORT_SYMBOL_GPL(dlm_new_lockspace);
// EXPORT_SYMBOL_GPL(dlm_release_lockspace);
// EXPORT_SYMBOL_GPL(dlm_lock);
// EXPORT_SYMBOL_GPL(dlm_unlock);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
