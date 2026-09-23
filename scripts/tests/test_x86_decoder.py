# SPDX-License-Identifier: GPL-2.0-only
"""Per-stage x86 decoder parity, including partial fields on decoding errors."""

import itertools
import os
import random
import shlex
import struct
import subprocess
import unittest

from x86_decoder_test_support import ROOT, decoder_tools, run_build


def record(code, mode=1, stage=8, size=None, operand=0, address=0, repeat=False):
    if size is None:
        size = len(code)
    return bytes((mode, stage, size, operand, address, int(repeat), 0, 0)) + code.ljust(32, b"\0")


class X86DecoderTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.work, cls.c, cls.rust, _ = decoder_tools()

    def compare(self, cases):
        iterator = iter(cases)
        while batch := list(itertools.islice(iterator, 8192)):
            data = b"".join(batch)
            results = [subprocess.run([tool], input=data, capture_output=True, timeout=20)
                       for tool in (self.c, self.rust)]
            for result in results:
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(result.stderr, b"")
                self.assertEqual(len(result.stdout), len(batch) * 168)
            c, rust = (result.stdout for result in results)
            if c != rust:
                offset = next(i for i, (a, b) in enumerate(zip(c, rust)) if a != b) // 168
                self.assertEqual(struct.unpack_from("<42I", rust, offset * 168),
                                 struct.unpack_from("<42I", c, offset * 168),
                                 "record=" + batch[offset].hex())

    def test_all_two_byte_sequences(self):
        self.compare(record(bytes((first, second)) + bytes([0xa5]) * 13, mode)
                     for mode in (0, 1) for first in range(256) for second in range(256))

    def test_prefixes_truncation_and_every_decoding_stage(self):
        samples = [bytes.fromhex(value) for value in (
            "90", "0f0b", "0f0b78656e4889c0", "0f0b6b766d6689c0",
            "666767f3f2f0656490", "666666666690", "4867c5f877", "c5f877",
            "c4e17d6f042400112233", "62f17d487f0424", "d508b81122334455667788",
            "d580100011223344", "8fe8789011223344", "678b0424", "678b063412",
            "488b05ffffffff", "48a1ffffffffffffffff", "66a1ffffffff",
            "9affffffffff", "669affffffff", "c8ffff80", "f7c011223344", "1f", "8ed0")]
        samples += [bytes([p]) * n for p in (0x66, 0x67, 0x40, 0xf3) for n in (1, 4, 14, 15, 16, 30)]
        self.compare(record(code, mode, stage, length, repeat=repeat)
                     for mode in (0, 1, 2, 3) for code in samples for length in range(len(code) + 1)
                     for stage in range(11) for repeat in (False, True))

    def test_vex_evex_xop_and_rex2_maps(self):
        def cases():
            for mode in (0, 1):
                for map_id in range(32):
                    for prefix in range(4):
                        for opcode in range(256):
                            for lead in (b"\xc4", b"\x62", b"\x8f"):
                                code = lead + bytes((0xe0 | map_id, 0x78 | prefix))
                                if lead == b"\x62":
                                    code += b"\x48"
                                yield record((code + bytes((opcode,)) + b"\x24\x25" * 8)[:15], mode)
                for rex2 in range(256):
                    for opcode in range(256):
                        yield record(bytes((0xd5, rex2, opcode)) + b"\xa5" * 12, mode)
        self.compare(cases())

    def test_random_buffers_and_manually_selected_widths(self):
        rng = random.Random(0x1a7dec0de)
        self.compare(record(rng.randbytes(rng.randrange(33)), rng.randrange(4), rng.randrange(11),
                            operand=rng.choice((0, 0, 2, 4, 8, 3)), address=rng.choice((0, 0, 2, 4, 8, 3)),
                            repeat=bool(rng.getrandbits(1))) for _ in range(40000))

    def test_core_only_module_compiles_without_std_or_allocator(self):
        source = self.work / "no_std.rs"
        source.write_text('''//! Standalone core-only instruction decoder smoke check.
#![no_std]
#[path = "''' + str(ROOT / "arch/x86/lib/insn.rs") + '''"]
#[allow(dead_code)]
mod decoder;
/// Decode a bounded instruction without allocation.
pub fn length(bytes: &[u8]) -> Result<u8, i32> {
    let mut insn = decoder::Instruction::new(bytes, decoder::Mode::Bits64);
    insn.decode().map_err(|err| err as i32)?;
    Ok(insn.length)
}
''')
        run_build(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "--crate-type=rlib", "-Dwarnings", "-Wmissing-docs",
            str(source), "-o", str(self.work / "decoder.rlib")])

    def test_header_field_extractors(self):
        names = ("MODRM_MOD", "MODRM_REG", "MODRM_RM", "SIB_SCALE", "SIB_INDEX", "SIB_BASE",
                 "REX2_M", "REX2_R", "REX2_X", "REX2_B", "REX_W", "REX_R", "REX_X", "REX_B",
                 "VEX_W", "VEX_R", "VEX_X", "VEX_B", "VEX_L", "EVEX_M", "VEX3_M", "VEX_V",
                 "VEX_P", "XOP_R", "XOP_X", "XOP_B", "XOP_M", "XOP_W", "XOP_V", "XOP_L", "XOP_P")
        c_source, rust_source = self.work / "bits.c", self.work / "bits.rs"
        c_source.write_text('#include <stdio.h>\n#include <asm/insn.h>\nint main(void) {\n'
                            'for (int i=0;i<256;i++) {\n' +
                            ''.join('putchar(X86_' + name + '(i));\n' for name in names) +
                            '} putchar(X86_VEX2_M); putchar(X86_VEX_M_MAX); '
                            'putchar(X86_XOP_M_MIN); putchar(X86_XOP_M_MAX); return 0; }\n')
        rust_source.write_text('//! Exhaustive instruction field-mask checks.\n'
                               '#[path="' + str(ROOT / "tools/arch/x86/include/asm/insn_header.rs") + '"]\n'
                               '#[allow(dead_code)] mod decoder;\nuse std::io::Write;\n'
                               'fn main() { let mut output=Vec::new(); for i in 0..=255u8 {\n' +
                               ''.join('output.push(decoder::' + name.lower() + '(i));\n' for name in names) +
                               '} output.extend([decoder::X86_VEX2_M,decoder::X86_VEX_M_MAX,'
                               'decoder::X86_XOP_M_MIN,decoder::X86_XOP_M_MAX]);'
                               'std::io::stdout().write_all(&output).unwrap(); }\n')
        c, rust = self.work / "bits-c", self.work / "bits-rust"
        run_build(shlex.split(os.environ.get("HOSTCC", "cc")) + ["-O2"] + decoder_tools()[3] +
                  [str(c_source), "-o", str(c)])
        run_build(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) +
                  ["--edition=2021", "-Dwarnings", "-Wmissing-docs", str(rust_source), "-o", str(rust)])
        self.assertEqual(run_build([str(c)]), run_build([str(rust)]))


if __name__ == "__main__":
    unittest.main()
