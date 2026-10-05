/* SPDX-License-Identifier: GPL-2.0-only */
/* Compiler, architecture, locking and existing-subsystem primitive boundary.
 * Compiler/locking operations and existing subsystem APIs are separated in
 * DEPENDENCIES.md. All fork-owned policy and the selected header algorithms
 * live in Rust; remaining subsystem providers are explicitly inventoried.
 * Runtime storage is Rust-owned; this header retains only typed native access
 * operations and compiler/registration metadata.
 */
#ifndef RUST_FORK_TASK_PRIMITIVES_H
#define RUST_FORK_TASK_PRIMITIVES_H
#ifndef ARCH_MIN_TASKALIGN
#define ARCH_MIN_TASKALIGN 0
#endif
#ifndef ARCH_MIN_MMSTRUCT_ALIGN
#define ARCH_MIN_MMSTRUCT_ALIGN 0
#endif
enum { RUST_FORK_UCOUNT_COUNTS = UCOUNT_COUNTS };
static const unsigned long RUST_FORK_MMF_DUMP_FILTER_MASK = MMF_DUMP_FILTER_MASK;
static const unsigned long RUST_FORK_VM_INIT_DEF_MASK = VM_INIT_DEF_MASK;
static const unsigned long RUST_FORK_CLONE_NS_ALL = CLONE_NS_ALL;
static const unsigned long RUST_FORK_THREAD_SIZE = THREAD_SIZE;
static const unsigned long RUST_FORK_PAGE_SIZE = PAGE_SIZE;
static const unsigned long RUST_FORK_THREAD_ALIGN = THREAD_ALIGN;
static const unsigned int RUST_FORK_THREAD_SIZE_ORDER = THREAD_SIZE_ORDER;
static const unsigned int RUST_FORK_PAGE_SHIFT = PAGE_SHIFT;
static const gfp_t RUST_FORK_GFP_KERNEL = GFP_KERNEL;
static const gfp_t RUST_FORK_THREADINFO_GFP = THREADINFO_GFP;
static const slab_flags_t RUST_FORK_SLAB_PANIC = SLAB_PANIC;
static const slab_flags_t RUST_FORK_SLAB_ACCOUNT = SLAB_ACCOUNT;
static const slab_flags_t RUST_FORK_SLAB_HWCACHE_ALIGN = SLAB_HWCACHE_ALIGN;
static const slab_flags_t RUST_FORK_SLAB_TYPESAFE_BY_RCU = SLAB_TYPESAFE_BY_RCU;
enum { RUST_FORK_RLIM_INFINITY = RLIM_INFINITY };
enum { RUST_FORK_L1_CACHE_BYTES = L1_CACHE_BYTES,
       RUST_FORK_ARCH_MIN_TASKALIGN = ARCH_MIN_TASKALIGN,
       RUST_FORK_ARCH_MIN_MMSTRUCT_ALIGN = ARCH_MIN_MMSTRUCT_ALIGN,
       RUST_FORK_MM_SAVED_AUXV_OFFSET = offsetof(struct mm_struct, saved_auxv),
       RUST_FORK_MM_SAVED_AUXV_SIZE = sizeof_field(struct mm_struct, saved_auxv) };
#ifdef __ARCH_BROKEN_SYS_CLONE3
enum { RUST_FORK_ARCH_BROKEN_SYS_CLONE3 = 1 };
#else
enum { RUST_FORK_ARCH_BROKEN_SYS_CLONE3 = 0 };
#endif
DECLARE_PER_CPU(unsigned long, process_counts);
/* tasklist_lock storage is Rust-owned; sched/task.h supplies its declaration. */
#ifdef CONFIG_VMAP_STACK
static const gfp_t RUST_FORK_GFP_VMAP_STACK = GFP_KERNEL | __GFP_ZERO | __GFP_SKIP_KASAN;
#define RUST_FORK_NR_CACHED_STACKS 2
DECLARE_PER_CPU(struct vm_struct *, rust_fork_cached_stacks[RUST_FORK_NR_CACHED_STACKS]);
struct rust_fork_vm_stack { struct rcu_head rcu; struct vm_struct *stack_vm_area; };
struct vm_struct *rust_fork_cached_stack_xchg(unsigned int i);
struct vm_struct *rust_fork_cached_stack_xchg(unsigned int i)
{ return this_cpu_xchg(rust_fork_cached_stacks[i], NULL); }
bool rust_fork_cached_stack_cmpxchg(unsigned int i, struct vm_struct *vm);
bool rust_fork_cached_stack_cmpxchg(unsigned int i, struct vm_struct *vm)
{ struct vm_struct *expected = NULL; return this_cpu_try_cmpxchg(rust_fork_cached_stacks[i], &expected, vm); }
struct vm_struct **rust_fork_cached_stack_slot(unsigned int cpu, unsigned int i);
struct vm_struct **rust_fork_cached_stack_slot(unsigned int cpu, unsigned int i)
{ return &per_cpu_ptr(rust_fork_cached_stacks, cpu)[i]; }
int rust_fork_cpuhp_stack_cache(int (*teardown)(unsigned int));
int rust_fork_cpuhp_stack_cache(int (*teardown)(unsigned int))
{ return cpuhp_setup_state(CPUHP_BP_PREPARE_DYN, "fork:vm_stack_cache", NULL, teardown); }
#endif
#define RFV(name, args, op) void rust_fork_##name args; void rust_fork_##name args { op; }
#define RFR(type, name, args, op) type rust_fork_##name args; type rust_fork_##name args { return (op); }
/* Original fork.c:903 aggregate assignment only, not a fork algorithm fallback.
 * dst is unpublished, but src is a live task: this native primitive supplies
 * no atomic snapshot, exclusion, or C/Rust/BPF memory-model qualification. */
RFV(task_struct_copy, (struct task_struct *dst, const struct task_struct *src), *dst = *src)
RFR(struct task_struct *, current, (void), current)
RFR(bool, is_err, (const void *p), IS_ERR(p))
RFR(unsigned int, nr_cpu_ids, (void), nr_cpu_ids)
RFR(int, next_possible_cpu, (int cpu), cpumask_next(cpu, cpu_possible_mask))
RFR(unsigned long, process_count, (int cpu), per_cpu(process_counts, cpu))
RFV(process_count_inc, (void), __this_cpu_inc(process_counts))
RFV(tasklist_read_lock, (void), read_lock(&tasklist_lock))
RFV(tasklist_read_unlock, (void), read_unlock(&tasklist_lock))
RFV(tasklist_write_lock_irq, (void), write_lock_irq(&tasklist_lock))
RFV(tasklist_write_unlock_irq, (void), write_unlock_irq(&tasklist_lock))
#ifdef CONFIG_PROVE_RCU
RFR(int, tasklist_held, (void), lockdep_is_held(&tasklist_lock))
#endif
RFV(preempt_disable, (void), preempt_disable())
RFV(preempt_enable, (void), preempt_enable())
RFR(int, numa_node_id, (void), numa_node_id())
RFR(bool, node_has_memory, (int nid), node_state(nid, N_MEMORY))
RFR(int, page_to_nid, (struct page *page), page_to_nid(page))
RFV(bug_on, (bool on), BUG_ON(on))
RFR(bool, warn_on, (bool on), WARN_ON(on))
RFR(bool, warn_seccomp_once, (bool on), WARN_ON_ONCE(on))
RFR(bool, warn_stack_once, (bool on), WARN_ON_ONCE(on))
RFR(int, memcg_charge_page, (struct page *p, gfp_t g, int order), memcg_kmem_charge_page(p, g, order))
RFV(memcg_uncharge_page, (struct page *p, int order), memcg_kmem_uncharge_page(p, order))
RFR(bool, kasan_hw_tags_enabled, (void), kasan_hw_tags_enabled())
RFV(kasan_unpoison_range, (void *p, size_t size), kasan_unpoison_range(p, size))
RFR(void *, kasan_reset_tag, (const void *p), kasan_reset_tag(p))
RFV(clear_pages, (void *p, unsigned long n), clear_pages(p, n))
RFR(const void *, return_address, (void), __builtin_return_address(0))
RFR(struct page *, virt_to_page, (void *p), virt_to_page(p))
RFR(struct page *, alloc_pages_node, (int node, gfp_t gfp, unsigned int order), alloc_pages_node(node, gfp, order))
RFR(void *, page_address, (struct page *p), page_address(p))
RFR(struct vm_struct *, task_stack_vm_area, (const struct task_struct *p), task_stack_vm_area(p))
RFR(void *, task_stack_page, (const struct task_struct *p), task_stack_page(p))
RFV(mod_lruvec_page_state, (struct page *p, enum node_stat_item i, long n), mod_lruvec_page_state(p, i, n))
RFV(mod_lruvec_kmem_state, (void *p, enum node_stat_item i, long n), mod_lruvec_kmem_state(p, i, n))
RFR(bool, refcount_dec_and_test, (refcount_t *r), refcount_dec_and_test(r))
RFR(unsigned int, refcount_read, (const refcount_t *r), refcount_read(r))
RFV(refcount_set, (refcount_t *r, int n), refcount_set(r, n))
RFV(refcount_inc, (refcount_t *r), refcount_inc(r))
RFV(atomic_inc, (atomic_t *r), atomic_inc(r))
RFR(int, atomic_read, (const atomic_t *r), atomic_read(r))
RFV(scs_release, (struct task_struct *p), scs_release(p))
RFR(int, scs_prepare, (struct task_struct *p, int n), scs_prepare(p, n))
RFV(scs_init, (void), scs_init())
RFV(smp_task_ipi_mask_free, (struct task_struct *p), smp_task_ipi_mask_free(p))
RFR(int, smp_task_ipi_mask_alloc, (struct task_struct *p), smp_task_ipi_mask_alloc(p))
RFV(rt_mutex_debug_task_free, (struct task_struct *p), rt_mutex_debug_task_free(p))
RFV(bpf_task_storage_free, (struct task_struct *p), bpf_task_storage_free(p))
RFV(put_task_exec_state, (struct task_exec_state *p), put_task_exec_state(p))
RFV(arch_thread_struct_whitelist, (unsigned long *o, unsigned long *s), arch_thread_struct_whitelist(o, s))
RFR(size_t, arch_task_struct_size, (void), arch_task_struct_size)
RFV(set_userns_rlimit_max, (struct user_namespace *ns, enum rlimit_type t, unsigned long max), set_userns_rlimit_max(ns, t, max))
RFV(lockdep_init_task, (struct task_struct *p), lockdep_init_task(p))
void __init rust_fork_uprobes_init(void);
void __init rust_fork_uprobes_init(void) { uprobes_init(); }
RFR(unsigned long *, end_of_stack, (struct task_struct *p), end_of_stack(p))
RFR(int, tsk_fork_get_node, (struct task_struct *p), tsk_fork_get_node(p))
RFV(setup_thread_stack, (struct task_struct *p, struct task_struct *orig), setup_thread_stack(p, orig))
RFV(clear_user_return_notifier, (struct task_struct *p), clear_user_return_notifier(p))
RFV(clear_tsk_need_resched, (struct task_struct *p), clear_tsk_need_resched(p))
RFV(clear_syscall_user_dispatch, (struct task_struct *p), clear_syscall_work_syscall_user_dispatch(p))
#ifdef CONFIG_STACKPROTECTOR
RFR(unsigned long, get_random_canary, (void), get_random_canary())
#endif
RFV(kcov_task_init, (struct task_struct *p), kcov_task_init(p))
RFV(kmsan_task_create, (struct task_struct *p), kmsan_task_create(p))
RFV(kmap_local_fork, (struct task_struct *p), kmap_local_fork(p))
RFR(pid_t, task_pid_vnr, (struct task_struct *p), task_pid_vnr(p))
RFV(pi_lock_init, (struct task_struct *p), raw_spin_lock_init(&p->pi_lock))
RFV(blocked_lock_init, (struct task_struct *p), raw_spin_lock_init(&p->blocked_lock))
RFV(alloc_lock_init, (struct task_struct *p), spin_lock_init(&p->alloc_lock))
RFV(spin_lock_irq, (spinlock_t *lock), spin_lock_irq(lock))
RFV(spin_unlock_irq, (spinlock_t *lock), spin_unlock_irq(lock))
RFV(spin_lock, (spinlock_t *lock), spin_lock(lock))
RFV(spin_unlock, (spinlock_t *lock), spin_unlock(lock))
RFR(bool, pid_has_task, (struct pid *p, enum pid_type t), pid_has_task(p, t))
RFV(oom_adj_lock, (void), mutex_lock(&oom_adj_mutex))
RFV(oom_adj_unlock, (void), mutex_unlock(&oom_adj_mutex))
RFV(mm_set_multiprocess, (struct mm_struct *mm), mm_flags_set(MMF_MULTIPROCESS, mm))
RFR(struct user_namespace *, current_user_ns, (void), current_user_ns())
RFV(sigemptyset, (sigset_t *s), sigemptyset(s))
RFV(siginitsetinv, (sigset_t *s, unsigned long mask), siginitsetinv(s, mask))
RFV(hlist_add_head, (struct hlist_node *n, struct hlist_head *h), hlist_add_head(n, h))
RFV(hlist_del_init, (struct hlist_node *n), hlist_del_init(n))
RFR(bool, task_sigpending, (struct task_struct *p), task_sigpending(p))
/* Copy ABI/provider only. Padding decisions and extent are Rust-owned. */
RFR(ssize_t, sized_strscpy, (char *d, const char *s, size_t n), sized_strscpy(d, s, n))
RFV(ftrace_graph_init_task, (struct task_struct *p), ftrace_graph_init_task(p))
RFV(assert_irqs_enabled, (void), lockdep_assert_irqs_enabled())
#ifdef CONFIG_PROVE_LOCKING
RFR(bool, softirqs_enabled, (struct task_struct *p), p->softirqs_enabled)
RFV(debug_locks_warn, (bool on), DEBUG_LOCKS_WARN_ON(on))
#endif
RFR(struct ucounts *, task_ucounts, (struct task_struct *p), task_ucounts(p))
RFR(unsigned long, rlimit, (unsigned int r), rlimit(r))
RFR(bool, is_rlimit_overlimit, (struct ucounts *u, enum rlimit_type t, unsigned long r), is_rlimit_overlimit(u, t, r))
RFR(int, read_nr_threads, (const int *n), data_race(*n))
RFV(delayacct_tsk_init, (struct task_struct *p), delayacct_tsk_init(p))
RFV(delayacct_tsk_free, (struct task_struct *p), delayacct_tsk_free(p))
RFV(init_sigpending, (struct sigpending *s), init_sigpending(s))
RFV(prev_cputime_init, (struct prev_cputime *p), prev_cputime_init(p))
#ifdef CONFIG_VIRT_CPU_ACCOUNTING_GEN
RFV(vtime_seqcount_init, (struct task_struct *p), seqcount_init(&p->vtime.seqcount))
#endif
RFV(task_io_accounting_init, (struct task_io_accounting *a), task_io_accounting_init(a))
RFV(acct_clear_integrals, (struct task_struct *p), acct_clear_integrals(p))
RFV(tick_dep_init_task, (struct task_struct *p), tick_dep_init_task(p))
RFV(audit_set_context, (struct task_struct *p, struct audit_context *a), audit_set_context(p, a))
RFV(cgroup_fork, (struct task_struct *p), cgroup_fork(p))
#ifdef CONFIG_CPUSETS
RFV(mems_allowed_seq_init, (struct task_struct *p), seqcount_spinlock_init(&p->mems_allowed_seq, &p->alloc_lock))
#endif
RFR(unsigned long, this_ip, (void), (unsigned long)__builtin_return_address(0))
RFV(unwind_task_init, (struct task_struct *p), unwind_task_init(p))
RFR(int, perf_event_init_task, (struct task_struct *p, u64 f), perf_event_init_task(p, f))
RFR(int, audit_alloc, (struct task_struct *p), audit_alloc(p))
RFV(shm_init_task, (struct task_struct *p), shm_init_task(p))
RFR(int, copy_semundo, (u64 f, struct task_struct *p), copy_semundo(f, p))
RFR(int, copy_io, (u64 f, struct task_struct *p), copy_io(f, p))
RFV(stackleak_task_init, (struct task_struct *p), stackleak_task_init(p))
RFR(int, put_user_int, (int v, int __user *p), put_user(v, p))
RFV(futex_init_task, (struct task_struct *p), futex_init_task(p))
RFV(sas_ss_reset, (struct task_struct *p), sas_ss_reset(p))
RFV(clear_syscall_trace, (struct task_struct *p), clear_task_syscall_work(p, SYSCALL_TRACE))
void rust_fork_clear_syscall_emu(struct task_struct *p);
void rust_fork_clear_syscall_emu(struct task_struct *p)
{
#if defined(CONFIG_GENERIC_ENTRY) || defined(TIF_SYSCALL_EMU)
    clear_task_syscall_work(p, SYSCALL_EMU);
#endif
}
RFV(clear_tsk_latency_tracing, (struct task_struct *p), clear_tsk_latency_tracing(p))
RFR(pid_t, pid_nr, (struct pid *p), pid_nr(p))
RFV(clear_posix_cputimers_work, (struct task_struct *p), clear_posix_cputimers_work(p))
RFR(int, cgroup_can_fork, (struct task_struct *p, struct kernel_clone_args *a), cgroup_can_fork(p, a))
RFV(cgroup_cancel_fork, (struct task_struct *p, struct kernel_clone_args *a), cgroup_cancel_fork(p, a))
RFR(int, futex_hash_allocate_default, (void), futex_hash_allocate_default())
RFR(u64, ktime_get_ns, (void), ktime_get_ns())
RFR(u64, ktime_get_boottime_ns, (void), ktime_get_boottime_ns())
RFV(klp_copy_process, (struct task_struct *p), klp_copy_process(p))
RFV(sched_core_fork, (struct task_struct *p), sched_core_fork(p))
RFV(sched_core_free, (struct task_struct *p), sched_core_free(p))
RFR(struct pid_namespace *, ns_of_pid, (struct pid *p), ns_of_pid(p))
RFR(bool, fatal_signal_pending, (struct task_struct *p), fatal_signal_pending(p))
RFV(task_set_no_new_privs, (struct task_struct *p), task_set_no_new_privs(p))
RFR(bool, thread_group_leader, (struct task_struct *p), thread_group_leader(p))
RFR(struct pid *, task_pgrp, (struct task_struct *p), task_pgrp(p))
RFR(struct pid *, task_session, (struct task_struct *p), task_session(p))
RFR(bool, is_child_reaper, (struct pid *p), is_child_reaper(p))
void rust_fork_set_child_reaper(struct pid_namespace *ns, struct task_struct *p);
void rust_fork_set_child_reaper(struct pid_namespace *ns, struct task_struct *p)
{ ASSERT_EXCLUSIVE_WRITER(ns->child_reaper); WRITE_ONCE(ns->child_reaper, p); }
RFR(struct tty_struct *, tty_kref_get, (struct tty_struct *t), tty_kref_get(t))
RFV(list_add_tail, (struct list_head *n, struct list_head *h), list_add_tail(n, h))
RFV(list_add_tail_rcu, (struct list_head *n, struct list_head *h), list_add_tail_rcu(n, h))
RFV(syscall_tracepoint_update, (struct task_struct *p), syscall_tracepoint_update(p))
RFV(proc_fork_connector, (struct task_struct *p), proc_fork_connector(p))
RFV(cgroup_post_fork, (struct task_struct *p, struct kernel_clone_args *a), cgroup_post_fork(p, a))
RFV(perf_event_fork, (struct task_struct *p), perf_event_fork(p))
RFV(trace_task_newtask, (struct task_struct *p, u64 f), trace_task_newtask(p, f))
RFV(uprobe_copy_process, (struct task_struct *p, u64 f), uprobe_copy_process(p, f))
RFV(user_events_fork, (struct task_struct *p, u64 f), user_events_fork(p, f))
RFV(exit_thread, (struct task_struct *p), exit_thread(p))
RFV(exit_io_context, (struct task_struct *p), exit_io_context(p))
RFV(exit_nsproxy_namespaces, (struct task_struct *p), exit_nsproxy_namespaces(p))
RFV(exit_sem, (struct task_struct *p), exit_sem(p))
RFV(audit_free, (struct task_struct *p), audit_free(p))
RFV(perf_event_free_task, (struct task_struct *p), perf_event_free_task(p))
RFV(lockdep_free_task, (struct task_struct *p), lockdep_free_task(p))
#ifdef CONFIG_NUMA
RFV(mpol_put, (struct mempolicy *p), mpol_put(p))
#endif
RFV(io_uring_free, (struct task_struct *p), io_uring_free(p))
RFR(int, cpu_to_node, (int cpu), cpu_to_node(cpu))
RFR(bool, valid_signal, (unsigned long sig), valid_signal(sig))
RFR(bool, ptrace_event_enabled, (struct task_struct *p, int event), ptrace_event_enabled(p, event))
RFV(add_latent_entropy, (void), add_latent_entropy())
RFV(trace_sched_process_fork, (struct task_struct *p, struct task_struct *c), trace_sched_process_fork(p, c))
RFV(init_completion, (struct completion *p), init_completion(p))
RFV(get_task_struct, (struct task_struct *p), get_task_struct(p))
RFV(task_lock, (struct task_struct *p), task_lock(p))
RFV(task_unlock, (struct task_struct *p), task_unlock(p))
RFV(lru_gen_add_mm, (struct mm_struct *mm), lru_gen_add_mm(mm))
RFR(unsigned long, copy_from_user, (void *d, const void __user *s, unsigned long n), copy_from_user(d, s, n))
RFR(bool, access_ok, (const void __user *p, size_t size), access_ok(p, size))
RFV(sighand_siglock_init, (struct sighand_struct *s), spin_lock_init(&s->siglock))
RFV(signalfd_wqh_init, (struct sighand_struct *s), init_waitqueue_head(&s->signalfd_wqh))
RFR(size_t, cpumask_size, (void), cpumask_size())
RFR(size_t, mm_cid_size, (void), mm_cid_size())
RFR(struct kmem_cache *, kmem_cache_create,
    (const char *name, size_t size, size_t align, slab_flags_t flags, void (*ctor)(void *)),
    kmem_cache_create(name, size, align, flags, ctor))
RFR(bool, thread_group_empty, (struct task_struct *p), thread_group_empty(p))
RFR(int, unshare_userns, (unsigned long f, struct cred **c), unshare_userns(f, c))
RFV(exit_shm, (struct task_struct *p), exit_shm(p))
RFV(perf_event_namespaces, (struct task_struct *p), perf_event_namespaces(p))
RFV(put_nsproxy, (struct nsproxy *p), put_nsproxy(p))
RFV(put_cred, (const struct cred *p), put_cred(p))
RFV(vfs_warn_once, (bool on), VFS_WARN_ON_ONCE(on))
RFR(struct kmem_cache *, kmem_cache_create_usercopy,
    (const char *name, size_t size, size_t align, slab_flags_t flags, size_t offset, size_t usize_, void (*ctor)(void *)),
    kmem_cache_create_usercopy(name, size, align, flags, offset, usize_, ctor))
RFR(void *, kmem_cache_alloc_node, (struct kmem_cache *c, gfp_t g, int n), kmem_cache_alloc_node(c, g, n))
RFV(ftrace_graph_exit_task, (struct task_struct *p), ftrace_graph_exit_task(p))
RFV(user_disable_single_step, (struct task_struct *p), user_disable_single_step(p))
RFV(ptrace_event_pid, (int e, struct pid *p), ptrace_event_pid(e, p))
RFR(void *, vmalloc_node, (unsigned long size, unsigned long align, gfp_t g, int n, const void *caller), __vmalloc_node(size, align, g, n, caller))
RFR(int, io_uring_fork, (struct task_struct *p), io_uring_fork(p))
#ifdef CONFIG_NUMA
RFR(struct mempolicy *, mpol_dup, (struct mempolicy *p), mpol_dup(p))
#endif
RFR(int, security_task_alloc, (struct task_struct *p, unsigned long f), security_task_alloc(p, f))
RFV(security_task_free, (struct task_struct *p), security_task_free(p))
#undef RFV
#undef RFR
#endif
