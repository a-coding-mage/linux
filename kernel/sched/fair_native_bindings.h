/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_FAIR_NATIVE_BINDINGS_H
#define RUST_FAIR_NATIVE_BINDINGS_H
#include <linux/energy_model.h>
#include <linux/mmap_lock.h>
#include <linux/hugetlb_inline.h>
#include <linux/jiffies.h>
#include <linux/mm_api.h>
#include <linux/highmem.h>
#include <linux/hrtimer.h>
#include <linux/hrtimer_bases.h>
#include <linux/spinlock_api.h>
#include <linux/cpumask_api.h>
#include <linux/lockdep_api.h>
#include <linux/softirq.h>
#include <linux/refcount_api.h>
#include <linux/topology.h>
#include <linux/sched/clock.h>
#include <linux/sched/cond_resched.h>
#include <linux/sched/cputime.h>
#include <linux/sched/isolation.h>
#include <linux/sched/nohz.h>
#include <linux/sched/prio.h>
#include <linux/static_call.h>

#include <linux/cpuidle.h>
#include <linux/interrupt.h>
#include <linux/memory-tiers.h>
#include <linux/mempolicy.h>
#include <linux/mutex_api.h>
#include <linux/profile.h>
#include <linux/psi.h>
#include <linux/ratelimit.h>
#include <linux/task_work.h>
#include <linux/rbtree_augmented.h>

#include <asm/switch_to.h>

#include <uapi/linux/sched/types.h>

#include "sched.h"
#include "stats.h"
#include "autogroup.h"

#include "pelt.h"
#include <linux/cpuset.h>
#include <linux/slab.h>
#include <linux/sched/numa_balancing.h>
#include "fair_private.h"
#include "fair_constants.h"
extern const struct sched_class fair_sched_class;
#ifdef CONFIG_FAIR_GROUP_SCHED
long calc_concur_shares(struct cfs_rq *);
DECLARE_STATIC_CALL(rust_fair_calc_group_shares, calc_concur_shares);
#endif
#ifdef CONFIG_CFS_BANDWIDTH
extern unsigned int sysctl_sched_cfs_bandwidth_slice;
#ifdef CONFIG_JUMP_LABEL

#endif
#endif
#ifdef CONFIG_NUMA_BALANCING
extern unsigned int sysctl_numa_balancing_promote_rate_limit;
#endif
#define FAIR_LEAF(ret, name, args, body) ret rust_fair_##name args;
#include "fair_native_primitives.inc"
#undef FAIR_LEAF
#define FAIR_WARNING(name) bool name(bool condition);
#include "fair_foundation_warnings.inc"
#include "fair_tail_warnings.inc"
#undef FAIR_WARNING
#ifdef CONFIG_SYSCTL
void rust_fair_register_sysctl_init(void);
#endif
#endif
