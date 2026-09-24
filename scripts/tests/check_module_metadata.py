#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Read-only validation of selected per-module loader metadata.

The C path retains its suppression guard. The Rust path validates actual source,
fixdep dependencies, generated data, nominal binding DWARF layout, every
allocated section/relocation and final import CRC. It never compiles or loads a
module. These native runtime gates support little-endian x86-64 and ARM64 with
the selected metadata's real DWARF, not invented module layouts.
"""
import ast
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess

ROOT = next(path for path in (Path(__file__).resolve().parents[2], Path.cwd())
            if (path / "scripts/module-metadata.rs").is_file())
SOURCE = ROOT / "scripts/module-metadata.rs"
HEADER = ROOT / "include/linux/export-internal_header.rs"


def configuration(build):
    return dict(line.removeprefix("CONFIG_").split("=", 1)
                for line in (build / ".config").read_text().splitlines()
                if line.startswith("CONFIG_") and "=" in line)


def metadata_source(build, module):
    """The selected original frontend input, never an inactive .mod.c orphan."""
    return module.with_suffix(".mod.h" if configuration(build).get("RUST_MODULE_METADATA") == "y" else ".mod.c")


def selected_metadata(build, module):
    """Validate selected Rust metadata, then expose its original C input."""
    if configuration(build).get("RUST_MODULE_METADATA") == "y":
        return verify_module_metadata(build, module)
    return metadata_source(build, module)


def command_root(obj):
    """Recover M=/MO=/in-tree command cwd from fixdep's exact target key."""
    text = obj.with_name("."+obj.name+".cmd").read_text()
    targets = re.findall(r"(?m)^source_(.+?) := ", text)
    if len(targets)!=1: raise ValueError("missing or duplicate module metadata source record")
    target = Path(targets[0])
    if target.is_absolute() or ".." in target.parts or not target.parts or target.name!=obj.name:
        raise ValueError("unsupported module metadata target path")
    work = obj.parents[len(target.parts)-1]
    if (work/target).resolve()!=obj.resolve(): raise ValueError("module metadata target key mismatch")
    return work


def newer(output, inputs):
    stamp = output.stat().st_mtime_ns
    if any(path.stat().st_mtime_ns > stamp for path in inputs):
        raise ValueError("stale selected module metadata: " + str(output))


def command(obj):
    text = obj.with_name("." + obj.name + ".cmd").read_text().replace("\\\n", " ")
    saved = re.findall(r"(?m)^savedcmd_[^\n]+? := ([^\n]*)", text)
    if len(saved) != 1 or re.search(r"(?m)^#SYMVER\b", text):
        raise ValueError("module metadata command missing/duplicated or recalculates owner CRCs")
    return shlex.split(saved[0])


def byte_array(text):
    if not re.fullmatch(r"\[[0-9, ]*\]", text):
        raise ValueError("non-literal generated metadata bytes")
    values = ast.literal_eval(text)
    try:
        return bytes(values)
    except (ValueError, TypeError) as error:
        raise ValueError("invalid generated metadata bytes") from error


def rust_string(text):
    # Debug-format output has a finite escape grammar; do not evaluate Rust.
    if not re.fullmatch(r'"(?:[^"\\]|\\(?:["\\nrt0]|x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}))*"', text):
        raise ValueError("invalid generated Rust string")
    def escape(match):
        item = match[1]
        if item.startswith("u{"): return chr(int(item[2:-1], 16))
        if item.startswith("x"): return chr(int(item[1:], 16))
        return {"n":"\n", "r":"\r", "t":"\t", "0":"\0", '"':'"', "\\":"\\"}[item]
    return re.sub(r'\\(u\{[0-9a-fA-F]+\}|x[0-9a-fA-F]{2}|.)', escape, text[1:-1]).encode()


def generated_values(path):
    text = re.sub(r"(?m)^\s*//[^\n]*\n?", "", path.read_text())
    info, sections, exports, flags, crcs = [], {}, {}, {}, {}
    owner = None
    string = r'"(?:[^"\\]|\\.)*"'
    while text.strip():
        text = text.lstrip()
        match = re.match(r'#\[used\]\s*#\[link_section = "([^"\n]+)"\]\s*static ([A-Za-z_0-9]+): \[(u8|u32); (\d+)\] = (\[[0-9, ]*\]);', text)
        if match:
            section, name, kind, count, literal = match.groups()
            values = ast.literal_eval(literal)
            if len(values) != int(count): raise ValueError("generated metadata length mismatch")
            if section == ".modinfo" and re.fullmatch(r"__MODULE_INFO_[0-9]+", name) and kind == "u8":
                data = byte_array(literal)
                if not data.endswith(b"\0") or b"\0" in data[:-1]: raise ValueError("invalid generated modinfo")
                info.append(data)
            elif (section, name, kind) in (("__version_ext_crcs", "____VERSION_EXT_CRCS", "u32"),
                                           ("__version_ext_names", "____VERSION_EXT_NAMES", "u8")):
                if section in sections: raise ValueError("duplicate generated version array")
                sections[section] = (b"".join(struct.pack("<I", value) for value in values)
                                     if kind == "u32" else byte_array(literal))
            else: raise ValueError("unexpected generated data declaration")
        else:
            match = re.match(r'#\[used\]\s*#\[no_mangle\]\s*#\[link_section = "\.gnu.linkonce.this_module"\]\s*pub static mut __this_module: bindings::module = \{\s*let (?:mut )?value = module_value\(&(\[[0-9, ]*\])\);\s*((?:value\.(?:init|exit) = Some\(bindings::(?:init_module|cleanup_module)\);\s*)*)value\s*\};', text)
            if match:
                if owner is not None: raise ValueError("duplicate generated module owner")
                name = byte_array(match[1])
                assignments = re.findall(r"value\.(\w+) = Some\(bindings::(\w+)\);", match[2])
                if (not name.endswith(b"\0") or b"\0" in name[:-1] or
                        len(dict(assignments)) != len(assignments) or
                        any((a,b) not in (("init","init_module"),("exit","cleanup_module")) for a,b in assignments)):
                    raise ValueError("invalid generated module name/lifecycle")
                owner = name, dict(assignments)
            else:
                match = re.match(r'#\[used\]\s*#\[link_section = "__versions"\]\s*static ____VERSIONS: \[bindings::modversion_info; (\d+)\] = \[(.*?)\];', text, re.S)
                if match:
                    if "__versions" in sections: raise ValueError("duplicate generated basic versions")
                    rows = re.findall(r"\s*version_value\((0x[0-9a-f]{8}), &(\[[0-9, ]*\])\),", match[2])
                    rest = re.sub(r"\s*version_value\((0x[0-9a-f]{8}), &(\[[0-9, ]*\])\),", "", match[2])
                    if rest.strip() or len(rows) != int(match[1]): raise ValueError("malformed generated basic versions")
                    data = bytearray()
                    for crc, literal in rows:
                        name = byte_array(literal)
                        if not name.endswith(b"\0") or b"\0" in name[:-1] or len(name) > 56:
                            raise ValueError("invalid generated basic version name")
                        data += struct.pack("<Q", int(crc,16)) + name + bytes(56-len(name))
                    sections["__versions"] = bytes(data)
                else:
                    match = re.match(r'__KSYMTAB_NORMALIZED!\((' + string + r'), (' + string + r'), (' + string + r')\);', text)
                    if match:
                        name, ref, ns = map(rust_string, match.groups())
                        if name in exports: raise ValueError("duplicate generated export")
                        exports[name] = ref, ns
                    else:
                        match = re.match(r'SYMBOL_(FLAGS|CRC)_NORMALIZED!\((' + string + r'), (0x[0-9a-f]+)\);', text)
                        if not match: raise ValueError("unexpected generated module metadata item: " + text[:100])
                        table = flags if match[1] == "FLAGS" else crcs
                        name = rust_string(match[2])
                        if name in table: raise ValueError("duplicate generated export flags/CRC")
                        table[name] = int(match[3],16)
        text = text[match.end():]
    if owner is None: raise ValueError("generated metadata lacks actual module owner")
    if set(exports) != set(flags) or set(crcs) - set(exports): raise ValueError("inconsistent generated export tables")
    return dict(info=info, sections=sections, exports=exports, flags=flags, crcs=crcs, owner=owner)


def module_layout(obj):
    """Read offsets from this object's actual nominal bindings::module DWARF."""
    output = subprocess.run([*shlex.split(os.environ.get("READELF", "readelf")), "--debug-dump=info", obj],
                            check=True, capture_output=True, timeout=120,
                            env={**os.environ, "LC_ALL":"C"}).stdout.decode(errors="strict")
    dies, current = {}, None
    for line in output.splitlines():
        entry = re.match(r"\s*<(\d+)><([0-9a-f]+)>:.*?(?:\((DW_TAG_\w+)\))?$", line)
        if entry:
            current = {"depth":int(entry[1]), "tag":entry[3]}
            dies[int(entry[2],16)] = current
        elif current is not None:
            attr = re.match(r"\s*<[0-9a-f]+>\s+DW_AT_(\w+)\s*:\s*(.*)", line)
            if attr: current[attr[1]] = attr[2]
    def name(die): return die.get("name", "").rsplit(": ",1)[-1]
    variables = [die for die in dies.values() if die["tag"] == "DW_TAG_variable" and name(die) == "__this_module"]
    if len(variables) != 1 or not re.fullmatch(r"<0x[0-9a-f]+>", variables[0].get("type", "")):
        raise ValueError("missing actual module owner binding DWARF")
    index = int(variables[0]["type"][3:-1],16)
    record = dies.get(index, {})
    if record.get("tag") != "DW_TAG_structure_type" or name(record) != "module":
        raise ValueError("module owner does not use nominal module binding")
    members = {}
    for offset, die in dies.items():
        if offset <= index: continue
        if die["depth"] <= record["depth"]: break
        if die["depth"] == record["depth"]+1 and die["tag"] == "DW_TAG_member":
            members[name(die)] = int(die["data_member_location"])
    if not {"name", "init"} <= members.keys(): raise ValueError("incomplete module binding layout")
    return int(record["byte_size"]), int(record["alignment"]), members


def verify_object(obj, owner_obj, config, values, layout=None):
    from check_reciprocal_kernel import module_elf
    from rust_exports_test_support import read_exports
    identity, sections, symbols, _, _ = module_elf(obj)
    targets=[name for name in ("X86_64","ARM64") if config.get(name)=="y"]
    if len(targets)!=1 or config.get("64BIT")!="y" or config.get("CPU_BIG_ENDIAN")=="y":
        raise ValueError("metadata runtime audit requires little-endian x86-64 or ARM64")
    arm = targets[0] == "ARM64"
    if identity[0][2] != (183 if arm else 62): raise ValueError("wrong module metadata architecture")
    if read_exports(obj): raise ValueError("metadata contains owner export records")
    expected = {name.encode(): (1,2,{"__versions":8,"__version_ext_crcs":4,"__version_ext_names":1}[name],data)
                for name,data in values["sections"].items()}
    expected[b".modinfo"] = (1,2,1,b"".join(values["info"]))
    size, alignment, offsets = layout or module_layout(obj)
    name, lifecycle = values["owner"]
    # Derive lifecycle independently from the actual implementation, as
    # modpost.c:handle_symbol/add_header do. Comparing generated Rust to its
    # own ELF alone would accept a consistently omitted cleanup callback.
    # SHN_UNDEF and SHN_COMMON do not establish lifecycle definitions.
    defined = {symbol[0] for symbol in module_elf(owner_obj)[2]
               if symbol[3] not in (0, 0xfff2)}
    expected_lifecycle = {}
    if b"init_module" in defined:
        expected_lifecycle["init"] = "init_module"
    if config.get("MODULE_UNLOAD") == "y" and b"cleanup_module" in defined:
        expected_lifecycle["exit"] = "cleanup_module"
    if lifecycle != expected_lifecycle:
        raise ValueError("metadata lifecycle differs from actual owner definitions")
    data = bytearray(size)
    if offsets["name"]+len(name)>size: raise ValueError("module name exceeds actual binding layout")
    data[offsets["name"]:offsets["name"]+len(name)] = name
    expected[b".gnu.linkonce.this_module"] = (1,3,alignment,bytes(data))
    owner_records = read_exports(owner_obj)
    original = {row["name"].encode(): row for row in owner_records}
    if len(original) != len(owner_records) or set(original) != set(values["exports"]):
        raise ValueError("metadata exports differ from actual owner")
    prel = config.get("HAVE_ARCH_PREL32_RELOCATIONS") == "y"
    width, reloc = (4,261 if arm else 2) if prel else (8,257 if arm else 1)
    strings = bytearray()
    expected_reloc = {b".gnu.linkonce.this_module": [(offsets[field],257 if arm else 1,symbol.encode(),0)
                                                    for field,symbol in lifecycle.items()]}
    for export, (reference, namespace) in values["exports"].items():
        row = original[export]
        if reference != export or namespace != row["namespace"].encode() or values["flags"][export] != int(row["license"] == "GPL"):
            raise ValueError("metadata export identity/license/namespace differs from owner")
        string_offset = len(strings)
        strings += export+b"\0"+namespace+b"\0"
        table = b"___ksymtab+"+export
        expected[table] = (1,2,width,bytes(width*3))
        expected_reloc[table] = [(0,reloc,export,0), (width,reloc,b"__ksymtab_strings",string_offset),
                                 (width*2,reloc,b"__ksymtab_strings",string_offset+len(export)+1)]
        expected[b"___kflagstab+"+export] = (1,2,1,bytes([values["flags"][export]]))
        if export in values["crcs"]:
            expected[b"___kcrctab+"+export] = (1,2,4,struct.pack("<I",values["crcs"][export]))
    if strings: expected[b"__ksymtab_strings"] = (1,0x32,1,bytes(strings))
    actual, relocations = {}, {}
    for index, section in enumerate(sections):
        label,kind,flags,_,length,_,target,align,_,content = section
        if flags & 4 and length: raise ValueError("module metadata contains executable code")
        if kind in (4,9) and sections[target][2]&2:
            target_name = sections[target][0]
            rows = []
            for offset, rkind, symbol, addend in content:
                symbol_name,_,_,symbol_section,value,_ = symbol
                if symbol_section and symbol_section < len(sections):
                    symbol_name = sections[symbol_section][0]
                    addend += value
                rows.append((offset,rkind,symbol_name,addend))
            relocations.setdefault(target_name, []).extend(rows)
        if flags&2 and (length or label in expected):
            if label == b".note.gnu.property": continue
            if label in actual: raise ValueError("duplicate allocated metadata section")
            actual[label] = (kind,flags,align,content)
    if actual != expected: raise ValueError("selected module metadata allocated payload/layout differs from generated data")
    normalized = {name:sorted(rows) for name,rows in relocations.items() if rows}
    if normalized != {name:sorted(rows) for name,rows in expected_reloc.items() if rows}:
        raise ValueError("metadata lifecycle/export relocations differ from actual binding fields")
    imports = {symbol[0] for symbol in symbols if symbol[0] and symbol[3] == 0}
    if imports != {value.encode() for value in lifecycle.values()} | set(values["exports"]):
        raise ValueError("metadata introduces non-lifecycle/export imports")
    owners = [symbol for symbol in symbols if symbol[0] == b"__this_module"]
    if (len(owners)!=1 or owners[0][1:3]!=(0x11,0) or owners[0][4:]!=(0,size) or
            sections[owners[0][3]][0]!=b".gnu.linkonce.this_module"):
        raise ValueError("metadata lacks exact loader-owned module definition")
    for name,info,other,index,value,length in symbols:
        if name.endswith(b"__IS_RUST_MODULE") or info&15 == 2:
            raise ValueError("metadata contains owner marker or runtime function")
        if index and info>>4 in (1,2) and name!=b"__this_module":
            if (info!=0x11 or other or not name.startswith(b"_RN") or b"module_metadata" not in name or index>=len(sections) or
                    sections[index][0] not in expected or value+length>sections[index][4]):
                raise ValueError("unexpected global module metadata definition")
            section = sections[index]
            families = {b"_____VERSIONS":b"__versions", b"_____VERSION_EXT_CRCS":b"__version_ext_crcs",
                        b"_____VERSION_EXT_NAMES":b"__version_ext_names"}
            matched = [target for suffix,target in families.items() if name.endswith(suffix)]
            if matched:
                if section[0]!=matched[0] or value or length!=section[4]:
                    raise ValueError("generated version symbol does not own its exact array")
            elif (not re.search(rb"___MODULE_INFO_[0-9]+$",name) or section[0]!=b".modinfo" or
                    section[-1][value:value+length] not in values["info"]):
                raise ValueError("unexpected generated metadata global family")


def verify_module_metadata(build, module, *, work=None, require_c_suppression=False,
                           c_flags=None, c_exports=None):
    """Validate one selected .mod.o and return its original frontend input."""
    from check_div64_kernel import verify_build_command
    from rust_exports_test_support import read_exports
    config = configuration(build)
    source, obj = metadata_source(build,module), module.with_suffix(".mod.o")
    work = work or command_root(obj)
    if config.get("RUST_MODULE_METADATA") != "y":
        if require_c_suppression:
            flags = (c_flags or command)(obj)
            if "-D__DISABLE_EXPORTS" not in flags or "-U__DISABLE_EXPORTS" in flags or (c_exports or read_exports)(obj):
                raise ValueError("C module metadata suppression missing or contains owner exports")
        return source
    if config.get("RUST") != "y" or config.get("MODULES") != "y":
        raise ValueError("Rust module metadata requires Rust and modules")
    data = module.with_suffix(".mod.rs")
    flags = command(obj)
    cfg = build / "include/generated/rustc_cfg"
    cfgs = [flag[1:] for flag in flags if flag.startswith("@")]
    if (f"MODULE_METADATA_DATA={data.resolve()}" not in flags or "--crate-name=module_metadata" not in flags or
            "--crate-type=rlib" not in flags or "-Zallow-features=" not in flags or
            len(cfgs)!=1 or (work/cfgs[0]).resolve()!=cfg.resolve() or
            "-Zsanitizer=kcfi" in flags or any("rust/helpers" in flag for flag in flags) or
            "-Cmetadata="+str(obj.relative_to(work)) not in flags or "--extern" not in flags or
            not any(flags[index:index+2]==["--cfg","MODULE"] for index in range(len(flags)-1)) or
            not {"kernel","pin_init"} <= set(flags)):
        raise ValueError("wrong selected Rust module metadata compilation mode")
    dependencies = [data,HEADER,*[build/"rust"/("lib"+name+".rmeta") for name in ("core","kernel","bindings","pin_init")]]
    verify_build_command(work,obj,SOURCE,dependencies)
    verify_build_command(work,data,source,[ROOT/"include/linux/module.h",ROOT/"include/linux/export-internal.h"])
    generated_flags = command(data)
    if ("-fsyntax-only" not in generated_flags or "-DLUPOS_RUST_MODULE_RECORDS" not in generated_flags or
            "--rust-module-records" not in generated_flags or "-D__DISABLE_EXPORTS" not in generated_flags or
            "-U__DISABLE_EXPORTS" in generated_flags):
        raise ValueError("generated metadata bypasses original frontend/record validation")
    # fixdep above checks the exact recorded CONFIG_* dependencies. A change
    # to an unrelated option updates auto.conf without regenerating identical
    # metadata; auto.conf itself is not a prerequisite of this data rule.
    newer(data,[source,build/"scripts/mod/modpost-rust"])
    newer(obj,[cfg,*dependencies])
    newer(module,[obj,module.with_suffix(".o"),data,source])
    values = generated_values(data)
    verify_object(obj,module.with_suffix(".o"),config,values)
    from check_reciprocal_kernel import module_elf
    original=module_elf(obj)
    linked=module_elf(module)
    def exact_section(parsed,name):
        found=[section for section in parsed[1] if section[0]==name]
        if len(found)!=1: raise ValueError("missing or duplicate linked module metadata section")
        section=found[0]
        return section[1],section[2],section[7],section[-1]
    for name in [b".gnu.linkonce.this_module",*[name.encode() for name in values["sections"]]]:
        if exact_section(original,name)!=exact_section(linked,name):
            raise ValueError("final module changed selected owner/version metadata")
    def lifecycle(parsed):
        return sorted((offset,kind,symbol[0],addend) for section in parsed[1]
                      if section[1]==4 and parsed[1][section[6]][0]==b".gnu.linkonce.this_module"
                      for offset,kind,symbol,addend in section[-1])
    if lifecycle(original)!=lifecycle(linked):
        raise ValueError("final module changed selected lifecycle relocations")
    fields=exact_section(linked,b".modinfo")[-1].split(b"\0")
    for value in values["info"]:
        if fields.count(value[:-1])!=values["info"].count(value):
            raise ValueError("final module lost or duplicated generated modinfo")
    def labeled(parsed,label):
        found=[symbol for symbol in parsed[2] if symbol[0]==label and symbol[3]]
        if len(found)!=1: raise ValueError("missing or duplicate final export label")
        symbol=found[0]; section=parsed[1][symbol[3]]
        return symbol[3],symbol[4]-section[3],section
    for name,(reference,namespace) in values["exports"].items():
        for prefix,length in ((b"__ksymtab_",12 if config.get("HAVE_ARCH_PREL32_RELOCATIONS")=="y" else 24),
                              (b"__flags_",1),*([(b"__crc_",4)] if name in values["crcs"] else [])):
            _,at,source_section=labeled(original,prefix+name)
            target_index,to,target_section=labeled(linked,prefix+name)
            if source_section[-1][at:at+length]!=target_section[-1][to:to+length]:
                raise ValueError("final module changed export table bytes")
            if prefix==b"__ksymtab_":
                width=length//3
                kind=(261 if config.get("ARM64")=="y" else 2) if width==4 else (257 if config.get("ARM64")=="y" else 1)
                relocs={row[0]-to:row for section in linked[1] if section[1]==4 and section[6]==target_index
                        for row in section[-1] if to<=row[0]<to+length}
                if (set(relocs)!={0,width,width*2} or any(row[1]!=kind for row in relocs.values()) or
                        relocs[0][2][0]!=reference or relocs[0][3]!=0):
                    raise ValueError("final module export no longer references its actual implementation")
                for offset,text in ((width,name),(width*2,namespace)):
                    _,_,symbol,addend=relocs[offset]
                    if not 0<symbol[3]<len(linked[1]): raise ValueError("final export string is unresolved")
                    section=linked[1][symbol[3]]; position=symbol[4]-section[3]+addend
                    if position<0 or section[-1][position:position+len(text)+1]!=text+b"\0":
                        raise ValueError("final export name/namespace differs from selected metadata")
    def properties(path):
        return [(section[1],section[2],section[7],section[-1])
                for section in module_elf(path)[1] if section[0]==b".note.gnu.property"]
    if properties(obj)!=properties(build/".module-common.o"):
        raise ValueError("module metadata compiler feature note differs from the target")
    # Reuse the strict loader-facing import checker without introducing a
    # module import cycle at import time. It validates all strong imports,
    # resolved weak imports and implicit module_layout against Module.symvers.
    from check_prime_numbers_kernel import verify_module_import_versions
    versions = verify_module_import_versions(build,module)
    if config.get("MODVERSIONS") == "y":
        if set(values["crcs"]) != set(values["exports"]): raise ValueError("metadata lacks exported CRCs")
        rows = [line.split() for line in (build/"Module.symvers").read_bytes().splitlines()]
        for name,crc in values["crcs"].items():
            matches = [row for row in rows if len(row)>=2 and row[1]==name]
            if len(matches)!=1 or int(matches[0][0],16)!=crc: raise ValueError("metadata export CRC differs from owner")
        # Exact object arrays were checked above; final ELF imports must carry
        # all those generated values rather than only the caller's two APIs.
        raw = values["sections"].get("__version_ext_names")
        if raw is not None:
            names = raw.split(b"\0")
            if names[-2:] != [b"",b""] or any(not name for name in names[:-2]):
                raise ValueError("generated extended versions lack exact final NUL")
            crcs = values["sections"].get("__version_ext_crcs",b"")
            expected = list(struct.iter_unpack("<I",crcs))
            if len(expected)!=len(names)-2 or any(versions.get(name)!=crc[0] for name,crc in zip(names[:-2],expected)):
                raise ValueError("final imports differ from generated metadata versions")
        else:
            basic=values["sections"].get("__versions",b"")
            for offset in range(0,len(basic),64):
                crc=struct.unpack_from("<Q",basic,offset)[0]
                name=basic[offset+8:offset+64].split(b"\0",1)[0]
                if versions.get(name)!=crc: raise ValueError("final basic imports differ from generated metadata")
    return source
