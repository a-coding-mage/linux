// SPDX-License-Identifier: GPL-2.0
// Native ABI primitives and registration only; no original C owner fallback.
#include "deadline_native_bindings.h"
#include "deadline_migration_storage.inc"
#define DL_LEAF(ret, name, args, body) ret rust_dl_##name args body
#include "deadline_native_primitives.inc"
#include "deadline_runqueue_primitives.inc"
#include "deadline_migration_primitives.inc"
#include "deadline_lifecycle_primitives.inc"
#undef DL_LEAF
#define DL_WARN_ONCE(name) \
	noinline bool rust_dl_warn_##name(bool condition) \
	{ return WARN_ON_ONCE(condition); }
#define DL_WARN(name) \
	noinline bool rust_dl_warn_##name(bool condition) \
	{ return WARN_ON(condition); }
#include "deadline_warnings.inc"
#include "deadline_runqueue_warnings.inc"
#include "deadline_migration_warnings.inc"
#include "deadline_lifecycle_warnings.inc"
#undef DL_WARN
#undef DL_WARN_ONCE
#ifdef CONFIG_SYSCTL
/* Original ctl_table initializer, with pointers to the Rust-owned scalars. */
static const struct ctl_table sched_dl_sysctls[] = {
	{
		.procname	= "sched_deadline_period_max_us",
		.data		= &rust_dl_period_max,
		.maxlen		= sizeof(unsigned int),
		.mode		= 0644,
		.proc_handler	= proc_douintvec_minmax,
		.extra1		= &rust_dl_period_min,
	},
	{
		.procname	= "sched_deadline_period_min_us",
		.data		= &rust_dl_period_min,
		.maxlen		= sizeof(unsigned int),
		.mode		= 0644,
		.proc_handler	= proc_douintvec_minmax,
		.extra2		= &rust_dl_period_max,
	},
};
void rust_dl_register_sysctl_init(void)
{
	register_sysctl_init("kernel", sched_dl_sysctls);
}
int __init sched_dl_sysctl_init(void);
late_initcall(sched_dl_sysctl_init);
#endif
#include "deadline_class.inc"
