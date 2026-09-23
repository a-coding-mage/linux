# SPDX-License-Identifier: GPL-2.0
"""Build unchanged gendwarfksyms references, including isolated header prefixes.

GENDWARF_PREFIX may name an extracted development package's usr directory;
GENDWARF_CFLAGS and GENDWARF_LIBS override additional compiler/linker flags.
Nothing is downloaded or installed by the test suite.
"""

import ctypes.util
import os
from pathlib import Path
import shlex
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "scripts/gendwarfksyms"


def _pkg_config(option):
    command = shlex.split(os.environ.get("HOSTPKG_CONFIG", "pkg-config"))
    try:
        result = subprocess.run([*command, option, "libdw", "libelf"], capture_output=True)
    except FileNotFoundError:
        return []
    return shlex.split(os.fsdecode(result.stdout)) if result.returncode == 0 else []


def cflags():
    """Return libdw/libelf include flags, also usable by focused C harnesses."""
    flags = _pkg_config("--cflags")
    if os.environ.get("GENDWARF_PREFIX"):
        flags += ["-I" + str(Path(os.environ["GENDWARF_PREFIX"]) / "include")]
    return flags + shlex.split(os.environ.get("GENDWARF_CFLAGS", ""))


def libraries():
    """Find the installed runtime even with development headers in /tmp."""
    if "GENDWARF_LIBS" in os.environ:
        return shlex.split(os.environ["GENDWARF_LIBS"])
    flags = _pkg_config("--libs")
    if not flags:
        soname = ctypes.util.find_library("dw")
        flags = ["-l:" + soname if soname and ".so" in soname else "-ldw", "-lelf"]
    return [*flags, "-lz"]


def require_c_headers():
    """Skip clearly when the optional elfutils development headers are absent."""
    cc = shlex.split(os.environ.get("HOSTCC", "cc"))
    probe = subprocess.run([*cc, *cflags(), "-x", "c", "-E", "-"],
                           input=b"#include <elfutils/libdw.h>\n#include <elfutils/libdwfl.h>\n",
                           stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    if probe.returncode:
        raise unittest.SkipTest("libdw/libelf development headers required; set GENDWARF_PREFIX "
                                "or GENDWARF_CFLAGS for an isolated package extraction")


def build_c(directory):
    """Compile all seven unchanged original translation units."""
    if os.environ.get("GENDWARFKSYMS_C"):
        return Path(os.environ["GENDWARFKSYMS_C"])
    require_c_headers()
    cc = shlex.split(os.environ.get("HOSTCC", "cc"))
    binary = Path(directory) / "gendwarfksyms-c"
    units = ("gendwarfksyms", "cache", "die", "dwarf", "kabi", "symbols", "types")
    subprocess.run([*cc, "-O2", "-Wall", "-I" + str(ROOT / "scripts/include"), *cflags(),
                    *[str(SOURCE / (unit + ".c")) for unit in units], *libraries(),
                    "-o", str(binary)], check=True)
    return binary


def build_rust(directory):
    """Compile the integrated implementation with the strict host-tool lints."""
    if os.environ.get("GENDWARFKSYMS_RUST"):
        return Path(os.environ["GENDWARFKSYMS_RUST"])
    binary = Path(directory) / "gendwarfksyms-rust"
    subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")),
                    "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
                    "-Wrust_2018_idioms", "-Wunreachable_pub",
                    *["-Clink-arg=" + flag for flag in libraries()],
                    *shlex.split(os.environ.get("GENDWARF_RUSTFLAGS", "")),
                    str(SOURCE / "gendwarfksyms.rs"), "-o", str(binary)], check=True)
    return binary
