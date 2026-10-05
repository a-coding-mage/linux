/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_SUPPORT_BINDINGS_H
#define LUPOS_SCHED_SUPPORT_BINDINGS_H

/* Native configured scheduler headers are the sole type/layout authority. */
#include <linux/slab.h>
#include "sched.h"

enum {
	LUPOS_SUPPORT_MAX_RT_PRIO = MAX_RT_PRIO,
	LUPOS_SUPPORT_CPUPRI_NR_PRIORITIES = CPUPRI_NR_PRIORITIES,
	LUPOS_SUPPORT_CPUPRI_INVALID = CPUPRI_INVALID,
	LUPOS_SUPPORT_CPUPRI_NORMAL = CPUPRI_NORMAL,
	LUPOS_SUPPORT_CPUPRI_HIGHER = CPUPRI_HIGHER,
	LUPOS_SUPPORT_IDX_INVALID = IDX_INVALID,
	LUPOS_SUPPORT_ENOMEM = ENOMEM,
};

int lupos_support_pri_atomic_read(const atomic_t *count);
void lupos_support_pri_atomic_set(atomic_t *count, int value);
void lupos_support_pri_atomic_inc(atomic_t *count);
void lupos_support_pri_atomic_dec(atomic_t *count);
void lupos_support_pri_read_barrier(void);
void lupos_support_pri_before_publish(void);
void lupos_support_pri_after_publish(void);
void lupos_support_pri_after_remove(void);
struct cpumask *lupos_support_pri_mask(struct cpupri_vec *vec);
bool lupos_support_pri_alloc_mask(struct cpupri_vec *vec);
void lupos_support_pri_free_mask(struct cpupri_vec *vec);
int *lupos_support_pri_alloc_priorities(void);
unsigned int lupos_support_pri_mask_any_and(const struct cpumask *a,
					  const struct cpumask *b);
bool lupos_support_pri_mask_and(struct cpumask *dst,
			      const struct cpumask *a, const struct cpumask *b);
bool lupos_support_pri_mask_empty(const struct cpumask *mask);
void lupos_support_pri_mask_set(int cpu, struct cpumask *mask);
void lupos_support_pri_mask_clear(int cpu, struct cpumask *mask);
const struct cpumask *lupos_support_pri_active_mask(void);
unsigned int lupos_support_pri_nr_cpu_ids(void);
int lupos_support_pri_mask_first(const struct cpumask *mask);
int lupos_support_pri_mask_next(int cpu, const struct cpumask *mask);
int lupos_support_pri_mask_limit(void);
int lupos_support_pri_possible_first(void);
int lupos_support_pri_possible_next(int cpu);
int lupos_support_pri_possible_limit(void);
void lupos_support_pri_warn_find_prio(int task_pri);
void lupos_support_pri_bug_set_prio(int newpri);
bool lupos_support_pri_likely_new(int newpri);
bool lupos_support_pri_likely_old(int oldpri);

bool lupos_support_dl_time_before(u64 a, u64 b);
struct cpumask *lupos_support_dl_free_cpus(struct cpudl *cp);
bool lupos_support_dl_alloc_mask(struct cpudl *cp);
void lupos_support_dl_free_mask(struct cpudl *cp);
struct cpudl_item *lupos_support_dl_alloc_elements(void);
bool lupos_support_dl_mask_and(struct cpumask *dst,
			     const struct cpumask *a, const struct cpumask *b);
bool lupos_support_dl_mask_empty(const struct cpumask *mask);
bool lupos_support_dl_mask_test(int cpu, const struct cpumask *mask);
void lupos_support_dl_mask_set(int cpu, struct cpumask *mask);
void lupos_support_dl_mask_clear(int cpu, struct cpumask *mask);
void lupos_support_dl_mask_set_private(int cpu, struct cpumask *mask);
void lupos_support_dl_mask_clear_private(int cpu, struct cpumask *mask);
void lupos_support_dl_warn_find_cpu(int best_cpu);
void lupos_support_dl_warn_clear_cpu(int cpu);
void lupos_support_dl_warn_set_cpu(int cpu);
void lupos_support_dl_lock_init(struct cpudl *cp);
unsigned long lupos_support_dl_clear_lock(struct cpudl *cp);
void lupos_support_dl_clear_unlock(struct cpudl *cp, unsigned long flags);
unsigned long lupos_support_dl_set_lock(struct cpudl *cp);
void lupos_support_dl_set_unlock(struct cpudl *cp, unsigned long flags);
bool lupos_support_dl_asym_active(void);
bool lupos_support_dl_fits_capacity(struct task_struct *p, int cpu);
unsigned long lupos_support_dl_cpu_capacity(int cpu);
unsigned int lupos_support_dl_task_cpu(const struct task_struct *p);
int lupos_support_dl_mask_first(const struct cpumask *mask);
int lupos_support_dl_mask_next(int cpu, const struct cpumask *mask);
int lupos_support_dl_mask_limit(void);
int lupos_support_dl_possible_first(void);
int lupos_support_dl_possible_next(int cpu);
int lupos_support_dl_possible_limit(void);
bool lupos_support_dl_likely_online(bool online);

#endif /* LUPOS_SCHED_SUPPORT_BINDINGS_H */
