/* SPDX-License-Identifier: GPL-2.0 */

/* Rust translation of the Linux Security Module hook declaration list. */
/*
 * The original LSM_HOOK macro is supplied by each consumer to generate
 * the desired hook data structures. This Rust macro preserves the complete
 * declaration stream and its source-level token arguments for such consumers.
 */
macro_rules! LSM_HOOK {
    ($($tokens:tt)*) => {};
}

/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Linux Security Module Hook declarations.
 *
 * Copyright (C) 2001 WireX Communications, Inc <chris@wirex.com>
 * Copyright (C) 2001 Greg Kroah-Hartman <greg@kroah.com>
 * Copyright (C) 2001 Networks Associates Technology, Inc <ssmalley@nai.com>
 * Copyright (C) 2001 James Morris <jmorris@intercode.com.au>
 * Copyright (C) 2001 Silicon Graphics, Inc. (Trust Technology Group)
 * Copyright (C) 2015 Intel Corporation.
 * Copyright (C) 2015 Casey Schaufler <casey@schaufler-ca.com>
 * Copyright (C) 2016 Mellanox Techonologies
 * Copyright (C) 2020 Google LLC.
 */

/*
 * The macro LSM_HOOK is used to define the data structures required by
 * the LSM framework using the pattern:
 *
 *	LSM_HOOK(<return_type>, <default_value>, <hook_name>, args...)
 *
 * struct security_hook_heads {
 *   #define LSM_HOOK(RET, DEFAULT, NAME, ...) struct hlist_head NAME;
 *   #include <linux/lsm_hook_defs.h>
 *   #undef LSM_HOOK
 * };
 */
LSM_HOOK!(int, 0, binder_set_context_mgr, const struct cred *mgr);
LSM_HOOK!(int, 0, binder_transaction, const struct cred *from,
	 const struct cred *to)
LSM_HOOK!(int, 0, binder_transfer_binder, const struct cred *from,
	 const struct cred *to)
LSM_HOOK!(int, 0, binder_transfer_file, const struct cred *from,
	 const struct cred *to, const struct file *file)
LSM_HOOK!(int, 0, ptrace_access_check, task_struct *child,
	 mode: core::ffi::c_uint)
LSM_HOOK!(int, 0, ptrace_traceme, task_struct *parent);
LSM_HOOK!(int, 0, capget, const struct task_struct *target, kernel_cap_t *effective,
	 kernel_cap_t *inheritable, kernel_cap_t *permitted)
LSM_HOOK!(int, 0, capset, cred *new, const struct cred *old,
	 const kernel_cap_t *effective, const kernel_cap_t *inheritable,
	 const kernel_cap_t *permitted)
LSM_HOOK!(int, 0, capable, const struct cred *cred, user_namespace *ns,
	 int cap, opts: core::ffi::c_uint)
LSM_HOOK!(int, 0, quotactl, int cmds, int type, int id, const struct super_block *sb);
LSM_HOOK!(int, 0, quota_on, dentry *dentry);
LSM_HOOK!(int, 0, syslog, int type);
LSM_HOOK!(int, 0, settime, const struct timespec64 *ts,
	 const struct timezone *tz)
LSM_HOOK!(int, 0, vm_enough_memory, mm_struct *mm, long pages);
LSM_HOOK!(int, 0, bprm_creds_for_exec, linux_binprm *bprm);
LSM_HOOK!(int, 0, bprm_creds_from_file, linux_binprm *bprm, const struct file *file);
LSM_HOOK!(int, 0, bprm_check_security, linux_binprm *bprm);
LSM_HOOK!(void, LSM_RET_VOID, bprm_committing_creds, const struct linux_binprm *bprm);
LSM_HOOK!(void, LSM_RET_VOID, bprm_committed_creds, const struct linux_binprm *bprm);
LSM_HOOK!(int, 0, fs_context_submount, fs_context *fc, super_block *reference);
LSM_HOOK!(int, 0, fs_context_dup, fs_context *fc,
	 fs_context *src_sc)
LSM_HOOK!(int, -ENOPARAM, fs_context_parse_param, fs_context *fc,
	 fs_parameter *param)
LSM_HOOK!(int, 0, sb_alloc_security, super_block *sb);
LSM_HOOK!(void, LSM_RET_VOID, sb_delete, super_block *sb);
LSM_HOOK!(void, LSM_RET_VOID, sb_free_security, super_block *sb);
LSM_HOOK!(void, LSM_RET_VOID, sb_free_mnt_opts, void *mnt_opts);
LSM_HOOK!(int, 0, sb_eat_lsm_opts, char *orig, void **mnt_opts);
LSM_HOOK!(int, 0, sb_mnt_opts_compat, super_block *sb, void *mnt_opts);
LSM_HOOK!(int, 0, sb_remount, super_block *sb, void *mnt_opts);
LSM_HOOK!(int, 0, sb_kern_mount, const struct super_block *sb);
LSM_HOOK!(int, 0, sb_show_options, seq_file *m, super_block *sb);
LSM_HOOK!(int, 0, sb_statfs, dentry *dentry);
LSM_HOOK!(int, 0, sb_mount, const char *dev_name, const struct path *path,
	 const char *type, flags: core::ffi::c_ulong, void *data)
LSM_HOOK!(int, 0, sb_umount, vfsmount *mnt, int flags);
LSM_HOOK!(int, 0, sb_pivotroot, const struct path *old_path,
	 const struct path *new_path)
LSM_HOOK!(int, 0, sb_set_mnt_opts, super_block *sb, void *mnt_opts,
	 kern_flags: core::ffi::c_ulong, core::ffi::c_ulong *set_kern_flags)
LSM_HOOK!(int, 0, sb_clone_mnt_opts, const struct super_block *oldsb,
	 super_block *newsb, kern_flags: core::ffi::c_ulong,
	 core::ffi::c_ulong *set_kern_flags)
LSM_HOOK!(int, 0, move_mount, const struct path *from_path,
	 const struct path *to_path)
LSM_HOOK!(int, -EOPNOTSUPP, dentry_init_security, dentry *dentry,
	 int mode, const struct qstr *name, const char **xattr_name,
	 lsm_context *cp)
LSM_HOOK!(int, 0, dentry_create_files_as, dentry *dentry, int mode,
	 const struct qstr *name, const struct cred *old, cred *new)

// #ifdef CONFIG_SECURITY_PATH
LSM_HOOK!(int, 0, path_unlink, const struct path *dir, dentry *dentry);
LSM_HOOK!(int, 0, path_mkdir, const struct path *dir, dentry *dentry,
	 umode_t mode)
LSM_HOOK!(int, 0, path_rmdir, const struct path *dir, dentry *dentry);
LSM_HOOK!(int, 0, path_mknod, const struct path *dir, dentry *dentry,
	 umode_t mode, dev: core::ffi::c_uint)
LSM_HOOK!(void, LSM_RET_VOID, path_post_mknod, mnt_idmap *idmap,
	 dentry *dentry)
LSM_HOOK!(int, 0, path_truncate, const struct path *path);
LSM_HOOK!(int, 0, path_symlink, const struct path *dir, dentry *dentry,
	 const char *old_name)
LSM_HOOK!(int, 0, path_link, dentry *old_dentry,
	 const struct path *new_dir, dentry *new_dentry)
LSM_HOOK!(int, 0, path_rename, const struct path *old_dir,
	 dentry *old_dentry, const struct path *new_dir,
	 dentry *new_dentry, flags: core::ffi::c_uint)
LSM_HOOK!(int, 0, path_chmod, const struct path *path, umode_t mode);
LSM_HOOK!(int, 0, path_chown, const struct path *path, kuid_t uid, kgid_t gid);
LSM_HOOK!(int, 0, path_chroot, const struct path *path);
// #endif /* CONFIG_SECURITY_PATH */

/* Needed for inode based security check */
LSM_HOOK!(int, 0, path_notify, const struct path *path, mask: u64,
	 obj_type: core::ffi::c_uint)
LSM_HOOK!(int, 0, inode_alloc_security, inode *inode);
LSM_HOOK!(void, LSM_RET_VOID, inode_free_security, inode *inode);
LSM_HOOK!(void, LSM_RET_VOID, inode_free_security_rcu, void *inode_security);
LSM_HOOK!(int, -EOPNOTSUPP, inode_init_security, inode *inode,
	 inode *dir, const struct qstr *qstr, xattr *xattrs,
	 int *xattr_count)
LSM_HOOK!(int, 0, inode_init_security_anon, inode *inode,
	 const struct qstr *name, const struct inode *context_inode)
LSM_HOOK!(int, 0, inode_create, inode *dir, dentry *dentry,
	 umode_t mode)
LSM_HOOK!(void, LSM_RET_VOID, inode_post_create_tmpfile, mnt_idmap *idmap,
	 inode *inode)
LSM_HOOK!(int, 0, inode_link, dentry *old_dentry, inode *dir,
	 dentry *new_dentry)
LSM_HOOK!(int, 0, inode_unlink, inode *dir, dentry *dentry);
LSM_HOOK!(int, 0, inode_symlink, inode *dir, dentry *dentry,
	 const char *old_name)
LSM_HOOK!(int, 0, inode_mkdir, inode *dir, dentry *dentry,
	 umode_t mode)
LSM_HOOK!(int, 0, inode_rmdir, inode *dir, dentry *dentry);
LSM_HOOK!(int, 0, inode_mknod, inode *dir, dentry *dentry,
	 umode_t mode, dev_t dev)
LSM_HOOK!(int, 0, inode_rename, inode *old_dir, dentry *old_dentry,
	 inode *new_dir, dentry *new_dentry)
LSM_HOOK!(int, 0, inode_readlink, dentry *dentry);
LSM_HOOK!(int, 0, inode_follow_link, dentry *dentry, inode *inode,
	 rcu: bool)
LSM_HOOK!(int, 0, inode_permission, inode *inode, int mask);
LSM_HOOK!(int, 0, inode_setattr, mnt_idmap *idmap, dentry *dentry,
	 iattr *attr)
LSM_HOOK!(void, LSM_RET_VOID, inode_post_setattr, mnt_idmap *idmap,
	 dentry *dentry, int ia_valid)
LSM_HOOK!(int, 0, inode_getattr, const struct path *path);
LSM_HOOK!(int, 0, inode_xattr_skipcap, const char *name);
LSM_HOOK!(int, 0, inode_setxattr, mnt_idmap *idmap,
	 dentry *dentry, const char *name, const void *value,
	 size_t size, int flags)
LSM_HOOK!(void, LSM_RET_VOID, inode_post_setxattr, dentry *dentry,
	 const char *name, const void *value, size_t size, int flags)
LSM_HOOK!(int, 0, inode_getxattr, dentry *dentry, const char *name);
LSM_HOOK!(int, 0, inode_listxattr, dentry *dentry);
LSM_HOOK!(int, 0, inode_removexattr, mnt_idmap *idmap,
	 dentry *dentry, const char *name)
LSM_HOOK!(void, LSM_RET_VOID, inode_post_removexattr, dentry *dentry,
	 const char *name)
LSM_HOOK!(int, 0, inode_file_setattr, dentry *dentry, file_kattr *fa);
LSM_HOOK!(int, 0, inode_file_getattr, dentry *dentry, file_kattr *fa);
LSM_HOOK!(int, 0, inode_set_acl, mnt_idmap *idmap,
	 dentry *dentry, const char *acl_name, posix_acl *kacl)
LSM_HOOK!(void, LSM_RET_VOID, inode_post_set_acl, dentry *dentry,
	 const char *acl_name, posix_acl *kacl)
LSM_HOOK!(int, 0, inode_get_acl, mnt_idmap *idmap,
	 dentry *dentry, const char *acl_name)
LSM_HOOK!(int, 0, inode_remove_acl, mnt_idmap *idmap,
	 dentry *dentry, const char *acl_name)
LSM_HOOK!(void, LSM_RET_VOID, inode_post_remove_acl, mnt_idmap *idmap,
	 dentry *dentry, const char *acl_name)
LSM_HOOK!(int, 0, inode_need_killpriv, dentry *dentry);
LSM_HOOK!(int, 0, inode_killpriv, mnt_idmap *idmap,
	 dentry *dentry)
LSM_HOOK!(int, -EOPNOTSUPP, inode_getsecurity, mnt_idmap *idmap,
	 inode *inode, const char *name, void **buffer, alloc: bool)
LSM_HOOK!(int, -EOPNOTSUPP, inode_setsecurity, inode *inode,
	 const char *name, const void *value, size_t size, int flags)
LSM_HOOK!(int, 0, inode_listsecurity, inode *inode, char **buffer,
	 ssize_t *remaining_size)
LSM_HOOK!(void, LSM_RET_VOID, inode_getlsmprop, inode *inode,
	 lsm_prop *prop)
LSM_HOOK!(int, 0, inode_copy_up, dentry *src, cred **new);
LSM_HOOK!(int, -EOPNOTSUPP, inode_copy_up_xattr, dentry *src,
	 const char *name)
LSM_HOOK!(int, 0, inode_setintegrity, const struct inode *inode,
	 lsm_integrity_type type, const void *value, size_t size)
LSM_HOOK!(int, 0, kernfs_init_security, kernfs_node *kn_dir,
	 kernfs_node *kn)
LSM_HOOK!(int, 0, file_permission, file *file, int mask);
LSM_HOOK!(int, 0, file_alloc_security, file *file);
LSM_HOOK!(void, LSM_RET_VOID, file_release, file *file);
LSM_HOOK!(void, LSM_RET_VOID, file_free_security, file *file);
LSM_HOOK!(int, 0, backing_file_alloc, file *backing_file,
	 const struct file *user_file)
LSM_HOOK!(void, LSM_RET_VOID, backing_file_free, file *backing_file);
LSM_HOOK!(int, 0, file_ioctl, file *file, cmd: core::ffi::c_uint,
	 arg: core::ffi::c_ulong)
LSM_HOOK!(int, 0, file_ioctl_compat, file *file, cmd: core::ffi::c_uint,
	 arg: core::ffi::c_ulong)
LSM_HOOK!(int, 0, mmap_addr, addr: core::ffi::c_ulong);
LSM_HOOK!(int, 0, mmap_file, file *file, reqprot: core::ffi::c_ulong,
	 prot: core::ffi::c_ulong, flags: core::ffi::c_ulong)
LSM_HOOK!(int, 0, mmap_backing_file, vm_area_struct *vma,
	 file *backing_file, file *user_file)
LSM_HOOK!(int, 0, file_mprotect, vm_area_struct *vma,
	 reqprot: core::ffi::c_ulong, prot: core::ffi::c_ulong)
LSM_HOOK!(int, 0, file_lock, file *file, cmd: core::ffi::c_uint);
LSM_HOOK!(int, 0, file_fcntl, file *file, cmd: core::ffi::c_uint,
	 arg: core::ffi::c_ulong)
LSM_HOOK!(void, LSM_RET_VOID, file_set_fowner, file *file);
LSM_HOOK!(int, 0, file_send_sigiotask, task_struct *tsk,
	 fown_struct *fown, int sig)
LSM_HOOK!(int, 0, file_receive, file *file);
LSM_HOOK!(int, 0, file_open, file *file);
LSM_HOOK!(int, 0, file_post_open, file *file, int mask);
LSM_HOOK!(int, 0, file_truncate, file *file);
LSM_HOOK!(int, 0, task_alloc, task_struct *task,
	 clone_flags: u64)
LSM_HOOK!(void, LSM_RET_VOID, task_free, task_struct *task);
LSM_HOOK!(int, 0, cred_alloc_blank, cred *cred, gfp_t gfp);
LSM_HOOK!(void, LSM_RET_VOID, cred_free, cred *cred);
LSM_HOOK!(int, 0, cred_prepare, cred *new, const struct cred *old,
	 gfp_t gfp)
LSM_HOOK!(void, LSM_RET_VOID, cred_transfer, cred *new,
	 const struct cred *old)
LSM_HOOK!(void, LSM_RET_VOID, cred_getsecid, const struct cred *c, u32 *secid);
LSM_HOOK!(void, LSM_RET_VOID, cred_getlsmprop, const struct cred *c,
	 lsm_prop *prop)
LSM_HOOK!(int, 0, kernel_act_as, cred *new, secid: u32);
LSM_HOOK!(int, 0, kernel_create_files_as, cred *new, inode *inode);
LSM_HOOK!(int, 0, kernel_module_request, char *kmod_name);
LSM_HOOK!(int, 0, kernel_load_data, kernel_load_data_id id, contents: bool);
LSM_HOOK!(int, 0, kernel_post_load_data, char *buf, loff_t size,
	 kernel_load_data_id id, char *description)
LSM_HOOK!(int, 0, kernel_read_file, file *file,
	 kernel_read_file_id id, contents: bool)
LSM_HOOK!(int, 0, kernel_post_read_file, file *file, char *buf,
	 loff_t size, kernel_read_file_id id)
LSM_HOOK!(int, 0, task_fix_setuid, cred *new, const struct cred *old,
	 int flags)
LSM_HOOK!(int, 0, task_fix_setgid, cred *new, const struct cred * old,
	 int flags)
LSM_HOOK!(int, 0, task_fix_setgroups, cred *new, const struct cred * old);
LSM_HOOK!(int, 0, task_setpgid, task_struct *p, pid_t pgid);
LSM_HOOK!(int, 0, task_getpgid, task_struct *p);
LSM_HOOK!(int, 0, task_getsid, task_struct *p);
LSM_HOOK!(void, LSM_RET_VOID, current_getlsmprop_subj, lsm_prop *prop);
LSM_HOOK!(void, LSM_RET_VOID, task_getlsmprop_obj,
	 task_struct *p, lsm_prop *prop)
LSM_HOOK!(int, 0, task_setnice, task_struct *p, int nice);
LSM_HOOK!(int, 0, task_setioprio, task_struct *p, int ioprio);
LSM_HOOK!(int, 0, task_getioprio, task_struct *p);
LSM_HOOK!(int, 0, task_prlimit, const struct cred *cred,
	 const struct cred *tcred, flags: core::ffi::c_uint)
LSM_HOOK!(int, 0, task_setrlimit, task_struct *p, resource: core::ffi::c_uint,
	 rlimit *new_rlim)
LSM_HOOK!(int, 0, task_setscheduler, task_struct *p);
LSM_HOOK!(int, 0, task_getscheduler, task_struct *p);
LSM_HOOK!(int, 0, task_movememory, task_struct *p);
LSM_HOOK!(int, 0, task_kill, task_struct *p, kernel_siginfo *info,
	 int sig, const struct cred *cred)
LSM_HOOK!(int, -ENOSYS, task_prctl, int option, arg2: core::ffi::c_ulong,
	 arg3: core::ffi::c_ulong, arg4: core::ffi::c_ulong, arg5: core::ffi::c_ulong)
LSM_HOOK!(void, LSM_RET_VOID, task_to_inode, task_struct *p,
	 inode *inode)
LSM_HOOK!(int, 0, userns_create, const struct cred *cred);
LSM_HOOK!(int, 0, ipc_permission, kern_ipc_perm *ipcp, short flag);
LSM_HOOK!(void, LSM_RET_VOID, ipc_getlsmprop, kern_ipc_perm *ipcp,
	 lsm_prop *prop)
LSM_HOOK!(int, 0, msg_msg_alloc_security, msg_msg *msg);
LSM_HOOK!(void, LSM_RET_VOID, msg_msg_free_security, msg_msg *msg);
LSM_HOOK!(int, 0, msg_queue_alloc_security, kern_ipc_perm *perm);
LSM_HOOK!(void, LSM_RET_VOID, msg_queue_free_security,
	 kern_ipc_perm *perm)
LSM_HOOK!(int, 0, msg_queue_associate, kern_ipc_perm *perm, int msqflg);
LSM_HOOK!(int, 0, msg_queue_msgctl, kern_ipc_perm *perm, int cmd);
LSM_HOOK!(int, 0, msg_queue_msgsnd, kern_ipc_perm *perm,
	 msg_msg *msg, int msqflg)
LSM_HOOK!(int, 0, msg_queue_msgrcv, kern_ipc_perm *perm,
	 msg_msg *msg, task_struct *target, long type, int mode)
LSM_HOOK!(int, 0, shm_alloc_security, kern_ipc_perm *perm);
LSM_HOOK!(void, LSM_RET_VOID, shm_free_security, kern_ipc_perm *perm);
LSM_HOOK!(int, 0, shm_associate, kern_ipc_perm *perm, int shmflg);
LSM_HOOK!(int, 0, shm_shmctl, kern_ipc_perm *perm, int cmd);
LSM_HOOK!(int, 0, shm_shmat, kern_ipc_perm *perm, char __user *shmaddr,
	 int shmflg)
LSM_HOOK!(int, 0, sem_alloc_security, kern_ipc_perm *perm);
LSM_HOOK!(void, LSM_RET_VOID, sem_free_security, kern_ipc_perm *perm);
LSM_HOOK!(int, 0, sem_associate, kern_ipc_perm *perm, int semflg);
LSM_HOOK!(int, 0, sem_semctl, kern_ipc_perm *perm, int cmd);
LSM_HOOK!(int, 0, sem_semop, kern_ipc_perm *perm, sembuf *sops,
	 unsigned nsops, int alter)
LSM_HOOK!(int, 0, netlink_send, sock *sk, sk_buff *skb);
LSM_HOOK!(void, LSM_RET_VOID, d_instantiate, dentry *dentry,
	 inode *inode)
LSM_HOOK!(int, -EOPNOTSUPP, getselfattr, attr: core::ffi::c_uint,
	 lsm_ctx __user *ctx, u32 *size, flags: u32)
LSM_HOOK!(int, -EOPNOTSUPP, setselfattr, attr: core::ffi::c_uint,
	 lsm_ctx *ctx, size: u32, flags: u32)
LSM_HOOK!(int, -EINVAL, getprocattr, task_struct *p, const char *name,
	 char **value)
LSM_HOOK!(int, -EINVAL, setprocattr, const char *name, void *value, size_t size);
LSM_HOOK!(int, 0, ismaclabel, const char *name);
LSM_HOOK!(int, -EOPNOTSUPP, secid_to_secctx, secid: u32, lsm_context *cp);
LSM_HOOK!(int, -EOPNOTSUPP, lsmprop_to_secctx, lsm_prop *prop,
	 lsm_context *cp)
LSM_HOOK!(int, 0, secctx_to_secid, const char *secdata, seclen: u32, u32 *secid);
LSM_HOOK!(void, LSM_RET_VOID, release_secctx, lsm_context *cp);
LSM_HOOK!(void, LSM_RET_VOID, inode_invalidate_secctx, inode *inode);
LSM_HOOK!(int, 0, inode_notifysecctx, inode *inode, void *ctx, ctxlen: u32);
LSM_HOOK!(int, 0, inode_setsecctx, dentry *dentry, void *ctx, ctxlen: u32);
LSM_HOOK!(int, -EOPNOTSUPP, inode_getsecctx, inode *inode,
	 lsm_context *cp)

// #if defined(CONFIG_SECURITY) && defined(CONFIG_WATCH_QUEUE)
LSM_HOOK!(int, 0, post_notification, const struct cred *w_cred,
	 const struct cred *cred, watch_notification *n)
// #endif /* CONFIG_SECURITY && CONFIG_WATCH_QUEUE */

// #if defined(CONFIG_SECURITY) && defined(CONFIG_KEY_NOTIFICATIONS)
LSM_HOOK!(int, 0, watch_key, key *key);
// #endif /* CONFIG_SECURITY && CONFIG_KEY_NOTIFICATIONS */

// #if defined(CONFIG_SECURITY_NETWORK) && defined(CONFIG_SECURITY_PATH)
LSM_HOOK!(int, 0, unix_find, const struct path *path, sock *other,
	 int flags)
// #endif /* CONFIG_SECURITY_NETWORK && CONFIG_SECURITY_PATH */

// #ifdef CONFIG_SECURITY_NETWORK
LSM_HOOK!(int, 0, unix_stream_connect, sock *sock, sock *other,
	 sock *newsk)
LSM_HOOK!(int, 0, unix_may_send, socket *sock, socket *other);
LSM_HOOK!(int, 0, socket_create, int family, int type, int protocol, int kern);
LSM_HOOK!(int, 0, socket_post_create, socket *sock, int family, int type,
	 int protocol, int kern)
LSM_HOOK!(int, 0, socket_socketpair, socket *socka, socket *sockb);
LSM_HOOK!(int, 0, socket_bind, socket *sock, sockaddr *address,
	 int addrlen)
LSM_HOOK!(int, 0, socket_connect, socket *sock, sockaddr *address,
	 int addrlen)
LSM_HOOK!(int, 0, socket_listen, socket *sock, int backlog);
LSM_HOOK!(int, 0, socket_accept, socket *sock, socket *newsock);
LSM_HOOK!(int, 0, socket_sendmsg, socket *sock, msghdr *msg,
	 int size)
LSM_HOOK!(int, 0, socket_recvmsg, socket *sock, msghdr *msg,
	 int size, int flags)
LSM_HOOK!(int, 0, socket_getsockname, socket *sock);
LSM_HOOK!(int, 0, socket_getpeername, socket *sock);
LSM_HOOK!(int, 0, socket_getsockopt, socket *sock, int level, int optname);
LSM_HOOK!(int, 0, socket_setsockopt, socket *sock, int level, int optname);
LSM_HOOK!(int, 0, socket_shutdown, socket *sock, int how);
LSM_HOOK!(int, 0, socket_sock_rcv_skb, sock *sk, sk_buff *skb);
LSM_HOOK!(int, -ENOPROTOOPT, socket_getpeersec_stream, socket *sock,
	 sockptr_t optval, sockptr_t optlen, len: core::ffi::c_uint)
LSM_HOOK!(int, -ENOPROTOOPT, socket_getpeersec_dgram, socket *sock,
	 sk_buff *skb, u32 *secid)
LSM_HOOK!(int, 0, sk_alloc_security, sock *sk, int family, gfp_t priority);
LSM_HOOK!(void, LSM_RET_VOID, sk_free_security, sock *sk);
LSM_HOOK!(void, LSM_RET_VOID, sk_clone_security, const struct sock *sk,
	 sock *newsk)
LSM_HOOK!(void, LSM_RET_VOID, sk_getsecid, const struct sock *sk, u32 *secid);
LSM_HOOK!(void, LSM_RET_VOID, sock_graft, sock *sk, socket *parent);
LSM_HOOK!(int, 0, inet_conn_request, const struct sock *sk, sk_buff *skb,
	 request_sock *req)
LSM_HOOK!(void, LSM_RET_VOID, inet_csk_clone, sock *newsk,
	 const struct request_sock *req)
LSM_HOOK!(void, LSM_RET_VOID, inet_conn_established, sock *sk,
	 sk_buff *skb)
LSM_HOOK!(int, 0, secmark_relabel_packet, secid: u32);
LSM_HOOK!(void, LSM_RET_VOID, secmark_refcount_inc, void);
LSM_HOOK!(void, LSM_RET_VOID, secmark_refcount_dec, void);
LSM_HOOK!(void, LSM_RET_VOID, req_classify_flow, const struct request_sock *req,
	 flowi_common *flic)
LSM_HOOK!(int, 0, tun_dev_alloc_security, void *security);
LSM_HOOK!(int, 0, tun_dev_create, void);
LSM_HOOK!(int, 0, tun_dev_attach_queue, void *security);
LSM_HOOK!(int, 0, tun_dev_attach, sock *sk, void *security);
LSM_HOOK!(int, 0, tun_dev_open, void *security);
LSM_HOOK!(int, 0, sctp_assoc_request, sctp_association *asoc,
	 sk_buff *skb)
LSM_HOOK!(int, 0, sctp_bind_connect, sock *sk, int optname,
	 sockaddr *address, int addrlen)
LSM_HOOK!(void, LSM_RET_VOID, sctp_sk_clone, sctp_association *asoc,
	 sock *sk, sock *newsk)
LSM_HOOK!(int, 0, sctp_assoc_established, sctp_association *asoc,
	 sk_buff *skb)
LSM_HOOK!(int, 0, mptcp_add_subflow, sock *sk, sock *ssk);
// #endif /* CONFIG_SECURITY_NETWORK */

// #ifdef CONFIG_SECURITY_INFINIBAND
LSM_HOOK!(int, 0, ib_pkey_access, void *sec, subnet_prefix: u64, pkey: u16);
LSM_HOOK!(int, 0, ib_endport_manage_subnet, void *sec, const char *dev_name,
	 port_num: u8)
LSM_HOOK!(int, 0, ib_alloc_security, void *sec);
// #endif /* CONFIG_SECURITY_INFINIBAND */

// #ifdef CONFIG_SECURITY_NETWORK_XFRM
LSM_HOOK!(int, 0, xfrm_policy_alloc_security, xfrm_sec_ctx **ctxp,
	 xfrm_user_sec_ctx *sec_ctx, gfp_t gfp)
LSM_HOOK!(int, 0, xfrm_policy_clone_security, xfrm_sec_ctx *old_ctx,
	 xfrm_sec_ctx **new_ctx)
LSM_HOOK!(void, LSM_RET_VOID, xfrm_policy_free_security,
	 xfrm_sec_ctx *ctx)
LSM_HOOK!(int, 0, xfrm_policy_delete_security, xfrm_sec_ctx *ctx);
LSM_HOOK!(int, 0, xfrm_state_alloc, xfrm_state *x,
	 xfrm_user_sec_ctx *sec_ctx)
LSM_HOOK!(int, 0, xfrm_state_alloc_acquire, xfrm_state *x,
	 xfrm_sec_ctx *polsec, secid: u32)
LSM_HOOK!(void, LSM_RET_VOID, xfrm_state_free_security, xfrm_state *x);
LSM_HOOK!(int, 0, xfrm_state_delete_security, xfrm_state *x);
LSM_HOOK!(int, 0, xfrm_policy_lookup, xfrm_sec_ctx *ctx, fl_secid: u32);
LSM_HOOK!(int, 1, xfrm_state_pol_flow_match, xfrm_state *x,
	 xfrm_policy *xp, const struct flowi_common *flic)
LSM_HOOK!(int, 0, xfrm_decode_session, sk_buff *skb, u32 *secid,
	 int ckall)
// #endif /* CONFIG_SECURITY_NETWORK_XFRM */

/* key management security hooks */
// #ifdef CONFIG_KEYS
LSM_HOOK!(int, 0, key_alloc, key *key, const struct cred *cred,
	 flags: core::ffi::c_ulong)
LSM_HOOK!(int, 0, key_permission, key_ref_t key_ref, const struct cred *cred,
	 key_need_perm need_perm)
LSM_HOOK!(int, 0, key_getsecurity, key *key, char **buffer);
LSM_HOOK!(void, LSM_RET_VOID, key_post_create_or_update, key *keyring,
	 key *key, const void *payload, size_t payload_len,
	 flags: core::ffi::c_ulong, create: bool)
// #endif /* CONFIG_KEYS */

// #ifdef CONFIG_AUDIT
LSM_HOOK!(int, 0, audit_rule_init, field: u32, op: u32, char *rulestr,
	 void **lsmrule, gfp_t gfp)
LSM_HOOK!(int, 0, audit_rule_known, audit_krule *krule);
LSM_HOOK!(int, 0, audit_rule_match, lsm_prop *prop, field: u32, op: u32,
	 void *lsmrule)
LSM_HOOK!(void, LSM_RET_VOID, audit_rule_free, void *lsmrule);
// #endif /* CONFIG_AUDIT */

// #ifdef CONFIG_BPF_SYSCALL
LSM_HOOK!(int, 0, bpf, int cmd, bpf_attr *attr, size: core::ffi::c_uint, kernel: bool);
LSM_HOOK!(int, 0, bpf_map, bpf_map *map, fmode_t fmode);
LSM_HOOK!(int, 0, bpf_prog, bpf_prog *prog);
LSM_HOOK!(int, 0, bpf_map_create, bpf_map *map, bpf_attr *attr,
	 bpf_token *token, kernel: bool)
LSM_HOOK!(void, LSM_RET_VOID, bpf_map_free, bpf_map *map);
LSM_HOOK!(int, 0, bpf_prog_load, bpf_prog *prog, bpf_attr *attr,
	 bpf_token *token, kernel: bool)
LSM_HOOK!(void, LSM_RET_VOID, bpf_prog_free, bpf_prog *prog);
LSM_HOOK!(int, 0, bpf_token_create, bpf_token *token, bpf_attr *attr,
	 const struct path *path)
LSM_HOOK!(void, LSM_RET_VOID, bpf_token_free, bpf_token *token);
LSM_HOOK!(int, 0, bpf_token_cmd, const struct bpf_token *token, bpf_cmd cmd);
LSM_HOOK!(int, 0, bpf_token_capable, const struct bpf_token *token, int cap);
// #endif /* CONFIG_BPF_SYSCALL */

LSM_HOOK!(int, 0, locked_down, lockdown_reason what);

// #ifdef CONFIG_PERF_EVENTS
LSM_HOOK!(int, 0, perf_event_open, int type);
LSM_HOOK!(int, 0, perf_event_alloc, perf_event *event);
LSM_HOOK!(int, 0, perf_event_read, perf_event *event);
LSM_HOOK!(int, 0, perf_event_write, perf_event *event);
// #endif /* CONFIG_PERF_EVENTS */

// #ifdef CONFIG_IO_URING
LSM_HOOK!(int, 0, uring_override_creds, const struct cred *new);
LSM_HOOK!(int, 0, uring_sqpoll, void);
LSM_HOOK!(int, 0, uring_cmd, io_uring_cmd *ioucmd);
LSM_HOOK!(int, 0, uring_allowed, void);
// #endif /* CONFIG_IO_URING */

LSM_HOOK!(void, LSM_RET_VOID, initramfs_populated, void);

LSM_HOOK!(int, 0, bdev_alloc_security, block_device *bdev);
LSM_HOOK!(void, LSM_RET_VOID, bdev_free_security, block_device *bdev);
LSM_HOOK!(int, 0, bdev_setintegrity, block_device *bdev,
	 lsm_integrity_type type, const void *value, size_t size)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
