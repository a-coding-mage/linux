/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_WORKQUEUE_PRIVATE_H
#define LUPOS_WORKQUEUE_PRIVATE_H
/*
 * Single configured native type authority for the proposed Rust workqueue
 * owner. Exact declaration text from kernel/workqueue.c at
 * 126a30fae3bba11420ec2fcbde51a0a01bab1b5b, lines 62-416, follows the original
 * direct include set. Original sources and headers are not changed.
 *
 * This is an input proposal, NOT generated bindings or ABI/layout proof.
 * The configured generator must retain native public type identities and
 * conditional fields, per-CPU address spaces, alignment and flexible arrays.
 */

#include <linux/export.h>
#include <linux/kernel.h>
#include <linux/sched.h>
#include <linux/init.h>
#include <linux/interrupt.h>
#include <linux/signal.h>
#include <linux/completion.h>
#include <linux/workqueue.h>
#include <linux/slab.h>
#include <linux/cpu.h>
#include <linux/notifier.h>
#include <linux/kthread.h>
#include <linux/hardirq.h>
#include <linux/mempolicy.h>
#include <linux/freezer.h>
#include <linux/debug_locks.h>
#include <linux/device/devres.h>
#include <linux/lockdep.h>
#include <linux/idr.h>
#include <linux/jhash.h>
#include <linux/hashtable.h>
#include <linux/rculist.h>
#include <linux/nodemask.h>
#include <linux/moduleparam.h>
#include <linux/uaccess.h>
#include <linux/sched/isolation.h>
#include <linux/sched/debug.h>
#include <linux/nmi.h>
#include <linux/kvm_para.h>
#include <linux/delay.h>
#include <linux/irq_work.h>

#include "workqueue_internal.h"

enum worker_pool_flags {
	/*
	 * worker_pool flags
	 *
	 * A bound pool is either associated or disassociated with its CPU.
	 * While associated (!DISASSOCIATED), all workers are bound to the
	 * CPU and none has %WORKER_UNBOUND set and concurrency management
	 * is in effect.
	 *
	 * While DISASSOCIATED, the cpu may be offline and all workers have
	 * %WORKER_UNBOUND set and concurrency management disabled, and may
	 * be executing on any CPU.  The pool behaves as an unbound one.
	 *
	 * Note that DISASSOCIATED should be flipped only while holding
	 * wq_pool_attach_mutex to avoid changing binding state while
	 * worker_attach_to_pool() is in progress.
	 *
	 * As there can only be one concurrent BH execution context per CPU, a
	 * BH pool is per-CPU and always DISASSOCIATED.
	 */
	POOL_BH			= 1 << 0,	/* is a BH pool */
	POOL_MANAGER_ACTIVE	= 1 << 1,	/* being managed */
	POOL_DISASSOCIATED	= 1 << 2,	/* cpu can't serve workers */
	POOL_BH_DRAINING	= 1 << 3,	/* draining after CPU offline */
};

enum worker_flags {
	/* worker flags */
	WORKER_DIE		= 1 << 1,	/* die die die */
	WORKER_IDLE		= 1 << 2,	/* is idle */
	WORKER_PREP		= 1 << 3,	/* preparing to run works */
	WORKER_CPU_INTENSIVE	= 1 << 6,	/* cpu intensive */
	WORKER_UNBOUND		= 1 << 7,	/* worker is unbound */
	WORKER_REBOUND		= 1 << 8,	/* worker was rebound */

	WORKER_NOT_RUNNING	= WORKER_PREP | WORKER_CPU_INTENSIVE |
				  WORKER_UNBOUND | WORKER_REBOUND,
};

enum work_cancel_flags {
	WORK_CANCEL_DELAYED	= 1 << 0,	/* canceling a delayed_work */
	WORK_CANCEL_DISABLE	= 1 << 1,	/* canceling to disable */
};

enum wq_internal_consts {
	NR_STD_WORKER_POOLS	= 2,		/* # standard pools per cpu */

	UNBOUND_POOL_HASH_ORDER	= 6,		/* hashed by pool->attrs */
	BUSY_WORKER_HASH_ORDER	= 6,		/* 64 pointers */

	MAX_IDLE_WORKERS_RATIO	= 4,		/* 1/4 of busy can be idle */
	IDLE_WORKER_TIMEOUT	= 300 * HZ,	/* keep idle ones for 5 mins */

	MAYDAY_INITIAL_TIMEOUT  = HZ / 100 >= 2 ? HZ / 100 : 2,
						/* call for help after 10ms
						   (min two ticks) */
	MAYDAY_INTERVAL		= HZ / 10,	/* and then every 100ms */
	CREATE_COOLDOWN		= HZ,		/* time to breath after fail */

	RESCUER_BATCH		= 16,		/* process items per turn */

	/*
	 * Rescue workers are used only on emergencies and shared by
	 * all cpus.  Give MIN_NICE.
	 */
	RESCUER_NICE_LEVEL	= MIN_NICE,
	HIGHPRI_NICE_LEVEL	= MIN_NICE,

	WQ_NAME_LEN		= 32,
	WORKER_ID_LEN		= 10 + WQ_NAME_LEN, /* "kworker/R-" + WQ_NAME_LEN */
};

/* Layout of shards within one LLC pod */
struct llc_shard_layout {
	int nr_large_shards;	/* number of large shards (cores_per_shard + 1) */
	int cores_per_shard;	/* base number of cores per default shard */
	int nr_shards;		/* total number of shards */
	/* nr_default shards = (nr_shards - nr_large_shards) */
};

/*
 * We don't want to trap softirq for too long. See MAX_SOFTIRQ_TIME and
 * MAX_SOFTIRQ_RESTART in kernel/softirq.c. These are macros because
 * msecs_to_jiffies() can't be an initializer.
 */
#define BH_WORKER_JIFFIES	msecs_to_jiffies(2)
#define BH_WORKER_RESTARTS	10

/*
 * Structure fields follow one of the following exclusion rules.
 *
 * I: Modifiable by initialization/destruction paths and read-only for
 *    everyone else.
 *
 * P: Preemption protected.  Disabling preemption is enough and should
 *    only be modified and accessed from the local cpu.
 *
 * L: pool->lock protected.  Access with pool->lock held.
 *
 * LN: pool->lock and wq_node_nr_active->lock protected for writes. Either for
 *     reads.
 *
 * K: Only modified by worker while holding pool->lock. Can be safely read by
 *    self, while holding pool->lock or from IRQ context if %current is the
 *    kworker.
 *
 * S: Only modified by worker self.
 *
 * A: wq_pool_attach_mutex protected.
 *
 * PL: wq_pool_mutex protected.
 *
 * PR: wq_pool_mutex protected for writes.  RCU protected for reads.
 *
 * PW: wq_pool_mutex and wq->mutex protected for writes.  Either for reads.
 *
 * PWR: wq_pool_mutex and wq->mutex protected for writes.  Either or
 *      RCU for reads.
 *
 * WQ: wq->mutex protected.
 *
 * WR: wq->mutex protected for writes.  RCU protected for reads.
 *
 * WO: wq->mutex protected for writes. Updated with WRITE_ONCE() and can be read
 *     with READ_ONCE() without locking.
 *
 * MD: wq_mayday_lock protected.
 *
 * WD: Used internally by the watchdog.
 */

/* struct worker is defined in workqueue_internal.h */

struct worker_pool {
	raw_spinlock_t		lock;		/* the pool lock */
	int			cpu;		/* I: the associated cpu */
	int			node;		/* I: the associated node ID */
	int			id;		/* I: pool ID */
	unsigned int		flags;		/* L: flags */

	unsigned long		last_progress_ts;	/* L: last forward progress timestamp */
	bool			cpu_stall;	/* WD: stalled cpu bound pool */

	/*
	 * The counter is incremented in a process context on the associated CPU
	 * w/ preemption disabled, and decremented or reset in the same context
	 * but w/ pool->lock held. The readers grab pool->lock and are
	 * guaranteed to see if the counter reached zero.
	 */
	int			nr_running;

	struct list_head	worklist;	/* L: list of pending works */

	int			nr_workers;	/* L: total number of workers */
	int			nr_idle;	/* L: currently idle workers */

	struct list_head	idle_list;	/* L: list of idle workers */
	struct timer_list	idle_timer;	/* L: worker idle timeout */
	struct work_struct      idle_cull_work; /* L: worker idle cleanup */

	struct timer_list	mayday_timer;	  /* L: SOS timer for workers */

	/* a workers is either on busy_hash or idle_list, or the manager */
	DECLARE_HASHTABLE(busy_hash, BUSY_WORKER_HASH_ORDER);
						/* L: hash of busy workers */

	struct worker		*manager;	/* L: purely informational */
	/* L: last worker woken by kick_pool() */
	struct worker		*last_woken_worker;
	struct list_head	workers;	/* A: attached workers */

	struct ida		worker_ida;	/* worker IDs for task name */

	struct workqueue_attrs	*attrs;		/* I: worker attributes */
	struct hlist_node	hash_node;	/* PL: unbound_pool_hash node */
	int			refcnt;		/* PL: refcnt for unbound pools */
#ifdef CONFIG_PREEMPT_RT
	spinlock_t		cb_lock;	/* BH worker cancel lock */
#endif
	/*
	 * Destruction of pool is RCU protected to allow dereferences
	 * from get_work_pool().
	 */
	struct rcu_head		rcu;
};

/*
 * Per-pool_workqueue statistics. These can be monitored using
 * tools/workqueue/wq_monitor.py.
 */
enum pool_workqueue_stats {
	PWQ_STAT_STARTED,	/* work items started execution */
	PWQ_STAT_COMPLETED,	/* work items completed execution */
	PWQ_STAT_CPU_TIME,	/* total CPU time consumed */
	PWQ_STAT_CPU_INTENSIVE,	/* wq_cpu_intensive_thresh_us violations */
	PWQ_STAT_CM_WAKEUP,	/* concurrency-management worker wakeups */
	PWQ_STAT_REPATRIATED,	/* unbound workers brought back into scope */
	PWQ_STAT_MAYDAY,	/* maydays to rescuer */
	PWQ_STAT_RESCUED,	/* linked work items executed by rescuer */

	PWQ_NR_STATS,
};

/*
 * The per-pool workqueue.  While queued, bits below WORK_PWQ_SHIFT
 * of work_struct->data are used for flags and the remaining high bits
 * point to the pwq; thus, pwqs need to be aligned at two's power of the
 * number of flag bits.
 */
struct pool_workqueue {
	struct worker_pool	*pool;		/* I: the associated pool */
	struct workqueue_struct *wq;		/* I: the owning workqueue */
	int			work_color;	/* L: current color */
	int			flush_color;	/* L: flushing color */
	int			refcnt;		/* L: reference count */
	int			nr_in_flight[WORK_NR_COLORS];
						/* L: nr of in_flight works */
	bool			plugged;	/* L: execution suspended */

	/*
	 * nr_active management and WORK_STRUCT_INACTIVE:
	 *
	 * When pwq->nr_active >= max_active, new work item is queued to
	 * pwq->inactive_works instead of pool->worklist and marked with
	 * WORK_STRUCT_INACTIVE.
	 *
	 * All work items marked with WORK_STRUCT_INACTIVE do not participate in
	 * nr_active and all work items in pwq->inactive_works are marked with
	 * WORK_STRUCT_INACTIVE. But not all WORK_STRUCT_INACTIVE work items are
	 * in pwq->inactive_works. Some of them are ready to run in
	 * pool->worklist or worker->scheduled. Those work itmes are only struct
	 * wq_barrier which is used for flush_work() and should not participate
	 * in nr_active. For non-barrier work item, it is marked with
	 * WORK_STRUCT_INACTIVE iff it is in pwq->inactive_works.
	 */
	int			nr_active;	/* L: nr of active works */
	struct list_head	inactive_works;	/* L: inactive works */
	struct list_head	pending_node;	/* LN: node on wq_node_nr_active->pending_pwqs */
	struct list_head	pwqs_node;	/* WR: node on wq->pwqs */
	struct list_head	mayday_node;	/* MD: node on wq->maydays */
	struct work_struct	mayday_cursor;	/* L: cursor on pool->worklist */

	u64			stats[PWQ_NR_STATS];

	/*
	 * Release of unbound pwq is punted to a kthread_worker. See put_pwq()
	 * and pwq_release_workfn() for details. pool_workqueue itself is also
	 * RCU protected so that the first pwq can be determined without
	 * grabbing wq->mutex.
	 */
	struct kthread_work	release_work;
	struct rcu_head		rcu;
} __aligned(1 << WORK_STRUCT_PWQ_SHIFT);

/*
 * Structure used to wait for workqueue flush.
 */
struct wq_flusher {
	struct list_head	list;		/* WQ: list of flushers */
	int			flush_color;	/* WQ: flush color waiting for */
	struct completion	done;		/* flush completion */
};

struct wq_device;

/*
 * Unlike in a per-cpu workqueue where max_active limits its concurrency level
 * on each CPU, in an unbound workqueue, max_active applies to the whole system.
 * As sharing a single nr_active across multiple sockets can be very expensive,
 * the counting and enforcement is per NUMA node.
 *
 * The following struct is used to enforce per-node max_active. When a pwq wants
 * to start executing a work item, it should increment ->nr using
 * tryinc_node_nr_active(). If acquisition fails due to ->nr already being over
 * ->max, the pwq is queued on ->pending_pwqs. As in-flight work items finish
 * and decrement ->nr, node_activate_pending_pwq() activates the pending pwqs in
 * round-robin order.
 */
struct wq_node_nr_active {
	int			max;		/* per-node max_active */
	atomic_t		nr;		/* per-node nr_active */
	raw_spinlock_t		lock;		/* nests inside pool locks */
	struct list_head	pending_pwqs;	/* LN: pwqs with inactive works */
};

/*
 * The externally visible workqueue.  It relays the issued work items to
 * the appropriate worker_pool through its pool_workqueues.
 */
struct workqueue_struct {
	struct list_head	pwqs;		/* WR: all pwqs of this wq */
	struct list_head	list;		/* PR: list of all workqueues */

	struct mutex		mutex;		/* protects this wq */
	int			work_color;	/* WQ: current work color */
	int			flush_color;	/* WQ: current flush color */
	atomic_t		nr_pwqs_to_flush; /* flush in progress */
	struct wq_flusher	*first_flusher;	/* WQ: first flusher */
	struct list_head	flusher_queue;	/* WQ: flush waiters */
	struct list_head	flusher_overflow; /* WQ: flush overflow list */

	struct list_head	maydays;	/* MD: pwqs requesting rescue */
	struct worker		*rescuer;	/* MD: rescue worker */

	int			nr_drainers;	/* WQ: drain in progress */

	/* See alloc_workqueue() function comment for info on min/max_active */
	int			max_active;	/* WO: max active works */
	int			min_active;	/* WO: min active works */
	int			saved_max_active; /* WQ: saved max_active */
	int			saved_min_active; /* WQ: saved min_active */

	struct workqueue_attrs	*attrs;	/* PW: workqueue attributes */
	struct pool_workqueue __rcu *dfl_pwq;   /* PW: only for unbound wqs */

#ifdef CONFIG_SYSFS
	struct wq_device	*wq_dev;	/* I: for sysfs interface */
#endif
#ifdef CONFIG_LOCKDEP
	char			*lock_name;
	struct lock_class_key	key;
	struct lockdep_map	__lockdep_map;
	struct lockdep_map	*lockdep_map;
#endif
	char			name[WQ_NAME_LEN]; /* I: workqueue name */

	/*
	 * Destruction of workqueue_struct is RCU protected to allow walking
	 * the workqueues list without grabbing wq_pool_mutex.
	 * This is used to dump all workqueues from sysrq.
	 */
	struct rcu_head		rcu;

	/* hot fields used during command issue, aligned to cacheline */
	unsigned int		flags ____cacheline_aligned; /* WQ: WQ_* flags */
	struct pool_workqueue __rcu * __percpu *cpu_pwq; /* I: per-cpu pwqs */
	struct wq_node_nr_active *node_nr_active[]; /* I: per-node nr_active */
};

/*
 * Each pod type describes how CPUs should be grouped for unbound workqueues.
 * See the comment above workqueue_attrs->affn_scope.
 */
struct wq_pod_type {
	int			nr_pods;	/* number of pods */
	cpumask_var_t		*pod_cpus;	/* pod -> cpus */
	int			*pod_node;	/* pod -> node */
	int			*cpu_pod;	/* cpu -> pod */
};

struct work_offq_data {
	u32			pool_id;
	u32			disable;
	u32			flags;
};

struct wq_barrier {
	struct work_struct	work;
	struct completion	done;
	struct task_struct	*task;	/* purely informational */
};

/* Native C3804-3808, shared configured stack-work type. */
struct wq_drain_dead_softirq_work {
	struct work_struct	work;
	struct worker_pool	*pool;
	struct completion	done;
};

/* Native C5449-5455, transaction-local configured flexible-array type. */
struct apply_wqattrs_ctx {
	struct workqueue_struct	*wq;		/* target workqueue */
	struct workqueue_attrs	*attrs;		/* attrs to apply */
	struct list_head	list;		/* queued for batching commit */
	struct pool_workqueue	*dfl_pwq;
	struct pool_workqueue	*pwq_tbl[];
};

/* Native C7028-7033 remains inside its original CONFIG_SMP family. */
#ifdef CONFIG_SMP
struct work_for_cpu {
	struct work_struct work;
	long (*fn)(void *);
	void *arg;
	long ret;
};
#endif

#endif /* LUPOS_WORKQUEUE_PRIVATE_H */
