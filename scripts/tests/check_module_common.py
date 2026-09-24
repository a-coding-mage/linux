#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Validate the selected common metadata without treating Rust as C glue.

The retained C branch keeps the existing private-fixture export-suppression
guard. The Rust branch additionally checks its real source/dependency chain,
the target-header-generated values, and the entire allocated ELF payload.
No compiler, make command, module load, or output-tree write is performed.
"""

from pathlib import Path
import re
import shlex
import struct

from check_div64_kernel import architecture, configuration, verify_build_command
from check_reciprocal_kernel import module_elf


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "scripts/module-common.rs"
INPUT = ROOT / "scripts/module-common-data.h"


def command(obj):
    text = obj.with_name("." + obj.name + ".cmd").read_text().replace("\\\n", " ")
    records = re.findall(r"(?m)^savedcmd_[^\n]+? := ([^\n]*)", text)
    if len(records) != 1 or re.search(r"(?m)^#SYMVER\b", text):
        raise ValueError("common metadata has missing commands or owner symbol versions")
    return shlex.split(records[0])


def newer(output, inputs):
    stamp = output.stat().st_mtime_ns
    if any(path.stat().st_mtime_ns > stamp for path in inputs):
        raise ValueError("stale common metadata: " + str(output))


def generated_values(path):
    """Parse only the generator's literal byte arrays, never evaluate Rust."""
    values = {}
    for line in path.read_text().splitlines():
        if not line or line.startswith("//"):
            continue
        array = re.fullmatch(r"pub\(super\) const (VERMAGIC|BUILD_SALT|ORC_HASH): \[u8; (\d+)\] = \[([\d, ]*)\];", line)
        integer = re.fullmatch(r"pub\(super\) const (LTO): i32 = ([01]);", line)
        if array:
            key = array[1]
            try:
                value = bytes(int(token.strip()) for token in array[3].split(",") if token.strip())
            except ValueError as error:
                raise ValueError("invalid common metadata byte") from error
            if len(value) != int(array[2]):
                raise ValueError("incorrect common metadata array length")
        elif integer:
            key, value = integer[1], int(integer[2])
        else:
            raise ValueError("unexpected generated common metadata declaration")
        if key in values:
            raise ValueError("duplicate generated common metadata value")
        values[key] = value
    if not {"VERMAGIC", "BUILD_SALT", "LTO"} <= values.keys():
        raise ValueError("incomplete generated common metadata")
    if not values["VERMAGIC"].startswith(b"vermagic=") or not values["VERMAGIC"].endswith(b"\0") or b"\0" in values["VERMAGIC"][:-1]:
        raise ValueError("invalid generated vermagic record")
    if not values["BUILD_SALT"].endswith(b"\0"):
        raise ValueError("build salt lacks the original trailing NUL")
    return values


def linux_notes(data):
    records, offset = [], 0
    while offset < len(data):
        if offset + 20 > len(data):
            raise ValueError("truncated common Linux note")
        namesz, descsz, kind = struct.unpack_from("<III", data, offset)
        end = offset + 20 + descsz
        padded = (end + 3) & ~3
        if namesz != 6 or data[offset + 12:offset + 20] != b"Linux\0\0\0" or padded > len(data) or any(data[end:padded]):
            raise ValueError("incorrect common Linux note layout")
        records.append((kind, data[offset + 20:end]))
        offset = padded
    return sorted(records)


def verify_rust_object(obj, config, values):
    identity, sections, symbols, _, imports = module_elf(obj)
    arch = architecture(config)
    if identity[0][2] != (183 if arch == "aarch64" else 62):
        raise ValueError("wrong common metadata target")
    owned = {b".modinfo": (1, 2, 1), b".note.Linux": (7, 2, 4)}
    if config.get("UNWINDER_ORC") == "y":
        owned[b".orc_header"] = (1, 2, 4)
    allocated = {}
    for index, section in enumerate(sections):
        name, kind, flags, _, size, _, target, align, _, content = section
        if name in allocated or name in (b".export_symbol", b"__ksymtab", b"__versions", b"__version_ext_crcs", b"__version_ext_names"):
            raise ValueError("duplicate or owner-specific common metadata section")
        if flags & 4 and size:
            raise ValueError("common metadata contains executable code")
        if kind in (4, 9) and target < len(sections) and sections[target][2] & 2 and content:
            raise ValueError("common metadata contains an allocated relocation")
        if flags & 2 and size:
            if name not in owned and name != b".note.gnu.property":
                raise ValueError("unexpected allocated common metadata: " + repr(name))
            allocated[name] = (index, section)
    if set(allocated) - {b".note.gnu.property"} != set(owned) or imports:
        raise ValueError("missing or owner-specific common metadata")
    for name, layout in owned.items():
        section = allocated[name][1]
        if (section[1], section[2], section[7]) != layout:
            raise ValueError("incorrect common metadata section flags/alignment")
    modinfo = allocated[b".modinfo"][1][-1]
    expected = [values["VERMAGIC"][:-1]]
    if config.get("MITIGATION_RETPOLINE") == "y":
        expected.append(b"retpoline=Y")
    if not modinfo.endswith(b"\0") or sorted(modinfo[:-1].split(b"\0")) != sorted(expected):
        raise ValueError("common metadata has unexpected or duplicate modinfo")
    expected_notes = sorted([(0x100, values["BUILD_SALT"]), (0x101, struct.pack("<i", values["LTO"]))])
    if linux_notes(allocated[b".note.Linux"][1][-1]) != expected_notes:
        raise ValueError("common metadata Linux note mismatch")
    if b".orc_header" in owned and (len(values.get("ORC_HASH", b"")) != 20 or
            allocated[b".orc_header"][1][-1] != values["ORC_HASH"]):
        raise ValueError("common metadata ORC hash mismatch")
    for name, info, _, index, value, size in symbols:
        if name and index == 0:
            raise ValueError("common metadata contains an undefined import")
        if info & 15 == 2 or name in (b"__IS_RUST_MODULE", b"__this_module", b"init_module", b"cleanup_module"):
            raise ValueError("common metadata contains an owner/runtime definition")
        if info >> 4 in (1, 2) and info & 15 != 1:
            raise ValueError("common metadata contains a non-metadata global definition")
        if info & 15 == 1 and (index >= len(sections) or sections[index][0] not in owned or
                               value + size > sections[index][4]):
            raise ValueError("common metadata has an unexpected data definition")


def verify_common_metadata(build, work, *, flags, exports):
    """Validate common metadata for an existing completed-kernel fixture.

    `flags` and `exports` are the caller's existing C command/record readers,
    retaining its C guard and independently testable ELF transport.
    """
    obj = work / ".module-common.o"
    config = configuration(build)
    if config.get("RUST_MODULE_COMMON") != "y":
        saved = flags(obj)
        if "-D__DISABLE_EXPORTS" not in saved or "-U__DISABLE_EXPORTS" in saved or exports(obj):
            raise ValueError("C common metadata suppression missing or contains exports")
        return
    if config.get("RUST") != "y" or config.get("MODULES") != "y":
        raise ValueError("Rust common metadata requires Rust and modules")
    data = work / ".module-common-data.rs"
    reference = build / ".module-common-data.rs"
    cfg = build / "include/generated/rustc_cfg"
    core = build / "rust/libcore.rmeta"
    saved = command(obj)
    cfgs = [token[1:] for token in saved if token.startswith("@")]
    data_paths = [token.split("=", 1)[1] for token in saved if token.startswith("MODULE_COMMON_DATA=")]
    # Make runs modfinal in the metadata output directory: the kernel build
    # directory for in-tree modules, or `work` for external modules. rustc
    # resolves @response files against that cwd, not against the source tree.
    # MODULE_COMMON_DATA is different: include!(env!(...)) would resolve a
    # relative value against module-common.rs, so retain its absolute contract.
    if (len(data_paths) != 1 or not Path(data_paths[0]).is_absolute() or
            Path(data_paths[0]).resolve() != data.resolve() or
            "--crate-name=module_common" not in saved or "--crate-type=rlib" not in saved or
            "-Zallow-features=" not in saved or len(cfgs) != 1 or not cfgs[0] or
            (work / cfgs[0]).resolve() != cfg.resolve() or
            "-Zsanitizer=kcfi" in saved or any("rust/helpers" in token for token in saved)):
        raise ValueError("wrong Rust common metadata compilation mode")
    verify_build_command(work, obj, SOURCE, [data, core])
    verify_build_command(work, data, INPUT, [ROOT / "include/linux/vermagic.h", build / "include/generated/utsrelease.h"])
    # fixdep records the individual CONFIG_* stamps consumed by the target
    # headers. An unrelated option can update auto.conf without invalidating
    # this data; the recorded dependencies above remain authoritative.
    newer(data, [ROOT / "scripts/module-common.c", ROOT / "scripts/module-common-data.rs",
                 build / "scripts/module-common-data"])
    newer(obj, [cfg, data, core])
    # The in-tree generated values come from the same authoritative target
    # headers; an external fixture must not substitute its own vermagic/notes.
    values = generated_values(data)
    if values != generated_values(reference):
        raise ValueError("external common metadata differs from the kernel target headers")
    if values["LTO"] != int(config.get("LTO") == "y"):
        raise ValueError("common metadata LTO note differs from configuration")
    verify_rust_object(obj, config, values)
    # Compiler-generated feature notes are not module records, but must still
    # be byte-identical to the completed kernel's selected common object.
    def properties(path):
        return [(section[1], section[2], section[7], section[-1])
                for section in module_elf(path)[1] if section[0] == b".note.gnu.property"]
    if properties(obj) != properties(build / ".module-common.o"):
        raise ValueError("external common metadata compiler feature note differs from the kernel")
