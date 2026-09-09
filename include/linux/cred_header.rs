/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Credentials management - Rust translation of linux/cred.h. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct group_info {
    pub usage: refcount_t,
    pub ngroups: i32,
    pub gid: [kgid_t; 0],
}

pub struct cred {
    pub usage: atomic_long_t,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub suid: kuid_t,
    pub sgid: kgid_t,
    pub euid: kuid_t,
    pub egid: kgid_t,
    pub fsuid: kuid_t,
    pub fsgid: kgid_t,
    pub securebits: u32,
    pub cap_inheritable: kernel_cap_t,
    pub cap_permitted: kernel_cap_t,
    pub cap_effective: kernel_cap_t,
    pub cap_bset: kernel_cap_t,
    pub cap_ambient: kernel_cap_t,
    // CONFIG_KEYS fields are present when that build option is enabled.
    #[cfg(feature = "CONFIG_KEYS")]
    pub jit_keyring: u8,
    #[cfg(feature = "CONFIG_KEYS")]
    pub session_keyring: *mut key,
    #[cfg(feature = "CONFIG_KEYS")]
    pub process_keyring: *mut key,
    #[cfg(feature = "CONFIG_KEYS")]
    pub thread_keyring: *mut key,
    #[cfg(feature = "CONFIG_KEYS")]
    pub request_key_auth: *mut key,
    // CONFIG_SECURITY field.
    #[cfg(feature = "CONFIG_SECURITY")]
    pub security: *mut core::ffi::c_void,
    pub user: *mut user_struct,
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub group_info: *mut group_info,
    pub rcu_or_non_rcu: cred_rcu_union,
}

#[repr(C)]
pub union cred_rcu_union {
    pub non_rcu: i32,
    pub rcu: rcu_head,
}

pub unsafe fn get_group_info(gi: *mut group_info) -> *mut group_info {
    refcount_inc(unsafe { &mut (*gi).usage });
    gi
}

pub unsafe fn put_group_info(group_info: *mut group_info) {
    if refcount_dec_and_test(unsafe { &mut (*group_info).usage }) {
        groups_free(group_info);
    }
}

#[cfg(feature = "CONFIG_MULTIUSER")]
extern "C" {
    pub fn groups_alloc(n: i32) -> *mut group_info;
    pub fn groups_free(gi: *mut group_info);
    pub fn in_group_p(gid: kgid_t) -> i32;
    pub fn in_egroup_p(gid: kgid_t) -> i32;
    pub fn groups_search(gi: *const group_info, gid: kgid_t) -> i32;
    pub fn set_current_groups(gi: *mut group_info) -> i32;
    pub fn set_groups(cred: *mut cred, gi: *mut group_info);
    pub fn may_setgroups() -> bool;
    pub fn groups_sort(gi: *mut group_info);
}

#[cfg(not(feature = "CONFIG_MULTIUSER"))]
pub unsafe fn groups_free(_group_info: *mut group_info) {}
#[cfg(not(feature = "CONFIG_MULTIUSER"))]
pub unsafe fn in_group_p(_grp: kgid_t) -> i32 { 1 }
#[cfg(not(feature = "CONFIG_MULTIUSER"))]
pub unsafe fn in_egroup_p(_grp: kgid_t) -> i32 { 1 }
#[cfg(not(feature = "CONFIG_MULTIUSER"))]
pub unsafe fn groups_search(_group_info: *const group_info, _grp: kgid_t) -> i32 { 1 }

extern "C" {
    pub static mut init_task: task_struct;
    pub fn __put_cred(c: *mut cred);
    pub fn exit_creds(task: *mut task_struct);
    pub fn copy_creds(task: *mut task_struct, clone_flags: u64) -> i32;
    pub fn get_task_cred(task: *mut task_struct) -> *const cred;
    pub fn cred_alloc_blank() -> *mut cred;
    pub fn prepare_creds() -> *mut cred;
    pub fn prepare_exec_creds() -> *mut cred;
    pub fn commit_creds(c: *mut cred) -> i32;
    pub fn abort_creds(c: *mut cred);
    pub fn prepare_kernel_cred(task: *mut task_struct) -> *mut cred;
    pub fn set_security_override(c: *mut cred, secid: u32) -> i32;
    pub fn set_create_files_as(c: *mut cred, inode: *mut inode) -> i32;
    pub fn cred_fscmp(a: *const cred, b: *const cred) -> i32;
    pub fn cred_init();
    pub fn set_cred_ucounts(c: *mut cred) -> i32;
}

pub unsafe fn kernel_cred() -> *const cred { (*(&raw mut init_task)).cred }

pub unsafe fn cap_ambient_invariant_ok(c: *const cred) -> bool {
    cap_issubset((*c).cap_ambient, cap_intersect((*c).cap_permitted, (*c).cap_inheritable))
}

pub unsafe fn get_cred_many(c: *const cred, nr: i32) -> *const cred {
    if c.is_null() { return c; }
    (*(c as *mut cred)).rcu_or_non_rcu.non_rcu = 0;
    atomic_long_add(nr, &mut (*(c as *mut cred)).usage);
    c
}
pub unsafe fn get_cred(c: *const cred) -> *const cred { get_cred_many(c, 1) }
pub unsafe fn get_cred_rcu(c: *const cred) -> *const cred {
    if c.is_null() || !atomic_long_inc_not_zero(&mut (*(c as *mut cred)).usage) { return core::ptr::null(); }
    (*(c as *mut cred)).rcu_or_non_rcu.non_rcu = 0; c
}
pub unsafe fn put_cred_many(c: *const cred, nr: i32) {
    if !c.is_null() && atomic_long_sub_and_test(nr, &mut (*(c as *mut cred)).usage) { __put_cred(c as *mut cred); }
}
pub unsafe fn put_cred(c: *const cred) { put_cred_many(c, 1) }

// C RCU accessors and scoped cleanup macros retain their source-level intent.
#[macro_export] macro_rules! current_cred { () => { rcu_dereference_protected(current.cred, 1) }; }
#[macro_export] macro_rules! current_real_cred { () => { rcu_dereference_protected(current.real_cred, 1) }; }
#[macro_export] macro_rules! get_current_cred { () => { $crate::get_cred(current_cred!()) }; }
#[macro_export] macro_rules! current_cred_xxx { ($x:ident) => { current_cred!().$x }; }
#[macro_export] macro_rules! current_uid { () => { current_cred_xxx!(uid) }; }
#[macro_export] macro_rules! current_gid { () => { current_cred_xxx!(gid) }; }
#[macro_export] macro_rules! current_euid { () => { current_cred_xxx!(euid) }; }
#[macro_export] macro_rules! current_egid { () => { current_cred_xxx!(egid) }; }
#[macro_export] macro_rules! current_suid { () => { current_cred_xxx!(suid) }; }
#[macro_export] macro_rules! current_sgid { () => { current_cred_xxx!(sgid) }; }
#[macro_export] macro_rules! current_fsuid { () => { current_cred_xxx!(fsuid) }; }
#[macro_export] macro_rules! current_fsgid { () => { current_cred_xxx!(fsgid) }; }
#[macro_export] macro_rules! current_cap { () => { current_cred_xxx!(cap_effective) }; }
#[macro_export] macro_rules! current_user { () => { current_cred_xxx!(user) }; }
#[macro_export] macro_rules! current_ucounts { () => { current_cred_xxx!(ucounts) }; }

// External kernel types and operations referenced by this header.
pub enum task_struct {}
pub enum inode {}
pub enum key {}
pub enum user_struct {}
pub enum user_namespace {}
pub enum ucounts {}
pub enum rcu_head {}
pub type refcount_t = i32;
pub type atomic_long_t = i64;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type kernel_cap_t = u64;
extern "C" { fn refcount_inc(v: *mut refcount_t); fn refcount_dec_and_test(v: *mut refcount_t) -> bool; fn atomic_long_add(n: i32, v: *mut atomic_long_t); fn atomic_long_inc_not_zero(v: *mut atomic_long_t) -> bool; fn atomic_long_sub_and_test(n: i32, v: *mut atomic_long_t) -> bool; fn cap_issubset(a: kernel_cap_t, b: kernel_cap_t) -> bool; fn cap_intersect(a: kernel_cap_t, b: kernel_cap_t) -> kernel_cap_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
