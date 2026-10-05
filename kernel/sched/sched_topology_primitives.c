// SPDX-License-Identifier: GPL-2.0
/* Native storage, header operations and metadata for topology.rs.
 * Baseline 126a30fae3bba11420ec2fcbde51a0a01bab1b5b, topology.c.
 * All native leaves are unqualified runtime C. No topology.c fallback.
 * Original compiler/instrumentation context: build_utility.c. */
#include "sched_topology_bindings.h"
#error "SOURCE ONLY HOLD: scheduler topology is not admitted"

DEFINE_MUTEX(sched_domains_mutex);
static cpumask_var_t sched_domains_llc_id_allocmask;
static cpumask_var_t sched_domains_tmpmask;
static cpumask_var_t sched_domains_tmpmask2;
int max_lid;

static int __init sched_debug_setup(char *str)
{
    return lupos_topology_debug_setup(str);
}
early_param("sched_verbose", sched_debug_setup);

/* Exact native X-macro metadata, including every sd_flags.h entry. */
#define SD_FLAG(_name, mflags) [__##_name] = { .meta_flags = mflags, .name = #_name },
const struct sd_flag_debug sd_flag_debug[] = {
#include <linux/sched/sd_flags.h>
};
#undef SD_FLAG
#define SD_FLAG(name, mflags) (name * !!((mflags) & SDF_NEEDS_GROUPS)) |
static const unsigned int SD_DEGENERATE_GROUPS_MASK =
#include <linux/sched/sd_flags.h>
0;
#undef SD_FLAG

void lupos_topology_domains_lock(void) { mutex_lock(&sched_domains_mutex); }
void lupos_topology_domains_unlock(void) { mutex_unlock(&sched_domains_mutex); }
struct cpumask *lupos_topology_tmpmask(void) { return sched_domains_tmpmask; }
struct cpumask *lupos_topology_tmpmask2(void) { return sched_domains_tmpmask2; }
struct cpumask *lupos_topology_domain_span(struct sched_domain *sd) { return sched_domain_span(sd); }
struct cpumask *lupos_topology_group_span(struct sched_group *sg) { return sched_group_span(sg); }
struct cpumask *lupos_topology_balance_mask(struct sched_group *sg) { return group_balance_mask(sg); }
void lupos_topology_mask_clear(struct cpumask *mask) { cpumask_clear(mask); }
bool lupos_topology_mask_test(int cpu, const struct cpumask *mask) { return cpumask_test_cpu(cpu, mask); }
bool lupos_topology_mask_empty(const struct cpumask *mask) { return cpumask_empty(mask); }
bool lupos_topology_mask_equal(const struct cpumask *a, const struct cpumask *b) { return cpumask_equal(a, b); }
bool lupos_topology_mask_subset(const struct cpumask *a, const struct cpumask *b) { return cpumask_subset(a, b); }
bool lupos_topology_mask_intersects(const struct cpumask *a, const struct cpumask *b) { return cpumask_intersects(a, b); }
void lupos_topology_mask_or(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b) { cpumask_or(dst, a, b); }
unsigned int lupos_topology_mask_weight(const struct cpumask *mask) { return cpumask_weight(mask); }
unsigned int lupos_topology_mask_first(const struct cpumask *mask) { return cpumask_first(mask); }
unsigned int lupos_topology_mask_next(int cpu, const struct cpumask *mask) { return cpumask_next(cpu, mask); }
unsigned int lupos_topology_nr_cpu_ids(void) { return nr_cpu_ids; }
unsigned int lupos_topology_flag_meta(unsigned int index) { return sd_flag_debug[index].meta_flags; }
unsigned int lupos_topology_degenerate_groups_mask(void) { return SD_DEGENERATE_GROUPS_MASK; }
void lupos_topology_debug_domain(struct sched_domain *sd, int level)
{
    printk(KERN_DEBUG "%*s domain-%d: ", level, "", level);
    printk(KERN_CONT "span=%*pbl level=%s\n", cpumask_pr_args(sched_domain_span(sd)), sd->name);
}
void lupos_topology_debug_missing_cpu(int cpu) { printk(KERN_ERR "ERROR: domain->span does not contain CPU%d\n", cpu); }
void lupos_topology_debug_missing_group_cpu(int cpu) { printk(KERN_ERR "ERROR: domain->groups does not contain CPU%d\n", cpu); }
void lupos_topology_debug_flag_child(unsigned int idx) { printk(KERN_ERR "ERROR: flag %s set here but not in child\n", sd_flag_debug[idx].name); }
void lupos_topology_debug_flag_parent(unsigned int idx) { printk(KERN_ERR "ERROR: flag %s set here but not in parent\n", sd_flag_debug[idx].name); }
void lupos_topology_debug_groups(int level) { printk(KERN_DEBUG "%*s groups:", level + 1, ""); }
void lupos_topology_debug_null_group(void) { printk("\n"); printk(KERN_ERR "ERROR: group is NULL\n"); }
void lupos_topology_debug_group(struct sched_group *group) { printk(KERN_CONT " %d:{ span=%*pbl", group->sgc->id, cpumask_pr_args(sched_group_span(group))); }
void lupos_topology_debug_balance_mask(struct sched_group *group) { printk(KERN_CONT " mask=%*pbl", cpumask_pr_args(group_balance_mask(group))); }
void lupos_topology_debug_capacity(unsigned long capacity) { printk(KERN_CONT " cap=%lu", capacity); }
void lupos_topology_debug_attach_null(int cpu) { printk(KERN_DEBUG "CPU%d attaching NULL sched-domain.\n", cpu); }
void lupos_topology_debug_attach(int cpu) { printk(KERN_DEBUG "CPU%d attaching sched-domain(s):\n", cpu); }
const struct cpumask *lupos_topology_active_mask(void) { return cpu_active_mask; }
struct rq *lupos_topology_cpu_rq(int cpu) { return cpu_rq(cpu); }
void lupos_topology_free(const void *ptr) { kfree(ptr); }
void lupos_topology_assert_domains_locked(void) { lockdep_assert_held(&sched_domains_mutex); }
unsigned int lupos_topology_mask_bound(void) { return small_cpumask_bits; }
struct sched_domain *lupos_topology_data_sd(struct s_data *d, int cpu) { return *per_cpu_ptr(d->sd, cpu); }
struct cpumask *lupos_topology_llc_id_allocmask(void) { return sched_domains_llc_id_allocmask; }
bool __init lupos_topology_init_llc_id_allocmask(void) { return zalloc_cpumask_var(&sched_domains_llc_id_allocmask, GFP_KERNEL); }
bool __init lupos_topology_init_tmpmask(void) { return zalloc_cpumask_var(&sched_domains_tmpmask, GFP_KERNEL); }
bool __init lupos_topology_init_tmpmask2(void) { return zalloc_cpumask_var(&sched_domains_tmpmask2, GFP_KERNEL); }
/* Separate original literal printk sites retain indexing/format metadata. */
void lupos_topology_debug_empty_group(void) { printk(KERN_CONT "\n"); printk(KERN_ERR "ERROR: empty group\n"); }
void lupos_topology_debug_repeated_cpu(void) { printk(KERN_CONT "\n"); printk(KERN_ERR "ERROR: repeated CPUs\n"); }
void lupos_topology_debug_group_child_mismatch(void) { printk(KERN_ERR "ERROR: domain->groups does not match domain->child\n"); }
void lupos_topology_debug_group_span_mismatch(void) { printk(KERN_ERR "ERROR: groups don't span domain->span\n"); }
void lupos_topology_debug_parent_span_mismatch(void) { printk(KERN_ERR "ERROR: parent span is not a superset of domain->span\n"); }
void lupos_topology_debug_group_end(void) { printk(KERN_CONT " }"); }
void lupos_topology_debug_group_separator(void) { printk(KERN_CONT ","); }
void lupos_topology_debug_groups_end(void) { printk(KERN_CONT "\n"); }
