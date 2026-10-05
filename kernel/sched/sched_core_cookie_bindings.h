/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_CORE_COOKIE_BINDINGS_H
#define LUPOS_SCHED_CORE_COOKIE_BINDINGS_H

/* Configured native headers are the sole ABI authority. No generated view is
 * checked in, and this source proposal is not connected to the build. */
#include <linux/build_bug.h>
#include <linux/kernel_stat.h>
#include <linux/math64.h>
#include <linux/pid.h>
#include <linux/ptrace_api.h>
#include <linux/rculist.h>
#include <linux/refcount_api.h>
#include <linux/sched/signal.h>
#include <linux/sched/smt.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <uapi/linux/prctl.h>
#include "sched.h"

#ifdef CONFIG_SCHED_CORE
/* Exact private type from core_sched.c; never a handwritten Rust layout. */
struct sched_core_cookie {
	refcount_t refcnt;
};

/* The original BUILD_BUG_ON checks, retained as native compile-time checks. */
static_assert(PR_SCHED_CORE_SCOPE_THREAD == PIDTYPE_PID);
static_assert(PR_SCHED_CORE_SCOPE_THREAD_GROUP == PIDTYPE_TGID);
static_assert(PR_SCHED_CORE_SCOPE_PROCESS_GROUP == PIDTYPE_PGID);

#define LUPOS_CORE_COOKIE_ENODEV ENODEV
#define LUPOS_CORE_COOKIE_EINVAL EINVAL
#define LUPOS_CORE_COOKIE_ESRCH ESRCH
#define LUPOS_CORE_COOKIE_EPERM EPERM
#define LUPOS_CORE_COOKIE_ENOMEM ENOMEM
#define LUPOS_CORE_COOKIE_DEQUEUE_SAVE DEQUEUE_SAVE
#define LUPOS_CORE_COOKIE_GET PR_SCHED_CORE_GET
#define LUPOS_CORE_COOKIE_CREATE PR_SCHED_CORE_CREATE
#define LUPOS_CORE_COOKIE_SHARE_TO PR_SCHED_CORE_SHARE_TO
#define LUPOS_CORE_COOKIE_SHARE_FROM PR_SCHED_CORE_SHARE_FROM
#define LUPOS_CORE_COOKIE_MAX PR_SCHED_CORE_MAX
#define LUPOS_CORE_COOKIE_PTRACE_MODE PTRACE_MODE_READ_REALCREDS

#define LUPOS_CORE_COOKIE_VALUE(type, name, args, ...) type name args;
#define LUPOS_CORE_COOKIE_VOID(name, args, ...) void name args;
#define LUPOS_CORE_COOKIE_BODY(type, name, args, ...) type name args;
#include "sched_core_cookie_native_leaves.def"
#undef LUPOS_CORE_COOKIE_VALUE
#undef LUPOS_CORE_COOKIE_VOID
#undef LUPOS_CORE_COOKIE_BODY
#endif /* CONFIG_SCHED_CORE */
#endif /* LUPOS_SCHED_CORE_COOKIE_BINDINGS_H */
