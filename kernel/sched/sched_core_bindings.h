/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_CORE_BINDINGS_H
#define LUPOS_SCHED_CORE_BINDINGS_H
/* Configured native header view, never MODULE; no handwritten Rust layouts. */
#define INSTANTIATE_EXPORTED_MIGRATE_DISABLE
#include <linux/sched.h>
#include <linux/highmem.h>
#include <linux/hrtimer_api.h>
#include <linux/ktime_api.h>
#include <linux/sched/signal.h>
#include <linux/syscalls_api.h>
#include <linux/debug_locks.h>
#include <linux/prefetch.h>
#include <linux/capability.h>
#include <linux/pgtable_api.h>
#include <linux/wait_bit.h>
#include <linux/jiffies.h>
#include <linux/spinlock_api.h>
#include <linux/cpumask_api.h>
#include <linux/lockdep_api.h>
#include <linux/hardirq.h>
#include <linux/softirq.h>
#include <linux/refcount_api.h>
#include <linux/topology.h>
#include <linux/sched/clock.h>
#include <linux/sched/cond_resched.h>
#include <linux/sched/cputime.h>
#include <linux/sched/debug.h>
#include <linux/sched/hotplug.h>
#include <linux/sched/init.h>
#include <linux/sched/isolation.h>
#include <linux/sched/loadavg.h>
#include <linux/sched/mm.h>
#include <linux/sched/nohz.h>
#include <linux/sched/rseq_api.h>
#include <linux/sched/rt.h>

#include <linux/context_tracking.h>
#include <linux/cpuset.h>
#include <linux/delayacct.h>
#include <linux/init_task.h>
#include <linux/interrupt.h>
#include <linux/ioprio.h>
#include <linux/kallsyms.h>
#include <linux/kcov.h>
#include <linux/kprobes.h>
#include <linux/llist_api.h>
#include <linux/mmu_context.h>
#include <linux/mmzone.h>
#include <linux/mutex_api.h>
#include <linux/nmi.h>
#include <linux/nospec.h>
#include <linux/perf_event_api.h>
#include <linux/profile.h>
#include <linux/psi.h>
#include <linux/rcuwait_api.h>
#include <linux/rseq.h>
#include <linux/sched/wake_q.h>
#include <linux/scs.h>
#include <linux/slab.h>
#include <linux/syscalls.h>
#include <linux/vtime.h>
#include <linux/wait_api.h>
#include <linux/workqueue_api.h>
#include <linux/livepatch_sched.h>

#ifdef CONFIG_PREEMPT_DYNAMIC
# ifdef CONFIG_GENERIC_IRQ_ENTRY
#  include <linux/irq-entry-common.h>
# endif
#endif

#include <uapi/linux/sched/types.h>

#include <asm/irq_regs.h>
#include <asm/switch_to.h>
#include <asm/tlb.h>

#ifdef LUPOS_CORE_DEFINE_TRACE_POINTS
#define CREATE_TRACE_POINTS
#endif
#include <linux/sched/rseq_api.h>
#include <trace/events/sched.h>
#include <trace/events/ipi.h>
#ifdef LUPOS_CORE_DEFINE_TRACE_POINTS
#undef CREATE_TRACE_POINTS
#endif

#include "sched.h"
#include "stats.h"

#include "autogroup.h"
#include "pelt.h"
#include "smp.h"

#include "../workqueue_internal.h"
#include "../../io_uring/io-wq.h"
#include "../smpboot.h"
#include "../locking/mutex.h"
/* Verbatim private core.c declarations, made visible only to configured bindgen. */
struct migration_arg {
    struct task_struct *task;
    int dest_cpu;
    struct set_affinity_pending *pending;
};
struct set_affinity_pending {
    refcount_t refs;
    unsigned int stop_pending;
    struct completion done;
    struct cpu_stop_work stop_work;
    struct migration_arg arg;
};
#ifdef CONFIG_NUMA_BALANCING
struct migration_swap_arg {
    struct task_struct *src_task, *dst_task;
    int src_cpu, dst_cpu;
};
extern int sysctl_numa_balancing_mode;
#endif
union lupos_core_cpumask_rcuhead { cpumask_t cpumask; struct rcu_head rcu; };
/* Same three-state local enum from select_fallback_rq; no guessed values. */
enum lupos_core_fallback_state { LUPOS_CORE_FALLBACK_CPUSET, LUPOS_CORE_FALLBACK_POSSIBLE, LUPOS_CORE_FALLBACK_FAIL };
#ifdef CONFIG_SCHED_HRTICK
/* Exact private hrtick enum from core.c, with uniquely scoped binding names. */
enum { LUPOS_CORE_HRTICK_SCHED_NONE = 0,
       LUPOS_CORE_HRTICK_SCHED_DEFER = BIT(1),
       LUPOS_CORE_HRTICK_SCHED_START = BIT(2),
       LUPOS_CORE_HRTICK_SCHED_REARM_HRTIMER = BIT(3) };
#endif
#ifdef CONFIG_SCHED_CORE
extern struct mutex lupos_core_sched_core_mutex;
extern atomic_t lupos_core_sched_core_count;
extern struct cpumask lupos_core_sched_core_mask;
extern struct work_struct lupos_core_sched_core_put_work;
void lupos_core_core_put_workfn(struct work_struct *work);
#endif
#ifdef CONFIG_UCLAMP_TASK
extern struct mutex lupos_core_uclamp_mutex;
extern unsigned int lupos_core_sysctl_sched_uclamp_util_min;
extern unsigned int lupos_core_sysctl_sched_uclamp_util_max;
extern unsigned int sysctl_sched_uclamp_util_min_rt_default;
extern struct uclamp_se lupos_core_uclamp_default[UCLAMP_CNT];
#endif
int lupos_core_setup_proxy_exec(char *str);
#ifdef CONFIG_SCHEDSTATS
int lupos_core_setup_schedstats(char *str);
#endif
#ifdef CONFIG_SYSCTL
int lupos_core_sched_core_sysctl_init(void);
#ifdef CONFIG_SCHEDSTATS
int lupos_core_sysctl_schedstats(const struct ctl_table *, int, void *, size_t *, loff_t *);
#endif
#ifdef CONFIG_UCLAMP_TASK
int lupos_core_sysctl_sched_uclamp_handler(const struct ctl_table *, int, void *, size_t *, loff_t *);
#endif
#ifdef CONFIG_NUMA_BALANCING
int lupos_core_sysctl_numa_balancing(const struct ctl_table *, int, void *, size_t *, loff_t *);
#endif
#endif
#ifndef CONFIG_X86_64
/* Deliberate open architecture lowering, not a stub or forwarding body. */
struct task_struct *lupos_core_arch_switch_to(struct task_struct *, struct task_struct *);
void lupos_core_compiler_barrier(void);
#endif
void lupos_core_context_transfer_rq(struct rq *rq);
void lupos_core_lockdep_stop_pi_class(struct task_struct *stop);
#ifdef CONFIG_SYSCTL
void lupos_core_register_core_sysctl(void);
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
extern struct static_key_false lupos_core_sk_dynamic_preempt_lazy;
#endif
#ifdef CONFIG_PREEMPT_NOTIFIERS
extern struct static_key_false lupos_core_preempt_notifier_key;
#endif
#include "sched_core_private.h"
#include "sched_core_callbacks.h"
#include "sched_core_constants.h"
#define LUPOS_CORE_VALUE(type, name, args, ...) type name args;
#define LUPOS_CORE_VOID(name, args, ...) void name args;
#define LUPOS_CORE_BODY(type, name, args, ...) type name args;
#include "sched_core_native_leaves.def"
#include "sched_core_header_leaves.def"
#include "sched_core_state_leaves.def"
#include "sched_core_dynamic_leaves.def"
#include "sched_core_diagnostics.def"
#undef LUPOS_CORE_VALUE
#undef LUPOS_CORE_VOID
#undef LUPOS_CORE_BODY
#endif
