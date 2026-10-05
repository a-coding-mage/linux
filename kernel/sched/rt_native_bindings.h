/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_RT_NATIVE_BINDINGS_H
#define LUPOS_RT_NATIVE_BINDINGS_H
/* Source-only native authority. Generate with the actual kernel configuration;
 * no fabricated Rust layouts or numeric kernel constants are supplied here. */
#include <linux/sched/clock.h>
#include <linux/sched/cputime.h>
#include <linux/sched/posix-timers.h>
#include <linux/sched/rt.h>
#include <linux/sched/signal.h>
#include <linux/jiffies.h>
#include <linux/slab.h>
#include <linux/posix-timers.h>
#include <uapi/linux/sched/types.h>
#include "sched.h"
#include "smp.h"
#include "autogroup.h"
#include "stats.h"
#include "pelt.h"

/* Private rt.c:2666 type, copied exactly; headers determine all field types. */
#ifdef CONFIG_RT_GROUP_SCHED
struct rt_schedulable_data {
	struct task_group *tg;
	u64 rt_period;
	u64 rt_runtime;
};
#endif
#ifdef CONFIG_SYSCTL
extern int rust_rt_sysctl_sched_rr_timeslice;
void rust_rt_register_sysctl_init(void);
#endif

#include "rt_native_constants.h"
#include "rt_class_callbacks.h"
#define RT_LEAF(ret, name, args, body) ret rust_rt_##name args;
#include "rt_native_primitives.inc"
#undef RT_LEAF
#define RT_WARN(name, operation) bool rust_rt_##name(bool condition);
#define RT_BUG(name) void rust_rt_##name(bool condition);
#include "rt_native_diagnostics.inc"
#undef RT_WARN
#undef RT_BUG
#endif
