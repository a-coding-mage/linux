/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_NUMA_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_NUMA_BINDINGS_H

/* Configured native headers are the type, constant and layout authority. */
#include <linux/bitmap.h>
#include <linux/bsearch.h>
#include <linux/err.h>
#include <linux/numa.h>
#include <linux/slab.h>
#include "sched.h"

#ifdef CONFIG_NUMA
/* All pointer arguments use configured native layouts; no Rust mirror is valid. */
#define LUPOS_TOPOLOGY_NUMA_NR_DISTANCE_VALUES (1 << DISTANCE_BITS)
#define LUPOS_TOPOLOGY_NUMA_LOCAL_DISTANCE LOCAL_DISTANCE
#define LUPOS_TOPOLOGY_NUMA_NO_NODE NUMA_NO_NODE
#define LUPOS_TOPOLOGY_NUMA_ENOMEM ENOMEM
#define LUPOS_TOPOLOGY_NUMA_EINVAL EINVAL
#define LUPOS_TOPOLOGY_NUMA_EBUSY EBUSY

/* The composer owns these shared topology-pointer slots and their storage. */
struct sched_domain_topology_level *lupos_topology_numa_get_topology(void);
struct sched_domain_topology_level *lupos_topology_numa_get_saved_topology(void);
void lupos_topology_numa_set_topology(struct sched_domain_topology_level *tl);
void lupos_topology_numa_set_saved_topology(struct sched_domain_topology_level *tl);

int lupos_topology_numa_levels(void);
void lupos_topology_numa_levels_set(int levels);
int lupos_topology_numa_node_levels(void);
void lupos_topology_numa_node_levels_set(int levels);
void lupos_topology_numa_node_levels_write_once(int levels);
int lupos_topology_numa_max_distance(void);
void lupos_topology_numa_max_distance_set(int distance);
void lupos_topology_numa_max_distance_write_once(int distance);
void lupos_topology_numa_type_direct(void);
void lupos_topology_numa_type_mesh(void);
void lupos_topology_numa_type_backplane(void);
int lupos_topology_numa_domain_distance(int level);
int *lupos_topology_numa_domain_distances(void);
int *lupos_topology_numa_node_distances(void);
int *lupos_topology_numa_node_distances_dereference(void);
void lupos_topology_numa_node_distances_assign(int *distances);
void lupos_topology_numa_domain_distances_assign(int *distances);
void lupos_topology_numa_node_distances_clear(void);
void lupos_topology_numa_domain_distances_clear(void);
/* Readers borrow arrays under RCU; serialized writers own publication/reset. */
struct cpumask ***lupos_topology_numa_masks(void);
struct cpumask ***lupos_topology_numa_masks_dereference(void);
void lupos_topology_numa_masks_assign(struct cpumask ***masks);
void lupos_topology_numa_masks_clear(void);

void lupos_topology_numa_rcu_read_lock(void);
void lupos_topology_numa_rcu_read_unlock(void);
void lupos_topology_numa_synchronize_rcu(void);
unsigned int lupos_topology_numa_nr_node_ids(void);
unsigned int lupos_topology_numa_nr_cpu_ids(void);
unsigned int lupos_topology_numa_max_num_nodes(void);
/* previous is -1 initially, then the last returned node; MAX_NUMNODES ends it. */
unsigned int lupos_topology_numa_next_cpu_node(int previous);
unsigned int lupos_topology_numa_next_possible_node(int previous);
bool lupos_topology_numa_node_has_cpu(int node);
int lupos_topology_numa_cpu_to_node(int cpu);
int lupos_topology_numa_nearest_cpu_node(int node);
int lupos_topology_numa_node_distance(int from, int to);
int arch_sched_node_distance(int from, int to);
bool lupos_topology_numa_modified_distance(void);
int lupos_topology_numa_sd_flags(void);

unsigned long *lupos_topology_numa_bitmap_alloc(void);
void lupos_topology_numa_bitmap_free(unsigned long *map);
void lupos_topology_numa_bitmap_zero(unsigned long *map);
void lupos_topology_numa_bitmap_set(unsigned long *map, int distance);
unsigned int lupos_topology_numa_bitmap_weight(const unsigned long *map);
unsigned long lupos_topology_numa_find_next_bit(const unsigned long *map,
					     unsigned long start);
int *lupos_topology_numa_distances_alloc(int levels);
struct cpumask ***lupos_topology_numa_mask_levels_alloc(int levels);
struct cpumask **lupos_topology_numa_mask_nodes_alloc(void);
/* A dynamic cpumask_size() allocation, never a by-value cpumask_var_t slot. */
struct cpumask *lupos_topology_numa_mask_alloc(void);
struct sched_domain_topology_level *lupos_topology_numa_topology_alloc(int count);
void lupos_topology_numa_free(const void *ptr);
void lupos_topology_numa_init_node(struct sched_domain_topology_level *tl,
				 sched_domain_mask_f mask);
void lupos_topology_numa_init_level(struct sched_domain_topology_level *tl,
				  sched_domain_mask_f mask,
				  sched_domain_flags_f flags);

const struct cpumask *lupos_topology_numa_node_mask(int node);
const struct cpumask *lupos_topology_numa_online_mask(void);
unsigned int lupos_topology_numa_mask_weight(const struct cpumask *mask);
unsigned int lupos_topology_numa_mask_weight_and(const struct cpumask *a,
					      const struct cpumask *b);
void lupos_topology_numa_mask_or(struct cpumask *dst, const struct cpumask *a,
				const struct cpumask *b);
void lupos_topology_numa_mask_set_cpu(unsigned int cpu, struct cpumask *mask);
void lupos_topology_numa_mask_clear_cpu(unsigned int cpu, struct cpumask *mask);
unsigned int lupos_topology_numa_mask_any_and_distribute(const struct cpumask *a,
						      const struct cpumask *b);
unsigned int lupos_topology_numa_mask_nth_and(unsigned int cpu,
					    const struct cpumask *a,
					    const struct cpumask *b);
unsigned int lupos_topology_numa_mask_nth_and_andnot(unsigned int cpu,
						   const struct cpumask *a,
						   const struct cpumask *b,
						   const struct cpumask *c);
struct cpumask ***lupos_topology_numa_bsearch(const void *key,
					  struct cpumask ***masks,
					  int levels, cmp_func_t cmp);
const struct cpumask *lupos_topology_numa_error_mask(int error);
void lupos_topology_numa_warn_begin(const char *message);
void lupos_topology_numa_warn_row_begin(void);
void lupos_topology_numa_warn_distance(int distance, bool has_cpus);
void lupos_topology_numa_warn_row_end(void);
void lupos_topology_numa_warn_end(void);
void lupos_topology_numa_warn_type(void);
#endif /* CONFIG_NUMA */
#endif /* LUPOS_SCHED_TOPOLOGY_NUMA_BINDINGS_H */
