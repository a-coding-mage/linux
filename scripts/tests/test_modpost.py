#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Whole-program modpost parity, including real and cross-target ELF objects."""

import contextlib
import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest

from modpost_test_support import Elf, ROOT, build, modpost_tools


class ModpostTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = modpost_tools()

    @contextlib.contextmanager
    def fixture(self, files):
        with tempfile.TemporaryDirectory(prefix="modpost-parity-") as work:
            paths = [Path(work) / name for name in ("c", "rust")]
            for path in paths:
                path.mkdir()
                for name, data in files.items():
                    file = path / name
                    file.parent.mkdir(parents=True, exist_ok=True)
                    if isinstance(data, str):
                        data = data.encode()
                    file.write_bytes(data)
            yield paths

    def compare(self, files, args, bits=64, expected_status=None, environment=None):
        with self.fixture(files) as paths:
            results = []
            for executable, path in zip(self.tools[bits][:2], paths):
                result = subprocess.run([str(executable), *args], cwd=path,
                                        capture_output=True, env=dict(os.environ, **(environment or {})))
                result.stderr = result.stderr.replace(str(executable).encode(), b"modpost")
                results.append(result)
            self.assertEqual(results[0].returncode, results[1].returncode, results[1].stderr.decode(errors="replace"))
            self.assertEqual(results[0].stdout, results[1].stdout)
            self.assertEqual(results[0].stderr, results[1].stderr)
            trees = [{str(file.relative_to(path)): file.read_bytes()
                      for file in path.rglob("*") if file.is_file()} for path in paths]
            self.assertEqual(trees[0].keys(), trees[1].keys())
            for name in trees[0]:
                self.assertEqual(trees[0][name], trees[1][name], name)
            if expected_status is not None:
                self.assertEqual(results[0].returncode, expected_status, results[0].stderr.decode(errors="replace"))
            return results[0], trees[0]

    def test_basic_module_headers_and_cli(self):
        for bits in (32, 64):
            for endian in ("little", "big"):
                with self.subTest(bits=bits, endian=endian):
                    elf = Elf(bits, endian)
                    elf.module_info()
                    text = elf.section(".text", bytes(16), flags=6)
                    elf.symbol("init_module", text, kind=2)
                    elf.symbol("cleanup_module", text, value=8, kind=2)
                    self.compare({"fixture.o": elf.build()}, ["-MWbx", "fixture.o", "-o", "Module.symvers"], bits, 0)
        for filename, args in (("drivers/staging/fixture.o", []), ("tools/testing/fixture.o", ["-e"]), ("fixture.o", ["-e"])):
            elf = Elf()
            elf.module_info()
            self.compare({filename: elf.build()}, args + [filename], expected_status=0)

    def test_licenses_description_and_import_errors(self):
        for license in (None, "GPL", "GPL v2", "GPL and additional rights", "Dual BSD/GPL", "Dual MIT/GPL", "Dual MPL/GPL", "Proprietary", ["GPL", "Proprietary"]):
            elf = Elf()
            elf.module_info(license=license, description=None, imports=("module:fixture", "SCOPE", "SCOPE"))
            elf.symbol("imported")
            self.compare({"fixture.o": elf.build(), "input.symvers": "0x12345678\timported\tvmlinux\tEXPORT_SYMBOL_GPL\tSCOPE\n"},
                         ["-i", "input.symvers", "fixture.o"])

    def test_exports_and_generated_function_data_records(self):
        for bits in (32, 64):
            for endian in ("little", "big"):
                elf = Elf(bits, endian)
                elf.module_info()
                elf.export("ordinary")
                elf.export("restricted", "NAMESPACE", gpl=True, kind=1)
                elf.export("weak", binding=2)
                self.compare({"fixture.o": elf.build(extended=True)}, ["-M", "fixture.o", "-o", "Module.symvers"], bits, 0)
        elf = Elf(machine=15)
        elf.module_info()
        elf.export("parisc_function", kind=13)
        self.compare({"fixture.o": elf.build()}, ["-M", "fixture.o"], expected_status=0)

    def test_export_validation(self):
        for changes in (dict(label="unexpected"), dict(binding=0), dict(label="__export_symbol_other"), dict(license="MIT")):
            elf = Elf()
            elf.module_info()
            elf.export("exported", **changes)
            self.compare({"fixture.o": elf.build()}, ["-M", "fixture.o"], expected_status=1)
        for section_name in (".init.text", ".exit.data"):
            elf = Elf()
            elf.module_info()
            section = elf.section(section_name, bytes(16), flags=6)
            elf.export("special", section=section)
            self.compare({"fixture.o": elf.build()}, ["-M", "fixture.o"], expected_status=0)

    def test_duplicate_exports_and_external_modules(self):
        elf = Elf()
        elf.module_info()
        elf.export("duplicate")
        data = elf.build()
        for owner in ("vmlinux", "other"):
            files = {"fixture.o": data, "reference.symvers": f"0x12345678\tduplicate\t{owner}\tEXPORT_SYMBOL\t\n"}
            for options in ([], ["-e"]):
                self.compare(files, ["-M", "-i", "reference.symvers", *options, "fixture.o"])
        self.compare({"one.o": data, "two.o": data}, ["-M", "one.o", "two.o"])

    def test_symbol_dump_parsing(self):
        for line in ("", "\n", "bad\n", "0\tname\tmodule\tEXPORT_SYMBOL\t\n",
                     "\tname\tmodule\tEXPORT_SYMBOL\t\n", "  -0x1\tname\tmodule\tEXPORT_SYMBOL_GPL\tNS\n",
                     "0xyz\tname\tmodule\tEXPORT_SYMBOL\t\n", "0\t\tmodule\tEXPORT_SYMBOL\t\n",
                     "0\tname\t\tEXPORT_SYMBOL\t\n", "0\tname\tmodule\tUNKNOWN\t\n",
                     "0xffffffffffffffffffffffff\tname\tmodule\tEXPORT_SYMBOL\tNS\textra\n"):
            with self.subTest(line=line):
                self.compare({"dump": line}, ["-i", "dump", "-o", "out"])

    def test_unresolved_limit_weak_common_and_architecture_exemptions(self):
        prefixes = {20: ["_restgpr_", "_savegpr_", "_rest32gpr_", "_save32gpr_", "_restvr_", "_savevr_"],
                    21: ["_restgpr0_", "_savegpr0_", "_restgpr1_", "_savegpr1_", "_restfpr_", "_savefpr_", "_restvr_", "_savevr_"]}
        for machine in (3, 20, 21, 2, 43):
            elf = Elf(machine=machine)
            elf.module_info()
            for index in range(15):
                elf.symbol("missing" + str(index))
            for name in ("__this_module", "_GLOBAL_OFFSET_TABLE_", "__start_test", "__stop_test"):
                elf.symbol(name)
            for prefix in prefixes.get(machine, []):
                elf.symbol(prefix + "31")
            elf.symbol(".TOC.")
            elf.symbol(".hello")
            elf.symbol("weak_missing", binding=2)
            elf.symbol("local_missing", binding=0)
            elf.symbol("register", kind=13)
            elf.symbol("common", section=0xfff2)
            elf.symbol("__gnu_lto_skip", section=0xfff2)
            self.compare({"fixture.o": elf.build()}, ["-w", "fixture.o"], expected_status=0)

    def test_namespaces_dependency_order_and_dot_names(self):
        dump = "".join(f"0x{index:08x}\t{name}\t{owner}\tEXPORT_SYMBOL\t{namespace}\n" for index, (name, owner, namespace) in enumerate([
            ("first", "drivers/one", "module:fix*,allowed"), ("second", "drivers/two", "NEEDED"),
            ("third", "drivers/one", "NEEDED"), ("builtin", "vmlinux", ""), ("dot", "three", "IMPORTED")]))
        for allow in (False, True):
            elf = Elf()
            elf.module_info(imports=("IMPORTED",))
            for name in ("first", "second", "third", "builtin", ".dot"):
                elf.symbol(name)
            self.compare({"fixture.o": elf.build(), "dump": dump},
                         ["-i", "dump", "-d", "namespaces", "fixture.o"] + (["-N"] if allow else []))

    def test_trim_whitelist_and_no_trim(self):
        provider = Elf()
        provider.module_info()
        for name in ("used", "unused", "keep", "white"):
            provider.export(name)
        consumer = Elf()
        consumer.module_info()
        consumer.symbol("used")
        consumer.section(".no_trim_symbol", b"keep\0\0absent\0")
        files = {"provider.o": provider.build(), "consumer.o": consumer.build(), "white": "white\nunknown\n"}
        self.compare(files, ["-Mt", "provider.o", "consumer.o", "-u", "white", "-o", "Module.symvers"], expected_status=0)

    def test_modversions_and_crc_input(self):
        provider = Elf()
        provider.module_info()
        for name in ("hex", "decimal", "octal", "ignored", "thin"):
            provider.export(name)
        consumer = Elf()
        consumer.module_info()
        for name in ("hex", "decimal", "octal", "ignored", "thin"):
            consumer.symbol(name)
        files = {"provider.o": provider.build(), "consumer.o": consumer.build(),
                 "provider.mod": "first.o\nsecond.thinlto-native.o\n", "consumer.mod": "consumer.o\n",
                 ".first.o.cmd": "#SYMVER ignored 0xffff\n#SYMVER hex 0x12345678\n#SYMVER decimal 1234\n#SYMVER octal 01234\n#SYMVER ignored nope\n#SYMVER orphan 0x1\n",
                 ".second.o.cmd": "\n#SYMVER thin 0xabcdef01\n", ".consumer.o.cmd": "empty\n",
                 "dump": "0x10203040\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n"}
        for version_flags in ("b", "x", "bx", ""):
            self.compare(files, ["-Mm" + version_flags, "-i", "dump", "provider.o", "consumer.o", "-o", "Module.symvers"], expected_status=0)

    def test_long_module_and_versioned_symbol_names(self):
        for bits in (32, 64):
            for length in (55, 56, 59, 60):
                name = "m" * length
                elf = Elf(bits)
                elf.module_info()
                elf.symbol("s" * 70)
                files = {name + ".o": elf.build(), "dump": "0x1\t" + "s" * 70 + "\tvmlinux\tEXPORT_SYMBOL\t\n"}
                for flags in ("b", "bx", "x"):
                    self.compare(files, ["-" + flags, "-i", "dump", name + ".o"], bits)

    def test_cli_missing_files_and_non_elf_inputs(self):
        for args in ([], ["-z"], ["--help"], ["-i"], ["-Mni"], ["-T", "missing"], ["-i", "missing"],
                     ["missing.o"], ["-n", "missing.o"], ["fixture.txt"], ["-u", "missing"],
                     ["-o", "missing/dump"], ["-n", "empty.o"], ["empty.o"], ["small.o"], ["random.o"]):
            self.compare({"empty.o": b"", "small.o": b"not an ELF", "random.o": b"A" * 70}, args)
        elf = Elf()
        elf.module_info()
        files = {"fixture.o": elf.build(), "objects": "fixture.o\n"}
        self.compare(files, ["-Tobjects", "-odump"])
        self.compare(files, ["fixture.o", "-e", "-o", "dump"], environment={"POSIXLY_CORRECT": "1"})

    def test_section_mismatches_and_whitelists(self):
        cases = [(".text", "caller", ".init.text", "target"), (".data", "value", ".exit.data", "target"),
                 (".init.text", "caller", ".exit.text", "target"), (".exit.text", "caller", ".init.text", "target"),
                 (".pci_fixup_early", "caller", ".init.data", "target"),
                 (".data", "__param_value", ".init.data", "target"), (".data", "__param_ops_value", ".init.text", "target"),
                 (".data", "driver_ops", ".exit.data", "target"), (".data.rel", "driver_console", ".init.text", "target"),
                 (".head.text", "entry", ".init.text", "target"), (".text", "foo.constprop.2", ".init.data", "target"),
                 (".text", "caller", ".init.text", "__init_begin"), (".debug_info", "debug", ".init.text", "target")]
        for source, caller, destination, target in cases:
            elf = Elf()
            elf.module_info()
            src = elf.section(source, bytes(16), flags=6)
            dst = elf.section(destination, bytes(16), flags=6)
            elf.symbol(caller, src, kind=2)
            sym = elf.symbol(target, dst, kind=2)
            elf.relocation(src, sym, offset=4)
            self.compare({"fixture.o": elf.build()}, ["-E", "fixture.o"])

    def test_nearest_symbols_ties_mapping_and_thumb(self):
        for machine in (62, 40):
            for address in (0, 3, 10, 20, 40, 61):
                elf = Elf(machine=machine)
                elf.module_info()
                src = elf.section(".text", bytes(80), flags=6)
                dst = elf.section(".init.text", bytes(80), flags=6)
                for name, offset in (("$mapping", 5), (".Llocal", 6), ("L0local", 7), ("caller", 0), ("duplicate", 0)):
                    elf.symbol(name, src, value=offset, kind=2)
                for name, offset in (("low", 1), ("middle", 21), ("high", 41)):
                    elf.symbol(name, dst, value=offset, kind=2)
                section_symbol = elf.symbol("", dst, kind=3)
                elf.relocation(src, section_symbol, offset=8, addend=address)
                self.compare({"fixture.o": elf.build()}, ["fixture.o"])

    def test_exception_table_diagnostics_and_nonalloc_sections(self):
        for destination, flags in ((".altinstr_replacement", 6), (".novel.text", 6), (".data", 3), (".text", 6)):
            elf = Elf()
            elf.module_info()
            src = elf.section("__ex_table", bytes(16))
            dst = elf.section(destination, bytes(16), flags=flags)
            target = elf.symbol("target", dst, kind=2)
            elf.relocation(src, target)
            elf.section(".custom", bytes(1), flags=0)
            elf.section(".comment", b"compiler", flags=0)
            self.compare({"fixture.o": elf.build()}, ["fixture.o"])

    def test_rel_addends_all_supported_architectures(self):
        cases = {3: [1, 2, 9], 8: [2, 4, 6, 9], 40: [1, 2, 3, 10, 28, 29, 30, 43, 44, 47, 48, 51, 9]}
        for endian in ("little", "big"):
            for machine, kinds in cases.items():
                for kind in kinds:
                    elf = Elf(32, endian, machine)
                    elf.module_info()
                    src = elf.section(".text", (4).to_bytes(4, endian) + bytes(12), flags=6)
                    dst = elf.section(".init.text", bytes(80), flags=6)
                    elf.symbol("caller", src, kind=2)
                    elf.symbol("target", dst, value=8, kind=2)
                    target = elf.symbol("", dst, kind=3)
                    elf.relocation(src, target, kind=kind, addend=None)
                    self.compare({"fixture.o": elf.build()}, ["fixture.o"], 32)

    def test_rela_mips64_riscv_and_loongarch(self):
        for machine in (8, 243, 258):
            for endian in ("little", "big"):
                elf = Elf(64, endian, machine)
                elf.module_info()
                src = elf.section("__ex_table", bytes(40))
                dst = elf.section(".data", bytes(40))
                target = elf.symbol("target", dst)
                for offset, kind in enumerate((1, 39, 55, 100, 102)):
                    elf.relocation(src, target, offset=offset * 4, kind=kind)
                self.compare({"fixture.o": elf.build()}, ["fixture.o"])

    def test_builtin_aliases_and_module_table_bounds(self):
        for bits in (32, 64):
            layout = self.tools[bits][2]
            for builtin in (False, True):
                elf = Elf(bits)
                elf.module_info()
                size = layout["SIZE_platform_device_id"]
                data = b"platform-name\0" + bytes(size * 2 - len("platform-name") - 1)
                section = elf.section(".rodata", data)
                elf.symbol("__mod_device_table__kmod_builtin__platform__ids", section, size=len(data), kind=1)
                elf.export("exported")
                name = "vmlinux.o" if builtin else "fixture.o"
                self.compare({name: elf.build()}, ["-M", name, "-o", "Module.symvers"], bits, 0)

    def test_genuine_compiler_and_assembler_objects(self):
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        assembly = r'''
.section .modinfo,"a"
.asciz "license=GPL"
.asciz "description=compiled object"
.section .text,"ax"
.globl init_module
.type init_module,@function
init_module:
 .long 0
.section .init.data,"aw"
target:
 .long 1
.section .data,"aw"
.globl exported
.type exported,@object
exported:
 .quad target
.section .export_symbol,"a"
__export_symbol_exported:
 .asciz "GPL"
 .asciz "NAMESPACE"
 .balign 8
 .quad exported
'''
        with tempfile.TemporaryDirectory(prefix="modpost-real-") as work:
            object_file = Path(work) / "fixture.o"
            build(cc + ["-m64", "-c", "-x", "assembler", "-", "-o", str(object_file)], input=assembly.encode())
            self.compare({"fixture.o": object_file.read_bytes()}, ["-M", "fixture.o", "-o", "Module.symvers"])
            source = b'''static const char license[] __attribute__((section(".modinfo"),used))="license=GPL";
static const char description[] __attribute__((section(".modinfo"),used))="description=compiled C";
extern int imported(void);
int init_module(void) { return imported(); }
void cleanup_module(void) { }
'''
            build(cc + ["-c", "-O2", "-x", "c", "-", "-o", str(object_file)], input=source)
            self.compare({"fixture.o": object_file.read_bytes(), "dump": "0x123\timported\tvmlinux\tEXPORT_SYMBOL\t\n"},
                         ["-Mbx", "-i", "dump", "fixture.o"])

    def test_write_if_changed_and_error_suppression(self):
        elf = Elf()
        elf.module_info()
        with self.fixture({"fixture.o": elf.build()}) as paths:
            for executable, path in zip(self.tools[64][:2], paths):
                command = [str(executable), "fixture.o"]
                subprocess.run(command, cwd=path, check=True, capture_output=True)
                output = path / "fixture.mod.c"
                os.utime(output, ns=(1000000000, 1000000000))
                subprocess.run(command, cwd=path, check=True, capture_output=True)
                self.assertEqual(output.stat().st_mtime_ns, 1000000000)
                result = subprocess.run(command + ["missing.bad"], cwd=path, capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertEqual(output.stat().st_mtime_ns, 1000000000)

    def test_randomized_symbol_reference_graphs(self):
        randomizer = random.Random(0x4D4F4450)
        for trial in range(36):
            bits = (32, 64)[trial % 2]
            endian = ("little", "big")[(trial // 2) % 2]
            files = {}
            names = [f"export_{index}" for index in range(15)]
            owners = [randomizer.randrange(3) for _ in names]
            namespaces = [randomizer.choice(("", "NS_A", "NS_B", "module:mod*")) for _ in names]
            for index in range(3):
                elf = Elf(bits, endian)
                elf.module_info(license=randomizer.choice(("GPL", "Dual MIT/GPL", "Proprietary")),
                                imports=randomizer.choice(((), ("NS_A",), ("NS_A", "NS_B"))))
                for symbol, name in enumerate(names):
                    if owners[symbol] == index:
                        elf.export(name, namespaces[symbol], gpl=bool(randomizer.randrange(2)), kind=randomizer.choice((1, 2)))
                    elif randomizer.randrange(3) == 0:
                        elf.symbol(name, binding=randomizer.choice((1, 2)))
                files[f"mod{index}.o"] = elf.build(extended=bool(trial % 3))
            self.compare(files, ["-MNtw", "-d", "namespaces", "-o", "dump", "mod0.o", "mod1.o", "mod2.o"], bits)

    def test_target_elf_malformed_input_is_checked(self):
        elf = Elf()
        elf.module_info()
        elf.symbol("symbol")
        original = elf.build()
        variants = []
        for offset, value in ((5, 0), (16, 2)):
            data = bytearray(original)
            data[offset] = value
            variants.append(bytes(data))
        data = bytearray(original)
        struct.pack_into("<Q", data, 40, len(data) + 100)
        variants.append(bytes(data))
        for data in variants:
            self.compare({"fixture.o": data}, ["fixture.o"], expected_status=1)
        # C assumes the remainder is trustworthy; Rust rejects these instead of
        # reading past mappings, dividing by zero, or allocating attacker sizes.
        corruptions = [original[:64], original[:100]]
        section_offset = struct.unpack_from("<Q", original, 40)[0]
        for field, size, value in ((section_offset + 64 + 24, "Q", len(original) - 1),
                                   (section_offset + 64 * 2 + 56, "Q", 0),
                                   (section_offset + 64 * 2 + 40, "I", 0xffffffff)):
            data = bytearray(original)
            struct.pack_into("<" + size, data, field, value)
            corruptions.append(bytes(data))
        with self.fixture({}) as paths:
            for data in corruptions:
                (paths[1] / "fixture.o").write_bytes(data)
                result = subprocess.run([str(self.tools[64][1]), "fixture.o"], cwd=paths[1], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertNotIn(b"panicked", result.stderr)

    def test_standalone_binary_requires_or_loads_target_metadata(self):
        elf = Elf()
        elf.module_info()
        size = self.tools[64][2]["SIZE_platform_device_id"]
        data = b"standalone\0" + bytes(size * 2 - 11)
        section = elf.section(".rodata", data)
        elf.symbol("__mod_device_table__kmod_fixture__platform__ids", section, size=len(data), kind=1)
        with self.fixture({"fixture.o": elf.build()}) as paths:
            standalone = paths[1] / "modpost"
            environment = os.environ.copy()
            environment.pop("MODPOST_DEVICETABLE_OFFSETS", None)
            build(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + ["--edition=2021", "-Dwarnings", "-Wmissing-docs",
                  str(ROOT / "scripts/mod/modpost.rs"), "-o", str(standalone)], env=environment)
            result = subprocess.run([str(standalone), "fixture.o"], cwd=paths[1], capture_output=True, env=environment)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"device-table layout metadata is unavailable", result.stderr)
            self.assertFalse((paths[1] / "fixture.mod.c").exists())
            environment["MODPOST_DEVICETABLE_OFFSETS"] = str(self.tools[64][0].parent / "devicetable-offsets.h")
            expected = subprocess.run([str(self.tools[64][0]), "fixture.o"], cwd=paths[0], capture_output=True)
            actual = subprocess.run([str(standalone), "fixture.o"], cwd=paths[1], capture_output=True, env=environment)
            self.assertEqual((actual.returncode, actual.stdout, actual.stderr),
                             (expected.returncode, expected.stdout, expected.stderr))
            self.assertEqual((paths[0] / "fixture.mod.c").read_bytes(), (paths[1] / "fixture.mod.c").read_bytes())
            result = subprocess.run([os.fsencode(standalone), b"\xff.o"], cwd=paths[1], capture_output=True)
            self.assertEqual(result.returncode, 1)
            self.assertNotIn(b"panicked", result.stderr)

    @unittest.skipUnless(os.environ.get("MODPOST_KERNEL_BUILD"), "set MODPOST_KERNEL_BUILD for existing kernel objects")
    def test_existing_kernel_build_objects(self):
        directory = Path(os.environ["MODPOST_KERNEL_BUILD"])
        files = {"vmlinux.o": (directory / "vmlinux.o").read_bytes()}
        order = directory / "modules.order"
        modules = order.read_text().splitlines() if order.exists() else []
        for module in modules:
            object_name = module.removesuffix(".ko") + ".o" if module.endswith(".ko") else module
            files[object_name] = (directory / object_name).read_bytes()
        objects = list(files)
        bits = 64 if files["vmlinux.o"][4] == 2 else 32
        self.compare(files, ["-M", "-w", "-o", "Module.symvers", *objects], bits, expected_status=0)
        if "CONFIG_MODVERSIONS=y\n" in (directory / ".config").read_text():
            lists = [".vmlinux.objs"] + [name.removesuffix(".o") + ".mod" for name in objects[1:]]
            for name in lists:
                files[name] = (directory / name).read_bytes()
                for object_name in files[name].decode().splitlines():
                    object_path = Path(object_name.replace(".thinlto-native.o", ".o"))
                    command = str(object_path.with_name("." + object_path.name + ".cmd"))
                    files[command] = (directory / command).read_bytes()
            self.compare(files, ["-Mmbx", "-w", "-o", "Module.symvers", *objects], bits, expected_status=0)


if __name__ == "__main__":
    unittest.main()
