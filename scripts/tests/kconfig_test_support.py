# SPDX-License-Identifier: GPL-2.0
"""Build the original and translated Kconfig tools once per test process."""

import atexit
from functools import lru_cache
import os
from pathlib import Path
import shlex
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def command(arguments):
    result = subprocess.run(arguments, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"Command failed: {shlex.join(map(str, arguments))}\n{result.stdout}{result.stderr}")


@lru_cache(maxsize=None)
def cached_conf_tools():
    """Return (C conf, Rust conf), honoring independent binary overrides."""
    temporary = tempfile.TemporaryDirectory(prefix="kconfig-conf-tools-")
    atexit.register(temporary.cleanup)
    work = Path(temporary.name)
    source = ROOT / "scripts/kconfig"
    c = Path(os.environ.get("KCONFIG_C_CONF", work / "conf-c")).resolve()
    rust = Path(os.environ.get("KCONFIG_RUST_CONF", work / "conf-rust")).resolve()
    if "KCONFIG_C_CONF" not in os.environ:
        command(shlex.split(os.environ.get("YACC", "bison")) + [
            "-o", str(work / "parser.tab.c"), "--defines=" + str(work / "parser.tab.h"),
            "-t", "-l", str(source / "parser.y")])
        command(shlex.split(os.environ.get("LEX", "flex")) + [
            "-o" + str(work / "lexer.lex.c"), "-L", str(source / "lexer.l")])
        sources = [source / (name + ".c") for name in
                   ("conf", "confdata", "expr", "menu", "preprocess", "symbol", "util")]
        command(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-Wall", "-Werror", "-I", str(source), "-I", str(ROOT / "scripts/include"),
            "-I", str(work)] + list(map(str, sources + [work / "lexer.lex.c", work / "parser.tab.c"])) +
            ["-o", str(c)])
    if "KCONFIG_RUST_CONF" not in os.environ:
        command(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
            "-Dwarnings", str(source / "conf.rs"), "-o", str(rust)])
    return c, rust


def full_tree_environment():
    """A host-tool environment for parsing the x86 kernel Kconfig tree."""
    environment = {name: value for name, value in os.environ.items()
                   if not name.startswith("KCONFIG_") and name not in (
                       "CONFIG_", "srctree", "MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "POSIXLY_CORRECT")}
    cc = os.environ.get("HOSTCC", "cc")
    rustc = os.environ.get("HOSTRUSTC", "rustc")
    environment.update({
        "LC_ALL": "C", "srctree": str(ROOT), "ARCH": "x86", "SRCARCH": "x86",
        "KERNELVERSION": "kconfig-integration-test", "CC": cc, "LD": "ld",
        "NM": "nm", "OBJCOPY": "objcopy", "OBJDUMP": "objdump", "AR": "ar",
        "RUSTC": rustc, "BINDGEN": "bindgen", "PAHOLE": "pahole", "PAHOLE_VERSION": "0",
        "CLANG_FLAGS": "", "RUSTC_BOOTSTRAP": "1", "KBUILD_EXTMOD": "", "PYTHON3": "python3",
        "KCONFIG_DEFCONFIG_LIST": "",
    })
    for name, tool in (("CC_VERSION_TEXT", cc), ("RUSTC_VERSION_TEXT", rustc)):
        result = subprocess.run(shlex.split(tool) + ["--version"], check=True, capture_output=True, text=True)
        environment[name] = result.stdout.splitlines()[0]
    return environment
