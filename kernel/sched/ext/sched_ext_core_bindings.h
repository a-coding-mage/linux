/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_BINDINGS_H

/* Native configured headers are the sole layout, enum and macro authority.
 * All sched_ext binding modules must reuse one native type identity universe;
 * separate generated lookalike struct definitions are not an integration plan.
 */
#include <linux/bitmap.h>
#include <linux/btf_ids.h>
#include <linux/rhashtable.h>
#include <linux/sched/clock.h>
#include <linux/sched/isolation.h>
#include <linux/suspend.h>
#include <linux/sysrq.h>
#include "../pelt.h"
#include "internal.h"
#include "cid.h"
#include "arena.h"
#include "idle.h"
#include "sub.h"
#include "inlines.h"

unsigned int lupos_scx_core_nr_cpu_ids(void);
bool lupos_scx_core_cpu_possible(s32 cpu);
bool lupos_scx_core_likely_cpu_valid(bool valid);
void lupos_scx_core_error_invalid_cpu(struct scx_sched *sch, s32 cpu,
                                     const char *where);

/* Exact ext.c:165-168,177-180,203-210 native-private layout authority. */
struct scx_kick_syncs {
	struct rcu_head		rcu;
	unsigned long		syncs[];
};

struct scx_tid_alloc {
	u64	next;
	u64	end;
};

struct scx_dump_data {
	s32			cpu;
	bool			first;
	s32			cursor;
	struct seq_buf		*s;
	const char		*prefix;
	struct scx_bstr_buf	buf;
};

#define LUPOS_SCX_CORE_DSQ_LOCAL SCX_DSQ_LOCAL
#define LUPOS_SCX_CORE_DSQ_REJECT SCX_DSQ_REJECT
#define LUPOS_SCX_CORE_DSQ_RESCUE SCX_DSQ_RESCUE
#define LUPOS_SCX_CORE_RQ_IN_DISPATCH SCX_RQ_IN_DISPATCH
#define LUPOS_SCX_CORE_ENQ_PREEMPT SCX_ENQ_PREEMPT
#define LUPOS_SCX_CORE_TASK_PROTECTED SCX_TASK_PROTECTED
#define LUPOS_SCX_CORE_USEC_PER_MSEC USEC_PER_MSEC
#define LUPOS_SCX_CORE_USEC_PER_SEC USEC_PER_SEC

/* Parameter callback entry points are Rust; native callbacks retain CFI type. */
int lupos_scx_core_set_slice_us(const char *val, const struct kernel_param *kp);
int lupos_scx_core_set_bypass_lb_intv_us(const char *val,
                                       const struct kernel_param *kp);
int lupos_scx_core_param_set_uint_minmax(const char *val,
                                        const struct kernel_param *kp,
                                        unsigned int min, unsigned int max);
bool lupos_scx_core_tid_to_task_enabled(void);
bool lupos_scx_core_time_after(unsigned long at, unsigned long now);
unsigned int lupos_scx_core_jiffies_to_msecs(unsigned long j);
struct scx_sched *lupos_scx_core_ancestor_at(struct scx_sched *sch, s32 level);
int lupos_scx_core_cpu_to_node(s32 cpu);
struct scx_dispatch_q *lupos_scx_core_dsq_lookup(struct rhashtable *ht,
                                               const u64 *id);
const struct sched_class *lupos_scx_core_stop_class(void);
const struct sched_class *lupos_scx_core_ext_class(void);
const struct sched_class *lupos_scx_core_setscheduler_class(int policy, int prio);
struct scx_sched *lupos_scx_core_parent(struct scx_sched *sch);
bool lupos_scx_core_bypassing(struct scx_sched *sch, s32 cpu);
struct scx_dispatch_q *lupos_scx_core_bypass_dsq(struct scx_sched *sch, s32 cpu);
/* Preserve independent assertion-site warning state from ext.c:356 and 423. */
void lupos_scx_core_assert_rq_open(struct rq *rq);
void lupos_scx_core_assert_rq_lock_drop(struct rq *rq);
bool lupos_scx_core_class_above(const struct sched_class *a,
                               const struct sched_class *b);
bool lupos_scx_core_likely_not_protected(bool value);
bool lupos_scx_core_sched_core_enabled(struct rq *rq);
struct rq *lupos_scx_core_locked_rq(void);
/* Plain rq->curr read under the caller's original rq lock. Native sched.h
 * owns the CONFIG_SCHED_PROXY_EXEC field/anonymous-union alternatives.
 */
struct task_struct *lupos_scx_core_rq_curr(struct rq *rq);
void lupos_scx_core_update_locked_rq(struct rq *rq);
void lupos_scx_core_raw_spin_rq_unlock(struct rq *rq);
void lupos_scx_core_raw_spin_rq_lock(struct rq *rq);
bool lupos_scx_core_is_cid_type(void);
struct scx_cmask *lupos_scx_core_this_cmask_scratch(struct scx_sched *sch);
unsigned int lupos_scx_core_num_possible_cpus(void);
void lupos_scx_core_call_set_cmask(struct scx_sched *sch, struct rq *rq,
                                  struct task_struct *task,
                                  struct scx_cmask *kern_va);
void lupos_scx_core_call_set_cpumask(struct scx_sched *sch, struct rq *rq,
                                    struct task_struct *task,
                                    const struct cpumask *cpumask);
int lupos_scx_core_enable_state_read(void);
int lupos_scx_core_enable_state_xchg(int to);
bool lupos_scx_core_enable_state_try_cmpxchg(int *from, int to);
void lupos_scx_core_cpu_relax(void);
long lupos_scx_core_ops_state_read_acquire(struct task_struct *p);

/* F06 borrows these single-owner native slots under its original lock/RCU/
 * preemption protocol. Returning an address does not acquire any protection.
 */
raw_spinlock_t *lupos_scx_core_tasks_lock(void);
struct list_head *lupos_scx_core_tasks_head(void);
struct rhashtable *lupos_scx_core_tid_hash(void);
const struct rhashtable_params *lupos_scx_core_tid_hash_params(void);
struct scx_tid_alloc *lupos_scx_core_this_tid_alloc(void);
u64 lupos_scx_core_tid_cursor_fetch_add(u64 count);
void lupos_scx_core_rejected_inc(void);
bool lupos_scx_core_init_task_enabled(void);
bool lupos_scx_core_switching_all_read_once(void);
struct scx_sched *lupos_scx_core_enabling_sub_sched(void);
struct percpu_rw_semaphore *lupos_scx_core_fork_rwsem(void);

/* F01 is the sole native authority for its ext.c-private enum/iterator types. */
#include "sched_ext_core_slice_cursor_bindings.h"

/* Independently reviewed F06 interfaces; no duplicate shared native state. */
#include "sched_ext_core_task_lifetime_bindings.h"

/* Independently reviewed F08 interfaces and original shared plain flag read. */
#include "sched_ext_core_cgroup_bindings.h"
bool lupos_scx_core_cgroup_enabled(void);

/* Additive F09 interfaces, outside the independently reviewed foundation.
 * F00 owns the referenced storage; F09 owns the algorithms using it.
 * Borrowed addresses do not acquire locks or extend object lifetimes.
 */
const struct rhashtable_params *lupos_scx_core_dsq_hash_params(void);
struct llist_head *lupos_scx_core_dsqs_to_free(void);
struct kset *lupos_scx_core_kset(void);
long lupos_scx_core_rejected_read(void);
long lupos_scx_core_hotplug_seq_read(void);
long lupos_scx_core_enable_seq_read(void);
const char *lupos_scx_core_enable_state_name(enum scx_enable_state state);
u64 lupos_scx_core_sched_id_inc(void);
raw_spinlock_t *lupos_scx_core_bypass_lock(void);
raw_spinlock_t *lupos_scx_core_sched_lock(void);
struct list_head *lupos_scx_core_sched_all(void);
#ifdef CONFIG_EXT_SUB_SCHED
struct rhashtable *lupos_scx_core_sched_hash(void);
const struct rhashtable_params *lupos_scx_core_sched_hash_params(void);
#endif


#include "sched_ext_core_object_lifetime_bindings.h"

/* F11 exact native callbacks/format holder and shared dump-state borrowing. */
#include "sched_ext_core_exit_dump_bindings.h"
raw_spinlock_t *lupos_scx_core_dump_lock(void);
struct scx_dump_data *lupos_scx_core_dump_data(void);
#include "sched_ext_core_enqueue_bindings.h"
struct task_struct **lupos_scx_core_this_direct_dispatch_task(void);
void lupos_scx_core_spoil_direct_dispatch_task(void);
#include "sched_ext_core_consume_bindings.h"
#endif /* LUPOS_SCHED_EXT_CORE_BINDINGS_H */
