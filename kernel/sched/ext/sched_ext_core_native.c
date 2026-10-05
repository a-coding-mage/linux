// SPDX-License-Identifier: GPL-2.0
/* One-definition owner of ext.c's original top-level native state and macro
 * leaves. This is an explicit, unqualified C runtime boundary. The original
 * ext.c cannot coexist with these definitions. Native include order and local
 * BTF visibility must be preserved within one build_policy translation unit.
 */
#error "SOURCE ONLY HOLD: sched_ext core native ownership and qualification incomplete"

#include "sched_ext_core_bindings.h"

/* Storage copied from the exact oracle; semantic mutators have family owners. */
DEFINE_RAW_SPINLOCK(scx_sched_lock);

/*
 * NOTE: sched_ext is in the process of growing multiple scheduler support and
 * scx_root usage is in a transitional state. Naked dereferences are safe if the
 * caller is one of the tasks attached to SCX and explicit RCU dereference is
 * necessary otherwise. Naked scx_root dereferences trigger sparse warnings but
 * are used as temporary markers to indicate that the dereferences need to be
 * updated to point to the associated scheduler instances rather than scx_root.
 */
struct scx_sched __rcu *scx_root;

/*
 * All scheds, writers must hold both scx_enable_mutex and scx_sched_lock.
 * Readers can hold either or rcu_read_lock().
 */
LIST_HEAD(scx_sched_all);

#ifdef CONFIG_EXT_SUB_SCHED
const struct rhashtable_params scx_sched_hash_params = {
	.key_len		= sizeof_field(struct scx_sched, ops.sub_cgroup_id),
	.key_offset		= offsetof(struct scx_sched, ops.sub_cgroup_id),
	.head_offset		= offsetof(struct scx_sched, hash_node),
	.insecure_elasticity	= true,	/* inserted under scx_sched_lock */
};

struct rhashtable scx_sched_hash;
#endif

/* see SCX_OPS_TID_TO_TASK */
static const struct rhashtable_params scx_tid_hash_params = {
	.key_len		= sizeof_field(struct sched_ext_entity, tid),
	.key_offset		= offsetof(struct sched_ext_entity, tid),
	.head_offset		= offsetof(struct sched_ext_entity, tid_hash_node),
	.insecure_elasticity	= true,	/* inserted/removed under scx_tasks_lock */
};
static struct rhashtable scx_tid_hash;

/*
 * During exit, a task may schedule after losing its PIDs. When disabling the
 * BPF scheduler, we need to be able to iterate tasks in every state to
 * guarantee system safety. Maintain a dedicated task list which contains every
 * task between its fork and eventual free.
 */
static DEFINE_RAW_SPINLOCK(scx_tasks_lock);
static LIST_HEAD(scx_tasks);

/* ops enable/disable */
DEFINE_MUTEX(scx_enable_mutex);
DEFINE_STATIC_KEY_FALSE(__scx_enabled);
DEFINE_PERCPU_RWSEM(scx_fork_rwsem);
static atomic_t scx_enable_state_var = ATOMIC_INIT(SCX_DISABLED);
static DEFINE_RAW_SPINLOCK(scx_bypass_lock);
static bool scx_init_task_enabled;
static bool scx_switching_all;
DEFINE_STATIC_KEY_FALSE(__scx_switched_all);
static DEFINE_STATIC_KEY_FALSE(__scx_tid_to_task_enabled);

/*
 * Gates cgroup ops delivery. Set at the end of the cgroup init phase of root
 * enable and cleared before root disable starts tearing down tasks, both under
 * scx_cgroup_lock(). Holding cgroup_lock() and seeing %true guarantees no race
 * against root tearing down tasks.
 */
bool scx_cgroup_enabled;
static atomic_long_t scx_nr_rejected = ATOMIC_LONG_INIT(0);
static atomic_long_t scx_hotplug_seq = ATOMIC_LONG_INIT(0);

/* Global cursor for the per-CPU tid allocator. Starts at 1; tid 0 is reserved. */
static atomic64_t scx_tid_cursor = ATOMIC64_INIT(1);
/* Cursor for unique scx_sched instance ids. id 0 is reserved. */
static atomic64_t scx_sched_id_cursor = ATOMIC64_INIT(0);

#ifdef CONFIG_EXT_SUB_SCHED
/*
 * The sub sched being enabled. Used by scx_disable_and_exit_task() to exit
 * tasks for the sub-sched being enabled. Use a global variable instead of a
 * per-task field as all enables are serialized.
 */
struct scx_sched *scx_enabling_sub_sched;
#else
#define scx_enabling_sub_sched	(struct scx_sched *)NULL
#endif	/* CONFIG_EXT_SUB_SCHED */

/*
 * A monotonically increasing sequence number that is incremented every time a
 * scheduler is enabled. This can be used to check if any custom sched_ext
 * scheduler has ever been used in the system.
 */
static atomic_long_t scx_enable_seq = ATOMIC_LONG_INIT(0);

/*
 * Watchdog interval. All scx_sched's share a single watchdog timer and the
 * interval is half of the shortest sch->watchdog_timeout.
 */
static unsigned long scx_watchdog_interval;

/*
 * The last time the delayed work was run. This delayed work relies on
 * ksoftirqd being able to run to service timer interrupts, so it's possible
 * that this work itself could get wedged. To account for this, we check that
 * it's not stalled in the timer tick, and trigger an error if it is.
 */
static unsigned long scx_watchdog_timestamp = INITIAL_JIFFIES;

static struct delayed_work scx_watchdog_work;

/*
 * For %SCX_KICK_WAIT: Each CPU has a pointer to an array of kick_sync sequence
 * numbers. The arrays are allocated with kvzalloc() as size can exceed percpu
 * allocator limits on large machines. O(nr_cpu_ids^2) allocation, allocated
 * lazily when enabling and freed when disabling to avoid waste when sched_ext
 * isn't active.
 */
static DEFINE_PER_CPU(struct scx_kick_syncs __rcu *, scx_kick_syncs);

/*
 * Per-CPU buffered allocator state for p->scx.tid. Each CPU pulls a chunk of
 * SCX_TID_CHUNK ids from scx_tid_cursor and hands them out locally without
 * further synchronization. See scx_alloc_tid().
 */
static DEFINE_PER_CPU(struct scx_tid_alloc, scx_tid_alloc);

/*
 * Direct dispatch marker.
 *
 * Non-NULL values are used for direct dispatch from enqueue path. A valid
 * pointer points to the task currently being enqueued. An ERR_PTR value is used
 * to indicate that direct dispatch has already happened.
 */
static DEFINE_PER_CPU(struct task_struct *, direct_dispatch_task);

static const struct rhashtable_params dsq_hash_params = {
	.key_len		= sizeof_field(struct scx_dispatch_q, id),
	.key_offset		= offsetof(struct scx_dispatch_q, id),
	.head_offset		= offsetof(struct scx_dispatch_q, hash_node),
};

static LLIST_HEAD(dsqs_to_free);

/* ops debug dump */
static DEFINE_RAW_SPINLOCK(scx_dump_lock);

static struct scx_dump_data scx_dump_data = {
	.cpu			= -1,
};

/* /sys/kernel/sched_ext interface */
static struct kset *scx_kset;

/*
 * Parameters that can be adjusted through /sys/module/sched_ext/parameters.
 * There usually is no reason to modify these as normal scheduler operation
 * shouldn't be affected by them. The knobs are primarily for debugging.
 */
static unsigned int scx_slice_bypass_us = SCX_SLICE_BYPASS / NSEC_PER_USEC;
static unsigned int scx_bypass_lb_intv_us = SCX_BYPASS_LB_DFL_INTV_US;

/* ext.c:414,447; neither the CID nor sub lane may redefine these slots. */
DEFINE_PER_CPU(struct rq *, scx_locked_rq_state);
DEFINE_STATIC_KEY_FALSE(__scx_is_cid_type);

/* Preserve native callback signatures, parameter metadata and source values. */
static int set_slice_us(const char *val, const struct kernel_param *kp)
{
	return lupos_scx_core_set_slice_us(val, kp);
}

static const struct kernel_param_ops slice_us_param_ops = {
	.set = set_slice_us,
	.get = param_get_uint,
};

static int set_bypass_lb_intv_us(const char *val, const struct kernel_param *kp)
{
	return lupos_scx_core_set_bypass_lb_intv_us(val, kp);
}

static const struct kernel_param_ops bypass_lb_intv_us_param_ops = {
	.set = set_bypass_lb_intv_us,
	.get = param_get_uint,
};
#undef MODULE_PARAM_PREFIX
#define MODULE_PARAM_PREFIX	"sched_ext."

module_param_cb(slice_bypass_us, &slice_us_param_ops, &scx_slice_bypass_us, 0600);
MODULE_PARM_DESC(slice_bypass_us, "bypass slice in microseconds, applied on [un]load (100us to 100ms)");
module_param_cb(bypass_lb_intv_us, &bypass_lb_intv_us_param_ops, &scx_bypass_lb_intv_us, 0600);
MODULE_PARM_DESC(bypass_lb_intv_us, "bypass load balance interval in microseconds (0 (disable) to 10s)");

#undef MODULE_PARAM_PREFIX

#define CREATE_TRACE_POINTS
#include <trace/events/sched_ext.h>
__printf(5, 6) bool __scx_exit(struct scx_sched *sch,
			       enum scx_exit_kind kind, s64 exit_code,
			       s32 exit_cpu, const char *fmt, ...)
{
	va_list args;
	bool ret;

	va_start(args, fmt);
	ret = scx_vexit(sch, kind, exit_code, exit_cpu, fmt, args);
	va_end(args);

	return ret;
}

unsigned int lupos_scx_core_nr_cpu_ids(void)
{
	return nr_cpu_ids;
}

bool lupos_scx_core_cpu_possible(s32 cpu)
{
	return cpu_possible(cpu);
}

bool lupos_scx_core_likely_cpu_valid(bool valid)
{
	return likely(valid);
}

void lupos_scx_core_error_invalid_cpu(struct scx_sched *sch, s32 cpu,
                                     const char *where)
{
	scx_error(sch, "invalid CPU %d%s%s", cpu, where ? " " : "", where ?: "");
}

int lupos_scx_core_param_set_uint_minmax(const char *val,
                                        const struct kernel_param *kp,
                                        unsigned int min, unsigned int max)
{
	return param_set_uint_minmax(val, kp, min, max);
}

bool lupos_scx_core_tid_to_task_enabled(void)
{
	return static_branch_likely(&__scx_tid_to_task_enabled);
}

bool lupos_scx_core_time_after(unsigned long at, unsigned long now)
{
	return time_after(at, now);
}

unsigned int lupos_scx_core_jiffies_to_msecs(unsigned long j)
{
	return jiffies_to_msecs(j);
}

struct scx_sched *lupos_scx_core_ancestor_at(struct scx_sched *sch, s32 level)
{
	return sch->ancestors[level];
}

int lupos_scx_core_cpu_to_node(s32 cpu)
{
	return cpu_to_node(cpu);
}

struct scx_dispatch_q *lupos_scx_core_dsq_lookup(struct rhashtable *ht,
                                               const u64 *id)
{
	return rhashtable_lookup(ht, id, dsq_hash_params);
}

const struct sched_class *lupos_scx_core_stop_class(void)
{
	return &stop_sched_class;
}

const struct sched_class *lupos_scx_core_ext_class(void)
{
	return &ext_sched_class;
}

const struct sched_class *lupos_scx_core_setscheduler_class(int policy, int prio)
{
	return __setscheduler_class(policy, prio);
}

struct scx_sched *lupos_scx_core_parent(struct scx_sched *sch)
{
	return scx_parent(sch);
}

bool lupos_scx_core_bypassing(struct scx_sched *sch, s32 cpu)
{
	return scx_bypassing(sch, cpu);
}

struct scx_dispatch_q *lupos_scx_core_bypass_dsq(struct scx_sched *sch, s32 cpu)
{
	return scx_bypass_dsq(sch, cpu);
}

void lupos_scx_core_assert_rq_open(struct rq *rq)
{
	lockdep_assert_rq_held(rq);
}

void lupos_scx_core_assert_rq_lock_drop(struct rq *rq)
{
	lockdep_assert_rq_held(rq);
}

bool lupos_scx_core_class_above(const struct sched_class *a,
                               const struct sched_class *b)
{
	return sched_class_above(a, b);
}

bool lupos_scx_core_likely_not_protected(bool value)
{
	return likely(value);
}

bool lupos_scx_core_sched_core_enabled(struct rq *rq)
{
	return sched_core_enabled(rq);
}

struct rq *lupos_scx_core_locked_rq(void)
{
	return scx_locked_rq();
}

struct task_struct *lupos_scx_core_rq_curr(struct rq *rq)
{
	return rq->curr;
}

void lupos_scx_core_update_locked_rq(struct rq *rq)
{
	update_locked_rq(rq);
}

void lupos_scx_core_raw_spin_rq_unlock(struct rq *rq)
{
	raw_spin_rq_unlock(rq);
}

void lupos_scx_core_raw_spin_rq_lock(struct rq *rq)
{
	raw_spin_rq_lock(rq);
}

bool lupos_scx_core_is_cid_type(void)
{
	return scx_is_cid_type();
}

struct scx_cmask *lupos_scx_core_this_cmask_scratch(struct scx_sched *sch)
{
	return *this_cpu_ptr(sch->set_cmask_scratch);
}

unsigned int lupos_scx_core_num_possible_cpus(void)
{
	return num_possible_cpus();
}

void lupos_scx_core_call_set_cmask(struct scx_sched *sch, struct rq *rq,
                                  struct task_struct *task,
                                  struct scx_cmask *kern_va)
{
	SCX_CALL_CID_OP_TASK(sch, set_cmask, rq, task, kern_va);
}

void lupos_scx_core_call_set_cpumask(struct scx_sched *sch, struct rq *rq,
                                    struct task_struct *task,
                                    const struct cpumask *cpumask)
{
	SCX_CALL_OP_TASK(sch, set_cpumask, rq, task, cpumask);
}

int lupos_scx_core_enable_state_read(void)
{
	return atomic_read(&scx_enable_state_var);
}

int lupos_scx_core_enable_state_xchg(int to)
{
	return atomic_xchg(&scx_enable_state_var, to);
}

bool lupos_scx_core_enable_state_try_cmpxchg(int *from, int to)
{
	return atomic_try_cmpxchg(&scx_enable_state_var, from, to);
}

void lupos_scx_core_cpu_relax(void)
{
	cpu_relax();
}

long lupos_scx_core_ops_state_read_acquire(struct task_struct *p)
{
	return atomic_long_read_acquire(&p->scx.ops_state);
}

/* Address borrowing only: lock/hash/task algorithms stay with F06/F12/F16. */
raw_spinlock_t *lupos_scx_core_tasks_lock(void)
{
	return &scx_tasks_lock;
}

struct list_head *lupos_scx_core_tasks_head(void)
{
	return &scx_tasks;
}

struct rhashtable *lupos_scx_core_tid_hash(void)
{
	return &scx_tid_hash;
}

const struct rhashtable_params *lupos_scx_core_tid_hash_params(void)
{
	return &scx_tid_hash_params;
}

struct scx_tid_alloc *lupos_scx_core_this_tid_alloc(void)
{
	return this_cpu_ptr(&scx_tid_alloc);
}

u64 lupos_scx_core_tid_cursor_fetch_add(u64 count)
{
	return atomic64_fetch_add(count, &scx_tid_cursor);
}

void lupos_scx_core_rejected_inc(void)
{
	atomic_long_inc(&scx_nr_rejected);
}

bool lupos_scx_core_init_task_enabled(void)
{
	return scx_init_task_enabled;
}

bool lupos_scx_core_switching_all_read_once(void)
{
	return READ_ONCE(scx_switching_all);
}

struct scx_sched *lupos_scx_core_enabling_sub_sched(void)
{
	return scx_enabling_sub_sched;
}

struct percpu_rw_semaphore *lupos_scx_core_fork_rwsem(void)
{
	return &scx_fork_rwsem;
}
