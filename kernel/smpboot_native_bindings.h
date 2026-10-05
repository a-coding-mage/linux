/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SMPBOOT_NATIVE_BINDINGS_H
#define LUPOS_SMPBOOT_NATIVE_BINDINGS_H

/* Native headers own configured layouts, function signatures and constants. */
#include <linux/cpu.h>
#include <linux/err.h>
#include <linux/init.h>
#include <linux/kthread.h>
#include <linux/list.h>
#include <linux/sched.h>
#include <linux/sched/task.h>
#include <linux/smpboot.h>

#include "smpboot.h"

/* The original smpboot.c private native declaration, not a Rust projection. */
struct smpboot_thread_data {
	unsigned int			cpu;
	unsigned int			status;
	struct smp_hotplug_thread	*ht;
};

enum {
	HP_THREAD_NONE = 0,
	HP_THREAD_ACTIVE,
	HP_THREAD_PARKED,
};

#ifdef __BINDGEN__
static const int LUPOS_SMPBOOT_ENOMEM = ENOMEM;
#endif

/* Exact callback prototype, supplied directly to native kthread creation. */
int smpboot_thread_fn(void *data);

struct smpboot_thread_data *lupos_smpboot_alloc_data(unsigned int cpu);
void lupos_smpboot_free_data(struct smpboot_thread_data *td);
unsigned int lupos_smpboot_data_cpu(const struct smpboot_thread_data *td);
void lupos_smpboot_set_data_cpu(struct smpboot_thread_data *td,
			       unsigned int cpu);
unsigned int lupos_smpboot_data_status(const struct smpboot_thread_data *td);
void lupos_smpboot_set_data_status(struct smpboot_thread_data *td,
				  unsigned int status);
struct smp_hotplug_thread *
lupos_smpboot_data_ht(const struct smpboot_thread_data *td);
void lupos_smpboot_set_data_ht(struct smpboot_thread_data *td,
			      struct smp_hotplug_thread *ht);

struct task_struct *lupos_smpboot_task_get(struct smp_hotplug_thread *ht,
					 unsigned int cpu);
void lupos_smpboot_task_set(struct smp_hotplug_thread *ht, unsigned int cpu,
			   struct task_struct *tsk);
struct task_struct *lupos_smpboot_create_on_cpu(struct smpboot_thread_data *td,
					      unsigned int cpu,
					      struct smp_hotplug_thread *ht);
void lupos_smpboot_get_task_struct(struct task_struct *tsk);
unsigned long lupos_smpboot_wait_parked(struct task_struct *tsk);
bool lupos_smpboot_is_err(const struct task_struct *tsk);
long lupos_smpboot_ptr_err(const struct task_struct *tsk);

/* Nullable callbacks stay in their real native descriptor. Presence checks
 * and calls are separate leaves; Rust alone decides ordering and whether to
 * invoke each optional callback. The two required callbacks are not checked.
 */
bool lupos_smpboot_has_create(const struct smp_hotplug_thread *ht);
bool lupos_smpboot_has_setup(const struct smp_hotplug_thread *ht);
bool lupos_smpboot_has_cleanup(const struct smp_hotplug_thread *ht);
bool lupos_smpboot_has_park(const struct smp_hotplug_thread *ht);
bool lupos_smpboot_has_unpark(const struct smp_hotplug_thread *ht);
bool lupos_smpboot_selfparking(const struct smp_hotplug_thread *ht);
int lupos_smpboot_thread_should_run(struct smp_hotplug_thread *ht,
				   unsigned int cpu);
void lupos_smpboot_thread_run(struct smp_hotplug_thread *ht, unsigned int cpu);
void lupos_smpboot_create(struct smp_hotplug_thread *ht, unsigned int cpu);
void lupos_smpboot_setup(struct smp_hotplug_thread *ht, unsigned int cpu);
void lupos_smpboot_cleanup(struct smp_hotplug_thread *ht, unsigned int cpu,
			  bool online);
void lupos_smpboot_park(struct smp_hotplug_thread *ht, unsigned int cpu);
void lupos_smpboot_unpark(struct smp_hotplug_thread *ht, unsigned int cpu);

void lupos_smpboot_cpus_read_lock(void);
void lupos_smpboot_cpus_read_unlock(void);
void lupos_smpboot_lock(void);
void lupos_smpboot_unlock(void);
struct list_head *lupos_smpboot_list_head(void);
struct list_head *lupos_smpboot_list_next(const struct list_head *node);
struct list_head *lupos_smpboot_list_prev(const struct list_head *node);
struct smp_hotplug_thread *lupos_smpboot_list_entry(struct list_head *node);
void lupos_smpboot_list_add(struct smp_hotplug_thread *ht);
void lupos_smpboot_list_del(struct smp_hotplug_thread *ht);

/* Inclusive bit search and limit match the original for_each_*_cpu macros,
 * including the distinct NR_CPUS == 1 macro expansion. Rust owns the loops.
 */
unsigned int lupos_smpboot_next_possible_cpu(unsigned int start);
unsigned int lupos_smpboot_next_online_cpu(unsigned int start);
unsigned int lupos_smpboot_cpu_iter_limit(void);
bool lupos_smpboot_cpu_online(unsigned int cpu);

void lupos_smpboot_preempt_disable(void);
void lupos_smpboot_preempt_enable(void);
void lupos_smpboot_preempt_enable_no_resched(void);
void lupos_smpboot_set_interruptible(void);
void lupos_smpboot_set_running_stop(void);
void lupos_smpboot_set_running_park(void);
void lupos_smpboot_set_running_setup(void);
void lupos_smpboot_set_running_unpark(void);
void lupos_smpboot_set_running_work(void);
void lupos_smpboot_bug_park_cpu(struct smpboot_thread_data *td);
void lupos_smpboot_bug_run_cpu(struct smpboot_thread_data *td);
void lupos_smpboot_warn_create_inactive(void);

#ifdef CONFIG_GENERIC_SMP_IDLE_THREAD
struct task_struct *lupos_smpboot_idle_get(unsigned int cpu);
void lupos_smpboot_idle_set(unsigned int cpu, struct task_struct *tsk);
unsigned int lupos_smpboot_processor_id(void);
struct task_struct *lupos_smpboot_current(void);
struct task_struct *lupos_smpboot_err_ptr(int error);
void lupos_smpboot_err_fork_idle(unsigned int cpu);

/* Sole Rust algorithm bodies behind the full native __init entry metadata. */
void lupos_smpboot_idle_thread_set_boot_cpu(void);
void lupos_smpboot_idle_threads_init(void);
#endif /* CONFIG_GENERIC_SMP_IDLE_THREAD */

#endif /* LUPOS_SMPBOOT_NATIVE_BINDINGS_H */
