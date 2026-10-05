/* SPDX-License-Identifier: GPL-2.0 */
/* Exact private declarations from fair.c, e1d84f501551943a11f4c5271e9f5c85d7e15168. */
#ifndef RUST_FAIR_PRIVATE_H
#define RUST_FAIR_PRIVATE_H
/* fair.c:2265 */
#ifdef CONFIG_NUMA_BALANCING
struct numa_group {
	refcount_t refcount;

	spinlock_t lock; /* nr_tasks, tasks */
	int nr_tasks;
	pid_t gid;
	int active_nodes;

	struct rcu_head rcu;
	unsigned long total_faults;
	unsigned long max_faults_cpu;
	/*
	 * faults[] array is split into two regions: faults_mem and faults_cpu.
	 *
	 * Faults_cpu is used to decide whether memory should move
	 * towards the CPU. As a consequence, these stats are weighted
	 * more by CPU use than by memory faults.
	 */
	unsigned long faults[];
};
#endif

/* fair.c:2833 */
#ifdef CONFIG_NUMA_BALANCING
enum numa_type {
	/* The node has spare capacity that can be used to run more tasks.  */
	node_has_spare = 0,
	/*
	 * The node is fully used and the tasks don't compete for more CPU
	 * cycles. Nevertheless, some tasks might wait before running.
	 */
	node_fully_busy,
	/*
	 * The node is overloaded and can't provide expected CPU cycles to all
	 * tasks.
	 */
	node_overloaded
};
#endif

/* fair.c:2849 */
#ifdef CONFIG_NUMA_BALANCING
struct numa_stats {
	unsigned long load;
	unsigned long runnable;
	unsigned long util;
	/* Total compute capacity of CPUs on a node */
	unsigned long compute_capacity;
	unsigned int nr_running;
	unsigned int weight;
	enum numa_type node_type;
	int idle_cpu;
};
#endif

/* fair.c:2861 */
#ifdef CONFIG_NUMA_BALANCING
struct task_numa_env {
	struct task_struct *p;

	int src_cpu, src_nid;
	int dst_cpu, dst_nid;
	int imb_numa_nr;

	struct numa_stats src_stats, dst_stats;

	int imbalance_pct;
	int dist;

	struct task_struct *best_task;
	long best_imp;
	int best_cpu;
};
#endif

/* fair.c:8788 */

enum asym_fits_state {
	ASYM_IDLE_UCLAMP_MISFIT = -4,
	ASYM_IDLE_COMPLETE_MISFIT,
	ASYM_IDLE_THREAD_FITS,
	ASYM_IDLE_THREAD_UCLAMP_MISFIT,
	ASYM_IDLE_THREAD_MISFIT,

	/* util_fits_cpu() bias for idle core */
	ASYM_IDLE_CORE_BIAS = -3,
};

/* fair.c:9329 */

struct energy_env {
	unsigned long task_busy_time;
	unsigned long pd_busy_time;
	unsigned long cpu_cap;
	unsigned long pd_cap;
};

/* fair.c:9827 */

enum preempt_wakeup_action {
	PREEMPT_WAKEUP_NONE,	/* No preemption. */
	PREEMPT_WAKEUP_SHORT,	/* Ignore slice protection. */
	PREEMPT_WAKEUP_PICK,	/* Let pick_eevdf() decide. */
	PREEMPT_WAKEUP_RESCHED,	/* Force reschedule. */
};

/* fair.c:10321 */

enum fbq_type { regular, remote, all };

/* fair.c:10330 */

enum group_type {
	/* The group has spare capacity that can be used to run more tasks.  */
	group_has_spare = 0,
	/*
	 * The group is fully used and the tasks don't compete for more CPU
	 * cycles. Nevertheless, some tasks might wait before running.
	 */
	group_fully_busy,
	/*
	 * One task doesn't fit with CPU's capacity and must be migrated to a
	 * more powerful CPU.
	 */
	group_misfit_task,
	/*
	 * Balance SMT group that's fully busy. Can benefit from migration
	 * a task on SMT with busy sibling to another CPU on idle core.
	 */
	group_smt_balance,
	/*
	 * SD_ASYM_PACKING only: One local CPU with higher capacity is available,
	 * and the task should be migrated to it instead of running on the
	 * current CPU.
	 */
	group_asym_packing,
	/*
	 * The tasks' affinity constraints previously prevented the scheduler
	 * from balancing the load across the system.
	 */
	group_imbalanced,
	/*
	 * There are tasks running on non-preferred LLC, possible to move
	 * them to their preferred LLC without creating too much imbalance.
	 * The priority of group_llc_balance is lower than that of
	 * group_overloaded and higher than that of all other group types.
	 * This is because group_llc_balance may exacerbate load imbalance.
	 * If the LLC balancing attempt fails, the nr_balance_failed
	 * mechanism will trigger other group types to rebalance the load.
	 */
	group_llc_balance,
	/*
	 * The CPU is overloaded and can't provide expected CPU cycles to all
	 * tasks.
	 */
	group_overloaded
};

/* fair.c:10376 */

enum migration_type {
	migrate_load = 0,
	migrate_util,
	migrate_task,
	migrate_misfit,
	migrate_llc_task
};

/* fair.c:10391 */

struct lb_env {
	struct sched_domain	*sd;

	struct rq		*src_rq;
	int			src_cpu;

	int			dst_cpu;
	struct rq		*dst_rq;
	bool			dst_core_idle;

	struct cpumask		*dst_grpmask;
	int			new_dst_cpu;
	enum cpu_idle_type	idle;
	long			imbalance;
	/* The set of CPUs under consideration for load-balancing */
	struct cpumask		*cpus;

	unsigned int		flags;

	unsigned int		loop;
	unsigned int		loop_break;
	unsigned int		loop_max;

	enum fbq_type		fbq_type;
	enum migration_type	migration_type;
	struct list_head	tasks;
};

/* fair.c:10640 */
#ifdef CONFIG_SCHED_CACHE
enum llc_mig {
	mig_forbid = 0,		/* N: Don't migrate task, respect LLC preference */
	mig_llc,		/* Y: Do LLC preference based migration */
	mig_unrestricted	/* G: Don't restrict generic load balance migration */
};
#endif

/* fair.c:11381 */

struct sg_lb_stats {
	unsigned long avg_load;			/* Avg load            over the CPUs of the group */
	unsigned long group_load;		/* Total load          over the CPUs of the group */
	unsigned long group_capacity;		/* Capacity            over the CPUs of the group */
	unsigned long group_util;		/* Total utilization   over the CPUs of the group */
	unsigned long group_runnable;		/* Total runnable time over the CPUs of the group */
	unsigned int sum_nr_running;		/* Nr of all tasks running in the group */
	unsigned int sum_h_nr_running;		/* Nr of CFS tasks running in the group */
	unsigned int idle_cpus;                 /* Nr of idle CPUs         in the group */
	unsigned int group_weight;
	enum group_type group_type;
	unsigned int group_asym_packing;	/* Tasks should be moved to preferred CPU */
	unsigned int group_smt_balance;		/* Task on busy SMT be moved */
	unsigned int group_llc_balance;		/* Tasks should be moved to preferred LLC */
	unsigned long group_misfit_task_load;	/* A CPU has a task too big for its capacity */
	unsigned int group_overutilized;	/* At least one CPU is overutilized in the group */
#ifdef CONFIG_NUMA_BALANCING
	unsigned int nr_numa_running;
	unsigned int nr_preferred_running;
#endif
#ifdef CONFIG_SCHED_CACHE
	unsigned int nr_pref_dst_llc;
#endif
};

/* fair.c:11409 */

struct sd_lb_stats {
	struct sched_group *busiest;		/* Busiest group in this sd */
	struct sched_group *local;		/* Local group in this sd */
	unsigned long total_load;		/* Total load of all groups in sd */
	unsigned long total_capacity;		/* Total capacity of all groups in sd */
	unsigned long avg_load;			/* Average load across all groups in sd */
	unsigned int prefer_sibling;		/* Tasks should go to sibling first */

	struct sg_lb_stats busiest_stat;	/* Statistics of the busiest group */
	struct sg_lb_stats local_stat;		/* Statistics of the local group */
};

#endif
/* Exact anonymous nohz layout, with a tag solely to expose native layout. */
#if defined(CONFIG_NO_HZ_COMMON) && !defined(RUST_FAIR_NOHZ_PRIVATE)
#define RUST_FAIR_NOHZ_PRIVATE
struct rust_fair_nohz_state {
    cpumask_var_t idle_cpus_mask;
    int has_blocked_load;
    int needs_update;
    unsigned long next_balance;
    unsigned long next_blocked;
};
#endif
