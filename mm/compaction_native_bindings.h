/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_COMPACTION_NATIVE_BINDINGS_H
#define RUST_COMPACTION_NATIVE_BINDINGS_H
#include "compaction_native_includes.h"
#if defined(CONFIG_COMPACTION) || defined(CONFIG_CMA)
/* Explicit scalar views preserve native values while avoiding bindgen's
 * expression-inference differences. No macro is declared as extern storage. */
#define RC_UINT(n) static const unsigned int RUST_COMPACTION_##n = (n)
#define RC_ULONG(n) static const unsigned long RUST_COMPACTION_##n = (n)
#define RC_INT(n) static const int RUST_COMPACTION_##n = (n)
#define RC_GFP(n) static const gfp_t RUST_COMPACTION_##n = (n)
RC_UINT(NR_PAGE_ORDERS);
RC_UINT(MAX_PAGE_ORDER);
RC_UINT(PAGE_ALLOC_COSTLY_ORDER);
RC_UINT(BITS_PER_LONG);
RC_UINT(ALLOC_DEFAULT);
RC_UINT(ALLOC_CMA);
RC_UINT(ALLOC_CPUSET);
RC_UINT(ALLOC_WMARK_MASK);
RC_UINT(ALLOC_WMARK_MIN);
RC_UINT(ALLOC_WMARK_HIGH);
RC_ULONG(COMPACT_CLUSTER_MAX);
RC_INT(EAGAIN);
RC_INT(EINTR);
RC_INT(EBUSY);
RC_INT(ENOMEM);
RC_INT(EINVAL);
RC_GFP(__GFP_MOVABLE);
RC_GFP(__GFP_FS);
RC_GFP(GFP_KERNEL);
static const isolate_mode_t RUST_COMPACTION_ISOLATE_UNEVICTABLE = ISOLATE_UNEVICTABLE;
static const isolate_mode_t RUST_COMPACTION_ISOLATE_ASYNC_MIGRATE = ISOLATE_ASYNC_MIGRATE;
#ifdef CONFIG_SPARSEMEM
RC_ULONG(PAGES_PER_SECTION);
#endif
#ifdef CONFIG_COMPACTION
RC_INT(CONFIG_COMPACT_UNEVICTABLE_DEFAULT);
RC_INT(MAX_NUMNODES);
static const long RUST_COMPACTION_MAX_SCHEDULE_TIMEOUT = MAX_SCHEDULE_TIMEOUT;
/* Derived from the actual SYSCTL_* pointer macros, not duplicated numbers. */
enum {
 RUST_COMPACTION_SYSCTL_ZERO_INDEX = (const int *)SYSCTL_ZERO - sysctl_vals,
 RUST_COMPACTION_SYSCTL_ONE_INDEX = (const int *)SYSCTL_ONE - sysctl_vals,
 RUST_COMPACTION_SYSCTL_ONE_HUNDRED_INDEX = (const int *)SYSCTL_ONE_HUNDRED - sysctl_vals,
 RUST_COMPACTION_SYSCTL_ONE_THOUSAND_INDEX = (const int *)SYSCTL_ONE_THOUSAND - sysctl_vals,
};
#if defined(CONFIG_CC_HAS_SANE_FUNCTION_ALIGNMENT) || CONFIG_FUNCTION_ALIGNMENT == 0
static const bool RUST_COMPACTION_CFG_INIT_COLD = true;
#endif
/* The companion source-only recipe decodes this exact configured expansion,
 * accepts the original empty or .data..read_mostly spelling, fails otherwise. */
#define RUST_COMPACTION_NATIVE_READ_MOSTLY __stringify(__read_mostly)
#endif
#undef RC_UINT
#undef RC_ULONG
#undef RC_INT
#undef RC_GFP
#include "compaction_native_primitives.h"
#endif
#endif
