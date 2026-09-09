// SPDX-License-Identifier: GPL-2.0-only
/*
 * These are the scheduling policy related scheduler files, built
 * in a single compilation unit for build efficiency reasons.
 *
 * ( Incidentally, the size of the compilation unit is roughly
 *   comparable to core.c and fair.c, the other two big compilation
 *   units. This helps balance build time, while coalescing source files to amortize header inclusion
 *   cost. )
 *
 * core.c and fair.c are built separately.
 */

/* Headers supplied by the surrounding kernel translation unit:
 *
 * linux/sched/clock.h, linux/sched/cputime.h, linux/sched/hotplug.h,
 * linux/sched/isolation.h, linux/sched/posix-timers.h, linux/sched/rt.h,
 * linux/cpuidle.h, linux/jiffies.h, linux/kobject.h, linux/livepatch.h,
 * linux/pm.h, linux/psi.h, linux/rhashtable.h, linux/seq_buf.h,
 * linux/seqlock_api.h, linux/slab.h, linux/suspend.h, linux/tsacct_kern.h,
 * linux/vtime.h, linux/sysrq.h, linux/percpu-rwsem.h,
 * uapi/linux/sched/types.h, sched.h, smp.h, autogroup.h, stats.h, pelt.h
 */

/* Source code modules included into this compilation unit:
 * idle.c
 * rt.c
 * cpudeadline.c
 * pelt.c
 * cputime.c
 * deadline.c
 */

/* CONFIG_SCHED_CLASS_EXT conditionally includes the following dependencies
 * and source modules:
 * linux/btf_ids.h, linux/find.h, linux/genalloc.h,
 * ext/types.h, ext/internal.h, ext/cid.h, ext/arena.h, ext/idle.h,
 * ext/sub.h, ext/inlines.h, ext/ext.c, ext/cid.c, ext/arena.c,
 * ext/idle.c, ext/sub.c
 */

/* syscalls.c is included here in the original compilation unit. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
