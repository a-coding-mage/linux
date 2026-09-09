/* SPDX-License-Identifier: GPL-2.0 */

// Declarations translated from ipc_namespace.h. Included Linux types and
// helper macros are supplied by other translated headers.

pub struct user_namespace;

#[repr(C)]
pub struct ipc_ids {
    pub in_use: ::core::ffi::c_int,
    pub seq: u16,
    pub rwsem: rw_semaphore,
    pub ipcs_idr: idr,
    pub max_idx: ::core::ffi::c_int,
    pub last_idx: ::core::ffi::c_int, /* For wrap around detection */
    #[cfg(feature = "CONFIG_CHECKPOINT_RESTORE")]
    pub next_id: ::core::ffi::c_int,
    pub key_ht: rhashtable,
}

#[repr(C)]
pub struct ipc_namespace {
    pub ids: [ipc_ids; 3],
    pub sem_ctls: [::core::ffi::c_int; 4],
    pub used_sems: ::core::ffi::c_int,
    pub msg_ctlmax: u32,
    pub msg_ctlmnb: u32,
    pub msg_ctlmni: u32,
    pub percpu_msg_bytes: percpu_counter,
    pub percpu_msg_hdrs: percpu_counter,
    pub shm_ctlmax: usize,
    pub shm_ctlall: usize,
    pub shm_tot: ::core::ffi::c_ulong,
    pub shm_ctlmni: ::core::ffi::c_int,
    /* Defines whether IPC_RMID is forced for _all_ shm segments regardless
     * of shmctl() */
    pub shm_rmid_forced: ::core::ffi::c_int,
    pub ipcns_nb: notifier_block,
    /* The kern_mount of the mqueuefs sb.  We take a ref on it */
    pub mq_mnt: *mut vfsmount,
    /* # queues in this ns, protected by mq_lock */
    pub mq_queues_count: u32,
    /* next fields are set through sysctl */
    pub mq_queues_max: u32,
    pub mq_msg_max: u32,
    pub mq_msgsize_max: u32,
    pub mq_msg_default: u32,
    pub mq_msgsize_default: u32,
    pub mq_set: ctl_table_set,
    pub mq_sysctls: *mut ctl_table_header,
    pub ipc_set: ctl_table_set,
    pub ipc_sysctls: *mut ctl_table_header,
    /* user_ns which owns the ipc ns */
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub mnt_llist: llist_node,
    pub ns: ns_common,
}

extern "C" {
    pub static mut init_ipc_ns: ipc_namespace;
    pub static mut mq_lock: spinlock_t;
}

#[cfg(feature = "CONFIG_SYSVIPC")]
extern "C" {
    pub fn shm_destroy_orphaned(ns: *mut ipc_namespace);
}
#[cfg(not(feature = "CONFIG_SYSVIPC"))]
#[inline]
pub unsafe fn shm_destroy_orphaned(_ns: *mut ipc_namespace) {}

#[cfg(feature = "CONFIG_POSIX_MQUEUE")]
extern "C" {
    pub fn mq_init_ns(ns: *mut ipc_namespace) -> ::core::ffi::c_int;
}

pub const DFLT_QUEUESMAX: ::core::ffi::c_int = 256;
pub const MIN_MSGMAX: ::core::ffi::c_int = 1;
pub const DFLT_MSG: u32 = 10;
pub const DFLT_MSGMAX: ::core::ffi::c_int = 10;
pub const HARD_MSGMAX: ::core::ffi::c_int = 65536;
pub const MIN_MSGSIZEMAX: ::core::ffi::c_int = 128;
pub const DFLT_MSGSIZE: u32 = 8192;
pub const DFLT_MSGSIZEMAX: ::core::ffi::c_int = 8192;
pub const HARD_MSGSIZEMAX: ::core::ffi::c_int = 16 * 1024 * 1024;

#[cfg(not(feature = "CONFIG_POSIX_MQUEUE"))]
#[inline]
pub unsafe fn mq_init_ns(_ns: *mut ipc_namespace) -> ::core::ffi::c_int { 0 }

#[cfg(feature = "CONFIG_IPC_NS")]
#[inline]
pub unsafe fn to_ipc_ns(ns: *mut ns_common) -> *mut ipc_namespace {
    container_of!(ns, ipc_namespace, ns)
}

#[cfg(feature = "CONFIG_IPC_NS")]
extern "C" {
    pub fn copy_ipcs(flags: u64, user_ns: *mut user_namespace, ns: *mut ipc_namespace) -> *mut ipc_namespace;
    pub fn put_ipc_ns(ns: *mut ipc_namespace);
}

#[cfg(feature = "CONFIG_IPC_NS")]
#[inline]
pub unsafe fn get_ipc_ns(ns: *mut ipc_namespace) -> *mut ipc_namespace {
    if !ns.is_null() { ns_ref_inc(ns); }
    ns
}

#[cfg(feature = "CONFIG_IPC_NS")]
#[inline]
pub unsafe fn get_ipc_ns_not_zero(ns: *mut ipc_namespace) -> *mut ipc_namespace {
    if !ns.is_null() && ns_ref_get(ns) { return ns; }
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_IPC_NS"))]
#[inline]
pub unsafe fn copy_ipcs(flags: u64, _user_ns: *mut user_namespace, ns: *mut ipc_namespace) -> *mut ipc_namespace {
    if flags & CLONE_NEWIPC != 0 { return ERR_PTR(-EINVAL); }
    ns
}
#[cfg(not(feature = "CONFIG_IPC_NS"))]
#[inline]
pub unsafe fn get_ipc_ns(ns: *mut ipc_namespace) -> *mut ipc_namespace { ns }
#[cfg(not(feature = "CONFIG_IPC_NS"))]
#[inline]
pub unsafe fn get_ipc_ns_not_zero(ns: *mut ipc_namespace) -> *mut ipc_namespace { ns }
#[cfg(not(feature = "CONFIG_IPC_NS"))]
#[inline]
pub unsafe fn put_ipc_ns(_ns: *mut ipc_namespace) {}

#[cfg(feature = "CONFIG_POSIX_MQUEUE_SYSCTL")]
extern "C" {
    pub fn retire_mq_sysctls(ns: *mut ipc_namespace);
    pub fn setup_mq_sysctls(ns: *mut ipc_namespace) -> bool;
}
#[cfg(not(feature = "CONFIG_POSIX_MQUEUE_SYSCTL"))]
#[inline]
pub unsafe fn retire_mq_sysctls(_ns: *mut ipc_namespace) {}
#[cfg(not(feature = "CONFIG_POSIX_MQUEUE_SYSCTL"))]
#[inline]
pub unsafe fn setup_mq_sysctls(_ns: *mut ipc_namespace) -> bool { true }

#[cfg(feature = "CONFIG_SYSVIPC_SYSCTL")]
extern "C" {
    pub fn setup_ipc_sysctls(ns: *mut ipc_namespace) -> bool;
    pub fn retire_ipc_sysctls(ns: *mut ipc_namespace);
}
#[cfg(not(feature = "CONFIG_SYSVIPC_SYSCTL"))]
#[inline]
pub unsafe fn retire_ipc_sysctls(_ns: *mut ipc_namespace) {}
#[cfg(not(feature = "CONFIG_SYSVIPC_SYSCTL"))]
#[inline]
pub unsafe fn setup_ipc_sysctls(_ns: *mut ipc_namespace) -> bool { true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
