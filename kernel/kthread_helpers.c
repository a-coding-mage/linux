// SPDX-License-Identifier: GPL-2.0-only
/* Native primitives, ABI adapters, and explicitly retained provider gaps.
 * kthread.c is never included or called as a second provider. The H3/H4/H5
 * header policy is in kthread_header_algorithms.rs, with leaves below. */
#include "kthread_bindings.h"
#include <trace/events/sched.h>

/* Exact native static lock initializers, including RT/lockdep configurations.
 * The create and affinity queues themselves are Rust-owned static objects. */
static DEFINE_SPINLOCK(kthread_create_lock);
static DEFINE_MUTEX(kthread_affinity_lock);

struct task_struct *lupos_kthread_current(void) { return current; }
bool lupos_kthread_warn(bool value) { return WARN_ON(value); }
/* Each original WARN_ON_ONCE site keeps its own once-state, not a shared bit. */
#define LUPOS_KTHREAD_WARN_SITE(name) \
	bool lupos_kthread_warn_##name(bool value) { return WARN_ON_ONCE(value); }
#include "kthread_warn_sites.inc"
#undef LUPOS_KTHREAD_WARN_SITE
void *lupos_kthread_alloc(size_t bytes) { return kmalloc(bytes, GFP_KERNEL); }
void *lupos_kthread_zalloc(size_t bytes) { return kzalloc(bytes, GFP_KERNEL); }
char *lupos_kthread_vasprintf(const char *format, void *args)
{ return kvasprintf(GFP_KERNEL, format, *(va_list *)args); }
ssize_t lupos_kthread_strscpy(char *dst, const char *src, size_t len)
{ return sized_strscpy(dst, src, len); }
void lupos_kthread_init_completion(struct completion *done) { init_completion(done); }
struct completion *lupos_kthread_xchg_done(struct completion **done) { return xchg(done, NULL); }
bool lupos_kthread_test_bit(unsigned int bit, const unsigned long *flags) { return test_bit(bit, flags); }
void lupos_kthread_set_bit(unsigned int bit, unsigned long *flags) { set_bit(bit, flags); }
void lupos_kthread_clear_bit(unsigned int bit, unsigned long *flags) { clear_bit(bit, flags); }
void lupos_kthread_create_lock(void) { spin_lock(&kthread_create_lock); }
void lupos_kthread_create_unlock(void) { spin_unlock(&kthread_create_lock); }
void lupos_kthread_affinity_lock(void) { mutex_lock(&kthread_affinity_lock); }
void lupos_kthread_affinity_unlock(void) { mutex_unlock(&kthread_affinity_lock); }
bool lupos_kthread_list_empty(const struct list_head *head) { return list_empty(head); }
void lupos_kthread_list_add(struct list_head *node, struct list_head *head) { list_add(node, head); }
void lupos_kthread_list_add_tail(struct list_head *node, struct list_head *head) { list_add_tail(node, head); }
void lupos_kthread_list_del(struct list_head *node) { list_del(node); }
void lupos_kthread_list_del_init(struct list_head *node) { list_del_init(node); }
void lupos_kthread_set_state(unsigned int state) { set_current_state(state); }
void lupos_kthread_set_state_relaxed(unsigned int state) { __set_current_state(state); }
void lupos_kthread_set_special_state(unsigned int state) { set_special_state(state); }
void lupos_kthread_preempt_disable(void) { preempt_disable(); }
void lupos_kthread_preempt_enable(void) { preempt_enable(); }
void lupos_kthread_might_sleep(void) { might_sleep(); }
bool lupos_kthread_refrigerator(bool check_stop) { return __refrigerator(check_stop); }
#ifdef CONFIG_FREEZER
bool lupos_kthread_freezer_active(void) { return static_branch_unlikely(&freezer_active); }
void lupos_kthread_debug_no_locks_held(void) { debug_check_no_locks_held(); }
#endif
void lupos_kthread_set_freezable(void) { set_freezable(); }
void lupos_kthread_cond_resched(void) { cond_resched(); }
void lupos_kthread_cgroup_ready(void) { cgroup_kthread_ready(); }
void lupos_kthread_cgroup_init(void) { cgroup_init_kthreadd(); }
void lupos_kthread_rcu_lock(void) { rcu_read_lock(); }
void lupos_kthread_rcu_unlock(void) { rcu_read_unlock(); }
const struct cpumask *lupos_kthread_housekeeping_mask(void) { return housekeeping_cpumask(HK_TYPE_DOMAIN); }
const struct cpumask *lupos_kthread_node_mask(int node) { return cpumask_of_node(node); }
const struct cpumask *lupos_kthread_cpu_mask(unsigned int cpu) { return cpumask_of(cpu); }
int lupos_kthread_cpu_to_node(int cpu) { return cpu_to_node(cpu); }
bool lupos_kthread_alloc_cpumask(struct lupos_kthread_cpumask *storage)
{ return zalloc_cpumask_var(&storage->mask, GFP_KERNEL); }
void lupos_kthread_free_cpumask(struct lupos_kthread_cpumask *storage) { free_cpumask_var(storage->mask); }
struct cpumask *lupos_kthread_cpumask_ptr(struct lupos_kthread_cpumask *storage) { return storage->mask; }
void lupos_kthread_cpumask_and(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b)
{ cpumask_and(dst, a, b); }
bool lupos_kthread_cpumask_empty(const struct cpumask *mask) { return cpumask_empty(mask); }
void lupos_kthread_cpumask_copy(struct cpumask *dst, const struct cpumask *src) { cpumask_copy(dst, src); }
void lupos_kthread_raw_lock_init(raw_spinlock_t *lock) { raw_spin_lock_init(lock); }
void lupos_kthread_lockdep_class(raw_spinlock_t *lock, struct lock_class_key *key, const char *name)
{ lockdep_set_class_and_name(lock, key, name); }
void lupos_kthread_assert_locked(raw_spinlock_t *lock) { lockdep_assert_held(lock); }
unsigned long lupos_kthread_raw_lock_irqsave(raw_spinlock_t *lock)
{
	unsigned long flags;
	raw_spin_lock_irqsave(lock, flags);
	return flags;
}
void lupos_kthread_raw_unlock_irqrestore(raw_spinlock_t *lock, unsigned long flags)
{ raw_spin_unlock_irqrestore(lock, flags); }
void lupos_kthread_raw_lock_irq(raw_spinlock_t *lock) { raw_spin_lock_irq(lock); }
void lupos_kthread_raw_unlock_irq(raw_spinlock_t *lock) { raw_spin_unlock_irq(lock); }
void lupos_kthread_notify_signal(struct task_struct *task) { set_tsk_thread_flag(task, TIF_NOTIFY_SIGNAL); }
void lupos_kthread_get_task(struct task_struct *task) { refcount_inc(&task->usage); }
bool lupos_kthread_task_usage_dec_and_test(struct task_struct *task)
{ return refcount_dec_and_test(&task->usage); }
/* The selected public callback is the existing fork Rust implementation.
 * Keep its native call_rcu function-pointer identity and deferred lifetime. */
void lupos_kthread_task_release_rcu(struct task_struct *task)
{ call_rcu(&task->rcu, __put_task_struct_rcu_cb); }
void lupos_kthread_set_comm(struct task_struct *task, const char *comm) { __set_task_comm(task, comm, false); }
void lupos_kthread_ignore_signals(struct task_struct *task) { ignore_signals(task); }
#ifdef CONFIG_CPUSETS
nodemask_t lupos_kthread_memory_nodes(void) { return node_states[N_MEMORY]; }
unsigned long lupos_kthread_irq_save(void)
{
	unsigned long flags;
	local_irq_save(flags);
	return flags;
}
void lupos_kthread_irq_restore(unsigned long flags) { local_irq_restore(flags); }
void lupos_kthread_mems_seq_begin(struct task_struct *task)
{ write_seqcount_begin(&task->mems_allowed_seq); }
void lupos_kthread_mems_seq_end(struct task_struct *task)
{ write_seqcount_end(&task->mems_allowed_seq); }
#endif
void lupos_kthread_init_worker_key(struct kthread_worker *worker) { kthread_init_worker(worker); }
unsigned long lupos_kthread_jiffies(void) { return jiffies; }
bool lupos_kthread_timer_callback_matches(struct timer_list *timer)
{ return timer->function == kthread_delayed_work_timer_fn; }
void lupos_kthread_trace_stop(struct task_struct *task) { trace_sched_kthread_stop(task); }
void lupos_kthread_trace_stop_ret(int result) { trace_sched_kthread_stop_ret(result); }
void lupos_kthread_trace_execute_start(struct kthread_work *work) { trace_sched_kthread_work_execute_start(work); }
void lupos_kthread_trace_execute_end(struct kthread_work *work, kthread_work_func_t func)
{ trace_sched_kthread_work_execute_end(work, func); }
void lupos_kthread_trace_queue_work(struct kthread_worker *worker, struct kthread_work *work)
{ trace_sched_kthread_work_queue_work(worker, work); }
void lupos_kthread_mmgrab(struct mm_struct *mm) { atomic_inc(&mm->mm_count); }
bool lupos_kthread_mm_count_dec_and_test(struct mm_struct *mm)
{ return atomic_dec_and_test(&mm->mm_count); }
void lupos_kthread_mb(void) { smp_mb(); }
void lupos_kthread_task_lock(struct task_struct *task) { task_lock(task); }
void lupos_kthread_task_unlock(struct task_struct *task) { task_unlock(task); }
void lupos_kthread_irq_disable(void) { local_irq_disable(); }
void lupos_kthread_irq_enable(void) { local_irq_enable(); }
void lupos_kthread_membarrier_update(struct mm_struct *mm) { membarrier_update_current_mm(mm); }
void lupos_kthread_switch_mm(struct mm_struct *prev, struct mm_struct *next, struct task_struct *task)
{ switch_mm_irqs_off(prev, next, task); }
void lupos_kthread_finish_arch_post_lock_switch(void)
{
#ifdef finish_arch_post_lock_switch
	finish_arch_post_lock_switch();
#endif
}
void lupos_kthread_mb_after_spinlock(void) { smp_mb__after_spinlock(); }
void lupos_kthread_enter_lazy_tlb(struct mm_struct *mm, struct task_struct *task) { enter_lazy_tlb(mm, task); }
#ifdef CONFIG_BLK_CGROUP
unsigned int lupos_kthread_css_flags(struct cgroup_subsys_state *css) { return css->flags; }
struct percpu_ref *lupos_kthread_css_refcount(struct cgroup_subsys_state *css)
{ return &css->refcnt; }
unsigned long lupos_kthread_percpu_ref_read_mode(struct percpu_ref *ref)
{ return READ_ONCE(ref->percpu_count_ptr); }
/* Rust passes the very same READ_ONCE value that it tested for mode flags.
 * These leaves must never reload ref->percpu_count_ptr. */
void lupos_kthread_percpu_ref_cpu_add(unsigned long pointer, unsigned long nr)
{
	unsigned long __percpu *count = (unsigned long __percpu *)pointer;
	this_cpu_add(*count, nr);
}
void lupos_kthread_percpu_ref_cpu_sub(unsigned long pointer, unsigned long nr)
{
	unsigned long __percpu *count = (unsigned long __percpu *)pointer;
	this_cpu_sub(*count, nr);
}
void lupos_kthread_percpu_ref_atomic_add(struct percpu_ref *ref, unsigned long nr)
{ atomic_long_add(nr, &ref->data->count); }
bool lupos_kthread_percpu_ref_atomic_sub_and_test(struct percpu_ref *ref, unsigned long nr)
{ return atomic_long_sub_and_test(nr, &ref->data->count); }
/* Native indirect-call/CFI identity only; Rust selects the zero transition. */
void lupos_kthread_percpu_ref_release(struct percpu_ref *ref) { ref->data->release(ref); }
#endif

/* Native callback identities avoid guessing LLVM CFI type encodings. */
int lupos_kthread_thread_callback(void *data) { return lupos_kthread_thread(data); }
void lupos_kthread_flush_callback(struct kthread_work *work) { lupos_kthread_flush_work_fn(work); }
void kthread_delayed_work_timer_fn(struct timer_list *timer) { lupos_kthread_delayed_work_timer(timer); }
static int lupos_kthread_online_callback(unsigned int cpu) { return lupos_kthread_online_cpu(cpu); }
int lupos_kthread_cpuhp_setup(const char *name)
{ return cpuhp_setup_state(CPUHP_AP_KTHREADS_ONLINE, name, lupos_kthread_online_callback, NULL); }
static int lupos_kthread_initcall(void) { return lupos_kthread_init(); }
early_initcall(lupos_kthread_initcall);

/* The nullable callback parameter needs a native public CFI signature. */
struct task_struct *kthread_create_on_cpu(int (*threadfn)(void *), void *data,
	unsigned int cpu, const char *format)
{ return lupos_kthread_create_on_cpu(threadfn, data, cpu, format); }

/* Rust does not define C variadic bodies. The native envelope preserves the
 * original va_list representation and lifetime; Rust owns all other steps. */
struct task_struct *kthread_create_on_node(int (*threadfn)(void *), void *data,
	int node, const char *namefmt, ...)
{
	struct task_struct *task;
	va_list args;
	va_start(args, namefmt);
	task = lupos_kthread_create_on_node_v(threadfn, data, node, namefmt, &args);
	va_end(args);
	return task;
}
struct kthread_worker *kthread_create_worker_on_node(unsigned int flags,
	int node, const char *namefmt, ...)
{
	struct kthread_worker *worker;
	va_list args;
	va_start(args, namefmt);
	worker = lupos_kthread_create_worker_on_node_v(flags, node, namefmt, &args);
	va_end(args);
	return worker;
}

/* Preserve the original export set and GPL classifications. */
EXPORT_SYMBOL(kthread_should_stop);
EXPORT_SYMBOL_GPL(kthread_should_park);
EXPORT_SYMBOL_GPL(kthread_freezable_should_stop);
EXPORT_SYMBOL_GPL(kthread_func);
EXPORT_SYMBOL_GPL(kthread_data);
EXPORT_SYMBOL_GPL(kthread_parkme);
EXPORT_SYMBOL(kthread_complete_and_exit);
EXPORT_SYMBOL(kthread_create_on_node);
EXPORT_SYMBOL(kthread_bind);
EXPORT_SYMBOL(kthread_create_on_cpu);
EXPORT_SYMBOL_GPL(kthread_unpark);
EXPORT_SYMBOL_GPL(kthread_park);
EXPORT_SYMBOL(kthread_stop);
EXPORT_SYMBOL(kthread_stop_put);
EXPORT_SYMBOL_GPL(kthread_affine_preferred);
EXPORT_SYMBOL_GPL(__kthread_init_worker);
EXPORT_SYMBOL_GPL(kthread_worker_fn);
EXPORT_SYMBOL(kthread_create_worker_on_node);
EXPORT_SYMBOL(kthread_create_worker_on_cpu);
EXPORT_SYMBOL_GPL(kthread_queue_work);
EXPORT_SYMBOL(kthread_delayed_work_timer_fn);
EXPORT_SYMBOL_GPL(kthread_queue_delayed_work);
EXPORT_SYMBOL_GPL(kthread_flush_work);
EXPORT_SYMBOL_GPL(kthread_mod_delayed_work);
EXPORT_SYMBOL_GPL(kthread_cancel_work_sync);
EXPORT_SYMBOL_GPL(kthread_cancel_delayed_work_sync);
EXPORT_SYMBOL_GPL(kthread_flush_worker);
EXPORT_SYMBOL(kthread_destroy_worker);
EXPORT_SYMBOL_GPL(kthread_use_mm);
EXPORT_SYMBOL_GPL(kthread_unuse_mm);
#ifdef CONFIG_BLK_CGROUP
EXPORT_SYMBOL(kthread_associate_blkcg);
#endif
