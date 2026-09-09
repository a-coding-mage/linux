// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Oracle Corporation
 */

// Kernel dependencies supplied by the surrounding tree.

#[repr(C)]
pub enum vhost_task_flags {
    VHOST_TASK_FLAGS_STOP,
    VHOST_TASK_FLAGS_KILLED,
}

#[repr(C)]
pub struct vhost_task {
    pub fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> bool>,
    pub handle_sigkill: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub data: *mut core::ffi::c_void,
    pub exited: completion,
    pub flags: core::ffi::c_ulong,
    pub task: *mut task_struct,
    // serialize SIGKILL and vhost_task_stop calls
    pub exit_mutex: mutex,
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernel_clone_args {
    pub flags: core::ffi::c_ulong,
    pub fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub name: *const core::ffi::c_char,
    pub user_worker: core::ffi::c_ulong,
    pub no_files: core::ffi::c_ulong,
    pub fn_arg: *mut core::ffi::c_void,
}

extern "C" {
    fn signal_pending(task: *mut task_struct) -> bool;
    fn get_signal(ksig: *mut core::ffi::c_void) -> bool;
    fn set_current_state(state: i32);
    fn __set_current_state(state: i32);
    fn test_bit(nr: usize, addr: *const core::ffi::c_ulong) -> bool;
    fn set_bit(nr: usize, addr: *mut core::ffi::c_ulong);
    fn schedule();
    fn current() -> *mut task_struct;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn complete(x: *mut completion);
    fn do_exit(error_code: i32) -> !;
    fn wake_up_process(task: *mut task_struct);
    fn wait_for_completion(x: *mut completion);
    fn put_task_struct(task: *mut task_struct);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn init_completion(x: *mut completion);
    fn mutex_init(lock: *mut mutex);
    fn copy_process(
        clone_flags: *mut core::ffi::c_void,
        node: i32,
        numa_node: i32,
        args: *mut kernel_clone_args,
    ) -> *mut task_struct;
    fn get_task_struct(task: *mut task_struct) -> *mut task_struct;
    fn wake_up_new_task(task: *mut task_struct);
}

unsafe fn vhost_task_fn(data: *mut core::ffi::c_void) -> i32 {
    let vtsk = data as *mut vhost_task;

    loop {
        let did_work: bool;

        if signal_pending(current()) {
            let mut ksig = core::mem::MaybeUninit::<[u8; 128]>::uninit();
            if get_signal(ksig.as_mut_ptr() as *mut core::ffi::c_void) {
                break;
            }
        }

        // mb paired w/ vhost_task_stop
        set_current_state(1); // TASK_INTERRUPTIBLE

        if test_bit(VHOST_TASK_FLAGS_STOP as usize, &(*vtsk).flags) {
            __set_current_state(0); // TASK_RUNNING
            break;
        }

        did_work = ((*vtsk).fn_.unwrap_unchecked())((*vtsk).data);
        if !did_work {
            schedule();
        }
    }

    mutex_lock(&mut (*vtsk).exit_mutex);
    /*
     * If a vhost_task_stop and SIGKILL race, we can ignore the SIGKILL.
     * When the vhost layer has called vhost_task_stop it's already stopped
     * new work and flushed.
     */
    if !test_bit(VHOST_TASK_FLAGS_STOP as usize, &(*vtsk).flags) {
        set_bit(VHOST_TASK_FLAGS_KILLED as usize, &mut (*vtsk).flags);
        ((*vtsk).handle_sigkill.unwrap_unchecked())((*vtsk).data);
    }
    mutex_unlock(&mut (*vtsk).exit_mutex);
    complete(&mut (*vtsk).exited);

    do_exit(0);
}

/// vhost_task_wake - wakeup the vhost_task
/// @vtsk: vhost_task to wake
///
/// wake up the vhost_task worker thread
pub unsafe extern "C" fn vhost_task_wake(vtsk: *mut vhost_task) {
    wake_up_process((*vtsk).task);
}

/// vhost_task_stop - stop a vhost_task
/// @vtsk: vhost_task to stop
///
/// vhost_task_fn ensures the worker thread exits after
/// VHOST_TASK_FLAGS_STOP becomes true.
pub unsafe extern "C" fn vhost_task_stop(vtsk: *mut vhost_task) {
    mutex_lock(&mut (*vtsk).exit_mutex);
    if !test_bit(VHOST_TASK_FLAGS_KILLED as usize, &(*vtsk).flags) {
        set_bit(VHOST_TASK_FLAGS_STOP as usize, &mut (*vtsk).flags);
        vhost_task_wake(vtsk);
    }
    mutex_unlock(&mut (*vtsk).exit_mutex);

    /*
     * Make sure vhost_task_fn is no longer accessing the vhost_task before
     * freeing it below.
     */
    wait_for_completion(&mut (*vtsk).exited);
    put_task_struct((*vtsk).task);
    kfree(vtsk as *mut core::ffi::c_void);
}

/// vhost_task_create - create a copy of a task to be used by the kernel
/// @fn: vhost worker function
/// @handle_sigkill: vhost function to handle when we are killed
/// @arg: data to be passed to fn and handled_kill
/// @name: the thread's name
pub unsafe extern "C" fn vhost_task_create(
    fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> bool>,
    handle_sigkill: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    arg: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
) -> *mut vhost_task {
    let mut args = kernel_clone_args {
        flags: 0,
        fn_: Some(vhost_task_fn),
        name,
        user_worker: 1,
        no_files: 1,
        fn_arg: core::ptr::null_mut(),
    };
    let vtsk = libc::calloc(1, core::mem::size_of::<vhost_task>()) as *mut vhost_task;
    if vtsk.is_null() {
        return (-12isize) as *mut vhost_task;
    }
    init_completion(&mut (*vtsk).exited);
    mutex_init(&mut (*vtsk).exit_mutex);
    (*vtsk).data = arg;
    (*vtsk).fn_ = fn_;
    (*vtsk).handle_sigkill = handle_sigkill;

    args.fn_arg = vtsk as *mut core::ffi::c_void;
    let tsk = copy_process(core::ptr::null_mut(), 0, -1, &mut args);
    if (tsk as isize) < 0 {
        kfree(vtsk as *mut core::ffi::c_void);
        return tsk as *mut vhost_task;
    }
    (*vtsk).task = get_task_struct(tsk);
    vtsk
}

/// vhost_task_start - start a vhost_task created with vhost_task_create
/// @vtsk: vhost_task to wake up
pub unsafe extern "C" fn vhost_task_start(vtsk: *mut vhost_task) {
    wake_up_new_task((*vtsk).task);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
