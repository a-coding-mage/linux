/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/pid_namespace.h. Included C headers provide the
// referenced types and operations.

pub const MAX_PID_NS_LEVEL: u32 = 32;

// struct fs_pin;
#[repr(C)]
pub struct fs_pin {
    _private: [u8; 0],
}

#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))]
pub const MEMFD_NOEXEC_SCOPE_EXEC: i32 = 0;
#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))]
pub const MEMFD_NOEXEC_SCOPE_NOEXEC_SEAL: i32 = 1;
#[cfg(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))]
pub const MEMFD_NOEXEC_SCOPE_NOEXEC_ENFORCED: i32 = 2;

#[repr(C)]
pub struct pid_namespace {
    pub idr: idr,
    pub rcu: rcu_head,
    pub pid_allocated: c_uint,
    #[cfg(feature = "CONFIG_SYSCTL")]
    #[cfg(feature = "CONFIG_MEMFD_CREATE")]
    pub memfd_noexec_scope: c_int,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub set: ctl_table_set,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub sysctls: *mut ctl_table_header,
    pub child_reaper: *mut task_struct,
    pub pid_cachep: *mut kmem_cache,
    pub level: c_uint,
    pub pid_max: c_int,
    pub parent: *mut pid_namespace,
    #[cfg(feature = "CONFIG_BSD_PROCESS_ACCT")]
    pub bacct: *mut fs_pin,
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub reboot: c_int,
    pub ns: ns_common,
    pub work: work_struct,
}

extern "C" {
    pub static mut init_pid_ns: pid_namespace;
}

pub const PIDNS_ADDING: c_uint = 1u32 << 31;

#[cfg(feature = "CONFIG_PID_NS")]
#[inline]
pub unsafe fn to_pid_ns(ns: *mut ns_common) -> *mut pid_namespace {
    container_of!(ns, pid_namespace, ns)
}

#[cfg(feature = "CONFIG_PID_NS")]
#[inline]
pub unsafe fn get_pid_ns(ns: *mut pid_namespace) -> *mut pid_namespace {
    ns_ref_inc!(ns);
    ns
}

#[cfg(all(feature = "CONFIG_PID_NS", feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
pub unsafe fn pidns_memfd_noexec_scope(mut ns: *mut pid_namespace) -> c_int {
    let mut scope: c_int = MEMFD_NOEXEC_SCOPE_EXEC;
    while !ns.is_null() {
        scope = core::cmp::max(scope, READ_ONCE!((*ns).memfd_noexec_scope));
        ns = (*ns).parent;
    }
    scope
}

#[cfg(any(not(feature = "CONFIG_PID_NS"), not(all(feature = "CONFIG_SYSCTL", feature = "CONFIG_MEMFD_CREATE"))))]
#[inline]
pub unsafe fn pidns_memfd_noexec_scope(_ns: *mut pid_namespace) -> c_int { 0 }

#[cfg(feature = "CONFIG_PID_NS")]
extern "C" {
    pub fn copy_pid_ns(flags: u64, user_ns: *mut user_namespace, ns: *mut pid_namespace) -> *mut pid_namespace;
    pub fn zap_pid_ns_processes(pid_ns: *mut pid_namespace);
    pub fn reboot_pid_ns(pid_ns: *mut pid_namespace, cmd: c_int) -> c_int;
    pub fn put_pid_ns(ns: *mut pid_namespace);
    pub fn pidns_is_ancestor(child: *mut pid_namespace, ancestor: *mut pid_namespace) -> bool;
}

#[cfg(not(feature = "CONFIG_PID_NS"))]
#[inline]
pub unsafe fn get_pid_ns(ns: *mut pid_namespace) -> *mut pid_namespace { ns }

#[cfg(not(feature = "CONFIG_PID_NS"))]
#[inline]
pub unsafe fn copy_pid_ns(flags: u64, _user_ns: *mut user_namespace, ns: *mut pid_namespace) -> *mut pid_namespace {
    if flags & CLONE_NEWPID != 0 { ERR_PTR!(-EINVAL) } else { ns }
}

#[cfg(not(feature = "CONFIG_PID_NS"))]
#[inline]
pub unsafe fn put_pid_ns(_ns: *mut pid_namespace) {}

#[cfg(not(feature = "CONFIG_PID_NS"))]
#[inline]
pub unsafe fn zap_pid_ns_processes(_ns: *mut pid_namespace) { BUG!(); }

#[cfg(not(feature = "CONFIG_PID_NS"))]
#[inline]
pub unsafe fn reboot_pid_ns(_pid_ns: *mut pid_namespace, _cmd: c_int) -> c_int { 0 }

#[cfg(not(feature = "CONFIG_PID_NS"))]
#[inline]
pub unsafe fn pidns_is_ancestor(_child: *mut pid_namespace, _ancestor: *mut pid_namespace) -> bool { false }

extern "C" {
    pub fn task_active_pid_ns(tsk: *mut task_struct) -> *mut pid_namespace;
    pub fn pidhash_init();
    pub fn pid_idr_init();
    pub fn register_pidns_sysctls(pidns: *mut pid_namespace) -> c_int;
    pub fn unregister_pidns_sysctls(pidns: *mut pid_namespace);
}

#[inline]
pub unsafe fn task_is_in_init_pid_ns(tsk: *mut task_struct) -> bool {
    task_active_pid_ns(tsk) == core::ptr::addr_of_mut!(init_pid_ns)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
