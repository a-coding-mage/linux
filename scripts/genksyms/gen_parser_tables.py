#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""Regenerate the Rust LR tables from genksyms' authoritative parse.y.

The production host tool does not need Bison.  This maintenance helper emits
an apply_patch patch, leaving file replacement to the caller.
"""

import json
import hashlib
import pathlib
import re
import subprocess
import tempfile
import xml.etree.ElementTree as ET


def main():
    directory = pathlib.Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix="genksyms-tables-") as temporary:
        source = pathlib.Path(temporary) / "parse.c"
        xml = pathlib.Path(temporary) / "parse.xml"
        subprocess.run(["bison", "-t", f"--xml={xml}", "-o", str(source),
                        str(directory / "parse.y")], check=True)
        c = source.read_text()
        grammar = ET.parse(xml).getroot()
        lines = ["// SPDX-License-Identifier: GPL-2.0-or-later",
                 "// LR tables derived from parse.y; regenerate with gen_parser_tables.py.",
                 "// Copyright 1996, 1997 Linux International.",
                 "// Original parser: Richard Henderson and Bjorn Ekwall.",
                 "#![allow(dead_code)]", "",
                 'pub(super) const GRAMMAR_SHA256: &str = "' +
                 hashlib.sha256((directory / "parse.y").read_bytes()).hexdigest() + '";']
        for name in ["YYFINAL", "YYLAST", "YYNTOKENS", "YYPACT_NINF"]:
            value = re.search(r"#define " + name + r"\s+\(?(-?\d+)", c)[1]
            lines.append(f"pub(super) const {name}: i16 = {value};")
        for name in ["yytranslate", "yypact", "yydefact", "yypgoto", "yydefgoto",
                     "yytable", "yycheck", "yyr1", "yyr2", "yyrline", "yystos"]:
            values = re.search(name + r"\[\]\s*=\s*\{([^}]+)\}", c, re.S)[1]
            numbers = re.findall(r"-?\d+", values)
            lines.append(f"pub(super) const {name.upper()}: [i16; {len(numbers)}] = [")
            for offset in range(0, len(numbers), 20):
                lines.append("    " + ", ".join(numbers[offset:offset + 20]) + ",")
            lines.append("];\n")
        symbols = sorted(grammar.findall("grammar/terminals/terminal") +
                         grammar.findall("grammar/nonterminals/nonterminal"),
                         key=lambda item: int(item.attrib["symbol-number"]))
        names = ['"invalid token"'] * (int(symbols[-1].attrib["symbol-number"]) + 1)
        for symbol in symbols:
            names[int(symbol.attrib["symbol-number"])] = symbol.attrib["name"]
        # Bison's display name for the end marker differs from its XML name.
        names[0] = '"end of file"'
        lines.append(f"pub(super) const NAMES: [&str; {len(names)}] = [")
        lines.extend("    " + json.dumps(name) + "," for name in names)
        lines.append("];\n")
        for symbol in grammar.findall("grammar/terminals/terminal"):
            name = symbol.attrib["name"]
            if name.isidentifier() and name.isupper():
                lines.append(f"pub(super) const {name}: i16 = {symbol.attrib['token-number']};")
        content = subprocess.run(["rustfmt", "--edition", "2021"],
                                 input="\n".join(lines) + "\n", text=True,
                                 capture_output=True, check=True).stdout
        target = directory / "parser_tables.rs"
        if target.exists():
            patch = "*** Begin Patch\n*** Update File: " + str(target) + "\n@@\n"
            patch += "".join("-" + line + "\n" for line in target.read_text().splitlines())
        else:
            patch = "*** Begin Patch\n*** Add File: " + str(target) + "\n"
        patch += "".join("+" + line + "\n" for line in content.splitlines())
        print(patch + "*** End Patch")


if __name__ == "__main__":
    main()
