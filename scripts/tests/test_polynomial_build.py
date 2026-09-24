# SPDX-License-Identifier: GPL-2.0-only
"""Actual polynomial FAM ABI, native versioning and selectable Kbuild tests.

All C adapters and compiler outputs are private fixtures. Original production
C stays untouched. Explicit tool/sysroot requests fail rather than skip.
"""

import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

from kconfig_test_support import cached_conf_tools
from modpost_test_support import modpost_tools
from rust_exports_test_support import dwarf_tools, dwarf_versions, read_exports, rust_targets
from test_ctype_translation import elf_symbol
from test_int_math_translation import C_TYPES
from test_rational_build import environment, headers as module_headers, module_info, run
import test_polynomial_translation as pure


ROOT = Path(__file__).resolve().parents[2]
OWNER = ROOT / "lib/math/polynomial_rust.rs"
SYMBOL = b"polynomial_calc"


def headers(work):
    flags = module_headers(work)
    include = work / "include"
    (include / "linux/types.h").write_text(C_TYPES)
    for path in ("asm", "uapi/linux"):
        (include / path).mkdir(parents=True, exist_ok=True)
    (include / "asm/div64.h").write_text("/* polynomial.c does not expand do_div. */\n")
    (include / "uapi/linux/kernel.h").write_text("/* No rounding macros expanded. */\n")
    return flags


def expected_info(module):
    values = [b"description=Generic polynomial calculations", b"license=GPL"]
    return sorted(values if module else [b"polynomial." + value for value in values] +
                  [b"polynomial.file=lib/math/polynomial"])


def abi_records(bits):
    records = [item for item, _, _ in pure.original_vectors()]
    records += [pure.record(value, total, terms) for _, total, terms in pure.driver_polynomials()
                for value in range(1024)]
    records += [item for item in pure.edge_records(bits) + pure.random_records(bits) if item[0] == 0]
    # This transport mode uses an exactly sized stack FAM and deliberately
    # leaves the constant term's unused divider uninitialized, as C permits.
    records += [pure.record(value, total, [(0, value, 0, 1)], mode=3)
                for value in (-(1 << (bits - 1)), -1, 0, 1, (1 << (bits - 1)) - 1)
                for total in (0, 1, -1, 3)]
    return [item for item in records if pure.checked_model(item, bits) is not None]


# The unchanged C header owns the descriptor and FAM layout in this caller.
# Reuse only the fixed-width syscall transport and original descriptor loader.
ABI_ADAPTER = pure.C_ADAPTER.replace("void c_eval", r'''
#ifdef TEST_KCFI
static long (*volatile actual)(const struct polynomial *, long) = polynomial_calc;
#define polynomial_calc(poly, data) actual(poly, data)
#endif
void c_eval''').replace("out[0] = 1;", r'''
if (mode == 3) {
    struct polynomial *constant = __builtin_alloca(sizeof(*constant) + sizeof(struct polynomial_term));
    constant->total_divider = (long)input[3];
    constant->terms[0].deg = 0;
    constant->terms[0].coef = (long)input[5];
    constant->terms[0].divider_leftover = (long)input[7];
    out[0] = 1;
    out[1] = (u64)(s64)polynomial_calc(constant, data);
    return;
}
out[0] = 1;''')


class TemporaryTest(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory(prefix="polynomial-build-")
        self.addCleanup(directory.cleanup)
        self.work = Path(directory.name)
        self.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.flags = headers(self.work)

    def c_flags(self, bits=64, module=False):
        return [*self.flags, "-m" + str(bits), "-D__KERNEL__", "-DCONFIG_GENDWARFKSYMS",
                '-DKBUILD_MODNAME="polynomial"', '-DKBUILD_MODFILE="lib/math/polynomial"',
                "-fno-strict-overflow", *(["-DCONFIG_64BIT"] if bits == 64 else []),
                *(["-DMODULE"] if module else [])]

    def binding_flags(self, bits=64):
        directory = self.work / ("bindings-" + str(bits))
        library = directory / "libkernel.rlib"
        if not library.exists():
            bindgen = shlex.split(os.environ.get("BINDGEN", "bindgen"))
            if not shutil.which(bindgen[0]):
                if "BINDGEN" in os.environ:
                    self.fail("explicit BINDGEN unavailable: " + bindgen[0])
                self.skipTest("actual polynomial header bindings require BINDGEN")
            directory.mkdir()
            generated = directory / "bindings.rs"
            run([*bindgen, ROOT / "include/linux/polynomial.h", "--use-core", "--rust-target=1.85",
                 "--ctypes-prefix=ffi",
                 "--allowlist-type=polynomial.*", "--allowlist-function=polynomial_calc",
                 "--no-layout-tests", "--no-doc-comments", "-o", generated, "--", *self.c_flags(bits)])
            ffi = directory / "libffi.rlib"
            run([*self.rustc, *rust_targets()[bits], "--edition=2021", "--crate-name=ffi", "--crate-type=rlib",
                 "-Cpanic=abort", "-Dwarnings", ROOT / "rust/ffi.rs", "-o", ffi], env=environment())
            source = directory / "kernel.rs"
            source.write_text('#![no_std]\n#![allow(non_camel_case_types)]\npub extern crate ffi;\n'
                              'pub mod bindings { include!("bindings.rs"); }\n')
            run([*self.rustc, *rust_targets()[bits], "--edition=2021", "--crate-name=kernel", "--crate-type=rlib",
                 "-Cpanic=abort", "-Dwarnings", "--extern", "ffi=" + str(ffi), source, "-o", library],
                env=environment())
        return ["--extern", "kernel=" + str(library), "-Ldependency=" + str(directory)]

    def c_object(self, bits=64, module=False, optimize="2", dwarf=5, compiler=None):
        output = self.work / "original.o"
        run([*(compiler or self.cc), *self.c_flags(bits, module), "-O" + optimize, "-g", "-gdwarf-" + str(dwarf),
             "-c", ROOT / "lib/math/polynomial.c", "-o", output])
        return output

    def rust_object(self, bits=64, module=False, optimize="2", dwarf=5, library=False, extra=()):
        if bits not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        source = self.work / "owner.rs"
        source.write_text('//! Actual native polynomial owner and original layout.\n#![no_std]\n'
                          '#[path="' + str(OWNER) + '"] mod production;\npub use production::*;\n' + r'''
/// Layout from the actual original header's generated types.
#[no_mangle]
pub static polynomial_layout: [u32; 9] = [
    core::mem::size_of::<kernel::bindings::polynomial_term>() as u32,
    core::mem::align_of::<kernel::bindings::polynomial_term>() as u32,
    core::mem::offset_of!(kernel::bindings::polynomial_term, deg) as u32,
    core::mem::offset_of!(kernel::bindings::polynomial_term, coef) as u32,
    core::mem::offset_of!(kernel::bindings::polynomial_term, divider) as u32,
    core::mem::offset_of!(kernel::bindings::polynomial_term, divider_leftover) as u32,
    core::mem::size_of::<kernel::bindings::polynomial>() as u32,
    core::mem::offset_of!(kernel::bindings::polynomial, terms) as u32,
    core::mem::size_of::<kernel::ffi::c_long>() as u32,
];
''' + (r'''
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        core::arch::asm!("mov ebx, 97", "int 0x80", in("eax") 1, options(noreturn));
        #[cfg(target_pointer_width = "64")]
        core::arch::asm!("syscall", in("rax") 60, in("rdi") 97, options(noreturn));
    }
}
/// Prove genuine core panic paths fail the test process.
#[no_mangle]
pub extern "C" fn trigger_fixture_panic() { panic!("polynomial ABI negative control"); }
''' if library else ""))
        output = self.work / ("native.a" if library else "native.o")
        run([*self.rustc, *rust_targets()[bits], *self.binding_flags(bits), "--edition=2021", "--crate-name=polynomial_owner",
             "--crate-type=" + ("staticlib" if library else "rlib"), "-Cpanic=abort", "-Crelocation-model=static",
             "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
             "-Coverflow-checks=yes", "-Cdebug-assertions=yes", "-Copt-level=" + optimize,
             "-Cdebuginfo=2", "-Zdwarf-version=" + str(dwarf), "-Zbinary_dep_depinfo=y",
             *(["--cfg=MODULE"] if module else []), "--emit=" + ("link" if library else "obj"),
             "--emit=dep-info=" + str(self.work / "native.d"), *extra, source, "-o", output],
            env={**environment(), "RUSTC_BOOTSTRAP": "1", "RUST_MODFILE": "lib/math/polynomial"})
        return output


    def driver(self, bits, obj, name, panic=False, compiler=None, extra=()):
        source, adapter = self.work / "caller.c", self.work / "adapter.c"
        source.write_text(pure.DRIVER)
        adapter.write_text(ABI_ADAPTER)
        linker = self.work / "no-unwind.lds"
        linker.write_text("SECTIONS { /DISCARD/ : { *(.eh_frame .eh_frame.* .eh_frame_hdr) "
                          "*(.gcc_except_table .gcc_except_table.*) *(.data.DW.ref.rust_eh_personality) "
                          "} } INSERT AFTER .text;\n")
        output = self.work / name
        run([*(compiler or self.cc), *self.c_flags(bits), "-O2", "-DTEST_BITS=" + str(bits),
             *(["-DPANIC_CONTROL"] if panic else []), "-ffreestanding", "-fno-builtin", "-fno-stack-protector",
             "-fno-pic", "-fno-pie", "-nostdlib", "-static", "-no-pie", "-Wl,-e,_start",
             "-Wl,--gc-sections", "-Wl,-T," + str(linker), source, adapter, *extra, obj, "-o", output])
        self.assertEqual(output.read_bytes()[:6], b"\x7fELF" + bytes([1 if bits == 32 else 2, 1]))
        return output


class PolynomialSelectionTests(TemporaryTest):
    def test_actual_kconfig_preserves_tristate_and_defaults_rust_off(self):
        source = (ROOT / "lib/Kconfig").read_text()
        choice = re.search(r"(?ms)^config RUST_POLYNOMIAL\n.*?(?=^config |\Z)", source)
        original = re.search(r"(?ms)^config POLYNOMIAL\n.*?(?=^config |\Z)",
                             (ROOT / "lib/math/Kconfig").read_text())
        self.assertIsNotNone(choice)
        self.assertIsNotNone(original)
        self.assertIn("tristate", original.group())
        config = self.work / "Kconfig"
        config.write_text('config MODULES\n\tbool "modules"\n\tmodules\n'
                          'config RUST\n\tbool "Rust support"\n\n' + choice.group() + "\n" + original.group())
        env = {**environment(), "KCONFIG_CONFIG": str(self.work / ".config")}
        for tool in cached_conf_tools():
            for rust, requested, expected in (("n", "y", False), ("y", None, False),
                                               ("y", "y", True), ("y", "n", False)):
                for selected in ("n", "y", "m"):
                    # POLYNOMIAL has no prompt; its caller's select/default
                    # owns enablement. Give the fixture the real tristate via
                    # an ordinary prompted parent selecting it.
                    text = config.read_text()
                    if "config TEST_POLYNOMIAL" not in text:
                        config.write_text(text + '\nconfig TEST_POLYNOMIAL\n\ttristate "polynomial caller"\n\tselect POLYNOMIAL\n')
                    value = "CONFIG_MODULES=y\nCONFIG_RUST=" + rust + "\nCONFIG_TEST_POLYNOMIAL=" + selected + "\n"
                    if requested is not None:
                        value += "CONFIG_RUST_POLYNOMIAL=" + requested + "\n"
                    (self.work / ".config").write_text(value)
                    run([tool, "--olddefconfig", config], cwd=self.work, env=env)
                    actual = (self.work / ".config").read_text().splitlines()
                    self.assertEqual("CONFIG_RUST_POLYNOMIAL=y" in actual, expected)
                    self.assertEqual("CONFIG_POLYNOMIAL=" + selected in actual, selected != "n")

    def test_actual_makefile_n_y_m_keeps_module_name_and_original_order(self):
        harness = self.work / "Makefile"
        harness.write_text(f'''include {ROOT}/lib/math/Makefile
.PHONY: selection
selection:
	@printf '%s\\n' '$(obj-y)' '$(obj-m)' '$(polynomial-y)'
''')
        for host in ("c", "rust"):
            for native in ("", "y"):
                for state in ("", "y", "m"):
                    result = run(["make", "--no-print-directory", "-rR", "-f", harness, "selection",
                                  "HOST_TOOLS_LANG=" + host, "CONFIG_RUST_POLYNOMIAL=" + native,
                                  "CONFIG_POLYNOMIAL=" + state], cwd=self.work, env=environment())
                    built, modules, parts = result.stdout.decode().splitlines()
                    self.assertEqual(built, "div64.o gcd.o lcm.o int_log.o int_pow.o int_sqrt.o reciprocal_div.o " +
                                     ("polynomial.o " if state == "y" else "") + "tests/")
                    self.assertEqual(modules, "polynomial.o" if state == "m" else "")
                    self.assertEqual(parts, "polynomial_rust.o" if native == "y" else "")




class PolynomialKbuildTests(TemporaryTest):
    @classmethod
    def setUpClass(cls):
        directory = tempfile.TemporaryDirectory(prefix="polynomial-build-tools-")
        cls.addClassCleanup(directory.cleanup)
        cls.tools = Path(directory.name)
        cls.fixdep = cls.tools / "fixdep"
        run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O", "-Dwarnings",
             ROOT / "scripts/basic/fixdep.rs", "-o", cls.fixdep])
        cls.gendwarf = dwarf_tools()[1]

    def setUp(self):
        super().setUp()
        for name in ("lib/math/tests", "scripts/basic", "scripts/gendwarfksyms", "include/config"):
            (self.work / name).mkdir(parents=True, exist_ok=True)
        (self.work / "scripts/basic/fixdep").symlink_to(self.fixdep)
        (self.work / "scripts/gendwarfksyms/gendwarfksyms").symlink_to(self.gendwarf)
        self.ar = shlex.split(os.environ.get("AR", "ar"))
        self.other = ("div64.o", "gcd.o", "lcm.o", "int_log.o", "int_pow.o", "int_sqrt.o", "reciprocal_div.o")
        empty = self.work / "empty.S"
        empty.write_text('.section .note.GNU-stack,"",@progbits\n')
        for name in self.other:
            run([*self.cc, "-c", empty, "-o", self.work / "lib/math" / name])
        run([*self.ar, "cr", self.work / "lib/math/tests/built-in.a"])
        (self.work / "lib/math/tests/modules.order").write_bytes(b"")
        (self.work / "kernel.symvers").write_bytes(b"0x12345678\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n")
        self.command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR", "-j4",
                        "-f", str(ROOT / "scripts/Makefile.build"), "obj=lib/math", "srcroot=" + str(ROOT),
                        "srctree=" + str(ROOT), "objtree=" + str(self.work), "VPATH=" + str(ROOT),
                        "need-builtin=1", "need-modorder=1", "KBUILD_BUILTIN=1", "KBUILD_MODULES=1",
                        "CONFIG_MODULES=y", "CONFIG_MODVERSIONS=y", "CONFIG_GENDWARFKSYMS=y", "KBUILD_SYMTYPES=1",
                        "AR=" + shlex.join(self.ar), "NM=" + os.environ.get("NM", "nm"), "LD=" + os.environ.get("LD", "ld"),
                        "AWK=" + os.environ.get("AWK", "awk"),
                        "rust_common_cmd=RUST_MODFILE=$(modfile) " + shlex.join(self.rustc + self.binding_flags()) +
                        " --crate-type=rlib --edition=2021 -O -g -Cpanic=abort -Dwarnings -Wmissing-docs"
                        " -Wunreachable-pub -Wrust-2018-idioms $(if $(part-of-module),--cfg MODULE)"
                        " -Zbinary_dep_depinfo=y -Zcrate-attr=no_std"
                        " --emit=dep-info=$(depfile)",
                        "cmd_cc_o_c=" + shlex.join(self.cc + self.c_flags()) +
                        " -O2 -g $(if $(part-of-module),-DMODULE) -MMD -MF $(depfile) -c $< -o $@"]
        for name in (*self.other, "tests/built-in.a", "tests/modules.order"):
            self.command += ["-o", "lib/math/" + name]

    def make(self, *targets, native=True, state="m", extra=()):
        if state == "m" and "lib/math/polynomial.o" in targets:
            targets += ("lib/math/polynomial.mod",)
        return run([*self.command, "CONFIG_RUST_POLYNOMIAL=" + ("y" if native else ""), "CONFIG_POLYNOMIAL=" + state,
                    *extra, *targets], cwd=self.work, env={**environment(), "RUSTC_BOOTSTRAP": "1"})

    def records(self, owner):
        data = (self.work / "lib/math" / ("." + owner + ".o.cmd")).read_bytes()
        return dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", data))

    def test_fresh_parallel_composite_module_crc_constituents_and_both_modposts(self):
        self.make("lib/math/polynomial.o", "lib/math/modules.order")
        self.assertEqual((self.work / "lib/math/polynomial.mod").read_bytes(), b"lib/math/polynomial_rust.o\n")
        self.assertEqual((self.work / "lib/math/modules.order").read_bytes(), b"lib/math/polynomial.o\n")
        crc = self.records("polynomial_rust")
        self.assertEqual(set(crc), {SYMBOL})
        self.assertEqual(self.records("polynomial"), {})
        outcomes = []
        for tool in modpost_tools()[64][:2]:
            result = run([tool, "-M", "-m", "-i", "kernel.symvers", "-o", "Module.symvers", "lib/math/polynomial.o"], cwd=self.work)
            self.assertEqual(result.stderr, b"")
            outcomes.append(((self.work / "Module.symvers").read_bytes(), (self.work / "lib/math/polynomial.mod.c").read_bytes()))
        self.assertEqual(outcomes[0], outcomes[1])
        self.assertEqual(outcomes[0][0], crc[SYMBOL] + b"\tpolynomial_calc\tlib/math/polynomial\tEXPORT_SYMBOL_GPL\t\n")
        self.assertIn(b"SYMBOL_CRC(polynomial_calc, " + crc[SYMBOL], outcomes[0][1])
        command = self.work / "lib/math/.polynomial_rust.o.cmd"
        command.write_bytes(re.sub(rb"\n#SYMVER [^\n]*", b"", command.read_bytes()))
        aggregate = self.work / "lib/math/.polynomial.o.cmd"
        aggregate.write_bytes(aggregate.read_bytes() + b"\n#SYMVER polynomial_calc " + crc[SYMBOL] + b"\n")
        result = subprocess.run([modpost_tools()[64][1], "-M", "-m", "-i", "kernel.symvers", "-o", "bad.symvers", "lib/math/polynomial.o"],
                                cwd=self.work, capture_output=True, timeout=120)
        self.assertIn(b"version generation failed", result.stderr)
        self.assertIn(b"0x00000000", (self.work / "bad.symvers").read_bytes())

    def test_c_rust_restore_builtin_module_and_disabled_selection(self):
        versions = {}
        for native in (False, True, False, True):
            self.make("lib/math/polynomial.o", "lib/math/modules.order", native=native)
            owner = "polynomial_rust" if native else "polynomial"
            self.assertEqual((self.work / "lib/math/polynomial.mod").read_text(), "lib/math/" + owner + ".o\n")
            crc = self.records(owner)
            self.assertEqual(set(crc), {SYMBOL})
            if native in versions:
                self.assertEqual(versions[native], crc)
            versions[native] = crc
            self.assertEqual(module_info(self.work / "lib/math" / (owner + ".o")), expected_info(True))
            self.make("lib/math/built-in.a", native=native, state="y")
            members = run([*self.ar, "t", self.work / "lib/math/built-in.a"]).stdout.decode().splitlines()
            self.assertEqual([Path(name).name for name in members], [*self.other, owner + ".o"])
            self.assertEqual(module_info(self.work / "lib/math" / (owner + ".o")), expected_info(False))
            self.make("lib/math/built-in.a", "lib/math/modules.order", native=native, state="")
            members = run([*self.ar, "t", self.work / "lib/math/built-in.a"]).stdout.decode().splitlines()
            self.assertEqual([Path(name).name for name in members], list(self.other))
            self.assertEqual((self.work / "lib/math/modules.order").read_bytes(), b"")
        self.assertNotEqual(versions[False], versions[True])

    def test_noop_source_header_export_and_modversions_dependencies(self):
        self.make("lib/math/polynomial.o", "lib/math/modules.order")
        paths = [self.work / "lib/math" / name for name in ("polynomial_rust.o", "polynomial.o", "polynomial.mod", "modules.order")]
        before = [path.stat().st_mtime_ns for path in paths]
        self.make("lib/math/polynomial.o", "lib/math/modules.order")
        self.assertEqual([path.stat().st_mtime_ns for path in paths], before)
        command = (self.work / "lib/math/.polynomial_rust.o.cmd").read_text()
        for source in ("polynomial_rust.rs", "polynomial.rs", "ffi_export.rs", "export_header.rs"):
            self.assertIn(source, command)
        self.assertIn("$(wildcard include/config/MODVERSIONS)", command)
        dependencies = command.split("deps_lib/math/polynomial_rust.o :=", 1)[1].split("\n\n", 1)[0]
        for name in ("polynomial.rs", "export_header.rs", "libkernel.rlib", "libffi.rlib"):
            dependency = next(token for token in dependencies.split() if token.endswith("/" + name))
            self.make("lib/math/polynomial.o", "lib/math/modules.order", extra=("-W", dependency))
            after = [path.stat().st_mtime_ns for path in paths]
            self.assertEqual([a != b for a, b in zip(after, before)], [True, True, False, True])
            before = after
        (self.work / "include/config/MODVERSIONS").write_bytes(b"")
        self.make("lib/math/polynomial.o", "lib/math/modules.order")
        self.assertGreater(paths[0].stat().st_mtime_ns, before[0])
        current = [path.stat().st_mtime_ns for path in paths]
        self.make("lib/math/polynomial.o", "lib/math/modules.order")
        self.assertEqual([path.stat().st_mtime_ns for path in paths], current)



class PolynomialNativeTests(TemporaryTest):
    def test_optional_completed_native_selection_metadata_and_crc_provenance(self):
        supplied = os.environ.get("NATIVE_POLYNOMIAL_KERNEL_BUILD")
        if not supplied:
            self.skipTest("set NATIVE_POLYNOMIAL_KERNEL_BUILD for a read-only completed-output audit")
        build = Path(supplied).resolve()
        config = (build / ".config").read_text().splitlines()
        native = "CONFIG_RUST_POLYNOMIAL=y" in config
        state = "m" if "CONFIG_POLYNOMIAL=m" in config else "y" if "CONFIG_POLYNOMIAL=y" in config else "n"
        owner = "polynomial_rust.o" if native else "polynomial.o"
        archive = build / "vmlinux.a"
        members = [Path(name).name for name in run([*shlex.split(os.environ.get("AR", "ar")), "t", archive]).stdout.decode().splitlines()]
        selected = [name for name in members if name in ("polynomial.o", "polynomial_rust.o")]
        self.assertEqual(selected, [owner] if state == "y" else [])
        order = [line.removesuffix(".ko") + ".o" if line.endswith(".ko") else line
                 for line in (build / "modules.order").read_text().splitlines()]
        self.assertEqual(order.count("lib/math/polynomial.o"), int(state == "m"))
        rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
        found = [fields for fields in rows if fields[1] == SYMBOL]
        if state == "n":
            self.assertEqual(found, [])
            return
        import check_polynomial_kernel as checker
        checker.verify_linked_implementation(build, "Rust" if native else "C")
        obj = build / "lib/math" / owner
        source = OWNER if native else ROOT / "lib/math/polynomial.c"
        self.assertGreaterEqual(obj.stat().st_mtime_ns, source.stat().st_mtime_ns)
        crc, _ = dwarf_versions(dwarf_tools(), obj, {SYMBOL}, self.work)
        command = obj.with_name("." + obj.name + ".cmd").read_bytes()
        self.assertEqual(dict(re.findall(rb"#SYMVER (\w+) (0x[0-9a-f]+)", command)), crc)
        self.assertEqual(len(found), 1)
        self.assertEqual(found[0][:4], [crc[SYMBOL], SYMBOL, b"lib/math/polynomial" if state == "m" else b"vmlinux", b"EXPORT_SYMBOL_GPL"])
        self.assertEqual(module_info(obj), expected_info(state == "m"))
        if state == "m":
            from boot_kernel import module_name
            module = build / "lib/math/polynomial.ko"
            self.assertEqual(module_name(module), "polynomial")
            self.assertGreaterEqual(module.stat().st_mtime_ns, obj.stat().st_mtime_ns)
            self.assertEqual((build / "lib/math/polynomial.mod").read_text(), "lib/math/" + owner + "\n")
            for value in expected_info(True):
                self.assertIn(value, module_info(module))
        else:
            self.assertGreaterEqual(archive.stat().st_mtime_ns, obj.stat().st_mtime_ns)
            self.assertGreaterEqual((build / "vmlinux").stat().st_mtime_ns, archive.stat().st_mtime_ns)


class PolynomialAbiTests(TemporaryTest):
    def check_abi(self, bits):
        records = abi_records(bits)
        payload = pure.packed(records)
        expected = b"".join(struct.pack("<Qq", 1, pure.checked_model(item, bits)) for item in records)
        runner = shlex.split(os.environ.get("INT_MATH_I686_RUNNER", "")) if bits == 32 else []
        baseline = None
        for label in ("C-O0", "C-O2", "Rust-O0", "Rust-O2"):
            optimize = label[-1]
            obj = self.c_object(bits, optimize=optimize) if label.startswith("C") else \
                self.rust_object(bits, optimize=optimize, library=True)
            executable = self.driver(bits, obj, label)
            actual = run([*runner, executable], input=payload)
            self.assertEqual(actual.stderr, b"")
            self.assertEqual(actual.stdout, expected, (bits, label))
            if baseline is None:
                baseline = actual.stdout
            self.assertEqual(actual.stdout, baseline)
            if label == "Rust-O0":
                negative = self.driver(bits, obj, "panic-control", panic=True)
                result = subprocess.run([*runner, negative], capture_output=True, timeout=30)
                self.assertEqual((result.returncode, result.stdout, result.stderr), (97, b"", b""))
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            for label, obj in (("clang-original", self.c_object(bits, compiler=["clang"])),
                               ("clang-native", self.rust_object(bits, library=True))):
                executable = self.driver(bits, obj, label, compiler=["clang"])
                self.assertEqual(run([*runner, executable], input=payload).stdout, expected)
        # A private result mutant must be observable through the real pointer
        # ABI even though the constant ignores data and its per-step divider.
        mutant = self.work / "bad-result.c"
        mutant.write_text('''#include <linux/polynomial.h>
long __real_polynomial_calc(const struct polynomial *, long);
long __wrap_polynomial_calc(const struct polynomial *poly, long data)
{ return __real_polynomial_calc(poly, data) ^ (poly->terms[0].deg == 0); }
''')
        executable = self.driver(bits, self.rust_object(bits, library=True), "bad-result",
                                 extra=(mutant, "-Wl,--wrap=polynomial_calc"))
        changed = run([*runner, executable], input=payload).stdout
        self.assertEqual(len(changed), len(expected))
        differences = [index for index in range(len(records))
                       if changed[index * 16:(index + 1) * 16] != expected[index * 16:(index + 1) * 16]]
        self.assertEqual(differences, [index for index, item in enumerate(records) if item[3][0][0] == 0])
        self.assertTrue(differences)

    def test_genuine_64bit_original_fam_abi_and_evaluated_fields(self):
        self.check_abi(64)

    def test_genuine_32bit_original_fam_abi_and_evaluated_fields(self):
        if 32 not in rust_targets():
            self.skipTest("matching i686 core unavailable; set INT_MATH_I686_SYSROOT")
        self.check_abi(32)

    def test_original_c_kcfi_identity_and_protected_calls_with_wrong_nominal_type(self):
        clang = shlex.split(os.environ.get("CLANG", "clang"))
        if not shutil.which(clang[0]):
            if "CLANG" in os.environ:
                self.fail("explicit CLANG unavailable")
            self.skipTest("Clang required for original C KCFI validation")
        c_flags = ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        rust_flags = ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]

        def type_id(path):
            text = path.read_text()
            function = re.search(r"(?m)^define .*@polynomial_calc\(.*!kcfi_type !(\d+)", text)
            self.assertIsNotNone(function, text)
            value = re.search(r"(?m)^!" + function[1] + r" = !\{i32 (-?\d+)\}$", text)
            self.assertIsNotNone(value, text)
            return int(value[1])

        original_ir = self.work / "original.ll"
        run([*clang, *self.c_flags(), *c_flags, "-O2", "-S", "-emit-llvm", ROOT / "lib/math/polynomial.c",
             "-o", original_ir])
        expected_id = type_id(original_ir)
        records = abi_records(64)
        payload = pure.packed(records)
        original = self.c_object(compiler=clang)
        baseline = run([self.driver(64, original, "baseline")], input=payload).stdout
        protected_c = self.work / "protected-c.o"
        run([*clang, *self.c_flags(), *c_flags, "-O2", "-c", ROOT / "lib/math/polynomial.c", "-o", protected_c])
        executable = self.driver(64, protected_c, "protected-original", compiler=clang, extra=(*c_flags, "-DTEST_KCFI"))
        self.assertEqual(run([executable], input=payload).stdout, baseline)
        for optimize in ("0", "2"):
            native_ir = self.work / "native.ll"
            native = self.rust_object(optimize=optimize, library=True,
                                      extra=(*rust_flags, "--emit=llvm-ir=" + str(native_ir)))
            self.assertEqual(type_id(native_ir), expected_id)
            executable = self.driver(64, native, "protected-native-" + optimize, compiler=clang,
                                     extra=(*c_flags, "-DTEST_KCFI"))
            self.assertEqual(run([executable], input=payload).stdout, baseline)
        # Same real FAM storage and original C algorithm, but an incorrectly
        # named single-field C-layout pointer target: ordinary calls work; KCFI must
        # reject it. No invented replacement C struct is involved.
        wrong = self.work / "wrong.rs"
        wrong.write_text('''#![no_std]
#[repr(C)]
pub struct WrongPolynomial(kernel::bindings::polynomial);
extern "C" {
 fn original_polynomial_calc(poly:*const kernel::bindings::polynomial, data:kernel::ffi::c_long)->kernel::ffi::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn polynomial_calc(poly:*const WrongPolynomial, data:kernel::ffi::c_long)->kernel::ffi::c_long {
 unsafe { original_polynomial_calc(poly.cast(), data) }
}
''')
        renamed = self.work / "renamed.o"
        run([*clang, *self.c_flags(), "-O2", "-Dpolynomial_calc=original_polynomial_calc", "-c",
             ROOT / "lib/math/polynomial.c", "-o", renamed])
        wrong_obj, wrong_ir = self.work / "wrong.o", self.work / "wrong.ll"
        run([*self.rustc, *self.binding_flags(), "--edition=2021", "--crate-type=rlib", "-Cpanic=abort", "-O",
             *rust_flags, "--emit=obj=" + str(wrong_obj), "--emit=llvm-ir=" + str(wrong_ir), wrong],
            env={**environment(), "RUSTC_BOOTSTRAP": "1"})
        self.assertNotEqual(type_id(wrong_ir), expected_id)
        unchecked = self.driver(64, wrong_obj, "wrong-unchecked", compiler=clang, extra=(renamed,))
        self.assertEqual(run([unchecked], input=payload).stdout, baseline)
        protected = self.driver(64, wrong_obj, "wrong-protected", compiler=clang,
                                extra=(renamed, *c_flags, "-DTEST_KCFI"))
        import resource
        import signal
        failure = subprocess.run([protected], input=pure.packed(records[:1]), capture_output=True, timeout=30,
                                 preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        self.assertEqual((failure.returncode, failure.stdout, failure.stderr), (-signal.SIGILL, b"", b""))

    def test_module_metadata_native_dwarf_dependencies_and_statelessness(self):
        compilers = [self.cc]
        if shutil.which("clang") and "clang" not in Path(self.cc[0]).name:
            compilers.append(["clang"])
        for bits in rust_targets():
            for module in (False, True):
                for optimize in ("0", "2"):
                    for dwarf in (4, 5):
                        native = self.rust_object(bits, module, optimize, dwarf)
                        self.assertEqual(module_info(native), expected_info(module))
                        symbols = run([*shlex.split(os.environ.get("NM", "nm")), "--defined-only", native]).stdout
                        names = {line.split()[-1] for line in symbols.splitlines() if line.split()}
                        self.assertEqual(sum(b"__IS_RUST_MODULE" in name for name in names), int(module))
                        self.assertFalse(names & {b"init_module", b"cleanup_module"})
                        dependencies = (self.work / "native.d").read_text()
                        for name in ("polynomial_rust.rs", "polynomial.rs", "ffi_export.rs", "export_header.rs",
                                     "libkernel.rlib", "libffi.rlib"):
                            self.assertIn(name, dependencies)
                        crc, types = dwarf_versions(dwarf_tools(), native, {SYMBOL}, self.work)
                        self.assertIn(b"polynomial", types)
                        for compiler in compilers:
                            original = self.c_object(bits, module, optimize, dwarf, compiler)
                            self.assertEqual(module_info(original), expected_info(module))
                            c_crc, c_types = dwarf_versions(dwarf_tools(), original, {SYMBOL}, self.work)
                            self.assertIn(b"polynomial", c_types)
                            self.assertNotEqual(crc, c_crc, "native Rust DWARF must not impersonate original C CRCs")

    def test_actual_fam_layout_and_single_gpl_export(self):
        source = self.work / "layout.c"
        source.write_text('''#include <linux/polynomial.h>
const unsigned polynomial_layout[] = {
 sizeof(struct polynomial_term), _Alignof(struct polynomial_term),
 __builtin_offsetof(struct polynomial_term, deg), __builtin_offsetof(struct polynomial_term, coef),
 __builtin_offsetof(struct polynomial_term, divider), __builtin_offsetof(struct polynomial_term, divider_leftover),
 sizeof(struct polynomial), __builtin_offsetof(struct polynomial, terms), sizeof(long)};
''')
        for bits in rust_targets():
            original = self.work / "layout.o"
            run([*self.cc, *self.c_flags(bits), "-c", source, "-o", original])
            native = self.rust_object(bits)
            word = bits // 8
            self.assertEqual(elf_symbol(original, b"polynomial_layout")[3],
                             struct.pack("<9I", 4 * word, word, 0, word, 2 * word, 3 * word, word, word, word))
            self.assertEqual(elf_symbol(native, b"polynomial_layout")[3], elf_symbol(original, b"polynomial_layout")[3])
            for obj in (native, self.c_object(bits)):
                self.assertEqual([(r["name"], r["license"], r["namespace"], r["relocation_target"], r["pointer_width"])
                                  for r in read_exports(obj)], [("polynomial_calc", "GPL", "", "polynomial_calc", word)])


if __name__ == "__main__":
    unittest.main()
