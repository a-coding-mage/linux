# SPDX-License-Identifier: GPL-2.0
"""Execute the original C and translated union-find through public C types."""

import json
import os
from pathlib import Path
import re
import shlex
import struct
import tempfile
import unittest

from rust_exports_test_support import read_exports, rust_targets
from kconfig_test_support import cached_conf_tools
from test_rational_build import environment, run


ROOT = Path(__file__).resolve().parents[2]


class UnionFindTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="union-find-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.env = {**environment(), "RUSTC_BOOTSTRAP": "1"}
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.bindgen = shlex.split(os.environ.get("BINDGEN", "bindgen"))

    def build(self, bits, target, optimize, compiler, cfi=False):
        folder = Path(tempfile.mkdtemp(
            prefix=str(bits) + "-" + optimize + "-" + str(cfi) + "-", dir=self.work))
        bindings = folder / "bindings.rs"
        run([*self.bindgen, ROOT / "include/linux/union_find.h", "--use-core",
             "--rust-target", "1.85", "--ctypes-prefix", "ffi", "--no-layout-tests",
             "--no-doc-comments", "-o", bindings, "--", "-m" + str(bits)], env=self.env)
        ffi = folder / "libffi.rlib"
        flags = [*target, "--edition=2021", "-Dwarnings", "-Cpanic=abort",
                 "-Coverflow-checks=yes", "-Copt-level=" + optimize]
        if cfi:
            flags += ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
        run([*self.rustc, *flags, "--crate-name=ffi", "--crate-type=rlib",
             ROOT / "rust/ffi.rs", "-o", ffi], env=self.env)
        wrapper = folder / "owner.rs"
        wrapper.write_text('''//! Compile production code against actual generated declarations.
#![no_std]
extern crate self as kernel;
pub use ffi;
#[allow(non_camel_case_types, missing_docs)]
pub mod bindings { include!(@BINDINGS@); }
#[path = @PUBLIC_API@]
pub mod union_find;
#[path = @OWNER@]
mod owner;
pub use owner::*;
static mut STATIC_NODE: bindings::uf_node = UF_INIT_NODE!(STATIC_NODE);
struct Containing { node: bindings::uf_node }
static mut STATIC_FIELD: Containing = Containing { node: unsafe { UF_INIT_NODE!(STATIC_FIELD.node) } };
static mut STATIC_ARRAY: [bindings::uf_node; 1] = [unsafe { UF_INIT_NODE!(STATIC_ARRAY[0]) }];
/// Exercise the public header initializer.
#[no_mangle]
pub unsafe extern "C" fn rust_node_init(node: *mut bindings::uf_node) {
    unsafe { union_find::uf_node_init(node) };
}
/// Verify the translated static initializer at its actual final address.
#[no_mangle]
pub extern "C" fn rust_static_init() -> i32 {
    let node = core::ptr::addr_of_mut!(STATIC_NODE);
    let field = unsafe { core::ptr::addr_of_mut!(STATIC_FIELD.node) };
    let element = unsafe { core::ptr::addr_of_mut!(STATIC_ARRAY[0]) };
    unsafe { i32::from((*node).parent == node && (*node).rank == 0
        && (*field).parent == field && (*field).rank == 0
        && (*element).parent == element && (*element).rank == 0) }
}
'''.replace("@BINDINGS@", json.dumps(str(bindings)))
            .replace("@PUBLIC_API@", json.dumps(str(ROOT / "rust/kernel/union_find.rs")))
            .replace("@OWNER@", json.dumps(str(ROOT / "lib/union_find_rust.rs"))))
        obj = folder / "owner.o"
        run([*self.rustc, *flags, "-Wmissing-docs", "-Wunsafe-op-in-unsafe-fn",
             "-Wunreachable-pub", "-Wrust-2018-idioms",
             "--extern", "ffi=" + str(ffi), "--crate-type=rlib", "--emit=obj",
             wrapper, "-o", obj], env=self.env)
        # Union-find has no module exports in C. Do not introduce any in Rust.
        self.assertEqual(read_exports(obj), [])
        cflags = ["-I" + str(ROOT / "include"), "-O" + optimize, "-Wall", "-Werror"]
        if cfi:
            cflags += ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
        original = folder / "original.o"
        run([*compiler, *cflags, "-Duf_find=c_uf_find", "-Duf_union=c_uf_union",
             "-c", ROOT / "lib/union_find.c", "-o", original], env=self.env)
        executable = folder / "compare"
        run([*compiler, *cflags, ROOT / "scripts/tests/union_find_driver.c",
             original, obj, "-o", executable], env=self.env)
        self.assertEqual(executable.read_bytes()[4], 1 if bits == 32 else 2)
        run([executable], env=self.env)

    def test_original_forests_and_unsigned_ranks(self):
        for compiler in (shlex.split(os.environ.get("HOSTCC", "cc")), ["clang"]):
            for optimize in ("0", "2", "s"):
                with self.subTest(compiler=compiler, optimize=optimize):
                    self.build(struct.calcsize("P") * 8, [], optimize, compiler)

    def test_real_i686_forests(self):
        targets = rust_targets()
        if 32 not in targets:
            self.skipTest("i686 Rust core is not installed")
        compiler = shlex.split(os.environ.get("UNION_FIND_I686_CC", "cc -m32"))
        for optimize in ("0", "2", "s"):
            with self.subTest(optimize=optimize):
                self.build(32, targets[32], optimize, compiler)

    def test_protected_public_c_calls(self):
        self.build(struct.calcsize("P") * 8, [], "2", ["clang"], cfi=True)

    def test_actual_kconfig_requires_rust_and_selected_union_find(self):
        source = (ROOT / "lib/Kconfig").read_text()
        options = []
        for name in ("UNION_FIND", "RUST_UNION_FIND"):
            found = re.search(r"(?ms)^config " + name + r"\n.*?(?=^config |\Z)", source)
            self.assertIsNotNone(found, name)
            options.append(found.group())
        kconfig = self.work / "Kconfig"
        kconfig.write_text('config RUST\n\tbool "Rust"\n'
                          'config USE\n\tbool "Consumer"\n\tselect UNION_FIND\n' +
                          "\n".join(options))
        config = self.work / ".config"
        env = {**self.env, "KCONFIG_CONFIG": str(config)}
        for tool in cached_conf_tools():
            for rust in ("n", "y"):
                for consumer in ("n", "y"):
                    for requested in (None, "n", "y"):
                        with self.subTest(tool=tool.name, rust=rust, consumer=consumer,
                                          requested=requested):
                            value = f"CONFIG_RUST={rust}\nCONFIG_USE={consumer}\n"
                            if requested is not None:
                                value += "CONFIG_RUST_UNION_FIND=" + requested + "\n"
                            config.write_text(value)
                            run([tool, "--olddefconfig", kconfig], cwd=self.work, env=env)
                            lines = config.read_text().splitlines()
                            self.assertEqual("CONFIG_UNION_FIND=y" in lines, consumer == "y")
                            self.assertEqual("CONFIG_RUST_UNION_FIND=y" in lines,
                                             rust == consumer == requested == "y")

    def test_actual_makefile_preserves_library_position_and_disabled_state(self):
        makefile = self.work / "Makefile"
        makefile.write_text(f'''srctree := {ROOT}
src := {ROOT}/lib
obj := lib
objtree := {self.work}
include {ROOT}/lib/Makefile
.PHONY: selection
selection:
\t@printf '%s\\n' '$(lib-y)' '$(filter union_find%.o,$(obj-y) $(obj-m))'
''')
        for host in ("c", "rust"):
            baseline = None
            for enabled, selected in ((True, False), (True, True), (True, False),
                                      (False, False), (False, True)):
                result = run(["make", "--no-print-directory", "-rR", "-f", makefile,
                              "selection", "HOST_TOOLS_LANG=" + host,
                              "CONFIG_UNION_FIND=" + ("y" if enabled else ""),
                              "CONFIG_RUST_UNION_FIND=" + ("y" if selected else "")],
                             cwd=self.work, env=self.env)
                library, other = result.stdout.splitlines()
                members = library.split()
                expected = b"union_find_rust.o" if selected else b"union_find.o"
                self.assertEqual([name for name in members if name.startswith(b"union_find")],
                                 [expected] if enabled else [])
                self.assertEqual(other, b"")
                if enabled:
                    normalized = [b"union_find.o" if name == expected else name for name in members]
                    if baseline is not None:
                        self.assertEqual(normalized, baseline)
                    baseline = normalized


class UnionFindNative(unittest.TestCase):
    def test_selected_library_and_real_cpuset_consumer(self):
        from check_div64_kernel import architecture, configuration, verify_build_command
        from check_glob_kernel import verify_saved_flags
        from check_polynomial_kernel import elf_target
        from rbtree_native.transport import NativeWriteWatch

        if "UNION_FIND_KERNEL_BUILD" not in os.environ:
            self.skipTest("UNION_FIND_KERNEL_BUILD supplies a completed cpuset kernel")
        supplied = os.environ["UNION_FIND_KERNEL_BUILD"]
        self.assertTrue(supplied.strip(), "UNION_FIND_KERNEL_BUILD is explicitly empty")
        build = Path(supplied).resolve()
        config = configuration(build)
        for name in ("RUST", "CFI", "UNION_FIND", "SMP", "CGROUPS", "CPUSETS", "CPUSETS_V1"):
            self.assertEqual(config.get(name), "y", name)
        selected = config.get("RUST_UNION_FIND") == "y"
        owner_name = "union_find_rust" if selected else "union_find"
        owner = build / "lib" / (owner_name + ".o")
        watch = NativeWriteWatch(build)
        with watch:
            source = ROOT / "lib" / (owner_name + (".rs" if selected else ".c"))
            dependency = ROOT / ("lib/union_find.rs" if selected else "include/linux/union_find.h")
            verify_build_command(build, owner, source, [dependency])
            verify_saved_flags(owner, "rust" if selected else "c", build)
            elf_target(owner, architecture(config))
            self.assertEqual(read_exports(owner), [])
            members = run(["ar", "t", build / "lib/lib.a"]).stdout.decode().splitlines()
            self.assertEqual([Path(name).name for name in members if "union_find" in Path(name).name],
                             [owner_name + ".o"])
            exports = (build / "Module.symvers").read_bytes().splitlines()
            self.assertFalse(any(line.split()[1] in (b"uf_find", b"uf_union") for line in exports))
            unresolved = run(["nm", "-u", build / "kernel/cgroup/cpuset-v1.o"]).stdout
            linked = run(["nm", "-g", "--defined-only", build / "vmlinux"]).stdout
            for name in (b"uf_find", b"uf_union"):
                self.assertRegex(unresolved, rb"(?m)\bU " + name + rb"$")
                self.assertRegex(linked, rb"(?m)^[0-9a-f]+ T " + name + rb"$")
        self.assertEqual(watch.events, [], "completed kernel must remain unmodified")


if __name__ == "__main__":
    unittest.main()
