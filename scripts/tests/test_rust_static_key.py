# SPDX-License-Identifier: GPL-2.0-only
"""Optional real-kernel static-key ABI, ownership and macro compile checks.

Set NATIVE_STATIC_KEY_BUILD or NATIVE_RUST_KERNEL_BUILD to a completed native
build. Tests reuse its exact compiler flags and real kernel/bindings metadata;
all generated sources, objects and dependency files go into a temporary tree.
No substitute C layouts or Opaque implementations are used. Run once with
JUMP_LABEL disabled and again with it enabled to cover both native paths.
"""

import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest

from rust_exports_test_support import ROOT, read_exports
from test_ctype_translation import elf_symbol


FALSE_DEFINITION = r'''
#include <linux/jump_label.h>
struct fixture_false_wrapper {
    unsigned long padding;
    struct static_key_false key;
};
struct fixture_false_wrapper fixture_false = {
    .padding = 0x1234, .key = STATIC_KEY_FALSE_INIT,
};
'''

C_SOURCE = FALSE_DEFINITION + r'''
#include <linux/jump_label.h>
DEFINE_STATIC_KEY_TRUE(fixture_key);
const unsigned long fixture_layout[] = {
    sizeof(struct static_key_true), __alignof__(struct static_key_true),
    offsetof(struct static_key_true, key), offsetof(struct static_key, enabled),
};
bool fixture_branch(void);
bool fixture_branch(void) { return static_branch_likely(&fixture_key); }
const unsigned long fixture_false_layout[] = {
    sizeof(struct fixture_false_wrapper), __alignof__(struct fixture_false_wrapper),
    offsetof(struct fixture_false_wrapper, key), sizeof(struct static_key_false),
};
bool fixture_unlikely(void);
bool fixture_unlikely(void) { return static_branch_unlikely(&fixture_false.key); }
'''

RUST_SOURCE = r'''
//! Independent consumer of the real native kernel static-key API.
use kernel::jump_label::StaticKeyTrue;
/// Permanently addressed, Rust-owned kernel key.
#[no_mangle]
pub static fixture_key: StaticKeyTrue = StaticKeyTrue::new();
/// Compare transparent wrapper layout with the original C header.
#[no_mangle]
pub static fixture_layout: [usize; 4] = [
    core::mem::size_of::<StaticKeyTrue>(), core::mem::align_of::<StaticKeyTrue>(),
    core::mem::offset_of!(kernel::bindings::static_key_true, key),
    core::mem::offset_of!(kernel::bindings::static_key, enabled),
];
/// The safe macro inspects the actual static's own storage.
#[no_mangle]
pub extern "C" fn fixture_branch() -> bool { kernel::static_branch_likely!(fixture_key) }
/// A safe raw pointer preserves storage identity without aliasing typed fields.
#[no_mangle]
pub extern "C" fn fixture_pointer() -> *mut kernel::bindings::static_key { fixture_key.as_ptr() }
// Only the outer test fixture is described here. Its real static-key field is
// the production bindgen type, and C supplies the actual initialized object.
#[repr(C)]
struct FalseWrapper { padding: usize, key: kernel::bindings::static_key_false }
unsafe extern "C" { static fixture_false: FalseWrapper; }
/// Compare the field offset consumed by the retained three-argument macro.
#[no_mangle]
pub static fixture_false_layout: [usize; 4] = [
    core::mem::size_of::<FalseWrapper>(), core::mem::align_of::<FalseWrapper>(),
    core::mem::offset_of!(FalseWrapper, key), core::mem::size_of::<kernel::bindings::static_key_false>(),
];
/// Exercise the existing C-owned false-key API at a nonzero field offset.
#[no_mangle]
pub extern "C" fn fixture_unlikely() -> bool {
    // SAFETY: The C fixture owns a permanent, correctly initialized false key.
    unsafe { kernel::static_branch_unlikely!(fixture_false, FalseWrapper, key) }
}
'''


def jump_relocations(path, symbol):
    command = shlex.split(os.environ.get("READELF", "readelf"))
    data = subprocess.run([*command, "-rW", path], check=True, capture_output=True,
                          env={**os.environ, "LC_ALL": "C", "LANGUAGE": "C"}).stdout.decode()
    active, rows = False, []
    for line in data.splitlines():
        if line.startswith("Relocation section "):
            active = "__jump_table'" in line
        elif active and symbol in line:
            found = re.search(r"\b(R_\w+)\s+[0-9a-f]+\s+" + re.escape(symbol) + r"\s+([+-])\s+([0-9a-f]+)", line)
            if found is None:
                raise AssertionError(line)
            rows.append((found[1], int(found[3], 16) * (1 if found[2] == "+" else -1)))
    return rows


def compiler_invocation(build, command_path, source, output, rust, extra=()):
    """Reuse flags, never execute saved shell fragments or retain output paths."""
    data = command_path.read_text()
    saved = next(line.partition(":=")[2] for line in data.splitlines() if line.startswith("savedcmd_"))
    original = next(line.partition(":=")[2].strip() for line in data.splitlines() if line.startswith("source_"))
    lexer = shlex.shlex(saved, posix=True, punctuation_chars=";&|")
    lexer.whitespace_split = True
    tokens = []
    for token in lexer:
        if token in (";", "&&", "||", "&", "|"):
            break
        tokens.append(token)
    environment = {**os.environ, "LC_ALL": "C", "LANGUAGE": "C", "RUSTC_BOOTSTRAP": "1"}
    while tokens and re.match(r"^[A-Za-z_][A-Za-z_0-9]*=", tokens[0]):
        name, _, value = tokens.pop(0).partition("=")
        environment[name] = value
    command = []
    skip = False
    for token in tokens:
        if skip:
            skip = False
        elif token in ("-o", "-MF", "-MT", "-MQ", "--out-dir", "--crate-name"):
            skip = True
        elif token == original or token in ("-MMD", "-MD"):
            continue
        elif token.startswith(("--emit=", "--out-dir=", "--crate-name=", "-Wp,-MMD,", "-Wp,-MD,")):
            continue
        else:
            command.append(token)
    if rust:
        environment["RUST_MODFILE"] = "static_key_fixture"
        command += ["--crate-name=static_key_fixture", "--out-dir", str(output.parent),
                    "--emit=obj=" + str(output), "--emit=dep-info=" + str(output.with_suffix(".d")),
                    "-Dwarnings", "-Coverflow-checks=yes"]
    else:
        command += ["-o", str(output)]
    command += [*extra, str(source)]
    return subprocess.run(command, cwd=build, env=environment, capture_output=True, timeout=120)


class RustStaticKeyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        supplied = os.environ.get("NATIVE_STATIC_KEY_BUILD", os.environ.get("NATIVE_RUST_KERNEL_BUILD"))
        if not supplied:
            raise unittest.SkipTest("set NATIVE_STATIC_KEY_BUILD to test actual native kernel bindings")
        cls.build = Path(supplied).resolve()
        config = (cls.build / ".config").read_text().splitlines()
        if "CONFIG_RUST=y" not in config:
            raise AssertionError("native kernel must enable CONFIG_RUST")
        cls.jump = "CONFIG_JUMP_LABEL=y" in config
        cls.hack = "CONFIG_HAVE_JUMP_LABEL_HACK=y" in config
        metadata = cls.build / "rust/libkernel.rmeta"
        for source in (ROOT / "rust/kernel/jump_label.rs", ROOT / "rust/kernel/types.rs"):
            if metadata.stat().st_mtime_ns < source.stat().st_mtime_ns:
                raise AssertionError(f"native kernel metadata is older than {source}; finish the build first")
        cls.temporary = tempfile.TemporaryDirectory(prefix="native-static-key-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.rust_command = cls.build / "lib/.ctype_rust.o.cmd"
        cls.c_command = cls.build / "lib/math/.gcd.o.cmd"
        for command in (cls.rust_command, cls.c_command):
            if not command.exists():
                raise AssertionError(f"retain original C gcd and native Rust owner commands: {command}")
        cls.sequence = 0
        cls.original = cls.compile_source(C_SOURCE, rust=False)
        cls.translated = cls.compile_source(RUST_SOURCE)

    @classmethod
    def compile_source(cls, source, *, rust=True, extra=(), failure=False):
        cls.sequence += 1
        stem = cls.work / str(cls.sequence)
        path = stem.with_suffix(".rs" if rust else ".c")
        obj = stem.with_suffix(".o")
        path.write_text(source)
        result = compiler_invocation(cls.build, cls.rust_command if rust else cls.c_command,
                                     path, obj, rust, extra)
        if failure:
            if result.returncode == 0:
                raise AssertionError("invalid static-key consumer unexpectedly compiled")
            return result.stderr
        if result.returncode:
            raise AssertionError(result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
        return obj

    def test_real_c_header_and_rust_bindings_have_identical_initial_storage(self):
        for symbol in (b"fixture_key", b"fixture_layout", b"fixture_false_layout"):
            with self.subTest(symbol=symbol):
                c = elf_symbol(self.original, symbol)
                rust = elf_symbol(self.translated, symbol)
                self.assertEqual(rust, c)
                self.assertEqual(rust[0], 0x11)  # Global object, not a pointer declaration.
        key = elf_symbol(self.translated, b"fixture_key")
        self.assertTrue(key[2] & 1, "Opaque interior mutation requires writable storage")
        expected_size = 16 if self.jump else 4
        # Actual header/bindings comparison above is authoritative; this sanity
        # check is explicitly limited to the available native x86-64 fixture.
        if self.translated.read_bytes()[4] == 2 and self.translated.read_bytes()[18:20] == b">\x00":
            self.assertEqual(key[1], expected_size)
        self.assertEqual(read_exports(self.translated), [])

    def test_exact_storage_arguments_reject_reference_and_deref_coercions(self):
        cases = {
            "reference": "static BAD_KEY: &StaticKeyTrue = &fixture_key;",
            "deref": "struct Indirect;\nimpl core::ops::Deref for Indirect {\n"
                     " type Target = StaticKeyTrue;\n fn deref(&self) -> &StaticKeyTrue { &fixture_key }\n"
                     "}\nstatic BAD_KEY: Indirect = Indirect;",
        }
        for name, declaration in cases.items():
            with self.subTest(name=name):
                source = RUST_SOURCE + declaration + "\n#[allow(dead_code)]\nfn bad() -> bool { kernel::static_branch_likely!(BAD_KEY) }\n"
                error = self.compile_source(source, failure=True)
                self.assertIn(b"E0308", error)
                self.assertIn(b"*const StaticKeyTrue", error)

    def test_stack_and_const_values_cannot_be_registered_as_static_storage(self):
        cases = ("let temporary = StaticKeyTrue::new();", "const temporary: StaticKeyTrue = StaticKeyTrue::new();")
        for declaration in cases:
            with self.subTest(declaration=declaration):
                source = RUST_SOURCE + "\n#[allow(dead_code, non_upper_case_globals)]\nfn bad() -> bool { " + declaration
                source += " kernel::static_branch_likely!(temporary) }\n"
                error = self.compile_source(source, failure=True)
                self.assertRegex(error, rb"E0597|E0745|does not live long enough|temporary value")

    def test_opaque_storage_is_sync_but_not_unpin(self):
        source = RUST_SOURCE + "\nfn assert_sync<T: Sync>() {}\n#[allow(dead_code)]\nfn good() { assert_sync::<StaticKeyTrue>(); }\n"
        self.compile_source(source)
        source = RUST_SOURCE + "\nfn assert_unpin<T: Unpin>() {}\n#[allow(dead_code)]\nfn bad() { assert_unpin::<StaticKeyTrue>(); }\n"
        error = self.compile_source(source, failure=True)
        self.assertIn(b"PhantomPinned", error)
        self.assertIn(b"Unpin", error)

    def test_native_macro_uses_correct_jump_table_key_offset_and_polarity(self):
        c = jump_relocations(self.original, "fixture_key")
        rust = jump_relocations(self.translated, "fixture_key")
        self.assertEqual(rust, c)
        if self.jump:
            self.assertEqual(len(rust), 1)
            self.assertEqual(rust[0][1], 3 if self.hack else 1,
                             "true likely branches use key offset zero, branch bit one, plus optional hack bit")
        else:
            self.assertEqual(rust, [])

    def test_retained_false_key_macro_uses_real_c_storage_at_nonzero_offset(self):
        c = jump_relocations(self.original, "fixture_false")
        rust = jump_relocations(self.translated, "fixture_false")
        self.assertEqual(rust, c)
        layout = elf_symbol(self.original, b"fixture_false_layout")[3]
        data = self.original.read_bytes()
        width, order = (8 if data[4] == 2 else 4), ("little" if data[5] == 1 else "big")
        offset = int.from_bytes(layout[2 * width:3 * width], order)
        self.assertGreater(offset, 0)
        if self.jump:
            self.assertEqual(len(rust), 1)
            self.assertEqual(rust[0][1], offset + (2 if self.hack else 0),
                             "false unlikely branches retain the field offset and branch bit zero")
        else:
            self.assertEqual(rust, [])
        definition = self.compile_source(FALSE_DEFINITION, rust=False)
        linked = self.work / "legacy-linked.o"
        result = subprocess.run([*shlex.split(os.environ.get("LD", "ld")), "-r", "-o", linked,
                                 self.translated, definition], capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
        self.assertEqual(elf_symbol(linked, b"fixture_false"), elf_symbol(self.original, b"fixture_false"))
        self.assertEqual(jump_relocations(linked, "fixture_false"), rust)

    def test_real_owner_retains_key_and_omits_branch_when_ffs_is_disabled(self):
        output = self.work / "no-efficient-ffs.o"
        result = compiler_invocation(self.build, self.rust_command, ROOT / "lib/math/gcd_lcm_rust.rs",
                                     output, True, ("--cfg", "CONFIG_CPU_NO_EFFICIENT_FFS"))
        self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
        self.assertEqual(elf_symbol(output, b"efficient_ffs_key"), elf_symbol(self.original, b"fixture_key"))
        records = read_exports(output)
        self.assertEqual({record["name"] for record in records}, {"gcd", "lcm", "lcm_not_zero"})
        self.assertTrue(all(record["license"] == "GPL" and record["namespace"] == "" for record in records))
        data = subprocess.run([*shlex.split(os.environ.get("READELF", "readelf")), "-SW", output],
                              check=True, capture_output=True).stdout
        self.assertNotIn(b"__jump_table", data)

    def test_compiler_outputs_and_dependency_files_stay_outside_native_tree(self):
        deps = self.translated.with_suffix(".d").read_text()
        self.assertIn("libkernel.rmeta", deps)
        self.assertIn("libbindings.rmeta", deps)
        for line in deps.splitlines():
            if ": " in line:
                targets = shlex.split(line.partition(": ")[0])
                for target in targets:
                    self.assertTrue(Path(target).is_relative_to(self.work), target)


if __name__ == "__main__":
    unittest.main()
