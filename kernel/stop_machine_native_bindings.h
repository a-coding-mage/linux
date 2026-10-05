/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_STOP_MACHINE_NATIVE_BINDINGS_H
#define LUPOS_STOP_MACHINE_NATIVE_BINDINGS_H

#include <linux/atomic.h>
#include <linux/completion.h>
#include <linux/cpu.h>
#include <linux/cpumask.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/list.h>
#include <linux/mutex.h>
#include <linux/sched.h>
#include <linux/smpboot.h>
#include <linux/spinlock.h>
#include <linux/stop_machine.h>

#ifdef CONFIG_SMP
/* Owner-private declarations from stop_machine.c, not substitute kernel types.
 * Bindgen must obtain all size/alignment and enum representation from this
 * configured native declaration. Rust treats the aggregate objects as opaque.
 */
struct cpu_stop_done {
	atomic_t nr_todo;
	int ret;
	struct completion completion;
};

struct cpu_stopper {
	struct task_struct *thread;
	raw_spinlock_t lock;
	bool enabled;
	struct list_head works;
	struct cpu_stop_work stop_work;
	unsigned long caller;
	cpu_stop_fn_t fn;
};

enum multi_stop_state {
	MULTI_STOP_NONE,
	MULTI_STOP_PREPARE,
	MULTI_STOP_DISABLE_IRQ,
	MULTI_STOP_RUN,
	MULTI_STOP_EXIT,
};

struct multi_stop_data {
	cpu_stop_fn_t fn;
	void *data;
	unsigned int num_threads;
	const struct cpumask *active_cpus;
	enum multi_stop_state state;
	atomic_t thread_ack;
};

/* No nullable function pointer appears in a Rust function signature. Native
 * entries borrow this container; native work/msdata initialization copies fn.
 */
struct lupos_stop_callback {
	cpu_stop_fn_t fn;
};

#ifdef __BINDGEN__
static const int LUPOS_STOP_ENOENT = ENOENT;
#endif

/* Native public adapters and descriptor invoke these existing Rust bodies. */
void lupos_stop_print_stop_info(const char *log_lvl, struct task_struct *task);
bool lupos_stop_cpu_stop_queue_work(unsigned int cpu, struct cpu_stop_work *work);
int lupos_stop_one_cpu(unsigned int cpu, const struct lupos_stop_callback *fn,
		       void *arg, unsigned long caller);
int lupos_stop_two_cpus(unsigned int cpu1, unsigned int cpu2,
			const struct lupos_stop_callback *fn, void *arg,
			unsigned long caller);
void lupos_stop_one_cpu_nowait(unsigned int cpu,
			     const struct lupos_stop_callback *fn, void *arg,
			     struct cpu_stop_work *work, unsigned long caller);
void lupos_stop_machine_park(int cpu);
void lupos_stop_machine_unpark(int cpu);
int lupos_stop_machine_cpuslocked(const struct lupos_stop_callback *fn,
				 void *data, const struct cpumask *cpus);
int lupos_stop_machine(const struct lupos_stop_callback *fn, void *data,
		       const struct cpumask *cpus);
#ifdef CONFIG_SCHED_SMT
int lupos_stop_core_cpuslocked(unsigned int cpu,
			      const struct lupos_stop_callback *fn, void *data);
#endif
int lupos_stop_machine_from_inactive_cpu(const struct lupos_stop_callback *fn,
				       void *data, const struct cpumask *cpus);
int lupos_stop_multi_cpu_stop(void *data);
int lupos_stop_cpu_stop_should_run(unsigned int cpu);
void lupos_stop_cpu_stopper_thread(unsigned int cpu);
void lupos_stop_cpu_stop_create(unsigned int cpu);
void lupos_stop_cpu_stop_park(unsigned int cpu);
int __init lupos_stop_cpu_stop_init(void);
bool lupos_stop_queue_stop_cpus_work(const struct cpumask *mask,
				   const struct lupos_stop_callback *fn,
				   void *arg, struct cpu_stop_done *done,
				   unsigned long caller);

/* A native call-site adapter, solely to evaluate _RET_IP_ in the queue entry. */
bool lupos_stop_queue_entry(const struct cpumask *mask,
			   const struct lupos_stop_callback *fn, void *arg,
			   struct cpu_stop_done *done);

/* Keep each original diagnostic expression at its own native macro site. */
void lupos_stop_register_threads(void);
void lupos_stop_bug_active_cpu(void);
void lupos_stop_warn_early_threads(const struct multi_stop_data *data);
void lupos_stop_warn_not_queued(unsigned int cpu, struct cpu_stop_work *work_buf);
void lupos_stop_warn_park_work(const struct cpu_stopper *stopper);
void lupos_stop_warn_preempt(const struct lupos_stop_callback *fn, void *arg);

#define STOP_LEAF(ret, name, args, ...) ret lupos_stop_##name args;
#include "stop_machine_native_primitives.inc"
#undef STOP_LEAF
#endif /* CONFIG_SMP */
#endif /* LUPOS_STOP_MACHINE_NATIVE_BINDINGS_H */
