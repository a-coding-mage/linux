/* SPDX-License-Identifier: GPL-2.0 */
/* ABI input only. Original configured headers are the sole layout authority. */
#ifndef RUST_DEADLINE_NATIVE_BINDINGS_H
#define RUST_DEADLINE_NATIVE_BINDINGS_H
#include <linux/cpuset.h>
#include <linux/sched/clock.h>
#include <linux/sched/deadline.h>
#include <linux/sched/isolation.h>
#include <linux/sched/task.h>
#include <linux/math64.h>
#include <linux/hrtimer_api.h>
#include <linux/ktime_api.h>
#include <linux/cpumask_api.h>
#include <linux/spinlock_api.h>
#include <uapi/linux/sched/types.h>
#include "sched.h"
#include "pelt.h"

extern unsigned int rust_dl_period_max;
extern unsigned int rust_dl_period_min;
#define DL_LEAF(ret, name, args, body) ret rust_dl_##name args;
#include "deadline_native_primitives.inc"
#include "deadline_runqueue_primitives.inc"
#include "deadline_migration_primitives.inc"
#include "deadline_lifecycle_primitives.inc"
#undef DL_LEAF
#define DL_WARN_ONCE(name) bool rust_dl_warn_##name(bool condition);
#define DL_WARN(name) bool rust_dl_warn_##name(bool condition);
#include "deadline_warnings.inc"
#include "deadline_runqueue_warnings.inc"
#include "deadline_migration_warnings.inc"
#include "deadline_lifecycle_warnings.inc"
#undef DL_WARN
#undef DL_WARN_ONCE
#ifdef CONFIG_SYSCTL
void rust_dl_register_sysctl_init(void);
#endif
#endif
