#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Generate immutable Rust instruction attributes from the canonical opcode map.

The normal build uses checked-in tables and verifies their source fingerprint.
This maintenance helper emits an apply_patch patch, or checks reproducibility
with --check. It does not run the original AWK script or compile C.
"""

import argparse
from pathlib import Path
import re
import sys


ROOT = Path(__file__).resolve().parents[3]
MAP = ROOT / "arch/x86/lib/x86-opcode-map.txt"
OUTPUT = ROOT / "arch/x86/lib/inat_tables.rs"

IMMEDIATES = {
    "Ib": "BYTE", "Jb": "BYTE", "Iw": "WORD", "Id": "DWORD", "Iq": "QWORD", "Ap": "PTR",
    "Iz": "VWORD32", "Jz": "VWORD32", "Iv": "VWORD", "Ob": None, "Ov": None, "Lx": "BYTE", "Lo": "BYTE",
}
PREFIXES = {
    "Operand-Size": "OPNDSZ", "REPNE": "REPNE", "REP/REPE": "REPE", "XACQUIRE": "REPNE",
    "XRELEASE": "REPE", "LOCK": "LOCK", "SEG=CS": "CS", "SEG=DS": "DS", "SEG=ES": "ES",
    "SEG=FS": "FS", "SEG=GS": "GS", "SEG=SS": "SS", "Address-Size": "ADDRSZ",
    "VEX+1byte": "VEX2", "VEX+2byte": "VEX3", "EVEX": "EVEX", "REX2": "REX2", "XOP": "XOP",
}


def combine(old, new):
    return " | ".join(value for value in (old, new) if value)


class Tables:
    def __init__(self):
        self.escapes = {}
        self.groups = {}
        self.definitions = []
        self.matrices = {"ESCAPE": {}, "GROUP": {}, "AVX": {}, "XOP": {}}
        self.line = 0
        self.clear()

    def clear(self):
        self.rows = [{}, {}, {}, {}]
        self.escape = self.group = self.avx = self.xop = -1
        self.name = ""

    def fail(self, message):
        raise ValueError(f"Semantic error at {self.line}: {message}")

    def operands(self, operands):
        immediate = modrm = ""
        for operand in operands.split(","):
            if re.match(r"[IJAOL][a-z]", operand):
                if operand not in IMMEDIATES:
                    self.fail("Unknown imm opnd: " + operand)
                if immediate:
                    if operand != "Ib":
                        self.fail("Second IMM error")
                    immediate = combine(immediate, "INAT_SCNDIMM")
                else:
                    kind = IMMEDIATES[operand]
                    immediate = f"inat_make_imm(INAT_IMM_{kind})" if kind else "INAT_MOFFSET"
            elif re.match(r"([CDEGMNPQRSUVW/][a-z]+|NTA|T[012])", operand):
                modrm = "INAT_MODRM"
        return combine(immediate, modrm)

    def instruction(self, fields, line):
        index = int(fields[0][:-1], 16)
        if not self.name or index >= (8 if self.group >= 0 else 256):
            self.fail("Opcode index outside active table")
        if index in self.rows[0]:
            self.fail(f"Redefine 0x{index:x} in {self.name.lower()}")
        if len(fields) > 1 and fields[1] == "escape":
            if len(fields) < 3 or fields[2] != "#":
                self.fail("No escaped name")
            reference = "".join(fields[3:])
            if reference in self.escapes:
                self.fail(f"Redefine escape ({reference})")
            number = len(self.escapes) + 1
            if number > 3:
                self.fail("Too many escape tables")
            self.escapes[reference] = number
            self.rows[0][index] = f"inat_make_escape({number})"
            return
        variant = False
        cursor = 1
        while cursor < len(fields):
            opcode = fields[cursor]
            cursor += 1
            flags = extension = ""
            if cursor < len(fields) and re.match(r"[A-Za-z/]", fields[cursor]):
                flags = self.operands(fields[cursor])
                cursor += 1
            if cursor < len(fields) and fields[cursor].startswith("("):
                extension = fields[cursor]
                cursor += 1
            if cursor < len(fields) and fields[cursor] == "|":
                cursor += 1
            elif cursor < len(fields) - 1:
                self.fail(fields[cursor] + " is not a separator")
            if re.match(r"Grp[0-9A-Za-z]+", opcode):
                if opcode not in self.groups:
                    self.groups[opcode] = len(self.groups) + 1
                if self.groups[opcode] > 31:
                    self.fail("Too many opcode groups")
                flags = combine(flags, f"inat_make_group({self.groups[opcode]})")
            for enabled, flag in (
                    (re.search(r"\([df]64\)", extension), "INAT_FORCE64"),
                    ("(i64)" in extension and "(o64)" not in line, "INAT_INV64"),
                    ("(!REX2)" in extension, "INAT_NO_REX2"),
                    (re.match(r"((REX(\.[XRWB]+)+)|(REX$))", opcode), "inat_make_prefix(INAT_PFX_REX)"),
                    (opcode.startswith("ESC"), "INAT_MODRM")):
                if enabled:
                    flags = combine(flags, flag)
            if "(ev)" in extension:
                flags = combine(flags, "INAT_VEXOK | INAT_EVEXONLY")
            elif "(es)" in extension:
                flags = combine(flags, "INAT_VEXOK | INAT_EVEXONLY | INAT_EVEX_SCALABLE")
            elif "(v)" in extension:
                flags = combine(flags, "INAT_VEXOK | INAT_VEXONLY")
            elif "(v1)" in extension or opcode.startswith(("v", "k")):
                flags = combine(flags, "INAT_VEXOK")
            elif "(xop)" in extension or self.xop >= 0:
                flags = combine(flags, "INAT_XOPOK")
            if "(Prefix)" in extension:
                if opcode not in PREFIXES:
                    self.fail("Unknown prefix: " + opcode)
                flags = combine(flags, f"inat_make_prefix(INAT_PFX_{PREFIXES[opcode]})")
            if not flags:
                continue
            for prefix, pattern in ((1, r"\((66|!F3)\)"), (2, r"\(F3\)"), (3, r"\((F2|!F3|66&F2)\)")):
                if re.search(pattern, extension):
                    self.rows[prefix][index] = combine(self.rows[prefix].get(index, ""), flags)
                    variant = True
            if "(REX2)" in extension:
                self.rows[0][index] = combine(self.rows[0].get(index, ""), "INAT_REX2_VARIANT")
            if not re.search(r"\((66|F2|F3)\)", extension):
                self.rows[0][index] = combine(self.rows[0].get(index, ""), flags)
        if variant:
            self.rows[0][index] = combine(self.rows[0].get(index, ""), "INAT_VARIANT")

    def parse(self, source):
        for self.line, line in enumerate(source.decode("latin1").split("\n"), 1):
            fields = re.split(r"[ \t]+", line.strip(" \t"))
            if line.startswith("Table:"):
                if self.name:
                    self.fail("Hit Table: before EndTable:.")
            elif line.startswith("Referrer:") and len(fields) > 1:
                reference = "".join(fields[1:])
                if reference not in self.escapes:
                    self.fail("Unknown escape: " + reference)
                self.escape = self.escapes[reference]
                self.name = f"INAT_ESCAPE_TABLE_{self.escape}"
            elif line.startswith(("AVXcode:", "XOPcode:")):
                avx = line.startswith("AVXcode:")
                if len(fields) > 1:
                    number = int(fields[1])
                    if not 0 <= number < (32 if avx else 24):
                        self.fail("Opcode map selector out of range")
                    self.avx, self.xop = (number, -1) if avx else (-1, number)
                    if not self.name:
                        self.name = f"INAT_{'AVX' if avx else 'XOP'}_TABLE_{number}"
                if (self.avx if avx else self.xop) == -1 and self.escape == -1:
                    self.name = "INAT_PRIMARY_TABLE"
            elif line.startswith("GrpTable:"):
                if len(fields) != 2 or fields[1] not in self.groups:
                    self.fail("No group: " + (fields[1] if len(fields) > 1 else ""))
                self.group = self.groups[fields[1]]
                self.name = f"INAT_GROUP_TABLE_{self.group}"
            elif line.startswith("EndTable"):
                for prefix, row in enumerate(self.rows):
                    if not row:
                        continue
                    name = self.name + (f"_{prefix}" if prefix else "")
                    self.definitions.append((name, 8 if self.group >= 0 else 256, row))
                    if self.group >= 0:
                        self.matrices["GROUP"][self.group, prefix] = name
                    else:
                        self.matrices["ESCAPE"][self.escape, prefix] = name
                        if self.avx >= 0:
                            self.matrices["AVX"][self.avx, prefix] = name
                        elif self.xop >= 0 and prefix == 0:
                            self.matrices["XOP"][self.xop, 0] = name
                self.clear()
            elif re.match(r"^[0-9a-f]+:", line) and self.line != 1:
                self.instruction(fields, line)
        if self.name:
            self.fail("Unterminated opcode table")
        return self


def fingerprint(source):
    value = 0xcbf29ce484222325
    for byte in source:
        value = ((value ^ byte) * 0x100000001b3) & 0xffffffffffffffff
    return value


def generate(source):
    tables = Tables().parse(source)
    output = ["// SPDX-License-Identifier: GPL-2.0", "// Generated by arch/x86/tools/gen_inat_tables.py. Do not edit.",
              "// Source: arch/x86/lib/x86-opcode-map.txt", "use super::attributes::*;", "",
              "#[allow(dead_code)] // Rust 1.85 does not count use by the anonymous const assertion.",
              "const fn source_hash(bytes: &[u8]) -> u64 {", "    let mut hash = 0xcbf29ce484222325u64;",
              "    let mut index = 0;", "    while index < bytes.len() {",
              "        hash = (hash ^ bytes[index] as u64).wrapping_mul(0x100000001b3);", "        index += 1;",
              "    }", "    hash", "}", "",
              "const fn table<const N: usize>(entries: &[(usize, Attr)]) -> [Attr; N] {",
              "    let mut result = [0; N];", "    let mut index = 0;", "    while index < entries.len() {",
              "        result[entries[index].0] = entries[index].1;", "        index += 1;",
              "    }", "    result", "}", "",
              "const _: () = assert!(", f'    source_hash(include_bytes!("x86-opcode-map.txt")) == 0x{fingerprint(source):016x},',
              '    "opcode map changed; regenerate tables with arch/x86/tools/gen_inat_tables.py"', ");", ""]
    for name, size, row in tables.definitions:
        output.append(f"pub(super) static {name}: [Attr; {size}] = table(&[")
        for index, flags in sorted(row.items()):
            output.append(f"    (0x{index:02x}, {flags}),")
        output += ["]);", ""]
    for kind, count, size, prefixes in (("ESCAPE", 4, 256, 4), ("GROUP", 32, 8, 4), ("AVX", 32, 256, 4), ("XOP", 24, 256, 1)):
        element = f"Option<&[Attr; {size}]>"
        item = f"[{element}; {prefixes}]" if prefixes > 1 else element
        output.append(f"pub(super) static INAT_{kind}_TABLES: [{item}; {count}] = [")
        for index in range(count):
            references = [tables.matrices[kind].get((index, prefix)) for prefix in range(prefixes)]
            values = [f"Some(&{name})" if name else "None" for name in references]
            output.append("    " + ("[" + ", ".join(values) + "]" if prefixes > 1 else values[0]) + ",")
        output += ["];", ""]
    return "\n".join(output)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--map", type=Path, default=MAP)
    parser.add_argument("--output", type=Path, default=OUTPUT)
    parser.add_argument("--check", action="store_true", help="verify the checked-in table without writing anything")
    args = parser.parse_args()
    try:
        generated = generate(args.map.read_bytes())
        old = args.output.read_text() if args.output.exists() else None
        if args.check:
            if old != generated:
                raise ValueError("instruction attribute tables are stale; regenerate them")
        elif old != generated:
            print("*** Begin Patch")
            if old is None:
                print("*** Add File: " + str(args.output))
            else:
                print("*** Update File: " + str(args.output))
                print("@@")
                for line in old.splitlines():
                    print("-" + line)
            for line in generated.splitlines():
                print("+" + line)
            print("*** End Patch")
    except (OSError, ValueError) as error:
        print(str(error), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
