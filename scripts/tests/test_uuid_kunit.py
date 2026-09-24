# SPDX-License-Identifier: GPL-2.0-only
"""UUID KUnit original-suite event parity, real layouts and native build rules.

HOSTCC/HOSTRUSTC/BINDGEN select tools. UUID_KUNIT_I686_SYSROOT (or the shared
INT_MATH_I686_SYSROOT) enables genuine32 execution, with an optional
UUID_KUNIT_I686_RUNNER. UUID_KUNIT_NATIVE_X86/ARM64 name optional completed
read-only strict native builds. Missing optional inputs skip only their gates;
explicit empty/invalid inputs and sandbox SIGSYS fail, never pass.
"""
import json
import os
from pathlib import Path
import re
import signal
import resource
import shlex
import shutil
import subprocess
import tempfile
import unittest
from unittest import mock

import test_kunit_parameters as support
import test_list_sort_kunit as listing
from test_polynomial_build import headers
from test_rational_build import environment, module_info, run
from test_int_math_translation import rust_flags

ROOT = Path(__file__).resolve().parents[2]
SUITE = ROOT / "lib/tests/uuid_kunit.rs"
MAKEFILE = ROOT / "lib/tests/Makefile"
KCONFIG = ROOT / "lib/Kconfig.debug"
FIXTURES = ROOT / "scripts/tests/uuid_kunit_fixtures"
NAMES = ("uuid_test_guid_valid", "uuid_test_uuid_valid", "uuid_test_guid_invalid",
         "uuid_test_uuid_invalid", "uuid_test_uuid_gen", "uuid_test_guid_gen",
         "uuid_test_generate_random_uuid", "uuid_test_generate_random_guid")


def fixture_header():
    text = support.original_header().replace('KBUILD_MODNAME "parameter_fixture"',
                                             'KBUILD_MODNAME "uuid_kunit"')
    original = (ROOT / "include/kunit/test.h").read_text()
    for name in ("KUNIT_CASE", "__kunit_test_suites", "kunit_test_suites", "kunit_test_suite",
                 "KUNIT_UNARY_ASSERTION", "KUNIT_TRUE_MSG_ASSERTION",
                 "KUNIT_EXPECT_TRUE", "KUNIT_EXPECT_TRUE_MSG"):
        text += support.macro(original, name)
    text += """
typedef unsigned char __u8;
#define __must_check
#define __initdata
#define __used __attribute__((used))
#define __section(s) __attribute__((section(s)))
int memcmp(const void *,const void *,size_t);
void *memcpy(void *,const void *,size_t);
void get_random_bytes(void *,size_t);
#define EINVAL 22
int hex_to_bin(unsigned char);
"""
    # Entire original UUID declarations/inlines/macros; only includes are
    # replaced by the genuine minimal type and stdio transport declarations.
    text += re.sub(r'^#include.*$', '', (ROOT / "include/linux/uuid.h").read_text(), flags=re.M)
    return "#ifndef UUID_KUNIT_FIXTURE_H\n#define UUID_KUNIT_FIXTURE_H\n"+text+"\n#endif\n"


def tool(name, fallback):
    args = shlex.split(os.environ.get(name, fallback))
    if not args or not shutil.which(args[0]):
        raise ValueError(name+" must name an executable")
    return args


def native_input(name):
    if name not in os.environ:
        return None
    if not os.environ[name].strip():
        raise ValueError(name+" explicitly empty")
    path = Path(os.environ[name]).resolve()
    for item in ("rust/libkernel.rmeta", "rust/libbindings.rmeta", "include/generated/rustc_cfg",
                 "rust/bindings/bindings_generated.rs", ".config", "lib/.scatterlist.o.cmd",
                 "lib/.list_sort_rust.o.cmd", "scripts/basic/fixdep", "scripts/kconfig/conf"):
        if not (path / item).is_file():
            raise ValueError(name+" missing "+item)
    expected = "CONFIG_ARM64=y" if name.endswith("ARM64") else "CONFIG_X86_64=y"
    config = (path / ".config").read_text().splitlines()
    if expected not in config or "CONFIG_RUST=y" not in config or "CONFIG_CFI=y" not in config:
        raise ValueError(name+" must identify its named Rust/CFI native architecture")
    return path


def native_c_flags(native):
    text = MAKEFILE.read_text()
    if re.search(r'^\s*CFLAGS_(?:REMOVE_)?uuid_kunit\.o\s*[:+?]?=', text, re.M):
        raise AssertionError("additional UUID suite C flags require explicit audit")
    compiler, flags = listing.native_c_flags(native)
    return compiler, [a.replace("test_list_sort", "uuid_kunit") for a in flags]


class Fixture:
    def __init__(self, work, sysroot=None, kcfi=False):
        self.work = work
        self.target = [] if sysroot is None else ["--target=i686-unknown-linux-gnu", "--sysroot="+str(sysroot)]
        self.env = {**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": "lib/tests/uuid_kunit"}
        self.rustc = tool("HOSTRUSTC", "rustc")
        self.cc = tool("HOSTCC", "cc")
        if kcfi:
            self.cc = tool("CLANG", "clang")
        found = shutil.which("bindgen") or shutil.which("bindgen-0.71")
        if found is None and "BINDGEN" not in os.environ:
            raise unittest.SkipTest("BINDGEN required for original target KUnit/UUID declarations")
        bindgen = tool("BINDGEN", found or "bindgen")
        self.cflags = [*headers(work), "-I"+str(work), "-funsigned-char", "-fno-builtin",
                       "-fno-stack-protector", "-fno-pie",
                       "-D__KERNEL__", '-DKBUILD_MODFILE="lib/tests/uuid_kunit"',
                       *(["-m32"] if sysroot else [])]
        if kcfi:
            self.cflags += ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        include = work / "include/linux"
        with (include / "module.h").open("a") as output:
            output.write(support.macro((ROOT / "include/linux/module.h").read_text(), "MODULE_AUTHOR"))
        (work / "fixture.h").write_text(fixture_header())
        run([*bindgen, work / "fixture.h", "--use-core", "--rust-target=1.85",
             "--ctypes-prefix=crate::ffi", "--no-layout-tests", "--no-doc-comments",
             "--no-derive-debug", "--no-derive-copy",
             "--allowlist-type=kunit.*|uuid_t|guid_t", "--allowlist-function=.*kunit.*",
             "--allowlist-var=KUNIT.*|EINVAL", "-o", work / "bindings.rs",
             "--", *self.cflags, "-x", "c"])
        (work / "kernel.rs").write_text(support.kernel_facade())
        self.library = work / "libkernel.rlib"
        run([*self.rustc, *rust_flags("2"), *self.target, "--crate-name=kernel",
             "--crate-type=rlib", work / "kernel.rs", "-o", self.library], env=self.env)
        # Keep the relative source import in production; private compile uses
        # a copied suite with only its source path resolved.
        self.suite_source = work / "uuid_kunit.rs"
        self.suite_source.write_text(SUITE.read_text().replace("../../include/linux/uuid_header.rs",
            str(ROOT / "include/linux/uuid_header.rs")))
        self.rflags = [*self.target, "--extern", "kernel="+str(self.library),
                       "-Ldependency="+str(work), "-Zcrate-attr=no_std",
                       "-Zcrate-attr=feature(used_with_arg)", "-Crelocation-model=static",
                       "-Zbinary_dep_depinfo=y"]
        if kcfi:
            self.rflags += ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
        (work / "suite.c").write_text('#include <linux/module.h>\n#include "fixture.h"\n'+
            re.sub(r'^#include.*$', '', (ROOT / "lib/tests/uuid_kunit.c").read_text(), flags=re.M))
        provider = '#include "fixture.h"\n#define EXPORT_SYMBOL(x)\n#define EXPORT_SYMBOL_GPL(x)\n'
        provider += re.sub(r'^#include.*$', '', (ROOT / "include/linux/ctype.h").read_text(), flags=re.M)
        provider += re.sub(r'^#include.*$', '', (ROOT / "lib/ctype.c").read_text(), flags=re.M)
        provider += re.sub(r'^#include.*$', '', (ROOT / "lib/uuid.c").read_text(), flags=re.M)
        provider += re.search(r'int hex_to_bin\(.*?\n}', (ROOT / "lib/hexdump.c").read_text(), re.S)[0]
        (work / "provider.c").write_text(provider)
        (work / "driver.c").write_text((FIXTURES / "driver.c.txt").read_text())
        (work / "suite.lds").write_text("SECTIONS { .kunit_test_suites : { __suites_start = .; "
            "KEEP(*(.kunit_test_suites)); __suites_end = .; } /DISCARD/ : { "
            "*(.eh_frame .eh_frame.* .eh_frame_hdr) *(.gcc_except_table .gcc_except_table.* "
            ".data.DW.ref.rust_eh_personality) } } INSERT AFTER .data;\n")

    def suite(self, rust, module=False, opt="2", source=None):
        out = self.work / f"suite-{rust}-{module}-{opt}.o"
        if rust:
            run([*self.rustc, *rust_flags(opt), *self.rflags, "--crate-name=uuid_kunit",
                 "--crate-type=rlib", *(["--cfg=MODULE"] if module else []),
                 "--emit=obj="+str(out), "--emit=dep-info="+str(out)+".d",
                 source or self.suite_source], env=self.env)
        else:
            run([*self.cc, *self.cflags, "-O"+opt, *(["-DMODULE"] if module else []),
                 "-c", self.work / "suite.c", "-o", out])
        return out

    def executable(self, rust, opt="2", source=None):
        suite = self.suite(rust, opt=opt, source=source)
        provider = self.work / "provider.o"
        run([*self.cc, *self.cflags, "-O2", "-c", self.work / "provider.c", "-o", provider])
        panic = self.work / "panic.rs"
        panic.write_text("//! Real panic transport fails.\n#![no_std]\n"+support.panic_handler())
        run([*self.rustc, *rust_flags("2"), *self.target, "--crate-type=staticlib", panic,
             "-o", self.work / "panic.a"], env=self.env)
        binary = self.work / f"run-{rust}-{opt}"
        transport = []
        if self.target:
            path = self.work / "elf32.c"
            path.write_text((ROOT / "scripts/tests/list_sort_kunit_elf32.c.txt").read_text())
            transport = ["-ffreestanding", "-fno-stack-protector", "-fno-pie",
                         "-nostdlib", "-static", "-Wl,-e,_start", path]
        wrap = ["guid_parse", "uuid_parse", "guid_gen", "uuid_gen",
                "generate_random_uuid", "generate_random_guid"]
        run([*self.cc, *self.cflags, "-O2", "-no-pie", "-Wl,--gc-sections", *transport,
             "-Wl,-T,"+str(self.work / "suite.lds"), self.work / "driver.c", suite, provider,
             self.library, self.work / "panic.a", *["-Wl,--wrap="+x for x in wrap], "-o", binary])
        return binary


class UUIDKunitTests(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory(prefix="uuid-kunit-")
        self.addCleanup(temp.cleanup)
        self.work = Path(temp.name)

    def test_original_full_traces_and_nonfatal_mutations(self):
        fixture = Fixture(self.work)
        oracle = fixture.executable(False)
        expected = {mode: run([oracle, str(mode), "42"]).stdout for mode in range(17)}
        self.assertIn(b"END 8 12 32 0\n", expected[0])
        self.assertIn(b"END 8 12 32 82\n", expected[16])
        self.assertEqual(expected[0].count(b"CASE "), 8)
        self.assertEqual(expected[0].count(b"RNG "), 32)
        self.assertEqual(expected[16].count(b"FAIL "), 82)
        for opt in ("0", "2", "s"):
            binary = fixture.executable(True, opt)
            for mode in range(17):
                self.assertEqual(run([binary, str(mode), "42"]).stdout, expected[mode], (opt, mode))
            for seed in ("0", "1", "4294967295"):
                self.assertEqual(run([binary, "0", seed]).stdout,
                                 run([oracle, "0", seed]).stdout)
        for spelling in (b"guid_parse(data->uuid, &le)", b"uuid_parse(data->uuid, &be)",
                         b"guid_equal(&data->le, &le)", b"uuid_equal(&data->be, &be)",
                         b"guid_parse(uuid, &le)", b"uuid_parse(uuid, &be)",
                         b"u.b[6] & 0xf0", b"g.b[7] & 0xf0", b"buf[8] & 0xc0"):
            self.assertIn(spelling, expected[16])

    def test_metadata_and_no_false_allocation_or_abort_imports(self):
        fixture = Fixture(self.work)
        for module in (False, True):
            original, candidate = fixture.suite(False, module), fixture.suite(True, module)
            self.assertEqual(module_info(original), module_info(candidate))
            symbols = run(["nm", candidate]).stdout
            for name in ("guid_parse", "uuid_parse", "uuid_gen", "guid_gen",
                         "generate_random_uuid", "generate_random_guid",
                         "__kunit_do_failed_assertion", "kunit_binary_assert_format",
                         "kunit_unary_assert_format"):
                self.assertIn((" U "+name+"\n").encode(), symbols)
            self.assertNotRegex(symbols, rb" U (?:__kunit_abort|kunit_kmalloc.*|kunit_get_current_test)\n")
            self.assertNotRegex(symbols, rb"\b(init_module|cleanup_module)\n")
            self.assertEqual(b"__IS_RUST_MODULE" in symbols, module)
            deps = Path(str(candidate)+".d").read_text()
            self.assertIn("uuid_header.rs", deps)
            self.assertIn(str(fixture.library), deps)

    def test_wrong_assertion_and_random_loop_negative_controls(self):
        fixture = Fixture(self.work)
        oracle = fixture.executable(False)
        expected = run([oracle, "16", "42"]).stdout
        source = fixture.suite_source.read_text()
        mutants = [
            source.replace("0..8", "0..7"),
            source.replace('c"g.b[7] & 0xf0"', 'c"incorrect operand"'),
            source.replace('test_suite!("uuid",', 'test_suite!("wrong",'),
            source.replace('expected_true: true', 'expected_true: false'),
        ]
        for index, mutant in enumerate(mutants):
            self.assertNotEqual(source, mutant)
            path = self.work / f"mutant-{index}.rs"
            path.write_text(mutant)
            binary = fixture.executable(True, source=path)
            actual = subprocess.run([binary, "16", "42"], capture_output=True, timeout=20)
            self.assertTrue(actual.returncode != 0 or actual.stdout != expected, index)
        path = self.work / "panic-control.rs"
        path.write_text(source.replace('for data in &TEST_UUID {',
            'if !test.is_null() { panic!("deliberate actual core panic"); } for data in &TEST_UUID {', 1))
        binary = fixture.executable(True, source=path)
        result = subprocess.run([binary, "0", "42"], capture_output=True, timeout=20)
        self.assertEqual(result.returncode, 97)

    def test_genuine_i686_event_traces(self):
        name = "UUID_KUNIT_I686_SYSROOT" if "UUID_KUNIT_I686_SYSROOT" in os.environ else "INT_MATH_I686_SYSROOT"
        if name not in os.environ:
            if "UUID_KUNIT_I686_RUNNER" in os.environ:
                self.fail("UUID_KUNIT_I686_RUNNER requires a genuine sysroot")
            self.skipTest("genuine i686 core absent")
        if not os.environ[name].strip():
            self.fail(name+" explicitly empty")
        sysroot = Path(os.environ[name]).resolve()
        if not list((sysroot / "lib/rustlib/i686-unknown-linux-gnu/lib").glob("libcore*.rlib")):
            self.fail(name+" lacks genuine i686 core")
        runner = []
        if "UUID_KUNIT_I686_RUNNER" in os.environ:
            runner = tool("UUID_KUNIT_I686_RUNNER", "")
        fixture = Fixture(self.work, sysroot)
        oracle = fixture.executable(False)
        candidates = [fixture.executable(True, opt) for opt in ("0", "2", "s")]
        self.assertEqual(oracle.read_bytes()[:6], b"\x7fELF\x01\x01")
        for candidate in candidates:
            self.assertEqual(candidate.read_bytes()[:6], b"\x7fELF\x01\x01")
        result = subprocess.run([*runner, oracle], capture_output=True, timeout=20)
        self.assertEqual(result.returncode, 95, "runner must execute actual ELF32; SIGSYS is failure")
        for mode in range(17):
            expected = run([*runner, oracle, str(mode), "42"]).stdout
            for candidate in candidates:
                self.assertEqual(run([*runner, candidate, str(mode), "42"]).stdout, expected)

    def test_protected_real_case_callbacks_and_wrong_nominal_type(self):
        ordinary_dir = self.work / "ordinary"
        protected_dir = self.work / "protected"
        ordinary_dir.mkdir()
        protected_dir.mkdir()
        ordinary = Fixture(ordinary_dir)
        protected = Fixture(protected_dir, kcfi=True)
        expected = run([ordinary.executable(False), "16", "42"]).stdout
        self.assertEqual(run([protected.executable(False), "16", "42"]).stdout, expected)
        for opt in ("0", "2", "s"):
            self.assertEqual(run([protected.executable(True, opt), "16", "42"]).stdout, expected)
        # Same machine ABI but a wrong nominal pointee type: valid ordinary
        # invocation is a positive control; protected case dispatch must trap.
        def wrong_type(fixture):
            source = fixture.suite_source.read_text()
            source = source.replace(
                'fn uuid_test_guid_valid(test: *mut bindings::kunit) {',
                'fn uuid_test_guid_valid(test: *mut kernel::ffi::c_void) { let test = test.cast::<bindings::kunit>();')
            source = source.replace('case(c"uuid_test_guid_valid", uuid_test_guid_valid)',
                'case(c"uuid_test_guid_valid", unsafe { core::mem::transmute::<unsafe extern "C" fn(*mut kernel::ffi::c_void), unsafe extern "C" fn(*mut bindings::kunit)>(uuid_test_guid_valid) })')
            path = fixture.work / "wrong-type.rs"
            path.write_text(source)
            return fixture.executable(True, source=path)
        normal = wrong_type(ordinary)
        self.assertEqual(run([normal, "16", "42"]).stdout, expected)
        invalid = wrong_type(protected)
        result = subprocess.run([invalid, "16", "42"], capture_output=True, timeout=20,
            preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        self.assertEqual(result.returncode, -signal.SIGILL)

    def test_provenance_and_explicit_bad_inputs(self):
        marker = "// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783"
        self.assertEqual([x for x in SUITE.read_text().splitlines() if "SOURCE-COMMIT:" in x], [marker])
        self.assertNotIn("struct uuid_t", SUITE.read_text())
        self.assertNotIn("type guid_t", SUITE.read_text())
        for value in ("", "/nonexistent-uuid-kunit-fixture"):
            with mock.patch.dict(os.environ, {"UUID_KUNIT_NATIVE_X86": value}):
                with self.assertRaises(ValueError):
                    native_input("UUID_KUNIT_NATIVE_X86")
        with mock.patch.dict(os.environ, {"BINDGEN": ""}):
            with self.assertRaises(ValueError):
                Fixture(self.work)


class UUIDKunitNativeTests(unittest.TestCase):
    def setUp(self):
        self.inputs = [native_input(name) for name in ("UUID_KUNIT_NATIVE_X86", "UUID_KUNIT_NATIVE_ARM64")]
        temp = tempfile.TemporaryDirectory(prefix="uuid-kunit-native-")
        self.addCleanup(temp.cleanup)
        self.work = Path(temp.name)
        self.env = {**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": "lib/tests/uuid_kunit"}

    def test_strict_native_both_lifecycles_and_callback_kcfi(self):
        inputs = [p for p in self.inputs if p]
        if not inputs:
            self.skipTest("native inputs absent")
        source = self.work / "uuid_kunit.rs"
        source.write_text(SUITE.read_text().replace("../../include/linux/uuid_header.rs",
            str(ROOT / "include/linux/uuid_header.rs")))
        for index, native in enumerate(inputs):
            compiler, flags = listing.native_flags(native)
            ccompiler, cflags = native_c_flags(native)
            c_ir = self.work / f"original-{index}.ll"
            run([ccompiler, *cflags, "-S", "-emit-llvm", "-o", c_ir,
                 ROOT / "lib/tests/uuid_kunit.c"], cwd=native)
            def ids(text):
                values = dict(re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}', text, re.M))
                return {name: values[number] for name, number in re.findall(
                    r'^define [^\n]*?@([^ (]+)\([^\n]*!kcfi_type !(\d+)', text, re.M)}
            cids = ids(c_ir.read_text())
            for module in (False, True):
                out = self.work / f"native-{index}-{module}"
                run([compiler, *flags, *(["--cfg=MODULE"] if module else []),
                     "--crate-name=uuid_kunit", "--emit=obj="+str(out)+".o",
                     "--emit=llvm-ir="+str(out)+".ll", "--emit=dep-info="+str(out)+".d", source],
                    env=self.env, cwd=native)
                original = self.work / f"original-{index}-{module}.o"
                run([ccompiler, *cflags, *(["-DMODULE"] if module else []),
                     "-c", ROOT / "lib/tests/uuid_kunit.c", "-o", original], cwd=native)
                self.assertEqual(module_info(original), module_info(Path(str(out)+".o")))
                deps = Path(str(out)+".d").read_text()
                for name in ("uuid_header.rs", "libkernel.rmeta", "libbindings.rmeta"):
                    self.assertIn(name, deps)
                llvm = Path(str(out)+".ll").read_text()
                rids = ids(llvm)
                for name in NAMES:
                    matching = [value for sym, value in rids.items() if re.search(
                        re.escape(name)+r"(?:17h|$)", sym)]
                    self.assertEqual(matching, [cids[name]], (name, rids, cids))
                symbols = run(["nm", str(out)+".o"]).stdout
                self.assertEqual(b"__IS_RUST_MODULE" in symbols, module)
                self.assertNotRegex(symbols, rb"\b(init_module|cleanup_module)\n")
                self.assertNotIn(b" U __kunit_abort\n", symbols)

    def test_actual_kbuild_selection_kconfig_dependencies_and_noops(self):
        native = self.inputs[0]
        if native is None:
            self.skipTest("UUID_KUNIT_NATIVE_X86 absent")
        stanza = re.search(r'^config RUST_UUID_KUNIT_TEST\n.*?(?=^config |\Z)',
                           KCONFIG.read_text(), re.M | re.S)[0]
        kconfig = self.work / "Kconfig"
        original = re.search(r'^config UUID_KUNIT_TEST\n.*?(?=^config |\Z)',
                             KCONFIG.read_text(), re.M | re.S)[0]
        kconfig.write_text('config MODULES\n\tbool "Modules"\n\tmodules\n'
            'config RUST\n\tbool "Rust"\nconfig KUNIT\n\ttristate "KUnit"\n'
            'config KUNIT_ALL_TESTS\n\tbool\n'+original+stanza)
        env = {**self.env, "KCONFIG_CONFIG": str(self.work / ".config")}
        config_work = self.work / "config-work"
        config_work.mkdir()
        for config, enabled in [("", False),
            ("CONFIG_KUNIT=y\nCONFIG_RUST=y\nCONFIG_UUID_KUNIT_TEST=y\n", False),
            ("CONFIG_KUNIT=y\nCONFIG_RUST_UUID_KUNIT_TEST=y\nCONFIG_UUID_KUNIT_TEST=y\n", False),
            ("CONFIG_RUST=y\nCONFIG_RUST_UUID_KUNIT_TEST=y\n", False),
            ("CONFIG_KUNIT=y\nCONFIG_RUST=y\nCONFIG_UUID_KUNIT_TEST=y\nCONFIG_RUST_UUID_KUNIT_TEST=y\n", True),
            ("CONFIG_KUNIT=y\nCONFIG_MODULES=y\nCONFIG_RUST=y\nCONFIG_UUID_KUNIT_TEST=m\nCONFIG_RUST_UUID_KUNIT_TEST=y\n", True),
            ("CONFIG_KUNIT=m\nCONFIG_MODULES=y\nCONFIG_RUST=y\nCONFIG_UUID_KUNIT_TEST=m\nCONFIG_RUST_UUID_KUNIT_TEST=y\n", True)]:
            (self.work / ".config").write_text(config)
            run([native / "scripts/kconfig/conf", "--olddefconfig", kconfig], cwd=config_work, env=env)
            self.assertEqual("CONFIG_RUST_UUID_KUNIT_TEST=y" in (self.work / ".config").read_text(), enabled)
            if "CONFIG_KUNIT=m" in config:
                self.assertIn("CONFIG_UUID_KUNIT_TEST=m", (self.work / ".config").read_text())
        source = self.work / "source"
        for name, origin in [("lib/tests/uuid_kunit.rs", SUITE), ("lib/tests/Makefile", MAKEFILE),
                             ("lib/tests/uuid_kunit.c", ROOT / "lib/tests/uuid_kunit.c"),
                             ("include/linux/uuid_header.rs", ROOT / "include/linux/uuid_header.rs")]:
            path = source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(origin, path)
        (self.work / "lib/tests").mkdir(parents=True)
        (self.work / "scripts/basic").mkdir(parents=True)
        shutil.copy2(native / "scripts/basic/fixdep", self.work / "scripts/basic/fixdep")
        (self.work / "tools/objtool").mkdir(parents=True)
        shutil.copy2(native / "tools/objtool/objtool", self.work / "tools/objtool/objtool")
        for name in ("include", "arch"):
            (self.work / name).symlink_to(native / name)
        compiler, flags = listing.native_flags(native)
        ccompiler, cflags = native_c_flags(native)
        wrapper = self.work / "rules.mk"
        wrapper.write_text("include "+str(ROOT / "scripts/Makefile.build")+"\n"+
            "rust_common_cmd = "+shlex.join([compiler, *flags])+
            " --out-dir $(dir $@) --emit=dep-info=$(depfile) $(if $(filter m,$(CONFIG_UUID_KUNIT_TEST)),--cfg=MODULE)\n"+
            "c_flags = "+shlex.join(cflags)+" -Wp,-MMD,$(depfile) $(if $(filter m,$(CONFIG_UUID_KUNIT_TEST)),-DMODULE)\n")
        base = ["make", "--no-print-directory", "-f", wrapper, "lib/tests/uuid_kunit.o",
                "obj=lib/tests", "srctree="+str(ROOT), "srcroot="+str(source),
                "objtree="+str(self.work), "VPATH="+str(source), "CC="+ccompiler,
                "CONFIG_UUID_KUNIT_TEST=y"]
        obj = self.work / "lib/tests/uuid_kunit.o"
        previous = None
        for selected in ("n", "y", "n", "y"):
            command = [*base, "CONFIG_RUST_UUID_KUNIT_TEST="+selected]
            run(command, cwd=self.work, env=self.env)
            saved = (self.work / "lib/tests/.uuid_kunit.o.cmd").read_text()
            self.assertIn("uuid_kunit."+("rs" if selected=="y" else "c"), saved.splitlines()[0])
            if previous is not None:
                self.assertGreater(obj.stat().st_mtime_ns, previous)
            previous = obj.stat().st_mtime_ns
            run(command, cwd=self.work, env=self.env)
            self.assertEqual(obj.stat().st_mtime_ns, previous)
        for name in ("lib/tests/uuid_kunit.rs", "include/linux/uuid_header.rs"):
            path = source / name
            path.write_text(path.read_text()+"\n")
            run(command, cwd=self.work, env=self.env)
            self.assertGreater(obj.stat().st_mtime_ns, previous)
            previous = obj.stat().st_mtime_ns
            run(command, cwd=self.work, env=self.env)
            self.assertEqual(obj.stat().st_mtime_ns, previous)
        for suffix in ("s", "ll"):
            args = [str(x).replace("lib/tests/uuid_kunit.o", "lib/tests/uuid_kunit."+suffix) for x in command]
            run(args, cwd=self.work, env=self.env)
            self.assertTrue((self.work / ("lib/tests/uuid_kunit."+suffix)).stat().st_size)
        # Modular selection keeps the original identity/owner marker.
        args = [str(x).replace("CONFIG_UUID_KUNIT_TEST=y", "CONFIG_UUID_KUNIT_TEST=m") for x in command]
        run(args, cwd=self.work, env=self.env)
        self.assertIn(b"__IS_RUST_MODULE", run(["nm", obj]).stdout)
        self.assertIn(b"license=Dual BSD/GPL", module_info(obj))


if __name__ == "__main__":
    unittest.main()
