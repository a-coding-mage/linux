# SPDX-License-Identifier: GPL-2.0-only
"""Discoverable glob runtime gates; no native kernel build or VM here.

GLOB_SOURCE_ROOT selects sources. GLOB_RUNTIME_RUSTC selects rustc >=1.85.
Optional GLOB_RUNTIME_X86_BUILD / GLOB_RUNTIME_ARM64_BUILD supply read-only
strict saved flags/rmeta. GLOB_RUNTIME_CONSOLE supplies original-suite evidence
(only its KTAP is checked, never claimed as this workload's VM result).
GLOB_RUNTIME_LOGS must be an existing directory outside source/native trees.
Every command and failed private fixture is retained; outputs default to /tmp.
"""
from concurrent.futures import ThreadPoolExecutor
from contextlib import ExitStack, redirect_stderr, redirect_stdout
import io
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import check_glob_kernel as checker
import glob_runtime_fixtures as fixtures
from test_module_metadata_fidelity import first_saved_command
from test_base64 import native_c_arguments

ROOT = checker.ROOT


def settings():
    """Validate ALL explicit inputs before output creation or optional skips."""
    for name in ("lib/glob.c", "lib/glob.rs", "lib/glob_rust.rs", "include/linux/glob.h", "include/linux/glob_header.rs", "lib/tests/glob_kunit.c"):
        if not (ROOT / name).is_file(): raise ValueError("invalid source: " + str(ROOT / name))
    result = {}
    for key in ("X86_BUILD", "ARM64_BUILD", "CONSOLE", "LOGS", "RUSTC"):
        variable = "GLOB_RUNTIME_" + key
        if variable in os.environ: result[key] = checker.explicit_path(os.environ[variable])
    result.setdefault("RUSTC", Path(shutil.which("rustc") or "missing-rustc").absolute())
    result.setdefault("LOGS", Path("/tmp"))
    if not result["LOGS"].is_dir(): raise ValueError("log directory must already exist")
    for key, arch in (("X86_BUILD", "x86_64"), ("ARM64_BUILD", "aarch64")):
        if key not in result: continue
        build = result[key]
        for name in (".config", "lib/.scatterlist.o.cmd", "lib/math/.int_log_rust.o.cmd",
                     "rust/bindings/.bindings_generated.rs.cmd", "rust/libkernel.rmeta", "rust/libbindings.rmeta"):
            if not (build / name).is_file(): raise ValueError("invalid native input: " + str(build / name))
        cfg = checker.configuration(build)
        if checker.architecture(cfg) != arch or any(cfg.get(k) != "y" for k in ("RUST", "CFI", "MODVERSIONS", "RUST_INT_LOG")):
            raise ValueError("native donor must have strict RUST_INT_LOG flags for " + arch)
    if "CONSOLE" in result and not result["CONSOLE"].is_file(): raise ValueError("invalid retained console")
    for source in (ROOT, *(result[k] for k in ("X86_BUILD", "ARM64_BUILD") if k in result)):
        if result["LOGS"] == source or source in result["LOGS"].parents:
            raise ValueError("outputs must be outside source/native trees")
    compiler = result["RUSTC"]
    if not compiler.is_file() or not os.access(compiler, os.X_OK): raise ValueError("invalid rustc")
    version = subprocess.run([compiler, "--version"], capture_output=True, text=True, timeout=30)
    match = re.fullmatch(r"rustc (\d+)\.(\d+)\.(\d+) .*\n?", version.stdout)
    if version.returncode or not match or tuple(map(int, match.groups())) < (1, 85, 0): raise ValueError("rustc >=1.85 required")
    return result


def suite_lines():
    return [b"# Subtest: glob", b"# module: glob_kunit", b"1..1", b"KTAP version 1", b"# Subtest: glob_test_match",
            *[b"ok " + str(i).encode() + b" " + label for i, label in enumerate(fixtures.labels(ROOT), 1)],
            b"# glob_test_match: pass:64 fail:0 skip:0 total:64", b"ok 1 glob_test_match",
            b"# Totals: pass:64 fail:0 skip:0 total:64", b"ok 1 glob"]


def console(caller="c", suite="n", framework=False, reload=False):
    preloads = int(suite == "m") + int(framework)
    lines = suite_lines() if suite == "y" else []
    for i in range(preloads):
        if i == preloads - 1: lines += suite_lines()
        lines += [f"LUPOS_RUST_PRELOAD_OK {i}".encode()]
    mark = (b"glob_rust_abi: " if caller == "rust" else b"") + fixtures.marker(ROOT, caller)
    lines += [mark, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        lines += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for i in range(preloads):
            if i == preloads - 1: lines += suite_lines()
            lines += [f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode()]
        lines += [mark, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    return b"\n".join([*lines, checker.MARKER]) + b"\n"


class Validated(unittest.TestCase):
    def setUp(self):
        self.inputs = settings()


class ProtocolTests(Validated):
    def test_actual_header_and_complete_original_cases(self):
        header = (ROOT / "include/linux/glob.h").read_text()
        self.assertEqual(re.findall(r"bool __pure (\w+)\(", header), list(checker.SYMBOLS))
        self.assertEqual(len(fixtures.original_cases(ROOT)), 64)
        self.assertEqual(fixtures.count(ROOT), 95537)
        for caller in ("c", "rust"):
            text = fixtures.caller_source(ROOT, caller)
            self.assertNotRegex(text, r"@[A-Z_]+@")
            self.assertIn("glob_call_match", text); self.assertIn("glob_call_len", text)
        self.assertNotIn("glob_case", fixtures.RUST)
        self.assertNotIn("dispatcher", fixtures.reference_source(ROOT))

    def test_bool_provider_and_independent_framework_matrix(self):
        for provider in ("n", "m", "", "bad"):
            with self.assertRaises(ValueError): checker.states(dict(GLOB=provider))
        for suite, framework in (("n", "n"), ("n", "y"), ("n", "m"), ("y", "y"), ("m", "y"), ("m", "m")):
            self.assertEqual(checker.states(dict(GLOB="y", GLOB_KUNIT_TEST=suite, KUNIT=framework)), (suite, framework))
        for suite, framework in (("y", "m"), ("y", "n"), ("m", "n"), ("bad", "y")):
            with self.assertRaises(ValueError): checker.states(dict(GLOB="y", GLOB_KUNIT_TEST=suite, KUNIT=framework))

    def test_complete_protocol_matrix(self):
        for caller in ("c", "rust"):
            for suite, framework in (("n", False), ("y", False), ("m", False), ("m", True)):
                for reload in (False, True):
                    data = console(caller, suite, framework, reload)
                    expected = 0 if suite == "n" else 128 if suite == "m" and reload else 64
                    for text in (data, b"\n".join(b"[  1.234567] " + line for line in data.splitlines())):
                        self.assertEqual(checker.verify_console(text, caller, suite=suite, framework_module=framework, reload=reload), expected)

    def test_every_parameter_and_structural_line_missing_duplicate_wrong_order(self):
        lines = suite_lines()
        for at, line in enumerate(lines):
            if line == b"# module: glob_kunit": continue
            mutations = [lines[:at] + lines[at+1:], lines[:at] + [line] + lines[at:]]
            if at: mutations.append(lines[:at-1] + [line, lines[at-1]] + lines[at+1:])
            for changed in mutations:
                with self.subTest(at=at), self.assertRaises(ValueError): checker.kunit_runs(b"\n".join(changed), 1)
        good = b"\n".join(lines)
        for old, new in ((b"1..1", b"1..64"), (b"ok 32", b"not ok 32"), (b"str:\"axb\"", b"str:\"axb\" # SKIP"),
                         (b"pass:64", b"pass:63"), (b"fail:0", b"fail:1"), (b"ok 64", b"ok 63")):
            with self.assertRaises(ValueError): checker.kunit_runs(good.replace(old, new), 1)
        with self.assertRaises(ValueError): checker.kunit_runs(good + b'\nok 1 pat:"x" str:"x"', 1)

    def test_each_lifecycle_event_required_and_ordered(self):
        lines = console("rust", "m", True, True).splitlines()
        for at, line in enumerate(lines):
            if b"LUPOS_" not in line: continue
            changes = [lines[:at] + lines[at+1:], lines[:at] + [line] + lines[at:]]
            if at: changes.append([line] + lines[:at] + lines[at+1:])
            for changed in changes:
                with self.assertRaises(ValueError): checker.verify_console(b"\n".join(changed), "rust", suite="m", framework_module=True, reload=True)

    def test_faults_wrong_count_caller_and_replay_transport(self):
        good = console()
        for changed in (good.replace(b"cases=", b"cases=9"), good.replace(b"GLOB_ABI", b"GLOB_RUST_ABI"),
                        good + b"WARNING: unknown\n", good + b"CFI failure\n", good + b"no symbol version for glob_match\n",
                        good + b"LUPOS_GLOB_FAILED\n", good.replace(b"LUPOS_GLOB", b"noise LUPOS_GLOB")):
            with self.assertRaises(ValueError): checker.verify_console(changed, "c")
        mark = fixtures.marker(ROOT, "c")
        replay = good.replace(mark, mark[:12] + b"\n** replaying previous printk message **\n" + mark)
        self.assertEqual(checker.verify_console(replay, "c"), 0)
        with self.assertRaises(ValueError): checker.verify_console(replay.replace(mark[:12], b"wrong", 1), "c")

    def test_retained_original_suite_only(self):
        if "CONSOLE" not in self.inputs: self.skipTest("no GLOB_RUNTIME_CONSOLE; retained suite evidence optional")
        data = self.inputs["CONSOLE"].read_bytes()
        self.assertEqual(len(checker.kunit_runs(checker.normalize_console_transport(data), 2)), 2)
        # Earlier root preflight lacks this independent workload marker.
        with self.assertRaises(ValueError): checker.verify_console(data, "rust", suite="m", reload=True)

    def test_explicit_invalid_inputs_fail_before_creation_or_skip(self):
        with tempfile.TemporaryDirectory(prefix="glob-validation-", dir=self.inputs["LOGS"]) as tmp:
            base = Path(tmp); empty = base / "empty"; empty.mkdir()
            before = set(base.rglob('*'))
            for key in ("X86_BUILD", "ARM64_BUILD", "CONSOLE", "LOGS", "RUSTC"):
                for value in ("", str(base / "missing"), str(empty)):
                    if key == "LOGS" and value == str(empty): continue
                    with mock.patch.dict(os.environ, {"GLOB_RUNTIME_" + key: value}), self.assertRaises(ValueError): settings()
                    self.assertEqual(set(base.rglob('*')), before)
            with mock.patch.dict(os.environ, {"GLOB_RUNTIME_RUSTC": "/bin/true"}), self.assertRaises(ValueError): settings()
            for arg in ("", str(base / "missing"), str(empty)):
                with redirect_stderr(io.StringIO()), self.assertRaises((ValueError, SystemExit)):
                    checker.main([arg, "--check-only"])
                self.assertEqual(set(base.rglob('*')), before)


class PrivateBuild(Validated):
    def workspace(self):
        work = Path(tempfile.mkdtemp(prefix="glob-runtime-test-", dir=self.inputs["LOGS"]))
        print("glob private evidence: " + str(work), flush=True)
        self.log = (work / "commands.log").open("w")
        self.addCleanup(self.log.close)
        self.work = work
        return work

    def run_command(self, args, *, cwd=None, env=None, success=True, timeout=120, input=None):
        cwd = Path(cwd or self.work).resolve()
        for donor in (ROOT, *(self.inputs[key] for key in ("X86_BUILD", "ARM64_BUILD") if key in self.inputs)):
            if cwd == donor or donor in cwd.parents:
                raise ValueError("compiler cwd must be outside source/native trees")
        env = fixtures.compiler_environment(self.work, os.environ if env is None else env)
        self.log.write("$ " + shlex.join(map(str, args)) + "\n"); self.log.flush()
        result = subprocess.run(list(map(str, args)), cwd=cwd, env=env,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=timeout, input=input)
        self.log.write(result.stdout.decode(errors="replace") + f"\nEXIT {result.returncode}\n"); self.log.flush()
        if success: self.assertEqual(result.returncode, 0, result.stdout[-6000:].decode(errors="replace"))
        else: self.assertNotEqual(result.returncode, 0, "negative control unexpectedly passed")
        return result

    def host_headers(self):
        include = self.work / "include/linux"; include.mkdir(parents=True)
        # Header types are from the original glob.h. Only kernel plumbing is
        # replaced to execute the exact algorithm and caller in userspace.
        (include / "glob.h").write_text('#include "' + str(ROOT / "include/linux/glob.h") + '"\n')
        values = {"types.h": "#include <stdbool.h>\n#include <stddef.h>\n", "compiler.h": "#define __pure\n",
            "module.h": '#include <stdio.h>\n#define MODULE_DESCRIPTION(x)\n#define MODULE_LICENSE(x)\n#define noinline __attribute__((noinline))\n#define fallthrough __attribute__((fallthrough))\n',
            "export.h": "#define EXPORT_SYMBOL(x)\n", "errno.h": "#define EINVAL 22\n", "init.h": "",
            "string.h": "#include <string.h>\n"}
        for name, text in values.items(): (include / name).write_text(text)
        return ["-I" + str(include.parent)]


class HostTests(PrivateBuild):
    def test_complete_protected_c_rust_caller_provider_matrix_and_corruptions(self):
        work = self.workspace()
        flags = [*self.host_headers(), "-O2", "-Wall", "-Wextra", "-Werror", "-fsanitize=kcfi",
                 "-fsanitize-cfi-icall-experimental-normalize-integers", "-DGLOB_HOST", "-D__DISABLE_EXPORTS"]
        env = dict(os.environ, RUSTC_BOOTSTRAP="1")
        rust = [self.inputs["RUSTC"], "--edition=2021", "-Dwarnings", "-Copt-level=2", "-Cpanic=abort",
                "-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
        reference = work / "reference.c"; reference.write_text(fixtures.reference_source(ROOT))
        self.run_command(["clang", *flags, "-c", reference, "-o", work / "reference.o"])
        self.run_command(["clang", *flags, "-c", ROOT / "lib/glob.c", "-o", work / "provider-c.o"])
        self.run_command([*rust, "--crate-type=staticlib", ROOT / "lib/glob.rs", "-o", work / "provider-rust.a"], env=env)
        c = work / "caller.c"; c.write_text(fixtures.caller_source(ROOT, "c"))
        r = work / "caller.rs"; r.write_text(fixtures.caller_source(ROOT, "rust"))
        self.run_command(["clang", *flags, "-c", c, "-o", work / "caller.o"])
        # Both selected providers and both callers are real compiled code.
        for provider in ("c.o", "rust.a"):
            obj = work / ("provider-" + provider)
            self.run_command(["clang", "-fsanitize=kcfi", work / "caller.o", work / "reference.o", obj, "-ldl", "-lpthread", "-lm", "-o", work / "c-run"])
            self.run_command([work / "c-run"])
            self.run_command([*rust, "--cfg=glob_host", r, "-Clink-arg=" + str(work / "reference.o"), "-Clink-arg=" + str(obj), "-o", work / "rust-run"], env=env)
            self.run_command([work / "rust-run"])
        # A genuine compiled C-dispatcher substitute can pass all semantic
        # comparisons while running zero independent Rust workload iterations.
        # Compile/run that negative, then require exact source provenance to
        # reject it. Retained no_mangle wrappers alone would be insufficient.
        dispatch = work / "dispatcher"; dispatch.mkdir()
        cs = dispatch / "dispatcher.c"
        cs.write_text(c.read_text() + '\nbool glob_c_dispatcher(void) { return glob_workload() == 0; }\n')
        self.run_command(["clang", *flags, "-Dmain=glob_c_main_host", "-c", cs, "-o", dispatch / "dispatcher.o"])
        text = r.read_text().replace('unsafe extern "C" {', 'unsafe extern "C" {\n    fn glob_c_dispatcher() -> bool;', 1)
        start = text.index('fn workload() -> bool {')
        end = text.index('\n#[cfg(glob_host)]', start)
        text = text[:start] + 'fn workload() -> bool { unsafe { glob_c_dispatcher() } }\n' + text[end:]
        # Remove the unused Rust dataset rather than suppress strict warnings.
        start = text.index('static CASES:')
        end = text.index('\nfn workload()', start)
        text = text[:start] + text[end:]
        text = re.sub(r"(?m)^    fn glob_reference_(?:match|len)\([^\n]+\n", "", text)
        rs = dispatch / "glob_rust_main.rs"; rs.write_text(text)
        (dispatch / "glob_reference.c").write_text(reference.read_text())
        self.run_command([*rust, "--cfg=glob_host", rs, "-Clink-arg=" + str(dispatch / "dispatcher.o"),
            "-Clink-arg=" + str(work / "reference.o"), "-Clink-arg=" + str(work / "provider-c.o"), "-o", dispatch / "run"], env=env)
        self.run_command([dispatch / "run"])
        with self.assertRaisesRegex(ValueError, "caller source changed"):
            checker.verify_fixture_sources(dispatch, "rust")
        # Mutate real Rust algorithm; neither constant-result oracle can pass.
        original = (ROOT / "lib/glob.rs").read_text()
        for value in ("true", "false"):
            mutant = work / ("corrupt-" + value + ".rs")
            mutant.write_text(original.replace("let c = input(s);", "let c = input(s); if p == 0 { return " + value + "; }"))
            library = work / ("corrupt-" + value + ".a")
            self.run_command([*rust, "--crate-type=staticlib", mutant, "-o", library], env=env)
            self.run_command(["clang", work / "caller.o", work / "reference.o", library, "-ldl", "-lpthread", "-lm", "-o", work / "c-bad"])
            self.run_command([work / "c-bad"], success=False)
            self.run_command([*rust, "--cfg=glob_host", r, "-Clink-arg=" + str(work / "reference.o"), "-Clink-arg=" + str(library), "-o", work / "rust-bad"], env=env)
            self.run_command([work / "rust-bad"], success=False)

    def test_guard_pages_valid_domains_and_private_overread_controls(self):
        work = self.workspace()
        flags = self.host_headers()
        rust = self.inputs["RUSTC"]
        self.run_command([rust, "--edition=2021", "-Dwarnings", "-Copt-level=2", "--crate-type=cdylib", ROOT / "lib/glob.rs", "-o", work / "provider.so"])
        self.run_command(["clang", *flags, "-O2", "-shared", "-fPIC", ROOT / "lib/glob.c", "-o", work / "original.so"])
        guard = work / "guard.c"
        guard.write_text(r'''
#include <sys/mman.h>
#include <unistd.h>
#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
extern bool glob_match(const char *, const char *);
extern bool glob_match_len(const char *, const char *, size_t);
int main(void) {
 size_t n = sysconf(_SC_PAGESIZE);
 char *p = mmap(0,n*2,PROT_READ|PROT_WRITE,MAP_PRIVATE|MAP_ANONYMOUS,-1,0);
 assert(p != MAP_FAILED); assert(!mprotect(p+n,n,PROT_NONE));
 bool (*volatile unbounded)(const char *,const char *) = glob_match;
 bool (*volatile bounded)(const char *,const char *,size_t) = glob_match_len;
 /* No NUL requirement for these exact bounded buffers. */
 assert(bounded("",p+n,0)); assert(bounded("*",p+n,0)); assert(!bounded("?",p+n,0));
 p[n-1]='x'; assert(bounded("?",p+n-1,1)); assert(bounded("*x",p+n-1,1)); assert(!bounded("xx",p+n-1,1));
 p[n-1]=(char)255; assert(bounded("[\200-\377]",p+n-1,1));
 p[n-1]=0; p[n-2]='['; assert(unbounded(p+n-2,"["));
 p[n-4]='['; p[n-3]='a'; p[n-2]='-'; assert(unbounded(p+n-4,"[a-"));
 assert(unbounded("\\",p+n-1));
#ifdef PRIVATE_OVERREAD
 /* Deliberately invalid probe of the harness guard, never a public case. */
 return *(volatile char *)(p+n);
#endif
 return 0;
}
''')
        for library in ("provider.so", "original.so"):
            for private in (False, True):
                exe = work / (library + str(private))
                self.run_command(["clang", "-O2", *( ["-DPRIVATE_OVERREAD"] if private else []), guard, work / library, "-o", exe])
                result = self.run_command([exe], success=not private)
                if private: self.assertEqual(result.returncode, -11)
        # The same valid-domain guard workload rejects a privately corrupted
        # provider that reads input even when the bound is zero.
        text = (ROOT / "lib/glob.rs").read_text()
        old = "    if index >= limit {\n        return 0;\n    }"
        self.assertEqual(text.count(old), 1)
        mutant = work / "overread.rs"
        mutant.write_text(text.replace(old, "    if index >= limit {\n        return unsafe { core::ptr::read_volatile(ptr) as u8 };\n    }"))
        self.run_command([rust, "--edition=2021", "-Dwarnings", "-Copt-level=2", "--crate-type=cdylib", mutant, "-o", work / "overread.so"])
        self.run_command(["clang", "-O2", guard, work / "overread.so", "-o", work / "overread"])
        self.assertEqual(self.run_command([work / "overread"], success=False).returncode, -11)

    def test_lazy_early_exit_counts_and_eager_scan_negative(self):
        work = self.workspace()
        flags = [self.inputs["RUSTC"], "--edition=2021", "-Dwarnings", "-Copt-level=2", "--test"]
        self.run_command([*flags, ROOT / "lib/glob.rs", "-o", work / "reads"])
        self.run_command([work / "reads", "--nocapture"])
        source = (ROOT / "lib/glob.rs").read_text()
        tag = "    // SAFETY: Forward the caller's string contracts to each lazy access."
        self.assertEqual(source.count(tag), 1)
        mutant = work / "eager.rs"
        mutant.write_text(source.replace(tag, "    unsafe { let mut i = 0; while read_byte(input, i, len) != 0 { i += 1; } }\n" + tag))
        self.run_command([*flags, mutant, "-o", work / "eager"])
        result = self.run_command([work / "eager", "--nocapture"], success=False)
        self.assertIn(b"early_exit_read_counts", result.stdout)


class PolicyTests(PrivateBuild):
    def test_effective_warning_policy_matches_actual_rustc(self):
        work = self.workspace()
        source = work / "lint.rs"
        source.write_text("fn unused() {}\nfn main() {}\n")
        strict = (["-Dwarnings"], ["-D", "warnings"], ["--deny=warnings"], ["--deny", "warnings"],
                  ["-Wwarnings", "-Dwarnings"], ["-Fwarnings", "-Awarnings"],
                  ["-Dwarnings", "--cap-lints=deny", "--cap-lints=warn"])
        weak = (["-Dwarnings", "-Wwarnings"], ["-Dwarnings", "--warn", "warnings"],
                ["-Dwarnings", "--allow=warnings"], ["-Dwarnings", "--cap-lints", "warn"],
                ["-Dwarnings", "--cap-lints=warn", "--cap-lints=deny"],
                ["-Dwarnings", "--force-warn=dead_code"], ["-D=warnings"], ["-Dwarnings,dead_code"],
                ["-Dwarnings,unused_variables"], ["-Wwarnings,unused_variables"])
        for flags in (*strict, *weak):
            with self.subTest(flags=flags):
                if flags in strict: checker.verify_strict_lints(flags)
                else:
                    with self.assertRaises(ValueError): checker.verify_strict_lints(flags)
                result = self.run_command([self.inputs["RUSTC"], "--edition=2021", "--emit=metadata", "--out-dir=" + str(work),
                    *flags, source], success=flags in weak)
                if flags in strict: self.assertIn(b"error: function `unused`", result.stdout)
                if any("," in flag for flag in flags):
                    self.assertIn(b"unknown lint: `warnings,", result.stdout)
        flags = ["-Dwarnings", "--cap-lints=force-warn"]
        with self.assertRaises(ValueError): checker.verify_strict_lints(flags)
        result = self.run_command([self.inputs["RUSTC"], "--emit=metadata", "--out-dir=" + str(work), *flags, source], success=False)
        self.assertIn(b"unknown lint level", result.stdout)

    def test_accumulated_sanitizers_match_real_config_and_object_types(self):
        work = self.workspace()
        source = work / "policy.rs"
        source.write_text('#![no_std]\n#[no_mangle]\npub extern "C" fn policy_callback(value: u32) -> u32 { value }\n')
        env = dict(os.environ, RUSTC_BOOTSTRAP="1")
        common = ["-Dwarnings", "-Cpanic=abort", "-Zsanitizer-cfi-normalize-integers"]
        baseline = None
        choices = (["-Zsanitizer=kcfi"], ["-Z", "sanitizer=address,kcfi"],
                   ["-Zsanitizer=kcfi", "-Zsanitizer=address"],
                   ["-Zsanitizer=address", "-Z", "sanitizer=kcfi"])
        for index, flags in enumerate(choices):
            checker.verify_flag_policy([*common, *flags], "rust")
            obj = work / (str(index) + ".o")
            command = [self.inputs["RUSTC"], "--edition=2021", "--crate-type=rlib", "--out-dir=" + str(work), *common, *flags]
            self.run_command([*command, "--emit=obj=" + str(obj), source], env=env)
            actual = checker.provider_type_ids(obj, names=("policy_callback",))
            if baseline is None: baseline = actual
            self.assertEqual(actual, baseline)
            cfg = self.run_command([*command, "--print=cfg"], env=env).stdout
            self.assertIn(b'sanitize="kcfi"', cfg)
            if index: self.assertIn(b'sanitize="address"', cfg)
            response = work / "policy.rsp"
            response.write_text("\n".join([*common, *flags]) + "\n")
            command_file = obj.with_name("." + obj.name + ".cmd")
            command_file.write_text("savedcmd_policy := rustc @policy.rsp\n")
            checker.verify_saved_flags(obj, "rust", work)
            response.write_text(response.read_text() + "--warn=warnings\n")
            with self.assertRaises(ValueError): checker.verify_saved_flags(obj, "rust", work)
        for flags in (["-Zsanitizer=kernel-address"], ["-Zsanitizer=kcfi", "-Zsanitizer=none"],
                      ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers=no"]):
            with self.assertRaises(ValueError): checker.verify_flag_policy([*common, *flags], "rust")
        for flags in (["-fsanitize=kcfi", "-fno-sanitize=all"], ["-fsanitize=kernel-address"]):
            with self.assertRaises(ValueError): checker.verify_flag_policy(["-fsanitize-cfi-icall-experimental-normalize-integers", *flags], "c")
        checker.verify_flag_policy(["-fsanitize=kcfi", "-fno-sanitize=all", "-fsanitize=kernel-address,kcfi",
            "-fsanitize-cfi-icall-experimental-normalize-integers"], "c")
        checker.verify_flag_policy([*common, "-Zsanitizer=kcfi", "-Zsanitizer=kernel-address"], "rust")

    def test_replay_paths_outputs_and_deleted_transients(self):
        work = self.workspace()
        donor = work / "transport-input"; donor.mkdir()
        (donor / "options").write_text('--target=scripts/target.json\n-Ldependency=rust\n--extern\nkernel=rust/libkernel.rmeta\n'
            '--out-dir=lib\n--emit=obj=lib/old.o\n-Dwarnings\n-Zsanitizer=kcfi\n')
        flags = fixtures.native_flags(["@options", "lib/old.rs"], donor, "rust")
        self.assertEqual(flags, ["--target=" + str(donor / "scripts/target.json"), "-Ldependency=" + str(donor / "rust"),
            "--extern", "kernel=" + str(donor / "rust/libkernel.rmeta"), "-Dwarnings", "-Zsanitizer=kcfi"])
        self.assertEqual(fixtures.native_flags(["-Iinclude", "-include", "include/config.h", "-Werror"], donor, "c"),
            ["-I" + str(donor / "include"), "-include", str(donor / "include/config.h"), "-Werror"])
        for flags in (["-Cincremental=lib/cache"], ["-C", "incremental=lib/cache"]):
            with self.assertRaises(ValueError): fixtures.native_flags(flags, donor, "rust")
        def transient(index):
            path = donor / (str(index) + ".rcgu.o")
            path.write_bytes(b"observer control; not an ABI object")
            path.unlink()
        with fixtures.NativeWriteWatch(donor) as writes:
            with ThreadPoolExecutor(max_workers=2) as pool: list(pool.map(transient, range(2)))
        self.assertEqual([p.name for p in donor.iterdir()], ["options"])
        for index in range(2):
            events = [int(mask, 16) for path, mask in writes.events if path == str(donor / (str(index) + ".rcgu.o"))]
            self.assertTrue(any(mask & 0x100 for mask in events), writes.events)
            self.assertTrue(any(mask & 0x200 for mask in events), writes.events)


class NativeTests(PrivateBuild):
    def native(self, key, arch):
        if key not in self.inputs: self.skipTest("no GLOB_RUNTIME_" + key + "; actual saved-flags object gate optional")
        with fixtures.NativeWriteWatch(self.inputs[key]) as writes:
            self._native(key, arch)
        self.assertEqual(writes.events, [], "native fixture wrote to read-only donor")

    def _native(self, key, arch):
        build = self.inputs[key]
        work = self.workspace()
        env = {**checker.clean_environment(), "RUSTC_BOOTSTRAP": "1", "OBJTREE": str(build), "RUST_MODFILE": "lib/glob"}
        def run(args): return self.run_command(args, cwd=work, env=env)
        compiler, cflags = native_c_arguments(build)
        if "/" in compiler and not Path(compiler).is_absolute(): compiler = str(build / compiler)
        cflags = fixtures.native_flags(cflags, build, "c")
        def flags(stem, module=True):
            return [f.replace("lib/base64", stem).replace('"base64"', '"' + stem + '"') if f.startswith("-DKBUILD_")
                else "-D__KBUILD_MODNAME=" + stem if f.startswith("-D__KBUILD_MODNAME=") else f for f in cflags] + (
                ["-DMODULE", "-D__DISABLE_EXPORTS"] if module else [])
        for stem, text in (("glob_c_main", fixtures.caller_source(ROOT, "c")), ("glob_reference", fixtures.reference_source(ROOT))):
            source = work / (stem + ".c"); source.write_text(text)
            run([compiler, *flags(stem), "-c", source, "-o", work / (stem + ".o")])
        run([compiler, *flags("glob", False), "-c", ROOT / "lib/glob.c", "-o", work / "c-provider.o"])
        types = checker.provider_type_ids(work / "c-provider.o", names=checker.SYMBOLS)
        ref = checker.provider_type_ids(work / "glob_reference.o", names=("glob_reference_match", "glob_reference_len"))
        self.assertEqual(list(types.values()), list(ref.values()))
        checker.verify_guarded_calls(work / "glob_c_main.o", arch, types, wrappers=checker.WRAPPERS)
        # Fresh original-header bindgen, using every genuine saved clang flag.
        args = first_saved_command(build / "rust/bindings/.bindings_generated.rs.cmd")
        bflags = fixtures.native_flags(args[args.index("--") + 1:], build, "c")
        binding = work / "bindings.rs"
        run([args[0], ROOT / "include/linux/glob.h", "--rust-target=1.85", "--use-core", "--ctypes-prefix=ffi",
             "--allowlist-function=glob_match.*", "--no-layout-tests", "--no-doc-comments", "-o", binding, "--", *bflags])
        args = first_saved_command(build / "lib/math/.int_log_rust.o.cmd")
        while "=" in args[0]: args.pop(0)
        rflags, iterator = [], iter(fixtures.native_flags(args[1:], build, "rust"))
        for arg in iterator:
            if arg == "--extern":
                dep = next(iterator)
                if dep.split("=", 1)[0] != "kernel": rflags += [arg, dep]
            elif arg != "--extern=kernel" and not arg.startswith("--extern=kernel="): rflags.append(arg)
        rflags += ["--out-dir=" + str(work)]
        self.assertIn("-Dwarnings", rflags); self.assertIn("-Zsanitizer=kcfi", rflags)
        if arch == "x86_64":
            # This genuine kernel target declares kernel-address support;
            # the host Linux target supports address instead. No target
            # capability is invented merely to make the policy probe pass.
            policy = work / "sanitizer_policy.rs"
            policy.write_text('//! Native accumulated sanitizer policy probe.\n/// Identity callback.\n'
                              '#[no_mangle]\npub extern "C" fn policy_callback(value: u32) -> u32 { value }\n')
            combined = [*rflags, "-Zsanitizer=kernel-address"]
            checker.verify_flag_policy(combined, "rust")
            run([self.inputs["RUSTC"], *combined, "--emit=obj=" + str(work / "sanitizer-policy.o"), policy])
            checker.provider_type_ids(work / "sanitizer-policy.o", names=("policy_callback",))
        facade = work / "kernel.rs"
        facade.write_text('//! Private genuine original-header forwarding; not final kernel bindings proof.\n'
            'pub use real_kernel::*;\n#[allow(missing_docs, non_camel_case_types)] pub mod bindings {\n'
            'pub use real_kernel::bindings::*;\nuse real_kernel::ffi;\ninclude!("' + str(binding) + '");\n}\n')
        library = work / "libglob_fixture_kernel.rlib"
        rustc = self.inputs["RUSTC"]
        run([rustc, *rflags, "--extern", "real_kernel=" + str(build / "rust/libkernel.rmeta"),
             "--crate-name=glob_fixture_kernel", "--emit=link", facade, "-o", library])
        source = work / "glob_rust_main.rs"; source.write_text(fixtures.caller_source(ROOT, "rust"))
        obj = work / "glob_rust_main.o"
        rargs = [rustc, *rflags, "--extern", "kernel=" + str(library), "--cfg=MODULE", "--crate-name=glob_rust_main"]
        run([*rargs, "--emit=obj=" + str(obj), source])
        checker.verify_guarded_calls(obj, arch, types, wrappers=checker.WRAPPERS)
        checker.verify_rust_entrypoints(obj, arch)
        provider = work / "rust-provider.o"
        run([rustc, *rflags, "--extern", "kernel=" + str(build / "rust/libkernel.rmeta"),
             "--emit=obj=" + str(provider), ROOT / "lib/glob_rust.rs"])
        self.assertEqual(checker.provider_type_ids(provider, names=checker.SYMBOLS), types)
        for image in (provider, work / "c-provider.o"):
            records = checker.read_exports(image)
            self.assertEqual(sorted(r['name'] for r in records), sorted(checker.SYMBOLS))
            self.assertTrue(all(r['license'] == '' and r['namespace'] == '' for r in records))
        # Real fixdep and gendwarfksyms records from fresh object compilations.
        # This is explicitly an object fixture, not a fabricated kernel image.
        provenance = work / "provenance"
        (provenance / "lib").mkdir(parents=True)
        for name in ("rust", "include", "arch", "scripts"):
            (provenance / name).symlink_to(build / name, target_is_directory=True)
        owner = provenance / "lib/glob.o"
        command_file = owner.with_name(".glob.o.cmd")
        records_by_language = {}
        for language in ("C", "Rust"):
            dep = provenance / "glob.d"
            if language == "C":
                cmd = [compiler, *[f.replace("base64", "glob") for f in cflags], "-MD", "-MF", dep,
                       "-c", ROOT / "lib/glob.c", "-o", owner]
            else:
                cmd = [rustc, *rflags, "--extern", "kernel=" + str(build / "rust/libkernel.rmeta"),
                       "--emit=obj=" + str(owner), "--emit=dep-info=" + str(dep), ROOT / "lib/glob_rust.rs"]
            run(cmd)
            fixed = run([build / "scripts/basic/fixdep", dep, "lib/glob.o", shlex.join(map(str, cmd))]).stdout
            crc = self.run_command([build / "scripts/gendwarfksyms/gendwarfksyms", owner], cwd=work, env=env,
                                  input=b"glob_match\nglob_match_len\n").stdout
            records = dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", crc))
            self.assertEqual(set(records), {s.encode() for s in checker.SYMBOLS})
            records_by_language[language] = records
            command_file.write_bytes(fixed + crc)
            symvers = provenance / "Module.symvers"
            symvers.write_bytes(b"".join(value + b"\t" + name + b"\tvmlinux\tEXPORT_SYMBOL\t\n" for name, value in records.items()))
            checker.verify_owner(provenance, owner, language, arch)
            saved = command_file.read_bytes()
            source_name = b"glob_rust.rs" if language == "Rust" else b"glob.c"
            for old, new in ((b"source_lib/glob.o := ", b"source_wrong := "), (source_name, b"wrong-source"),
                             (b"#SYMVER glob_match ", b"#SYMVER wrong ")):
                command_file.write_bytes(saved.replace(old, new))
                with self.assertRaises((ValueError, OSError)): checker.verify_owner(provenance, owner, language, arch)
            command_file.write_bytes(saved)
            if language == "Rust":
                changed = saved.replace(str(ROOT / "lib/glob.rs").encode(), b"missing-glob-core")
                command_file.write_bytes(changed)
                with self.assertRaises((ValueError, OSError)): checker.verify_owner(provenance, owner, language, arch)
                command_file.write_bytes(saved)
            stamp = owner.stat().st_mtime_ns
            os.utime(owner, ns=(1, 1))
            with self.assertRaises(ValueError): checker.verify_owner(provenance, owner, language, arch)
            os.utime(owner, ns=(stamp, stamp))
            data = symvers.read_bytes()
            for changed in (data.replace(b"EXPORT_SYMBOL", b"EXPORT_SYMBOL_GPL"), data.replace(b"vmlinux", b"glob_module"), data + data.splitlines()[0] + b"\n"):
                symvers.write_bytes(changed)
                with self.assertRaises(ValueError): checker.verify_owner(provenance, owner, language, arch)
            symvers.write_bytes(data)
        # DWARF language encodings may yield distinct CRCs despite equal CFI
        # prototypes. Never substitute the C CRC for the selected Rust owner.
        self.log.write("Actual per-language DWARF CRCs: " + repr(records_by_language) + "\n")
        archive = provenance / "vmlinux.a"
        run(["llvm-ar", "crT", archive, owner])
        self.assertEqual(checker.linked_owner(provenance), ({owner.resolve()}, owner.resolve()))
        opposite = owner.with_name("glob_rust.o")
        shutil.copyfile(owner, opposite)
        # An orphan file cannot change selection; membership can.
        self.assertEqual(checker.linked_owner(provenance)[1], owner.resolve())
        run(["llvm-ar", "qT", archive, opposite])
        with self.assertRaises(ValueError): checker.linked_owner(provenance)
        archive.unlink()
        run(["llvm-ar", "crT", archive, opposite])
        with self.assertRaises(ValueError): checker.linked_owner(provenance)
        archive.unlink()
        run(["llvm-ar", "qT", archive, owner, owner])
        with self.assertRaises(ValueError): checker.linked_owner(provenance)
        # Compile both missing-guard and wrong-signature/dispatcher controls.
        broken = work / "unguarded.o"
        run([compiler, *flags("glob_c_main"), "-fno-sanitize=kcfi", "-c", work / "glob_c_main.c", "-o", broken])
        with self.assertRaises(ValueError): checker.verify_guarded_calls(broken, arch, types, wrappers=checker.WRAPPERS)
        mutated = work / "dispatcher.rs"
        text = source.read_text().replace('core::ptr::read_volatile(&actual)(p, s)', 'glob_reference_match(p, s)').replace(
            'core::ptr::read_volatile(&actual)(p, s, n)', 'glob_reference_len(p, s, n)').replace('let actual:', 'let _actual:')
        mutated.write_text(text)
        run([*rargs, "--emit=obj=" + str(broken), mutated])
        with self.assertRaises(ValueError): checker.verify_guarded_calls(broken, arch, types, wrappers=checker.WRAPPERS)
        # Do not fabricate CFI constants: wrong types come from real C code.
        wrong = work / "wrong.c"
        wrong.write_text('int glob_match(int x) { return x; }\nint glob_match_len(int x) { return x; }\n')
        run([compiler, *flags("wrong"), "-Wno-missing-prototypes", "-c", wrong, "-o", work / "wrong.o"])
        wrong_types = checker.provider_type_ids(work / "wrong.o", names=checker.SYMBOLS)
        for image in (obj, work / "glob_c_main.o"):
            with self.assertRaises(ValueError): checker.verify_guarded_calls(image, arch, wrong_types, wrappers=checker.WRAPPERS)
        self.log.write("PASS actual saved-flags caller/provider/header/guard objects. UNPROVEN final module/VM; private binding facade only.\n")
        self.suite_registration(build, arch, compiler, cflags, run)
        selection = "Rust" if checker.configuration(build).get("RUST_GLOB") == "y" else "C"
        checker.verify_linked_implementation(build, selection)

    def suite_registration(self, build, arch, compiler, cflags, run):
        work = self.work
        makefile = (ROOT / "lib/tests/Makefile").read_text()
        self.assertNotRegex(makefile, r"(?m)^\s*(?:CFLAGS_(?:REMOVE_)?glob_kunit\.o|ccflags[^\s]*)\s*[:+?]?=")
        flags = [flag.replace("lib/base64", "lib/tests/glob_kunit").replace('"base64"', '"glob_kunit"')
                 if flag.startswith("-DKBUILD_") else "-D__KBUILD_MODNAME=glob_kunit"
                 if flag.startswith("-D__KBUILD_MODNAME=") else flag for flag in cflags]
        expected = checker.kunit_callback_types(build, arch)
        self.log.write("Actual framework callback identities: " + repr(expected) + "\n")
        for builtin in (False, True):
            obj = work / ("glob-suite-" + ("y" if builtin else "m") + ".o")
            run([compiler, *flags, *([] if builtin else ["-DMODULE"]), "-Werror", "-c", ROOT / "lib/tests/glob_kunit.c", "-o", obj])
            checker.verify_suite_registration(obj, arch, expected_types=expected)
            fields = checker.metadata_fields(obj)
            prefix = b"glob_kunit." if builtin else b""
            checker.require_metadata_field(fields, prefix + b"license", b"Dual MIT/GPL")
            checker.require_metadata_field(fields, prefix + b"description", b"Test cases for glob functions")
            linked = obj.with_suffix(".linked.o")
            run(["ld.lld", "-r", "-o", linked, obj])
            checker.verify_suite_registration(linked, arch, expected_types=expected)
            self.registration_byte_negatives(obj, arch, expected)
        for label, extra in (("wrong-tag", "-Dkunit=wrong_kunit"), ("disabled-cfi", "-fno-sanitize=kcfi")):
            obj = work / ("glob-suite-" + label + ".o")
            run([compiler, *flags, extra, "-Werror", "-c", ROOT / "lib/tests/glob_kunit.c", "-o", obj])
            checker.verify_suite_registration(obj, arch, cfi=False)
            if label == "wrong-tag":
                wrong = checker.provider_type_ids(obj, names=tuple(expected))
                for name in expected: self.assertNotEqual(wrong[name], expected[name])
            with self.assertRaises(ValueError): checker.verify_suite_registration(obj, arch, expected_types=expected)
        self.log.write("PASS unchanged original glob suite y/m graph and genuine wrong-tag/disabled-CFI controls; private objects, not final module/VM proof.\n")

    def registration_byte_negatives(self, obj, arch, expected):
        original = obj.read_bytes()
        _, sections, _, _, _ = checker.module_elf(obj)
        table = struct.unpack_from("<Q", original, 40)[0]
        raw = [struct.unpack_from("<IIQQQQIIQQ", original, table + i * 64) for i in range(len(sections))]
        def location(symbol, addend=0): return raw[symbol[3]][4] + symbol[4] + addend - sections[symbol[3]][3]
        def relocation(owner, offset):
            records = [(i, j, record) for i, section in enumerate(sections) if section[1] == 4 and section[6] == owner
                       for j, record in enumerate(section[-1]) if record[0] == offset]
            self.assertEqual(len(records), 1)
            index, number, record = records[0]
            return raw[index][4] + 24 * number, record
        registration = next(i for i, section in enumerate(sections) if section[0] == b".kunit_test_suites")
        at, record = relocation(registration, 0)
        suite, addend = record[2:]
        suite_offset = suite[4] + addend - sections[suite[3]][3]
        case_at, case_record = relocation(suite[3], suite_offset + 288)
        cases, extra = case_record[2:]
        case_offset = cases[4] + extra - sections[cases[3]][3]
        changes = [("registration-relocation", at, struct.pack("<Q", 8)),
                   ("registration-target", at + 16, struct.pack("<q", addend + 8)),
                   ("suite-name", location(suite, addend), b"!"),
                   ("suite-lifecycle", location(suite, addend + 256), b"!"),
                   ("detached-case-array", case_at, struct.pack("<Q", suite_offset + 280)),
                   ("case-array-target", case_at + 16, struct.pack("<q", extra + 72)),
                   ("case-attributes", location(cases, extra + 24), b"!"),
                   ("terminator", location(cases, extra + 72), b"!")]
        callbacks = checker.verify_suite_registration(obj, arch, expected_types=expected)
        for name, symbol in callbacks.items(): changes.append(("type-" + name, location(symbol) - 4, bytes(4)))
        for field, label in ((8, "case-name"), (56, "module-name")):
            _, record = relocation(cases[3], case_offset + field)
            changes.append((label, location(*record[2:]), b"!"))
        callback_at, callback = relocation(cases[3], case_offset)
        generator_at, generator = relocation(cases[3], case_offset + 16)
        changes += [("wrong-case", callback_at + 8, original[generator_at + 8:generator_at + 24]),
                    ("wrong-generator", generator_at + 8, original[callback_at + 8:callback_at + 24]),
                    ("missing-generator", generator_at, struct.pack("<Q", case_offset + 64))]
        for label, offset, replacement in changes:
            changed = bytearray(original)
            changed[offset:offset + len(replacement)] = replacement
            bad = obj.with_name(obj.stem + "-bad-" + label + ".o")
            bad.write_bytes(changed)
            with self.subTest(arch=arch, label=label), self.assertRaises(ValueError):
                checker.verify_suite_registration(bad, arch, expected_types=expected)

    def test_x86_saved_flags_original_header_and_protected_objects(self): self.native("X86_BUILD", "x86_64")
    def test_arm64_saved_flags_original_header_and_protected_objects(self): self.native("ARM64_BUILD", "aarch64")

    def test_concurrent_native_compilers_keep_donors_read_only(self):
        for key in ("X86_BUILD", "ARM64_BUILD"):
            if key not in self.inputs: self.skipTest("no GLOB_RUNTIME_" + key + "; concurrent native proof unavailable")
        work = self.workspace()
        command = [sys.executable, "-m", "unittest", "test_glob_kernel_check.NativeTests.test_x86_saved_flags_original_header_and_protected_objects",
                   "test_glob_kernel_check.NativeTests.test_arm64_saved_flags_original_header_and_protected_objects", "-v"]
        env = fixtures.compiler_environment(work, {**os.environ, "GLOB_RUNTIME_LOGS": str(work),
            "GLOB_SOURCE_ROOT": str(ROOT), "GLOB_RUNTIME_RUSTC": str(self.inputs["RUSTC"]), "PYTHONDONTWRITEBYTECODE": "1",
            "PYTHONPATH": str(Path(__file__).resolve().parent) + os.pathsep + os.environ.get("PYTHONPATH", "")})
        jobs, watches = [], []
        try:
            with ExitStack() as stack:
                for key in ("X86_BUILD", "ARM64_BUILD"):
                    watches.append(stack.enter_context(fixtures.NativeWriteWatch(self.inputs[key])))
                for index in range(2):
                    log = work / f"concurrent-{index}.log"
                    output = stack.enter_context(log.open("wb"))
                    jobs.append((log, subprocess.Popen(command, cwd=work, env=env, stdout=output, stderr=subprocess.STDOUT)))
                for _, process in jobs: process.wait(timeout=240)
        finally:
            for _, process in jobs:
                if process.poll() is None: process.kill(); process.wait()
        observations = [{"root": str(watch.root), "directories": len(watch.paths), "writes": watch.events} for watch in watches]
        (work / "concurrent-observation.json").write_text(json.dumps({"command": command, "donors": observations}, indent=2) + "\n")
        for log, process in jobs:
            self.assertEqual(process.returncode, 0, log.read_text()[-6000:])
            self.assertNotIn("skipped", log.read_text())
        self.assertTrue(all(not watch.events for watch in watches), observations)


class CliTests(PrivateBuild):
    """Mock only orchestration boundaries; no synthetic CFI, CRC or bindings.

    Real object gates are exercised by NativeTests. These tests cannot count as
    module linkage or runtime evidence and never execute make/boot_kernel.
    """
    def test_preflight_requirements_before_any_output(self):
        work = self.workspace()
        build = work / "input"; build.mkdir()
        good = dict(GLOB="y", RUST_GLOB="y", RUST="y", X86_64="y", **{
            k: "y" for k in ("64BIT", "MODULES", "MODULE_UNLOAD", "PRINTK", "MULTIUSER", "CFI", "MODVERSIONS")})
        for changes in ({"GLOB": "m"}, {"RUST_GLOB": "n"}, {"MODULES": "n"}, {"CFI": "n"},
                        {"MODVERSIONS": "n"}, {"MODULE_UNLOAD": "n"}, {"MODULE_SIG_FORCE": "y"},
                        {"GLOB_KUNIT_TEST": "y", "KUNIT": "m"}):
            config = {**good, **changes}
            (build / ".config").write_text(''.join(f"CONFIG_{k}={v}\n" for k, v in config.items()))
            with mock.patch.object(checker, "verify_linked_implementation") as linked, \
                    mock.patch.object(checker, "private_copy") as copied, redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                checker.main([str(build), "--reload-modules", "--output-parent", str(work)])
            linked.assert_not_called(); copied.assert_not_called()
            self.assertEqual([p.name for p in build.iterdir()], [".config"])
        for caller in ("c", "rust"):
            cfg = {**good, "RUST_GLOB": "n"}
            (build / ".config").write_text(''.join(f"CONFIG_{k}={v}\n" for k, v in cfg.items()))
            with mock.patch.object(checker, "verify_linked_implementation", side_effect=ValueError("real owner gate")) as linked, \
                    mock.patch.object(checker, "private_copy") as copied, redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                checker.main([str(build), "--caller", caller, "--allow-c-baseline", "--output-parent", str(work)])
            linked.assert_called_once_with(build, "C"); copied.assert_not_called()
        for extra in (("--qemu", ""), ("--qemu", str(work / "missing-qemu")), ("--qemu", "true\0--version"),
                      ("--qemu-data", str(work / "missing-data")), ("--output-parent", ""),
                      ("--make-arg=O=" + str(build),), ("--make-arg=-C",), ("--make-arg=LLVM=",)):
            with mock.patch.object(checker, "validate_inputs") as validate, \
                    mock.patch.object(checker, "private_copy") as copied, redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                checker.main([str(build), *extra])
            validate.assert_not_called(); copied.assert_not_called()
        for make in ("", "  ", "'unfinished", str(work / "missing-make"), str(work)):
            before = set(work.rglob("*"))
            with mock.patch.dict(os.environ, {"MAKE": make}), mock.patch.object(checker, "validate_inputs") as validate, \
                    mock.patch.object(checker, "private_copy") as copied, mock.patch.object(checker.tempfile, "mkdtemp") as created, \
                    redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                checker.main([str(build), "--output-parent", str(work)])
            validate.assert_not_called(); copied.assert_not_called(); created.assert_not_called()
            self.assertEqual(set(work.rglob("*")), before)
        real_which = shutil.which
        for arch in ("x86_64", "aarch64"):
            data = ({}, arch, "n", "n", "Rust", [])
            before = set(work.rglob("*"))
            def missing_qemu(command):
                return None if command == "qemu-system-" + arch else real_which(command)
            with mock.patch.dict(os.environ, {"MAKE": real_which("make")}, clear=True), \
                    mock.patch.object(checker.shutil, "which", side_effect=missing_qemu), \
                    mock.patch.object(checker, "validate_inputs", return_value=data) as validate, \
                    mock.patch.object(checker, "private_copy") as copied, mock.patch.object(checker.tempfile, "mkdtemp") as created:
                with redirect_stderr(io.StringIO()) as errors, self.assertRaises(SystemExit):
                    checker.main([str(build), "--output-parent", str(work)])
                self.assertIn("invalid default QEMU executable", errors.getvalue())
                validate.assert_called_once(); copied.assert_not_called(); created.assert_not_called()
                # Read-only artifact checking needs no default runtime executable.
                with redirect_stdout(io.StringIO()):
                    checker.main([str(build), "--check-only", "--output-parent", str(work)])
                copied.assert_not_called(); created.assert_not_called()
            self.assertEqual(set(work.rglob("*")), before)

    def test_private_copy_dereferences_outputs_and_preserves_inputs(self):
        work = self.workspace()
        build = work / "input"; build.mkdir()
        original = build / "real"; original.write_text("read-only output")
        (build / "alias").symlink_to(original)
        (build / "source").symlink_to(ROOT, target_is_directory=True)
        dest = work / "copy"
        checker.private_copy(build, dest)
        self.assertFalse((dest / "alias").is_symlink())
        (dest / "alias").write_text("private change")
        self.assertEqual(original.read_text(), "read-only output")
        self.assertEqual((dest / "source").resolve(), ROOT)

    def test_transport_and_module_generation_matrix(self):
        work = self.workspace()
        build = work / "input"; build.mkdir()
        make = work / "quoted make"
        make.symlink_to(shutil.which("make"))
        make_argv = [str(make.absolute()), "--trace", "label=two words"]
        qemu = work / "relative qemu"
        qemu.symlink_to(shutil.which("true"))
        qemu_data = work / "qemu data"; qemu_data.mkdir()
        qemu_argument = shlex.join([os.path.relpath(qemu), "--opaque-option", "two words"])
        qemu_argv = [str(Path(os.path.relpath(qemu)).absolute()), "--opaque-option", "two words"]
        for arch in ("x86_64", "aarch64"):
            for caller in ("c", "rust"):
                for suite, framework in (("n", "n"), ("y", "y"), ("m", "y"), ("m", "m")):
                    modules = ([build / "lib/kunit/kunit.ko"] if framework == "m" else [])
                    if suite == "m": modules.append(build / "lib/tests/glob_kunit.ko")
                    calls = []
                    def copy(source, dest): dest.mkdir()
                    def run(args, **kwargs):
                        calls.append((args, kwargs))
                        if "--build" in args:
                            dest = Path(args[args.index("--build") + 1]) / "rust-boot-test"
                            dest.mkdir()
                            (dest / "console.log").write_bytes(console(caller, suite, framework == "m", True))
                        return subprocess.CompletedProcess(args, 0)
                    data = ({}, arch, suite, framework, "Rust", modules)
                    with mock.patch.object(checker, "validate_inputs", return_value=data), \
                            mock.patch.object(checker, "private_copy", side_effect=copy), \
                            mock.patch.object(checker, "verify_consumer") as consumer, \
                            mock.patch.object(checker.subprocess, "run", side_effect=run), \
                            mock.patch.dict(os.environ, {"MAKE": shlex.join([str(make), *make_argv[1:]])}):
                        checker.main([str(build), "--caller", caller, "--reload-modules", "--output-parent", str(work),
                            "--make-arg=LLVM=1", "--qemu", qemu_argument, "--qemu-data", os.path.relpath(qemu_data)])
                    self.assertEqual(len(calls), 3)
                    for command, _ in calls[:2]: self.assertEqual(command[:len(make_argv)], make_argv)
                    self.assertEqual(consumer.call_count, 2)
                    consumer.assert_any_call(consumer.call_args.args[0], consumer.call_args.args[1], "c", license="Proprietary")
                    boot = calls[-1][0]
                    self.assertEqual(boot[boot.index("--arch") + 1], arch)
                    self.assertEqual(shlex.split(boot[boot.index("--qemu") + 1]), qemu_argv)
                    self.assertEqual(boot[boot.index("--qemu-data") + 1], str(qemu_data.resolve()))
                    self.assertEqual(boot.count("--preload-module"), len(modules))
                    self.assertIn("--reload-modules", boot)
                    for args, kwargs in calls:
                        self.assertTrue(kwargs["check"])
                        self.assertNotIn("MAKEFLAGS", kwargs["env"])
                    copied = Path(boot[boot.index("--build") + 1])
                    self.assertNotEqual(copied, build)
                    module_dir = copied.parent / "consumer"
                    filename = "glob_" + caller + "_main" + (".rs" if caller == "rust" else ".c")
                    self.assertEqual((module_dir / filename).read_text(), fixtures.caller_source(ROOT, caller))
                    self.assertEqual((module_dir / "glob_reference.c").read_text(), fixtures.reference_source(ROOT))
        self.assertEqual(list(build.iterdir()), [])

    def test_consumer_source_mutations_fail_closed_before_metadata(self):
        work = self.workspace()
        # Only the outer ELF/identity read is isolated; the exact durable
        # source gate must reject a C dispatcher even when compilation passed.
        for caller in ("c", "rust"):
            build = work / caller; build.mkdir()
            (build / ".config").write_text("CONFIG_X86_64=y\nCONFIG_64BIT=y\n")
            main = build / ("glob_" + caller + "_main" + (".rs" if caller == "rust" else ".c"))
            main.write_text(fixtures.caller_source(ROOT, caller) + "\n/* substituted C dispatcher */\n")
            with mock.patch.object(checker, "elf_target"), \
                    mock.patch.object(checker, "module_name", return_value="glob_rust_abi" if caller == "rust" else "glob_abi"), \
                    self.assertRaisesRegex(ValueError, "caller source changed"):
                checker.verify_consumer(build, build, caller)


if __name__ == "__main__": unittest.main(verbosity=2)
