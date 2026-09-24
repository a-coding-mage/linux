#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Focused fail-closed list_sort runtime protocol and real-fixture checks."""

from contextlib import redirect_stderr
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

import check_list_sort_kernel as check

ROOT = Path(os.environ.get("LIST_SORT_SOURCE_ROOT", Path(__file__).resolve().parents[2])).resolve()
check.ROOT = ROOT


def suite_log():
    return (b"    KTAP version 1\n    # Subtest: list_sort\n    # module: test_list_sort\n"
            b"    1..1\n    ok 1 list_sort_test\nok 12 list_sort\n")


def console(caller="c", suite="y", framework=False, reload=False):
    output = suite_log() if suite == "y" else b""
    preloads = int(framework) + int(suite == "m")
    for index in range(preloads):
        if index == preloads - 1: output += suite_log()
        output += f"LUPOS_RUST_PRELOAD_OK {index}\n".encode()
    output += check.result_marker(caller) + b"\nLUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        for index in reversed(range(preloads + 1)):
            output += f"LUPOS_RUST_MODULE_UNLOAD_OK {index}\n".encode()
        for index in range(preloads):
            if index == preloads - 1: output += suite_log()
            output += f"LUPOS_RUST_MODULE_RELOAD_OK {index}\n".encode()
        output += check.result_marker(caller) + f"\nLUPOS_RUST_MODULE_RELOAD_OK {preloads}\n".encode()
    return output + b"LUPOS_RUST_BUILD_BOOT_OK\n"


class ProtocolTests(unittest.TestCase):
    def test_every_language_lifecycle_framework_state(self):
        for caller in ("c", "rust"):
            for state in ("n", "y", "m"):
                for framework in ((False, True) if state == "m" else (False,)):
                    for reload in (False, True):
                        with self.subTest(caller=caller, suite=state, framework=framework, reload=reload):
                            self.assertEqual(check.verify_console(console(caller, state, framework, reload), caller,
                                suite=state, framework_module=framework, reload=reload),
                                0 if state == "n" else 2 if state == "m" and reload else 1)

    def test_timestamp_and_exact_caller_prefixes(self):
        for caller in ("c", "rust"):
            data = console(caller).replace(check.result_marker(caller),
                (b"list_sort_rust_abi: " if caller == "rust" else b"list_sort_abi: ") + check.result_marker(caller))
            data = b"".join(b"[    1.234567] " + line + b"\n" for line in data.splitlines())
            self.assertEqual(check.verify_console(data, caller), 1)

    def test_missing_duplicate_reordered_wrong_case_plan(self):
        source = console()
        variants = [source.replace(b"1..1", b"1..2"), source.replace(b"    1..1\n", b""),
                    source.replace(b"    1..1\n", b"    1..1\n    1..1\n"),
                    source.replace(b"    1..1\n    ok 1 list_sort_test", b"    ok 1 list_sort_test\n    1..1"),
                    source.replace(b"ok 1 list_sort_test", b"ok 2 list_sort_test"),
                    source.replace(b"ok 1 list_sort_test", b"ok 1 list_sort_test # SKIP unavailable"),
                    source.replace(b"ok 1 list_sort_test", b"ok 1 list_sort_test\n    ok 1 list_sort_test"),
                    source.replace(b"ok 1 list_sort_test", b"ok 1 wrong_case"),
                    source.replace(b"# module: test_list_sort", b"# module: list_sort"),
                    source.replace(b"ok 12 list_sort\n", b""), source + suite_log(),
                    source + b"ok 1 list_sort_test\n"]
        for variant in variants:
            with self.subTest(variant=variant):
                with self.assertRaises(ValueError): check.verify_console(variant, "c")

    def test_optional_totals_exact_and_ordered(self):
        source = console().replace(b"    ok 1 list_sort_test", b"    # list_sort_test: pass:1 fail:0 skip:0 total:1\n    ok 1 list_sort_test")
        source = source.replace(b"ok 12 list_sort", b"# list_sort: pass:1 fail:0 skip:0 total:1\n# Totals: pass:1 fail:0 skip:0 total:1\nok 12 list_sort")
        self.assertEqual(check.verify_console(source, "c"), 1)
        for old, new in ((b"pass:1", b"pass:2"), (b"fail:0", b"fail:1"), (b"total:1", b"total:2"),
                         (b"skip:0", b"skip:1"), (b"# Totals:", b"# Totals: bad ")):
            with self.assertRaises(ValueError): check.verify_console(source.replace(old, new, 1), "c")
        for label in (b"list_sort_test", b"list_sort", b"Totals"):
            line = b"# " + label + b": pass:1 fail:0 skip:0 total:1"
            with self.assertRaises(ValueError): check.verify_console(source.replace(line, line + b"\n" + line), "c")

    def test_every_protocol_event_is_required_once_in_order(self):
        for caller in ("c", "rust"):
            source = console(caller, "m", True, True)
            events = [line for line in source.splitlines() if b"LUPOS_" in line]
            for line in events:
                with self.subTest(line=line):
                    with self.assertRaises(ValueError):
                        check.verify_console(source.replace(line + b"\n", b"", 1), caller, suite="m", framework_module=True, reload=True)
                    with self.assertRaises(ValueError):
                        check.verify_console(source.replace(line, line + b"\n" + line, 1), caller, suite="m", framework_module=True, reload=True)
            wrong = source.replace(b"LUPOS_RUST_MODULE_UNLOAD_OK 2\nLUPOS_RUST_MODULE_UNLOAD_OK 1",
                                   b"LUPOS_RUST_MODULE_UNLOAD_OK 1\nLUPOS_RUST_MODULE_UNLOAD_OK 2")
            with self.assertRaises(ValueError): check.verify_console(wrong, caller, suite="m", framework_module=True, reload=True)
            with self.assertRaises(ValueError): check.verify_console(source, "rust" if caller == "c" else "c", suite="m", framework_module=True, reload=True)

    def test_suite_must_finish_before_loader_event(self):
        source = console(suite="m", framework=True)
        wrong = source.replace(b"ok 12 list_sort\nLUPOS_RUST_PRELOAD_OK 1", b"LUPOS_RUST_PRELOAD_OK 1\nok 12 list_sort")
        with self.assertRaises(ValueError): check.verify_console(wrong, "c", suite="m", framework_module=True)
        for marker in (b"LUPOS_RUST_PRELOAD_OK 0", check.result_marker("c")):
            wrong = console().replace(b"    ok 1 list_sort_test", marker + b"\n    ok 1 list_sort_test")
            with self.assertRaises(ValueError): check.verify_console(wrong, "c")

    def test_raw_faults_and_malformed_success_are_rejected(self):
        for fault in (b"BUG:", b"WARNING:", b"Oops:", b"Kernel panic", b"KASAN:", b"UBSAN:", b"CFI failure",
                      b"EXPECTATION FAILED", b"ASSERTION FAILED", b"not ok 2 other", b"no symbol version",
                      b"no extended symbol version", b"general protection fault", b"Internal error:"):
            with self.subTest(fault=fault):
                with self.assertRaises(ValueError): check.verify_console(console() + fault + b" injected\n", "c")
        for old, new in ((b"callbacks=exact", b"callbacks=partial"), (b"stable=yes", b"stable=no"),
                         (str(check.COUNT).encode(), b"0"), (b"LUPOS_LIST_SORT_ABI_OK", b"LUPOS_LIST_SORT_FAIL")):
            with self.assertRaises(ValueError): check.verify_console(console().replace(old, new), "c")
        with self.assertRaises(ValueError):
            check.verify_console(console() + b"not ok\n** replaying previous printk message **\nnot okay harmless\n", "c")

    def test_exact_replay_transport_only(self):
        marker = check.result_marker("c")
        replay = console().replace(marker + b"\n", marker[:20] + b"\n** replaying previous printk message **\n" + marker + b"\n")
        self.assertEqual(check.verify_console(replay, "c"), 1)
        with self.assertRaises(ValueError): check.verify_console(replay.replace(marker[:20], b"different", 1), "c")

    def test_configuration_states(self):
        for provider in ("n", "y"):
            for selected in ("n", "y"):
                for framework, suite in (("n", "n"), ("y", "n"), ("y", "y"), ("y", "m"), ("m", "m")):
                    self.assertEqual(check.states(dict(RUST="y", RUST_LIST_SORT=provider, RUST_LIST_SORT_KUNIT_TEST=selected,
                                                       KUNIT=framework, TEST_LIST_SORT=suite)), (suite, framework))
        for config in (dict(KUNIT="m", TEST_LIST_SORT="y"), dict(TEST_LIST_SORT="m"),
                       dict(TEST_LIST_SORT="wrong"), dict(RUST_LIST_SORT="y"), dict(RUST_LIST_SORT_KUNIT_TEST="y")):
            with self.assertRaises(ValueError): check.states(config)

    def test_configured_original_intlog_warnings_only(self):
        import test_int_log_runtime as log_tests
        source = log_tests.console(bug=True)
        # This helper's fixture includes its own ABI probes; keep only KUnit.
        lines = source.splitlines(keepends=True)
        end = next(i for i, line in enumerate(lines) if re.fullmatch(rb"\s*ok \d+ math-int_log\s*", line))
        prefix = b"".join(lines[:end + 1])
        cfg = dict(INT_LOG_KUNIT_TEST="y", RUST_INT_LOG="y", BUG="y")
        data = prefix + console()
        self.assertEqual(check.verify_console(data, "c", config=cfg), 1)
        with self.assertRaises(ValueError): check.verify_console(data, "c")
        with self.assertRaises(ValueError): check.verify_console(data + b"WARNING: extra\n", "c", config=cfg)
        with self.assertRaises(ValueError): check.verify_console(data.replace(b"int_log_rust.rs", b"list_sort.rs"), "c", config=cfg)


class ArtifactTests(unittest.TestCase):
    def test_cli_preflight_rejects_before_creating_fixture(self):
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            configs = [{}, {"RUST_LIST_SORT": "y"}, {"RUST": "y", "RUST_LIST_SORT": "y", "MODULES": "y"},
                       dict(RUST="y", RUST_LIST_SORT="y", MODULES="y", PRINTK="y", MULTIUSER="y", MODULE_SIG_FORCE="y")]
            for config in configs:
                with mock.patch.object(sys, "argv", ["check", str(build)]), mock.patch.object(check, "configuration", return_value=config), \
                     redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                    check.main()
                self.assertEqual(error.exception.code, 2)
                self.assertEqual(list(build.iterdir()), [])

    def test_selected_export_is_unrestricted_and_owned_once(self):
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            valid = b"0x12345678\tlist_sort\tvmlinux\tEXPORT_SYMBOL\t\n"
            (build / "Module.symvers").write_bytes(valid)
            self.assertEqual(check.selected_version(build), b"0x12345678")
            for data in (b"", valid * 2, valid.replace(b"vmlinux", b"lib/list_sort"), valid.replace(b"EXPORT_SYMBOL", b"EXPORT_SYMBOL_GPL"),
                         valid.replace(b"\t\n", b"\tNAMESPACE\n"), valid.replace(b"0x12345678", b"0x123")):
                (build / "Module.symvers").write_bytes(data)
                with self.assertRaises(ValueError): check.selected_version(build)

    def test_suite_metadata_exact_not_substrings(self):
        obj = Path("fixture.o")
        for builtin in (False, True):
            prefix = b"test_list_sort." if builtin else b""
            fields = [prefix + b"license=GPL", prefix + b"description=list_sort() KUnit test suite"]
            with mock.patch.object(check, "metadata_fields", return_value=fields): check.verify_suite_metadata(obj, builtin)
            for bad in ([fields[0] + b"broken", fields[1]], [fields[0], fields[1] + b"broken"], fields * 2):
                with mock.patch.object(check, "metadata_fields", return_value=bad):
                    with self.assertRaises(ValueError): check.verify_suite_metadata(obj, builtin)

    def test_defining_object_dwarf_not_cmd_only(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            def fake_run(args, **kwargs):
                Path(args[2]).write_bytes(b"list_sort s#list_head\n")
                return subprocess.CompletedProcess(args, 0, b"#SYMVER list_sort 0x12345678\n", b"")
            with mock.patch.object(check, "configuration", return_value={"MODVERSIONS": "y", "GENDWARFKSYMS": "y"}), \
                    mock.patch.object(check.subprocess, "run", side_effect=fake_run):
                check.verify_dwarf(root, root / "owner.o", b"0x12345678")
                with self.assertRaises(ValueError): check.verify_dwarf(root, root / "owner.o", b"0x00000000")
            with mock.patch.object(check, "configuration", return_value={"MODVERSIONS": "y"}):
                with self.assertRaises(ValueError): check.verify_dwarf(root, root / "owner.o", b"0x12345678")

    def test_source_oracle_and_nonnull_actual_binding_boundary(self):
        self.assertIn(str(ROOT / "lib/list_sort.c"), check.reference_source())
        self.assertIn("#define list_sort list_sort_reference", check.reference_source())
        self.assertNotIn("merge_final", check.reference_source())
        self.assertIn("-D__DISABLE_EXPORTS", check.PRIVATE_C_FLAGS)
        for source in (check.sources("c"), check.sources("rust")):
            self.assertIn("Proprietary", source)
            self.assertIn("list_sort_call", source)
            self.assertNotIn("no_sanitize", source)
            self.assertNotIn("nocf_check", source)
        rust = check.sources("rust")
        self.assertIn(str(ROOT / "include/linux/list_sort_header.rs"), rust)
        self.assertNotIn("Option<", rust)
        self.assertIn("read_volatile", rust)
        self.assertIn("list_sort_exercise(list_sort_call, comparator)", rust)
        self.assertIn("__IS_RUST_MODULE", rust)
        self.assertNotIn("module!", rust)

    def test_count_matches_independent_complete_corpus(self):
        lengths = list(range(261)) + [511, 512, 513, 642, 1023, 1024, 1025]
        self.assertEqual(list(check.LENGTHS), lengths)
        self.assertEqual(check.COUNT, 6435)
        self.assertIn("ai >= bi", check.WORKLOAD)
        self.assertIn("x->ap != y->ap", check.WORKLOAD)
        self.assertIn("p->id >= nb->id", check.WORKLOAD)
        self.assertIn("list_sort_reference(NULL", check.WORKLOAD)


def private_compile(build, out, caller):
    """Replay saved flags read-only, with every compiler output in out."""
    out.mkdir(parents=True, exist_ok=True)
    env = {**os.environ, "RUSTC_BOOTSTRAP": "1", "OBJTREE": str(build), "RUST_MODFILE": "list_sort_abi", "LC_ALL": "C"}
    commands = []
    def run(args):
        commands.append(shlex.join(list(map(str, args))))
        result = subprocess.run(args, cwd=build, env=env, capture_output=True, timeout=120)
        if result.returncode:
            raise AssertionError("\n".join(commands) + "\n" + result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
        return result
    # The suite's .cmd changes language, and a fresh Rust-provider tree need
    # never have built original list_sort.o. This neighboring, always-C unit
    # supplies actual target/lib flags without assuming either selection.
    raw = check.compilation_flags(build / "lib/scatterlist.o")
    cflags = raw[1:raw.index("-c")]
    cflags = [flag for flag in cflags if not flag.startswith(("-Wp,-MMD,", "-DKBUILD_MODFILE=", "-DKBUILD_MODNAME=", "-DKBUILD_BASENAME=", "-D__KBUILD_MODNAME="))]
    cflags += ["-DMODULE", "-D__DISABLE_EXPORTS", '-DKBUILD_MODNAME="list_sort_abi"', '-DKBUILD_BASENAME="list_sort_abi"',
               '-DKBUILD_MODFILE="list_sort_abi"', '-D__KBUILD_MODNAME=list_sort_abi', "-Werror"]
    for stem, source in (("list_sort_reference", check.reference_source()), ("list_sort_workload", check.WORKLOAD)):
        path = out / (stem + ".c")
        path.write_text(source)
        run([raw[0], *cflags, "-c", "-o", out / (stem + ".o"), path])
    path = out / ("caller.rs" if caller == "rust" else "caller.c")
    path.write_text(check.sources(caller))
    if caller == "c": run([raw[0], *cflags, "-c", "-o", out / "caller.o", path])
    else:
        raw = check.compilation_flags(build / "lib/list_sort_rust.o")
        while raw and not raw[0].endswith("rustc"): raw.pop(0)
        compiler = os.environ.get("HOSTRUSTC", raw.pop(0))
        flags = []
        index = 0
        while index < len(raw):
            flag = raw[index]
            if flag == "--out-dir": index += 2; continue
            if flag.startswith("--emit=") or flag.endswith(".rs"): index += 1; continue
            flags.append(flag); index += 1
        run([compiler, *flags, "--cfg", "MODULE", "-Dwarnings", "--crate-name=list_sort_rust_abi",
             "--emit=obj=" + str(out / "caller.o") + ",llvm-ir=" + str(out / "caller.ll"), path])
    linker = shlex.split(os.environ.get("LD_LLD", "ld.lld"))
    run([*linker, "-r", "-o", out / "linked.o", out / "caller.o", out / "list_sort_reference.o", out / "list_sort_workload.o"])
    (out / "commands.txt").write_text("\n".join(commands) + "\n")
    return out / "caller.o", out / "linked.o"


class NativeCompileTests(unittest.TestCase):
    def _build(self, variable):
        value = os.environ.get(variable)
        if value is None: self.skipTest(variable + " not supplied: real native gate not run")
        if not value: self.fail(variable + " explicitly empty")
        build = Path(value).resolve()
        self.assertTrue(build.is_dir())
        return build

    def _compile(self, variable):
        build = self._build(variable)
        for name in ("lib/.scatterlist.o.cmd", "lib/.list_sort_rust.o.cmd", "rust/libkernel.rmeta"):
            self.assertTrue((build / name).is_file(), name)
        arch = check.architecture(check.configuration(build))
        with tempfile.TemporaryDirectory(prefix="list-sort-runtime-compile-") as temporary:
            out = Path(temporary)
            types = check.provider_type_ids(build / "lib/list_sort_rust.o", names=("list_sort",))
            for caller in ("c", "rust"):
                obj, linked = private_compile(build, out / caller, caller)
                for path in (obj, linked):
                    check.elf_target(path, arch)
                    check.verify_guarded_calls(path, arch, types, wrappers=(("list_sort_call", "list_sort"),))
                    with self.assertRaises(ValueError):
                        check.verify_guarded_calls(path, arch, {"list_sort": types["list_sort"] ^ 1}, wrappers=(("list_sort_call", "list_sort"),))
                self.assertEqual(check.provider_type_ids(obj, names=("list_sort_call",))["list_sort_call"], types["list_sort"])
                if caller == "rust":
                    imports = {line.split()[-1] for line in check.tool("nm", "-u", obj).splitlines()}
                    self.assertEqual(imports, {b"_printk", b"list_sort", b"list_sort_compare", b"list_sort_exercise"})
                    check.verify_rust_entrypoints(obj, arch)
                    ir = (obj.parent / "caller.ll").read_text()
                    hashes = {n: int(value) & 0xffffffff for n, value in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}$', ir, re.M)}
                    callbacks = re.findall(r'^define [^\n]*@[^\s(]*comparator[^\s(]*\([^\n]*!kcfi_type !(\d+)', ir, re.M)
                    self.assertEqual(len(callbacks), 1)
                    c_hash = check.provider_type_ids(obj.parent / "list_sort_workload.o", names=("list_sort_compare",))["list_sort_compare"]
                    self.assertEqual(hashes[callbacks[0]], c_hash)

    def test_private_x86_native_headers_bindings_and_guarded_calls(self):
        self._compile("NATIVE_LIST_SORT_KERNEL_BUILD")

    def test_private_arm64_native_headers_bindings_and_guarded_calls(self):
        self._compile("NATIVE_LIST_SORT_ARM64_KERNEL_BUILD")

    def test_workload_executes_original_algorithm_and_detects_no_sort(self):
        build = self._build("NATIVE_LIST_SORT_KERNEL_BUILD")
        with tempfile.TemporaryDirectory(prefix="list-sort-workload-run-") as temporary:
            out = Path(temporary)
            private_compile(build, out, "c")
            raw = shlex.split((out / "commands.txt").read_text().splitlines()[0])
            flags = raw[1:raw.index("-c")]
            driver = out / "driver.c"
            driver.write_text(r'''
#include <linux/list_sort.h>
#include <linux/printk.h>
typedef void (*sort_fn)(void *, struct list_head *, list_cmp_func_t);
int list_sort_exercise(sort_fn, list_cmp_func_t);
int list_sort_compare(void *, const struct list_head *, const struct list_head *);
extern int vprintf(const char *, __builtin_va_list);
int _printk(const char *format, ...) {
    __builtin_va_list arguments;
    int result;
    __builtin_va_start(arguments, format);
    result = vprintf(format, arguments);
    __builtin_va_end(arguments);
    return result;
}
static void no_sort(void *ctx, struct list_head *head, list_cmp_func_t cmp) { (void)ctx; (void)head; (void)cmp; }
int main(int argc, char **argv) {
    int result = list_sort_exercise(argc > 1 ? no_sort : list_sort, list_sort_compare);
    (void)argv;
    return result == @COUNT@ ? 0 : 1;
}
'''.replace("@COUNT@", str(check.COUNT)))
            commands = [[raw[0], *flags, "-c", "-o", out / "selected.o", ROOT / "lib/list_sort.c"],
                        [raw[0], *flags, "-c", "-o", out / "driver.o", driver],
                        [raw[0], "-no-pie", "-o", out / "run", out / "driver.o", out / "selected.o",
                         out / "list_sort_workload.o", out / "list_sort_reference.o"]]
            for command in commands:
                result = subprocess.run(command, cwd=build, capture_output=True, timeout=120)
                self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
            for arguments, code in (([], 0), (["wrong"], 1)):
                result = subprocess.run([str(out / "run"), *arguments], capture_output=True, timeout=60)
                self.assertEqual(result.returncode, code, result.stdout + result.stderr)
                if arguments: self.assertIn(b"LUPOS_LIST_SORT_FAIL stage=1 length=2", result.stdout)

    def _linked(self, variable):
        build = self._build(variable)
        config = check.configuration(build)
        selection = "Rust" if config.get("RUST_LIST_SORT") == "y" else "C"
        modules = check.verify_linked_implementation(build, selection)
        self.assertEqual(len(modules), int(config.get("TEST_LIST_SORT") == "m") * (1 + int(config.get("KUNIT") == "m")))
        wrong = {**config, "RUST_LIST_SORT": "n" if selection == "Rust" else "y"}
        with mock.patch.object(check, "configuration", return_value=wrong):
            with self.assertRaisesRegex(ValueError, "archive owner"):
                check.verify_linked_implementation(build, "C" if selection == "Rust" else "Rust")
        if config.get("TEST_LIST_SORT") in ("y", "m"):
            wrong = {**config, "RUST_LIST_SORT_KUNIT_TEST": "n" if config.get("RUST_LIST_SORT_KUNIT_TEST") == "y" else "y"}
            with mock.patch.object(check, "configuration", return_value=wrong):
                with self.assertRaises(ValueError): check.verify_linked_implementation(build, selection)

    def test_actual_x86_owner_suite_dwarf_exports_and_selection(self):
        self._linked("NATIVE_LIST_SORT_KERNEL_BUILD")

    def test_actual_arm64_owner_suite_dwarf_exports_and_selection(self):
        self._linked("NATIVE_LIST_SORT_ARM64_KERNEL_BUILD")


if __name__ == "__main__":
    unittest.main()
