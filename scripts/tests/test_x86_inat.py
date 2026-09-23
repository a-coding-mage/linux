#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Exhaustive x86 attribute lookup/table parity against original AWK and C."""

import importlib.util
import os
from pathlib import Path
import re
import shlex
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
GENERATOR = ROOT / "arch/x86/tools/gen_inat_tables.py"
SPEC = importlib.util.spec_from_file_location("gen_inat_tables", GENERATOR)
GEN = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GEN)
HEADER = ROOT / "arch/x86/include/asm/inat_header.rs"
MAP = ROOT / "arch/x86/lib/x86-opcode-map.txt"
TABLES = ROOT / "arch/x86/lib/inat_tables.rs"
LOOKUP = ROOT / "arch/x86/lib/inat.rs"
PREDICATES = (
    "inat_is_legacy_prefix", "inat_is_address_size_prefix", "inat_is_operand_size_prefix", "inat_is_rex_prefix",
    "inat_is_rex2_prefix", "inat_is_vex_prefix", "inat_is_evex_prefix", "inat_is_vex3_prefix", "inat_is_xop_prefix",
    "inat_is_escape", "inat_is_group", "inat_has_immediate", "inat_has_modrm", "inat_is_force64",
    "inat_has_second_immediate", "inat_has_moffset", "inat_has_variant", "inat_accept_vex", "inat_accept_xop",
    "inat_must_vex", "inat_must_evex", "inat_evex_scalable", "inat_is_invalid64",
)
EXTRACTORS = ("inat_last_prefix_id", "inat_escape_id", "inat_group_id", "inat_group_common_attribute", "inat_immediate_size")


class X86AttributeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="x86-inat-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        cls.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.c = cls.work / "inat-c"
        cls.rust = cls.work / "inat-rust"
        cls.tools = cls.work / "inat-tools"
        awk = shlex.split(os.environ.get("AWK", "awk"))
        result = subprocess.run(awk + ["-f", str(ROOT / "arch/x86/tools/gen-insn-attr-x86.awk"), str(MAP)], capture_output=True)
        if result.returncode:
            raise RuntimeError(result.stderr)
        (cls.work / "inat-tables.c").write_bytes(result.stdout)
        constants = re.findall(r"pub\(crate\) const (INAT_[A-Z0-9_]+):", HEADER.read_text())
        c_constants = "".join(f"put({name});" for name in constants)
        c_helpers = "".join(f"put(!!{name}(value));" for name in PREDICATES)
        c_helpers += "".join(f"put({name}(value));" for name in EXTRACTORS)
        rust_constants = "".join(f"put(&mut out, {name} as u32);" for name in constants)
        rust_helpers = "".join(f"put(&mut out, {name}(value) as u32);" for name in (*PREDICATES, *EXTRACTORS))
        c_source = f'''#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include "{ROOT / 'arch/x86/lib/inat.c'}"
static void put(uint32_t v) {{ unsigned char bytes[] = {{v, v >> 8, v >> 16, v >> 24}}; fwrite(bytes, 1, 4, stdout); }}
static void table(const insn_attr_t *p, unsigned int size) {{ put(p != NULL); if (p) for (unsigned int i=0;i<size;i++) put(p[i]); }}
int main(int argc, char **argv) {{
    char mode = argc > 1 ? argv[1][0] : 'l';
    if (mode == 't') {{
        table(inat_primary_table, 256);
        for (unsigned i=0;i<4;i++) for (unsigned p=0;p<4;p++) table(inat_escape_tables[i][p],256);
        for (unsigned i=0;i<32;i++) for (unsigned p=0;p<4;p++) table(inat_group_tables[i][p],8);
        for (unsigned i=0;i<32;i++) for (unsigned p=0;p<4;p++) table(inat_avx_tables[i][p],256);
        for (unsigned i=0;i<24;i++) table(inat_xop_tables[i],256);
    }} else if (mode == 'h') {{
        {c_constants}
        for (uint32_t value=0;value<256;value++) {{ put(INAT_MAKE_PREFIX(value)); put(INAT_MAKE_ESCAPE(value)); put(INAT_MAKE_GROUP(value)); put(INAT_MAKE_IMM(value)); }}
        uint32_t state = 0x1234abcd;
        for (unsigned i=0;i<20000;i++) {{ state=state*1664525+1013904223; uint32_t value=i<32?1U<<i:state; {c_helpers} }}
    }} else {{
        for (unsigned op=0;op<256;op++) {{put(inat_get_opcode_attribute(op));put(inat_get_last_prefix_id(op));}}
        for (unsigned e=0;e<4;e++) for (unsigned p=0;p<4;p++) for(unsigned op=0;op<256;op++) put(inat_get_escape_attribute(op,p,INAT_MAKE_ESCAPE(e)));
        for (unsigned g=0;g<32;g++) for (unsigned p=0;p<4;p++) for(unsigned m=0;m<256;m++) put(inat_get_group_attribute(m,p,(g<<INAT_GRP_OFFS)|(0x0d3f8011&~INAT_GRP_MASK)));
        for (unsigned map=0;map<256;map++) for(unsigned p=0;p<8;p++) for(unsigned op=0;op<256;op++) put(inat_get_avx_attribute(op,map,p));
        for (unsigned map=0;map<256;map++) for(unsigned op=0;op<256;op++) put(inat_get_xop_attribute(op,map));
    }}
    return ferror(stdout) != 0;
}}
'''
        cls.rust_source = f'''//! Attribute test harness.
#[path="{LOOKUP}"] mod inat;
use inat::*;
use std::io::Write;
fn put(out: &mut Vec<u8>, value: u32) {{ out.extend_from_slice(&value.to_le_bytes()); }}
mod raw {{
    #[path="{HEADER}"] mod attributes;
    #[path="{TABLES}"] mod tables;
    fn table<const N: usize>(out:&mut Vec<u8>, values:Option<&[u32;N]>) {{
        super::put(out, values.is_some() as u32);
        if let Some(values)=values {{ for &value in values {{super::put(out,value);}} }}
    }}
    pub(super) fn dump(out:&mut Vec<u8>) {{
        table(out,Some(&tables::INAT_PRIMARY_TABLE));
        for row in tables::INAT_ESCAPE_TABLES {{ for value in row {{ table(out,value); }} }}
        for row in tables::INAT_GROUP_TABLES {{ for value in row {{ table(out,value); }} }}
        for row in tables::INAT_AVX_TABLES {{ for value in row {{ table(out,value); }} }}
        for value in tables::INAT_XOP_TABLES {{ table(out,value); }}
    }}
}}
fn main() {{
    let mut out=Vec::new();
    let mode=std::env::args().nth(1).unwrap_or_default();
    if mode=="t" {{ raw::dump(&mut out); }}
    else if mode=="h" {{
        {rust_constants}
        for value in 0..=255u8 {{ put(&mut out,inat_make_prefix(value)); put(&mut out,inat_make_escape(value)); put(&mut out,inat_make_group(value)); put(&mut out,inat_make_imm(value)); }}
        let mut state=0x1234abcdu32;
        for i in 0..20000 {{ state=state.wrapping_mul(1664525).wrapping_add(1013904223); let value=if i<32 {{1u32<<i}} else {{state}}; {rust_helpers} }}
    }} else {{
        for op in 0..=255u8 {{put(&mut out,inat_get_opcode_attribute(op));put(&mut out,inat_get_last_prefix_id(op) as u32);}}
        for e in 0..4u8 {{ for p in 0..4u8 {{ for op in 0..=255u8 {{put(&mut out,inat_get_escape_attribute(op,p,inat_make_escape(e)));}}}}}}
        for g in 0..32u32 {{ for p in 0..4u8 {{ for m in 0..=255u8 {{put(&mut out,inat_get_group_attribute(m,p,(g<<INAT_GRP_OFFS)|(0x0d3f8011&!INAT_GRP_MASK)));}}}}}}
        for map in 0..=255u8 {{ for p in 0..8u8 {{ for op in 0..=255u8 {{put(&mut out,inat_get_avx_attribute(op,map,p));}}}}}}
        for map in 0..=255u8 {{ for op in 0..=255u8 {{put(&mut out,inat_get_xop_attribute(op,map));}}}}
    }}
    std::io::stdout().lock().write_all(&out).unwrap();
}}
'''
        c_file = cls.work / "harness.c"
        c_file.write_text(c_source)
        result = subprocess.run(cls.cc + ["-O2", "-Wall", "-Werror", "-I" + str(cls.work),
                                          "-I" + str(ROOT / "tools/arch/x86/include"), "-I" + str(ROOT / "tools/include"),
                                          str(c_file), "-o", str(cls.c)], capture_output=True)
        if result.returncode:
            raise RuntimeError(result.stderr)
        cls.compile_rust(cls.rust_source, cls.rust)
        cls.compile_rust(cls.rust_source.replace(str(LOOKUP), str(ROOT / "tools/arch/x86/lib/inat.rs")), cls.tools)

    @classmethod
    def compile_rust(cls, source, output, extra=()):
        result = subprocess.run(cls.rustc + ["--edition=2021", "-O", "-Dwarnings", "-Wmissing_docs", "-Wunreachable_pub",
                                            "-Wrust_2018_idioms", "--crate-name=inat_test", *extra, "-", "-o", str(output)],
                                input=source.encode(), capture_output=True)
        if result.returncode:
            raise RuntimeError(result.stderr)
        return result

    def compare(self, mode):
        result = [subprocess.run([str(tool), mode], capture_output=True, check=True, timeout=20) for tool in (self.c, self.rust, self.tools)]
        self.assertEqual(result[0].stdout, result[1].stdout)
        self.assertEqual(result[0].stdout, result[2].stdout)
        self.assertTrue(result[0].stdout)

    def test_all_generated_tables_and_null_matrices_match_original_awk(self):
        self.compare("t")

    def test_exhaustive_lookups_all_opcodes_groups_prefixes_and_maps(self):
        self.compare("l")

    def test_every_constant_constructor_predicate_and_extractor(self):
        self.compare("h")

    def test_generator_reproducibility_and_compile_time_stale_map_rejection(self):
        self.assertEqual(GEN.generate(MAP.read_bytes()), TABLES.read_text())
        result = subprocess.run(["python3", str(GENERATOR), "--check"], capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        with tempfile.TemporaryDirectory(prefix="inat-stale-") as temporary:
            directory = Path(temporary)
            (directory / "inat_tables.rs").write_bytes(TABLES.read_bytes())
            (directory / "x86-opcode-map.txt").write_bytes(MAP.read_bytes() + b"\n# changed map\n")
            source = self.rust_source.replace(str(TABLES), str(directory / "inat_tables.rs"))
            result = subprocess.run(self.rustc + ["--edition=2021", "--crate-name=stale", "-", "-o", str(directory / "test")],
                                    input=source.encode(), capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"opcode map changed; regenerate tables", result.stderr)

    def test_core_only_attribute_module_and_map_dependency_tracking(self):
        # Export wrappers make every lookup reachable without relying on std.
        source = f'''//! Core-only attribute smoke test.
#![no_std]
#[path="{LOOKUP}"] mod inat;
/// Exercise every attribute lookup without heap allocation or external calls.
pub fn attributes(op:u8, map:u8, prefix:u8, attr:u32)->u32 {{
    inat::inat_get_opcode_attribute(op) ^ inat::inat_get_last_prefix_id(op) as u32 ^
    inat::inat_get_escape_attribute(op,prefix,attr) ^ inat::inat_get_group_attribute(op,prefix,attr) ^
    inat::inat_get_avx_attribute(op,map,prefix) ^ inat::inat_get_xop_attribute(op,map)
}}
'''
        dependency = self.work / "core.d"
        self.compile_rust(source, self.work / "core.rlib", ["--crate-type=lib", "--emit=link,dep-info=" + str(dependency)])
        self.assertIn(str(MAP), dependency.read_text())
        self.assertIn(str(TABLES), dependency.read_text())

    def test_tools_header_imports_are_core_only_and_preserve_types(self):
        source = f'''//! Core-only tools header smoke test.
#![no_std]
#[path="{ROOT / 'tools/arch/x86/include/asm/inat_header.rs'}"] mod attributes;
#[path="{ROOT / 'tools/arch/x86/include/asm/inat_types_header.rs'}"] mod types;
/// Verify the thin tools imports use the same integer types and definitions.
pub fn attributes(attr: types::Attr, byte: types::Byte, value: types::Value)->u32 {{
    attributes::inat_group_common_attribute(attr) ^ attributes::inat_make_prefix(byte) ^ value as u32
}}
'''
        self.compile_rust(source, self.work / "headers.rlib", ["--crate-type=lib"])

    def test_invalid_prefix_indices_are_checked_without_panics(self):
        source = f'''//! Invalid attribute-index smoke test.
#[path="{LOOKUP}"] mod inat;
fn main() {{
    for value in 0..=255u8 {{
        let _ = inat::inat_get_opcode_attribute(value);
        let _ = inat::inat_get_last_prefix_id(value);
        for prefix in 4..=255u8 {{
            for escape in 0..4 {{ let _ = inat::inat_get_escape_attribute(value,prefix,inat::inat_make_escape(escape)); }}
            for group in 0..32 {{ let _ = inat::inat_get_group_attribute(value,prefix,inat::inat_make_group(group)); }}
            assert_eq!(inat::inat_get_avx_attribute(value,value,prefix),0);
        }}
        if !(8..=31).contains(&value) {{ assert_eq!(inat::inat_get_xop_attribute(value,value),0); }}
    }}
}}
'''
        binary = self.work / "invalid-indices"
        self.compile_rust(source, binary)
        subprocess.run([str(binary)], check=True, timeout=10)

    def test_generator_rejects_malformed_opcode_maps(self):
        for source in (b"Table: x\nAVXcode:\n00: Unknown Ib,Iw\nEndTable\n",
                       b"Table: x\nAVXcode:\n00: Unknown Ix\nEndTable\n",
                       b"Table: x\nAVXcode:\n00: NOPE (Prefix)\nEndTable\n",
                       b"GrpTable: GrpMissing\nEndTable\n", b"Table: x\nAVXcode: 32\nEndTable\n",
                       b"Table: x\nAVXcode:\n00: ADD Eb,Gb\n00: ADD Eb,Gb\nEndTable\n",
                       b"Table: x\nAVXcode:\n00: ADD Eb,Gb\n"):
            with self.subTest(source=source), self.assertRaisesRegex(ValueError, "Semantic error"):
                GEN.generate(source)


if __name__ == "__main__":
    unittest.main()
