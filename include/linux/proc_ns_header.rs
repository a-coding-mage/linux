/* SPDX-License-Identifier: GPL-2.0 */
/*
 * procfs namespace bits
 */

// Dependencies supplied by the corresponding Linux namespace headers.

pub struct pid_namespace;
pub struct nsset;
pub struct path;
pub struct task_struct;
pub struct inode;
pub struct ns_common;
pub struct user_namespace;

#[repr(C)]
pub struct proc_ns_operations {
    pub name: *const ::core::ffi::c_char,
    pub real_ns_name: *const ::core::ffi::c_char,
    pub get: Option<unsafe extern "C" fn(task: *mut task_struct) -> *mut ns_common>,
    pub put: Option<unsafe extern "C" fn(ns: *mut ns_common)>,
    pub install: Option<unsafe extern "C" fn(nsset: *mut nsset, ns: *mut ns_common) -> ::core::ffi::c_int>,
    pub owner: Option<unsafe extern "C" fn(ns: *mut ns_common) -> *mut user_namespace>,
    pub get_parent: Option<unsafe extern "C" fn(ns: *mut ns_common) -> *mut ns_common>,
}

// __randomize_layout

extern "C" {
    pub static netns_operations: proc_ns_operations;
    pub static utsns_operations: proc_ns_operations;
    pub static ipcns_operations: proc_ns_operations;
    pub static pidns_operations: proc_ns_operations;
    pub static pidns_for_children_operations: proc_ns_operations;
    pub static userns_operations: proc_ns_operations;
    pub static mntns_operations: proc_ns_operations;
    pub static cgroupns_operations: proc_ns_operations;
    pub static timens_operations: proc_ns_operations;
    pub static timens_for_children_operations: proc_ns_operations;
}

/*
 * We always define these enumerators
 */
pub const PROC_IPC_INIT_INO: _ = IPC_NS_INIT_INO;
pub const PROC_UTS_INIT_INO: _ = UTS_NS_INIT_INO;
pub const PROC_USER_INIT_INO: _ = USER_NS_INIT_INO;
pub const PROC_PID_INIT_INO: _ = PID_NS_INIT_INO;
pub const PROC_CGROUP_INIT_INO: _ = CGROUP_NS_INIT_INO;
pub const PROC_TIME_INIT_INO: _ = TIME_NS_INIT_INO;
pub const PROC_NET_INIT_INO: _ = NET_NS_INIT_INO;
pub const PROC_MNT_INIT_INO: _ = MNT_NS_INIT_INO;

// CONFIG_PROC_FS conditionally selects the external declarations or the
// inline fallback definitions below.
#[cfg(CONFIG_PROC_FS)]
extern "C" {
    pub fn proc_alloc_inum(pino: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn proc_free_inum(inum: ::core::ffi::c_uint);
}

#[cfg(not(CONFIG_PROC_FS))]
pub unsafe fn proc_alloc_inum(inum: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int {
    *inum = 1;
    0
}

#[cfg(not(CONFIG_PROC_FS))]
pub unsafe fn proc_free_inum(_inum: ::core::ffi::c_uint) {}

#[macro_export]
macro_rules! get_proc_ns {
    ($inode:expr) => {
        ($inode).i_private as *mut $crate::ns_common
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
