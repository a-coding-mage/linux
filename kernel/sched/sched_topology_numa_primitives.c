// SPDX-License-Identifier: GPL-2.0
/*
 * NUMA storage, configured header primitives and native registration only.
 * Rust retains the existing topology.c NUMA algorithms. These real runtime
 * C boundaries remain unqualified; no original owner body is a fallback.
 */
#include "sched_topology_numa_bindings.h"

#error "SOURCE ONLY HOLD: scheduler topology NUMA is not admitted"

#ifdef CONFIG_NUMA
enum numa_topology_type sched_numa_topology_type;
int sched_max_numa_distance;

/* Private counterparts of topology.c's original NUMA state. */
static int lupos_topology_numa_domains_levels;
static int lupos_topology_numa_nodes_levels;
static int *lupos_topology_numa_domains_distance;
static int *lupos_topology_numa_nodes_distance;
static struct cpumask ***lupos_topology_numa_domains_masks;

int lupos_topology_numa_levels(void)
{
	return lupos_topology_numa_domains_levels;
}
void lupos_topology_numa_levels_set(int levels)
{
	lupos_topology_numa_domains_levels = levels;
}
int lupos_topology_numa_node_levels(void)
{
	return lupos_topology_numa_nodes_levels;
}
void lupos_topology_numa_node_levels_set(int levels)
{
	lupos_topology_numa_nodes_levels = levels;
}
void lupos_topology_numa_node_levels_write_once(int levels)
{
	WRITE_ONCE(lupos_topology_numa_nodes_levels, levels);
}
int lupos_topology_numa_max_distance(void)
{
	return sched_max_numa_distance;
}
void lupos_topology_numa_max_distance_set(int distance)
{
	sched_max_numa_distance = distance;
}
void lupos_topology_numa_max_distance_write_once(int distance)
{
	WRITE_ONCE(sched_max_numa_distance, distance);
}
void lupos_topology_numa_type_direct(void)
{
	sched_numa_topology_type = NUMA_DIRECT;
}
void lupos_topology_numa_type_mesh(void)
{
	sched_numa_topology_type = NUMA_GLUELESS_MESH;
}
void lupos_topology_numa_type_backplane(void)
{
	sched_numa_topology_type = NUMA_BACKPLANE;
}
int lupos_topology_numa_domain_distance(int level)
{
	return lupos_topology_numa_domains_distance[level];
}
int *lupos_topology_numa_domain_distances(void)
{
	return lupos_topology_numa_domains_distance;
}
int *lupos_topology_numa_node_distances(void)
{
	return lupos_topology_numa_nodes_distance;
}
int *lupos_topology_numa_node_distances_dereference(void)
{
	return rcu_dereference(lupos_topology_numa_nodes_distance);
}
void lupos_topology_numa_node_distances_assign(int *distances)
{
	rcu_assign_pointer(lupos_topology_numa_nodes_distance, distances);
}
void lupos_topology_numa_domain_distances_assign(int *distances)
{
	rcu_assign_pointer(lupos_topology_numa_domains_distance, distances);
}
/* Retain the original constant-NULL branch of rcu_assign_pointer(). */
void lupos_topology_numa_node_distances_clear(void)
{
	rcu_assign_pointer(lupos_topology_numa_nodes_distance, NULL);
}
void lupos_topology_numa_domain_distances_clear(void)
{
	rcu_assign_pointer(lupos_topology_numa_domains_distance, NULL);
}
struct cpumask ***lupos_topology_numa_masks(void)
{
	return lupos_topology_numa_domains_masks;
}
struct cpumask ***lupos_topology_numa_masks_dereference(void)
{
	return rcu_dereference(lupos_topology_numa_domains_masks);
}
void lupos_topology_numa_masks_assign(struct cpumask ***masks)
{
	rcu_assign_pointer(lupos_topology_numa_domains_masks, masks);
}
void lupos_topology_numa_masks_clear(void)
{
	rcu_assign_pointer(lupos_topology_numa_domains_masks, NULL);
}

void lupos_topology_numa_rcu_read_lock(void) { rcu_read_lock(); }
void lupos_topology_numa_rcu_read_unlock(void) { rcu_read_unlock(); }
void lupos_topology_numa_synchronize_rcu(void) { synchronize_rcu(); }
unsigned int lupos_topology_numa_nr_node_ids(void) { return nr_node_ids; }
unsigned int lupos_topology_numa_nr_cpu_ids(void) { return nr_cpu_ids; }
unsigned int lupos_topology_numa_max_num_nodes(void) { return MAX_NUMNODES; }
/* Match for_each_node_state, including its unconditional single-node case. */
unsigned int lupos_topology_numa_next_cpu_node(int previous)
{
#if MAX_NUMNODES > 1
	return next_node(previous, node_states[N_CPU]);
#else
	return previous < 0 ? 0 : MAX_NUMNODES;
#endif
}
unsigned int lupos_topology_numa_next_possible_node(int previous)
{
#if MAX_NUMNODES > 1
	return next_node(previous, node_states[N_POSSIBLE]);
#else
	return previous < 0 ? 0 : MAX_NUMNODES;
#endif
}
bool lupos_topology_numa_node_has_cpu(int node) { return node_state(node, N_CPU); }
int lupos_topology_numa_cpu_to_node(int cpu) { return cpu_to_node(cpu); }
int lupos_topology_numa_nearest_cpu_node(int node)
{
	return numa_nearest_node(node, N_CPU);
}

/* Keep native weak-alias identity: Rust function-address equality is not used. */
int lupos_topology_numa_node_distance(int from, int to)
{
	return node_distance(from, to);
}
int arch_sched_node_distance(int from, int to)
	__weak __alias(lupos_topology_numa_node_distance);
bool lupos_topology_numa_modified_distance(void)
{
	return lupos_topology_numa_node_distance != arch_sched_node_distance;
}
int lupos_topology_numa_sd_flags(void) { return SD_NUMA; }

unsigned long *lupos_topology_numa_bitmap_alloc(void)
{
	return bitmap_alloc(LUPOS_TOPOLOGY_NUMA_NR_DISTANCE_VALUES, GFP_KERNEL);
}
void lupos_topology_numa_bitmap_free(unsigned long *map) { bitmap_free(map); }
void lupos_topology_numa_bitmap_zero(unsigned long *map)
{
	bitmap_zero(map, LUPOS_TOPOLOGY_NUMA_NR_DISTANCE_VALUES);
}
void lupos_topology_numa_bitmap_set(unsigned long *map, int distance)
{
	bitmap_set(map, distance, 1);
}
unsigned int lupos_topology_numa_bitmap_weight(const unsigned long *map)
{
	return bitmap_weight(map, LUPOS_TOPOLOGY_NUMA_NR_DISTANCE_VALUES);
}
unsigned long lupos_topology_numa_find_next_bit(const unsigned long *map,
					     unsigned long start)
{
	return find_next_bit(map, LUPOS_TOPOLOGY_NUMA_NR_DISTANCE_VALUES, start);
}
int *lupos_topology_numa_distances_alloc(int levels)
{
	return kzalloc_objs(int, levels);
}
struct cpumask ***lupos_topology_numa_mask_levels_alloc(int levels)
{
	return kzalloc(sizeof(void *) * levels, GFP_KERNEL);
}
struct cpumask **lupos_topology_numa_mask_nodes_alloc(void)
{
	return kzalloc(nr_node_ids * sizeof(void *), GFP_KERNEL);
}
struct cpumask *lupos_topology_numa_mask_alloc(void)
{
	/* Match the original dynamic mask; CONFIG_CPUMASK_OFFSTACK changes no slot. */
	return kzalloc(cpumask_size(), GFP_KERNEL);
}
struct sched_domain_topology_level *lupos_topology_numa_topology_alloc(int count)
{
	return kzalloc(count * sizeof(struct sched_domain_topology_level), GFP_KERNEL);
}
void lupos_topology_numa_free(const void *ptr) { kfree(ptr); }
void lupos_topology_numa_init_node(struct sched_domain_topology_level *tl,
				 sched_domain_mask_f mask)
{
	*tl = SDTL_INIT(mask, NULL, NODE);
}
void lupos_topology_numa_init_level(struct sched_domain_topology_level *tl,
				  sched_domain_mask_f mask,
				  sched_domain_flags_f flags)
{
	*tl = SDTL_INIT(mask, flags, NUMA);
}

const struct cpumask *lupos_topology_numa_node_mask(int node)
{
	return cpumask_of_node(node);
}
const struct cpumask *lupos_topology_numa_online_mask(void) { return cpu_online_mask; }
unsigned int lupos_topology_numa_mask_weight(const struct cpumask *mask)
{
	return cpumask_weight(mask);
}
unsigned int lupos_topology_numa_mask_weight_and(const struct cpumask *a,
					      const struct cpumask *b)
{
	return cpumask_weight_and(a, b);
}
void lupos_topology_numa_mask_or(struct cpumask *dst, const struct cpumask *a,
				const struct cpumask *b)
{
	cpumask_or(dst, a, b);
}
void lupos_topology_numa_mask_set_cpu(unsigned int cpu, struct cpumask *mask)
{
	cpumask_set_cpu(cpu, mask);
}
void lupos_topology_numa_mask_clear_cpu(unsigned int cpu, struct cpumask *mask)
{
	cpumask_clear_cpu(cpu, mask);
}
unsigned int lupos_topology_numa_mask_any_and_distribute(const struct cpumask *a,
						      const struct cpumask *b)
{
	return cpumask_any_and_distribute(a, b);
}
unsigned int lupos_topology_numa_mask_nth_and(unsigned int cpu,
					    const struct cpumask *a,
					    const struct cpumask *b)
{
	return cpumask_nth_and(cpu, a, b);
}
unsigned int lupos_topology_numa_mask_nth_and_andnot(unsigned int cpu,
						   const struct cpumask *a,
						   const struct cpumask *b,
						   const struct cpumask *c)
{
	return cpumask_nth_and_andnot(cpu, a, b, c);
}
struct cpumask ***lupos_topology_numa_bsearch(const void *key,
					  struct cpumask ***masks,
					  int levels, cmp_func_t cmp)
{
	/* cmp alone interprets the live private Rust key; bsearch retains neither. */
	return bsearch(key, masks, levels, sizeof(masks[0]), cmp);
}
const struct cpumask *lupos_topology_numa_error_mask(int error)
{
	return ERR_PTR(error);
}
void lupos_topology_numa_warn_begin(const char *message)
{
	printk(KERN_WARNING "ERROR: %s\n\n", message);
}
void lupos_topology_numa_warn_row_begin(void) { printk(KERN_WARNING "  "); }
void lupos_topology_numa_warn_distance(int distance, bool has_cpus)
{
	if (!has_cpus)
		printk(KERN_CONT "(%02d) ", distance);
	else
		printk(KERN_CONT " %02d  ", distance);
}
void lupos_topology_numa_warn_row_end(void) { printk(KERN_CONT "\n"); }
void lupos_topology_numa_warn_end(void) { printk(KERN_WARNING "\n"); }
void lupos_topology_numa_warn_type(void)
{
	pr_err("Failed to find a NUMA topology type, defaulting to DIRECT\n");
}

/* Registrations remain native; both exported algorithms are Rust definitions. */
EXPORT_SYMBOL_GPL(sched_numa_find_nth_cpu);
EXPORT_SYMBOL_GPL(sched_numa_hop_mask);
#endif /* CONFIG_NUMA */
