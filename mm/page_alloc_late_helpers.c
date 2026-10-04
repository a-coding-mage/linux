/* SPDX-License-Identifier: GPL-2.0-only */
/* Included by page_alloc_helpers.c: native ABI/compiler/metadata leaves only. */
#include "page_alloc_late_helpers.h"
#define PA_LATE(ret, name, args, ...) ret rust_pa_late_##name args __VA_ARGS__
#include "page_alloc_late_primitives.inc"
#undef PA_LATE

static DEFINE_RATELIMIT_STATE(nopage_rs, 10 * HZ, 1);
static DEFINE_SPINLOCK(alloc_stall_lock);
static DEFINE_SPINLOCK(wmark_lock);
static DEFINE_SEQLOCK(zonelist_update_seq);
static DEFINE_PER_CPU(struct per_cpu_pages, boot_pageset);
static DEFINE_PER_CPU(struct per_cpu_zonestat, boot_zonestats);

bool rust_pa_late_nopage_ratelimit(void) { return __ratelimit(&nopage_rs); }
bool rust_pa_late_stall_trylock(void) { return spin_trylock(&alloc_stall_lock); }
void rust_pa_late_stall_unlock(void) { spin_unlock(&alloc_stall_lock); }
void rust_pa_late_wmark_lock(void) { spin_lock(&wmark_lock); }
void rust_pa_late_wmark_unlock(void) { spin_unlock(&wmark_lock); }
unsigned int rust_pa_late_read_zonelist_seq(void) { return read_seqbegin(&zonelist_update_seq); }
bool rust_pa_late_retry_zonelist_seq(unsigned int seq) { return read_seqretry(&zonelist_update_seq, seq); }
unsigned long rust_pa_late_write_zonelist_lock(void) { unsigned long flags; write_seqlock_irqsave(&zonelist_update_seq, flags); return flags; }
void rust_pa_late_write_zonelist_unlock(unsigned long flags) { write_sequnlock_irqrestore(&zonelist_update_seq, flags); }
struct per_cpu_pages __percpu *rust_pa_late_boot_pageset(void) { return &boot_pageset; }
struct per_cpu_zonestat __percpu *rust_pa_late_boot_zonestats(void) { return &boot_zonestats; }
struct per_cpu_pages *rust_pa_late_boot_pageset_cpu(unsigned int cpu) { return &per_cpu(boot_pageset, cpu); }
struct per_cpu_zonestat *rust_pa_late_boot_zonestats_cpu(unsigned int cpu) { return &per_cpu(boot_zonestats, cpu); }
void rust_pa_late_print_newline(void) { pr_cont("\n"); }
void rust_pa_late_print_stall(gfp_t gfp, const nodemask_t *mask, unsigned int order, unsigned long seconds)
{
 pr_warn("%s: page allocation stall for %lu secs: order:%d, mode:%#x(%pGg) nodemask=%*pbl",
  current->comm, seconds, order, gfp, &gfp, nodemask_pr_args(mask));
}
void warn_alloc(gfp_t gfp, const nodemask_t *mask, const char *fmt, ...)
{
 struct va_format vaf;
 va_list args;
 if (!rust_pa_warn_alloc_allowed(gfp))
  return;
 va_start(args, fmt);
 vaf.fmt = fmt;
 vaf.va = &args;
 pr_warn("%s: %pV, mode:%#x(%pGg), nodemask=%*pbl",
  current->comm, &vaf, gfp, &gfp, nodemask_pr_args(mask));
 va_end(args);
 rust_pa_warn_alloc_finish(gfp, mask);
}
void rust_pa_late_print_min_free_unchanged(int value, int user)
{
 pr_warn_ratelimited("min_free_kbytes is not updated to %d because user defined value %d is preferred\n", value, user);
}
#ifdef CONFIG_LOCKDEP
static struct lockdep_map __fs_reclaim_map = STATIC_LOCKDEP_MAP_INIT("fs_reclaim", &__fs_reclaim_map);
void rust_pa_late_lock_acquire_fs(unsigned long ip) { lock_acquire_exclusive(&__fs_reclaim_map, 0, 0, NULL, ip); }
void rust_pa_late_lock_release_fs(unsigned long ip) { lock_release(&__fs_reclaim_map, ip); }
#ifdef CONFIG_MMU_NOTIFIER
void rust_pa_late_mmu_lock_map_acquire(void) { lock_map_acquire(&__mmu_notifier_invalidate_range_start_map); }
void rust_pa_late_mmu_lock_map_release(void) { lock_map_release(&__mmu_notifier_invalidate_range_start_map); }
#endif
void fs_reclaim_acquire(gfp_t gfp) { rust_pa_fs_reclaim_acquire(gfp, _RET_IP_); }
void fs_reclaim_release(gfp_t gfp) { rust_pa_fs_reclaim_release(gfp, _RET_IP_); }
EXPORT_SYMBOL_GPL(fs_reclaim_acquire);
EXPORT_SYMBOL_GPL(fs_reclaim_release);
#endif
bool has_managed_zone(enum zone_type index) { return rust_pa_has_managed_zone(index); }
#ifdef CONFIG_CONTIG_ALLOC
bool rust_pa_late_migrate_debug_enabled(void)
{
 DEFINE_DYNAMIC_DEBUG_METADATA(descriptor, "migrate failure");
 return DYNAMIC_DEBUG_BRANCH(descriptor);
}
#endif

postcore_initcall(init_per_zone_wmark_min);
#ifdef CONFIG_UNACCEPTED_MEMORY
early_param("accept_memory", accept_memory_parse);
#endif
EXPORT_SYMBOL_GPL(alloc_pages_bulk_noprof);
EXPORT_SYMBOL(__alloc_frozen_pages_noprof);
EXPORT_SYMBOL(alloc_pages_node_noprof);
EXPORT_SYMBOL(__folio_alloc_noprof);
EXPORT_SYMBOL(get_free_pages_noprof);
EXPORT_SYMBOL(get_zeroed_page_noprof);
EXPORT_SYMBOL(__free_pages);
EXPORT_SYMBOL(free_pages);
EXPORT_SYMBOL(alloc_pages_exact_noprof);
EXPORT_SYMBOL(free_pages_exact);
EXPORT_SYMBOL_GPL(nr_free_buffer_pages);
EXPORT_SYMBOL(adjust_managed_page_count);
EXPORT_SYMBOL(free_reserved_pages);
#ifdef CONFIG_CONTIG_ALLOC
EXPORT_SYMBOL(alloc_contig_frozen_range_noprof);
EXPORT_SYMBOL(alloc_contig_range_noprof);
EXPORT_SYMBOL(alloc_contig_frozen_pages_noprof);
EXPORT_SYMBOL(alloc_contig_pages_noprof);
EXPORT_SYMBOL(free_contig_frozen_range);
EXPORT_SYMBOL(free_contig_range);
#endif
EXPORT_SYMBOL(is_free_buddy_page);
EXPORT_SYMBOL_GPL(alloc_pages_nolock_noprof);
