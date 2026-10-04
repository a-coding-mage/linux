/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_PAGE_ALLOC_LATE_HELPERS_H
#define LUPOS_PAGE_ALLOC_LATE_HELPERS_H
/* Included after all native includes and shared allocator declarations. */
#define PA_LATE(ret, name, args, ...) ret rust_pa_late_##name args;
#include "page_alloc_late_primitives.inc"
#undef PA_LATE

#define PA_LATE_CONST(x) RUST_PA_##x = x
enum {
 PA_LATE_CONST(ALLOC_NO_CODETAG), PA_LATE_CONST(SHOW_MEM_FILTER_NODES),
 PA_LATE_CONST(MAX_RECLAIM_RETRIES), PA_LATE_CONST(MSEC_PER_SEC),
 PA_LATE_CONST(PF_MEMALLOC), PA_LATE_CONST(PF_EXITING), PA_LATE_CONST(PF_DUMPCORE), PA_LATE_CONST(PF_WQ_WORKER),
 PA_LATE_CONST(SZ_256K), PA_LATE_CONST(SWAP_CLUSTER_MAX), PA_LATE_CONST(MAX_ORDER_NR_PAGES),
 PA_LATE_CONST(EINVAL), PA_LATE_CONST(EINTR), PA_LATE_CONST(ENOMEM), PA_LATE_CONST(EBUSY), PA_LATE_CONST(EAGAIN),
 RUST_PA_LATE_SYSCTL_ZERO_INDEX = (const int *)SYSCTL_ZERO - sysctl_vals,
 RUST_PA_LATE_SYSCTL_ONE_INDEX = (const int *)SYSCTL_ONE - sysctl_vals,
 RUST_PA_LATE_SYSCTL_ONE_HUNDRED_INDEX = (const int *)SYSCTL_ONE_HUNDRED - sysctl_vals,
 RUST_PA_LATE_SYSCTL_THREE_THOUSAND_INDEX = (const int *)SYSCTL_THREE_THOUSAND - sysctl_vals,
#ifdef CONFIG_NUMA
 PA_LATE_CONST(PENALTY_FOR_NODE_WITH_CPUS),
#endif
#ifdef CONFIG_CONTIG_ALLOC
 PA_LATE_CONST(ACR_FLAGS_CMA), PA_LATE_CONST(ACR_FLAGS_NONE), PA_LATE_CONST(MAX_FOLIO_ORDER),
#endif
};
#undef PA_LATE_CONST
static const int RUST_PA_NUMA_NO_NODE = NUMA_NO_NODE;
#define PA_LATE_GFP(x) static const gfp_t RUST_PA_##x = x;
PA_LATE_GFP(__GFP_NOMEMALLOC)
PA_LATE_GFP(__GFP_NOWARN)
PA_LATE_GFP(__GFP_DMA)
PA_LATE_GFP(__GFP_HARDWALL)
PA_LATE_GFP(__GFP_RETRY_MAYFAIL)
PA_LATE_GFP(__GFP_THISNODE)
PA_LATE_GFP(__GFP_NOFAIL)
PA_LATE_GFP(__GFP_NOLOCKDEP)
PA_LATE_GFP(__GFP_FS)
PA_LATE_GFP(__GFP_MEMALLOC)
PA_LATE_GFP(__GFP_NORETRY)
PA_LATE_GFP(__GFP_WRITE)
PA_LATE_GFP(__GFP_ACCOUNT)
PA_LATE_GFP(__GFP_ZERO)
PA_LATE_GFP(__GFP_COMP)
PA_LATE_GFP(__GFP_HIGHMEM)
PA_LATE_GFP(__GFP_IO)
PA_LATE_GFP(__GFP_RECLAIM)
PA_LATE_GFP(__GFP_ZEROTAGS)
PA_LATE_GFP(__GFP_SKIP_ZERO)
PA_LATE_GFP(__GFP_SKIP_KASAN)
PA_LATE_GFP(__GFP_RECLAIMABLE)
PA_LATE_GFP(__GFP_MOVABLE)
PA_LATE_GFP(GFP_KERNEL)
PA_LATE_GFP(GFP_USER)
PA_LATE_GFP(GFP_HIGHUSER_MOVABLE)
PA_LATE_GFP(GFP_ZONEMASK)
#undef PA_LATE_GFP

bool rust_pa_warn_alloc_allowed(gfp_t gfp);
void rust_pa_warn_alloc_finish(gfp_t gfp, const nodemask_t *mask);
bool rust_pa_late_nopage_ratelimit(void);
void rust_pa_late_print_newline(void);
void rust_pa_late_print_stall(gfp_t gfp, const nodemask_t *mask, unsigned int order, unsigned long seconds);
bool rust_pa_late_stall_trylock(void);
void rust_pa_late_stall_unlock(void);
unsigned int rust_pa_late_read_zonelist_seq(void);
bool rust_pa_late_retry_zonelist_seq(unsigned int seq);
unsigned long rust_pa_late_write_zonelist_lock(void);
void rust_pa_late_write_zonelist_unlock(unsigned long flags);
void rust_pa_late_wmark_lock(void);
void rust_pa_late_wmark_unlock(void);
struct per_cpu_pages __percpu *rust_pa_late_boot_pageset(void);
struct per_cpu_zonestat __percpu *rust_pa_late_boot_zonestats(void);
struct per_cpu_pages *rust_pa_late_boot_pageset_cpu(unsigned int cpu);
struct per_cpu_zonestat *rust_pa_late_boot_zonestats_cpu(unsigned int cpu);
void rust_pa_late_print_min_free_unchanged(int value, int user);
bool rust_pa_has_managed_zone(unsigned int index);
#ifdef CONFIG_LOCKDEP
void rust_pa_fs_reclaim_acquire(gfp_t gfp, unsigned long ip);
void rust_pa_fs_reclaim_release(gfp_t gfp, unsigned long ip);
void rust_pa_late_lock_acquire_fs(unsigned long ip);
void rust_pa_late_lock_release_fs(unsigned long ip);
#ifdef CONFIG_MMU_NOTIFIER
void rust_pa_late_mmu_lock_map_acquire(void);
void rust_pa_late_mmu_lock_map_release(void);
#endif
#endif
#ifdef CONFIG_CONTIG_ALLOC
bool rust_pa_late_migrate_debug_enabled(void);
#endif
#ifdef CONFIG_UNACCEPTED_MEMORY
int accept_memory_parse(char *p);
#endif
#endif
