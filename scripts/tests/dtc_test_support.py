# SPDX-License-Identifier: GPL-2.0-or-later
"""Source-built DTC references shared by compiler differential tests."""

import os
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "scripts/dtc"


def build_c(directory):
    """Build the unchanged compiler, including its authoritative grammar."""
    directory = Path(directory)
    binary = directory / "dtc-c"
    subprocess.run(["bison", "-d", "-o", str(directory / "dtc-parser.tab.c"),
                    str(SOURCE / "dtc-parser.y")], check=True)
    subprocess.run(["flex", "-o", str(directory / "dtc-lexer.lex.c"),
                    str(SOURCE / "dtc-lexer.l")], check=True)
    sources = [SOURCE / (name + ".c") for name in
               ("dtc", "flattree", "fstree", "data", "livetree", "treesource",
                "srcpos", "checks", "util")]
    sources += [directory / "dtc-parser.tab.c", directory / "dtc-lexer.lex.c"]
    subprocess.run([*shlex.split(os.environ.get("HOSTCC", "cc")), "-O2", "-DNO_YAML",
                    "-I", str(SOURCE), "-I", str(SOURCE / "libfdt"),
                    "-I", str(directory), *map(str, sources), "-o", str(binary)], check=True)
    return binary


def build_rust(directory, source=None):
    """Compile the port with the same strict host-tool lints as Kbuild."""
    binary = Path(directory) / "dtc-rust"
    subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")),
                    "--edition=2021", "-O", "-Wmissing-docs", "-Wrust_2018_idioms",
                    "-Wunreachable_pub", "-Dwarnings", str(source or SOURCE / "dtc.rs"),
                    "-o", str(binary)], check=True)
    return binary
