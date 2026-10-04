// SPDX-License-Identifier: GPL-2.0-only
/* Compiler/native primitive boundary only. The algorithms in kthread.c are
 * implemented in kthread.rs, never included or called as a second provider. */
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
void lupos_kthread_strscpy(char *dst, const char *src, size_t len) { strscpy(dst, src, len); }
void lupos_kthread_strscpy_pad(char *dst, const char *src, size_t len) { strscpy_pad(dst, src, len); }
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
bool lupos_kthread_freezing(struct task_struct *task) { return freezing(task); }
bool lupos_kthread_refrigerator(bool check_stop) { return __refrigerator(check_stop); }
void lupos_kthread_try_to_freeze(void) { try_to_freeze(); }
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
void lupos_kthread_get_task(struct task_struct *task) { get_task_struct(task); }
void lupos_kthread_put_task(struct task_struct *task) { put_task_struct(task); }
void lupos_kthread_set_comm(struct task_struct *task, const char *comm) { __set_task_comm(task, comm, false); }
void lupos_kthread_ignore_signals(struct task_struct *task) { ignore_signals(task); }
void lupos_kthread_set_mems_allowed(void) { set_mems_allowed(node_states[N_MEMORY]); }
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
void lupos_kthread_mmgrab(struct mm_struct *mm) { mmgrab(mm); }
void lupos_kthread_mmdrop(struct mm_struct *mm) { mmdrop(mm); }
void lupos_kthread_mmgrab_lazy(struct mm_struct *mm) { mmgrab_lazy_tlb(mm); }
void lupos_kthread_mmdrop_lazy(struct mm_struct *mm) { mmdrop_lazy_tlb(mm); }
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
void lupos_kthread_css_put(struct cgroup_subsys_state *css) { css_put(css); }
void lupos_kthread_css_get(struct cgroup_subsys_state *css) { css_get(css); }
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
