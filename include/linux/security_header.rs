//! Rust translation of Linux `security.h`.
//!
//! Kernel configuration conditionals from the original header are retained as
//! comments; the declarations below intentionally use the kernel's C ABI and
//! opaque types supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;
pub type size_t = usize;
pub type ssize_t = isize;
pub type loff_t = i64;
pub type bool_ = bool;

#[repr(C)] pub struct linux_binprm { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct rlimit { _private: [u8; 0] }
#[repr(C)] pub struct kernel_siginfo { _private: [u8; 0] }
#[repr(C)] pub struct sembuf { _private: [u8; 0] }
#[repr(C)] pub struct kern_ipc_perm { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct vfsmount { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct qstr { _private: [u8; 0] }
#[repr(C)] pub struct iattr { _private: [u8; 0] }
#[repr(C)] pub struct fown_struct { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct msg_msg { _private: [u8; 0] }
#[repr(C)] pub struct xattr { _private: [u8; 0] }
#[repr(C)] pub struct kernfs_node { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_sec_ctx { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct fs_context { _private: [u8; 0] }
#[repr(C)] pub struct fs_parameter { _private: [u8; 0] }
#[repr(C)] pub struct user_namespace { _private: [u8; 0] }
#[repr(C)] pub struct timezone { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { _private: [u8; 0] }
#[repr(C)] pub struct ctl_table { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct posix_acl { _private: [u8; 0] }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct sock { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { _private: [u8; 0] }
#[repr(C)] pub struct msghdr { _private: [u8; 0] }
#[repr(C)] pub struct flowi_common { _private: [u8; 0] }
#[repr(C)] pub struct request_sock { _private: [u8; 0] }
#[repr(C)] pub struct sctp_association { _private: [u8; 0] }
#[repr(C)] pub struct key { _private: [u8; 0] }
#[repr(C)] pub struct audit_krule { _private: [u8; 0] }

pub type gfp_t = u32;
pub type umode_t = u16;
pub type dev_t = u64;
pub type pid_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type kernel_cap_t = u64;
pub type sockptr_t = *mut c_void;

pub const CAP_OPT_NONE: u32 = 0x0;
pub const CAP_OPT_NOAUDIT: u32 = 1 << 1;
pub const CAP_OPT_INSETID: u32 = 1 << 2;
pub const SECURITY_LSM_NATIVE_LABELS: u32 = 1;
pub const LSM_SETID_ID: u32 = 1;
pub const LSM_SETID_RE: u32 = 2;
pub const LSM_SETID_RES: u32 = 4;
pub const LSM_SETID_FS: u32 = 8;
pub const LSM_PRLIMIT_READ: u32 = 1;
pub const LSM_PRLIMIT_WRITE: u32 = 2;
pub const LSM_UNSAFE_SHARE: u32 = 1;
pub const LSM_UNSAFE_PTRACE: u32 = 2;
pub const LSM_UNSAFE_NO_NEW_PRIVS: u32 = 4;

#[repr(C)] #[derive(Copy, Clone)] pub enum lsm_event { LSM_POLICY_CHANGE, LSM_STARTED_ALL }
#[repr(C)] #[derive(Copy, Clone)] pub enum lsm_integrity_type {
    LSM_INT_DMVERITY_SIG_VALID, LSM_INT_DMVERITY_ROOTHASH,
    LSM_INT_FSVERITY_BUILTINSIG_VALID,
}
#[repr(C)] #[derive(Copy, Clone)] pub enum lockdown_reason {
    LOCKDOWN_NONE, LOCKDOWN_MODULE_SIGNATURE, LOCKDOWN_DEV_MEM, LOCKDOWN_EFI_TEST,
    LOCKDOWN_KEXEC, LOCKDOWN_HIBERNATION, LOCKDOWN_PCI_ACCESS, LOCKDOWN_IOPORT,
    LOCKDOWN_MSR, LOCKDOWN_ACPI_TABLES, LOCKDOWN_DEVICE_TREE, LOCKDOWN_PCMCIA_CIS,
    LOCKDOWN_TIOCSSERIAL, LOCKDOWN_MODULE_PARAMETERS, LOCKDOWN_MMIOTRACE,
    LOCKDOWN_DEBUGFS, LOCKDOWN_XMON_WR, LOCKDOWN_BPF_WRITE_USER, LOCKDOWN_DBG_WRITE_KERNEL,
    LOCKDOWN_RTAS_ERROR_INJECTION, LOCKDOWN_XEN_USER_ACTIONS, LOCKDOWN_INTEGRITY_MAX,
    LOCKDOWN_KCORE, LOCKDOWN_KPROBES, LOCKDOWN_BPF_READ_KERNEL, LOCKDOWN_DBG_READ_KERNEL,
    LOCKDOWN_PERF, LOCKDOWN_TRACEFS, LOCKDOWN_XMON_RW, LOCKDOWN_XFRM_SECRET,
    LOCKDOWN_CONFIDENTIALITY_MAX,
}

#[repr(C)] pub struct dm_verity_digest { pub alg: *const c_char, pub digest: *const u8, pub digest_len: size_t }
#[repr(C)] pub struct lsm_context { pub context: *mut c_char, pub len: u32, pub id: c_int }
#[repr(C)] pub struct lsm_prop { pub selinux: [u8; 0], pub smack: [u8; 0], pub apparmor: [u8; 0], pub bpf: [u8; 0] }
pub type initxattrs = Option<unsafe extern "C" fn(*mut inode, *const xattr, *mut c_void) -> c_int>;

extern "C" {
    pub static lockdown_reasons: [*const c_char; 32];
    pub fn cap_capable(cred: *const cred, ns: *mut user_namespace, cap: c_int, opts: u32) -> c_int;
    pub fn cap_settime(ts: *const timespec64, tz: *const timezone) -> c_int;
    pub fn cap_ptrace_access_check(child: *mut task_struct, mode: u32) -> c_int;
    pub fn cap_ptrace_traceme(parent: *mut task_struct) -> c_int;
    pub fn cap_mmap_addr(addr: c_ulong) -> c_int;
    pub fn cap_task_fix_setuid(new: *mut cred, old: *const cred, flags: c_int) -> c_int;
    pub fn cap_task_prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    pub fn cap_task_setscheduler(p: *mut task_struct) -> c_int;
    pub fn cap_task_setioprio(p: *mut task_struct, ioprio: c_int) -> c_int;
    pub fn cap_task_setnice(p: *mut task_struct, nice: c_int) -> c_int;
    pub fn cap_vm_enough_memory(mm: *mut mm_struct, pages: c_long) -> c_int;
    pub fn security_init() -> c_int;
    pub fn early_security_init() -> c_int;
    pub fn lsm_name_to_attr(name: *const c_char) -> u64;
    pub fn security_capable(cred: *const cred, ns: *mut user_namespace, cap: c_int, opts: u32) -> c_int;
    pub fn security_ptrace_access_check(child: *mut task_struct, mode: u32) -> c_int;
    pub fn security_ptrace_traceme(parent: *mut task_struct) -> c_int;
    pub fn security_inode_alloc(inode: *mut inode, gfp: gfp_t) -> c_int;
    pub fn security_inode_free(inode: *mut inode);
    pub fn security_file_permission(file: *mut file, mask: c_int) -> c_int;
    pub fn security_file_open(file: *mut file) -> c_int;
    pub fn security_task_alloc(task: *mut task_struct, clone_flags: u64) -> c_int;
    pub fn security_task_free(task: *mut task_struct);
    pub fn security_task_kill(p: *mut task_struct, info: *mut kernel_siginfo, sig: c_int, cred: *const cred) -> c_int;
    pub fn security_locked_down(what: lockdown_reason) -> c_int;
}

// CONFIG_SECURITY, CONFIG_SECURITY_NETWORK, CONFIG_SECURITY_PATH, CONFIG_KEYS,
// CONFIG_AUDIT, CONFIG_SECURITYFS, CONFIG_BPF_SYSCALL, CONFIG_PERF_EVENTS and
// CONFIG_IO_URING provide the remaining declarations and inline default hooks.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
