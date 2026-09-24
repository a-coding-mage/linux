# SPDX-License-Identifier: GPL-2.0-only
"""Strict native stress evidence/parser guards; all generated files are private."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import prime_numbers_stress as stress
from check_reciprocal_kernel import module_elf as structural_elf
from test_prime_numbers_runtime import suite


def console(framework=False, reload=True):
    preloads = 2 + int(framework)
    output = b"LUPOS_FAILSLAB_SETUP_OK\n"
    for index in range(preloads):
        if index == preloads - 1: output += b"\n".join(stress.stress_records()) + b"\n"
        output += f"LUPOS_RUST_PRELOAD_OK {index}\n".encode()
    output += suite() + b"LUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        output += b"".join(f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode() for i in reversed(range(preloads + 1)))
        for index in range(preloads):
            if index == preloads - 1: output += b"\n".join(stress.stress_records()) + b"\n"
            output += f"LUPOS_RUST_MODULE_RELOAD_OK {index}\n".encode()
        output += suite() + f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}\n".encode()
    return output + b"LUPOS_RUST_BUILD_BOOT_OK\n"


class StressConsoleTests(unittest.TestCase):
    def test_complete_scoped_failure_concurrency_and_reload_protocol(self):
        for framework in (False, True):
            for reload in (False, True):
                for timestamp in (False, True):
                    data = console(framework, reload)
                    if timestamp: data = b"\n".join(b"[  1.123456] " + line for line in data.splitlines()) + b"\n"
                    result = stress.verify_console(data, framework_module=framework, reload=reload)
                    self.assertEqual(result, dict(runs=2 if reload else 1, failures_per_run=4,
                                                held_readers_per_run=1, cancelled_writers_per_run=1,
                                                reclamation_proven=False))

    def test_every_missing_duplicate_or_reordered_stress_record_rejected(self):
        original = console(True)
        records = [line + b"\n" for line in stress.stress_records()]
        for record in records:
            for value in (original.replace(record, b"", 1), original.replace(record, record * 2, 1)):
                with self.subTest(record=record), self.assertRaises(ValueError): stress.verify_console(value, framework_module=True)
        for first, second in zip(records, records[1:]):
            value = original.replace(first + second, second + first, 1)
            with self.subTest(first=first), self.assertRaises(ValueError): stress.verify_console(value, framework_module=True)

    def test_observation_mutants_do_not_pass_as_real_allocator_failure(self):
        original = console()
        for before, after in ((b"request=48", b"request=56"), (b"seen=1 null=1", b"seen=0 null=1"),
                (b"null=1", b"null=0"), (b"fail_nth=0 same=1", b"fail_nth=1 same=1"),
                (b"fail_nth=0 same=1", b"fail_nth=0 same=0"),
                (b"request=0 seen=0 null=0 fail_nth=1", b"request=0 seen=0 null=0 fail_nth=0"),
                (b"retry_static request=48 seen=1 null=0", b"retry_static request=48 seen=1 null=1"),
                (b"dynamic_next request=64", b"dynamic_next request=48"),
                (b"initial_bits=64", b"initial_bits=65536"), (b"failures=4", b"failures=3")):
            with self.subTest(before=before), self.assertRaises(ValueError): stress.verify_console(original.replace(before, after, 1))

    def test_concurrency_and_cleanup_proof_fields_are_mandatory(self):
        original = console()
        for before, after in ((b"old_bits=256", b"old_bits=64"), (b"new_bits=512", b"new_bits=256"),
                (b"held=1", b"held=0"), (b"churn=8", b"churn=0"), (b"joined=2", b"joined=1"),
                (b"TRACE_OFF fail_nth=0", b"TRACE_OFF fail_nth=1")):
            with self.subTest(before=before), self.assertRaises(ValueError): stress.verify_console(original.replace(before, after, 1))

    def test_failslab_setup_once_before_any_module_action(self):
        original = console(True)
        setup = b"LUPOS_FAILSLAB_SETUP_OK\n"
        for data in (original.replace(setup, b""), setup + original,
                     original.replace(setup, b"").replace(b"LUPOS_RUST_PRELOAD_OK 0\n", b"LUPOS_RUST_PRELOAD_OK 0\n" + setup),
                     original.replace(setup, b"LUPOS_FAILSLAB_SETUP_OK unverified\n")):
            with self.assertRaises(ValueError): stress.verify_console(data, framework_module=True)

    def test_original_suite_and_exact_module_lifecycle_are_not_optional(self):
        original = console(True)
        for data in (original.replace(suite(), b"", 1), original.replace(b"ok 1 prime_numbers_test", b"not ok 1 prime_numbers_test", 1),
                     original.replace(b"LUPOS_RUST_MODULE_UNLOAD_OK 1\n", b""),
                     original.replace(b"LUPOS_RUST_MODULE_RELOAD_OK 1\n", b""),
                     original.replace(b"LUPOS_RUST_BUILD_BOOT_OK\n", b"")):
            with self.assertRaises(ValueError): stress.verify_console(data, framework_module=True)
        block = b"\n".join(stress.stress_records()) + b"\n"
        bad = original.replace(block, b"", 1).replace(suite(), suite() + block, 1)
        with self.assertRaises(ValueError): stress.verify_console(bad, framework_module=True)

    def test_raw_faults_and_extra_fail_markers_rejected_even_after_success(self):
        for failure in (b"BUG:", b"WARNING:", b"CFI failure", b"KASAN:", b"UBSAN:", b"Oops:",
                        b"Kernel panic", b"not ok 1 other", b"LUPOS_PRIMES_STRESS_FAIL phase=bad error=-22",
                        b"prime_numbers_stress: no symbol version for unexpected_import",
                        b"prime_numbers_stress: no extended symbol version for unexpected_import"):
            with self.subTest(failure=failure), self.assertRaises(ValueError): stress.verify_console(console() + failure + b"\n")

    def test_one_online_cpu_and_worker_failures_cannot_be_reported_as_success(self):
        original = console()
        success = next(line for line in stress.stress_records() if b"RCU_OK" in line)
        errors = (b"LUPOS_PRIMES_STRESS_CONCURRENCY_ERROR reason=online_cpus online=1 required=2",
                  b"LUPOS_PRIMES_STRESS_CONCURRENCY_ERROR reason=wait_workers online=2 reader_error=-110 writer_error=0")
        for error in errors:
            for value in (original.replace(success, error, 1), original + error + b"\n"):
                with self.subTest(error=error), self.assertRaises(ValueError): stress.verify_console(value)

    def test_two_writer_cancellation_requires_exact_observations(self):
        original = console(True)
        record = next(line for line in stress.stress_records() if b"RACE_OK" in line)
        for before, after in ((b"input=512", b"input=256"), (b"request=160", b"request=96"),
                (b"allocations=2", b"allocations=1"), (b"allocations=2", b"allocations=3"),
                (b"distinct=1", b"distinct=0"), (b"winner=0", b"winner=1"),
                (b"cancelled=1", b"cancelled=0"), (b"winner_frees=0", b"winner_frees=1"),
                (b"loser_frees=1", b"loser_frees=0"), (b"loser_frees=1", b"loser_frees=2"),
                (b"stable=1", b"stable=0"), (b"joined=2", b"joined=1")):
            with self.subTest(before=before, after=after), self.assertRaises(ValueError):
                stress.verify_console(original.replace(record, record.replace(before, after), 1), framework_module=True)
        for failure in (b"LUPOS_PRIMES_STRESS_RACE_ERROR reason=wait_workers error0=-110 error1=0",
                        b"LUPOS_PRIMES_STRESS_RACE_ERROR reason=cancellation_evidence frees0=0 frees1=0",
                        b"LUPOS_PRIMES_STRESS_RACE_ERROR reason=cancellation_evidence winner=0 loser=1"):
            for data in (original.replace(record, failure, 1), original + failure + b"\n"):
                with self.subTest(failure=failure), self.assertRaises(ValueError):
                    stress.verify_console(data, framework_module=True)

    def test_race_cannot_run_after_trace_cleanup_or_be_reused_across_reload(self):
        original = console()
        record = next(line for line in stress.stress_records() if b"RACE_OK" in line) + b"\n"
        cleanup = b"LUPOS_PRIMES_STRESS_TRACE_OFF fail_nth=0\n"
        for data in (original.replace(record + cleanup, cleanup + record, 1),
                     original.replace(record, b"", 1),
                     original.replace(record, b"", 1).replace(record, record * 2, 1)):
            with self.assertRaises(ValueError): stress.verify_console(data)


class StressFixtureTests(unittest.TestCase):
    def test_every_existing_snapshot_call_uses_the_same_volatile_native_wrapper(self):
        source = stress.source()
        wrapper = re.search(r"static noinline notrace void primes_call_with\([^}]+\}", source)[0]
        self.assertIn("void (*volatile target)(void *, primes_fn) = with_primes;", wrapper)
        self.assertIn("target(ctx, fn);", wrapper)
        self.assertEqual(source.count("#define with_primes primes_call_with"), 1)
        self.assertLess(source.index("#define with_primes primes_call_with"), source.index("with_primes(&before"))
        self.assertNotIn("#undef with_primes", source)
        self.assertEqual(len(re.findall(r"\bwith_primes\(", source)), 9)

    def test_actual_abi_callbacks_faultfield_and_no_mock_allocator(self):
        source = stress.source()
        self.assertIn(str(stress.ROOT / "lib/math/prime_numbers_private.h"), source)
        self.assertIn("#include <trace/events/kmem.h>", source)
        self.assertIn("register_trace_kmalloc(allocation_probe, NULL)", source)
        self.assertIn("flags != (GFP_KERNEL | __GFP_NOWARN)", source)
        self.assertIn("requested != window.request", source)
        self.assertIn("window.nulls += ptr == NULL", source)
        self.assertIn("current != READ_ONCE(window.task)", source)
        self.assertIn("!in_task()", source)
        self.assertNotIn("should_failslab(", source)
        self.assertIn('MODULE_LICENSE("GPL")', source)
        self.assertNotIn("EXPORT_SYMBOL", source)

    def test_injection_has_no_intervening_logging_or_allocations_and_disarms(self):
        source = stress.source()
        window = source[source.index("WRITE_ONCE(current->fail_nth, inject ? 1 : 0)"):
                        source.index("/* Every diagnostic")]
        for token in ("kmalloc", "pr_info", "pr_err", "with_primes", "return", "goto"):
            self.assertNotIn(token, window)
        self.assertRegex(window, r"actual = next \? actual_next\(x\) : actual_is\(x\);")
        self.assertIn("remaining = READ_ONCE(current->fail_nth)", window)
        self.assertIn("WRITE_ONCE(current->fail_nth, 0)", window)
        self.assertRegex(source, r"unregister:\s*WRITE_ONCE\(current->fail_nth, 0\)")

    def test_threads_have_bounded_atomic_reader_and_join_before_unregistration(self):
        source = stress.source()
        held = source[source.index("static void held_reader"):source.index("static int reader_thread")]
        for token in ("wait_for_completion", "kmalloc", "msleep", "schedule(", "cond_resched"):
            self.assertNotIn(token, held)
        self.assertIn("time_before(jiffies, deadline)", held)
        self.assertIn("smp_load_acquire(&writer_published)", held)
        self.assertIn("same_snapshot(&old_cache, &after)", held)
        self.assertIn("get_task_struct(reader)", source)
        self.assertIn("get_task_struct(writer)", source)
        self.assertIn("kthread_stop(reader); put_task_struct(reader)", source)
        self.assertIn("kthread_stop(writer); put_task_struct(writer)", source)
        main = source[source.index("static int __init prime_stress_init"):]
        self.assertLess(main.index('TRY("concurrency", concurrency())'), main.index("unregister_trace_kmalloc"))
        self.assertLess(main.index('TRY("competing_writers", competing_writers())'), main.index("unregister_trace_kmalloc"))
        self.assertLess(main.index("unregister_trace_kmalloc"), main.index("tracepoint_synchronize_unregister()"))
        self.assertLess(main.index("tracepoint_synchronize_unregister()"), main.index("STRESS_TRACE_OFF"))

    def test_runtime_cpu_check_and_post_join_diagnostics_are_preserved(self):
        source = stress.source()
        self.assertIn("online = num_online_cpus();", source)
        self.assertRegex(source, r"if \(online < 2\) \{\s*pr_err\(\"LUPOS_PRIMES_STRESS_CONCURRENCY_ERROR reason=online_cpus online=%u required=2")
        self.assertLess(source.index("if (online < 2)"), source.index("kthread_create(reader_thread"))
        report = source.index("report:\n")
        self.assertLess(source.index("kthread_stop(reader)"), report)
        self.assertLess(source.index("kthread_stop(writer)"), report)
        for state in ("publication_timeout", "compare_held_cache", "wait_reader", "invalid_old_cache", "invalid_new_cache", "churn_allocation"):
            self.assertIn('"' + state + '"', source)

    def test_snapshot_checks_original_slow_one_and_never_copies_rcu_header(self):
        source = stress.source()
        capture = source[source.index("static void capture"):source.index("static void snapshot_callback")]
        self.assertIn("x > 1 && prime_reference_is(x)", capture)
        self.assertIn("bitmap_copy(out->words, p->primes, out->bits)", capture)
        self.assertNotIn("p->rcu", capture)
        self.assertIn("!slow_is_prime_number(1)", source)
        self.assertIn("initial.bits != BITS_PER_LONG", source)
        self.assertIn("after.bits != new_bits", source)

    def test_race_callbacks_only_observe_exact_real_tasks_allocations_and_frees(self):
        source = stress.source()
        task = source[source.index("static int race_task"):source.index("static void race_allocation_probe")]
        self.assertIn("!smp_load_acquire(&race_enabled) || !in_task()", task)
        self.assertIn("current == READ_ONCE(race[i].task) && READ_ONCE(race[i].in_provider)", task)
        probes = source[source.index("static void race_allocation_probe"):source.index("static void race_snapshot")]
        for token in ("wait_for_completion", "msleep", "schedule(", "cond_resched", "pr_info(", "pr_err(",
                      "kmalloc(", "kfree(", "actual_next(", "WRITE_ONCE(current->fail_nth"):
            # Ignore comments explaining why null free notifications are excluded.
            self.assertNotIn(token, re.sub(r"/\*.*?\*/", "", probes, flags=re.S))
        for token in ("++worker->allocations != 1", "requested != race_request", "allocated < requested",
                      "flags != (GFP_KERNEL | __GFP_NOWARN)", "READ_ONCE(current->fail_nth)",
                      "time_before(jiffies, deadline)", "atomic_read(&race_abort)",
                      "smp_load_acquire(&race_published)", "smp_load_acquire(&race[1].arrived)",
                      "if (!smp_load_acquire(&race_enabled) || !ptr) return", "(unsigned long)ptr != READ_ONCE(race[i].allocation)",
                      "if (i != 1 || index != 1) atomic_inc(&race_wrong_free)"):
            self.assertIn(token, probes)
        free = probes[probes.index("static void race_free_probe"):]
        self.assertNotIn("if (index < 0", free)
        self.assertNotIn("!in_task()", free)

    def test_race_requires_actual_publication_before_releasing_loser(self):
        source = stress.source()
        snapshot = source[source.index("static void race_snapshot"):source.index("static int race_thread")]
        self.assertIn("capture(p, &worker->cache)", snapshot)
        self.assertIn("worker->winner = (unsigned long)p == READ_ONCE(race[0].allocation)", snapshot)
        self.assertIn("worker->loser = (unsigned long)p == READ_ONCE(race[1].allocation)", snapshot)
        worker = source[source.index("static int race_thread"):source.index("static int competing_writers")]
        ordered = ["WRITE_ONCE(worker->in_provider, true)", "worker->value = actual_next(race_input)",
                   "WRITE_ONCE(worker->in_provider, false)", "with_primes(worker, race_snapshot)",
                   "!worker->winner || worker->loser", "worker->status = 0",
                   "if (first) smp_store_release(&race_published, true)"]
        self.assertEqual([worker.index(token) for token in ordered], sorted(worker.index(token) for token in ordered))
        self.assertIn("prime_reference_next(race_input)", worker)
        self.assertIn("!same_snapshot(&race[0].cache, &worker->cache)", worker)
        self.assertNotIn("WRITE_ONCE(current->fail_nth", worker)

    def test_race_start_gate_join_and_trace_cleanup_cover_partial_setup(self):
        source = stress.source()
        race = source[source.index("static int competing_writers"):source.index("static int __init prime_stress_init")]
        self.assertLess(race.index("wake_up_process(race[i].task)"), race.index("complete_all(&race_start_gate)"))
        self.assertIn("get_task_struct(task)", race)
        self.assertIn("kthread_bind(task, cpus[i])", race)
        cleanup = race[race.index("stop:\n"):]
        ordered = ["atomic_set(&race_abort, 1)", "complete_all(&race_start_gate)",
                   "kthread_stop(race[i].task)", "smp_store_release(&race_enabled, false)",
                   "cpus_read_unlock()",
                   "unregister_trace_kfree", "unregister_trace_kmalloc", "tracepoint_synchronize_unregister()",
                   "put_task_struct(race[i].task)", "with_primes(&final, race_snapshot)"]
        self.assertEqual([cleanup.index(token) for token in ordered], sorted(cleanup.index(token) for token in ordered))
        self.assertIn("if (ret) goto unregister;\n    registered_free = true;", race)
        self.assertIn("if (IS_ERR(task)) { ret = PTR_ERR(task); goto stop; }", race)
        for token in ("race[0].allocation == race[1].allocation", "atomic_read(&race_frees[0])",
                      "atomic_read(&race_frees[1]) != 1", "atomic_read(&race_wrong_free)",
                      "!final.winner || final.loser", "!same_snapshot(&race[0].cache, &final.cache)"):
            self.assertIn(token, cleanup)
        self.assertNotRegex(race, r"\bkfree\s*\(")
        self.assertNotRegex(race, r"\(struct primes\s*\*\)")

    def test_tracepoint_updates_never_run_under_cpu_hotplug_lock(self):
        source = stress.source()
        race = source[source.index("static int competing_writers"):source.index("static int __init prime_stress_init")]
        acquired, released = race.index("cpus_read_lock();"), race.index("cpus_read_unlock();")
        self.assertEqual(race.count("cpus_read_lock();"), 1)
        self.assertEqual(race.count("cpus_read_unlock();"), 1)
        self.assertLess(race.index("register_trace_kmalloc(race_allocation_probe"), acquired)
        self.assertLess(race.index("register_trace_kfree(race_free_probe"), acquired)
        self.assertLess(acquired, race.index("num_online_cpus()"))
        self.assertLess(acquired, race.index("kthread_create(race_thread"))
        self.assertLess(race.index("kthread_stop(race[i].task)"), released)
        self.assertLess(released, race.index("unregister_trace_kfree"))
        self.assertLess(released, race.index("unregister_trace_kmalloc"))
        self.assertLess(released, race.index("tracepoint_synchronize_unregister()"))
        # Register failures never unlock an unheld CPU lock; partial success
        # still unregisters the allocation probe. Every locked error joins and
        # unlocks first, including zero created threads/insufficient CPUs.
        self.assertRegex(race, r"register_trace_kmalloc\(race_allocation_probe, NULL\);\s*if \(ret\) goto report;")
        self.assertRegex(race, r"register_trace_kfree\(race_free_probe, NULL\);\s*if \(ret\) goto unregister;")
        self.assertRegex(race, r"ret = -EINVAL;\s*if \(num_online_cpus\(\) < 2\) goto stop;")
        self.assertLess(released, race.index("unregister:\n"))
        self.assertIn("if (IS_ERR(task)) { ret = PTR_ERR(task); goto stop; }", race)
        locked = race[acquired:released]
        self.assertNotRegex(locked, r"goto (?:unregister|report);|\breturn\b")


class StressPrivateOutputTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="prime-stress-tests-")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)

    def build(self, **overrides):
        path = Path(tempfile.mkdtemp(dir=self.directory))
        config = dict(X86_64="y", RUST="y", RUST_PRIME_NUMBERS="y", PRIME_NUMBERS="m", PRIME_NUMBERS_KUNIT_TEST="m",
                      KUNIT="m", NR_CPUS="4")
        for name in ("64BIT", "MODULES", "MODULE_UNLOAD", "PRINTK", "MULTIUSER", "SMP", "TRACEPOINTS", "DEBUG_FS",
                     "SYSFS", "FAULT_INJECTION", "FAILSLAB", "FAULT_INJECTION_DEBUG_FS"):
            config[name] = "y"
        config.update(overrides)
        (path / ".config").write_text("".join(f"CONFIG_{name}={value}\n" for name, value in config.items()))
        return path, config

    def protected_wrapper(self, arch):
        """Compile the actual wrapper and original C callback-bearing type."""
        work = self.directory / arch
        work.mkdir()
        include = work / "include/linux"
        include.mkdir(parents=True)
        original_types = (stress.ROOT / "include/linux/types.h").read_text()
        rcu = re.search(r"struct callback_head \{.*?\} __attribute__\(\(aligned\(sizeof\(void \*\)\)\)\);",
                        original_types, re.S)[0]
        (include / "types.h").write_text("typedef _Bool bool;\n#define rcu_head callback_head\n" + rcu)
        prefix = '#define IS_ENABLED(x) 1\n#include "' + str(stress.ROOT / "lib/math/prime_numbers_private.h") + '"\n'
        prefix += '#define noinline __attribute__((noinline))\n#define notrace __attribute__((no_instrument_function))\n'
        wrapper = re.search(r"static noinline notrace void primes_call_with\([^}]+\}", stress.source())[0]
        source = work / "caller.c"
        source.write_text(prefix + wrapper +
            "\nvoid probe(void *ctx,primes_fn fn){primes_call_with(ctx,fn);}\n"
            '__attribute__((section(".init.text"))) void init_probe(void *ctx,primes_fn fn){\n'
            "void (*volatile target)(void *,primes_fn)=with_primes;target(ctx,fn);}\n")
        original = (stress.ROOT / "lib/math/prime_numbers.c").read_text()
        body = re.search(r"void with_primes\(void \*ctx, primes_fn fn\)\n\{.*?\n\}", original, re.S)[0]
        oracle = work / "type.c"
        # Original function/header are used only as a type oracle here; this
        # object is never run as a replacement cache or RCU implementation.
        oracle.write_text(prefix + "static const struct primes *primes;\n"
            "void rcu_read_lock(void); void rcu_read_unlock(void);\n#define rcu_dereference(p) (p)\n" + body)
        flags = [*shlex.split(os.environ.get("CLANG", "clang")), "--target=" +
            ("x86_64-linux-gnu" if arch == "x86_64" else "aarch64-linux-gnu"),
            "-I"+str(include.parent), "-O2", "-ffreestanding", "-fno-pic", "-fno-pie",
            "-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        if arch == "x86_64": flags += ["-pg", "-mfentry"]
        for path in (source, oracle):
            subprocess.run([*flags, "-c", path, "-o", path.with_suffix(".o")], check=True, capture_output=True)
        obj, linked = source.with_suffix(".o"), work / "caller.ko"
        if arch == "x86_64":
            # Native stress has guards in both init and persistent text. LLVM
            # emits a distinct SHF_LINK_ORDER trap section for each, not one.
            traps = [s for s in structural_elf(obj)[1] if s[0] == b".kcfi_traps"]
            self.assertEqual(len(traps),2)
        subprocess.run([*shlex.split(os.environ.get("LD_LLD", "ld.lld")), "-r", obj, "-o", linked], check=True, capture_output=True)
        return obj, linked, stress.provider_type_ids(oracle.with_suffix(".o"), names=("with_primes",))

    def test_real_wrapper_and_linked_module_retain_typed_outer_guards_both_architectures(self):
        for arch in ("x86_64", "aarch64"):
            obj, linked, types = self.protected_wrapper(arch)
            for path in (obj, linked):
                with self.subTest(arch=arch,path=path.name):
                    stress.verify_guarded_calls(path,arch,types,wrappers=(("primes_call_with","with_primes"),))

    def test_outer_guard_trap_type_branch_import_and_register_mutants_are_rejected(self):
        for arch in ("x86_64", "aarch64"):
            obj, linked, types = self.protected_wrapper(arch)
            for path in (obj, linked):
                assembly = subprocess.run([*shlex.split(os.environ.get("LLVM_OBJDUMP","llvm-objdump")),
                    "-dr", "--no-show-raw-insn", path], check=True, capture_output=True).stdout
                block = re.search(rb"(?ms)^[0-9a-f]+ <primes_call_with>:\n.*?(?=^[0-9a-f]+ <|^Disassembly|\Z)",assembly)[0]
                if arch == "x86_64":
                    changes = ((rb"\bud2\b",b"nop"),(rb"\bje\b",b"jne"),
                        (rb"-0x4\(",b"-0x8("),(rb"\*%r\w+",b"*%r15"),
                        (rb"\$0x[0-9a-f]+, %r10d",b"$0x1, %r10d"))
                else:
                    changes = ((rb"\bbrk\b",b"nop"),(rb"\bb.eq\b",b"b.ne"),
                        (rb"#-0x4",b"#-0x8"),(rb"\b(?:blr|br)\s+x\d+",b"blr x15"),
                        (rb"\bmovk\s+w\d+, #0x[0-9a-f]+",b"movk w17, #0x1"))
                changes += ((rb"\bwith_primes\b",b"wrong_import"),)
                for pattern, replacement in changes:
                    changed, count = re.subn(pattern,replacement,block,count=1)
                    self.assertEqual(count,1)
                    with self.subTest(arch=arch,path=path.name,pattern=pattern), \
                            mock.patch.object(stress.subprocess,"run",return_value=subprocess.CompletedProcess([],0,stdout=assembly.replace(block,changed,1))), \
                            self.assertRaises(ValueError):
                        stress.verify_guarded_calls(path,arch,types,wrappers=(("primes_call_with","with_primes"),))

    def test_every_split_trap_section_requires_complete_records_and_executable_link(self):
        obj, _, types = self.protected_wrapper("x86_64")
        original = structural_elf(obj)
        indices = [i for i, section in enumerate(original[1]) if section[0] == b".kcfi_traps"]
        self.assertEqual(len(indices),2)
        for index in indices:
            for field,value in ((1,8),(2,0),(2,0x83),(4,0),(4,3),(5,0),(5,len(original[1])),(5,index)):
                changed = list(original)
                sections = list(changed[1])
                trap = list(sections[index])
                trap[field] = value
                sections[index] = tuple(trap)
                changed[1] = sections
                with self.subTest(section=index,field=field,value=value), \
                        mock.patch("check_prime_numbers_kernel.structural_elf",return_value=changed), \
                        self.assertRaises(ValueError):
                    stress.verify_guarded_calls(obj,"x86_64",types,wrappers=(("primes_call_with","with_primes"),))

    def test_required_configuration_fails_closed_without_warm_cache_fallback(self):
        build, _ = self.build()
        self.assertEqual(stress.required_configuration(build)[1], "x86_64")
        build, _ = self.build(X86_64="n", ARM64="y")
        self.assertEqual(stress.required_configuration(build)[1], "aarch64")
        for option, value in (("PRIME_NUMBERS", "y"), ("PRIME_NUMBERS_KUNIT_TEST", "y"), ("FAILSLAB", "n"),
                ("FAULT_INJECTION_DEBUG_FS", "n"), ("TRACEPOINTS", "n"), ("SMP", "n"), ("NR_CPUS", "1"),
                ("FAIL_PAGE_ALLOC", "y"), ("FAULT_INJECTION_USERCOPY", "y"), ("MODULE_SIG_FORCE", "y")):
            build, _ = self.build(**{option: value})
            with self.subTest(option=option), self.assertRaises(ValueError): stress.required_configuration(build)

    def test_compile_probe_redirects_all_outputs_away_from_native_build(self):
        build, _ = self.build()
        cmd = build / "lib/kunit/.test.o.cmd"; cmd.parent.mkdir(parents=True)
        original = stress.ROOT / "lib/kunit/test.c"
        cmd.write_text(f'savedcmd_lib/kunit/test.o := clang -Wp,-MMD,lib/kunit/.test.o.d -I./include -c -o lib/kunit/test.o {original} ; objtool lib/kunit/test.o\n')
        saved = cmd.read_bytes()
        output = self.directory / "private-compile"
        with mock.patch.object(stress.subprocess, "run") as run:
            result = stress.compile_probe(build, output)
        self.assertEqual(result, output / "prime_numbers_stress.o")
        arguments = run.call_args.args[0]
        self.assertEqual(run.call_args.kwargs["cwd"], build)
        self.assertNotIn(str(original), arguments)
        self.assertNotIn("lib/kunit/test.o", arguments)
        self.assertFalse(any("-MMD" in arg or "objtool" in arg for arg in arguments))
        self.assertIn(str(result), arguments)
        self.assertEqual((output / "prime_numbers_stress.c").read_text(), stress.source())
        self.assertEqual(cmd.read_bytes(), saved)
        self.assertFalse((build / "lib/kunit/.test.o.d").exists())

    def test_cli_requests_scoped_setup_and_places_stress_before_suite(self):
        for framework in ("y", "m"):
            build, config = self.build(KUNIT=framework)
            modules = [build / "lib/math/prime_numbers.ko", build / "lib/math/tests/prime_numbers_kunit.ko"]
            if framework == "m": modules.insert(0, build / "lib/kunit/kunit.ko")
            calls = []
            def run(command, **kwargs):
                calls.append(command)
                if command[0] == sys.executable:
                    path = build / "rust-boot-test/console.log"; path.parent.mkdir()
                    path.write_bytes(console(framework == "m"))
                return subprocess.CompletedProcess(command, 0)
            with mock.patch.object(sys, "argv", ["stress", str(build), "--reload-modules", "--make-arg=LLVM=1"]), \
                    mock.patch.object(stress, "verify_linked_implementation", return_value=modules), \
                    mock.patch.object(stress, "verify_probe"), mock.patch.object(stress.subprocess, "run", side_effect=run), \
                    redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
                stress.main()
            self.assertIn("--prepare-failslab", calls[1])
            self.assertIn("--reload-modules", calls[1])
            self.assertEqual(calls[1][calls[1].index("--module") + 1], str(modules[-1]))
            preloads = [calls[1][i + 1] for i, value in enumerate(calls[1]) if value == "--preload-module"]
            self.assertEqual(preloads, [*map(str, modules[:-1]), str(build / "rust-prime-stress-test/prime_numbers_stress.ko")])
            self.assertIn("LLVM=1", calls[0])
            self.assertEqual((build / "rust-prime-stress-test/prime_reference.c").read_text(), stress.reference_source())

    def test_real_kfree_tracepoint_import_is_required_in_linked_probe(self):
        build, _ = self.build(CFI="y")
        work = self.directory / "probe"; work.mkdir()
        (work / "prime_stress_main.c").write_text(stress.source())
        (work / "prime_reference.c").write_text(stress.reference_source())
        (build / "Module.symvers").write_text("")
        imports = (*stress.PUBLIC, *stress.PRIVATE, "__tracepoint_kmalloc", "__tracepoint_kfree",
                   "tracepoint_probe_register", "tracepoint_probe_unregister", "kthread_create_on_node",
                   "kthread_bind", "kthread_stop", "synchronize_srcu")
        with mock.patch.object(stress, "elf_target"), mock.patch.object(stress, "module_name", return_value=stress.NAME), \
                mock.patch.object(stress, "verify_build_command"), mock.patch.object(stress, "newer"), \
                mock.patch.object(stress, "metadata_fields"), mock.patch.object(stress, "require_metadata_field"), \
                mock.patch.object(stress, "verify_module_import_versions") as versions, \
                mock.patch.object(stress, "compilation_flags", return_value=["-fsanitize=kcfi"]), \
                mock.patch.object(stress, "provider_type_ids", return_value={"with_primes": 1}) as types, \
                mock.patch.object(stress, "verify_guarded_calls") as guards, \
                mock.patch.object(stress, "read_exports", return_value=[]), mock.patch.object(stress, "tool") as tool:
            tool.return_value = b"".join(b"U " + name.encode() + b"\n" for name in imports)
            stress.verify_probe(build, work, "x86_64")
            versions.assert_called_once_with(build, work / "prime_numbers_stress.ko")
            types.assert_called_once_with(build / "lib/math/prime_numbers.o", names=("with_primes",))
            self.assertEqual(guards.call_args_list, [mock.call(path, "x86_64", {"with_primes": 1},
                wrappers=(("primes_call_with", "with_primes"),)) for path in
                (work / "prime_stress_main.o", work / "prime_numbers_stress.ko")])
            tool.return_value = tool.return_value.replace(b"U __tracepoint_kfree\n", b"")
            with self.assertRaisesRegex(ValueError, "actual native APIs"):
                stress.verify_probe(build, work, "x86_64")
            # Even a complete required-API list cannot excuse an additional
            # unversioned final ELF import rejected by the shared real parser.
            tool.return_value = b"".join(b"U " + name.encode() + b"\n" for name in imports)
            versions.side_effect = ValueError("unversioned final module import: incidental_orphan")
            with self.assertRaisesRegex(ValueError, "incidental_orphan"):
                stress.verify_probe(build, work, "x86_64")

    def test_matching_free_from_other_task_or_interrupt_is_observed(self):
        # Execute the exact observer body with a deterministic single-threaded
        # bookkeeping harness. This tests its filtering, not kernel atomics,
        # tracepoint ABI or memory ordering (which the native run must prove).
        source = stress.source()
        observer = source[source.index("static void race_free_probe"):source.index("static void race_snapshot")]
        fixture = r'''
#include <stdbool.h>
#include <stddef.h>
#define ARRAY_SIZE(x) (sizeof(x) / sizeof((x)[0]))
#define READ_ONCE(x) (x)
#define smp_load_acquire(p) (*(p))
typedef unsigned int atomic_t;
static void atomic_inc(atomic_t *value) { ++*value; }
static struct { unsigned long allocation; } race[2] = {{0x1000}, {0x2000}};
static atomic_t race_frees[2], race_wrong_free;
static bool race_enabled;
static int context;
/* -1 models another task, an interrupt or a worker outside its provider call. */
static int race_task(void) { return context; }
''' + observer + r'''
int main(void)
{
    int who;
    for (who = -1; who <= 1; who++) {
        context = who;
        race_enabled = true;
        race_frees[0] = race_frees[1] = race_wrong_free = 0;
        race_free_probe(NULL, 0, (const void *)0x1000UL);
        if (race_frees[0] != 1 || race_frees[1] || race_wrong_free != 1) return 1;
        race_free_probe(NULL, 0, (const void *)0x2000UL);
        if (race_frees[0] != 1 || race_frees[1] != 1 || race_wrong_free != (who == 1 ? 1U : 2U)) return 2;
        race_enabled = false;
        race_free_probe(NULL, 0, (const void *)0x1000UL);
        race_free_probe(NULL, 0, (const void *)0x2000UL);
        if (race_frees[0] != 1 || race_frees[1] != 1) return 3;
        race_enabled = true;
        race_free_probe(NULL, 0, NULL);
        race_free_probe(NULL, 0, (const void *)0x3000UL);
        if (race_frees[0] != 1 || race_frees[1] != 1 || race_wrong_free != (who == 1 ? 1U : 2U)) return 4;
    }
    return 0;
}
'''
        path = self.directory / "observer.c"
        path.write_text(fixture)
        executable = self.directory / "observer"
        subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-std=gnu11", "-Wall", "-Wextra",
                        "-Werror", "-Wno-unused-parameter", str(path), "-o", str(executable)],
                       check=True, capture_output=True, timeout=30)
        subprocess.run([str(executable)], check=True, capture_output=True, timeout=10)


if __name__ == "__main__":
    unittest.main()
