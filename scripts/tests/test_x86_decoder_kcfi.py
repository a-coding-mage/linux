#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""Real C-enum KCFI identity and indirect calls through the native decoder.

Bindgen reads the unchanged architecture header with the production enum
parameters. The ordinary ABI fixture remains separately checked against C.
All generated C/Rust files and executables live in a temporary directory.
"""

import os
from pathlib import Path
import random
import re
import resource
import shlex
import shutil
import signal
import struct
import subprocess
import tempfile
import unittest

from test_x86_decoder_abi import FUNCTIONS, ROOT, SAMPLES, c_environment, run
from check_x86_decoder_kernel import INDIRECT_SOURCE


DRIVER = r'''
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <asm/insn.h>

/* A volatile load prevents direct-call devirtualization, including under O2. */
static int (* volatile decode)(struct insn *, const void *, int, enum insn_mode) = insn_decode;
int main(void)
{
    struct { uint32_t mode; int32_t length; unsigned char bytes[16]; } record;
    while (fread(&record, 1, sizeof(record), stdin) == sizeof(record)) {
        struct insn state;
        int result = decode(&state, record.bytes, record.length, (enum insn_mode)record.mode);
        /* Only normalize addresses; retain every value, cached flag and pad. */
        state.kaddr = state.end_kaddr = state.next_byte = NULL;
        if (fwrite(&result, sizeof(result), 1, stdout) != 1 ||
            fwrite(&state, sizeof(state), 1, stdout) != 1)
            return 2;
    }
    return ferror(stdin) || ferror(stdout);
}
'''


def kcfi_ids(path):
    """Read the compiler-emitted LLVM KCFI metadata, not guessed type hashes."""
    source = path.read_text()
    metadata = {number: int(value) & 0xffffffff for number, value in
                re.findall(r"(?m)^!(\d+) = !\{i32 (-?\d+)\}$", source)}
    return {name: metadata[number] for name, number in re.findall(
        r"(?m)^define [^\n]*?@([A-Za-z_][A-Za-z_0-9]*)\([^\n]*!kcfi_type !(\d+)", source)}


class X86DecoderKcfiTests(unittest.TestCase):
    def setUp(self):
        for variable, fallback in (("BINDGEN", "bindgen"), ("CLANG", "clang")):
            command = shlex.split(os.environ.get(variable, fallback))
            if not command or not shutil.which(command[0]):
                if variable in os.environ:
                    self.fail("explicit " + variable + " is unavailable")
                self.skipTest(variable + " required for real generated-binding KCFI proof")
            setattr(self, variable.lower(), command)
        temporary = tempfile.TemporaryDirectory(prefix="x86-decoder-kcfi-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.includes = c_environment(self.work)
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.environment = {**os.environ, "RUSTC_BOOTSTRAP": "1"}
        self.cflags = [*self.includes, "-DCONFIG_X86_64", "-O2", "-fsanitize=kcfi",
                       "-fsanitize-cfi-icall-experimental-normalize-integers"]
        self.rustflags = ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]

    def generated_bindings(self):
        parameters = shlex.split((ROOT / "rust/bindgen_parameters").read_text(), comments=True)
        selected = []
        for index, parameter in enumerate(parameters):
            if parameter == "--newtype-enum" and parameters[index + 1] == "insn_mode":
                selected.extend((parameter, "insn_mode"))
            elif parameter.startswith("--with-attribute-custom-enum=insn_mode="):
                selected.append(parameter)
        self.assertEqual(selected, ["--newtype-enum", "insn_mode",
            '--with-attribute-custom-enum=insn_mode=#[cfi_encoding="9insn_mode"]'])
        output = self.work / "bindings.rs"
        run([*self.bindgen, ROOT / "arch/x86/include/asm/insn.h", *selected,
             "--allowlist-function", "(" + "|".join(FUNCTIONS) + ")",
             "--allowlist-type", "insn_mode", "--use-core", "--ctypes-prefix", "ffi",
             "--no-layout-tests", "--no-doc-comments", "--no-debug", ".*", "--rust-target", "1.85",
             "--output", output, "--", *self.includes, "-DCONFIG_X86_64"])
        source = output.read_text()
        self.assertIn('#[cfi_encoding = "9insn_mode"]', source)
        self.assertIn("#[repr(transparent)]", source)
        self.assertIn("pub struct insn_mode(pub ffi::c_uint);", source)
        self.assertNotIn("pub enum insn_mode", source)
        return output

    def rust_owner(self, bindings, optimize, *, plain=False):
        label = "plain" if plain else "native"
        owner = ROOT / "arch/x86/lib/insn_rust.rs"
        if plain:
            # Deliberately restore the old ABI-identical but nominally wrong
            # u32 argument. Actual decoding still uses the production function.
            body = f'''#[path = "{owner}"] mod implementation;
#[export_name = "insn_decode"]
pub unsafe extern "C" fn wrong_insn_decode(state: *mut bindings::insn,
    input: *const core::ffi::c_void, length: i32, mode: u32) -> i32 {{
    // Keep this deliberately wrong entry point distinct from a possible LLVM
    // alias of the correct transparent-newtype entry point.
    let mode = unsafe {{ core::ptr::read_volatile(&mode) }};
    unsafe {{ implementation::insn_decode(state, input, length, bindings::insn_mode(mode)) }}
}}
'''
            # Avoid a duplicate global definition without editing production.
            source = owner.read_text().replace('#[no_mangle]\npub unsafe extern "C" fn insn_decode(',
                '#[export_name = "correct_insn_decode"]\npub unsafe extern "C" fn insn_decode(')
            # The adjacent decoder module must still resolve to the real file.
            source = source.replace('#[path = "insn.rs"]',
                                    '#[path = "' + str(owner.with_name("insn.rs")) + '"]')
            mutant = self.work / "renamed_owner.rs"
            mutant.write_text(source)
            body = body.replace(str(owner), str(mutant))
        else:
            body = f'#[path = "{owner}"] mod implementation;\npub use implementation::*;\n'
        wrapper = self.work / f"{label}-{optimize}.rs"
        wrapper.write_text('''//! Native KCFI test owner with genuine generated C bindings.
#![no_std]
#![feature(cfi_encoding)]
extern crate std;
extern crate self as kernel;
#[allow(dead_code, missing_docs, non_camel_case_types, non_upper_case_globals)]
pub mod bindings {
    use core::ffi;
    include!("''' + str(bindings) + '''");
}
''' + body)
        archive, llvm = wrapper.with_suffix(".a"), wrapper.with_suffix(".ll")
        run([*self.rustc, "--edition=2021", "--crate-type=staticlib", "-Cpanic=abort",
             "-Copt-level=" + optimize, "-Coverflow-checks=yes", "-Dwarnings",
             "-Dunsafe-op-in-unsafe-fn", "--cfg", "CONFIG_RUST", "--cfg", "CONFIG_X86_64",
             *self.rustflags, "--emit=link=" + str(archive), "--emit=llvm-ir=" + str(llvm), wrapper],
            env=self.environment)
        return archive, llvm

    def driver(self, objects, label, checked=True):
        source, output = self.work / "driver.c", self.work / label
        source.write_text(DRIVER)
        flags = self.cflags if checked else [*self.includes, "-DCONFIG_X86_64", "-O2"]
        run([*self.clang, *flags, source, *objects, "-ldl", "-lpthread", "-lm", "-o", output])
        return output

    def test_real_bindgen_normalized_ids_and_volatile_indirect_calls(self):
        bindings = self.generated_bindings()
        original, expected_ids = [], {}
        for name in ("insn", "inat"):
            source = ROOT / "arch/x86/lib" / (name + ".c")
            obj, llvm = self.work / (name + ".o"), self.work / (name + ".ll")
            run([*self.clang, *self.cflags, "-c", source, "-o", obj])
            run([*self.clang, *self.cflags, "-S", "-emit-llvm", source, "-o", llvm])
            original.append(obj)
            expected_ids.update(kcfi_ids(llvm))
        self.assertEqual(set(expected_ids) & set(FUNCTIONS), set(FUNCTIONS))
        data = bytearray()
        for code in SAMPLES:
            for length in range(len(code) + 1):
                for mode in (0, 1, 2, 3, 0x80000000, 0xffffffff):
                    data += struct.pack("<Ii16s", mode, length, code)
        randomizer = random.Random(0x434649)
        for _ in range(8192):
            data += struct.pack("<Ii16s", randomizer.getrandbits(32), randomizer.randrange(17),
                                randomizer.randbytes(16))
        expected = run([self.driver(original, "original")], input=data)
        for optimize in ("0", "2", "s"):
            archive, llvm = self.rust_owner(bindings, optimize)
            actual = kcfi_ids(llvm)
            self.assertEqual({name: actual[name] for name in FUNCTIONS},
                             {name: expected_ids[name] for name in FUNCTIONS})
            self.assertEqual(run([self.driver([archive], "native-" + optimize)], input=data), expected)
        archive, llvm = self.rust_owner(bindings, "2", plain=True)
        self.assertNotEqual(kcfi_ids(llvm)["insn_decode"], expected_ids["insn_decode"])
        self.assertEqual(run([self.driver([archive], "plain-unchecked", checked=False)], input=data), expected)
        checked = self.driver([archive], "plain-checked")
        failure = subprocess.run([checked], input=struct.pack("<Ii16s", 1, 1, b"\x90"),
                                 capture_output=True, timeout=30,
                                 preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        self.assertEqual((failure.returncode, failure.stdout, failure.stderr), (-signal.SIGILL, b"", b""))

    def test_vm_fixture_cases_pass_the_unchanged_c_decoder_with_kcfi(self):
        source = self.work / "guest-cases.c"
        fixture = INDIRECT_SOURCE.replace("@ADDRESS@UL", "(unsigned long)&insn_decode")
        for header in ("init", "module", "printk"):
            fixture = fixture.replace("#include <linux/" + header + ".h>", "")
        source.write_text('''#include <stdio.h>
#include <stdbool.h>
#define __init
#define __exit
#define pr_err(...) fprintf(stderr, __VA_ARGS__)
#define pr_info(...) printf(__VA_ARGS__)
#define module_init(function) int main(void) { return function(); }
#define module_exit(function)
#define MODULE_LICENSE(text)
#define MODULE_DESCRIPTION(text)
''' + fixture)
        output = self.work / "guest-cases"
        run([*self.clang, *self.cflags, source, ROOT / "arch/x86/lib/insn.c",
             ROOT / "arch/x86/lib/inat.c", "-o", output])
        self.assertEqual(run([output]), b"LUPOS_X86_DECODER_INDIRECT_OK cases=54\n")


if __name__ == "__main__":
    unittest.main()
