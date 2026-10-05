// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native storage, header primitives and registration for isolation.rs.
 * Unqualified runtime C boundaries, not algorithm-translation coverage.
 * Native compiler/instrumentation policy origin: build_utility.c.
 * No original isolation.c body is included or used as a fallback.
 */
#include "sched_isolation_bindings.h"

#error "SOURCE ONLY HOLD: scheduler isolation is not admitted"

#ifndef CONFIG_CPU_ISOLATION
#error "isolation native primitives require CONFIG_CPU_ISOLATION"
#endif

DEFINE_STATIC_KEY_FALSE(housekeeping_overridden);
EXPORT_SYMBOL_GPL(housekeeping_overridden);

struct housekeeping {
	struct cpumask __rcu *cpumasks[HK_TYPE_MAX];
	unsigned long flags;
};

static struct housekeeping housekeeping;
static __initdata LLIST_HEAD(memblock_freelist);

/*
 * Keep native stack allocation and pointer/array decay for both configured
 * cpumask_var_t representations. Rust decides when each slot is allocated
 * and freed; the adapter only provides their stack lifetime.
 */
struct lupos_isolation_boot_masks {
	cpumask_var_t non_housekeeping_mask;
	cpumask_var_t housekeeping_staging;
};

static int __init housekeeping_nohz_full_setup(char *str)
{
	struct lupos_isolation_boot_masks masks;

	return lupos_isolation_nohz_full_setup(str, &masks);
}
__setup("nohz_full=", housekeeping_nohz_full_setup);

static int __init housekeeping_isolcpus_setup(char *str)
{
	struct lupos_isolation_boot_masks masks;

	return lupos_isolation_isolcpus_setup(str, &masks);
}
__setup("isolcpus=", housekeeping_isolcpus_setup);

void __init housekeeping_init(void)
{
	lupos_isolation_init();
}

static int __init housekeeping_late_init(void)
{
	return lupos_isolation_late_init();
}
pure_initcall(housekeeping_late_init);

EXPORT_SYMBOL_GPL(housekeeping_enabled);
EXPORT_SYMBOL_GPL(housekeeping_cpumask);
EXPORT_SYMBOL_GPL(housekeeping_any_cpu);
EXPORT_SYMBOL_GPL(housekeeping_affine);
EXPORT_SYMBOL_GPL(housekeeping_test_cpu);

unsigned long lupos_isolation_flags(void)
{
	return housekeeping.flags;
}

unsigned long lupos_isolation_flags_read_once(void)
{
	return READ_ONCE(housekeeping.flags);
}

void lupos_isolation_flags_write_once(unsigned long flags)
{
	WRITE_ONCE(housekeeping.flags, flags);
}

void __init lupos_isolation_flags_or(unsigned long flags)
{
	housekeeping.flags |= flags;
}

bool lupos_isolation_overridden(void)
{
	return static_branch_unlikely(&housekeeping_overridden);
}

void lupos_isolation_enable(void)
{
	static_branch_enable(&housekeeping_overridden);
}

struct cpumask *lupos_isolation_mask_dereference(enum hk_type type)
{
	/* Keep conditional evaluation of the Rust predicate inside the macro. */
	return rcu_dereference_all_check(housekeeping.cpumasks[type],
					 lupos_isolation_dereference_check(type));
}

struct cpumask *__init lupos_isolation_mask_init_dereference(enum hk_type type)
{
	return rcu_dereference(housekeeping.cpumasks[type]);
}

void lupos_isolation_mask_assign(enum hk_type type, struct cpumask *mask)
{
	rcu_assign_pointer(housekeeping.cpumasks[type], mask);
}

void __init lupos_isolation_mask_init(enum hk_type type, struct cpumask *mask)
{
	RCU_INIT_POINTER(housekeeping.cpumasks[type], mask);
}

const struct cpumask *lupos_isolation_possible_mask(void)
{
	return cpu_possible_mask;
}

const struct cpumask *lupos_isolation_present_mask(void)
{
	return cpu_present_mask;
}

const struct cpumask *lupos_isolation_online_mask(void)
{
	return cpu_online_mask;
}

unsigned int lupos_isolation_nr_cpu_ids(void)
{
	return nr_cpu_ids;
}

unsigned int __init lupos_isolation_setup_max_cpus(void)
{
	return setup_max_cpus;
}

int lupos_isolation_current_cpu(void)
{
	return smp_processor_id();
}

bool lupos_isolation_at_most_scheduling(void)
{
	return system_state <= SYSTEM_SCHEDULING;
}

bool lupos_isolation_running(void)
{
	return system_state == SYSTEM_RUNNING;
}

#if defined(CONFIG_LOCKDEP) && defined(CONFIG_HOTPLUG_CPU)
int lupos_isolation_cpus_write_held(void)
{
	return lockdep_is_cpus_write_held();
}
#endif

#if defined(CONFIG_LOCKDEP) && defined(CONFIG_CPUSETS)
bool lupos_isolation_cpuset_held(void)
{
	return lockdep_is_cpuset_held();
}
#endif

int lupos_isolation_numa_find_closest(const struct cpumask *mask, int cpu)
{
	return sched_numa_find_closest(mask, cpu);
}

unsigned int lupos_isolation_any_and_distribute(const struct cpumask *mask,
					      const struct cpumask *online)
{
	return cpumask_any_and_distribute(mask, online);
}

bool lupos_isolation_likely_cpu_valid(bool valid)
{
	return likely(valid);
}

void lupos_isolation_andnot(struct cpumask *dst, const struct cpumask *a,
			    const struct cpumask *b)
{
	cpumask_andnot(dst, a, b);
}

bool lupos_isolation_intersects(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_intersects(a, b);
}

bool lupos_isolation_test_cpu(int cpu, const struct cpumask *mask)
{
	return cpumask_test_cpu(cpu, mask);
}

void lupos_isolation_copy(struct cpumask *dst, const struct cpumask *src)
{
	cpumask_copy(dst, src);
}

bool lupos_isolation_empty(const struct cpumask *mask)
{
	return cpumask_empty(mask);
}

bool lupos_isolation_equal(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_equal(a, b);
}

unsigned int lupos_isolation_first_and(const struct cpumask *a,
				      const struct cpumask *b)
{
	return cpumask_first_and(a, b);
}

unsigned int lupos_isolation_first_and_and(const struct cpumask *a,
					  const struct cpumask *b,
					  const struct cpumask *c)
{
	return cpumask_first_and_and(a, b, c);
}

void __init lupos_isolation_set_cpu(int cpu, struct cpumask *mask)
{
	__cpumask_set_cpu(cpu, mask);
}

void __init lupos_isolation_clear_cpu(int cpu, struct cpumask *mask)
{
	__cpumask_clear_cpu(cpu, mask);
}

int __init lupos_isolation_cpulist_parse(const char *str, struct cpumask *mask)
{
	return cpulist_parse(str, mask);
}

/* Separate source allocation sites retain the update/init distinction. */
struct cpumask *lupos_isolation_update_alloc(void)
{
	return kmalloc(cpumask_size(), GFP_KERNEL);
}

struct cpumask *__init lupos_isolation_init_alloc(void)
{
	return kmalloc(cpumask_size(), GFP_KERNEL);
}

struct cpumask *__init lupos_isolation_memblock_alloc(void)
{
	/* Same header expansion, retaining the original diagnostic label. */
	return __memblock_alloc_or_panic(cpumask_size(), SMP_CACHE_BYTES,
					 "housekeeping_setup_type");
}

void lupos_isolation_mask_free(struct cpumask *mask)
{
	kfree(mask);
}

struct cpumask *__init
lupos_isolation_boot_non_alloc(struct lupos_isolation_boot_masks *masks)
{
	alloc_bootmem_cpumask_var(&masks->non_housekeeping_mask);
	return masks->non_housekeeping_mask;
}

struct cpumask *__init
lupos_isolation_boot_staging_alloc(struct lupos_isolation_boot_masks *masks)
{
	alloc_bootmem_cpumask_var(&masks->housekeeping_staging);
	return masks->housekeeping_staging;
}

void __init
lupos_isolation_boot_non_free(struct lupos_isolation_boot_masks *masks)
{
	free_bootmem_cpumask_var(masks->non_housekeeping_mask);
}

void __init
lupos_isolation_boot_staging_free(struct lupos_isolation_boot_masks *masks)
{
	free_bootmem_cpumask_var(masks->housekeeping_staging);
}

void __init lupos_isolation_memblock_queue(struct cpumask *mask)
{
	__llist_add((struct llist_node *)mask, &memblock_freelist);
}

struct llist_node *__init lupos_isolation_memblock_take_all(void)
{
	return __llist_del_all(&memblock_freelist);
}

struct llist_node *__init lupos_isolation_memblock_next(struct llist_node *node)
{
	return node->next;
}

void __init lupos_isolation_memblock_free(struct llist_node *node)
{
	memblock_free(node, cpumask_size());
}

void lupos_isolation_pci_flush(void)
{
	pci_probe_flush_workqueue();
}

void lupos_isolation_memcg_flush(void)
{
	mem_cgroup_flush_workqueue();
}

void lupos_isolation_vmstat_flush(void)
{
	vmstat_flush_workqueue();
}

int lupos_isolation_workqueue_update(const struct cpumask *mask)
{
	return workqueue_unbound_housekeeping_update(mask);
}

int lupos_isolation_timer_update(struct cpumask *mask)
{
	return tmigr_isolated_exclude_cpumask(mask);
}

int lupos_isolation_kthreads_update(void)
{
	return kthreads_update_housekeeping();
}

void __init lupos_isolation_tick_offload_init(void)
{
	sched_tick_offload_init();
}

void __init lupos_isolation_nohz_setup(struct cpumask *mask)
{
	tick_nohz_full_setup(mask);
}

bool __init lupos_isolation_isalpha(char ch)
{
	return isalpha(ch);
}

/* Keep distinct native warning state for each original WARN_ON_ONCE site. */
void lupos_isolation_warn_any_cpu(bool invalid)
{
	WARN_ON_ONCE(invalid);
}

void lupos_isolation_warn_workqueue(bool failed)
{
	WARN_ON_ONCE(failed);
}

void lupos_isolation_warn_timer(bool failed)
{
	WARN_ON_ONCE(failed);
}

void lupos_isolation_warn_kthreads(bool failed)
{
	WARN_ON_ONCE(failed);
}

bool __init lupos_isolation_warn_alloc(bool failed)
{
	return WARN_ON_ONCE(failed);
}

void __init lupos_isolation_warn_empty(bool empty)
{
	WARN_ON_ONCE(empty);
}

void __init lupos_isolation_warn_nohz_unsupported(void)
{
	pr_warn("Housekeeping: nohz unsupported. Build with CONFIG_NO_HZ_FULL\n");
}

void __init lupos_isolation_warn_range(void)
{
	pr_warn("Housekeeping: nohz_full= or isolcpus= incorrect CPU range\n");
}

void __init lupos_isolation_warn_present(int cpu)
{
	pr_warn("Housekeeping: must include one present CPU, using boot CPU:%d\n", cpu);
}

void __init lupos_isolation_warn_mismatch(void)
{
	pr_warn("Housekeeping: nohz_full= must match isolcpus=\n");
}

void __init lupos_isolation_warn_joint_present(const char *str)
{
	pr_warn("Housekeeping: must include one present CPU neither in nohz_full= nor in isolcpus=domain, ignoring setting %s\n", str);
}

void __init lupos_isolation_warn_illegal(int len, const char *str)
{
	pr_warn("isolcpus: Invalid flag %.*s\n", len, str);
}

void __init lupos_isolation_info_unknown(int len, const char *str)
{
	pr_info("isolcpus: Skipped unknown flag %.*s\n", len, str);
}
