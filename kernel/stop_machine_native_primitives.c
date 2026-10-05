// SPDX-License-Identifier: GPL-2.0-or-later
/* Native storage and primitive leaves; no stop-machine control algorithm. */
#error "SOURCE ONLY HOLD: stop-machine native owner is not admitted"
#include "stop_machine_native_bindings.h"
#include <linux/bug.h>
#include <linux/compiler.h>
#include <linux/interrupt.h>
#include <linux/kallsyms.h>
#include <linux/kthread.h>
#include <linux/nmi.h>
#include <linux/percpu.h>
#include <linux/rcupdate.h>
#include <linux/string.h>
#include <linux/topology.h>

#ifdef CONFIG_SMP
static DEFINE_PER_CPU(struct cpu_stopper, cpu_stopper);
static DEFINE_MUTEX(stop_cpus_mutex);
static bool stop_cpus_in_progress;
static bool stop_machine_initialized;

/* The exact native per-CPU member address is shared with every stopper getter.
 * smpboot owns task publication, retention and destruction. selfparking keeps
 * explicit stop-machine park/unpark requests with this owner.
 */
static struct smp_hotplug_thread cpu_stop_threads = {
	.store = &cpu_stopper.thread,
	.thread_should_run = lupos_stop_cpu_stop_should_run,
	.thread_fn = lupos_stop_cpu_stopper_thread,
	.thread_comm = "migration/%u",
	.create = lupos_stop_cpu_stop_create,
	.park = lupos_stop_cpu_stop_park,
	.selfparking = true,
};

static const struct lupos_stop_callback multi_cpu_stop_callback = {
	.fn = lupos_stop_multi_cpu_stop,
};

/* Preserve the original WARN_ON_ONCE expression without moving its queue
 * algorithm out of Rust. This synchronous adapter retains neither pointer.
 */
static bool cpu_stop_queue_work(unsigned int cpu, struct cpu_stop_work *work)
{
	return lupos_stop_cpu_stop_queue_work(cpu, work);
}

#define STOP_LEAF(ret, name, args, ...) \
	ret lupos_stop_##name args { __VA_ARGS__ }
#include "stop_machine_native_primitives.inc"
#undef STOP_LEAF

/* Native diagnostics retain their separate expressions, evaluation and source
 * loci. The line directives do not assert instruction-pointer/trace identity.
 */
void lupos_stop_register_threads(void)
{
#line 578 "kernel/stop_machine.c"
	BUG_ON(smpboot_register_percpu_thread(&cpu_stop_threads));
}

void lupos_stop_bug_active_cpu(void)
{
#line 690 "kernel/stop_machine.c"
	BUG_ON(cpu_active(raw_smp_processor_id()));
}

void lupos_stop_warn_early_threads(const struct multi_stop_data *data)
{
	/* Lvalue alias preserves the original condition text without copying the
	 * native object or evaluating unrelated fields before the macro.
	 */
#define msdata (*data)
#line 606 "kernel/stop_machine.c"
	WARN_ON_ONCE(msdata.num_threads != 1);
#undef msdata
}

void lupos_stop_warn_not_queued(unsigned int cpu, struct cpu_stop_work *work_buf)
{
#line 387 "kernel/stop_machine.c"
	WARN_ON_ONCE(!cpu_stop_queue_work(cpu, work_buf));
}

void lupos_stop_warn_park_work(const struct cpu_stopper *stopper)
{
#line 546 "kernel/stop_machine.c"
	WARN_ON(!list_empty(&stopper->works));
}

void lupos_stop_warn_preempt(const struct lupos_stop_callback *fn, void *arg)
{
#line 519 "kernel/stop_machine.c"
	WARN_ONCE(preempt_count(),
		  "cpu_stop: %ps(%p) leaked preempt count\n", fn->fn, arg);
}
#endif /* CONFIG_SMP */
