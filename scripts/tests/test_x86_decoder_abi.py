#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""Native decoder ABI parity against unchanged architecture C headers/sources.

All state snapshots include padding and the original pointers, not a translated
field serialization. The C oracle and Rust bridge receive the same input address.
Protected-page checks run in subprocesses so an eager read is a reported failure.
"""

import ctypes
import itertools
import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
STAGES = ("prefixes", "opcode", "modrm", "sib", "displacement", "immediate", "length")
FUNCTIONS = ("insn_init", *("insn_get_" + name for name in STAGES), "insn_decode", "insn_rip_relative",
             "inat_get_opcode_attribute", "inat_get_last_prefix_id", "inat_get_escape_attribute",
             "inat_get_group_attribute", "inat_get_avx_attribute", "inat_get_xop_attribute")
FIELDS = ("prefixes", "rex_prefix", "vex_prefix", "opcode", "modrm", "sib", "displacement",
          "immediate1", "immediate2", "emulate_prefix_size", "attr", "opnd_bytes", "addr_bytes",
          "length", "x86_64", "kaddr", "end_kaddr", "next_byte")
RUST_FIELDS = ("prefixes", "rex_prefix", "__bindgen_anon_1", "opcode", "modrm", "sib", "displacement",
               "__bindgen_anon_2", "__bindgen_anon_3", *FIELDS[9:])
SAMPLES = tuple(bytes.fromhex(value) for value in (
    "90", "0f0b", "0f0b78656e4889c0", "0f0b6b766d6689c0", "f3666790",
    "666767f3f2f0656490", "666666666690", "4867c5f877", "c5f877",
    "c4e17d6f042400112233", "62f17d487f0424", "d508b81122334455667788",
    "d580100011223344", "8fe8789011223344", "678b0424", "678b063412",
    "488b05ffffffff", "48a1ffffffffffffffff", "66a1ffffffff", "9affffffffff",
    "669affffffff", "c8ffff80", "f7c011223344", "1f", "8ed0", "b880ffffff",
    "e900000080", "ea112233445566", "8b848d11223344", "c7051122334488776655"))


def run(command, **kwargs):
    result = subprocess.run(command, capture_output=True, **kwargs)
    if result.returncode:
        raise RuntimeError(shlex.join(map(str, command)) + "\n" +
                           result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result.stdout


def c_environment(work):
    """Use original architecture declarations, tools' host portability headers."""
    include = work / "include"
    (include / "asm").mkdir(parents=True)
    for header in ("insn.h", "inat.h", "inat_types.h", "emulate_prefix.h"):
        (include / "asm" / header).write_text('#include "' + str(ROOT / "arch/x86/include/asm" / header) + '"\n')
    tables = run([*shlex.split(os.environ.get("AWK", "awk")), "-f",
                  ROOT / "arch/x86/tools/gen-insn-attr-x86.awk", ROOT / "arch/x86/lib/x86-opcode-map.txt"])
    (work / "inat-tables.c").write_bytes(tables)
    return ["-I" + str(path) for path in (include, work, ROOT / "tools/include", ROOT / "include/uapi",
                                         ROOT / "arch/x86/include/uapi")]


C_HELPER = r'''
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <asm/insn.h>
@DECLARATIONS@

size_t abi_layout(unsigned index) {
    const size_t values[] = {sizeof(struct insn), _Alignof(struct insn),
        sizeof(struct insn_field), _Alignof(struct insn_field), sizeof(insn_attr_t),
        sizeof(insn_byte_t), sizeof(insn_value_t), sizeof(enum insn_mode),
        @OFFSETS@,
        offsetof(struct insn_field, value), offsetof(struct insn_field, bytes),
        offsetof(struct insn_field, got), offsetof(struct insn_field, nbytes)};
    return index < sizeof(values) / sizeof(values[0]) ? values[index] : (size_t)-1;
}
void abi_init(unsigned rust, struct insn *state, const void *input, int length, int mode) {
    (rust ? insn_init : c_insn_init)(state, input, length, mode);
}
int abi_decode(unsigned rust, struct insn *state, const void *input, int length, unsigned mode) {
    return (rust ? insn_decode : c_insn_decode)(state, input, length, (enum insn_mode)mode);
}
int abi_stage(unsigned rust, unsigned stage, struct insn *state) {
    switch (stage) {
    @STAGES@
    case 8: return (rust ? insn_rip_relative : c_insn_rip_relative)(state);
    default: return 0;
    }
}
unsigned abi_attribute(unsigned rust, unsigned operation, unsigned first, int second, unsigned third) {
    switch (operation) {
    case 0: return (rust ? inat_get_opcode_attribute : c_inat_get_opcode_attribute)(first);
    case 1: return (rust ? inat_get_last_prefix_id : c_inat_get_last_prefix_id)(first);
    case 2: return (rust ? inat_get_escape_attribute : c_inat_get_escape_attribute)(first, second, third);
    case 3: return (rust ? inat_get_group_attribute : c_inat_get_group_attribute)(first, second, third);
    case 4: return (rust ? inat_get_avx_attribute : c_inat_get_avx_attribute)(first, second, third);
    case 5: return (rust ? inat_get_xop_attribute : c_inat_get_xop_attribute)(first, second);
    default: abort();
    }
}
'''

RUST_WRAPPER = r'''
//! Exercise the production native bridge with audited generated C layouts.
#![no_std]
// Host-only link runtime for prebuilt core unwind/personality references.
extern crate std;
extern crate self as kernel;
#[allow(dead_code, missing_docs, non_camel_case_types, non_upper_case_globals)]
#[path = "@BINDINGS@"]
pub mod bindings;
#[path = "@SOURCE@"]
mod production;
pub use production::*;
/// Independent layout snapshot checked against actual architecture C headers.
#[no_mangle]
pub extern "C" fn abi_rust_layout(index: u32) -> usize {
    use core::mem::{align_of, offset_of, size_of};
    use bindings::{insn, insn_field, insn_mode};
    let values = [size_of::<insn>(), align_of::<insn>(), size_of::<insn_field>(), align_of::<insn_field>(),
        size_of::<bindings::insn_attr_t>(), size_of::<bindings::insn_byte_t>(),
        size_of::<bindings::insn_value_t>(), size_of::<insn_mode>(),
        @OFFSETS@,
        offset_of!(insn_field, __bindgen_anon_1), offset_of!(insn_field, __bindgen_anon_1),
        offset_of!(insn_field, got), offset_of!(insn_field, nbytes)];
    values.get(index as usize).copied().unwrap_or(usize::MAX)
}
'''

GUARD_RUNNER = r'''
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>
#include <asm/insn.h>

int main(int argc, char **argv) {
    if (argc != 5) return 2;
    void *library = dlopen(argv[1], RTLD_NOW);
    if (!library) { fputs(dlerror(), stderr); return 2; }
    void (*init)(unsigned, struct insn *, const void *, int, int) = dlsym(library, "abi_init");
    int (*stage)(unsigned, unsigned, struct insn *) = dlsym(library, "abi_stage");
    if (!init || !stage) return 2;
    unsigned rust = atoi(argv[2]), test = atoi(argv[3]), step = atoi(argv[4]);
    size_t page = sysconf(_SC_PAGESIZE);
    unsigned char *pages = mmap(NULL, page * 3, PROT_NONE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (pages == MAP_FAILED || mprotect(pages + page, page, PROT_READ | PROT_WRITE)) return 2;
    unsigned char *end = pages + page * 2;
    struct insn state;
    unsigned char *input = end;
    int count = 0, result;
    if (test == 0) {
        // A cached stage must not inspect unrelated pointers or state fields.
        memset(&state, 0xa5, sizeof(state));
        state.kaddr = (void *)1; state.end_kaddr = (void *)2; state.next_byte = end;
        if (step == 8) state.x86_64 = 0;
    } else {
        static const unsigned char code[][8] = {
            {0x90}, {0x66,0x90}, {0x0f,0x0b,0}, {0x0f,0x0b,0x78,0x65,0x6e,0x90},
            {0x48,0x89,0xc0}, {0xc5,0xf8,0x77}, {0x8b,0x44,0x24,0x11},
            {0xb8,0x11,0x22,0x33,0x44}, {0x8b,0x05,0x11,0x22,0x33,0x44}
        };
        // UD2 needs a third byte to rule out the Xen/KVM emulation prefixes.
        static const unsigned char lengths[] = {1,2,3,6,3,3,4,5,6};
        if (test <= 9) {
            count = lengths[test - 1]; input -= count; memcpy(input, code[test - 1], count);
            // Deliberately larger logical bound: each stage may read only the
            // bytes it needs, never an eager copy of the entire claimed span.
            init(rust, &state, input, 15, 1);
        } else if (test == 10) {
            init(rust, &state, end, 0, 1);
        } else if (test == 11) {
            // Cached prefix bypass with an inaccessible old base address.
            input -= 1; *input = 0x90; init(rust, &state, input, 1, 1);
            state.kaddr = pages; state.prefixes.got = 255;
        } else return 2;
    }
    result = stage(rust, step, &state);
    // Pointer provenance differs between processes. Normalize only pointer
    // values after the decoder has returned; all other state bytes are exact.
    state.kaddr = (void *)((uintptr_t)state.kaddr - (uintptr_t)input);
    state.end_kaddr = (void *)((uintptr_t)state.end_kaddr - (uintptr_t)input);
    state.next_byte = (void *)((uintptr_t)state.next_byte - (uintptr_t)input);
    if (test == 0) state.kaddr = state.end_kaddr = NULL;
    if (test == 11) state.kaddr = NULL;
    fwrite(&result, sizeof(result), 1, stdout);
    fwrite(&state, sizeof(state), 1, stdout);
    return ferror(stdout);
}
'''

ALIAS_RUNNER = r'''
#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <asm/insn.h>
int main(int argc, char **argv) {
    if (argc != 3) return 2;
    void *library = dlopen(argv[1], RTLD_NOW);
    if (!library) { fputs(dlerror(), stderr); return 2; }
    void (*init)(unsigned, struct insn *, const void *, int, int) = dlsym(library, "abi_init");
    int (*decode)(unsigned, struct insn *, const void *, int, unsigned) = dlsym(library, "abi_decode");
    int (*stage)(unsigned, unsigned, struct insn *) = dlsym(library, "abi_stage");
    if (!init || !decode || !stage) return 2;
    unsigned rust = atoi(argv[2]);
    unsigned char record[40];
    while (fread(record, 1, sizeof(record), stdin) == sizeof(record)) {
        struct insn state;
        unsigned offset = record[1], length = record[4];
        if (length > 15 || offset + length > offsetof(struct insn, kaddr)) return 2;
        unsigned char *input = (unsigned char *)&state + offset;
        memset(&state, 0xa5, sizeof(state));
        int result;
        if (record[3]) {
            memcpy(input, record + 8, length);
            result = decode(rust, &state, input, length, record[2]);
        } else {
            init(rust, &state, input, length, record[2]);
            memcpy(input, record + 8, length);
            result = stage(rust, record[0], &state);
        }
        state.kaddr = (void *)((uintptr_t)state.kaddr - (uintptr_t)&state);
        state.end_kaddr = (void *)((uintptr_t)state.end_kaddr - (uintptr_t)&state);
        state.next_byte = (void *)((uintptr_t)state.next_byte - (uintptr_t)&state);
        fwrite(&result, sizeof(result), 1, stdout);
        fwrite(&state, sizeof(state), 1, stdout);
    }
    return ferror(stdin) || ferror(stdout);
}
'''


def build_abi_tools(work, kernel64):
    work.mkdir()
    includes = c_environment(work)
    cc = shlex.split(os.environ.get("HOSTCC", "cc"))
    cflags = ["-O2", "-fPIC", "-fno-strict-aliasing", *includes]
    if kernel64:
        cflags.append("-DCONFIG_X86_64=1")
    rename = ["-D" + name + "=c_" + name for name in FUNCTIONS]
    objects = []
    for name in ("insn", "inat"):
        output = work / (name + "-c.o")
        run([*cc, *cflags, *rename, "-c", ROOT / "arch/x86/lib" / (name + ".c"), "-o", output])
        objects.append(output)
    helper = work / "helper.c"
    helper.write_text(C_HELPER.replace("@DECLARATIONS@", "\n".join(
        "extern __typeof__(" + name + ") c_" + name + ";" for name in FUNCTIONS)).replace(
        "@OFFSETS@", ",".join("offsetof(struct insn, " + field + ")" for field in FIELDS)).replace(
        "@STAGES@", "\n".join(f"case {index}: return (rust ? insn_get_{name} : c_insn_get_{name})(state);"
                              for index, name in enumerate(STAGES, 1))))
    run([*cc, *cflags, "-c", helper, "-o", work / "helper.o"])
    wrapper = work / "bridge.rs"
    wrapper.write_text(RUST_WRAPPER.replace("@BINDINGS@", str(ROOT / "scripts/tests/x86_decoder_abi_bindings.rs"))
                       .replace("@SOURCE@", str(ROOT / "arch/x86/lib/insn_rust.rs"))
                       .replace("@OFFSETS@", ",".join("offset_of!(insn, " + name + ")" for name in RUST_FIELDS)))
    archive = work / "bridge.a"
    flags = ["--cfg", "CONFIG_RUST"] + (["--cfg", "CONFIG_X86_64"] if kernel64 else [])
    run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "--crate-type=staticlib",
         "-Cpanic=abort", "-Copt-level=2", "-Coverflow-checks=yes", "-Dwarnings", "-Dunsafe-op-in-unsafe-fn",
         "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms", *flags, wrapper, "-o", archive])
    library = work / "decoder.so"
    run([*cc, "-shared", "-Wl,-Bsymbolic", *objects, work / "helper.o", archive,
         "-ldl", "-lpthread", "-lm", "-o", library])
    guard = work / "guard.c"
    guard.write_text(GUARD_RUNNER)
    runner = work / "guard"
    run([*cc, *cflags, guard, "-ldl", "-o", runner])
    alias_source = work / "alias.c"
    alias_source.write_text(ALIAS_RUNNER)
    alias = work / "alias"
    run([*cc, *cflags, alias_source, "-ldl", "-o", alias])
    return library, runner, alias


class X86DecoderAbiTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="x86-decoder-abi-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.tools = [build_abi_tools(cls.work / str(mode), mode) for mode in (False, True)]
        cls.libraries = [ctypes.CDLL(str(paths[0])) for paths in cls.tools]
        for library in cls.libraries:
            library.abi_layout.argtypes = library.abi_rust_layout.argtypes = [ctypes.c_uint]
            library.abi_layout.restype = library.abi_rust_layout.restype = ctypes.c_size_t
            library.abi_init.argtypes = [ctypes.c_uint, ctypes.c_void_p, ctypes.c_void_p, ctypes.c_int, ctypes.c_int]
            library.abi_init.restype = None
            library.abi_decode.argtypes = [ctypes.c_uint, ctypes.c_void_p, ctypes.c_void_p, ctypes.c_int, ctypes.c_uint]
            library.abi_decode.restype = ctypes.c_int
            library.abi_stage.argtypes = [ctypes.c_uint, ctypes.c_uint, ctypes.c_void_p]
            library.abi_stage.restype = ctypes.c_int
            library.abi_attribute.argtypes = [ctypes.c_uint, ctypes.c_uint, ctypes.c_uint, ctypes.c_int, ctypes.c_uint]
            library.abi_attribute.restype = ctypes.c_uint
        cls.size = cls.libraries[0].abi_layout(0)
        cls.offsets = {field: cls.libraries[0].abi_layout(8 + index) for index, field in enumerate(FIELDS)}
        cls.field_size = cls.libraries[0].abi_layout(2)
        cls.got = cls.libraries[0].abi_layout(8 + len(FIELDS) + 2)

    def states(self, library, code, length=None, mode=1):
        source = ctypes.create_string_buffer(code.ljust(96, b"\xa5"))
        pointer = ctypes.addressof(source) + 32
        ctypes.memmove(pointer, code, len(code))
        states = [ctypes.create_string_buffer(b"\xa5" * self.size, self.size) for _ in range(2)]
        for rust, state in enumerate(states):
            library.abi_init(rust, state, pointer, len(code) if length is None else length, mode)
        self.assertEqual(states[0].raw, states[1].raw, ("init", code.hex(), length, mode))
        return source, states

    def stage(self, library, states, stage, context):
        status = [library.abi_stage(rust, stage, state) for rust, state in enumerate(states)]
        self.assertEqual(status[0], status[1], context)
        self.assertEqual(states[0].raw, states[1].raw, context)

    def padding(self, states, byte):
        # Each field has value/bytes at offset zero and got/nbytes next. Infer
        # padding from the real C layout instead of assuming its total size.
        for state in states:
            for field in FIELDS[:9]:
                start = self.offsets[field] + self.got + 2
                ctypes.memset(ctypes.addressof(state) + start, byte, self.field_size - self.got - 2)
            start = self.offsets["x86_64"] + 1
            ctypes.memset(ctypes.addressof(state) + start, byte, self.offsets["kaddr"] - start)

    def test_all_c_and_generated_binding_sizes_offsets_and_exported_signatures(self):
        for library in self.libraries:
            for index in range(8 + len(FIELDS) + 4):
                self.assertEqual(library.abi_layout(index), library.abi_rust_layout(index), index)
            for name in FUNCTIONS:
                self.assertTrue(getattr(library, name))

    def test_init_clears_padding_clamps_length_and_preserves_raw_mode_truncation(self):
        for library, length, mode in itertools.product(self.libraries, (-32, -1, 0, 1, 14, 15, 16, 32, 2**31 - 1),
                                                       (-2**31, -256, -255, -1, 0, 1, 2, 127, 128, 255, 256, 257, 2**31 - 1)):
            self.states(library, b"\x90" * 32, length, mode)

    def test_per_stage_whole_state_padding_and_repeated_partial_errors(self):
        for library, code, stage, mode in itertools.product(self.libraries, SAMPLES, range(1, 9), (0, 1)):
            for length in range(min(16, len(code) + 1)):
                source, states = self.states(library, code, length, mode)
                self.padding(states, 0x5a)
                context = (code.hex(), length, stage, mode)
                self.stage(library, states, stage, context)
                self.stage(library, states, stage, ("retry", *context))
                del source

    def test_sequential_stages_manual_widths_and_non_boolean_cached_fields(self):
        for library, code, operand, address in itertools.product(self.libraries, SAMPLES,
                                                                  (2, 4, 8, 3), (2, 4, 8, 3)):
            source, states = self.states(library, code, mode=0)
            for state in states:
                state[self.offsets["opnd_bytes"]] = bytes([operand])
                state[self.offsets["addr_bytes"]] = bytes([address])
            self.padding(states, 0xab)
            for stage, field in enumerate(STAGES, 1):
                context = (code.hex(), operand, address, stage)
                self.stage(library, states, stage, context)
                if stage < 7:
                    offset = self.offsets["immediate1" if field == "immediate" else field] + self.got
                    for state in states:
                        if state[offset] != b"\0":
                            state[offset] = b"\xfe"
                self.stage(library, states, stage, ("cached", *context))
            del source

    def test_decode_modes_and_random_c_abi_states(self):
        rng = random.Random(0xa81dec0de)
        for library in self.libraries:
            for index in range(12000):
                code = rng.randbytes(rng.randrange(33))
                source, states = self.states(library, code, mode=rng.choice((0, 1, 2, 255, 256)))
                if index % 2:
                    self.padding(states, rng.randrange(256))
                    stage = rng.randrange(1, 9)
                    self.stage(library, states, stage, (index, code.hex(), stage))
                else:
                    mode = rng.choice((0, 1, 2, 3, 255, 2**32 - 1))
                    result = [library.abi_decode(rust, state, ctypes.addressof(source) + 32, len(code), mode)
                              for rust, state in enumerate(states)]
                    self.assertEqual(result[0], result[1], (index, code.hex(), mode))
                    self.assertEqual(states[0].raw, states[1].raw, (index, code.hex(), mode))

    def test_prefix_processing_rewrites_only_the_flags_c_assigns(self):
        for library, code, cached in itertools.product(self.libraries, SAMPLES, (2, 128, 255)):
            source, states = self.states(library, code)
            self.padding(states, 0x91)
            for state in states:
                for field in ("rex_prefix", "vex_prefix"):
                    state[self.offsets[field] + self.got] = bytes([cached])
            for stage in (1, 2, 3, 7):
                self.stage(library, states, stage, (code.hex(), cached, stage))
            del source

    def test_all_six_attribute_entrypoints_retain_c_argument_widths(self):
        rng = random.Random(0x1a7ab1)
        for library in self.libraries:
            cases = [(operation, opcode, 0, 0) for operation in (0, 1) for opcode in range(256)]
            cases += [(2, opcode, prefix, escape << 5) for opcode in range(256)
                      for prefix in range(4) for escape in range(4)]
            cases += [(3, modrm, prefix, (group << 7) | (0x0d3f8011 & ~(31 << 7)))
                      for modrm in range(256) for prefix in range(4) for group in range(32)]
            cases += [(4, rng.randrange(256), rng.randrange(256), rng.randrange(256)) for _ in range(8000)]
            cases += [(5, opcode, map_id, 0) for opcode in range(256) for map_id in range(256)]
            for case in cases:
                result = [library.abi_attribute(rust, *case) for rust in range(2)]
                self.assertEqual(result[0], result[1], case)

    def test_protected_pages_cached_stages_and_lazy_input_reads(self):
        for library, runner, _ in self.tools:
            for case, stage in itertools.product(range(12), range(1, 9)):
                results = [subprocess.run([runner, library, str(rust), str(case), str(stage)],
                                          capture_output=True, timeout=5) for rust in range(2)]
                for rust, result in enumerate(results):
                    self.assertEqual(result.returncode, 0, (library, rust, case, stage, result.stderr))
                self.assertEqual(results[0].stdout, results[1].stdout, (library, case, stage))
                self.assertEqual(results[0].stderr, results[1].stderr)

    def test_rebased_cursor_before_instruction_base_still_reads_actual_cursor(self):
        for library, code, distance, stage in itertools.product(self.libraries, SAMPLES, (1, 2, 16, 32), range(1, 9)):
            source, states = self.states(library, code)
            for state in states:
                ctypes.c_void_p.from_buffer(state, self.offsets["kaddr"]).value = ctypes.addressof(source) + 32 + distance
            self.padding(states, 0x61)
            self.stage(library, states, stage, (code.hex(), distance, stage))
            del source

    def test_instruction_bytes_inside_native_state_preserve_c_write_read_order(self):
        cases = []
        samples = (*SAMPLES, b"\x66\xff\xff\xff\xff\xff\x90", b"\x67" * 14 + b"\x90")
        for code, offset, mode, stage in itertools.product(samples, range(self.offsets["kaddr"] - 15), (0, 1), range(1, 9)):
            code = code[:15]
            cases.append(bytes((stage, offset, mode, 0, len(code), 0, 0, 0)) + code.ljust(32, b"\0"))
        for offset, mode in itertools.product(range(self.offsets["kaddr"] - 15), (0, 1, 2)):
            cases.append(bytes((0, offset, mode, 1, 15, 0, 0, 0)) + b"\x90" * 15 + bytes(17))
        payload = b"".join(cases)
        stride = self.size + ctypes.sizeof(ctypes.c_int)
        for library, _, runner in self.tools:
            results = [subprocess.run([runner, library, str(rust)], input=payload,
                                      capture_output=True, timeout=30) for rust in range(2)]
            for rust, result in enumerate(results):
                self.assertEqual(result.returncode, 0, (library, rust, result.stderr))
                self.assertEqual(len(result.stdout), len(cases) * stride)
            if results[0].stdout != results[1].stdout:
                index = next(i for i, pair in enumerate(zip(results[0].stdout, results[1].stdout)) if pair[0] != pair[1]) // stride
                self.assertEqual(results[0].stdout[index * stride:(index + 1) * stride],
                                 results[1].stdout[index * stride:(index + 1) * stride], cases[index].hex())


if __name__ == "__main__":
    unittest.main()
