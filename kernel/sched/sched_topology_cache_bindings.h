/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_CACHE_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_CACHE_BINDINGS_H
#ifdef CONFIG_SCHED_CACHE
extern int sysctl_sched_cache_user;
struct cacheinfo *lupos_topology_cache_info(int cpu);
u64 lupos_topology_cache_div_u64(u64 dividend, u32 divisor);
unsigned int *lupos_topology_cache_counts_alloc(int cpu);
struct sched_domain *lupos_topology_cache_parent(struct sched_domain *sd);
void lupos_topology_cache_assert_cpus_held(void);
void lupos_topology_cache_cpus_read_lock(void);
void lupos_topology_cache_cpus_read_unlock(void);
bool lupos_topology_cache_present(void);
void lupos_topology_cache_active_disable(void);
void lupos_topology_cache_active_enable(void);
void lupos_topology_cache_present_disable(void);
void lupos_topology_cache_present_enable(void);
void lupos_topology_cache_debug_unsupported(void);
void lupos_topology_cache_debug_enable(void);
void lupos_topology_cache_debug_disable(void);
struct sched_domain *lupos_topology_cache_sd_llc_dereference(unsigned int cpu);
struct sched_domain *lupos_topology_cache_rq_sd_dereference(unsigned int cpu);
#endif
#endif
