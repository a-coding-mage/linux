/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This is <linux/capability.h>
 *
 * Translated from the C header. The original Linux dependencies and
 * build-time configuration are expected to be supplied by other modules.
 */

// #include <uapi/linux/capability.h>
// #include <linux/uidgid.h>
// #include <linux/bits.h>

pub const _KERNEL_CAPABILITY_VERSION: u32 = _LINUX_CAPABILITY_VERSION_3;

unsafe extern "C" {
    pub static mut file_caps_enabled: ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_cap_t {
    pub val: u64,
}

#[repr(C)]
pub struct cpu_vfs_cap_data {
    pub magic_etc: __u32,
    pub rootid: kuid_t,
    pub permitted: kernel_cap_t,
    pub inheritable: kernel_cap_t,
}

pub const _USER_CAP_HEADER_SIZE: usize = ::core::mem::size_of::<__user_cap_header_struct>();
pub const _KERNEL_CAP_T_SIZE: usize = ::core::mem::size_of::<kernel_cap_t>();

#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct user_namespace { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }

pub const CAP_FS_MASK: u64 = (1u64 << CAP_CHOWN)
    | (1u64 << CAP_MKNOD)
    | (1u64 << CAP_DAC_OVERRIDE)
    | (1u64 << CAP_DAC_READ_SEARCH)
    | (1u64 << CAP_FOWNER)
    | (1u64 << CAP_FSETID)
    | (1u64 << CAP_MAC_OVERRIDE);
pub const CAP_VALID_MASK: u64 = (1u64 << (CAP_LAST_CAP + 1)) - 1;

pub const CAP_EMPTY_SET: kernel_cap_t = kernel_cap_t { val: 0 };
pub const CAP_FULL_SET: kernel_cap_t = kernel_cap_t { val: CAP_VALID_MASK };
pub const CAP_FS_SET: kernel_cap_t = kernel_cap_t { val: CAP_FS_MASK | (1u64 << CAP_LINUX_IMMUTABLE) };
pub const CAP_NFSD_SET: kernel_cap_t = kernel_cap_t { val: CAP_FS_MASK | (1u64 << CAP_SYS_RESOURCE) };

#[inline]
pub unsafe fn cap_clear(c: &mut kernel_cap_t) { c.val = 0; }
#[inline]
pub fn cap_raise(c: &mut kernel_cap_t, flag: u32) { c.val |= 1u64 << flag; }
#[inline]
pub fn cap_lower(c: &mut kernel_cap_t, flag: u32) { c.val &= !(1u64 << flag); }
#[inline]
pub fn cap_raised(c: &kernel_cap_t, flag: u32) -> bool { (c.val & (1u64 << flag)) != 0 }

#[inline] pub fn cap_combine(a: kernel_cap_t, b: kernel_cap_t) -> kernel_cap_t { kernel_cap_t { val: a.val | b.val } }
#[inline] pub fn cap_intersect(a: kernel_cap_t, b: kernel_cap_t) -> kernel_cap_t { kernel_cap_t { val: a.val & b.val } }
#[inline] pub fn cap_drop(a: kernel_cap_t, drop: kernel_cap_t) -> kernel_cap_t { kernel_cap_t { val: a.val & !drop.val } }
#[inline] pub fn cap_isclear(a: kernel_cap_t) -> bool { a.val == 0 }
#[inline] pub fn cap_isidentical(a: kernel_cap_t, b: kernel_cap_t) -> bool { a.val == b.val }
#[inline] pub fn cap_issubset(a: kernel_cap_t, set: kernel_cap_t) -> bool { (a.val & !set.val) == 0 }
#[inline] pub fn cap_drop_fs_set(a: kernel_cap_t) -> kernel_cap_t { cap_drop(a, CAP_FS_SET) }
#[inline] pub fn cap_raise_fs_set(a: kernel_cap_t, permitted: kernel_cap_t) -> kernel_cap_t { cap_combine(a, cap_intersect(permitted, CAP_FS_SET)) }
#[inline] pub fn cap_drop_nfsd_set(a: kernel_cap_t) -> kernel_cap_t { cap_drop(a, CAP_NFSD_SET) }
#[inline] pub fn cap_raise_nfsd_set(a: kernel_cap_t, permitted: kernel_cap_t) -> kernel_cap_t { cap_combine(a, cap_intersect(permitted, CAP_NFSD_SET)) }

#[cfg(CONFIG_MULTIUSER)]
unsafe extern "C" {
    pub fn has_ns_capability(t: *mut task_struct, ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
    pub fn has_capability_noaudit(t: *mut task_struct, cap: ::core::ffi::c_int) -> bool;
    pub fn has_ns_capability_noaudit(t: *mut task_struct, ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
    pub fn capable(cap: ::core::ffi::c_int) -> bool;
    pub fn ns_capable(ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
    pub fn ns_capable_noaudit(ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
    pub fn ns_capable_setid(ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
}

#[cfg(not(CONFIG_MULTIUSER))]
#[inline] pub fn has_ns_capability(_: *mut task_struct, _: *mut user_namespace, _: ::core::ffi::c_int) -> bool { true }
#[cfg(not(CONFIG_MULTIUSER))]
#[inline] pub fn has_capability_noaudit(_: *mut task_struct, _: ::core::ffi::c_int) -> bool { true }
#[cfg(not(CONFIG_MULTIUSER))]
#[inline] pub fn has_ns_capability_noaudit(_: *mut task_struct, _: *mut user_namespace, _: ::core::ffi::c_int) -> bool { true }
#[cfg(not(CONFIG_MULTIUSER))]
#[inline] pub fn capable(_: ::core::ffi::c_int) -> bool { true }
#[cfg(not(CONFIG_MULTIUSER))]
#[inline] pub fn ns_capable(_: *mut user_namespace, _: ::core::ffi::c_int) -> bool { true }
#[cfg(not(CONFIG_MULTIUSER))]
#[inline] pub fn ns_capable_noaudit(_: *mut user_namespace, _: ::core::ffi::c_int) -> bool { true }
#[cfg(not(CONFIG_MULTIUSER))]
#[inline] pub fn ns_capable_setid(_: *mut user_namespace, _: ::core::ffi::c_int) -> bool { true }

unsafe extern "C" {
    pub fn privileged_wrt_inode_uidgid(ns: *mut user_namespace, idmap: *mut mnt_idmap, inode: *const inode) -> bool;
    pub fn capable_wrt_inode_uidgid(idmap: *mut mnt_idmap, inode: *const inode, cap: ::core::ffi::c_int) -> bool;
    pub fn file_ns_capable(file: *const file, ns: *mut user_namespace, cap: ::core::ffi::c_int) -> bool;
    pub fn ptracer_capable(tsk: *mut task_struct, ns: *mut user_namespace) -> bool;
}

#[inline] pub fn perfmon_capable() -> bool { capable(CAP_PERFMON) || capable(CAP_SYS_ADMIN) }
#[inline] pub fn bpf_capable() -> bool { capable(CAP_BPF) || capable(CAP_SYS_ADMIN) }
#[inline] pub fn checkpoint_restore_ns_capable(ns: *mut user_namespace) -> bool { ns_capable(ns, CAP_CHECKPOINT_RESTORE) || ns_capable(ns, CAP_SYS_ADMIN) }
#[inline] pub fn checkpoint_restore_ns_capable_noaudit(ns: *mut user_namespace) -> bool { ns_capable_noaudit(ns, CAP_CHECKPOINT_RESTORE) || ns_capable_noaudit(ns, CAP_SYS_ADMIN) }

unsafe extern "C" {
    pub fn get_vfs_caps_from_disk(idmap: *mut mnt_idmap, dentry: *const dentry, cpu_caps: *mut cpu_vfs_cap_data) -> ::core::ffi::c_int;
    pub fn cap_convert_nscap(idmap: *mut mnt_idmap, dentry: *mut dentry, ivalue: *mut *const ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
