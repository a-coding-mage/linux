/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations supplied by the corresponding Linux headers. */
pub struct cgroup_namespace;
pub struct dentry;
pub struct ipc_namespace;
pub struct mnt_namespace;
pub struct net;
pub struct pid_namespace;
pub struct proc_ns_operations;
pub struct time_namespace;
pub struct user_namespace;
pub struct uts_namespace;

extern "C" {
    pub static mut init_cgroup_ns: cgroup_namespace;
    pub static mut init_ipc_ns: ipc_namespace;
    pub static mut init_mnt_ns: mnt_namespace;
    pub static mut init_net: net;
    pub static mut init_pid_ns: pid_namespace;
    pub static mut init_time_ns: time_namespace;
    pub static mut init_user_ns: user_namespace;
    pub static mut init_uts_ns: uts_namespace;

    pub static cgroupns_operations: proc_ns_operations;
    pub static ipcns_operations: proc_ns_operations;
    pub static mntns_operations: proc_ns_operations;
    pub static netns_operations: proc_ns_operations;
    pub static pidns_operations: proc_ns_operations;
    pub static pidns_for_children_operations: proc_ns_operations;
    pub static timens_operations: proc_ns_operations;
    pub static timens_for_children_operations: proc_ns_operations;
    pub static userns_operations: proc_ns_operations;
    pub static utsns_operations: proc_ns_operations;
}

/*
 * Namespace lifetimes are managed via a two-tier reference counting model.
 * __ns_ref controls memory lifetime; __ns_ref_active controls visibility and
 * active users. Active, inactive, and destroyed state transitions retain the
 * same semantics as the C declaration. Initial namespaces remain active.
 */
#[repr(C)]
pub struct ns_common {
    pub __ns_ref: refcount_t, /* do not use directly */
    pub ns_type: u32,
    pub stashed: *mut dentry,
    pub ops: *const proc_ns_operations,
    pub inum: c_uint,
    pub union_data: ns_common_union,
}

#[repr(C)]
pub union ns_common_union {
    pub ns_tree: ns_tree,
    pub ns_rcu: rcu_head,
}

/* The following types are provided by the included kernel headers. */
pub type c_uint = u32;

/* C's cacheline alignment wrapper is represented by the containing layout. */

#[macro_export]
macro_rules! to_ns_common {
    ($ns:expr) => {
        unsafe { &mut (*$ns).ns }
    };
}

#[macro_export]
macro_rules! ns_init_inum {
    ($ns:expr) => {{
        compile_error!("ns_init_inum! requires the namespace-specific init inode constants");
        0
    }};
}

#[macro_export]
macro_rules! ns_init_ns {
    ($ns:expr) => {{
        compile_error!("ns_init_ns! requires the namespace-specific initial namespace type");
        core::ptr::null_mut()
    }};
}

#[macro_export]
macro_rules! ns_init_id {
    ($ns:expr) => {{
        compile_error!("ns_init_id! requires the namespace-specific init ID constants");
        0
    }};
}

#[macro_export]
macro_rules! to_ns_operations {
    ($ns:expr) => {{
        compile_error!("to_ns_operations! requires CONFIG_* feature values and namespace types");
        core::ptr::null()
    }};
}

/* FOR_EACH_NS_TYPE is a canonical list of (struct type, CLONE_NEW* flag) pairs. */
#[macro_export]
macro_rules! for_each_ns_type {
    ($callback:ident) => {
        $callback!(cgroup_namespace, CLONE_NEWCGROUP);
        $callback!(ipc_namespace, CLONE_NEWIPC);
        $callback!(mnt_namespace, CLONE_NEWNS);
        $callback!(net, CLONE_NEWNET);
        $callback!(pid_namespace, CLONE_NEWPID);
        $callback!(time_namespace, CLONE_NEWTIME);
        $callback!(user_namespace, CLONE_NEWUSER);
        $callback!(uts_namespace, CLONE_NEWUTS);
    };
}

#[macro_export]
macro_rules! clone_ns_all {
    () => {
        0 | CLONE_NEWCGROUP | CLONE_NEWIPC | CLONE_NEWNS | CLONE_NEWNET
            | CLONE_NEWPID | CLONE_NEWTIME | CLONE_NEWUSER | CLONE_NEWUTS
    };
}

#[macro_export]
macro_rules! ns_common_type {
    ($ns:expr) => {{
        compile_error!("ns_common_type! requires the namespace pointer type associations");
        0
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
