#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Validate Cmdline's actual C/Rust ABI and selected original KUnit in QEMU.

Only disposable external fixtures are generated; never load host modules.
The fifth global parse_option_str is audited but never imported by a module.
Requires strict KCFI/MODVERSIONS x86-64 or ARM64. All original C algorithms,
native bindings, active module metadata and import versions stay real.
"""
import argparse
import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import sys
import tempfile

from cmdline_runtime_fixtures import caller_source, marker, reference_source, case_count
from boot_kernel import MARKER, module_name, verify_module_events
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_prime_numbers_kernel import provider_type_ids, verify_guarded_calls, verify_module_import_versions, verify_rust_entrypoints
from check_int_log_kernel import normalize_console_transport, kunit_runs as int_log_runs, verify_kunit_warnings
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_rust_exports_bridge import version_records
from check_reciprocal_kernel import module_elf
from rust_exports_test_support import read_exports

ROOT = Path(os.environ.get("CMDLINE_SOURCE_ROOT", Path(__file__).resolve().parents[2]))
SYMBOLS = ("get_option", "get_options", "memparse", "next_arg")
WRAPPERS = tuple(("cmdline_call_" + name, name) for name in SYMBOLS)
CASES = tuple(("cmdline_test_" + name).encode() for name in ("noint", "lead_int", "tail_int", "range",
    "next_arg_quoted_value", "next_arg_bare_quote_regression", "next_arg_mixed_tokens", "memparse"))
DESCRIPTION = b"Independent cmdline native ABI and parsing test"


def states(config):
    suite = config.get("CMDLINE_KUNIT_TEST", "n")
    framework = config.get("KUNIT", "n")
    if suite not in ("n", "y", "m") or framework not in ("n", "y", "m"):
        raise ValueError("invalid Cmdline/KUnit tristate")
    if suite != "n" and (framework == "n" or suite == "y" and framework == "m"):
        raise ValueError("Cmdline suite requires compatible KUnit framework")
    return suite, framework


def selected_versions(build):
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    if any(len(row)>1 and row[1]==b"parse_option_str" for row in rows):
        raise ValueError("parse_option_str must not be exported")
    result = {}
    for name in SYMBOLS:
        found = [row for row in rows if len(row) >= 2 and row[1] == name.encode()]
        if (len(found) != 1 or len(found[0]) != 4 or found[0][2:] != [b"vmlinux", b"EXPORT_SYMBOL"] or
                not re.fullmatch(rb"0x[0-9a-fA-F]{8}", found[0][0])):
            raise ValueError("wrong Cmdline export/license/provider/CRC: " + name)
        result[name.encode()] = found[0][0].lower()
    return result


def verify_bindings(build):
    generated = build / 'rust/bindings/bindings_generated.rs'
    helper = ROOT / 'rust/bindings/bindings_helper.h'
    headers = [ROOT / 'include/linux/kernel.h', ROOT / 'include/linux/string.h']
    generated_deps = verify_external_command(build, generated, helper, headers)
    # fixdep records the CONFIG stamps that actually affect C preprocessing.
    # An unrelated provider toggle need not regenerate this text, even though
    # .config/autoconf.h/rustc_cfg and Rust metadata are regenerated.
    newer(generated, [*generated_deps, ROOT / 'rust/bindgen_parameters'])
    obj = build / 'rust/bindings.o'
    deps = verify_external_command(build, obj, ROOT / 'rust/bindings/lib.rs', [generated])
    outputs = [part.split('=', 1)[1] for arg in compilation_flags(obj) if arg.startswith('--emit=')
               for part in arg[7:].split(',') if part.startswith('metadata=')]
    metadata = build / 'rust/libbindings.rmeta'
    if len(outputs) != 1 or (build / outputs[0]).resolve() != metadata.resolve():
        raise ValueError('actual bindings command must emit selected metadata')
    newer(obj, [build / 'include/generated/rustc_cfg'])
    newer(metadata, [*deps, *headers, build / 'include/generated/rustc_cfg'])
    newer(build / "rust/libkernel.rmeta", [metadata])


def dwarf_versions(build, owner, names, *, shapes=None):
    if configuration(build).get("GENDWARFKSYMS") != "y":
        raise ValueError("cmdline requires actual GENDWARFKSYMS type proof")
    protected=[build.resolve(),ROOT.resolve()]
    parent=next((p.resolve() for p in [Path(os.environ.get('TMPDIR','/tmp')),Path('/tmp')]
                 if p.is_dir() and not any(p.resolve().is_relative_to(x) for x in protected)),None)
    if parent is None: raise ValueError('no external DWARF proof directory')
    with tempfile.TemporaryDirectory(prefix="cmdline-dwarf-",dir=parent) as directory:
        types=Path(directory)/"types"
        result = subprocess.run([str(build / "scripts/gendwarfksyms/gendwarfksyms"), "--symtypes", str(types), str(owner)],
            input=b"".join(name.encode()+b"\n" for name in names), cwd=directory,
            env=dict(os.environ,TMPDIR=directory,TEMP=directory,TMP=directory),capture_output=True, check=True, timeout=180)
        if shapes is not None:
            for line in types.read_text().splitlines():
                name,body=line.split(" ",1)
                if name in names: shapes[name]=abi_shape(body)
    rows = [line.split() for line in result.stdout.splitlines()]
    if result.stderr or len(rows)!=len(names) or any(len(row)!=3 or row[0]!=b"#SYMVER" for row in rows):
        raise ValueError("malformed cmdline DWARF version proof")
    versions = {row[1]: row[2].lower() for row in rows}
    if set(versions)!={name.encode() for name in names}: raise ValueError("wrong cmdline DWARF symbol set")
    return versions


def abi_shape(body):
    """Compare calling-layout types, never manufacture or substitute a CRC.

    Rust spells pointers/base types differently and DWARF uses unsigned (7)
    rather than unsigned-char (8). Const qualification is not a calling-layout
    distinction. Widths, pointer nesting, argument order and signedness remain.
    """
    body=re.sub(r"pointer_type \*(?:const|mut) [^{]+\{", "pointer_type {", body)
    body=re.sub(r"base_type [^{}]*?byte_size", "base_type byte_size", body)
    body=re.sub(r"const_type \{ (base_type [^{}]+) \}",r"\1",body)
    body=body.replace("encoding(8)","encoding(7)")
    return re.sub(r" (?:str_?|pint|nints|ints|ptr|retptr|args|param|val)(?= ,| \))","",body)


def verify_workload(obj):
    imports = {row.split()[-1] for row in tool("nm", "-u", obj).splitlines() if row.split()}
    expected = {name.encode() for name in SYMBOLS} | {("cmdline_reference_"+name).encode() for name in SYMBOLS}
    if not expected <= imports or imports-expected-{b"_printk", b"memcpy", b"memset", b"memcmp", b"bcmp", b"strlen", b"__stack_chk_fail", b"__x86_return_thunk", b"__fentry__"}:
        raise ValueError("cmdline caller missing imports or delegating workload: "+repr(sorted(imports)))


def verify_external_command(cwd,obj,source,required):
    """Verify a command against its actual Kbuild compiler working directory.

    Apply the same fixdep/freshness checks as verify_build_command, retaining
    actual saved paths instead of rewriting an input .cmd file. In-tree
    commands run in O=; external module commands run in their M= directory.
    """
    data=obj.with_name('.'+obj.name+'.cmd').read_text().replace('\\\n',' ')
    fields={}
    for kind in ('savedcmd','source','deps'):
        rows=re.findall(r'(?m)^'+kind+r'_([^\n]+?) := ([^\n]*)',data)
        if len(rows)!=1 or (cwd/rows[0][0]).resolve()!=obj.resolve():
            raise ValueError('wrong external saved-command target')
        fields[kind]=rows[0][1]
    tokens=shlex.split(fields['source'])
    if len(tokens)!=1 or (cwd/tokens[0]).resolve()!=source.resolve(): raise ValueError('wrong external source')
    if source.resolve() not in {(cwd/p).resolve() for p in shlex.split(fields['savedcmd'])}:
        raise ValueError('external command does not compile verified source')
    dependencies=fields['deps']; paths={source.resolve()}
    for pattern in re.findall(r'\$\(wildcard ([^()]*)\)',dependencies):
        for item in shlex.split(pattern):
            import glob
            paths.update(Path(p).resolve() for p in glob.glob(str(cwd/item)))
    dependencies=re.sub(r'\$\(wildcard [^()]*\)','',dependencies)
    if '$' in dependencies: raise ValueError('unsupported external dependency expression')
    paths.update((cwd/p).resolve() for p in shlex.split(dependencies))
    if not {p.resolve() for p in required}<=paths: raise ValueError('missing external build dependencies')
    newer(obj,paths)
    return paths


def verify_library_owner(build, owner):
    """Kbuild sorts lib-y before archiving; source-list adjacency is not retained."""
    ordered = [(build / os.fsdecode(line)).resolve() for line in tool('ar', 't', build/'lib/lib.a').splitlines()]
    owners = {(build/'lib/cmdline.o').resolve(), (build/'lib/cmdline_rust.o').resolve()}
    if [path for path in ordered if path in owners] != [owner.resolve()]:
        raise ValueError('cmdline original lib/lib.a ownership differs')
    relative = [str(path.relative_to((build/'lib').resolve())) for path in ordered]
    if relative != sorted(set(relative)):
        raise ValueError('cmdline lib-y archive is not uniquely Kbuild-sorted')


def verify_strict_lints(flags):
    levels={'A':'allow','W':'warn','D':'deny','F':'forbid'}
    tracked={'warnings':None,'unsafe_op_in_unsafe_fn':None};cap=None
    tokens=iter(flags)
    for token in tokens:
        level,value=None,None
        if token in ('--cap-lints','--force-warn','--allow','--warn','--deny','--forbid'):
            level,value=token[2:],next(tokens,None)
        elif any(token.startswith('--'+name+'=') for name in ('cap-lints','force-warn','allow','warn','deny','forbid')):
            level,value=token[2:].split('=',1)
        elif len(token)>=2 and token[0]=='-' and token[1] in levels:
            level,value=levels[token[1]],token[2:] or next(tokens,None)
        if level is None: continue
        if not value or value.startswith('='): raise ValueError('malformed Rust lint option')
        if level=='cap-lints':
            if value not in ('allow','warn','deny','forbid'): raise ValueError('unsupported lint cap')
            if cap is None: cap=value  # rustc uses the first cap, not the last.
        elif level=='force-warn': raise ValueError('force-warn bypasses strict diagnostics')
        else:
            # Rust 1.85 treats comma spellings as ONE unknown lint, not a list.
            name=value.replace('-','_')
            if name in tracked and tracked[name]!='forbid': tracked[name]=level
    if any(value not in ('deny','forbid') for value in tracked.values()) or cap not in (None,'deny','forbid'):
        raise ValueError('Rust caller must retain effective strict diagnostics')


def response_flags(flags, base, active=()):
    result=[]
    for flag in flags:
        if flag.startswith('@'):
            path=(base/flag[1:]).resolve()
            if path in active: raise ValueError('recursive Rust response file')
            result.extend(response_flags(path.read_text().splitlines(),base,(*active,path)))
        else: result.append(flag)
    return result


def verify_rust_compiler(flags, build=None):
    verify_strict_lints(response_flags(flags,build or Path.cwd()))
    command=flags.copy()
    while command and '=' in command[0] and not command[0].startswith('-'): command.pop(0)
    if not command: raise ValueError('missing actual Rust compiler')
    output=subprocess.run([command[0],'--version'],capture_output=True,check=True).stdout
    version=re.search(rb'rustc (\d+)\.(\d+)\.(\d+)',output)
    if not version or tuple(map(int,version.groups()))<(1,85,0): raise ValueError('requires Rust >=1.85')


def verify_source(root):
    source = (root / "lib/tests/cmdline_kunit.c").read_bytes()
    if re.findall(rb"KUNIT_CASE\((\w+)\)", source)!=list(CASES):
        raise ValueError("original cmdline KUnit cases changed")
    if not re.search(rb'\.name\s*=\s*"cmdline"',source): raise ValueError("wrong original suite")
    case_count(root)
    original = (root / "lib/cmdline.c").read_text()
    if re.findall(r'^EXPORT_SYMBOL\((\w+)\);',original,re.M)!=list(SYMBOLS):
        raise ValueError("original unrestricted export set changed")


def verify_fixture_sources(work,caller):
    main='cmdline_'+('rust' if caller=='rust' else 'c')+'_main'
    source=work/(main+('.rs' if caller=='rust' else '.c'))
    if source.read_text()!=caller_source(ROOT,caller): raise ValueError('cmdline caller source changed')
    if (work/'cmdline_reference.c').read_text()!=reference_source(ROOT): raise ValueError('original C oracle source changed')


def kunit_layout(obj):
    """Read actual compiler DWARF, not hand-redeclared KUnit structures."""
    dies=[]; current=None
    for line in tool('readelf', '--debug-dump=info', obj).decode().splitlines():
        entry=re.match(r'\s*<(\d+)><([0-9a-f]+)>:.*?(?:\((DW_TAG_\w+)\))?$', line)
        if entry:
            current={'depth':int(entry[1]), 'tag':entry[3], 'attrs':{}}
            dies.append(current)
        else:
            attr=re.match(r'\s*<[0-9a-f]+>\s+DW_AT_(\w+)\s*:\s*(.*)', line)
            if attr and current is not None: current['attrs'][attr[1]]=attr[2].rsplit(': ',1)[-1]
    result={}
    for name, wanted in (('kunit_suite', ('name','test_cases')), ('kunit_case', ('run_case','name','module_name'))):
        found=[]
        for at, die in enumerate(dies):
            if die['tag']!='DW_TAG_structure_type' or die['attrs'].get('name')!=name or 'byte_size' not in die['attrs']: continue
            fields={}
            for child in dies[at+1:]:
                if child['depth']<=die['depth']: break
                if child['depth']==die['depth']+1 and child['tag']=='DW_TAG_member':
                    attrs=child['attrs']
                    if attrs.get('name') in wanted: fields[attrs['name']]=int(attrs['data_member_location'],0)
            if set(fields)!=set(wanted): raise ValueError('missing actual KUnit layout fields')
            found.append((int(die['attrs']['byte_size'],0),fields))
        if not found or any(layout!=found[0] for layout in found): raise ValueError('missing/ambiguous actual KUnit DWARF layout')
        result[name]=found[0]
    return result


def kunit_callback_type(build, arch):
    """Use a real framework void(struct kunit *) definition as the type witness."""
    obj=build/'lib/kunit/test.o'; source=ROOT/'lib/kunit/test.c'; header=ROOT/'include/kunit/test.h'
    for path in (source, header):
        if not re.search(r'void\s+kunit_cleanup\(struct kunit \*test\)',path.read_text()):
            raise ValueError('actual KUnit callback witness prototype changed')
    verify_build_command(build,obj,source,[header])
    elf_target(obj,arch)
    flags=compilation_flags(obj)
    if '-fsanitize=kcfi' not in flags or any(arg.startswith('-fno-sanitize=') for arg in flags):
        raise ValueError('actual KUnit callback witness lacks KCFI')
    return provider_type_ids(obj,names=('kunit_cleanup',))['kunit_cleanup']


def verify_kunit_graph(obj, arch, callback_type):
    """Follow registration -> named suite -> ordered cases -> real callbacks."""
    elf_target(obj,arch)
    _,sections,symbols,_,_=module_elf(obj)
    layout=kunit_layout(obj)
    suite_size,suite_fields=layout['kunit_suite']; case_size,case_fields=layout['kunit_case']
    def named(name):
        matches=[s for s in symbols if s[0]==name and s[3] and s[3]<len(sections)]
        if len(matches)!=1: raise ValueError('missing/duplicate KUnit symbol '+repr(name))
        return matches[0]
    def relocations(section):
        return [r for table in sections if table[1]==4 and table[6]==section for r in table[9]]
    def target(section,offset):
        rows=[r for r in relocations(section) if r[0]==offset]
        if len(rows)!=1 or rows[0][1]!=(257 if arch=='aarch64' else 1): raise ValueError('wrong KUnit pointer relocation')
        _,_,symbol,addend=rows[0]
        if not 0<symbol[3]<len(sections): raise ValueError('KUnit pointer has no actual definition')
        position=symbol[4]+addend
        if position<0 or position>=sections[symbol[3]][4]: raise ValueError('KUnit pointer outside section')
        return symbol[3],position
    def payload(section,offset,length):
        data=sections[section][9]
        if not isinstance(data,bytes) or offset<0 or offset+length>len(data): raise ValueError('truncated KUnit data')
        return data[offset:offset+length]
    registration=[(i,s) for i,s in enumerate(sections) if s[0]==b'.kunit_test_suites']
    if (len(registration)!=1 or registration[0][1][1]!=1 or registration[0][1][2]!=3 or
            registration[0][1][4]!=8 or registration[0][1][7]!=8 or
            any(registration[0][1][9]) or len(relocations(registration[0][0]))!=1):
        raise ValueError('KUnit registration must contain exactly one suite')
    suite=named(b'cmdline_test_suite'); cases=named(b'cmdline_test_cases')
    if target(registration[0][0],0)!=(suite[3],suite[4]) or suite[5]!=suite_size:
        raise ValueError('registration does not point to actual named suite')
    if payload(suite[3],suite[4]+suite_fields['name'],8)!=b'cmdline\0': raise ValueError('wrong suite name')
    if target(suite[3],suite[4]+suite_fields['test_cases'])!=(cases[3],cases[4]): raise ValueError('suite does not point to named case array')
    if sections[suite[3]][2]!=3 or sections[cases[3]][2]!=3: raise ValueError('suite/cases lack writable allocated storage')
    if cases[5]!=(len(CASES)+1)*case_size: raise ValueError('wrong case array length')
    module_names=set()
    for i,name in enumerate(CASES):
        offset=cases[4]+i*case_size
        function=named(name)
        if function[1]&15!=2 or not sections[function[3]][2]&4: raise ValueError('callback is not an actual function')
        if target(cases[3],offset+case_fields['run_case'])!=(function[3],function[4]): raise ValueError('wrong ordered callback graph')
        section,position=target(cases[3],offset+case_fields['name'])
        if payload(section,position,len(name)+1)!=name+b'\0': raise ValueError('wrong ordered case name')
        section,position=target(cases[3],offset+case_fields['module_name'])
        module_name=sections[section][9][position:].split(b'\0',1)[0]
        if not re.fullmatch(rb'[A-Za-z0-9_]+',module_name): raise ValueError('invalid original KUnit module name')
        module_names.add(module_name)
        if (any(payload(cases[3],offset,case_size)) or
                {r[0]-offset for r in relocations(cases[3]) if offset<=r[0]<offset+case_size}!=set(case_fields.values())):
            raise ValueError('unexpected original KUnit case initializer')
    if len(module_names)!=1: raise ValueError('mixed original KUnit module names')
    end=cases[4]+len(CASES)*case_size
    if any(payload(cases[3],end,case_size)) or any(end<=r[0]<end+case_size for r in relocations(cases[3])):
        raise ValueError('case array lacks zero sentinel')
    types=provider_type_ids(obj,names=tuple(name.decode() for name in CASES))
    if not callback_type or set(types.values())!={callback_type}: raise ValueError('KUnit callback prototype/CFI differs from actual framework')
    return layout


def verify_linked_implementation(build, selection):
    verify_source(ROOT)
    config = configuration(build)
    arch = architecture(config)
    suite, framework = states(config)
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_CMDLINE") == "y" else "C"):
        raise ValueError("Cmdline selection/config mismatch")
    if selection == "Rust" and config.get("RUST") != "y":
        raise ValueError("Rust Cmdline requires RUST=y")
    if selection == "Rust": verify_bindings(build)
    archive = build / "vmlinux.a"
    lines = tool("ar", "t", archive).splitlines()
    members = {(build / os.fsdecode(line)).resolve() for line in lines}
    if len(members) != len(lines): raise ValueError("duplicate linked archive members")
    original, translated = [(build / "lib" / (name + ".o")).resolve() for name in ("cmdline", "cmdline_rust")]
    owner = translated if selection == "Rust" else original
    if members & {original, translated} != {owner}:
        raise ValueError("linked Cmdline owner is absent, mixed or wrong")
    library = build / "lib/lib.a"
    verify_library_owner(build, owner)
    elf_target(owner, arch)
    source = ROOT / "lib" / (owner.stem + (".rs" if selection == "Rust" else ".c"))
    dependencies = ([ROOT / path for path in ("lib/cmdline.rs", "rust/ffi_export.rs", "include/linux/export_header.rs")]
        + [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust"
        else [ROOT / "include/linux/kernel.h"])
    verify_build_command(build, owner, source, dependencies)
    owner_flags=compilation_flags(owner)
    if ('-Zsanitizer=kcfi' if selection=='Rust' else '-fsanitize=kcfi') not in owner_flags:
        raise ValueError('selected owner lacks native KCFI')
    if selection=='Rust': verify_rust_compiler(owner_flags,build)
    if selection == "Rust" and "RUST_MODFILE=lib/cmdline" not in compilation_flags(owner):
        raise ValueError("cmdline Rust module ownership changed")
    for name in ("vsprintf", "kstrtox"):
        obj = build / "lib" / (name+".o")
        if obj.resolve() not in members: raise ValueError("original C numeric dependency absent")
        verify_build_command(build,obj,ROOT / "lib" / (name+".c"),[])
        newer(archive,[obj])
    records = read_exports(owner)
    if sorted(record["name"] for record in records) != sorted(SYMBOLS):
        raise ValueError("wrong Cmdline owner export set")
    definitions = [line.split() for line in tool("nm", "--defined-only", owner).splitlines() if line.split()]
    fifth = [row for row in definitions if row[-1]==b"parse_option_str"]
    if len(fifth)!=1 or fifth[0][-2]!=b"T": raise ValueError("fifth cmdline global linkage lost")
    for record in records:
        expected = dict(license="", namespace="", relocation_target=record["name"], relocation_addend=0,
            pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1, label_binding=0,
            label_kind=0, section_flags=2, section_alignment=8)
        if any(record[key] != value for key, value in expected.items()):
            raise ValueError("wrong Cmdline export record/relocation")
        found = [row for row in definitions if row[-1] == record["name"].encode()]
        if len(found) != 1 or found[0][-2] != b"T": raise ValueError("Cmdline export lacks actual global definition")
    versions = selected_versions(build)
    if version_records(owner.with_name("." + owner.name + ".cmd")) != versions:
        raise ValueError("Cmdline CRC provenance does not match defining owner")
    if dwarf_versions(build,owner,SYMBOLS)!=versions: raise ValueError("actual owner DWARF CRC differs from symvers")
    provider_type_ids(owner, names=(*SYMBOLS,"parse_option_str"))
    newer(library,[owner])
    tests = (build / "lib/tests/cmdline_kunit.o").resolve()
    if (tests in members) != (suite == "y"):
        raise ValueError("Cmdline KUnit archive membership mismatch")
    preloads = []
    if suite != "n":
        elf_target(tests, arch)
        verify_build_command(build, tests, ROOT / "lib/tests/cmdline_kunit.c",
                             [ROOT / "include/linux/kernel.h", ROOT / "include/kunit/test.h"])
        verify_references(tests, SYMBOLS)
        verify_kunit_graph(tests,arch,kunit_callback_type(build,arch))
        info = metadata_fields(tests)
        prefix = b"cmdline_kunit." if suite == "y" else b""
        for key, value in ((b"license", b"GPL"), (b"description", b"Test cases for API provided by cmdline.c")):
            require_metadata_field(info, prefix + key, value)
        if framework == "m": preloads.append(verify_framework_module(build, members, arch))
        else:
            for name in ("test", "assert"):
                obj = build / "lib/kunit" / (name + ".o")
                if obj.resolve() not in members: raise ValueError("built-in KUnit framework missing")
                verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"), [])
                newer(archive, [obj])
        if suite == "m":
            module = tests.with_suffix(".ko")
            verify_module(build, module, tests, "cmdline_kunit", arch)
            newer(module, [selected_metadata(build, module)])
            verify_references(module, SYMBOLS)
            verify_module_import_versions(build, module)
            preloads.append(module)
    newer(build / "Module.symvers", [owner, *([tests] if suite != "n" else [])])
    newer(archive, [owner, *([tests] if suite == "y" else [])])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return preloads


def verify_consumer(build, work, caller):
    verify_fixture_sources(work,caller)
    config = configuration(build)
    arch = architecture(config)
    main = "cmdline_" + ("rust" if caller == "rust" else "c") + "_main"
    stem = "cmdline_rust_abi" if caller == "rust" else "cmdline_abi"
    obj, module, reference = work / (main + ".o"), work / (stem + ".ko"), work / "cmdline_reference.o"
    for path in (obj, module, reference): elf_target(path, arch)
    if module_name(module) != stem: raise ValueError("wrong Cmdline consumer module identity")
    source = work / (main + (".rs" if caller == "rust" else ".c"))
    required = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust"
                else [ROOT / "include/linux/kernel.h"])
    verify_external_command(work, obj, source, required)
    verify_workload(obj)
    verify_external_command(work, reference, work / "cmdline_reference.c", [ROOT / "lib/cmdline.c", ROOT / "include/linux/kernel.h"])
    flags = compilation_flags(obj)
    if caller=='rust': verify_rust_compiler(flags,work)
    required_flag = "-Zsanitizer=kcfi" if caller == "rust" else "-fsanitize=kcfi"
    if required_flag not in flags or any(flag.startswith(("-fno-sanitize=", "-Zsanitizer=")) and
        (flag.startswith("-fno-sanitize=") and {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")) or
         flag.startswith("-Zsanitizer=") and flag != "-Zsanitizer=kcfi") for flag in flags):
        raise ValueError("Cmdline caller KCFI disabled or absent")
    provider = build / "lib" / ("cmdline_rust.o" if config.get("RUST_CMDLINE") == "y" else "cmdline.o")
    types = provider_type_ids(provider, names=SYMBOLS)
    ref_types = provider_type_ids(reference, names=tuple("cmdline_reference_" + name for name in SYMBOLS))
    if [types[name] for name in SYMBOLS] != [ref_types["cmdline_reference_" + name] for name in SYMBOLS]:
        raise ValueError("selected Cmdline KCFI identity differs from original C")
    old_shapes,selected_shapes={},{}
    dwarf_versions(build,reference,tuple("cmdline_reference_"+name for name in SYMBOLS),shapes=old_shapes)
    actual_versions=dwarf_versions(build,provider,SYMBOLS,shapes=selected_shapes)
    if actual_versions != selected_versions(build): raise ValueError("provider actual DWARF CRC differs")
    if {name:old_shapes.get("cmdline_reference_"+name) for name in SYMBOLS} != selected_shapes:
        raise ValueError("original C/native provider DWARF ABI shapes differ")
    for image in (obj, module):
        verify_references(image, SYMBOLS)
        verify_guarded_calls(image, arch, types, wrappers=WRAPPERS)
        if caller == "rust": verify_rust_entrypoints(image, arch)
    if any(read_exports(path) for path in (obj, reference, module)):
        raise ValueError("private Cmdline fixture unexpectedly exports symbols")
    for path in (reference, *([obj] if caller == "c" else [])):
        saved = compilation_flags(path)
        if "-D__DISABLE_EXPORTS" not in saved or "-U__DISABLE_EXPORTS" in saved:
            raise ValueError("private Cmdline C fixture retains incidental export metadata")
    generated = selected_metadata(build, module)
    newer(module, [generated, obj, reference])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    versions = verify_module_import_versions(build, module)
    for name, expected in selected_versions(build).items():
        if versions.get(name) != int(expected, 16): raise ValueError("Cmdline caller import CRC mismatch")
    info = metadata_fields(module)
    require_metadata_field(info, b"license", b"Proprietary")
    require_metadata_field(info, b"description", DESCRIPTION)
    imports = {row.split()[-1] for row in tool("nm","-u",module).splitlines() if row.split()}
    rows = [row.split() for row in (build / "Module.symvers").read_bytes().splitlines()]
    if b"parse_option_str" in imports or any(len(row)>=4 and row[1] in imports and row[3]!=b"EXPORT_SYMBOL" for row in rows):
        raise ValueError("private Proprietary caller imports an unexported/GPL-only symbol")


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).removeprefix(b"cmdline_rust_abi: ")
            for line in console.splitlines()]


def kunit_runs(console, runs):
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: cmdline"]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+cmdline(?:\s.*)?", line)]
    if runs not in (0, 1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("missing or duplicate cmdline KUnit suite")
    result = []
    for start, end in zip(starts, ends):
        if start >= end or result and start <= result[-1][1]: raise ValueError("reordered cmdline suite")
        cases, plan, summaries = [], False, set()
        for at in range(start + 1, end + 1):
            line = lines[at]
            if re.match(rb"\d+\.\.", line):
                if line != b"1..8" or plan or cases: raise ValueError("wrong cmdline plan")
                plan = True
            elif re.match(rb"(?:not )?ok\s", line):
                if at == end and re.fullmatch(rb"ok [1-9]\d* cmdline", line):
                    if cases != list(CASES): raise ValueError("premature cmdline completion")
                elif plan and len(cases) < len(CASES) and line == b"ok " + str(len(cases)+1).encode() + b" " + CASES[len(cases)]:
                    cases.append(CASES[len(cases)])
                else: raise ValueError("failed/skipped/duplicate/wrong cmdline case")
            elif re.match(rb"# (?:cmdline|Totals):", line):
                match = re.fullmatch(rb"# (cmdline|Totals): pass:8 fail:0 skip:0 total:8", line)
                if not match or match[1] in summaries or cases != list(CASES): raise ValueError("wrong cmdline summary")
                summaries.add(match[1])
            elif any(line.startswith(b"# " + case + b": pass:") for case in CASES):
                if len(cases) >= len(CASES): raise ValueError("misplaced case summary")
                name = CASES[len(cases)]
                if line != b"# " + name + b": pass:1 fail:0 skip:0 total:1" or name in summaries:
                    raise ValueError("wrong cmdline case summary")
                summaries.add(name)
            elif line.startswith(b"# Subtest:"): raise ValueError("unexpected cmdline parameterization")
        if not plan or cases != list(CASES) or not {b'cmdline', b'Totals'} <= summaries:
            raise ValueError("incomplete cmdline suite or missing summaries")
        result.append((start, end))
    for i, line in enumerate(lines):
        if (any(case in line for case in CASES) or line.startswith(b"# cmdline:")) and not any(a < i < b for a,b in result):
            raise ValueError("cmdline case outside suite")
    return result


def verify_console(console, caller, *, suite="n", framework_module=False, reload=False, config=None):
    if suite not in ("n", "y", "m") or framework_module and suite != "m":
        raise ValueError("incompatible Cmdline suite/framework state")
    if re.search(rb"CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version", console, re.I):
        raise ValueError("unexpected Cmdline kernel fault/test failure")
    console = normalize_console_transport(console)
    lines = console_lines(console)
    allowed = set()
    if config and config.get("INT_LOG_KUNIT_TEST") == "y":
        allowed = set(verify_kunit_warnings(lines, int_log_runs(console, 1),
            "Rust" if config.get("RUST_INT_LOG") == "y" else "C", config.get("BUG") == "y"))
    if {i for i, line in enumerate(lines) if b"WARNING:" in line} != allowed:
        raise ValueError("warning outside configured original integer-log zero cases")
    ranges = kunit_runs(console, 0 if suite == "n" else 2 if suite == "m" and reload else 1)
    preloads = int(framework_module) + int(suite == "m")
    expected = [b"KUNIT"] if suite == "y" else []
    for i in range(preloads):
        if i == preloads - 1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {i}".encode())
    expected += [marker(ROOT, caller), b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for i in range(preloads):
            if i == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode())
        expected += [marker(ROOT, caller), f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(MARKER)
    actual = []
    for i, line in enumerate(console_lines(console)):
        if any(i == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start <= i <= end for start, end in ranges): raise ValueError("module event inside Cmdline suite")
            actual.append(line)
    if actual != expected: raise ValueError("wrong Cmdline result/count/event order")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return 8 * len(ranges)


def clean_environment():
    return {name: value for name, value in os.environ.items() if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_"))
        and name not in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}


def nonempty_path(value):
    if not value or not value.strip(): raise argparse.ArgumentTypeError("explicit empty path")
    return Path(value)


def executable_command(value, name):
    try:
        command = shlex.split(value)
    except ValueError as error:
        raise ValueError('malformed explicit ' + name + ' command') from error
    if not command or not command[0]: raise ValueError('empty explicit ' + name + ' input')
    if any('\0' in word for word in command): raise ValueError(name + ' command contains NUL')
    executable = shutil.which(command[0])
    if executable is None: raise ValueError('invalid ' + name + ' executable: ' + command[0])
    command[0] = os.path.abspath(executable)
    return command


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=nonempty_path)
    parser.add_argument("--work", required=True, type=nonempty_path)
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--reload-modules", action="store_true")
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=nonempty_path)
    args = parser.parse_args()
    build = args.build.resolve()
    work = args.work.resolve()
    try:
        if work.exists() or work == build or build in work.parents or ROOT.resolve() in work.parents:
            raise ValueError('--work must be a new external directory')
        if "CMDLINE_SOURCE_ROOT" in os.environ: nonempty_path(os.environ["CMDLINE_SOURCE_ROOT"])
        if args.qemu is not None: args.qemu = shlex.join(executable_command(args.qemu, 'QEMU'))
        if any(not value.strip() for value in args.make_arg): raise ValueError("explicit empty make argument")
        if args.qemu_data is not None and not args.qemu_data.is_dir(): raise ValueError("invalid QEMU data directory")
        if args.qemu_data is not None: args.qemu_data = args.qemu_data.resolve()
        make_command=executable_command(os.environ.get("MAKE","make"), 'MAKE')
        verify_source(ROOT)
        config = configuration(build)
        arch = architecture(config)
        if args.qemu is None: args.qemu = shlex.join(executable_command('qemu-system-' + arch, 'QEMU'))
        suite, framework = states(config)
        selection = "Rust" if config.get("RUST_CMDLINE") == "y" else "C"
        if config.get("RUST_CMDLINE","n") not in ("n","y"): raise ValueError("invalid cmdline selector")
        if selection == "C" and not args.allow_c_baseline: raise ValueError("requires RUST_CMDLINE=y or --allow-c-baseline")
        for name in ("MODULES", "PRINTK", "MULTIUSER", "CFI", "MODVERSIONS"):
            if config.get(name) != "y": raise ValueError("requires CONFIG_" + name + "=y")
        if args.caller == "rust" and config.get("RUST") != "y": raise ValueError("Rust caller requires RUST=y")
        if args.reload_modules and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires MODULE_UNLOAD=y")
        if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("unsigned private fixture cannot use forced signatures")
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_bindings(build)
    except (OSError, ValueError, argparse.ArgumentTypeError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work.mkdir(parents=True)
    stem = "cmdline_rust_abi" if args.caller == "rust" else "cmdline_abi"
    source = "cmdline_" + ("rust" if args.caller == "rust" else "c") + "_main"
    (work / "cmdline_reference.c").write_text(reference_source(ROOT))
    (work / (source + (".rs" if args.caller == "rust" else ".c"))).write_text(caller_source(ROOT, args.caller))
    (work / "Makefile").write_text(f"obj-m := {stem}.o\n{stem}-y := {source}.o cmdline_reference.o\n")
    env = clean_environment()
    def make(directory):
        return [*make_command, "-C", str(ROOT), "O=" + str(build),
                "M=" + str(directory), *args.make_arg, "RUSTFLAGS_MODULE+=-Dwarnings", "CFLAGS_MODULE+=-D__DISABLE_EXPORTS", "modules"]
    run_logged(make(work),work/'module-build.log',env)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build), "--arch", arch,
               "--module", str(work / (stem + ".ko")), "--qemu", args.qemu or "qemu-system-" + arch]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    run_logged(command,work/'boot.log',env)
    cases = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
                          suite=suite, framework_module=framework == "m" and suite == "m", reload=args.reload_modules, config=config)
    print(f"{selection} Cmdline/{arch}/{args.caller}: {case_count(ROOT)} workload groups per load; {cases} original KUnit cases; reload={args.reload_modules}")


def run_logged(command,log,env):
    scratch=Path(tempfile.mkdtemp(prefix='command-', dir=log.parent))
    env=dict(env, TMPDIR=str(scratch), TEMP=str(scratch), TMP=str(scratch))
    with log.open('wb') as output:
        output.write((shlex.join(command)+'\ncwd='+str(scratch)+'\n').encode());output.flush()
        try: result=subprocess.run(command,cwd=scratch,env=env,stdout=output,stderr=subprocess.STDOUT)
        except (OSError, subprocess.TimeoutExpired) as error:
            output.write(('launch-error='+repr(error)+'\n').encode())
            raise RuntimeError('command failed to start; preserved '+str(log)) from error
        output.write(f'\nexit={result.returncode}\n'.encode())
    if result.returncode: raise RuntimeError(f'command failed ({result.returncode}); preserved {log}')


if __name__ == "__main__": main()
