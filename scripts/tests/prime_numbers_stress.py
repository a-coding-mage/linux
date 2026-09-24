#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Separate GPL native prime-cache allocation-failure and held-reader probe.

The existing Proprietary public ABI gate is not changed. This disposable GPL
module needs tracing/kthread APIs. It observes real FAILSLAB NULL allocations,
cache preservation/retry, a live old RCU bitmap across writer publication and
deterministic cancellation of a competing writer's real allocation.
It does not claim complete batched deferred-free reclamation.

Guest debugfs preparation is mandatory (--prepare-failslab): probability=0,
cache-filter=N, ignore-gfp-wait=N and permissive stack filtering. Injection is
then armed only in current->fail_nth immediately around an actual provider call.
Provider and suite must be modules so reload genuinely starts with a fresh cache.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

from boot_kernel import module_name, verify_module_events
from check_div64_kernel import architecture, configuration, verify_build_command
from check_cordic_kernel import imported_crc, tool
from check_polynomial_kernel import elf_target, newer
from check_prime_numbers_kernel import (PUBLIC, PRIVATE, kunit_runs, reference_source,
                                      verify_linked_implementation, verify_module_import_versions,
                                      provider_type_ids, verify_guarded_calls)
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
NAME = "prime_numbers_stress"
TIMEOUT_MS = 5000
SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0-only
/* Disposable native test only; no provider or allocator implementation. */
#include <linux/atomic.h>
#include <linux/bitmap.h>
#include <linux/completion.h>
#include <linux/cpu.h>
#include <linux/delay.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/jiffies.h>
#include <linux/kthread.h>
#include <linux/module.h>
#include <linux/prime_numbers.h>
#include <linux/rcupdate.h>
#include <linux/sched/task.h>
#include <linux/slab.h>
#include <linux/string.h>
#include <trace/events/kmem.h>
#include "@PRIVATE_HEADER@"

#if !defined(CONFIG_FAILSLAB) || !defined(CONFIG_FAULT_INJECTION)
#error This fixture needs real per-task FAILSLAB injection
#endif

#define MAX_BITS 2048
#define WAIT_MS 5000
#define CHURN 8
bool prime_reference_is(unsigned long);
unsigned long prime_reference_next(unsigned long);
static bool (*volatile actual_is)(unsigned long) = is_prime_number;
static unsigned long (*volatile actual_next)(unsigned long) = next_prime_number;

/* Keep the complete callback-bearing export type and force an indirect load:
 * both its outer KCFI identity and each callback are checked by the kernel. */
static noinline notrace void primes_call_with(void *ctx, primes_fn fn)
{
    void (*volatile target)(void *, primes_fn) = with_primes;
    target(ctx, fn);
}
#define with_primes primes_call_with

struct snapshot {
    unsigned long last, bits;
    unsigned long words[BITS_TO_LONGS(MAX_BITS)];
    bool valid;
};

/* The callback copies only immutable fields, never the concurrently mutable
 * rcu_head, and never retains a cache pointer beyond the read-side section. */
static void capture(const struct primes *p, struct snapshot *out)
{
    unsigned long x;

    memset(out, 0, sizeof(*out));
    out->last = p->last;
    out->bits = p->sz;
    if (!out->bits || out->bits > MAX_BITS || out->bits % BITS_PER_LONG ||
        out->last >= out->bits || !rcu_read_lock_held()) return;
    bitmap_copy(out->words, p->primes, out->bits);
    for (x = 0; x < out->bits; x++)
        if (!!test_bit(x, out->words) != (x > 1 && prime_reference_is(x))) return;
    out->valid = out->last == find_last_bit(out->words, out->bits);
}

static void snapshot_callback(void *ctx, const struct primes *p)
{
    capture(p, ctx);
}

static bool same_snapshot(const struct snapshot *a, const struct snapshot *b)
{
    return a->valid && b->valid && a->last == b->last && a->bits == b->bits &&
           !memcmp(a->words, b->words, bitmap_size(a->bits));
}

struct allocation_window {
    struct task_struct *task;
    size_t request;
    unsigned int seen, nulls, bad;
    bool active;
};
static struct allocation_window window;

/* Real exported tracepoint signature; no allocation, logging, blocking, or
 * mutation of the allocator result. Interrupt-context allocations are excluded. */
static void allocation_probe(void *unused, unsigned long call_site, const void *ptr,
                             size_t requested, size_t allocated, gfp_t flags, int node)
{
    if (!smp_load_acquire(&window.active) || current != READ_ONCE(window.task) || !in_task()) return;
    window.seen++;
    window.nulls += ptr == NULL;
    if (requested != window.request || allocated < requested ||
        flags != (GFP_KERNEL | __GFP_NOWARN)) window.bad++;
}

static int attempt(const char *name, unsigned long x, bool next, bool inject, bool allocation)
{
    struct snapshot before, after;
    unsigned long actual, expected, new_bits = round_up(2 * x, BITS_PER_LONG);
    unsigned int remaining;
    bool same;

    with_primes(&before, snapshot_callback);
    if (!before.valid || READ_ONCE(current->fail_nth)) return -EINVAL;
    expected = next ? prime_reference_next(x) : x > 1 && prime_reference_is(x);
    WRITE_ONCE(window.task, current);
    window.request = allocation ? sizeof(struct primes) + bitmap_size(new_bits) : 0;
    window.seen = window.nulls = window.bad = 0;
    smp_store_release(&window.active, true);
    WRITE_ONCE(current->fail_nth, inject ? 1 : 0);
    actual = next ? actual_next(x) : actual_is(x);
    remaining = READ_ONCE(current->fail_nth);
    WRITE_ONCE(current->fail_nth, 0);
    smp_store_release(&window.active, false);
    /* Every diagnostic, snapshot and error path is after disarming injection. */
    with_primes(&after, snapshot_callback);
    same = same_snapshot(&before, &after);
    if (actual != expected || !after.valid || window.bad ||
        window.seen != (allocation ? 1 : 0) ||
        window.nulls != (allocation && inject ? 1 : 0) ||
        remaining != (inject && !allocation ? 1 : 0) ||
        ((inject || !allocation) ? !same : (same || after.bits != new_bits || after.last <= x)))
        return -EINVAL;
    pr_info("LUPOS_PRIMES_STRESS_ALLOC name=%s request=%zu seen=%u null=%u fail_nth=%u same=%u\n",
            name, window.request, window.seen, window.nulls, remaining, same);
    return 0;
}

static DECLARE_COMPLETION(reader_entered);
static DECLARE_COMPLETION(reader_finished);
static DECLARE_COMPLETION(writer_finished);
static atomic_t abort_threads = ATOMIC_INIT(0);
static bool writer_published;
static struct snapshot old_cache, new_cache;
static unsigned long writer_value;
static int reader_status, writer_status;
static const char *reader_phase = "not_started", *writer_phase = "not_started";
static void *churn[CHURN];

static void held_reader(void *unused, const struct primes *p)
{
    struct snapshot after;
    unsigned long deadline = jiffies + msecs_to_jiffies(WAIT_MS);

    reader_phase = "capture_old";
    capture(p, &old_cache);
    complete(&reader_entered);
    /* Never sleep in the RCU callback. The writer is bound to another CPU. */
    reader_phase = "wait_publication";
    while (!smp_load_acquire(&writer_published) && !atomic_read(&abort_threads) &&
           time_before(jiffies, deadline)) cpu_relax();
    if (!smp_load_acquire(&writer_published) || atomic_read(&abort_threads)) {
        reader_phase = atomic_read(&abort_threads) ? "aborted" : "publication_timeout";
        reader_status = -ETIMEDOUT;
        return;
    }
    reader_phase = "compare_held_cache";
    capture(p, &after);
    reader_status = same_snapshot(&old_cache, &after) ? 0 : -EINVAL;
    if (!reader_status) reader_phase = "done";
}

static int reader_thread(void *unused)
{
    if (atomic_read(&abort_threads)) { reader_phase = "aborted_before_callback"; reader_status = -ECANCELED; }
    else with_primes(NULL, held_reader);
    complete(&reader_finished);
    return reader_status;
}

static int writer_thread(void *unused)
{
    unsigned int i;

    writer_phase = "wait_reader";
    writer_status = -ETIMEDOUT;
    if (!wait_for_completion_timeout(&reader_entered, msecs_to_jiffies(WAIT_MS))) goto out;
    writer_phase = "reader_aborted";
    if (atomic_read(&abort_threads)) goto out;
    writer_phase = "invalid_old_cache";
    if (!old_cache.valid) goto out;
    writer_phase = "native_publication";
    writer_value = actual_next(old_cache.bits);
    with_primes(&new_cache, snapshot_callback);
    writer_phase = "invalid_new_cache";
    if (!new_cache.valid || new_cache.bits <= old_cache.bits ||
        writer_value != prime_reference_next(old_cache.bits)) goto out;
    /* Apply real allocator pressure while the old dynamic reader is held.
     * This does not claim every possible premature-free bug is detected. */
    writer_phase = "churn_allocation";
    for (i = 0; i < CHURN; i++) {
        churn[i] = kmalloc(sizeof(struct primes) + bitmap_size(old_cache.bits), GFP_KERNEL);
        if (!churn[i]) { writer_status = -ENOMEM; goto out; }
        memset(churn[i], 0xa5, sizeof(struct primes) + bitmap_size(old_cache.bits));
    }
    writer_status = 0;
    writer_phase = "done";
out:
    if (writer_status) atomic_set(&abort_threads, 1);
    smp_store_release(&writer_published, true);
    complete(&writer_finished);
    return writer_status;
}

static int concurrency(void)
{
    struct task_struct *reader = NULL, *writer = NULL;
    unsigned int cpu_reader, cpu_writer, i, online;
    bool reader_done = false, writer_done = false;
    const char *phase = "online_cpus";
    int ret = -EINVAL;

    cpus_read_lock();
    online = num_online_cpus();
    if (online < 2) {
        pr_err("LUPOS_PRIMES_STRESS_CONCURRENCY_ERROR reason=online_cpus online=%u required=2\n", online);
        goto unlock;
    }
    cpu_reader = cpumask_first(cpu_online_mask);
    cpu_writer = cpumask_next(cpu_reader, cpu_online_mask);
    phase = "reader_create";
    reader = kthread_create(reader_thread, NULL, "prime-rcu-reader");
    if (IS_ERR(reader)) { ret = PTR_ERR(reader); reader = NULL; goto report; }
    /* Keep explicit task references: callbacks can return before stop/join. */
    get_task_struct(reader);
    phase = "writer_create";
    writer = kthread_create(writer_thread, NULL, "prime-rcu-writer");
    if (IS_ERR(writer)) { ret = PTR_ERR(writer); writer = NULL; goto stop; }
    get_task_struct(writer);
    kthread_bind(reader, cpu_reader);
    kthread_bind(writer, cpu_writer);
    wake_up_process(writer);
    wake_up_process(reader);
    phase = "wait_workers";
    reader_done = wait_for_completion_timeout(&reader_finished, msecs_to_jiffies(2 * WAIT_MS));
    writer_done = wait_for_completion_timeout(&writer_finished, msecs_to_jiffies(2 * WAIT_MS));
    ret = reader_done && writer_done && !reader_status && !writer_status ? 0 : -EINVAL;
stop:
    atomic_set(&abort_threads, 1);
    complete_all(&reader_entered);
    if (reader) { kthread_stop(reader); put_task_struct(reader); }
    if (writer) { kthread_stop(writer); put_task_struct(writer); }
    for (i = 0; i < CHURN; i++) { kfree(churn[i]); churn[i] = NULL; }
    if (!ret) pr_info("LUPOS_PRIMES_STRESS_RCU_OK old_bits=%lu new_bits=%lu held=1 churn=8 joined=2\n",
                      old_cache.bits, new_cache.bits);
report:
    /* Threads are joined before reading their diagnostics. Never printk while
     * holding the old reader's RCU callback or inside an injection window. */
    if (ret) pr_err("LUPOS_PRIMES_STRESS_CONCURRENCY_ERROR reason=%s online=%u reader_cpu=%u writer_cpu=%u reader_done=%u writer_done=%u reader_phase=%s writer_phase=%s reader_error=%d writer_error=%d old_valid=%u old_bits=%lu new_valid=%u new_bits=%lu value=%lu\n",
                    phase, online, cpu_reader, cpu_writer, reader_done, writer_done,
                    reader_phase, writer_phase, reader_status, writer_status,
                    old_cache.valid, old_cache.bits, new_cache.valid, new_cache.bits, writer_value);
unlock:
    cpus_read_unlock();
    return ret;
}

/* Two real allocations precede publication. A may leave its allocation callback
 * only after B arrived; B may leave only after A returned from the provider and
 * observed its own published cache under a fresh RCU guard. Thus B must exercise
 * the provider's locked growth recheck rather than publish an equal bitmap.
 * Addresses are opaque identities: no retained cache pointer is dereferenced. */
struct race_worker {
    struct task_struct *task;
    struct completion finished;
    struct snapshot cache;
    unsigned long allocation, value;
    unsigned int allocations, bad;
    int status;
    bool in_provider, arrived, winner, loser;
    const char *phase;
};
static struct race_worker race[2];
static DECLARE_COMPLETION(race_start_gate);
static atomic_t race_abort = ATOMIC_INIT(0);
static atomic_t race_frees[2] = { ATOMIC_INIT(0), ATOMIC_INIT(0) };
static atomic_t race_wrong_free = ATOMIC_INIT(0);
static bool race_enabled, race_published;
static unsigned long race_input;
static size_t race_request;

static int race_task(void)
{
    unsigned int i;

    if (!smp_load_acquire(&race_enabled) || !in_task()) return -1;
    for (i = 0; i < ARRAY_SIZE(race); i++)
        if (current == READ_ONCE(race[i].task) && READ_ONCE(race[i].in_provider)) return i;
    return -1;
}

static void race_allocation_probe(void *unused, unsigned long call_site, const void *ptr,
                                  size_t requested, size_t allocated, gfp_t flags, int node)
{
    struct race_worker *worker;
    unsigned long deadline;
    int index = race_task();

    if (index < 0) return;
    worker = &race[index];
    if (++worker->allocations != 1 || !ptr || requested != race_request || allocated < requested ||
        flags != (GFP_KERNEL | __GFP_NOWARN) || READ_ONCE(current->fail_nth)) {
        worker->bad++;
        atomic_set(&race_abort, 1);
        return;
    }
    WRITE_ONCE(worker->allocation, (unsigned long)ptr);
    smp_store_release(&worker->arrived, true);
    deadline = jiffies + msecs_to_jiffies(WAIT_MS);
    /* Trace callbacks must not sleep, allocate, printk or invoke the provider.
     * The peer is already runnable and bound to the other online CPU. */
    while (!(index ? smp_load_acquire(&race_published) : smp_load_acquire(&race[1].arrived)) &&
           !atomic_read(&race_abort) && time_before(jiffies, deadline)) cpu_relax();
    if (atomic_read(&race_abort) ||
        !(index ? smp_load_acquire(&race_published) : smp_load_acquire(&race[1].arrived))) {
        worker->bad++;
        atomic_set(&race_abort, 1);
    }
}

static void race_free_probe(void *unused, unsigned long call_site, const void *ptr)
{
    unsigned int i;
    int index;

    /* krealloc(NULL, ...) may emit kfree(NULL). Old-cache RCU retirement and
     * unrelated frees are not evidence of cancelling these new allocations.
     * Observe matching addresses from every task/context while enabled; only
     * the expected loser free is authorized in B's native provider call. */
    if (!smp_load_acquire(&race_enabled) || !ptr) return;
    index = race_task();
    for (i = 0; i < ARRAY_SIZE(race); i++) {
        if ((unsigned long)ptr != READ_ONCE(race[i].allocation)) continue;
        atomic_inc(&race_frees[i]);
        if (i != 1 || index != 1) atomic_inc(&race_wrong_free);
    }
}

static void race_snapshot(void *ctx, const struct primes *p)
{
    struct race_worker *worker = ctx;

    capture(p, &worker->cache);
    worker->winner = (unsigned long)p == READ_ONCE(race[0].allocation);
    worker->loser = (unsigned long)p == READ_ONCE(race[1].allocation);
}

static int race_thread(void *ctx)
{
    struct race_worker *worker = ctx;
    bool first = worker == &race[0];

    worker->phase = "start_gate";
    worker->status = -ETIMEDOUT;
    if (!wait_for_completion_timeout(&race_start_gate, msecs_to_jiffies(WAIT_MS))) goto out;
    worker->phase = "before_provider";
    if (atomic_read(&race_abort) || kthread_should_stop() || READ_ONCE(current->fail_nth)) goto out;
    WRITE_ONCE(worker->in_provider, true);
    worker->value = actual_next(race_input);
    WRITE_ONCE(worker->in_provider, false);
    worker->phase = "guarded_publication";
    with_primes(worker, race_snapshot);
    worker->status = -EINVAL;
    if (atomic_read(&race_abort) || worker->allocations != 1 || worker->bad ||
        READ_ONCE(current->fail_nth) || !worker->cache.valid || !worker->winner || worker->loser ||
        worker->cache.bits != 2 * race_input || worker->cache.last <= race_input ||
        worker->value != prime_reference_next(race_input)) goto out;
    if (!first && (!smp_load_acquire(&race_published) || !same_snapshot(&race[0].cache, &worker->cache))) goto out;
    worker->status = 0;
    worker->phase = "done";
    if (first) smp_store_release(&race_published, true);
out:
    if (worker->status) atomic_set(&race_abort, 1);
    complete(&worker->finished);
    return worker->status;
}

static int competing_writers(void)
{
    struct snapshot before;
    struct race_worker final = {};
    bool registered_alloc = false, registered_free = false, done[2] = { false, false };
    const char *phase = "initial_cache";
    unsigned int i, cpus[2] = {};
    int ret = -EINVAL;

    with_primes(&before, snapshot_callback);
    if (!before.valid || before.bits != 512 || READ_ONCE(current->fail_nth)) goto report;
    race_input = before.bits;
    race_request = sizeof(struct primes) + bitmap_size(2 * race_input);
    for (i = 0; i < ARRAY_SIZE(race); i++) {
        init_completion(&race[i].finished);
        race[i].status = -ECANCELED;
        race[i].phase = "not_started";
    }
    /* Tracepoint registration/unregistration takes tracepoints_mutex and may
     * acquire cpu_hotplug_lock through static-call updates. Never reverse that
     * order by performing either operation under our CPU-stability guard. */
    phase = "register_allocation";
    ret = register_trace_kmalloc(race_allocation_probe, NULL);
    if (ret) goto report;
    registered_alloc = true;
    phase = "register_free";
    ret = register_trace_kfree(race_free_probe, NULL);
    if (ret) goto unregister;
    registered_free = true;
    cpus_read_lock();
    phase = "online_cpus";
    ret = -EINVAL;
    if (num_online_cpus() < 2) goto stop;
    cpus[0] = cpumask_first(cpu_online_mask);
    cpus[1] = cpumask_next(cpus[0], cpu_online_mask);
    phase = "create_workers";
    for (i = 0; i < ARRAY_SIZE(race); i++) {
        struct task_struct *task = kthread_create(race_thread, &race[i], "prime-race-%u", i);

        if (IS_ERR(task)) { ret = PTR_ERR(task); goto stop; }
        get_task_struct(task);
        WRITE_ONCE(race[i].task, task);
        kthread_bind(task, cpus[i]);
    }
    /* Wake both into a shared sleeping gate before allowing either callback to
     * spin; otherwise the first callback could preempt its peer's creator. */
    phase = "wait_workers";
    smp_store_release(&race_enabled, true);
    for (i = 0; i < ARRAY_SIZE(race); i++) wake_up_process(race[i].task);
    complete_all(&race_start_gate);
    for (i = 0; i < ARRAY_SIZE(race); i++)
        done[i] = wait_for_completion_timeout(&race[i].finished, msecs_to_jiffies(2 * WAIT_MS));
    ret = done[0] && done[1] && !race[0].status && !race[1].status ? 0 : -EINVAL;
stop:
    atomic_set(&race_abort, 1);
    complete_all(&race_start_gate);
    for (i = 0; i < ARRAY_SIZE(race); i++)
        if (race[i].task) kthread_stop(race[i].task);
    /* Both native calls have returned before testing frees or removing probes.
     * The fixture never frees a captured candidate on behalf of the provider. */
    smp_store_release(&race_enabled, false);
    cpus_read_unlock();
unregister:
    if (registered_free && unregister_trace_kfree(race_free_probe, NULL)) ret = -EINVAL;
    if (registered_alloc && unregister_trace_kmalloc(race_allocation_probe, NULL)) ret = -EINVAL;
    if (registered_alloc || registered_free) tracepoint_synchronize_unregister();
    for (i = 0; i < ARRAY_SIZE(race); i++) {
        if (race[i].task) { put_task_struct(race[i].task); WRITE_ONCE(race[i].task, NULL); }
    }
    if (!ret) {
        phase = "cancellation_evidence";
        with_primes(&final, race_snapshot);
        if (!race[0].allocation || !race[1].allocation || race[0].allocation == race[1].allocation ||
            race[0].allocations != 1 || race[1].allocations != 1 ||
            atomic_read(&race_frees[0]) || atomic_read(&race_frees[1]) != 1 || atomic_read(&race_wrong_free) ||
            !final.winner || final.loser || !same_snapshot(&race[0].cache, &final.cache)) ret = -EINVAL;
    }
report:
    if (ret) pr_err("LUPOS_PRIMES_STRESS_RACE_ERROR reason=%s cpu0=%u cpu1=%u done0=%u done1=%u phase0=%s phase1=%s error0=%d error1=%d allocations0=%u allocations1=%u bad0=%u bad1=%u frees0=%d frees1=%d wrong_free=%d published=%u winner=%u loser=%u\n",
                    phase, cpus[0], cpus[1], done[0], done[1], race[0].phase ?: "not_started", race[1].phase ?: "not_started",
                    race[0].status, race[1].status, race[0].allocations, race[1].allocations, race[0].bad, race[1].bad,
                    atomic_read(&race_frees[0]), atomic_read(&race_frees[1]), atomic_read(&race_wrong_free),
                    race_published, final.winner, final.loser);
    else pr_info("LUPOS_PRIMES_STRESS_RACE_OK input=%lu request=%zu allocations=2 distinct=1 winner=0 cancelled=1 winner_frees=0 loser_frees=1 stable=1 joined=2\n",
                 race_input, race_request);
    return ret;
}

static int __init prime_stress_init(void)
{
    struct snapshot initial, dynamic;
    const char *phase = "initial";
    int ret;

    with_primes(&initial, snapshot_callback);
    if (!initial.valid || initial.bits != BITS_PER_LONG || initial.last != (BITS_PER_LONG == 64 ? 61 : 31) ||
        !slow_is_prime_number(1) || slow_is_prime_number(0) || actual_is(1) || READ_ONCE(current->fail_nth))
        return -EINVAL;
    pr_info("LUPOS_PRIMES_STRESS_BEGIN width=%u initial_bits=%lu\n", BITS_PER_LONG, initial.bits);
    ret = register_trace_kmalloc(allocation_probe, NULL);
    if (ret) goto failed;
#define TRY(name, expression) do { phase = name; ret = (expression); if (ret) goto unregister; } while (0)
    TRY("cached", attempt("cached", 2, false, true, false));
    TRY("static_composite", attempt("static_composite", BITS_PER_LONG, false, true, true));
    TRY("static_prime", attempt("static_prime", prime_reference_next(BITS_PER_LONG), false, true, true));
    TRY("static_next", attempt("static_next", initial.last, true, true, true));
    TRY("retry_static", attempt("retry_static", BITS_PER_LONG, false, false, true));
    with_primes(&dynamic, snapshot_callback);
    if (!dynamic.valid) { ret = -EINVAL; goto unregister; }
    TRY("dynamic_next", attempt("dynamic_next", dynamic.last, true, true, true));
    TRY("retry_dynamic", attempt("retry_dynamic", dynamic.last, true, false, true));
    TRY("concurrency", concurrency());
    TRY("competing_writers", competing_writers());
unregister:
    WRITE_ONCE(current->fail_nth, 0);
    smp_store_release(&window.active, false);
    if (unregister_trace_kmalloc(allocation_probe, NULL)) ret = -EINVAL;
    tracepoint_synchronize_unregister();
    if (ret) goto failed;
    pr_info("LUPOS_PRIMES_STRESS_TRACE_OFF fail_nth=0\n");
    pr_info("LUPOS_PRIMES_STRESS_OK failures=4 retries=2 cached=1 held_reader=1\n");
    return 0;
failed:
    pr_err("LUPOS_PRIMES_STRESS_FAIL phase=%s error=%d\n", phase, ret);
    return ret;
}
static void __exit prime_stress_exit(void) {}
module_init(prime_stress_init);
module_exit(prime_stress_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Scoped native prime FAILSLAB and RCU reader test");
'''


def source():
    return SOURCE.replace("@PRIVATE_HEADER@", str(ROOT / "lib/math/prime_numbers_private.h"))


def required_configuration(build):
    config = configuration(build)
    arch = architecture(config)
    for option in ("PRIME_NUMBERS", "PRIME_NUMBERS_KUNIT_TEST"):
        if config.get(option) != "m": raise ValueError("fresh-cache stress requires CONFIG_" + option + "=m")
    for option in ("MODULES", "MODULE_UNLOAD", "PRINTK", "MULTIUSER", "SMP", "TRACEPOINTS", "DEBUG_FS",
                   "SYSFS", "FAULT_INJECTION", "FAILSLAB", "FAULT_INJECTION_DEBUG_FS"):
        if config.get(option) != "y": raise ValueError("stress requires CONFIG_" + option + "=y")
    if int(config.get("NR_CPUS", "0")) < 2: raise ValueError("stress requires at least two configured/online CPUs")
    if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("private stress module is unsigned")
    for option in ("FAIL_PAGE_ALLOC", "FAIL_FUTEX", "FAULT_INJECTION_USERCOPY", "FAIL_IO_TIMEOUT",
                   "FAIL_MAKE_REQUEST", "FAIL_FUNCTION", "FAIL_SUNRPC", "FAIL_MMC_REQUEST"):
        if config.get(option) == "y": raise ValueError("unrelated fault injector could consume per-task fail_nth: " + option)
    return config, arch


def compile_probe(build, directory):
    """Compile against actual saved native C flags, writing only directory.

    This is a private compile check, not a Kbuild or VM operation. In particular
    remove the original dependency-file flag, so the read-only build tree never
    receives a .d output, and replace both source and -o with private paths.
    """
    directory = Path(directory).resolve()
    build = Path(build).resolve()
    directory.mkdir(parents=True, exist_ok=True)
    destination = directory / (NAME + ".c")
    destination.write_text(source())
    saved = (build / "lib/kunit/.test.o.cmd").read_text().splitlines()[0]
    command = shlex.split(saved.split(" := ", 1)[1].split(";", 1)[0])
    if not command or not any(flag == "-c" for flag in command): raise ValueError("no real native C compile command")
    original = str(ROOT / "lib/kunit/test.c")
    if command.count(original) != 1 or command.count("-o") != 1: raise ValueError("unexpected original native compilation command")
    result = []
    skip = False
    for index, token in enumerate(command):
        if skip: skip = False; continue
        if token.startswith("-Wp,-MMD,") or token.startswith(("-DKBUILD_BASENAME=", "-DKBUILD_MODNAME=", "-D__KBUILD_MODNAME=")):
            continue
        if token == "-o":
            result += ["-o", str(directory / (NAME + ".o"))]; skip = True
        elif token == original: result.append(str(destination))
        else: result.append(token)
    result += ["-DMODULE", '-DKBUILD_BASENAME="prime_numbers_stress"', '-DKBUILD_MODNAME="prime_numbers_stress"',
               "-D__KBUILD_MODNAME=prime_numbers_stress"]
    subprocess.run(result, cwd=build, check=True, capture_output=True, timeout=120)
    return directory / (NAME + ".o")


def stress_records():
    return [b"LUPOS_PRIMES_STRESS_BEGIN width=64 initial_bits=64", *(
        f"LUPOS_PRIMES_STRESS_ALLOC name={name} request={size} seen={seen} null={nulls} fail_nth={nth} same={same}".encode()
        for name, size, seen, nulls, nth, same in (
            ("cached", 0, 0, 0, 1, 1), ("static_composite", 48, 1, 1, 0, 1),
            ("static_prime", 56, 1, 1, 0, 1), ("static_next", 48, 1, 1, 0, 1),
            ("retry_static", 48, 1, 0, 0, 0), ("dynamic_next", 64, 1, 1, 0, 1),
            ("retry_dynamic", 64, 1, 0, 0, 0))),
        b"LUPOS_PRIMES_STRESS_RCU_OK old_bits=256 new_bits=512 held=1 churn=8 joined=2",
        b"LUPOS_PRIMES_STRESS_RACE_OK input=512 request=160 allocations=2 distinct=1 winner=0 cancelled=1 winner_frees=0 loser_frees=1 stable=1 joined=2",
        b"LUPOS_PRIMES_STRESS_TRACE_OFF fail_nth=0",
        b"LUPOS_PRIMES_STRESS_OK failures=4 retries=2 cached=1 held_reader=1"]


def verify_console(console, *, framework_module=False, reload=True):
    if re.search(rb"CFI failure|BUG:|WARNING:|Oops:|Kernel panic|UBSAN:|KASAN:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|\bno (?:extended )?symbol version\b", console, re.I):
        raise ValueError("unexpected prime stress kernel fault")
    lines = [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).strip() for line in console.splitlines()]
    ranges = kunit_runs(console, 2 if reload else 1)
    preloads = 2 + int(framework_module)  # framework?, provider, stress; suite is main
    expected = [b"LUPOS_FAILSLAB_SETUP_OK"]
    for index in range(preloads):
        if index == preloads - 1: expected += stress_records()
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [b"KUNIT", b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for index in range(preloads):
            if index == preloads - 1: expected += stress_records()
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [b"KUNIT", f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(b"LUPOS_RUST_BUILD_BOOT_OK")
    actual = []
    for index, line in enumerate(lines):
        if any(index == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start <= index <= end for start, end in ranges): raise ValueError("loader/stress event inside unfinished KUnit")
            actual.append(line)
    if actual != expected: raise ValueError("missing, malformed, duplicated or reordered prime stress evidence")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return {"runs": 2 if reload else 1, "failures_per_run": 4, "held_readers_per_run": 1,
            "cancelled_writers_per_run": 1, "reclamation_proven": False}


def verify_probe(build, work, arch):
    obj, reference, module = work / (NAME + ".o"), work / "prime_reference.o", work / (NAME + ".ko")
    main = work / "prime_stress_main.o"
    for image in (obj, main, module): elf_target(image, arch)
    if module_name(module) != NAME: raise ValueError("wrong stress module name")
    verify_build_command(work, main, main.with_suffix(".c"), [ROOT / "include/trace/events/kmem.h",
        ROOT / "include/linux/prime_numbers.h", ROOT / "lib/math/prime_numbers_private.h", ROOT / "include/linux/sched.h"])
    verify_build_command(work, reference, reference.with_suffix(".c"), [ROOT / "include/linux/math.h"])
    if main.with_suffix(".c").read_text() != source() or reference.with_suffix(".c").read_text() != reference_source():
        raise ValueError("native stress fixture differs from audited template/original oracle")
    newer(reference, [ROOT / "lib/math/prime_numbers.c"])
    newer(module, [main, obj, reference, module.with_suffix(".mod.c")])
    require_metadata_field(metadata_fields(module), b"license", b"GPL")
    require_metadata_field(metadata_fields(module), b"description", b"Scoped native prime FAILSLAB and RCU reader test")
    if read_exports(module): raise ValueError("disposable stress module exports unexpected symbols")
    # The loader checks every final import, including unreferenced incidental
    # symbols. Selected APIs or generated-C text alone cannot establish this.
    verify_module_import_versions(build, module)
    undefined = {line.split()[-1] for line in tool("nm", "-u", module).splitlines() if line.split()}
    required = (*PUBLIC, *PRIVATE, "__tracepoint_kmalloc", "__tracepoint_kfree", "tracepoint_probe_register", "tracepoint_probe_unregister",
                "kthread_create_on_node", "kthread_bind", "kthread_stop", "synchronize_srcu")
    if not {symbol.encode() for symbol in required} <= undefined: raise ValueError("stress module omits required actual native APIs")
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    if configuration(build).get("MODVERSIONS") == "y":
        generated = module.with_suffix(".mod.c").read_text()
        for name in required:
            matches = [row for row in rows if len(row) >= 2 and row[1] == name.encode()]
            if len(matches) != 1 or imported_crc(generated, name) != matches[0][0].lower():
                raise ValueError("native stress API import CRC mismatch: " + name)
    if configuration(build).get("CFI") == "y":
        flags = compilation_flags(main)
        if "-fsanitize=kcfi" not in flags: raise ValueError("stress fixture lacks real KCFI flags")
        types = provider_type_ids(build / "lib/math/prime_numbers.o", names=("with_primes",))
        for image in (main, module):
            verify_guarded_calls(image, arch, types, wrappers=(("primes_call_with", "with_primes"),))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--reload-modules", action="store_true")
    args = parser.parse_args()
    build = args.build.resolve()
    try:
        config, arch = required_configuration(build)
        modules = verify_linked_implementation(build, "Rust" if config.get("RUST_PRIME_NUMBERS") == "y" else "C")
    except (OSError, ValueError, KeyError, IndexError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-prime-stress-test"
    work.mkdir(exist_ok=True)
    (work / "prime_stress_main.c").write_text(source())
    (work / "prime_reference.c").write_text(reference_source())
    (work / "Makefile").write_text(f"obj-m := {NAME}.o\n{NAME}-y := prime_stress_main.o prime_reference.o\n")
    env = {key: value for key, value in os.environ.items() if not key.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and key not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build), "M=" + str(work),
                    *args.make_arg, "CFLAGS_MODULE+=-D__DISABLE_EXPORTS", "modules"], check=True, env=env)
    verify_probe(build, work, arch)
    suite = modules[-1]
    if suite.name != "prime_numbers_kunit.ko": raise ValueError("stress requires modular original/selected suite last")
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build), "--arch", arch,
               "--prepare-failslab", "--module", str(suite), "--qemu", args.qemu or ("qemu-system-aarch64" if arch == "aarch64" else "qemu-system-x86_64")]
    for module in [*modules[:-1], work / (NAME + ".ko")]: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, check=True, env=env)
    result = verify_console((build / "rust-boot-test/console.log").read_bytes(), framework_module=config["KUNIT"] == "m", reload=args.reload_modules)
    print(f"Prime stress passed {result['runs']} fresh-cache runs: actual4 NULL allocations,2 retries, cached control, held reader/writer and one competing-writer cancellation per run; deferred reclamation not measured.")


if __name__ == "__main__":
    main()
