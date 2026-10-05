// SPDX-License-Identifier: GPL-2.0-only
/* Native storage, field/header primitives and callback signatures only.
 * All worker, registration, unwind and CPU/list loops stay in smpboot.rs.
 */
#error "SOURCE ONLY HOLD: smpboot native primitives are not admitted"

#include "smpboot_native_bindings.h"
#include <linux/mutex.h>
#include <linux/percpu.h>
#include <linux/slab.h>
#include <linux/smp.h>

/* Preserve original native lockdep/RT/list/per-CPU initializer contracts. */
static LIST_HEAD(hotplug_threads);
static DEFINE_MUTEX(smpboot_threads_lock);

struct smpboot_thread_data *lupos_smpboot_alloc_data(unsigned int cpu)
{
	struct smpboot_thread_data *td;

	return kzalloc_node(sizeof(*td), GFP_KERNEL, cpu_to_node(cpu));
}

void lupos_smpboot_free_data(struct smpboot_thread_data *td)
{
	kfree(td);
}

unsigned int lupos_smpboot_data_cpu(const struct smpboot_thread_data *td)
{
	return td->cpu;
}

void lupos_smpboot_set_data_cpu(struct smpboot_thread_data *td,
			       unsigned int cpu)
{
	td->cpu = cpu;
}

unsigned int lupos_smpboot_data_status(const struct smpboot_thread_data *td)
{
	return td->status;
}

void lupos_smpboot_set_data_status(struct smpboot_thread_data *td,
				  unsigned int status)
{
	td->status = status;
}

struct smp_hotplug_thread *
lupos_smpboot_data_ht(const struct smpboot_thread_data *td)
{
	return td->ht;
}

void lupos_smpboot_set_data_ht(struct smpboot_thread_data *td,
			      struct smp_hotplug_thread *ht)
{
	td->ht = ht;
}

struct task_struct *lupos_smpboot_task_get(struct smp_hotplug_thread *ht,
					 unsigned int cpu)
{
	return *per_cpu_ptr(ht->store, cpu);
}

void lupos_smpboot_task_set(struct smp_hotplug_thread *ht, unsigned int cpu,
			   struct task_struct *tsk)
{
	*per_cpu_ptr(ht->store, cpu) = tsk;
}

struct task_struct *lupos_smpboot_create_on_cpu(struct smpboot_thread_data *td,
					      unsigned int cpu,
					      struct smp_hotplug_thread *ht)
{
	/* Prototype-checked native callback binding; no creation policy here. */
	return kthread_create_on_cpu(smpboot_thread_fn, td, cpu, ht->thread_comm);
}

void lupos_smpboot_get_task_struct(struct task_struct *tsk)
{
	get_task_struct(tsk);
}

unsigned long lupos_smpboot_wait_parked(struct task_struct *tsk)
{
	return wait_task_inactive(tsk, TASK_PARKED);
}

bool lupos_smpboot_is_err(const struct task_struct *tsk)
{
	return IS_ERR(tsk);
}

long lupos_smpboot_ptr_err(const struct task_struct *tsk)
{
	return PTR_ERR(tsk);
}

bool lupos_smpboot_has_create(const struct smp_hotplug_thread *ht)
{
	return ht->create;
}

bool lupos_smpboot_has_setup(const struct smp_hotplug_thread *ht)
{
	return ht->setup;
}

bool lupos_smpboot_has_cleanup(const struct smp_hotplug_thread *ht)
{
	return ht->cleanup;
}

bool lupos_smpboot_has_park(const struct smp_hotplug_thread *ht)
{
	return ht->park;
}

bool lupos_smpboot_has_unpark(const struct smp_hotplug_thread *ht)
{
	return ht->unpark;
}

bool lupos_smpboot_selfparking(const struct smp_hotplug_thread *ht)
{
	return ht->selfparking;
}

int lupos_smpboot_thread_should_run(struct smp_hotplug_thread *ht,
				   unsigned int cpu)
{
	return ht->thread_should_run(cpu);
}

void lupos_smpboot_thread_run(struct smp_hotplug_thread *ht, unsigned int cpu)
{
	ht->thread_fn(cpu);
}

void lupos_smpboot_create(struct smp_hotplug_thread *ht, unsigned int cpu)
{
	ht->create(cpu);
}

void lupos_smpboot_setup(struct smp_hotplug_thread *ht, unsigned int cpu)
{
	ht->setup(cpu);
}

void lupos_smpboot_cleanup(struct smp_hotplug_thread *ht, unsigned int cpu,
			  bool online)
{
	ht->cleanup(cpu, online);
}

void lupos_smpboot_park(struct smp_hotplug_thread *ht, unsigned int cpu)
{
	ht->park(cpu);
}

void lupos_smpboot_unpark(struct smp_hotplug_thread *ht, unsigned int cpu)
{
	ht->unpark(cpu);
}

void lupos_smpboot_cpus_read_lock(void)
{
	cpus_read_lock();
}

void lupos_smpboot_cpus_read_unlock(void)
{
	cpus_read_unlock();
}

void lupos_smpboot_lock(void)
{
	mutex_lock(&smpboot_threads_lock);
}

void lupos_smpboot_unlock(void)
{
	mutex_unlock(&smpboot_threads_lock);
}

struct list_head *lupos_smpboot_list_head(void)
{
	return &hotplug_threads;
}

struct list_head *lupos_smpboot_list_next(const struct list_head *node)
{
	return node->next;
}

struct list_head *lupos_smpboot_list_prev(const struct list_head *node)
{
	return node->prev;
}

struct smp_hotplug_thread *lupos_smpboot_list_entry(struct list_head *node)
{
	return list_entry(node, struct smp_hotplug_thread, list);
}

void lupos_smpboot_list_add(struct smp_hotplug_thread *ht)
{
	list_add(&ht->list, &hotplug_threads);
}

void lupos_smpboot_list_del(struct smp_hotplug_thread *ht)
{
	list_del(&ht->list);
}

unsigned int lupos_smpboot_next_possible_cpu(unsigned int start)
{
#if NR_CPUS == 1
	return start;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask),
			     small_cpumask_bits, start);
#endif
}

unsigned int lupos_smpboot_next_online_cpu(unsigned int start)
{
#if NR_CPUS == 1
	return start;
#else
	return find_next_bit(cpumask_bits(cpu_online_mask),
			     small_cpumask_bits, start);
#endif
}

unsigned int lupos_smpboot_cpu_iter_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}

bool lupos_smpboot_cpu_online(unsigned int cpu)
{
	return cpu_online(cpu);
}

void lupos_smpboot_preempt_disable(void)
{
	preempt_disable();
}

void lupos_smpboot_preempt_enable(void)
{
	preempt_enable();
}

void lupos_smpboot_preempt_enable_no_resched(void)
{
	preempt_enable_no_resched();
}

/* Keep six distinct macro sites and their original state operands. No
 * stronger atomics replace set_current_state/__set_current_state. Native
 * debug state checks and state tracing remain part of each macro expansion.
 */
void lupos_smpboot_set_interruptible(void)
{
#line 108 "kernel/smpboot.c"
	set_current_state(TASK_INTERRUPTIBLE);
}
#line 1 "kernel/smpboot_native_primitives.c"

void lupos_smpboot_set_running_stop(void)
{
#line 111 "kernel/smpboot.c"
	__set_current_state(TASK_RUNNING);
}
#line 1 "kernel/smpboot_native_primitives.c"

void lupos_smpboot_set_running_park(void)
{
#line 121 "kernel/smpboot.c"
	__set_current_state(TASK_RUNNING);
}
#line 1 "kernel/smpboot_native_primitives.c"

void lupos_smpboot_set_running_setup(void)
{
#line 138 "kernel/smpboot.c"
	__set_current_state(TASK_RUNNING);
}
#line 1 "kernel/smpboot_native_primitives.c"

void lupos_smpboot_set_running_unpark(void)
{
#line 146 "kernel/smpboot.c"
	__set_current_state(TASK_RUNNING);
}
#line 1 "kernel/smpboot_native_primitives.c"

void lupos_smpboot_set_running_work(void)
{
#line 158 "kernel/smpboot.c"
	__set_current_state(TASK_RUNNING);
}
#line 1 "kernel/smpboot_native_primitives.c"

/* Separate original diagnostic sites retain their native operands. In
 * particular WARN_ON(1) must not become WARN_ON(!wait_task_inactive(...)).
 * The wait and callback decision belong only to the Rust creation body.
 */
void lupos_smpboot_bug_park_cpu(struct smpboot_thread_data *td)
{
#line 124 "kernel/smpboot.c"
	BUG_ON(td->cpu != smp_processor_id());
}
#line 1 "kernel/smpboot_native_primitives.c"

void lupos_smpboot_bug_run_cpu(struct smpboot_thread_data *td)
{
#line 133 "kernel/smpboot.c"
	BUG_ON(td->cpu != smp_processor_id());
}
#line 1 "kernel/smpboot_native_primitives.c"

void lupos_smpboot_warn_create_inactive(void)
{
#line 202 "kernel/smpboot.c"
	WARN_ON(1);
}
#line 1 "kernel/smpboot_native_primitives.c"

#ifdef CONFIG_GENERIC_SMP_IDLE_THREAD
static DEFINE_PER_CPU(struct task_struct *, idle_threads);

struct task_struct *lupos_smpboot_idle_get(unsigned int cpu)
{
	return per_cpu(idle_threads, cpu);
}

void lupos_smpboot_idle_set(unsigned int cpu, struct task_struct *tsk)
{
	per_cpu(idle_threads, cpu) = tsk;
}

unsigned int lupos_smpboot_processor_id(void)
{
	return smp_processor_id();
}

struct task_struct *lupos_smpboot_current(void)
{
	return current;
}

struct task_struct *lupos_smpboot_err_ptr(int error)
{
	return ERR_PTR(error);
}

void lupos_smpboot_err_fork_idle(unsigned int cpu)
{
#line 55 "kernel/smpboot.c"
	pr_err("SMP: fork_idle() failed for CPU %u\n", cpu);
}
#line 1 "kernel/smpboot_native_primitives.c"
#endif /* CONFIG_GENERIC_SMP_IDLE_THREAD */
