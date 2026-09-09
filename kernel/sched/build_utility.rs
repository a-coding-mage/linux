// SPDX-License-Identifier: GPL-2.0-only
/*
 * These are various utility functions of the scheduler,
 * built in a single compilation unit for build efficiency reasons.
 *
 * ( Incidentally, the size of the compilation unit is roughly
 *   comparable to core.c, fair.c, smp.c and policy.c, the other
 *   big compilation units. This helps balance build time, while
 *   coalescing source files to amortize header inclusion
 *   cost. )
 */

// External dependencies supplied by the scheduler and kernel modules:
// linux/sched/clock.h, linux/sched/cputime.h, linux/sched/debug.h,
// linux/sched/isolation.h, linux/sched/loadavg.h, linux/sched/nohz.h,
// linux/sched/mm.h, linux/sched/rseq_api.h, linux/sched/task_stack.h,
// linux/cpufreq.h, linux/cpumask_api.h, linux/cpuset.h, linux/ctype.h,
// linux/debugfs.h, linux/energy_model.h, linux/hashtable_api.h, linux/irq.h,
// linux/kobject_api.h, linux/membarrier.h, linux/mempolicy.h, linux/nmi.h,
// linux/nospec.h, linux/proc_fs.h, linux/psi.h, linux/ptrace_api.h,
// linux/sched_clock.h, linux/security.h, linux/spinlock_api.h,
// linux/swait_api.h, linux/timex.h, linux/utsname.h, linux/wait_api.h,
// linux/workqueue_api.h, uapi/linux/prctl.h, uapi/linux/sched/types.h,
// asm/switch_to.h, sched.h, sched-pelt.h, stats.h, and autogroup.h.

// The C source includes the following compilation units in this unit:
// clock.c
// debug.c
// loadavg.c
// completion.c
// swait.c
// wait_bit.c
// wait.c
// cpupri.c
// stop_task.c
// topology.c

// Build-time conditional compilation units. Their conditions are supplied
// by the surrounding kernel build configuration and are intentionally not
// resolved in this translation unit.
// CONFIG_CGROUP_CPUACCT: cpuacct.c
// CONFIG_CPU_FREQ: cpufreq.c
// CONFIG_CPU_FREQ_GOV_SCHEDUTIL: cpufreq_schedutil.c
// CONFIG_SCHEDSTATS: stats.c
// CONFIG_SCHED_CORE: core_sched.c
// CONFIG_PSI: psi.c
// CONFIG_MEMBARRIER: membarrier.c
// CONFIG_CPU_ISOLATION: isolation.c
// CONFIG_SCHED_AUTOGROUP: autogroup.c

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
