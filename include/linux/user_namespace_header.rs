/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies are supplied by other translated headers.

pub const UID_GID_MAP_MAX_BASE_EXTENTS: usize = 5;
pub const UID_GID_MAP_MAX_EXTENTS: usize = 340;

#[repr(C)]
pub struct uid_gid_extent {
    pub first: u32,
    pub lower_first: u32,
    pub count: u32,
}

#[repr(C)]
pub union uid_gid_map {
    pub base: uid_gid_map_base,
    pub extended: uid_gid_map_extended,
}

#[repr(C)]
pub struct uid_gid_map_base {
    pub extent: [uid_gid_extent; UID_GID_MAP_MAX_BASE_EXTENTS],
    pub nr_extents: u32,
}

#[repr(C)]
pub struct uid_gid_map_extended {
    pub forward: *mut uid_gid_extent,
    pub reverse: *mut uid_gid_extent,
}

pub const USERNS_SETGROUPS_ALLOWED: libc::c_ulong = 1;
pub const USERNS_INIT_FLAGS: libc::c_ulong = USERNS_SETGROUPS_ALLOWED;

pub enum ucounts {}

#[repr(C)]
pub enum ucount_type {
    UCOUNT_USER_NAMESPACES,
    UCOUNT_PID_NAMESPACES,
    UCOUNT_UTS_NAMESPACES,
    UCOUNT_IPC_NAMESPACES,
    UCOUNT_NET_NAMESPACES,
    UCOUNT_MNT_NAMESPACES,
    UCOUNT_CGROUP_NAMESPACES,
    UCOUNT_TIME_NAMESPACES,
    // CONFIG_INOTIFY_USER
    UCOUNT_INOTIFY_INSTANCES,
    UCOUNT_INOTIFY_WATCHES,
    // CONFIG_FANOTIFY
    UCOUNT_FANOTIFY_GROUPS,
    UCOUNT_FANOTIFY_MARKS,
    // IS_ENABLED(CONFIG_BINFMT_MISC)
    UCOUNT_BINFMT_MISC_INTERPRETERS,
    UCOUNT_COUNTS,
}

#[repr(C)]
pub enum rlimit_type {
    UCOUNT_RLIMIT_NPROC,
    UCOUNT_RLIMIT_MSGQUEUE,
    UCOUNT_RLIMIT_SIGPENDING,
    UCOUNT_RLIMIT_MEMLOCK,
    UCOUNT_RLIMIT_COUNTS,
}

// IS_ENABLED(CONFIG_BINFMT_MISC)
pub enum binfmt_misc {}

#[repr(C)]
pub struct user_namespace {
    pub uid_map: uid_gid_map,
    pub gid_map: uid_gid_map,
    pub projid_map: uid_gid_map,
    pub parent: *mut user_namespace,
    pub level: libc::c_int,
    pub owner: kuid_t,
    pub group: kgid_t,
    pub ns: ns_common,
    pub flags: libc::c_ulong,
    pub parent_could_setfcap: bool,
    // CONFIG_KEYS
    pub keyring_name_list: list_head,
    pub user_keyring_register: *mut key,
    pub keyring_sem: rw_semaphore,
    // CONFIG_PERSISTENT_KEYRINGS
    pub persistent_keyring_register: *mut key,
    pub work: work_struct,
    // CONFIG_SYSCTL
    pub set: ctl_table_set,
    pub sysctls: *mut ctl_table_header,
    pub ucounts: *mut ucounts,
    pub ucount_max: [libc::c_long; ucount_type::UCOUNT_COUNTS as usize],
    pub rlimit_max: [libc::c_long; rlimit_type::UCOUNT_RLIMIT_COUNTS as usize],
    // IS_ENABLED(CONFIG_BINFMT_MISC)
    pub binfmt_misc: *mut binfmt_misc,
}

#[repr(C)]
pub struct ucounts_struct {
    pub node: hlist_nulls_node,
    pub ns: *mut user_namespace,
    pub uid: kuid_t,
    pub rcu: rcu_head,
    pub count: rcuref_t,
    pub ucount: [atomic_long_t; ucount_type::UCOUNT_COUNTS as usize],
    pub rlimit: [atomic_long_t; rlimit_type::UCOUNT_RLIMIT_COUNTS as usize],
}

extern "C" {
    pub static mut init_user_ns: user_namespace;
    pub static mut init_ucounts: ucounts_struct;

    pub fn setup_userns_sysctls(ns: *mut user_namespace) -> bool;
    pub fn retire_userns_sysctls(ns: *mut user_namespace);
    pub fn inc_ucount(ns: *mut user_namespace, uid: kuid_t, type_: ucount_type) -> *mut ucounts;
    pub fn dec_ucount(ucounts: *mut ucounts, type_: ucount_type);
    pub fn alloc_ucounts(ns: *mut user_namespace, uid: kuid_t) -> *mut ucounts;
    pub fn put_ucounts(ucounts: *mut ucounts);

    pub fn inc_rlimit_ucounts(ucounts: *mut ucounts, type_: rlimit_type, v: libc::c_long) -> libc::c_long;
    pub fn dec_rlimit_ucounts(ucounts: *mut ucounts, type_: rlimit_type, v: libc::c_long) -> bool;
    pub fn inc_rlimit_get_ucounts(ucounts: *mut ucounts, type_: rlimit_type, override_rlimit: bool) -> libc::c_long;
    pub fn dec_rlimit_put_ucounts(ucounts: *mut ucounts, type_: rlimit_type);
    pub fn is_rlimit_overlimit(ucounts: *mut ucounts, type_: rlimit_type, max: libc::c_ulong) -> bool;
}

// The following inline functions retain their C semantics and depend on symbols from included headers.
pub unsafe fn get_ucounts(ucounts: *mut ucounts) -> *mut ucounts {
    if rcuref_get(&mut (*(ucounts as *mut ucounts_struct)).count) != 0 { ucounts } else { core::ptr::null_mut() }
}

pub unsafe fn get_rlimit_value(ucounts: *mut ucounts, type_: rlimit_type) -> libc::c_long {
    atomic_long_read(&(*(ucounts as *mut ucounts_struct)).rlimit[type_ as usize])
}

pub unsafe fn get_userns_rlimit_max(ns: *mut user_namespace, type_: rlimit_type) -> libc::c_long {
    core::ptr::read_volatile((*(ns)).rlimit_max.as_ptr().add(type_ as usize))
}

pub unsafe fn set_userns_rlimit_max(ns: *mut user_namespace, type_: rlimit_type, max: libc::c_ulong) {
    (*(ns)).rlimit_max[type_ as usize] = if max <= libc::LONG_MAX as libc::c_ulong { max as libc::c_long } else { libc::LONG_MAX };
}

pub unsafe fn to_user_ns(ns: *mut ns_common) -> *mut user_namespace {
    container_of(ns, user_namespace, ns)
}

// CONFIG_USER_NS
extern "C" {
    pub static proc_uid_seq_operations: seq_operations;
    pub static proc_gid_seq_operations: seq_operations;
    pub static proc_projid_seq_operations: seq_operations;
    pub fn proc_uid_map_write(file: *mut file, buf: *const libc::c_char, count: usize, pos: *mut loff_t) -> ssize_t;
    pub fn proc_gid_map_write(file: *mut file, buf: *const libc::c_char, count: usize, pos: *mut loff_t) -> ssize_t;
    pub fn proc_projid_map_write(file: *mut file, buf: *const libc::c_char, count: usize, pos: *mut loff_t) -> ssize_t;
    pub fn proc_setgroups_write(file: *mut file, buf: *const libc::c_char, count: usize, pos: *mut loff_t) -> ssize_t;
    pub fn proc_setgroups_show(m: *mut seq_file, v: *mut libc::c_void) -> libc::c_int;
    pub fn userns_may_setgroups(ns: *const user_namespace) -> bool;
    pub fn in_userns(ancestor: *const user_namespace, child: *const user_namespace) -> bool;
    pub fn current_in_userns(target_ns: *const user_namespace) -> bool;
    pub fn ns_get_owner(ns: *mut ns_common) -> *mut ns_common;
}

extern "C" {
    pub fn create_user_ns(new: *mut cred) -> libc::c_int;
    pub fn unshare_userns(unshare_flags: libc::c_ulong, new_cred: *mut *mut cred) -> libc::c_int;
    pub fn __put_user_ns(ns: *mut user_namespace);
}

pub unsafe fn get_user_ns(ns: *mut user_namespace) -> *mut user_namespace {
    if !ns.is_null() { ns_ref_inc(ns); }
    ns
}

pub unsafe fn put_user_ns(ns: *mut user_namespace) {
    if !ns.is_null() && ns_ref_put(ns) { __put_user_ns(ns); }
}

// When CONFIG_USER_NS is disabled, the C header provides these inline fallbacks.
pub unsafe fn get_user_ns_disabled(_ns: *mut user_namespace) -> *mut user_namespace { &mut init_user_ns }
pub unsafe fn create_user_ns_disabled(_new: *mut cred) -> libc::c_int { -EINVAL }
pub unsafe fn unshare_userns_disabled(unshare_flags: libc::c_ulong, _new_cred: *mut *mut cred) -> libc::c_int {
    if unshare_flags & CLONE_NEWUSER != 0 { -EINVAL } else { 0 }
}
pub unsafe fn put_user_ns_disabled(_ns: *mut user_namespace) {}
pub unsafe fn userns_may_setgroups_disabled(_ns: *const user_namespace) -> bool { true }
pub unsafe fn in_userns_disabled(_ancestor: *const user_namespace, _child: *const user_namespace) -> bool { true }
pub unsafe fn current_in_userns_disabled(_target_ns: *const user_namespace) -> bool { true }
pub unsafe fn ns_get_owner_disabled(_ns: *mut ns_common) -> *mut ns_common { ERR_PTR(-EPERM) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
