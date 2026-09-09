/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel/monitor translation unit. */

/* Dummy values if not available. */
#[cfg(not(feature = "nr_sched_setscheduler"))]
pub const __NR_SCHED_SETSCHEDULER: isize = -1;
#[cfg(not(feature = "nr_sched_setattr"))]
pub const __NR_SCHED_SETATTR: isize = -2;

extern "C" {
    static mut rv_deadline: rv_monitor;
    /* Initialised when registering the deadline container. */
    static mut rv_ext_sched_class: *mut sched_class;
}

/* External C types and functions are provided by the corresponding kernel headers. */
#[allow(non_camel_case_types)]
pub type u8 = core::ffi::c_uchar;
#[allow(non_camel_case_types)]
pub type pid_t = i32;
#[allow(non_camel_case_types)]
pub type u64 = core::ffi::c_ulonglong;
#[allow(non_camel_case_types)]
pub type long = isize;
#[allow(non_camel_case_types)]
pub type size_t = usize;
#[allow(non_camel_case_types)]
pub type rv_monitor = core::ffi::c_void;
#[allow(non_camel_case_types)]
pub type sched_class = core::ffi::c_void;
#[allow(non_camel_case_types)]
pub type sched_dl_entity = core::ffi::c_void;
#[allow(non_camel_case_types)]
pub type task_struct = core::ffi::c_void;
#[allow(non_camel_case_types)]
pub type pt_regs = core::ffi::c_void;

/* If both have dummy values, the syscalls are not supported. */
#[inline]
pub fn should_skip_syscall_handle() -> bool {
    __NR_SCHED_SETATTR < 0 && __NR_SCHED_SETSCHEDULER < 0
}

#[inline]
pub fn is_supported_type(type_: u8) -> bool {
    type_ == DL_TASK || type_ == DL_SERVER_FAIR || type_ == DL_SERVER_EXT
}

#[inline]
pub fn is_server_type(type_: u8) -> bool {
    is_supported_type(type_) && type_ != DL_TASK
}

#[inline]
pub const fn fair_server_id(cpu: i32) -> i32 { -cpu }
#[inline]
pub fn ext_server_id(cpu: i32) -> i32 { -cpu - num_possible_cpus() }
pub fn no_server_id() -> i32 { -2 * num_possible_cpus() }

#[inline]
pub unsafe fn get_entity_id(dl_se: *mut sched_dl_entity, cpu: i32, type_: u8) -> i32 {
    if dl_server(dl_se) && type_ != DL_TASK {
        if type_ == DL_SERVER_FAIR { return fair_server_id(cpu); }
        if type_ == DL_SERVER_EXT { return ext_server_id(cpu); }
        return no_server_id();
    }
    dl_task_of(dl_se).as_ref().unwrap().pid
}

#[inline]
pub unsafe fn task_is_scx_enabled(tsk: *mut task_struct) -> bool {
    cfg!(feature = "CONFIG_SCHED_CLASS_EXT") && (*tsk).sched_class == rv_ext_sched_class
}

#[inline]
pub unsafe fn get_server_type(tsk: *mut task_struct) -> u8 {
    if (*tsk).policy == SCHED_NORMAL || (*tsk).policy == SCHED_EXT ||
       (*tsk).policy == SCHED_BATCH || (*tsk).policy == SCHED_IDLE {
        return if task_is_scx_enabled(tsk) { DL_SERVER_EXT } else { DL_SERVER_FAIR };
    }
    DL_OTHER
}

/* Expand id and target as arguments for DA functions. */
#[inline]
pub unsafe fn expand_id(dl_se: *mut sched_dl_entity, cpu: i32, type_: u8) -> (i32, *mut sched_dl_entity) {
    (get_entity_id(dl_se, cpu, type_), dl_se)
}

/* The following declarations preserve the header's conditional monitor helpers. */
#[cfg(feature = "RV_MON_TYPE")]
#[inline]
pub unsafe fn get_server(tsk: *mut task_struct, type_: u8) -> *mut sched_dl_entity {
    if !(*tsk).dl_server.is_null() && get_server_type(tsk) == type_ { return (*tsk).dl_server; }
    if type_ == DL_SERVER_FAIR { return da_get_target_by_id(fair_server_id(task_cpu(tsk))); }
    if type_ == DL_SERVER_EXT { return da_get_target_by_id(ext_server_id(task_cpu(tsk))); }
    core::ptr::null_mut()
}

/*
 * The declarations below correspond to kernel structures, constants, and
 * monitor utilities supplied by the included headers in the C source.
 */
extern "C" {
    fn syscall_get_arguments(task: *mut task_struct, regs: *mut pt_regs, args: *mut usize);
    fn copy_struct_from_user(dst: *mut core::ffi::c_void, size: usize,
                             src: *const core::ffi::c_void, usize_: usize) -> i32;
    fn da_get_target_by_id(id: i32) -> *mut sched_dl_entity;
    fn da_create_empty_storage(id: i32) -> bool;
    fn da_create_storage(id: i32, dl_se: *mut sched_dl_entity, extra: *mut core::ffi::c_void) -> bool;
    fn da_monitor_destroy();
    fn da_destroy_storage(id: i32);
    fn num_possible_cpus() -> i32;
    fn dl_server(dl_se: *mut sched_dl_entity) -> bool;
    fn dl_task_of(dl_se: *mut sched_dl_entity) -> *mut task_struct;
    fn task_cpu(tsk: *mut task_struct) -> i32;
}

/* C constants retained as externally supplied symbols. */
extern "C" {
    static current: *mut task_struct;
    static tasklist_lock: core::ffi::c_void;
}

/*
 * extract_params is kept as a direct low-level declaration because sched_attr
 * and pt_regs are defined by the kernel headers included by the original file.
 */
#[inline]
pub unsafe fn extract_params(regs: *mut pt_regs, id: long, pid_out: *mut pid_t) -> i32 {
    let mut args = [0usize; 6];
    syscall_get_arguments(current, regs, args.as_mut_ptr());
    if id == __NR_SCHED_SETSCHEDULER {
        *pid_out = args[0] as pid_t;
        return (args[1] as i32) & !SCHED_RESET_ON_FORK;
    }
    if id == __NR_SCHED_SETATTR {
        *pid_out = args[0] as pid_t;
        /* sched_attr fields and copy_struct_from_user are external kernel ABI. */
        return sched_attr_policy_from_user(args[1] as *const core::ffi::c_void);
    }
    -EINVAL
}

extern "C" {
    fn sched_attr_policy_from_user(attr: *const core::ffi::c_void) -> i32;
}

#[cfg(feature = "RV_MON_TYPE")]
#[inline]
pub unsafe fn init_storage(skip_tasks: bool) -> i32 {
    /* for_each_possible_cpu and tasklist traversal are kernel macros; their
       control flow is represented by the corresponding external helper. */
    if !init_server_storage() { da_monitor_destroy(); return -ENOMEM; }
    if skip_tasks { return 0; }
    if !init_deadline_task_storage() { da_monitor_destroy(); return -ENOMEM; }
    0
}

extern "C" {
    fn init_server_storage() -> bool;
    fn init_deadline_task_storage() -> bool;
    fn handle_newtask_storage(task: *mut task_struct);
    fn handle_exit_storage(task: *mut task_struct);
}

#[cfg(feature = "RV_MON_TYPE")]
#[inline]
pub unsafe fn handle_newtask(_data: *mut core::ffi::c_void, task: *mut task_struct, _flags: u64) {
    handle_newtask_storage(task);
}

#[cfg(feature = "RV_MON_TYPE")]
#[inline]
pub unsafe fn handle_exit(_data: *mut core::ffi::c_void, task: *mut task_struct, _group_dead: bool) {
    handle_exit_storage(task);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
