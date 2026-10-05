// SPDX-License-Identifier: GPL-2.0-or-later
/* Exact public signatures, callback value adaptation, init/export/weak metadata.
 * No participant, queue, lock, completion, CPU-hotplug or unwind policy lives here.
 */
#error "SOURCE ONLY HOLD: stop-machine native metadata is not admitted"
#include "stop_machine_native_bindings.h"
#include <linux/compiler.h>
#include <linux/export.h>
#include <linux/instruction_pointer.h>

#ifdef CONFIG_SMP
void print_stop_info(const char *log_lvl, struct task_struct *task)
{
	lupos_stop_print_stop_info(log_lvl, task);
}

int stop_one_cpu(unsigned int cpu, cpu_stop_fn_t fn, void *arg)
{
	const struct lupos_stop_callback callback = { .fn = fn };

	return lupos_stop_one_cpu(cpu, &callback, arg, _RET_IP_);
}

int stop_two_cpus(unsigned int cpu1, unsigned int cpu2, cpu_stop_fn_t fn, void *arg)
{
	const struct lupos_stop_callback callback = { .fn = fn };

	return lupos_stop_two_cpus(cpu1, cpu2, &callback, arg, _RET_IP_);
}

void stop_one_cpu_nowait(unsigned int cpu, cpu_stop_fn_t fn, void *arg,
			 struct cpu_stop_work *work_buf)
{
	const struct lupos_stop_callback callback = { .fn = fn };

	lupos_stop_one_cpu_nowait(cpu, &callback, arg, work_buf, _RET_IP_);
}

void stop_machine_park(int cpu)
{
	lupos_stop_machine_park(cpu);
}

void stop_machine_unpark(int cpu)
{
	lupos_stop_machine_unpark(cpu);
}

notrace void __weak stop_machine_yield(const struct cpumask *cpumask)
{
	cpu_relax();
}

/* This entry, rather than a nested return-address leaf, captures the queue
 * caller as the original queue_stop_cpus_work entry did. Policy remains Rust.
 */
bool lupos_stop_queue_entry(const struct cpumask *mask,
			   const struct lupos_stop_callback *fn, void *arg,
			   struct cpu_stop_done *done)
{
	return lupos_stop_queue_stop_cpus_work(mask, fn, arg, done, _RET_IP_);
}

static int __init cpu_stop_init(void)
{
	return lupos_stop_cpu_stop_init();
}
early_initcall(cpu_stop_init);

int stop_machine_cpuslocked(cpu_stop_fn_t fn, void *data,
			    const struct cpumask *cpus)
{
	const struct lupos_stop_callback callback = { .fn = fn };

	return lupos_stop_machine_cpuslocked(&callback, data, cpus);
}

int stop_machine(cpu_stop_fn_t fn, void *data, const struct cpumask *cpus)
{
	const struct lupos_stop_callback callback = { .fn = fn };

	return lupos_stop_machine(&callback, data, cpus);
}
EXPORT_SYMBOL_GPL(stop_machine);

#ifdef CONFIG_SCHED_SMT
int stop_core_cpuslocked(unsigned int cpu, cpu_stop_fn_t fn, void *data)
{
	const struct lupos_stop_callback callback = { .fn = fn };

	return lupos_stop_core_cpuslocked(cpu, &callback, data);
}
EXPORT_SYMBOL_GPL(stop_core_cpuslocked);
#endif

int stop_machine_from_inactive_cpu(cpu_stop_fn_t fn, void *data,
				  const struct cpumask *cpus)
{
	const struct lupos_stop_callback callback = { .fn = fn };

	return lupos_stop_machine_from_inactive_cpu(&callback, data, cpus);
}
#endif /* CONFIG_SMP */
