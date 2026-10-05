/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_ISOLATION_BINDINGS_H
#define LUPOS_SCHED_ISOLATION_BINDINGS_H

/* Configured native headers are the only type, layout and constant authority. */
#include <linux/sched/isolation.h>
#include <linux/cpuhplock.h>
#include <linux/llist.h>
#include <linux/pci.h>
#include <linux/slab.h>
#include <linux/string.h>
#include "sched.h"

#define LUPOS_ISOLATION_HK_FLAG_DOMAIN_BOOT BIT(HK_TYPE_DOMAIN_BOOT)
#define LUPOS_ISOLATION_HK_FLAG_DOMAIN BIT(HK_TYPE_DOMAIN)
#define LUPOS_ISOLATION_HK_FLAG_MANAGED_IRQ BIT(HK_TYPE_MANAGED_IRQ)
#define LUPOS_ISOLATION_HK_FLAG_KERNEL_NOISE BIT(HK_TYPE_KERNEL_NOISE)
#define LUPOS_ISOLATION_ENOMEM ENOMEM
#define LUPOS_ISOLATION_EINVAL EINVAL

struct lupos_isolation_boot_masks;

unsigned long lupos_isolation_flags(void);
unsigned long lupos_isolation_flags_read_once(void);
void lupos_isolation_flags_write_once(unsigned long flags);
void __init lupos_isolation_flags_or(unsigned long flags);
bool lupos_isolation_overridden(void);
void lupos_isolation_enable(void);
struct cpumask *lupos_isolation_mask_dereference(enum hk_type type);
struct cpumask *__init lupos_isolation_mask_init_dereference(enum hk_type type);
void lupos_isolation_mask_assign(enum hk_type type, struct cpumask *mask);
void __init lupos_isolation_mask_init(enum hk_type type, struct cpumask *mask);

const struct cpumask *lupos_isolation_possible_mask(void);
const struct cpumask *lupos_isolation_present_mask(void);
const struct cpumask *lupos_isolation_online_mask(void);
unsigned int lupos_isolation_nr_cpu_ids(void);
unsigned int __init lupos_isolation_setup_max_cpus(void);
int lupos_isolation_current_cpu(void);
bool lupos_isolation_at_most_scheduling(void);
bool lupos_isolation_running(void);
#if defined(CONFIG_LOCKDEP) && defined(CONFIG_HOTPLUG_CPU)
int lupos_isolation_cpus_write_held(void);
#endif
#if defined(CONFIG_LOCKDEP) && defined(CONFIG_CPUSETS)
bool lupos_isolation_cpuset_held(void);
#endif
int lupos_isolation_numa_find_closest(const struct cpumask *mask, int cpu);
unsigned int lupos_isolation_any_and_distribute(const struct cpumask *mask,
					      const struct cpumask *online);
bool lupos_isolation_likely_cpu_valid(bool valid);
void lupos_isolation_andnot(struct cpumask *dst, const struct cpumask *a,
			    const struct cpumask *b);
bool lupos_isolation_intersects(const struct cpumask *a, const struct cpumask *b);
bool lupos_isolation_test_cpu(int cpu, const struct cpumask *mask);
void lupos_isolation_copy(struct cpumask *dst, const struct cpumask *src);
bool lupos_isolation_empty(const struct cpumask *mask);
bool lupos_isolation_equal(const struct cpumask *a, const struct cpumask *b);
unsigned int lupos_isolation_first_and(const struct cpumask *a,
				      const struct cpumask *b);
unsigned int lupos_isolation_first_and_and(const struct cpumask *a,
					  const struct cpumask *b,
					  const struct cpumask *c);
void __init lupos_isolation_set_cpu(int cpu, struct cpumask *mask);
void __init lupos_isolation_clear_cpu(int cpu, struct cpumask *mask);
int __init lupos_isolation_cpulist_parse(const char *str, struct cpumask *mask);

struct cpumask *lupos_isolation_update_alloc(void);
struct cpumask *__init lupos_isolation_init_alloc(void);
struct cpumask *__init lupos_isolation_memblock_alloc(void);
void lupos_isolation_mask_free(struct cpumask *mask);
struct cpumask *__init
lupos_isolation_boot_non_alloc(struct lupos_isolation_boot_masks *masks);
struct cpumask *__init
lupos_isolation_boot_staging_alloc(struct lupos_isolation_boot_masks *masks);
void __init
lupos_isolation_boot_non_free(struct lupos_isolation_boot_masks *masks);
void __init
lupos_isolation_boot_staging_free(struct lupos_isolation_boot_masks *masks);
void __init lupos_isolation_memblock_queue(struct cpumask *mask);
struct llist_node *__init lupos_isolation_memblock_take_all(void);
struct llist_node *__init lupos_isolation_memblock_next(struct llist_node *node);
void __init lupos_isolation_memblock_free(struct llist_node *node);

void lupos_isolation_pci_flush(void);
void lupos_isolation_memcg_flush(void);
void lupos_isolation_vmstat_flush(void);
int lupos_isolation_workqueue_update(const struct cpumask *mask);
int lupos_isolation_timer_update(struct cpumask *mask);
int lupos_isolation_kthreads_update(void);
void __init lupos_isolation_tick_offload_init(void);
void __init lupos_isolation_nohz_setup(struct cpumask *mask);
bool __init lupos_isolation_isalpha(char ch);

void lupos_isolation_warn_any_cpu(bool invalid);
void lupos_isolation_warn_workqueue(bool failed);
void lupos_isolation_warn_timer(bool failed);
void lupos_isolation_warn_kthreads(bool failed);
bool __init lupos_isolation_warn_alloc(bool failed);
void __init lupos_isolation_warn_empty(bool empty);
void __init lupos_isolation_warn_nohz_unsupported(void);
void __init lupos_isolation_warn_range(void);
void __init lupos_isolation_warn_present(int cpu);
void __init lupos_isolation_warn_mismatch(void);
void __init lupos_isolation_warn_joint_present(const char *str);
void __init lupos_isolation_warn_illegal(int len, const char *str);
void __init lupos_isolation_info_unknown(int len, const char *str);

/* Rust definitions called by the native RCU and boot-registration adapters. */
bool lupos_isolation_dereference_check(enum hk_type type);
void __init lupos_isolation_init(void);
int __init lupos_isolation_late_init(void);
int __init lupos_isolation_nohz_full_setup(char *str,
					 struct lupos_isolation_boot_masks *masks);
int __init lupos_isolation_isolcpus_setup(char *str,
					struct lupos_isolation_boot_masks *masks);

#endif /* LUPOS_SCHED_ISOLATION_BINDINGS_H */
