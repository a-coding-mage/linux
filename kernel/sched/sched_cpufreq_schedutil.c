// SPDX-License-Identifier: GPL-2.0
/* Native storage, configured inlines/macros, and metadata only. No C governor
 * implementation inclusion or fallback. These leaves are runtime boundaries,
 * not evidence of compiled, sanitizer/CFI or concurrency qualification.
 */
#error "SOURCE ONLY HOLD: schedutil native boundary is unqualified"
#include "sched_cpufreq_bindings.h"

#ifdef CONFIG_CPU_FREQ_GOV_SCHEDUTIL
static DEFINE_PER_CPU(struct sugov_cpu, sugov_cpu);
static struct sugov_tunables *global_tunables;
static DEFINE_MUTEX(global_tunables_lock);
#include "sched_cpufreq_metadata.inc"
#define SUGOV_LEAF(ret, name, args, ...) ret lupos_sugov_##name args __VA_ARGS__
#include "sched_cpufreq_leaves.inc"
#undef SUGOV_LEAF
#endif /* CONFIG_CPU_FREQ_GOV_SCHEDUTIL */
