/* SPDX-License-Identifier: GPL-2.0-only */
/* One native header operation or field access per leaf. No transition fallback.
 * NSPROXY_LEAF emits the same declaration and definition signatures.
 */
NSPROXY_LEAF(__init struct kmem_cache *, cache_create, (void), { return KMEM_CACHE(nsproxy, SLAB_PANIC | SLAB_ACCOUNT); })
NSPROXY_LEAF(struct nsproxy *, alloc, (struct kmem_cache *cache), { return kmem_cache_alloc(cache, GFP_KERNEL); })
NSPROXY_LEAF(void, free, (struct kmem_cache *cache, struct nsproxy *ns), { kmem_cache_free(cache, ns); })
NSPROXY_LEAF(void, refcount_set, (struct nsproxy *ns, int value), { refcount_set(&ns->count, value); })
NSPROXY_LEAF(struct nsproxy *, error, (int error), { return ERR_PTR(error); })

#define NSPROXY_ERROR_LEAVES(name, type) \
	NSPROXY_LEAF(bool, is_err_##name, (const type *ptr), { return IS_ERR(ptr); }) \
	NSPROXY_LEAF(int, error_##name, (const type *ptr), { return PTR_ERR(ptr); })
NSPROXY_ERROR_LEAVES(nsproxy, struct nsproxy)
NSPROXY_ERROR_LEAVES(mnt, struct mnt_namespace)
NSPROXY_ERROR_LEAVES(uts, struct uts_namespace)
NSPROXY_ERROR_LEAVES(ipc, struct ipc_namespace)
NSPROXY_ERROR_LEAVES(pid_ns, struct pid_namespace)
NSPROXY_ERROR_LEAVES(cgroup, struct cgroup_namespace)
NSPROXY_ERROR_LEAVES(net, struct net)
NSPROXY_ERROR_LEAVES(time, struct time_namespace)
/* pidfd error is classified, never propagated by the original syscall. */
NSPROXY_LEAF(bool, is_err_pid, (const struct pid *ptr), { return IS_ERR(ptr); })
#undef NSPROXY_ERROR_LEAVES

NSPROXY_LEAF(struct task_struct *, current, (void), { return current; })
NSPROXY_LEAF(struct nsproxy *, task_nsproxy, (struct task_struct *task), { return task->nsproxy; })
NSPROXY_LEAF(void, store_task_nsproxy, (struct task_struct *task, struct nsproxy *ns), { task->nsproxy = ns; })
NSPROXY_LEAF(struct fs_struct *, task_fs, (struct task_struct *task), { return task->fs; })
NSPROXY_LEAF(struct user_namespace *, cred_user_ns, (const struct cred *cred), { return cred->user_ns; })
NSPROXY_LEAF(struct user_namespace *, task_real_cred_user_ns, (struct task_struct *task), { return task->real_cred->user_ns; })
NSPROXY_LEAF(struct user_namespace *, task_cred_user_ns, (struct task_struct *task), { return task_cred_xxx(task, user_ns); })
/* Caller owns the existing RCU read-side critical section. */
NSPROXY_LEAF(struct user_namespace *, task_user_ns_rcu, (struct task_struct *task), { return __task_cred(task)->user_ns; })
NSPROXY_LEAF(struct user_namespace *, current_user_ns, (void), { return current_user_ns(); })
NSPROXY_LEAF(const struct cred *, current_cred, (void), { return current_cred(); })
NSPROXY_LEAF(struct cred *, prepare_creds, (void), { return prepare_creds(); })
NSPROXY_LEAF(int, commit_creds, (struct cred *cred), { return commit_creds(cred); })
NSPROXY_LEAF(void, put_cred, (const struct cred *cred), { put_cred(cred); })
NSPROXY_LEAF(struct cred *, nsset_cred, (struct nsset *set), { return nsset_cred(set); })
NSPROXY_LEAF(void, user_active_get, (struct user_namespace *ns), { ns_ref_active_get(ns); })
NSPROXY_LEAF(void, user_active_put, (struct user_namespace *ns), { ns_ref_active_put(ns); })

NSPROXY_LEAF(bool, likely, (bool value), { return likely(value); })
NSPROXY_LEAF(bool, unlikely, (bool value), { return unlikely(value); })
NSPROXY_LEAF(bool, ns_capable, (struct user_namespace *ns, int cap), { return ns_capable(ns, cap); })
NSPROXY_LEAF(bool, ptrace_may_access, (struct task_struct *task, unsigned int mode), { return ptrace_may_access(task, mode); })
NSPROXY_LEAF(void, might_sleep, (void), { might_sleep(); })
NSPROXY_LEAF(void, task_lock, (struct task_struct *task), { task_lock(task); })
NSPROXY_LEAF(void, task_unlock, (struct task_struct *task), { task_unlock(task); })
NSPROXY_LEAF(void, rcu_read_lock, (void), { rcu_read_lock(); })
NSPROXY_LEAF(void, rcu_read_unlock, (void), { rcu_read_unlock(); })
NSPROXY_LEAF(struct task_struct *, pid_task, (struct pid *pid), { return pid_task(pid, PIDTYPE_PID); })
NSPROXY_LEAF(struct pid_namespace *, task_active_pid_ns, (struct task_struct *task), { return task_active_pid_ns(task); })
NSPROXY_LEAF(struct pid_namespace *, get_pid_ns, (struct pid_namespace *ns), { return get_pid_ns(ns); })
NSPROXY_LEAF(struct user_namespace *, get_user_ns, (struct user_namespace *ns), { return get_user_ns(ns); })
NSPROXY_LEAF(void, put_user_ns, (struct user_namespace *ns), { put_user_ns(ns); })
NSPROXY_LEAF(void, get_nsproxy, (struct nsproxy *ns), { get_nsproxy(ns); })
NSPROXY_LEAF(void, put_nsproxy, (struct nsproxy *ns), { put_nsproxy(ns); })
NSPROXY_LEAF(void, active_get, (struct nsproxy *ns), { nsproxy_ns_active_get(ns); })
NSPROXY_LEAF(void, active_put, (struct nsproxy *ns), { nsproxy_ns_active_put(ns); })

NSPROXY_LEAF(struct mnt_namespace *, copy_mnt_ns, (u64 flags, struct mnt_namespace *old, struct user_namespace *user, struct fs_struct *fs), { return copy_mnt_ns(flags, old, user, fs); })
NSPROXY_LEAF(struct uts_namespace *, copy_utsname, (u64 flags, struct user_namespace *user, struct uts_namespace *old), { return copy_utsname(flags, user, old); })
NSPROXY_LEAF(struct ipc_namespace *, copy_ipcs, (u64 flags, struct user_namespace *user, struct ipc_namespace *old), { return copy_ipcs(flags, user, old); })
NSPROXY_LEAF(struct pid_namespace *, copy_pid_ns, (u64 flags, struct user_namespace *user, struct pid_namespace *old), { return copy_pid_ns(flags, user, old); })
NSPROXY_LEAF(struct cgroup_namespace *, copy_cgroup_ns, (u64 flags, struct user_namespace *user, struct cgroup_namespace *old), { return copy_cgroup_ns(flags, user, old); })
NSPROXY_LEAF(struct net *, copy_net_ns, (u64 flags, struct user_namespace *user, struct net *old), { return copy_net_ns(flags, user, old); })
NSPROXY_LEAF(struct time_namespace *, copy_time_ns, (u64 flags, struct user_namespace *user, struct time_namespace *old), { return copy_time_ns(flags, user, old); })
NSPROXY_LEAF(struct time_namespace *, get_time_ns, (struct time_namespace *ns), { return get_time_ns(ns); })
NSPROXY_LEAF(void, put_mnt_ns, (struct mnt_namespace *ns), { put_mnt_ns(ns); })
NSPROXY_LEAF(void, put_uts_ns, (struct uts_namespace *ns), { put_uts_ns(ns); })
NSPROXY_LEAF(void, put_ipc_ns, (struct ipc_namespace *ns), { put_ipc_ns(ns); })
NSPROXY_LEAF(void, put_pid_ns, (struct pid_namespace *ns), { put_pid_ns(ns); })
NSPROXY_LEAF(void, put_time_ns, (struct time_namespace *ns), { put_time_ns(ns); })
NSPROXY_LEAF(void, put_cgroup_ns, (struct cgroup_namespace *ns), { put_cgroup_ns(ns); })
NSPROXY_LEAF(void, put_net, (struct net *ns), { put_net(ns); })
NSPROXY_LEAF(void, timens_on_fork, (struct nsproxy *ns, struct task_struct *task), { timens_on_fork(ns, task); })

NSPROXY_LEAF(struct fs_struct *, copy_fs_struct, (struct fs_struct *fs), { return copy_fs_struct(fs); })
NSPROXY_LEAF(void, free_fs_struct, (struct fs_struct *fs), { free_fs_struct(fs); })
NSPROXY_LEAF(void, set_fs_root, (struct fs_struct *target, struct fs_struct *source), { set_fs_root(target, &source->root); })
NSPROXY_LEAF(void, set_fs_pwd, (struct fs_struct *target, struct fs_struct *source), { set_fs_pwd(target, &source->pwd); })
#ifdef CONFIG_IPC_NS
NSPROXY_LEAF(void, exit_sem, (struct task_struct *task), { exit_sem(task); })
#endif
#ifdef CONFIG_TIME_NS
NSPROXY_LEAF(void, timens_commit, (struct task_struct *task, struct time_namespace *ns), { timens_commit(task, ns); })
#endif

/* The randomized operations struct and callback call ABI remain native. */
NSPROXY_LEAF(int, install, (struct nsset *set, struct ns_common *ns), { return ns->ops->install(set, ns); })
NSPROXY_LEAF(u32, ns_type, (const struct ns_common *ns), { return ns->ns_type; })
NSPROXY_LEAF(struct ns_common *, from_mnt_ns, (struct mnt_namespace *ns), { return from_mnt_ns(ns); })
#ifdef CONFIG_USER_NS
NSPROXY_LEAF(struct ns_common *, from_user_ns, (struct user_namespace *ns), { return &ns->ns; })
#endif
#ifdef CONFIG_UTS_NS
NSPROXY_LEAF(struct ns_common *, from_uts_ns, (struct uts_namespace *ns), { return &ns->ns; })
#endif
#ifdef CONFIG_IPC_NS
NSPROXY_LEAF(struct ns_common *, from_ipc_ns, (struct ipc_namespace *ns), { return &ns->ns; })
#endif
#ifdef CONFIG_PID_NS
NSPROXY_LEAF(struct ns_common *, from_pid_ns, (struct pid_namespace *ns), { return &ns->ns; })
#endif
#ifdef CONFIG_CGROUPS
NSPROXY_LEAF(struct ns_common *, from_cgroup_ns, (struct cgroup_namespace *ns), { return &ns->ns; })
#endif
#ifdef CONFIG_NET_NS
NSPROXY_LEAF(struct ns_common *, from_net_ns, (struct net *ns), { return &ns->ns; })
#endif
#ifdef CONFIG_TIME_NS
NSPROXY_LEAF(struct ns_common *, from_time_ns, (struct time_namespace *ns), { return &ns->ns; })
#endif

/* The fd word and its conditional fput remain wholly native. */
NSPROXY_LEAF(struct fd, fdget, (int fd), { return fdget(fd); })
NSPROXY_LEAF(bool, fd_empty, (const struct fd *fd), { return fd_empty(*fd); })
NSPROXY_LEAF(struct file *, fd_file, (const struct fd *fd), { return fd_file(*fd); })
NSPROXY_LEAF(void, fdput, (const struct fd *fd), { fdput(*fd); })
NSPROXY_LEAF(bool, proc_ns_file, (struct file *file), { return proc_ns_file(file); })
NSPROXY_LEAF(struct ns_common *, proc_ns, (struct file *file), { return get_proc_ns(file_inode(file)); })
NSPROXY_LEAF(struct pid *, pidfd_pid, (const struct file *file), { return pidfd_pid(file); })
NSPROXY_LEAF(void, perf_event_namespaces, (struct task_struct *task), { perf_event_namespaces(task); })
