# SPDX-License-Identifier: GPL-2.0
"""Original setup metadata, init lifetime, alignment and protected callbacks."""

import json
import os
from pathlib import Path
import shlex
import tempfile
import unittest

from rust_exports_test_support import rust_targets
from test_hexdump_abi import ElfRecords
from test_init_header import native_init_includes
from test_rational_build import environment, run
from rbtree_native import transport
import test_sort_native


ROOT = Path(__file__).resolve().parents[2]


class InitSetup(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="init-setup-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.includes = native_init_includes(self.work)
        self.env = dict(environment(), RUSTC_BOOTSTRAP="1")
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.bindgen = shlex.split(os.environ.get("BINDGEN", "bindgen"))
        self.c = self.work / "owner.c"
        self.c.write_text('''#include <linux/init.h>
int __init setup_one(char *value) { return *value + 1; }
int __init setup_two(char *value) { return *value + 2; }
#ifndef MODULE
__setup_param("init=", first, setup_one, 0);
__setup_param("debug", second, setup_two, 1);
__setup_param("debug", third, setup_two, 1);
__setup_param("obsolete", fourth, 0, 0);
#endif
''')
        self.rust = self.work / "owner.rs"
        self.rust.write_text('''//! Setup metadata emitted against the original header.
#![cfg_attr(not(CONFIG_RUST), no_std)]
pub use ffi;
#[allow(missing_docs, non_camel_case_types)]
pub mod bindings { include!("bindings.rs"); }
#[path = @SOURCE@]
mod setup;
#[no_mangle]
#[link_section = ".init.text"]
/// First original callback. The argument must point to one readable character.
pub unsafe extern "C" fn setup_one(value: *mut ffi::c_char) -> ffi::c_int {
    ffi::c_int::from(unsafe { value.read() }).wrapping_add(1)
}
#[no_mangle]
#[link_section = ".init.text"]
/// Second original callback. The argument must point to one readable character.
pub unsafe extern "C" fn setup_two(value: *mut ffi::c_char) -> ffi::c_int {
    ffi::c_int::from(unsafe { value.read() }).wrapping_add(2)
}
setup::setup_param!("init=", first, Some(setup_one), 0);
setup::setup_param!("debug", second, Some(setup_two), 1);
setup::setup_param!("debug", third, Some(setup_two), 1);
setup::setup_param!("obsolete", fourth, None, 0);
'''.replace("@SOURCE@", json.dumps(str(ROOT / "init/main_setup.rs"))))
        self.linker = self.work / "setup.lds"
        self.linker.write_text('''SECTIONS {
  .init.setup : {
    __setup_start = .;
    KEEP(*(.init.setup))
    __setup_end = .;
  }
} INSERT AFTER .data;
''')

    def records(self, path, module):
        image = ElfRecords(path)
        if module:
            self.assertNotIn(b".init.setup", image.names)
            self.assertNotIn(b".init.rodata", image.names)
            return []
        sections = [i for i, name in enumerate(image.names) if name == b".init.setup"]
        records = []
        # Rust may emit same-named input sections separately. Their natural
        # alignment and exact record size must still form one linker array.
        for section in sections:
            header = image.sections[section]
            self.assertEqual(header[2] & 3, 3, "original metadata is allocated/writable")
            self.assertEqual(header[8], image.word)
            size = 3 * image.word
            raw = image.section(section)
            self.assertEqual(len(raw) % size, 0)
            for offset in range(0, len(raw), size):
                name = image.pointer_string(section, offset)
                string_section, _ = image.relocations[(section, offset)]
                self.assertEqual(image.names[string_section], b".init.rodata")
                self.assertEqual(image.sections[string_section][2] & 3, 2)
                self.assertEqual(image.sections[string_section][8], 1)
                early = int.from_bytes(raw[offset + 2 * image.word:offset + 2 * image.word + 4],
                                       "little", signed=True)
                if name == b"obsolete":
                    self.assertNotIn((section, offset + image.word), image.relocations)
                    self.assertEqual(raw[offset + image.word:offset + 2 * image.word], bytes(image.word))
                else:
                    self.assertIn((section, offset + image.word), image.relocations)
                records.append((name, early))
        self.assertEqual(sorted(records), [(b"debug", 1), (b"debug", 1), (b"init=", 0), (b"obsolete", 0)])
        return sorted(records)

    def check(self, bits, compiler, optimize, *, protected=False):
        targets = rust_targets()
        if bits not in targets:
            self.skipTest("the actual i686 Rust core is required")
        for module in (False, True):
            with self.subTest(module=module):
                cflags = [*self.includes, "-m" + str(bits), "-O" + optimize, "-funsigned-char",
                          "-Wall", "-Wextra", "-Werror", *(["-DMODULE"] if module else [])]
                rflags = [*targets[bits], "--edition=2021", "--crate-type=rlib", "-Dwarnings",
                          "-Wunsafe-op-in-unsafe-fn", "-Wrust-2018-idioms", "-Cpanic=abort",
                          "-Coverflow-checks=yes", "-Crelocation-model=static", "-Copt-level=" + optimize,
                          *(["--cfg", "MODULE"] if module else [])]
                if protected:
                    cflags += ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
                    rflags += ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
                run([*self.bindgen, ROOT / "include/linux/init.h", "--use-core", "--rust-target=1.85",
                     "--ctypes-prefix=ffi", "--no-layout-tests", "--no-doc-comments",
                     "--allowlist-type=^obs_kernel_param$", "-o", self.work / "bindings.rs", "--",
                     *self.includes, "-m" + str(bits), "-funsigned-char",
                     *(["-DMODULE"] if module else [])], env=self.env)
                ffi = self.work / "libffi.rlib"
                run([*self.rustc, *rflags, "--crate-name=ffi", ROOT / "rust/ffi.rs", "-o", ffi], env=self.env)
                original, translated = self.work / "c.o", self.work / "rust.o"
                run([*compiler, *cflags, "-c", self.c, "-o", original], env=self.env)
                run([*self.rustc, *rflags, "--extern", "ffi=" + str(ffi),
                     "--emit=obj=" + str(translated), self.rust], env=self.env)
                self.assertEqual(self.records(original, module), self.records(translated, module))
                for owner in (original, translated):
                    binary = self.work / "compare"
                    run([*compiler, *cflags, ROOT / "scripts/tests/init_setup_driver.c", owner,
                         "-no-pie", "-Wl,-T," + str(self.linker), "-o", binary], env=self.env)
                    self.assertEqual(binary.read_bytes()[4], 1 if bits == 32 else 2)
                    run([binary], env=self.env)

    def test_original_registration_and_init_sections(self):
        for compiler in (shlex.split(os.environ.get("HOSTCC", "cc")), ["clang"]):
            for optimize in ("0", "2", "s"):
                with self.subTest(compiler=compiler, optimize=optimize):
                    self.check(64, compiler, optimize)

    def test_real_i686_record_stride(self):
        compiler = shlex.split(os.environ.get("INIT_HEADER_I686_CC", "cc -m32"))
        for optimize in ("0", "2", "s"):
            self.check(32, compiler, optimize)

    def test_protected_original_parser_callbacks(self):
        self.check(64, ["clang"], "2", protected=True)

    def native(self, variable):
        if variable not in os.environ:
            self.skipTest(variable + " supplies an immutable native kernel donor")
        self.assertTrue(os.environ[variable].strip(), variable + " is explicitly empty")
        build = Path(os.environ[variable]).resolve()
        reader = test_sort_native.SortNativeTests()
        reader.rust = Path(self.rustc[0]).resolve()
        watch = transport.NativeWriteWatch(build)
        with watch:
            cflags = reader.native_flags(build, "init/.main.o.cmd", False)
            rflags = reader.native_flags(build, "lib/.list_sort_rust.o.cmd", True)
            saved = reader.saved(build, "rust/bindings/.bindings_generated.rs.cmd")
            bflags = transport.native_flags(saved[saved.index("--") + 1:], build, "c")
            run([*self.bindgen, ROOT / "rust/bindings/init_main.h", "--use-core", "--rust-target=1.85",
                 "--ctypes-prefix=kernel::ffi", "--no-layout-tests", "--no-doc-comments",
                 "--allowlist-type=^obs_kernel_param$", "-o", self.work / "bindings.rs", "--",
                 *bflags], env=self.env)
            self.rust.write_text(self.rust.read_text().replace("pub use ffi;", "use kernel::ffi;"))
            original, translated = self.work / "native-c.o", self.work / "native-rust.o"
            run([*cflags, "-c", self.c, "-o", original], env=self.env)
            run([*rflags, "--crate-name=init_setup_native", "--emit=obj=" + str(translated),
                 self.rust], env=self.env)
            self.assertEqual(self.records(original, False), self.records(translated, False))
        self.assertEqual(watch.events, [], "native donor must remain unchanged")

    def test_native_x86_canonical_setup_metadata(self):
        self.native("INIT_MAIN_X86_BUILD")

    def test_native_arm64_canonical_setup_metadata(self):
        self.native("INIT_MAIN_ARM64_BUILD")


if __name__ == "__main__":
    unittest.main()
