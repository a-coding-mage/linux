# SPDX-License-Identifier: GPL-2.0-only
"""Build the unchanged x86 C decoder and a per-field Rust differential harness."""

import atexit
import functools
import os
from pathlib import Path
import shlex
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]

C_HARNESS = r'''
#include <stdio.h>
#include <stdint.h>
#include <asm/insn.h>
#include <inat.c>
#include <insn.c>
static void put(uint32_t v) {
    unsigned char b[4] = {v, v >> 8, v >> 16, v >> 24};
    fwrite(b, 1, 4, stdout);
}
static int stage(struct insn *i, unsigned char step) {
    int ret;
    switch (step) {
    case 0: return 0;
    case 1: return insn_get_prefixes(i);
    case 2: return insn_get_opcode(i);
    case 3: return insn_get_modrm(i);
    case 4: return insn_get_sib(i);
    case 5: return insn_get_displacement(i);
    case 6: return insn_get_immediate(i);
    case 7: return insn_get_length(i);
    case 8:
        ret = insn_get_length(i);
        return ret ? ret : insn_complete(i) ? 0 : -EINVAL;
    case 9: return insn_rip_relative(i);
    default:
        ret = !!insn_is_rex2(i);
        ret |= !!insn_is_avx_or_xop(i) << 1;
        ret |= !!insn_is_evex(i) << 2;
        ret |= !!insn_is_xop(i) << 3;
        ret |= !!insn_has_emulate_prefix(i) << 4;
        ret |= !!insn_masking_exception(i) << 5;
        return ret | insn_last_prefix_id(i) << 8;
    }
}
int main(void) {
    unsigned char r[40];
    while (fread(r, 1, sizeof(r), stdin) == sizeof(r)) {
        struct insn i;
        struct insn_field *fields[] = {&i.prefixes, &i.rex_prefix, &i.vex_prefix,
            &i.opcode, &i.modrm, &i.sib, &i.displacement, &i.immediate1, &i.immediate2};
        int ret, n;
        insn_init(&i, r + 8, r[2], r[0] & 1);
        if (r[3]) i.opnd_bytes = r[3];
        if (r[4]) i.addr_bytes = r[4];
        ret = stage(&i, r[1]);
        if (r[5]) ret = stage(&i, r[1]);
        put(ret); put(i.attr); put(i.next_byte - i.kaddr); put(i.emulate_prefix_size);
        put(i.opnd_bytes); put(i.addr_bytes); put(i.length); put(i.x86_64);
        for (n = 0; n < 9; n++) {
            put(fields[n]->value); put(fields[n]->got); put(fields[n]->nbytes);
        }
        put(insn_offset_rex_prefix(&i)); put(insn_offset_vex_prefix(&i));
        put(insn_offset_opcode(&i)); put(insn_offset_modrm(&i)); put(insn_offset_sib(&i));
        put(insn_offset_displacement(&i)); put(insn_offset_immediate(&i));
    }
    return ferror(stdin) || ferror(stdout);
}
'''

RUST_HARNESS = r'''
//! Compare bounded x86 decoder fields against the original implementation.
#[path = "SOURCE"]
#[allow(dead_code)]
mod decoder;
use decoder::{DecodeResult, Instruction, Mode};
use std::io::{self, Read, Write};
fn status(result: DecodeResult) -> i32 { result.err().map_or(0, |err| err as i32) }
fn stage(i: &mut Instruction<'_>, step: u8) -> i32 {
    match step {
        0 => 0,
        1 => status(i.get_prefixes()), 2 => status(i.get_opcode()),
        3 => status(i.get_modrm()), 4 => status(i.get_sib()),
        5 => status(i.get_displacement()), 6 => status(i.get_immediate()),
        7 => status(i.get_length()), 8 => status(i.decode()),
        9 => i.rip_relative() as i32,
        _ => {
            let mut ret = i.is_rex2() as i32;
            ret |= (i.is_avx_or_xop() as i32) << 1;
            ret |= (i.is_evex() as i32) << 2;
            ret |= (i.is_xop() as i32) << 3;
            ret |= (i.has_emulate_prefix() as i32) << 4;
            ret |= (i.masking_exception() as i32) << 5;
            ret | (i.last_prefix_id() as i32) << 8
        }
    }
}
fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut output = Vec::new();
    for r in input.chunks_exact(40) {
        let mode = match r[0] { 0 => Mode::Bits32, 1 => Mode::Bits64,
            other => Mode::Kernel { x86_64: other & 1 != 0 } };
        let mut i = Instruction::new(&r[8..8 + r[2] as usize], mode);
        if r[3] != 0 { i.opnd_bytes = r[3]; }
        if r[4] != 0 { i.addr_bytes = r[4]; }
        let mut ret = stage(&mut i, r[1]);
        if r[5] != 0 { ret = stage(&mut i, r[1]); }
        let mut put = |v: u32| output.extend_from_slice(&v.to_le_bytes());
        for value in [ret as u32, i.attr, i.next as u32, i.emulate_prefix_size as u32,
                      i.opnd_bytes as u32, i.addr_bytes as u32, i.length as u32, i.x86_64 as u32] { put(value); }
        for field in [&i.prefixes, &i.rex_prefix, &i.vex_prefix, &i.opcode, &i.modrm,
                      &i.sib, &i.displacement, &i.immediate1, &i.immediate2] {
            put(field.value() as u32); put(field.got as u32); put(field.nbytes as u32);
        }
        for offset in [i.offset_rex_prefix(), i.offset_vex_prefix(), i.offset_opcode(),
                       i.offset_modrm(), i.offset_sib(), i.offset_displacement(), i.offset_immediate()] { put(offset as u32); }
    }
    io::stdout().write_all(&output).unwrap();
}
'''


def run_build(command, **kwargs):
    result = subprocess.run(command, capture_output=True, **kwargs)
    if result.returncode:
        raise RuntimeError(shlex.join(map(str, command)) + "\n" +
                           result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result.stdout


@functools.lru_cache(maxsize=1)
def decoder_tools():
    temporary = tempfile.TemporaryDirectory(prefix="x86-decoder-")
    atexit.register(temporary.cleanup)
    work = Path(temporary.name)
    cc = shlex.split(os.environ.get("HOSTCC", "cc"))
    rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
    tables = run_build(shlex.split(os.environ.get("AWK", "awk")) + [
        "-f", str(ROOT / "arch/x86/tools/gen-insn-attr-x86.awk"),
        str(ROOT / "arch/x86/lib/x86-opcode-map.txt")])
    (work / "inat-tables.c").write_bytes(tables)
    c_source, rust_source = work / "harness.c", work / "harness.rs"
    c_source.write_text(C_HARNESS)
    rust_source.write_text(RUST_HARNESS.replace("SOURCE", str(ROOT / "arch/x86/lib/insn.rs")))
    includes = ["-I" + str(path) for path in (
        ROOT / "tools/arch/x86/lib", ROOT / "tools/arch/x86/include", work,
        ROOT / "tools/include", ROOT / "include/uapi", ROOT / "arch/x86/include/uapi")]
    c, rust = work / "decoder-c", work / "decoder-rust"
    run_build(cc + ["-O2"] + includes + [str(c_source), "-o", str(c)])
    run_build(rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
                      "-Wunreachable-pub", "-Wrust-2018-idioms", str(rust_source), "-o", str(rust)])
    return work, c, rust, includes
