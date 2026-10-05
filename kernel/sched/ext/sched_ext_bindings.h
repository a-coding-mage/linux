/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_EXT_BINDINGS_H
#define LUPOS_SCHED_EXT_BINDINGS_H

/* Sole sched_ext bindgen root. Keep the native build_policy.c header prefix
 * and ext header order; do not import C bodies, redefine configuration macros,
 * or manufacture Rust layouts. This declaration input does not reproduce the
 * complete native translation unit or qualify its macro/BTF/CFI behavior.
 */
#include <linux/sched/clock.h>
#include <linux/sched/cputime.h>
#include <linux/sched/hotplug.h>
#include <linux/sched/isolation.h>
#include <linux/sched/posix-timers.h>
#include <linux/sched/rt.h>

#include <linux/cpuidle.h>
#include <linux/jiffies.h>
#include <linux/kobject.h>
#include <linux/livepatch.h>
#include <linux/pm.h>
#include <linux/psi.h>
#include <linux/rhashtable.h>
#include <linux/seq_buf.h>
#include <linux/seqlock_api.h>
#include <linux/slab.h>
#include <linux/suspend.h>
#include <linux/tsacct_kern.h>
#include <linux/vtime.h>
#include <linux/sysrq.h>
#include <linux/percpu-rwsem.h>

#include <uapi/linux/sched/types.h>

#include "../sched.h"
#include "../smp.h"
#include "../autogroup.h"
#include "../stats.h"
#include "../pelt.h"

#ifdef CONFIG_SCHED_CLASS_EXT
#include <linux/btf_ids.h>
#include <linux/find.h>
#include <linux/genalloc.h>
#include "types.h"
#include "internal.h"
#include "cid.h"
#include "arena.h"
#include "idle.h"
#include "sub.h"
#include "inlines.h"

/* Original owner order. Guards make the shared native declarations unique.
 * These remain C API fragments for native consumers, not bindgen roots.
 * Their native CONFIG_* conditions remain authoritative for availability.
 */
#include "sched_ext_core_bindings.h"
#include "sched_ext_cid_bindings.h"
#include "sched_ext_arena_bindings.h"
#include "sched_ext_idle_bindings.h"
#include "sched_ext_sub_bindings.h"
#endif /* CONFIG_SCHED_CLASS_EXT */

#endif /* LUPOS_SCHED_EXT_BINDINGS_H */
