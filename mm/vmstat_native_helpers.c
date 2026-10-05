// SPDX-License-Identifier: GPL-2.0-only
/* Native storage/macro/architecture/header leaves only. No original vmstat.c
 * function body is compiled or forwarded here. All entries are catalogued. */
#include "vmstat_native_includes.h"
#define RVM(ret, name, args, body) ret name args;
#include "vmstat_native_primitives.h"
#undef RVM

/* Native arrays preserve exact object size, alignment, and native section.
 * Wrapping these arrays in an aligned Rust struct would add trailing padding,
 * altering ELF symbol size; do not replace with a guessed aligned facade. */
atomic_long_t vm_zone_stat[NR_VM_ZONE_STAT_ITEMS] __cacheline_aligned_in_smp;
atomic_long_t vm_node_stat[NR_VM_NODE_STAT_ITEMS] __cacheline_aligned_in_smp;
atomic_long_t vm_numa_event[NR_VM_NUMA_EVENT_ITEMS] __cacheline_aligned_in_smp;
EXPORT_SYMBOL(vm_zone_stat);
EXPORT_SYMBOL(vm_node_stat);
#ifdef CONFIG_VM_EVENT_COUNTERS
DEFINE_PER_CPU(struct vm_event_state, vm_event_states) = {{0}};
EXPORT_PER_CPU_SYMBOL(vm_event_states);
EXPORT_SYMBOL_GPL(all_vm_events);
#endif
#if defined(CONFIG_NUMA) && defined(CONFIG_PROC_FS)
static DEFINE_MUTEX(vm_numa_stat_lock);
#endif
#ifdef CONFIG_SMP
extern void rust_vmstat_update(struct work_struct *work);
extern void rust_vmstat_shepherd(struct work_struct *work);
extern int __init rust_vmstat_late_init(void);
static DEFINE_PER_CPU(struct delayed_work, vmstat_work);
static DECLARE_DEFERRABLE_WORK(shepherd, rust_vmstat_shepherd);
late_initcall(rust_vmstat_late_init);
EXPORT_SYMBOL(__mod_zone_page_state);
EXPORT_SYMBOL(__mod_node_page_state);
EXPORT_SYMBOL(__inc_zone_page_state);
EXPORT_SYMBOL(__inc_node_page_state);
EXPORT_SYMBOL(__dec_zone_page_state);
EXPORT_SYMBOL(__dec_node_page_state);
EXPORT_SYMBOL(mod_zone_page_state);
EXPORT_SYMBOL(mod_node_page_state);
EXPORT_SYMBOL(inc_zone_page_state);
EXPORT_SYMBOL(inc_node_page_state);
EXPORT_SYMBOL(dec_zone_page_state);
EXPORT_SYMBOL(dec_node_page_state);
#endif
#if defined(CONFIG_DEBUG_FS) && defined(CONFIG_COMPACTION)
extern int __init rust_vmstat_extfrag_debug_init(void);
module_init(rust_vmstat_extfrag_debug_init);
#endif
#define RVM(ret, name, args, body) ret name args body
#include "vmstat_native_primitives.h"
#undef RVM
