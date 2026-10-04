/* SPDX-License-Identifier: GPL-2.0-only */
/* ABI/compiler/locking adapters for parts/mm.rs. Include after fork's headers.
 * Each function below is an accessor, a compiler/locking primitive, or one call
 * to an existing subsystem API. The selected task/MM header decisions live in
 * fork_header_algorithms.rs, alongside the fork.c lifetime/copy algorithms.
 * Parse this header for the fork-specific generated bindings as well as include
 * it once in the helper translation unit. Types/constants come from Linux.
 */
#ifndef RUST_FORK_MM_PRIMITIVES_H
#define RUST_FORK_MM_PRIMITIVES_H

#include <linux/init_task.h>
#include <linux/idr.h>

#define RFMM_FIELD(prefix, type, member) \
	__typeof__(((struct type *)0)->member) * \
	rust_fork_mm_##prefix##_##member(struct type *p); \
	__typeof__(((struct type *)0)->member) * \
	rust_fork_mm_##prefix##_##member(struct type *p) { return &p->member; }
RFMM_FIELD(mm, mm_struct, mm_count)
RFMM_FIELD(mm, mm_struct, mm_users)
RFMM_FIELD(mm, mm_struct, mm_mt)
RFMM_FIELD(mm, mm_struct, mmap_lock)
RFMM_FIELD(mm, mm_struct, write_protect_seq)
RFMM_FIELD(mm, mm_struct, mmlist)
RFMM_FIELD(mm, mm_struct, map_count)
RFMM_FIELD(mm, mm_struct, locked_vm)
RFMM_FIELD(mm, mm_struct, pinned_vm)
RFMM_FIELD(mm, mm_struct, rss_stat)
RFMM_FIELD(mm, mm_struct, page_table_lock)
RFMM_FIELD(mm, mm_struct, arg_lock)
RFMM_FIELD(mm, mm_struct, def_flags)
RFMM_FIELD(mm, mm_struct, exe_file)
RFMM_FIELD(mm, mm_struct, async_put_work)
RFMM_FIELD(mm, mm_struct, binfmt)
RFMM_FIELD(mm, mm_struct, hiwater_rss)
RFMM_FIELD(mm, mm_struct, hiwater_vm)
RFMM_FIELD(mm, mm_struct, total_vm)
#ifdef CONFIG_MMU
RFMM_FIELD(mm, mm_struct, pgd)
#endif
#ifdef CONFIG_MM_ID
RFMM_FIELD(mm, mm_struct, mm_id)
#endif
#ifdef CONFIG_AIO
RFMM_FIELD(mm, mm_struct, ioctx_lock)
RFMM_FIELD(mm, mm_struct, ioctx_table)
#endif
#ifdef CONFIG_MEMCG
RFMM_FIELD(mm, mm_struct, owner)
#endif
#ifdef CONFIG_PER_VMA_LOCK
RFMM_FIELD(mm, mm_struct, vma_writer_wait)
#endif
#if defined(CONFIG_TRANSPARENT_HUGEPAGE) && !defined(CONFIG_SPLIT_PMD_PTLOCKS)
RFMM_FIELD(mm, mm_struct, pmd_huge_pte)
#endif
#ifdef CONFIG_UPROBES
struct xol_area **rust_fork_mm_xol_area(struct mm_struct *mm);
struct xol_area **rust_fork_mm_xol_area(struct mm_struct *mm)
{ return &mm->uprobes_state.xol_area; }
#endif
RFMM_FIELD(task, task_struct, mm)
RFMM_FIELD(task, task_struct, active_mm)
RFMM_FIELD(task, task_struct, flags)
RFMM_FIELD(task, task_struct, exit_state)
RFMM_FIELD(task, task_struct, usage)
RFMM_FIELD(task, task_struct, rcu)
RFMM_FIELD(task, task_struct, signal)
RFMM_FIELD(task, task_struct, sighand)
RFMM_FIELD(task, task_struct, vfork_done)
RFMM_FIELD(task, task_struct, clear_child_tid)
RFMM_FIELD(task, task_struct, min_flt)
RFMM_FIELD(task, task_struct, maj_flt)
RFMM_FIELD(task, task_struct, nvcsw)
RFMM_FIELD(task, task_struct, nivcsw)
RFMM_FIELD(task, task_struct, exec_state)
RFMM_FIELD(task, task_struct, nsproxy)
RFMM_FIELD(task, task_struct, fs)
RFMM_FIELD(task, task_struct, real_fs)
RFMM_FIELD(task, task_struct, files)
RFMM_FIELD(task, task_struct, thread_node)
RFMM_FIELD(task, task_struct, group_leader)
#ifdef CONFIG_DETECT_HUNG_TASK
RFMM_FIELD(task, task_struct, last_switch_count)
RFMM_FIELD(task, task_struct, last_switch_time)
#endif
#ifdef CONFIG_SECCOMP
RFMM_FIELD(task, task_struct, seccomp)
#endif
RFMM_FIELD(sig, signal_struct, oom_mm)
RFMM_FIELD(sig, signal_struct, sigcnt)
RFMM_FIELD(sig, signal_struct, exec_update_lock)
RFMM_FIELD(sig, signal_struct, rlim)
RFMM_FIELD(sig, signal_struct, posix_cputimers)
RFMM_FIELD(sig, signal_struct, nr_threads)
RFMM_FIELD(sig, signal_struct, quick_threads)
RFMM_FIELD(sig, signal_struct, live)
RFMM_FIELD(sig, signal_struct, thread_head)
RFMM_FIELD(sig, signal_struct, wait_chldexit)
RFMM_FIELD(sig, signal_struct, curr_target)
RFMM_FIELD(sig, signal_struct, shared_pending)
RFMM_FIELD(sig, signal_struct, multiprocess)
RFMM_FIELD(sig, signal_struct, stats_lock)
RFMM_FIELD(sig, signal_struct, prev_cputime)
RFMM_FIELD(sig, signal_struct, oom_score_adj)
RFMM_FIELD(sig, signal_struct, oom_score_adj_min)
RFMM_FIELD(sig, signal_struct, cred_guard_mutex)
#ifdef CONFIG_POSIX_TIMERS
RFMM_FIELD(sig, signal_struct, posix_timers)
RFMM_FIELD(sig, signal_struct, ignored_posix_timers)
RFMM_FIELD(sig, signal_struct, real_timer)
#endif
#ifdef CONFIG_CGROUPS
RFMM_FIELD(sig, signal_struct, cgroup_threadgroup_rwsem)
#endif
RFMM_FIELD(sh, sighand_struct, count)
RFMM_FIELD(sh, sighand_struct, siglock)
RFMM_FIELD(sh, sighand_struct, action)
RFMM_FIELD(fs, fs_struct, seq)
RFMM_FIELD(fs, fs_struct, in_exec)
RFMM_FIELD(fs, fs_struct, users)
RFMM_FIELD(files, files_struct, count)
RFMM_FIELD(exec, task_exec_state, count)
RFMM_FIELD(nsproxy, nsproxy, mnt_ns)
RFMM_FIELD(vma, vm_area_struct, vm_file)
RFMM_FIELD(file, file, f_path)
RFMM_FIELD(binfmt, linux_binfmt, module)
#undef RFMM_FIELD

/* Addresses only: x86 context initialization and allocation lifetime policy
 * belong to Rust. The RUST_FORK selector already requires X86_64. */
#define RFMM_MM_MEMBER(name, member) \
	__typeof__(((struct mm_struct *)0)->member) * \
	rust_fork_mm_##name(struct mm_struct *mm); \
	__typeof__(((struct mm_struct *)0)->member) * \
	rust_fork_mm_##name(struct mm_struct *mm) { return &mm->member; }
RFMM_MM_MEMBER(context_ctx_id, context.ctx_id)
RFMM_MM_MEMBER(context_tlb_gen, context.tlb_gen)
RFMM_MM_MEMBER(context_next_trim_cpumask, context.next_trim_cpumask)
#ifdef CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS
RFMM_MM_MEMBER(context_pkey_allocation_map, context.pkey_allocation_map)
RFMM_MM_MEMBER(context_execute_only_pkey, context.execute_only_pkey)
#endif
#ifdef CONFIG_ADDRESS_MASKING
RFMM_MM_MEMBER(context_untag_mask, context.untag_mask)
#endif
#ifdef CONFIG_MODIFY_LDT_SYSCALL
RFMM_MM_MEMBER(context_ldt, context.ldt)
#endif
#ifdef CONFIG_SCHED_MM_CID
RFMM_MM_MEMBER(cid_pcpu, mm_cid.pcpu)
#endif
#ifdef CONFIG_SCHED_CACHE
RFMM_MM_MEMBER(sched_pcpu, sc_stat.pcpu_sched)
#endif
#undef RFMM_MM_MEMBER

#define RFMM_VOID(name, args, call) void rust_fork_mm_##name args; void rust_fork_mm_##name args { call; }
#define RFMM_RET(type, name, args, call) type rust_fork_mm_##name args; type rust_fork_mm_##name args { return (call); }
RFMM_RET(gfp_t, gfp_kernel, (void), GFP_KERNEL)
RFMM_RET(gfp_t, gfp_kernel_account, (void), GFP_KERNEL_ACCOUNT)
RFMM_RET(void *, cache_alloc, (struct kmem_cache *c, gfp_t g), kmem_cache_alloc(c, g))
RFMM_RET(void *, cache_zalloc, (struct kmem_cache *c, gfp_t g), kmem_cache_zalloc(c, g))
RFMM_VOID(cache_free, (struct kmem_cache *c, void *p), kmem_cache_free(c, p))
RFMM_RET(bool, atomic_dec_and_test, (atomic_t *a), atomic_dec_and_test(a))
RFMM_RET(int, atomic_read, (const atomic_t *a), atomic_read(a))
RFMM_VOID(atomic_set, (atomic_t *a, int n), atomic_set(a, n))
RFMM_VOID(atomic_inc, (atomic_t *a), atomic_inc(a))
RFMM_VOID(atomic64_set, (atomic64_t *a, s64 n), atomic64_set(a, n))
RFMM_RET(bool, refcount_dec_and_test, (refcount_t *r), refcount_dec_and_test(r))
RFMM_RET(unsigned int, refcount_read, (const refcount_t *r), refcount_read(r))
RFMM_VOID(refcount_set, (refcount_t *r, int n), refcount_set(r, n))
RFMM_VOID(refcount_inc, (refcount_t *r), refcount_inc(r))
/* These are the exact native operations used by the header, with one lockdep
 * key at each original initialization site. Keep the native argument spelling
 * so the lockdep names remain &mm->context.lock / &mm->context.ldt_usr_sem. */
RFMM_VOID(init_context_lock, (struct mm_struct *mm), mutex_init(&mm->context.lock))
RFMM_RET(s64, next_context_id, (void), atomic64_inc_return(&last_mm_ctx_id))
RFMM_RET(unsigned long, context_jiffies, (void), jiffies)
RFMM_RET(unsigned long, context_hz, (void), HZ)
#ifdef CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS
RFMM_RET(bool, context_ospke_enabled, (void), cpu_feature_enabled(X86_FEATURE_OSPKE))
#endif
#ifdef CONFIG_MODIFY_LDT_SYSCALL
RFMM_VOID(init_context_ldt_sem, (struct mm_struct *mm), init_rwsem(&mm->context.ldt_usr_sem))
RFMM_VOID(destroy_context_ldt, (struct mm_struct *mm), destroy_context_ldt(mm))
#endif
/* External ASID/RCU providers. This unconditional enqueue preserves the
 * public callback's native identity; Rust selects the last-reference branch. */
RFMM_VOID(init_global_asid, (struct mm_struct *mm), mm_init_global_asid(mm))
RFMM_VOID(free_global_asid, (struct mm_struct *mm), mm_free_global_asid(mm))
RFMM_VOID(defer_task_release, (struct task_struct *t), call_rcu(&t->rcu, __put_task_struct_rcu_cb))

/* Exactly one typed allocation/tag site per original mm_alloc_* call. Do not
 * use a generic byte allocator: native sizeof/alignment, GFP and alloc_hooks
 * metadata stay together. The external mm_init_cid/mm_init_sched providers do
 * not allocate; their initialization is sequenced by Rust after success. */
#ifdef CONFIG_SCHED_MM_CID
RFMM_RET(struct mm_cid_pcpu __percpu *, alloc_cid_pcpu, (void), alloc_hooks(alloc_percpu_noprof(struct mm_cid_pcpu)))
RFMM_VOID(free_cid_pcpu, (struct mm_cid_pcpu __percpu *pcpu), free_percpu(pcpu))
RFMM_VOID(init_cid, (struct mm_struct *mm, struct task_struct *t), mm_init_cid(mm, t))
#endif
#ifdef CONFIG_SCHED_CACHE
RFMM_RET(struct sched_cache_time __percpu *, alloc_sched_pcpu, (void), alloc_hooks(alloc_percpu_noprof(struct sched_cache_time)))
RFMM_VOID(free_sched_pcpu, (struct sched_cache_time __percpu *pcpu), free_percpu(pcpu))
RFMM_VOID(init_sched, (struct mm_struct *mm, struct sched_cache_time __percpu *pcpu), mm_init_sched(mm, pcpu))
#endif
RFMM_VOID(spin_lock, (spinlock_t *l), spin_lock(l))
RFMM_VOID(spin_unlock, (spinlock_t *l), spin_unlock(l))
RFMM_VOID(spin_lock_irq, (spinlock_t *l), spin_lock_irq(l))
RFMM_VOID(spin_unlock_irq, (spinlock_t *l), spin_unlock_irq(l))
/* Distinct initialization adapters preserve distinct lockdep classes. */
RFMM_VOID(init_page_table_lock, (spinlock_t *l), spin_lock_init(l))
RFMM_VOID(init_arg_lock, (spinlock_t *l), spin_lock_init(l))
#ifdef CONFIG_AIO
RFMM_VOID(init_ioctx_lock, (spinlock_t *l), spin_lock_init(l))
#endif
RFMM_VOID(init_mmap_lock, (struct rw_semaphore *l), init_rwsem(l))
RFMM_VOID(init_exec_update_lock, (struct rw_semaphore *l), init_rwsem(l))
#ifdef CONFIG_CGROUPS
RFMM_VOID(init_cgroup_rwsem, (struct rw_semaphore *l), init_rwsem(l))
#endif
RFMM_VOID(init_write_protect_seq, (seqcount_t *s), seqcount_init(s))
RFMM_VOID(init_stats_lock, (seqlock_t *s), seqlock_init(s))
RFMM_VOID(init_cred_guard_mutex, (struct mutex *m), mutex_init(m))
RFMM_VOID(init_wait_chldexit, (wait_queue_head_t *w), init_waitqueue_head(w))
RFMM_VOID(read_seqlock_excl, (seqlock_t *s), read_seqlock_excl(s))
RFMM_VOID(read_sequnlock_excl, (seqlock_t *s), read_sequnlock_excl(s))
RFMM_VOID(task_lock, (struct task_struct *t), task_lock(t))
RFMM_VOID(task_unlock, (struct task_struct *t), task_unlock(t))
RFMM_VOID(rcu_read_lock, (void), rcu_read_lock())
RFMM_VOID(rcu_read_unlock, (void), rcu_read_unlock())
RFMM_VOID(exe_init_pointer, (struct mm_struct *m, struct file *f), RCU_INIT_POINTER(m->exe_file, f))
RFMM_VOID(exe_assign_pointer, (struct mm_struct *m, struct file *f), rcu_assign_pointer(m->exe_file, f))
RFMM_RET(struct file *, exe_dereference_raw, (struct mm_struct *m), rcu_dereference_raw(m->exe_file))
RFMM_VOID(sighand_init_pointer, (struct task_struct *t, struct sighand_struct *s), RCU_INIT_POINTER(t->sighand, s))
RFMM_RET(struct task_exec_state *, exec_dereference_protected, (struct task_struct *t), rcu_dereference_protected(t->exec_state, true))
RFMM_VOID(exec_assign_pointer, (struct task_struct *t, struct task_exec_state *e), rcu_assign_pointer(t->exec_state, e))
#ifdef CONFIG_MEMCG
RFMM_VOID(owner_write_once, (struct mm_struct *m, struct task_struct *p), WRITE_ONCE(m->owner, p))
#endif
RFMM_RET(unsigned long, cpu_limit_read_once, (struct signal_struct *s), READ_ONCE(s->rlim[RLIMIT_CPU].rlim_cur))
RFMM_VOID(init_list_head, (struct list_head *h), INIT_LIST_HEAD(h))
RFMM_VOID(init_hlist_head, (struct hlist_head *h), INIT_HLIST_HEAD(h))
RFMM_RET(bool, list_empty, (const struct list_head *h), list_empty(h))
RFMM_VOID(list_del, (struct list_head *h), list_del(h))
RFMM_VOID(init_drop_work, (struct work_struct *w, work_func_t f), INIT_WORK(w, f))
#if defined(CONFIG_MMU) || defined(CONFIG_FUTEX_PRIVATE_HASH)
RFMM_VOID(init_put_work, (struct work_struct *w, work_func_t f), INIT_WORK(w, f))
#endif
RFMM_RET(bool, schedule_work, (struct work_struct *w), schedule_work(w))
RFMM_RET(struct mm_struct *, work_to_mm, (struct work_struct *w), container_of(w, struct mm_struct, async_put_work))
RFMM_RET(struct task_struct *, rcu_to_task, (struct rcu_head *r), container_of(r, struct task_struct, rcu))
RFMM_RET(void *, err_ptr, (long e), ERR_PTR(e))
RFMM_RET(bool, is_err, (const void *p), IS_ERR(p))
RFMM_RET(long, ptr_err, (const void *p), PTR_ERR(p))
RFMM_RET(int, put_user_zero, (int __user *p), put_user(0, p))
RFMM_RET(int, kstrtoul, (const char *s, unsigned int base, unsigned long *v), kstrtoul(s, base, v))
RFMM_RET(long, do_futex, (u32 __user *p, int op, u32 val, ktime_t *time,
		 u32 __user *p2, u32 val2, u32 val3), do_futex(p, op, val, time, p2, val2, val3))
RFMM_VOID(might_sleep, (void), might_sleep())
RFMM_VOID(warn_dup_exe, (void), pr_warn_once("exe_file_deny_write_access() failed in dup_mm_exe_file\n"))
RFMM_VOID(bug_init_mm, (bool condition), BUG_ON(condition))
RFMM_VOID(warn_drop_current_mm, (bool condition), WARN_ON_ONCE(condition))
RFMM_VOID(warn_drop_active_mm, (bool condition), WARN_ON_ONCE(condition))
RFMM_VOID(warn_check_lazy, (bool condition), WARN_ON_ONCE(condition))
RFMM_VOID(warn_shoot_lazy, (bool condition), WARN_ON_ONCE(condition))
RFMM_VOID(warn_task_exit, (bool condition), WARN_ON(condition))
RFMM_VOID(warn_task_usage, (bool condition), WARN_ON(condition))
RFMM_VOID(warn_task_current, (bool condition), WARN_ON(condition))
RFMM_VOID(warn_fs, (bool condition), VFS_WARN_ON_ONCE(condition))
RFMM_VOID(bug_mm_users, (bool condition), VM_BUG_ON(condition))
#if defined(CONFIG_TRANSPARENT_HUGEPAGE) && !defined(CONFIG_SPLIT_PMD_PTLOCKS)
RFMM_VOID(bug_huge_pte, (struct mm_struct *m), VM_BUG_ON_MM(m->pmd_huge_pte, m))
#endif

/* Only boot-option registration metadata is emitted here. All runtime
 * storage, its initialization, and the parser are Rust-owned. */
/* mmlist_lock storage is defined and initialized in Rust. */
RFMM_RET(spinlock_t *, mmlist_lock, (void), &mmlist_lock)
extern int rust_fork_coredump_filter_setup(char *s);
__setup("coredump_filter=", rust_fork_coredump_filter_setup);
#ifdef CONFIG_MM_ID
extern struct ida rust_fork_mm_ida_storage;
RFMM_RET(struct ida *, ida, (void), &rust_fork_mm_ida_storage)
RFMM_RET(bool, warn_bad_id, (bool condition), WARN_ON_ONCE(condition))
#endif

/* Source-owned diagnostics, kept here only for printf/format macro support. */
void rust_fork_mm_bad_rss(struct mm_struct *m, const char *kind, long x);
void rust_fork_mm_bad_rss(struct mm_struct *m, const char *kind, long x)
{
	pr_alert("BUG: Bad rss-counter state mm:%p type:%s val:%ld Comm:%s Pid:%d\n",
		 m, kind, x, current->comm,
		 task_pid_nr(current));
}
void rust_fork_mm_bad_pgtables(unsigned long bytes);
void rust_fork_mm_bad_pgtables(unsigned long bytes)
{
	pr_alert("BUG: non-zero pgtables_bytes on freeing mm: %ld\n", bytes);
}

/* Existing subsystem APIs with inline/configuration/macro definitions. */
#ifdef CONFIG_MMU
RFMM_RET(pgd_t *, pgd_alloc, (struct mm_struct *m), pgd_alloc(m))
RFMM_VOID(pgd_free, (struct mm_struct *m, pgd_t *p), pgd_free(m, p))
#endif
RFMM_RET(long, counter_sum, (struct percpu_counter *p), percpu_counter_sum(p))
RFMM_RET(unsigned long, pgtables_bytes, (struct mm_struct *m), mm_pgtables_bytes(m))
RFMM_VOID(pgtables_bytes_init, (struct mm_struct *m), mm_pgtables_bytes_init(m))
RFMM_RET(struct cpumask *, cpumask, (struct mm_struct *m), mm_cpumask(m))
RFMM_VOID(on_each_cpu_mask, (const struct cpumask *m, smp_call_func_t f, void *p, bool wait), on_each_cpu_mask(m, f, p, wait))
RFMM_VOID(on_each_cpu, (smp_call_func_t f, void *p, bool wait), on_each_cpu(f, p, wait))
RFMM_VOID(switch_mm, (struct mm_struct *p, struct mm_struct *n, struct task_struct *t), switch_mm(p, n, t))
RFMM_VOID(notifier_destroy, (struct mm_struct *m), mmu_notifier_subscriptions_destroy(m))
RFMM_VOID(pasid_drop, (struct mm_struct *m), mm_pasid_drop(m))
RFMM_VOID(counter_destroy_many, (struct percpu_counter *p, unsigned int n), percpu_counter_destroy_many(p, n))
RFMM_VOID(taskstats_tgid_free, (struct signal_struct *s), taskstats_tgid_free(s))
RFMM_VOID(autogroup_exit, (struct signal_struct *s), sched_autogroup_exit(s))
RFMM_VOID(unwind_task_free, (struct task_struct *t), unwind_task_free(t))
RFMM_VOID(io_uring_free, (struct task_struct *t), io_uring_free(t))
RFMM_VOID(cgroup_task_free, (struct task_struct *t), cgroup_task_free(t))
RFMM_VOID(task_numa_free, (struct task_struct *t, bool final), task_numa_free(t, final))
RFMM_VOID(security_task_free, (struct task_struct *t), security_task_free(t))
RFMM_VOID(delayacct_tsk_free, (struct task_struct *t), delayacct_tsk_free(t))
RFMM_VOID(sched_core_free, (struct task_struct *t), sched_core_free(t))
RFMM_VOID(mt_init, (struct maple_tree *t, unsigned int flags), mt_init_flags(t, flags))
RFMM_VOID(mt_set_external_lock, (struct maple_tree *t, struct rw_semaphore *l), mt_set_external_lock(t, l))
RFMM_VOID(lock_seqcount_init, (struct mm_struct *m), mm_lock_seqcount_init(m))
#ifdef CONFIG_PER_VMA_LOCK
RFMM_VOID(rcuwait_init, (struct rcuwait *r), rcuwait_init(r))
#endif
RFMM_VOID(init_cpumask, (struct mm_struct *m), mm_init_cpumask(m))
RFMM_VOID(pasid_init, (struct mm_struct *m), mm_pasid_init(m))
RFMM_VOID(notifier_init, (struct mm_struct *m), mmu_notifier_subscriptions_init(m))
RFMM_VOID(init_tlb_flush_pending, (struct mm_struct *m), init_tlb_flush_pending(m))
RFMM_VOID(hugetlb_count_init, (struct mm_struct *m), hugetlb_count_init(m))
RFMM_VOID(futex_mm_init, (struct mm_struct *m), futex_mm_init(m))
RFMM_VOID(flags_clear_all, (struct mm_struct *m), mm_flags_clear_all(m))
RFMM_RET(unsigned long, flags_get, (struct mm_struct *m), __mm_flags_get_word(m))
RFMM_VOID(flags_overwrite, (struct mm_struct *m, unsigned long v), __mm_flags_overwrite_word(m, v))
RFMM_RET(unsigned long, flags_initial, (unsigned long v), mmf_init_legacy_flags(v))
RFMM_RET(int, counter_init_many, (struct percpu_counter *p, s64 n, gfp_t g, unsigned int count), percpu_counter_init_many(p, n, g, count))
RFMM_VOID(lru_gen_init, (struct mm_struct *m), lru_gen_init_mm(m))
RFMM_VOID(uprobe_clear_state, (struct mm_struct *m), uprobe_clear_state(m))
RFMM_VOID(exit_aio, (struct mm_struct *m), exit_aio(m))
RFMM_VOID(ksm_exit, (struct mm_struct *m), ksm_exit(m))
RFMM_VOID(khugepaged_exit, (struct mm_struct *m), khugepaged_exit(m))
RFMM_VOID(put_huge_zero_folio, (struct mm_struct *m), mm_put_huge_zero_folio(m))
RFMM_VOID(module_put, (struct module *m), module_put(m))
RFMM_RET(bool, try_module_get, (struct module *m), try_module_get(m))
RFMM_VOID(lru_gen_del, (struct mm_struct *m), lru_gen_del_mm(m))
RFMM_VOID(futex_hash_free, (struct mm_struct *m), futex_hash_free(m))
RFMM_VOID(mmget, (struct mm_struct *m), mmget(m))
RFMM_RET(int, exe_deny_write_access, (struct file *f), exe_file_deny_write_access(f))
RFMM_VOID(exe_allow_write_access, (struct file *f), exe_file_allow_write_access(f))
RFMM_RET(struct file *, get_file, (struct file *f), get_file(f))
RFMM_RET(struct vm_area_struct *, vma_next, (struct vma_iterator *v), vma_next(v))
RFMM_VOID(mmap_read_lock, (struct mm_struct *m), mmap_read_lock(m))
RFMM_VOID(mmap_read_unlock, (struct mm_struct *m), mmap_read_unlock(m))
RFMM_VOID(mmap_write_lock, (struct mm_struct *m), mmap_write_lock(m))
RFMM_VOID(mmap_write_unlock, (struct mm_struct *m), mmap_write_unlock(m))
RFMM_RET(bool, path_equal, (const struct path *a, const struct path *b), path_equal(a, b))
RFMM_RET(bool, perfmon_capable, (void), perfmon_capable())
RFMM_VOID(cgroup_enter_frozen, (void), cgroup_enter_frozen())
RFMM_VOID(cgroup_leave_frozen, (bool b), cgroup_leave_frozen(b))
RFMM_VOID(uprobe_free_utask, (struct task_struct *t), uprobe_free_utask(t))
RFMM_VOID(deactivate_mm, (struct task_struct *t, struct mm_struct *m), deactivate_mm(t, m))
RFMM_VOID(futex_exit_exec_release, (struct task_struct *t), futex_exit_exec_release(t))
RFMM_VOID(uprobe_start_dup_mmap, (void), uprobe_start_dup_mmap())
RFMM_VOID(uprobe_end_dup_mmap, (void), uprobe_end_dup_mmap())
RFMM_RET(unsigned long, get_rss, (struct mm_struct *m), get_mm_rss(m))
RFMM_VOID(signalfd_cleanup, (struct sighand_struct *s), signalfd_cleanup(s))
RFMM_VOID(posix_cputimers_group_init, (struct posix_cputimers *p, unsigned long n), posix_cputimers_group_init(p, n))
RFMM_VOID(init_sigpending, (struct sigpending *s), init_sigpending(s))
RFMM_VOID(prev_cputime_init, (struct prev_cputime *p), prev_cputime_init(p))
#ifdef CONFIG_POSIX_TIMERS
RFMM_VOID(setup_real_timer, (struct hrtimer *t), hrtimer_setup(t, it_real_fn, CLOCK_MONOTONIC, HRTIMER_MODE_REL))
#endif
RFMM_VOID(tty_audit_fork, (struct signal_struct *s), tty_audit_fork(s))
RFMM_VOID(autogroup_fork, (struct signal_struct *s), sched_autogroup_fork(s))
#ifdef CONFIG_SECCOMP
RFMM_VOID(assert_spin_locked, (spinlock_t *l), assert_spin_locked(l))
RFMM_VOID(get_seccomp_filter, (struct task_struct *t), get_seccomp_filter(t))
RFMM_RET(bool, task_no_new_privs, (struct task_struct *t), task_no_new_privs(t))
RFMM_VOID(task_set_no_new_privs, (struct task_struct *t), task_set_no_new_privs(t))
RFMM_VOID(set_seccomp_syscall_work, (struct task_struct *t), set_task_syscall_work(t, SECCOMP))
RFMM_RET(int, seccomp_mode, (struct task_struct *t), t->seccomp.mode)
#endif
#undef RFMM_VOID
#undef RFMM_RET
#endif
