#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Original C table/header oracle for the allocation-free Rust ctype helpers."""

import ctypes
import itertools
import json
import os
from pathlib import Path
import random
import re
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
HELPERS = ("__ismask", "isalnum", "isalpha", "iscntrl", "isgraph", "islower",
           "isprint", "ispunct", "isspace", "isupper", "isxdigit", "isascii",
           "isdigit", "isodigit", "toascii", "__tolower", "__toupper",
           "tolower", "toupper", "_tolower")
CONSTANTS = ("_U", "_L", "_D", "_C", "_P", "_S", "_X", "_SP")


def command(name, fallback):
    return shlex.split(os.environ.get(name, fallback))


def run(args, **kwargs):
    result = subprocess.run(args, capture_output=True, timeout=90, **kwargs)
    if result.returncode:
        raise RuntimeError(shlex.join(map(str, args)) + "\n" +
                           result.stdout.decode(errors="replace") +
                           result.stderr.decode(errors="replace"))
    return result.stdout


def elf_symbol(path, name):
    """Read the symbol's bytes/size/section flags directly from its ELF object."""
    data = Path(path).read_bytes()
    order = "<" if data[5] == 1 else ">"
    if data[4] == 2:
        offset, = struct.unpack_from(order + "Q", data, 40)
        stride, count = struct.unpack_from(order + "HH", data, 58)
        section_format, symbol_format = "IIQQQQIIQQ", "IBBHQQ"
    else:
        offset, = struct.unpack_from(order + "I", data, 32)
        stride, count = struct.unpack_from(order + "HH", data, 46)
        section_format, symbol_format = "IIIIIIIIII", "IIIBBH"
    sections = [struct.unpack_from(order + section_format, data, offset + i * stride)
                for i in range(count)]
    found = []
    for symbols in sections:
        if symbols[1] != 2:
            continue
        strings = sections[symbols[6]]
        for at in range(symbols[4], symbols[4] + symbols[5], symbols[9]):
            fields = struct.unpack_from(order + symbol_format, data, at)
            if data[4] == 2:
                label, info, _, section, value, size = fields
            else:
                label, value, size, info, _, section = fields
            label += strings[4]
            if data[label:data.index(0, label)] != name:
                continue
            target = sections[section]
            begin = target[4] + value - target[3]
            found.append((info, size, target[2], data[begin:begin + size]))
    if len(found) != 1:
        raise AssertionError((name, found))
    return found[0]


C_ADAPTER = r"""
#include <limits.h>
#include <stdint.h>
#include <linux/ctype.h>
int c_eval(unsigned helper, int value)
{
    switch (helper) {
@CASES@
    default: return -999;
    }
}
int c_constant(unsigned index)
{
    const unsigned char values[] = { @CONSTANTS@ };
    return index < sizeof(values) ? values[index] : -1;
}
int c_char_signed(void) { return CHAR_MIN < 0; }
"""

RUST_ADAPTER = r"""
//! Safe helper calls through nested modules and the real production wrapper.
#![no_std]
#[cfg(CONFIG_RUST)]
extern crate self as kernel;
#[cfg(CONFIG_RUST)]
pub extern crate ffi;

mod outer {
    pub(crate) mod inner {
        #[path = @SOURCE@]
        pub(crate) mod implementation;
    }
}
pub use outer::inner::implementation::*;

mod safe_calls {
    #![forbid(unsafe_code)]
    use super::outer::inner::implementation as ctype;
    pub(super) fn evaluate(helper: u32, value: i32) -> i32 {
        match helper {
@CASES@
            _ => -999,
        }
    }
    pub(super) fn constant(index: u32) -> i32 {
        match index {
@CONSTANTS@
            _ => -1,
        }
    }
}

/// Call each safe Rust helper without an unsafe expression or block.
#[no_mangle]
pub extern "C" fn rust_eval(helper: u32, value: i32) -> i32 {
    safe_calls::evaluate(helper, value)
}
/// Check the byte-valued classification constants.
#[no_mangle]
pub extern "C" fn rust_constant(index: u32) -> i32 {
    safe_calls::constant(index)
}
/// Report the actual plain-char representation used by the selected API.
#[no_mangle]
pub extern "C" fn rust_char_signed() -> i32 {
    #[cfg(CONFIG_RUST)]
    { i32::from(ffi::c_char::MIN != 0) }
    #[cfg(not(CONFIG_RUST))]
    { i32::from(core::ffi::c_char::MIN != 0) }
}
"""

KERNEL_ADAPTER = r"""
//! Minimal kernel environment around the real public ctype API.
#![no_std]
extern crate self as kernel;
pub use ffi;
/// The original C declaration is an incomplete immutable byte array.
pub mod bindings {
    extern "C" {
        /// Bindgen's representation of include/linux/ctype.h's declaration.
        pub static _ctype: [u8; 0];
    }
}
#[path = @SOURCE@]
pub mod ctype;
"""

CONSUMER_ADAPTER = r"""
//! An independent consumer imports helpers, never the table-owning source.
#![no_std]
mod safe_calls {
    #![forbid(unsafe_code)]
    use kernel::ctype;
    pub(super) fn evaluate(helper: u32, value: i32) -> i32 {
        match helper {
@CASES@
            _ => -999,
        }
    }
    pub(super) fn constant(index: u32) -> i32 {
        match index {
@CONSTANTS@
            _ => -1,
        }
    }
}
/// Exercise the public API from a separately compiled crate.
#[no_mangle]
pub extern "C" fn rust_eval(helper: u32, value: i32) -> i32 {
    safe_calls::evaluate(helper, value)
}
/// Read the API's public classification constants.
#[no_mangle]
pub extern "C" fn rust_constant(index: u32) -> i32 {
    safe_calls::constant(index)
}
/// Preserve the selected kernel API's actual C-char representation.
#[no_mangle]
pub extern "C" fn rust_char_signed() -> i32 {
    i32::from(kernel::ffi::c_char::MIN != 0)
}
/// Report the symbol address without reading the zero-length binding type.
#[no_mangle]
pub extern "C" fn rust_table_address() -> *const u8 {
    // Only the address is formed; the zero-length declaration is not used to
    // access any byte. Even this operation requires no unsafe block.
    core::ptr::addr_of!(kernel::bindings::_ctype).cast()
}
"""


class CtypeTranslationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="ctype-translation-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.directory = Path(cls.temporary.name)
        cls.cc = command("HOSTCC", "cc")
        cls.rustc = command("HOSTRUSTC", "rustc")
        include = cls.directory / "include" / "linux"
        include.mkdir(parents=True)
        # Only the kernel build environment is adapted: the table and every
        # classification/conversion expression come from unchanged C sources.
        (include / "ctype.h").write_text('#include "' + str(ROOT / "include/linux/ctype.h") + '"\n')
        (include / "compiler.h").write_text("""
#ifdef CTYPE_TEST_FALLBACK
#undef __has_builtin
#define __has_builtin(x) 0
#endif
""")
        (include / "export.h").write_text("#define EXPORT_SYMBOL(symbol)\n")
        c_source = cls.directory / "adapter.c"
        cases = []
        for index, helper in enumerate(HELPERS):
            # Rust's predicates deliberately return bool. The C builtin may
            # represent true as any nonzero int; compare its truth value.
            expression = helper + "(value)"
            if helper.startswith("is"):
                expression = "!!(" + expression + ")"
            cases.append(f"    case {index}: return {expression};")
        c_source.write_text(C_ADAPTER.replace("@CASES@", "\n".join(cases))
                            .replace("@CONSTANTS@", ", ".join(CONSTANTS)))
        cls.c = {}
        cls.c_objects = []
        for signed, fallback in itertools.product((False, True), (False, True)):
            stem = f"c-{signed}-{fallback}"
            table = cls.directory / (stem + ".o")
            flags = ["-O2", "-fPIC", "-fsigned-char" if signed else "-funsigned-char",
                     "-I" + str(include.parent)]
            if fallback:
                flags += ["-DCTYPE_TEST_FALLBACK"]
            run(cls.cc + flags + ["-c", ROOT / "lib/ctype.c", "-o", table])
            library = cls.directory / (stem + ".so")
            run(cls.cc + flags + ["-shared", "-Wl,-z,defs", c_source, table, "-o", library])
            cls.c_objects.append(table)
            cls.c[signed, fallback] = cls.bind(ctypes.CDLL(str(library)), "c")

        ffi = cls.directory / "libffi.rlib"
        run(cls.rustc + ["--edition=2021", "--crate-type=rlib", "--crate-name=ffi",
                        "-Dwarnings", "-Cpanic=abort", ROOT / "rust/ffi.rs", "-o", ffi])
        cls.rust = {}
        cls.rust_objects = []
        for native, wrapper in itertools.product((False, True), (False, True)):
            stem = f"rust-{native}-{wrapper}"
            source = cls.directory / (stem + ".rs")
            cases = []
            for index, helper in enumerate(HELPERS):
                argument = "value as _" if helper in ("__tolower", "__toupper", "_tolower") else "value"
                cases.append(f"            {index} => ctype::{helper}({argument}) as i32,")
            source.write_text(RUST_ADAPTER.replace("@SOURCE@", json.dumps(str(
                ROOT / "lib" / ("ctype_rust.rs" if wrapper else "ctype.rs"))))
                .replace("@CASES@", "\n".join(cases))
                .replace("@CONSTANTS@", "\n".join(
                    f"            {index} => i32::from(ctype::{name}),"
                    for index, name in enumerate(CONSTANTS))))
            obj = cls.directory / (stem + ".o")
            flags = ["--edition=2021", "--crate-type=rlib", "--crate-name=ctype_oracle",
                     "--emit=obj", "-Crelocation-model=pic", "-Copt-level=2",
                     "-Cpanic=abort", "-Coverflow-checks=yes", "-Dwarnings",
                     "-Dunsafe-op-in-unsafe-fn", "-Wmissing-docs", "-Wrust-2018-idioms",
                     "-Wunreachable-pub", "--check-cfg=cfg(CONFIG_RUST)"]
            if native:
                flags += ["--cfg=CONFIG_RUST", "--extern", "ffi=" + str(ffi)]
            run(cls.rustc + flags + [source, "-o", obj])
            library = cls.directory / (stem + ".so")
            # No host panic/runtime or implementation stub is linked. A
            # dependency on one is an error, not silently satisfied by std.
            run(cls.cc + ["-shared", "-Wl,-z,defs", obj, "-o", library])
            cls.rust_objects.append(obj)
            cls.rust[native, wrapper] = cls.bind(ctypes.CDLL(str(library)), "rust")
        cls.build_independent_consumers(ffi)

    @classmethod
    def build_independent_consumers(cls, native_ffi):
        """Build the public API and consumers as genuinely separate crates."""
        flags = ["--edition=2021", "-Crelocation-model=pic", "-Copt-level=2",
                 "-Cpanic=abort", "-Coverflow-checks=yes", "-Dwarnings",
                 "-Dunsafe-op-in-unsafe-fn", "-Wmissing-docs", "-Wrust-2018-idioms",
                 "-Wunreachable-pub", "--check-cfg=cfg(CONFIG_RUST)", "--cfg=CONFIG_RUST"]
        cls.independent_flags = flags
        cls.native_ffi = native_ffi
        owner = cls.directory / "table-owner.rs"
        owner.write_text("//! The sole Rust table owner.\n#![no_std]\n"
                         "extern crate self as kernel;\npub use ffi;\n"
                         "#[path = " + json.dumps(str(ROOT / "lib/ctype_rust.rs")) + "]\n"
                         "mod implementation;\npub use implementation::*;\n")
        cls.owner_object = cls.directory / "table-owner.o"
        run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=ctype_owner",
                                "--emit=obj", "--extern", "ffi=" + str(native_ffi),
                                owner, "-o", cls.owner_object])
        cls.consumers = {}
        cls.consumer_objects = []
        cls.kernel_objects = []
        cls.consumer_builds = []
        c_source = cls.directory / "adapter.c"
        include = cls.directory / "include"
        address = cls.directory / "address.c"
        address.write_text("#include <linux/ctype.h>\n"
                           "const unsigned char *c_table_address(void) { return _ctype; }\n")
        for signed in (False, True):
            directory = cls.directory / ("consumer-signed" if signed else "consumer-native")
            directory.mkdir()
            # The native case uses the production ffi crate. The signed case
            # deliberately varies only the C-char alias, exercising the same
            # API's return representation independently of the host ABI.
            ffi = directory / "libffi.rlib"
            ffi_source = ROOT / "rust/ffi.rs"
            if signed:
                ffi_source = directory / "ffi.rs"
                ffi_source.write_text("//! Signed-char ABI fixture.\n#![no_std]\n"
                                      "/// Plain C char.\n#[allow(non_camel_case_types)]\n"
                                      "pub type c_char = i8;\n")
            run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=ffi",
                                    ffi_source, "-o", ffi])
            kernel_source = directory / "kernel.rs"
            kernel_source.write_text(KERNEL_ADAPTER.replace(
                "@SOURCE@", json.dumps(str(ROOT / "rust/kernel/ctype.rs"))))
            kernel = directory / "libkernel.rlib"
            kernel_obj = directory / "kernel.o"
            run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=kernel",
                                    "--emit=link=" + str(kernel) + ",obj=" + str(kernel_obj),
                                    "--extern", "ffi=" + str(ffi), kernel_source])
            cls.kernel_objects.append(kernel_obj)
            cls.consumer_builds.append((directory, kernel))
            consumer = directory / "consumer.rs"
            cases = []
            for index, helper in enumerate(HELPERS):
                argument = "value as _" if helper in ("__tolower", "__toupper", "_tolower") else "value"
                cases.append(f"            {index} => ctype::{helper}({argument}) as i32,")
            consumer.write_text(CONSUMER_ADAPTER.replace("@CASES@", "\n".join(cases))
                                .replace("@CONSTANTS@", "\n".join(
                                    f"            {index} => i32::from(ctype::{name}),"
                                    for index, name in enumerate(CONSTANTS))))
            obj = directory / "consumer.o"
            run(cls.rustc + flags + ["--crate-type=rlib", "--crate-name=consumer", "--emit=obj",
                                    "--extern", "kernel=" + str(kernel),
                                    "-Ldependency=" + str(directory), consumer, "-o", obj])
            cls.consumer_objects.append(obj)
            for implementation, table in (("c", cls.c_objects[0]), ("rust", cls.owner_object)):
                library = directory / (implementation + ".so")
                run(cls.cc + ["-O2", "-shared", "-fPIC", "-Wl,-z,defs",
                              "-fsigned-char" if signed else "-funsigned-char",
                              "-I" + str(include), c_source, address, obj, kernel, table,
                              "-o", library])
                bound = cls.bind(ctypes.CDLL(str(library)), "rust")
                cls.consumers[signed, implementation] = library, bound

    @staticmethod
    def bind(library, prefix):
        evaluate = getattr(library, prefix + "_eval")
        evaluate.argtypes = [ctypes.c_uint, ctypes.c_int]
        evaluate.restype = ctypes.c_int
        constant = getattr(library, prefix + "_constant")
        constant.argtypes = [ctypes.c_uint]
        constant.restype = ctypes.c_int
        signed = getattr(library, prefix + "_char_signed")
        signed.argtypes = []
        signed.restype = ctypes.c_int
        return library, evaluate, constant, bool(signed())

    def test_all_256_table_bytes_and_readonly_unmangled_symbol(self):
        reference = bytes((ctypes.c_ubyte * 256).in_dll(self.c[False, False][0], "_ctype"))
        self.assertEqual(len(reference), 256)
        for obj in self.c_objects + self.rust_objects:
            info, size, flags, data = elf_symbol(obj, b"_ctype")
            self.assertEqual(info, 0x11, obj)  # STB_GLOBAL + STT_OBJECT
            self.assertEqual(size, 256, obj)
            self.assertEqual(flags & 3, 2, obj)  # allocated and not writable
            self.assertEqual(data, reference, obj)
        for library, *_ in self.rust.values():
            self.assertEqual(bytes((ctypes.c_ubyte * 256).in_dll(library, "_ctype")), reference)

    def compare_values(self, values):
        for native_wrapper, (_, rust, _, signed) in self.rust.items():
            oracles = [self.c[signed, fallback][1] for fallback in (False, True)]
            for value in values:
                for index, helper in enumerate(HELPERS):
                    expected = oracles[0](index, value)
                    self.assertEqual(oracles[1](index, value), expected, (helper, value))
                    self.assertEqual(rust(index, value), expected,
                                     (native_wrapper, helper, value, signed))

    def test_all_classification_and_conversion_helpers_on_every_byte(self):
        self.compare_values(range(256))
        for _, _, constants, signed in self.rust.values():
            for index in range(len(CONSTANTS)):
                self.assertEqual(constants(index), self.c[signed, False][2](index))

    def test_full_signed_and_unsigned_16bit_promotions(self):
        self.compare_values(range(-32768, 65536))

    def test_large_integer_edges_and_random_out_of_byte_inputs(self):
        values = [-2**31, -2**31 + 1, -2**24, -65537, -65536, -257, -256, -255,
                  -129, -128, -1, 0, 127, 128, 255, 256, 257, 65535, 65536,
                  65537, 2**24, 2**31 - 2, 2**31 - 1]
        rng = random.Random(0xc7_1a71)
        values += [rng.randrange(-2**31, 2**31) for _ in range(4096)]
        self.compare_values(values)

    def test_original_latin1_nbsp_and_punctuation_exceptions(self):
        def call(helper, value):
            return next(iter(self.rust.values()))[1](HELPERS.index(helper), value)
        for value in range(128, 160):
            self.assertEqual(call("__ismask", value), 0)
        for helper in ("isspace", "isprint"):
            self.assertEqual(call(helper, 160), 1)
        for helper in ("isascii", "isgraph", "isalnum"):
            self.assertEqual(call(helper, 160), 0)
        for value in (215, 247):
            self.assertEqual(call("ispunct", value), 1)
            self.assertEqual(call("isalpha", value), 0)
            self.assertEqual(call("tolower", value), value)
            self.assertEqual(call("toupper", value), value)
        self.assertEqual(call("tolower", 216), 248)
        self.assertEqual(call("toupper", 248), 216)
        # These surprising historical table conversions are not Unicode case
        # folding: sharp-s (223) and y-diaeresis (255) remain the C behavior.
        self.assertEqual(call("toupper", 223), 191)
        self.assertEqual(call("toupper", 255), 223)

    def test_digit_keeps_int_but_other_byte_helpers_truncate(self):
        for _, evaluate, _, _ in self.rust.values():
            for base in (-65536, -256, 256, 65536):
                for digit in range(ord("0"), ord("7") + 1):
                    value = base + digit
                    self.assertEqual(evaluate(HELPERS.index("isdigit"), value), 0)
                    self.assertEqual(evaluate(HELPERS.index("isodigit"), value), 1)
                    self.assertEqual(evaluate(HELPERS.index("isxdigit"), value), 1)
                self.assertEqual(evaluate(HELPERS.index("isascii"), base), 1)
                self.assertEqual(evaluate(HELPERS.index("isspace"), base), 0)

    def test_real_native_unsigned_char_and_host_char_return_forms(self):
        self.assertFalse(self.c[False, False][3])
        self.assertTrue(self.c[True, False][3])
        for (native, _), (_, evaluate, _, signed) in self.rust.items():
            if native:
                self.assertFalse(signed)
            for byte in range(256):
                expected = self.c[signed, False][1](HELPERS.index("_tolower"), byte)
                self.assertEqual(evaluate(HELPERS.index("_tolower"), byte), expected)
                self.assertEqual(expected & 255, byte | 32)

    def test_no_std_nested_imports_require_no_runtime_or_unsafe_helpers(self):
        # The setup compiled canonical and production imports two modules
        # deep, invoked every helper under forbid(unsafe_code), and linked only
        # the Rust object. Also reject unresolved panic/allocator/FFI symbols.
        for obj in self.rust_objects:
            undefined = run(command("NM", "nm") + ["--undefined-only", obj])
            self.assertEqual(undefined, b"", obj)

    def test_independent_consumers_match_all_helpers_and_char_forms(self):
        rng = random.Random(0x_c7_1b)
        values = list(range(-1024, 1024))
        values += [-2**31, -2**31 + 1, 2**31 - 2, 2**31 - 1]
        values += [rng.randrange(-2**31, 2**31) for _ in range(4096)]
        for key, (_, (_, evaluate, constants, signed)) in self.consumers.items():
            self.assertEqual(signed, key[0])
            oracle = self.c[signed, False][1]
            for value in values:
                for index, helper in enumerate(HELPERS):
                    self.assertEqual(evaluate(index, value), oracle(index, value),
                                     (key, helper, value))
            for index in range(len(CONSTANTS)):
                self.assertEqual(constants(index), self.c[signed, False][2](index))

    def test_independent_consumers_share_exactly_one_selected_table(self):
        reference = bytes((ctypes.c_ubyte * 256).in_dll(self.c[False, False][0], "_ctype"))
        for key, (path, (library, *_)) in self.consumers.items():
            info, size, flags, data = elf_symbol(path, b"_ctype")
            self.assertEqual((info, size, flags & 3, data), (0x11, 256, 2, reference), key)
            table = (ctypes.c_ubyte * 256).in_dll(library, "_ctype")
            self.assertEqual(bytes(table), reference)
            for name in ("c_table_address", "rust_table_address"):
                address = getattr(library, name)
                address.argtypes = []
                address.restype = ctypes.c_void_p
                self.assertEqual(address(), ctypes.addressof(table), (key, name))
            # Dynamic artifacts may have libc setup references, but no missing
            # table, Rust runtime, allocation or panic implementation is allowed.
            undefined = run(command("NM", "nm") + ["--undefined-only", path])
            for forbidden in (b"_ctype", b"alloc", b"panic", b"rust_"):
                self.assertNotIn(forbidden, undefined, (key, undefined))

    def test_independent_crates_reference_but_do_not_define_the_table(self):
        kernel_definitions = set()
        for obj in self.kernel_objects + self.consumer_objects:
            defined = run(command("NM", "nm") + ["--defined-only", obj])
            names = [line.split()[-1] for line in defined.splitlines() if line.split()]
            self.assertNotIn(b"_ctype", names, obj)
            if obj in self.kernel_objects:
                kernel_definitions.update(names)
                undefined = run(command("NM", "nm") + ["--undefined-only", obj])
                self.assertEqual([line.split()[-1] for line in undefined.splitlines()],
                                 [b"_ctype"], obj)
        for obj in self.consumer_objects:
            undefined = run(command("NM", "nm") + ["--undefined-only", obj])
            references = {line.split()[-1] for line in undefined.splitlines()}
            self.assertIn(b"_ctype", references, obj)
            # Cross-crate calls may inline the public helper yet retain a call
            # to the private mask accessor. It must resolve from the real API
            # crate, whose sole remaining dependency is the selected table.
            self.assertLessEqual(references, kernel_definitions | {b"_ctype"}, obj)
        info, size, flags, _ = elf_symbol(self.owner_object, b"_ctype")
        self.assertEqual((info, size, flags & 3), (0x11, 256, 2))

    def test_actual_kernel_crate_registers_public_unconditional_ctype_api(self):
        source = (ROOT / "rust/kernel/lib.rs").read_text()
        # The host adapter intentionally omits unrelated native kernel APIs.
        # Check the real crate registration separately so it cannot disappear
        # while the adapter's own module declaration continues to compile.
        source = re.sub(r"/\*.*?\*/|//[^\n]*", "", source, flags=re.DOTALL)
        declarations = list(re.finditer(r"(?m)^pub mod ctype;[ \t]*$", source))
        self.assertEqual(len(declarations), 1, "kernel must publicly register ctype exactly once")
        # Inspect the whole prefix after the previous module/use declaration,
        # including multi-line attributes, rather than just the previous line.
        attached = source[:declarations[0].start()].rsplit(";", 1)[-1]
        self.assertEqual(attached.strip(), "", "ctype must remain an unconditional public API")

    def test_shared_kernel_api_dep_info_tracks_helper_not_table_owner(self):
        for directory, _ in self.consumer_builds:
            depfile = directory / "shared-api.d"
            source = directory / "kernel.rs"
            run(self.rustc + self.independent_flags + [
                "--crate-type=rlib", "--crate-name=kernel", "--emit=dep-info=" + str(depfile),
                "--extern", "ffi=" + str(directory / "libffi.rlib"), source])
            # Normalize the relative components kept by rustc for #[path]
            # dependencies. shlex also handles make's escaped path spaces.
            dependencies = {Path(name).resolve() for name in shlex.split(
                depfile.read_text().splitlines()[0].partition(": ")[2])}
            self.assertLessEqual({source.resolve(), ROOT / "rust/kernel/ctype.rs",
                                  ROOT / "include/linux/ctype_header.rs"}, dependencies)
            for owner in ("lib/ctype.rs", "lib/ctype_rust.rs", "lib/ctype.c"):
                self.assertNotIn(ROOT / owner, dependencies)

    def test_independent_consumers_at_o0_and_os_with_both_char_forms(self):
        values = list(range(-256, 512))
        values += [-2**31, -2**31 + 1, -65537, -65536, 65535, 65536,
                   65536 + ord("7"), 2**31 - 2, 2**31 - 1]
        for (directory, _), optimize in itertools.product(self.consumer_builds, ("0", "s")):
            signed = directory.name == "consumer-signed"
            flags = [flag for flag in self.independent_flags if not flag.startswith("-Copt-level=")]
            flags += ["-Copt-level=" + optimize]
            kernel = directory / ("libkernel-o" + optimize + ".rlib")
            run(self.rustc + flags + [
                "--crate-type=rlib", "--crate-name=kernel",
                "--extern", "ffi=" + str(directory / "libffi.rlib"),
                directory / "kernel.rs", "-o", kernel])
            obj = directory / ("consumer-o" + optimize + ".o")
            run(self.rustc + flags + [
                "--crate-type=rlib", "--crate-name=consumer", "--emit=obj",
                "--extern", "kernel=" + str(kernel), "-Ldependency=" + str(directory),
                directory / "consumer.rs", "-o", obj])
            for implementation, table in (("c", self.c_objects[0]), ("rust", self.owner_object)):
                with self.subTest(optimize=optimize, signed=signed, table=implementation):
                    library = directory / (implementation + "-o" + optimize + ".so")
                    run(self.cc + [
                        "-O" + optimize, "-shared", "-fPIC", "-Wl,-z,defs",
                        "-fsigned-char" if signed else "-funsigned-char",
                        "-I" + str(self.directory / "include"), self.directory / "adapter.c",
                        obj, kernel, table, "-o", library])
                    loaded = ctypes.CDLL(str(library))
                    _, evaluate, constants, char_signed = self.bind(loaded, "rust")
                    _, c_evaluate, _, c_signed = self.bind(loaded, "c")
                    self.assertEqual(char_signed, signed)
                    self.assertEqual(c_signed, signed)
                    oracle = self.c[signed, False][1]
                    for value in values:
                        for index, helper in enumerate(HELPERS):
                            expected = oracle(index, value)
                            self.assertEqual(c_evaluate(index, value), expected, (helper, value))
                            self.assertEqual(evaluate(index, value), expected, (helper, value))
                    for index in range(len(CONSTANTS)):
                        self.assertEqual(constants(index), self.c[signed, False][2](index))
                    info, size, section_flags, _ = elf_symbol(library, b"_ctype")
                    self.assertEqual((info, size, section_flags & 3), (0x11, 256, 2))
                    undefined = run(command("NM", "nm") + ["--undefined-only", library])
                    for forbidden in (b"_ctype", b"alloc", b"panic", b"rust_"):
                        self.assertNotIn(forbidden, undefined)

    def test_shared_api_const_evaluation_respects_foreign_table_boundary(self):
        for directory, kernel in self.consumer_builds:
            source = directory / "const-consumer.rs"
            source.write_text("""
//! Only table-independent helpers are evaluable in consumer constants.
#![no_std]
use kernel::ctype;
const _: () = assert!(ctype::isascii(0x141));
const _: () = assert!(ctype::toascii(0xff) == 0x7f);
const _: () = assert!(!ctype::isdigit(0x137));
const _: () = assert!(ctype::isdigit(0x37));
const _: () = assert!(ctype::isodigit(0x137));
const _: () = assert!(ctype::_tolower(0x41) == 0x61);
const _: () = assert!(ctype::_tolower(0xffu8 as kernel::ffi::c_char) as u8 == 0xff);
""")
            obj = directory / "const-consumer.o"
            args = self.rustc + self.independent_flags + [
                "--crate-type=rlib", "--crate-name=const_consumer", "--emit=obj",
                "--extern", "kernel=" + str(kernel), "-Ldependency=" + str(directory),
                source, "-o", obj]
            run(args)
            self.assertEqual(run(command("NM", "nm") + ["--undefined-only", obj]), b"")
            source.write_text("//! Foreign bytes cannot be read by CTFE.\n#![no_std]\n"
                              "const _: bool = kernel::ctype::isalpha(65);\n")
            failed = subprocess.run(args, capture_output=True, timeout=90)
            self.assertNotEqual(failed.returncode, 0)
            self.assertIn(b"E0080", failed.stderr)
            self.assertIn(b"extern static", failed.stderr)

    def test_owning_crate_helpers_remain_const_evaluable(self):
        source = self.directory / "const-owner.rs"
        values = (160, 215, 247, 255, 65536 + ord("7"))
        calls, expected = [], []
        oracle = self.c[False, False][1]
        for value in values:
            for index, helper in enumerate(HELPERS):
                argument = str(value) + "i32"
                if helper in ("__tolower", "__toupper", "_tolower"):
                    argument += " as _"
                calls.append(f"    {helper}({argument}) as i32,")
                expected.append(oracle(index, value))
        source.write_text("//! All owner helpers retain their const API.\n#![no_std]\n"
                          "extern crate self as kernel;\npub use ffi;\n"
                          "#[path = " + json.dumps(str(ROOT / "lib/ctype.rs")) + "]\n"
                          "mod implementation;\npub use implementation::*;\n"
                          "/// Compile-time results of every helper.\n#[no_mangle]\n"
                          f"pub static CONST_RESULTS: [i32; {len(calls)}] = [\n" +
                          "\n".join(calls) + "\n];\n")
        obj = self.directory / "const-owner.o"
        run(self.rustc + self.independent_flags + [
            "--crate-type=rlib", "--crate-name=const_owner", "--emit=obj",
            "--extern", "ffi=" + str(self.native_ffi), source, "-o", obj])
        _, size, _, data = elf_symbol(obj, b"CONST_RESULTS")
        self.assertEqual(size, 4 * len(expected))
        self.assertEqual(data, struct.pack("=" + "i" * len(expected), *expected))
        self.assertEqual(run(command("NM", "nm") + ["--undefined-only", obj]), b"")


if __name__ == "__main__":
    unittest.main()
