#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Final export tables: real original-C/Rust relocations and linked bytes.

Only temporary fixtures are written. EXPORT_INTERNAL_ROOT/SOURCE allow private
pre-integration candidates; normal discovery uses the actual repository header.
Optional genuine cross targets use INT_MATH_I686_SYSROOT and
MODULE_COMMON_{ARM64,POWERPC}_SYSROOT. Explicit unusable tools/targets fail.
This verifies metadata, not target execution or a full architecture kernel.
"""
import ast
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest

ROOT = Path(os.environ.get("EXPORT_INTERNAL_ROOT", Path(__file__).resolve().parents[2]))
SOURCE = Path(os.environ.get("EXPORT_INTERNAL_SOURCE", ROOT / "include/linux/export-internal_header.rs"))
ENV = dict(os.environ, RUSTC_BOOTSTRAP="1", LC_ALL="C", LANGUAGE="C")
ENTRIES = [('function', '', 1, 0x12345678, True),
           ('data', 'NS_Δ', 0, 0xffffffff, False),
           ('dotted.name', 'module:name,name*', 1, 0x80000000, True),
           ('dollar$name', 'braces {safe}', 0, 0, False),
           ('type', 'same', 0xff, 0x01020304, True),
           ('repeat', 'same', 1, 0xdeadbeef, False)]

# Intercept only the final original C macro. The original outer macros and
# target-specific KSYM_FUNC still perform their normal argument expansion.
NORMALIZED_RECORDS = r'''
#undef __KSYMTAB
#define LUPOS_STRINGIFY_REF(sym) #sym
#define __KSYMTAB(name, sym, ns) LUPOS_EXPORT_KSYM #name , LUPOS_STRINGIFY_REF(sym) , ns ;
#undef SYMBOL_CRC
#define SYMBOL_CRC(sym, value) LUPOS_EXPORT_CRC #sym , #value ;
#undef SYMBOL_FLAGS
#define SYMBOL_FLAGS(sym, value) LUPOS_EXPORT_FLAGS #sym , #value ;
'''

NORMALIZED_CASES = [
    ('plain', '', 'KSYMTAB_FUNC(plain, "ns");', ('plain', 'plain', 'ns')),
    ('builtin', '', 'KSYMTAB_DATA(linux, "");', ('1', '1', '')),
    ('chain', '#define NAME NEXT\n#define NEXT target\n',
     'KSYMTAB_FUNC(NAME, "two" " parts");', ('target', 'target', 'two parts')),
    ('redirect', '#undef KSYM_FUNC\n#define KSYM_FUNC(name) alias_ ## name\n',
     'KSYMTAB_FUNC(target, "module:foo,bar*");', ('target', 'alias_target', 'module:foo,bar*')),
    ('prescan', '#undef KSYM_FUNC\n#define KSYM_FUNC(name) REF_ ## name\n#define REF_target resolved\n',
     'KSYMTAB_FUNC(target, "braces {x}");', ('target', 'resolved', 'braces {x}')),
    ('direct', '#define NAME target\n#define NS "prefix" "\\t" "suffix"\n',
     '__KSYMTAB(NAME, KSYM_FUNC(NAME), NS);', ('NAME', 'target', 'prefix\tsuffix')),
    ('expression', '', '__KSYMTAB(expression, (target + 4), "octal\\\\101");',
     ('expression', '(target + 4)', r'octal\101')),
    ('numeric', '#define NAME 123\n', 'KSYMTAB_FUNC(NAME, "");', ('123', '123', '')),
    ('current', '', '__KSYMTAB(current, ., "");', ('current', '.', '')),
    ('ns_macro', '#define JOIN(a,b) a b\n#define NS JOIN("UTF8 λ ","{literal}")\n',
     'KSYMTAB_DATA(target, NS);', ('target', 'target', 'UTF8 λ {literal}')),
]
STRING_TOKEN = r'"(?:\\.|[^"\\])*"'


def normalized_fields(line):
    """Decode only this finite proof's CPP strings, not a production C parser."""
    tail = line.split(' ', 1)[1].strip().rstrip(';').rstrip()
    values = []
    while tail:
        parts = []
        while (match := re.match(STRING_TOKEN, tail)):
            parts.append(ast.literal_eval(match[0]))
            tail = tail[match.end():].lstrip()
        assert parts, line
        values.append(''.join(parts))
        if not tail:
            break
        assert tail.startswith(','), line
        tail = tail[1:].lstrip()
    return values


def original_assembly(preprocessed):
    return [''.join(ast.literal_eval(token) for token in re.findall(STRING_TOKEN, record))
            for record in re.findall(r'asm\((.*?)\)\s*;', preprocessed, re.S)]


def tool(name, default):
    return shlex.split(os.environ.get(name, default))


def run(args, *, failure=False):
    result = subprocess.run(list(map(str, args)), capture_output=True, env=ENV, timeout=120)
    if (result.returncode == 0) == failure:
        raise AssertionError(shlex.join(list(map(str, args))) + "\n" + result.stderr.decode(errors="replace"))
    return result


def c_literal(value):
    return '"' + ''.join(
        '\\\\' if byte == 92 else '\\\"' if byte == 34 else
        chr(byte) if 32 <= byte < 127 else '\\%03o' % byte
        for byte in value.encode()) + '"'


def rust_literal(value):
    return '"' + ''.join(
        '\\\\' if char == '\\' else '\\\"' if char == '"' else
        '\\x%02x' % ord(char) if ord(char) < 32 or ord(char) == 127 else char
        for char in value) + '"'


def elf(path):
    data = path.read_bytes()
    assert data[:4] == b'\x7fELF'
    bits, endian = data[4], '<' if data[5] == 1 else '>'
    wide = bits == 2
    u = lambda fmt, off: struct.unpack_from(endian+fmt,data,off)
    shoff = u('Q' if wide else 'I',40 if wide else 32)[0]
    stride,n,strings = u('HHH',58 if wide else 46)
    sections = [u('IIQQQQIIQQ' if wide else '10I',shoff+i*stride) for i in range(n)]
    raw = lambda s: data[s[4]:s[4]+s[5]]
    string = lambda blob,i: blob[i:blob.index(0,i)]
    section_names = raw(sections[strings])
    names = [string(section_names,s[0]) for s in sections]
    symtabs = {}
    for i,s in enumerate(sections):
        if s[1] != 2: continue
        pool = raw(sections[s[6]])
        syms = []
        for pos in range(s[4],s[4]+s[5],s[9]):
            if wide: name,info,other,idx,value,size = u('IBBHQQ',pos)
            else: name,value,size,info,other,idx = u('IIIBBH',pos)
            syms.append((string(pool,name),info,other,idx,value,size))
        symtabs[i]=syms
    assert len(symtabs)==1
    syms=next(iter(symtabs.values()))
    wanted=lambda name:name==b'__ksymtab_strings' or name.startswith((b'___ksymtab+',b'___kcrctab+',b'___kflagstab+'))
    selected={names[i]:(s[1],s[2],s[8],s[9],raw(s)) for i,s in enumerate(sections) if wanted(names[i])}
    relocs=[]
    for s in sections:
        if s[1] not in (4,9) or not wanted(names[s[7]]):continue
        table=symtabs[s[6]]
        for pos in range(s[4],s[4]+s[5],s[9]):
            off,info=u('QQ' if wide else 'II',pos)
            number,kind=info>>(32 if wide else 8),info& (0xffffffff if wide else 0xff)
            sym=table[number]
            target=sections[s[7]]
            pointer_size=target[5]//3
            assert pointer_size in (4,8)
            addend=u('q' if wide else 'i',pos+(16 if wide else 8))[0] if s[1]==4 else int.from_bytes(
                raw(target)[off:off+pointer_size], 'little' if endian=='<' else 'big', signed=True)
            # GNU as retains local labels where LLVM MC instead uses the
            # containing section+offset. Normalize only that exact identity;
            # final-link section-byte comparisons below independently verify it.
            if sym[3] and sym[1]>>4==0:
                identity=(b'local',names[sym[3]])
                addend+=sym[4]
            else:identity=(b'external',sym[0],sym[1],sym[2],sym[3])
            item=list(selected[names[s[7]]])
            masked=bytearray(item[-1]);masked[off:off+pointer_size]=bytes(pointer_size)
            item[-1]=bytes(masked);selected[names[s[7]]]=tuple(item)
            relocs.append((names[s[7]],off,kind,identity,addend))
    labels=sorted((name,info,other,names[idx],value,size) for name,info,other,idx,value,size in syms
                  if name.startswith((b'__ksymtab_',b'__kstrtab_',b'__kstrtabns_',b'__crc_',b'__flags_')))
    undefined={name for name,info,_,idx,_,_ in syms if not idx and name}
    definitions={name for name,info,_,idx,_,_ in syms if idx and info>>4 in (1,2)}
    return selected, sorted(relocs), labels, undefined, definitions, names

def linked(input, directory, arch):
    defs=directory/'definitions.S'
    text=['.section .owner,"ax"']
    for name,_,_,_,function in ENTRIES:
        text += ['.balign 8','.globl '+name,'.type '+name+',%'+('function' if function else 'object'),
                 name+':','.long 0x12345678','.size '+name+',4']
    defs.write_text('\n'.join(text)+'\n')
    target={'x86_64':'x86_64-linux-gnu','i686':'i686-linux-gnu','arm64':'aarch64-linux-gnu','powerpc':'powerpc-linux-gnu'}[arch]
    run([*tool('CLANG','clang'),'--target='+target,'-c',defs,'-o',directory/'definitions.o'])
    layout=directory/'layout.lds'
    entries=elf(input)[0]
    sections=['SECTIONS {']
    for index,name in enumerate(entries):
        label=name.decode()
        sections.append(f'  "{label}" 0x{0x10000+0x1000*index:x} : {{ *("{label}") }}')
    sections += ['  .owner 0x100000 : { *(.owner) }','  /DISCARD/ : { *(.debug*) *(.comment) *(.note*) }','}']
    layout.write_text('\n'.join(sections)+'\n')
    output=input.with_suffix('.linked')
    run([*tool('LD_LLD','ld.lld'),'-T',layout,'--entry=function',input,directory/'definitions.o','-o',output])
    result=elf(output)
    assert not result[1] and not result[3]
    return result[0]


class ExportInternalTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        temporary = tempfile.TemporaryDirectory(prefix="export-internal-")
        cls.addClassCleanup(temporary.cleanup)
        cls.work = Path(temporary.name)
        cls.rustc = tool("HOSTRUSTC", "rustc")
        cls.cc = tool("HOSTCC", "cc")
        cls.clang = tool("CLANG", "clang")
        # Explicitly selected broken tools must never become optional skips.
        run([*cls.rustc, "--version"])
        run([*cls.cc, "--version"])
        for variable in ("CLANG", "LD_LLD"):
            if variable in os.environ:
                run([*tool(variable, ""), "--version"])
        cls.cross_tools = all(command and shutil.which(command[0]) for command in
                              (cls.clang, tool("LD_LLD", "ld.lld")))
        if not cls.cross_tools:
            raise unittest.SkipTest("Clang and LLD required for exact final-link metadata proof")
        cls.targets = {}
        for name, target, variable in (
                ("x86_64", "x86_64-unknown-linux-gnu", None),
                ("i686", "i686-unknown-linux-gnu", "INT_MATH_I686_SYSROOT"),
                ("arm64", "aarch64-unknown-linux-musl", "MODULE_COMMON_ARM64_SYSROOT"),
                ("powerpc", "powerpc-unknown-linux-gnu", "MODULE_COMMON_POWERPC_SYSROOT")):
            flags = ["--target=" + target]
            requested = os.environ.get(variable) if variable else None
            if requested:
                flags += ["--sysroot", requested]
            libdir = Path(run([*cls.rustc, *flags, "--print=target-libdir"]).stdout.decode().strip())
            if any(libdir.glob("libcore*.rlib")):
                cls.targets[name] = flags
            elif requested or name == "x86_64":
                raise AssertionError(f"explicit/required {name} target has no matching core: {libdir}")
        cls.include = cls.work / "include/linux"
        cls.include.mkdir(parents=True)
        # The actual macro header needs no facilities from these two headers.
        (cls.include / "compiler.h").write_text("/* unused compiler plumbing */\n")
        (cls.include / "types.h").write_text("/* no external type declarations */\n")

    def rustflags(self, arch, prel, optimize):
        flags = [*self.targets[arch], "--edition=2021", "--crate-type=rlib", "--crate-name=metadata",
                 "-Cpanic=abort", "-Copt-level=" + optimize, "-Cdebuginfo=2", "-Zdwarf-version=5",
                 "-Dwarnings", "-Wmissing-docs", "-Wrust-2018-idioms", "-Wunreachable-pub"]
        if arch == "powerpc":
            flags += ["-Zcrate-attr=feature(asm_experimental_arch)"]
        if arch in ("x86_64", "arm64"):
            flags += ["--cfg=CONFIG_64BIT"]
        if prel:
            flags += ["--cfg=CONFIG_HAVE_ARCH_PREL32_RELOCATIONS"]
        return flags

    def preamble(self):
        return ('//! Data-only final symbol table.\n#![no_std]\n' +
                '#[path=' + rust_literal(str(SOURCE)) + '] mod export_internal;\n')

    def sources(self, directory, entries=ENTRIES):
        original, translated = ['#include <linux/export-internal.h>'], [self.preamble()]
        for name, namespace, flags, crc, function in entries:
            family = "FUNC" if function else "DATA"
            label = (name if re.fullmatch('[A-Za-z_][A-Za-z_0-9]*', name) and name != 'type'
                     else rust_literal(name))
            original += [f'KSYMTAB_{family}({name}, {c_literal(namespace)});',
                         f'SYMBOL_FLAGS({name}, 0x{flags:02x});', f'SYMBOL_CRC({name}, 0x{crc:08x});']
            translated += [f'KSYMTAB_{family}!({label}, {rust_literal(namespace)});',
                           f'SYMBOL_FLAGS!({label}, 0x{flags:02x});', f'SYMBOL_CRC!({label}, 0x{crc:08x});']
        c, rust = directory / 'metadata.c', directory / 'metadata.rs'
        c.write_text('\n'.join(original) + '\n')
        rust.write_text('\n'.join(translated) + '\n')
        return c, rust

    def cflags(self, arch, prel, optimize, compiler):
        target = {'x86_64': 'x86_64-linux-gnu', 'i686': 'i686-linux-gnu',
                  'arm64': 'aarch64-linux-gnu', 'powerpc': 'powerpc-linux-gnu'}[arch]
        flags = ['--target=' + target] if compiler == self.clang else ['-m64' if arch == 'x86_64' else '-m32']
        flags += ['-O' + optimize, '-g', '-fno-pic', '-fno-pie', '-I' + str(self.include.parent),
                  '-I' + str(ROOT / 'include')]
        if arch in ('x86_64', 'arm64'):
            flags += ['-DCONFIG_64BIT']
        if prel:
            flags += ['-DCONFIG_HAVE_ARCH_PREL32_RELOCATIONS']
        return flags

    def matrix(self, arch):
        if arch not in self.targets:
            self.skipTest(f"matching {arch} Rust core unavailable")
        for prel in (False, True):
            for optimize in ('0', '2', 's'):
                with self.subTest(arch=arch, prel=prel, optimize=optimize):
                    directory = self.work / f'{arch}-{prel}-{optimize}'
                    directory.mkdir()
                    c, rust = self.sources(directory)
                    output = directory / 'rust.o'
                    run([*self.rustc, *self.rustflags(arch, prel, optimize), '--emit=obj', rust, '-o', output])
                    observed = elf(output)
                    self.assertEqual(observed[3], {entry[0].encode() for entry in ENTRIES})
                    self.assertEqual(observed[4], set())
                    self.assertFalse({b'.export_symbol', b'.modinfo', b'.init.text', b'.exit.text'} & set(observed[5]))
                    dwarf = run([*tool('READELF', 'readelf'), '--debug-dump=info', output]).stdout
                    self.assertNotIn(b'DW_TAG_subprogram', dwarf)
                    self.assertNotIn(b'DW_TAG_variable', dwarf)
                    self.assertNotIn(b'__IS_RUST_MODULE', run([*tool('NM', 'nm'), output]).stdout)
                    self.assertEqual(tuple(map(len, observed[:3])), (19, 18, 30))
                    resolved = linked(output, directory, arch)
                    compilers = [self.clang] if arch in ('arm64', 'powerpc') else [self.cc, self.clang]
                    for index, compiler in enumerate(compilers):
                        original = directory / f'original-{index}.o'
                        run([*compiler, *self.cflags(arch, prel, optimize, compiler), '-c', c, '-o', original])
                        self.assertEqual(observed[:5], elf(original)[:5])
                        self.assertEqual(resolved, linked(original, directory, arch))

    def test_x86_64_private_header_prel32_absolute_objects_and_final_bytes(self):
        self.matrix('x86_64')

    def test_genuine_i686_prel32_absolute_objects_and_final_bytes(self):
        self.matrix('i686')

    def test_genuine_arm64_prel32_absolute_objects_and_final_bytes(self):
        self.matrix('arm64')

    def test_genuine_big_endian_powerpc_prel32_absolute_objects_and_final_bytes(self):
        self.matrix('powerpc')

    def test_namespace_macro_argument_uses_original_assembler_escape_semantics(self):
        # These are values of C macro string arguments, not raw modpost input:
        # a generator still must model its additional C-source-literal layer.
        namespaces = ['tab\tspace', 'bell\a', 'form\f', 'vertical\v', 'delete\x7f',
                      r'escaped\"quote\"', r'back\\slash', r'octal\101', r'hex\x41',
                      r'newline\n', 'bare\nnewline', '\r', 'UTF8 λ {}', r'embedded\000tail']
        for index, namespace in enumerate(namespaces):
            with self.subTest(namespace=namespace):
                directory = self.work / f'namespace-{index}'
                directory.mkdir()
                c, rust = self.sources(directory, [('function', namespace, 0, 0, True)])
                output = directory / 'rust.o'
                run([*self.rustc, *self.rustflags('x86_64', True, '2'), '--emit=obj', rust, '-o', output])
                for cc_index, compiler in enumerate((self.cc, self.clang)):
                    original = directory / f'original-{cc_index}.o'
                    run([*compiler, *self.cflags('x86_64', True, '2', compiler), '-c', c, '-o', original])
                    self.assertEqual(elf(output)[:5], elf(original)[:5])

    def test_normalized_fields_preserve_original_cpp_expansion_and_relocations(self):
        # GCC/Clang predefines "linux" in the GNU modes used by Kbuild.
        # This concrete collision must produce table name/reference 1, while
        # direct CRC/FLAGS stringify NAME without expanding its definition.
        for architecture in self.targets:
            compilers = [self.cc, self.clang] if architecture in ('x86_64', 'i686') else [self.clang]
            for prel in (False, True):
                for name, macros, table, expected in NORMALIZED_CASES:
                    with self.subTest(architecture=architecture, prel=prel, case=name):
                        directory = self.work / f'normalized-{architecture}-{prel}-{name}'
                        directory.mkdir()
                        suffix = '\nSYMBOL_FLAGS(NAME, 0x01);\nSYMBOL_CRC(NAME, 0x12345678);\n'
                        original, records = directory / 'original.c', directory / 'records.c'
                        prefix = '#include <linux/export-internal.h>\n' + macros
                        original.write_text(prefix + table + suffix)
                        records.write_text(prefix + NORMALIZED_RECORDS + table + suffix)
                        for compiler_number, compiler in enumerate(compilers):
                            flags = self.cflags(architecture, prel, '2', compiler)
                            normalized = run([*compiler, *flags, '-E', '-P', records]).stdout.decode()
                            original_text = run([*compiler, *flags, '-E', '-P', original]).stdout.decode()
                            lines = [line for line in normalized.splitlines() if line.startswith('LUPOS_EXPORT_')]
                            decoded = [normalized_fields(line) for line in lines]
                            self.assertEqual(decoded, [list(expected), ['NAME', '0x01'], ['NAME', '0x12345678']])
                            assembly = original_assembly(original_text)
                            directive = '.long ' if prel or architecture in ('i686', 'powerpc') else '.quad '
                            self.assertIn(directive + expected[1] + ('- .' if prel else ''), assembly[0])
                            self.assertIn('__kstrtab_' + expected[0] + ':', assembly[0])
                            self.assertIn('__flags_NAME:', assembly[1])
                            self.assertIn('__crc_NAME:', assembly[2])
                            rust = directory / f'normalized-{compiler_number}.rs'
                            rust.write_text(self.preamble() + '\n__KSYMTAB_NORMALIZED!(' +
                                            ', '.join(rust_literal(v) for v in decoded[0]) + ');\n' +
                                            'SYMBOL_FLAGS!("NAME", 0x01);\nSYMBOL_CRC!("NAME", 0x12345678);\n')
                            c_object, r_object = directory / f'original-{compiler_number}.o', rust.with_suffix('.o')
                            run([*compiler, *flags, '-c', original, '-o', c_object])
                            run([*self.rustc, *self.rustflags(architecture, prel, '2'), '--emit=obj', rust, '-o', r_object])
                            self.assertEqual(elf(c_object)[:5], elf(r_object)[:5])
                            self.assertFalse(elf(r_object)[4], 'metadata invented global owner definitions')

    def test_normalized_invalid_assembler_expressions_remain_diagnostics(self):
        for name, expression in [('invalid', 'target +'), ('bad name', 'target')]:
            with self.subTest(name=name, expression=expression):
                original, translated = self.work / 'invalid-normalized.c', self.work / 'invalid-normalized.rs'
                original.write_text('#include <linux/export-internal.h>\n' +
                                    f'__KSYMTAB({name}, {expression}, "");\n')
                translated.write_text(self.preamble() + '__KSYMTAB_NORMALIZED!(' +
                                      ', '.join(rust_literal(v) for v in (name, expression, '')) + ');\n')
                for compiler in (self.cc, self.clang):
                    run([*compiler, *self.cflags('x86_64', True, '2', compiler), '-c', original,
                         '-o', original.with_suffix('.o')], failure=True)
                result = run([*self.rustc, *self.rustflags('x86_64', True, '2'), '--emit=obj', translated,
                              '-o', translated.with_suffix('.o')], failure=True)
                self.assertIn(b'inline asm', result.stderr)

    def test_bad_linkage_namespace_and_numeric_inputs_fail_before_injection(self):
        statements = [
            'KSYMTAB_FUNC!("", "");', 'KSYMTAB_FUNC!(".", "");', 'KSYMTAB_FUNC!("1bad", "");',
            'KSYMTAB_FUNC!("foo; .globl injected", "");', 'KSYMTAB_FUNC!("a\\nb", "");',
            'KSYMTAB_FUNC!("r#type", "");', 'KSYMTAB_FUNC!("é", "");',
            'KSYMTAB_FUNC!(foo, "bad\\\"quote");', 'KSYMTAB_FUNC!(foo, "trailing\\\\");',
            'KSYMTAB_FUNC!(foo, "bad\\0nul");', 'SYMBOL_FLAGS!(foo, 256);',
            'SYMBOL_CRC!(foo, 0x100000000);', 'SYMBOL_CRC!(foo, -1);']
        for index, statement in enumerate(statements):
            path = self.work / f'invalid-{index}.rs'
            path.write_text(self.preamble() + statement + '\n')
            result = run([*self.rustc, *self.rustflags('x86_64', True, '2'), '--emit=obj', path,
                          '-o', path.with_suffix('.o')], failure=True)
            self.assertTrue(any(reason in result.stderr for reason in
                                (b'error[E0080]', b'literal out of range', b'cannot apply unary operator')), result.stderr)

    def test_duplicate_records_are_not_hidden(self):
        path = self.work / 'duplicate.rs'
        path.write_text(self.preamble() + 'KSYMTAB_FUNC!(foo, "");\nKSYMTAB_FUNC!(foo, "");\n')
        result = run([*self.rustc, *self.rustflags('x86_64', True, '2'), '--emit=obj', path,
                      '-o', path.with_suffix('.o')], failure=True)
        self.assertIn(b'already defined', result.stderr)

    def test_original_marker_and_parisc64_expansion_only(self):
        baseline = run(['git', '-C', ROOT, 'show', '68f3e0875:include/linux/export-internal_header.rs']).stdout.decode()
        self.assertEqual(re.findall(r'SOURCE-COMMIT: (\w+)', SOURCE.read_text()),
                         re.findall(r'SOURCE-COMMIT: (\w+)', baseline))
        empty = self.work / 'empty.rs'
        empty.write_text(self.preamble())
        empty_object = empty.with_suffix('.o')
        run([*self.rustc, *self.rustflags('x86_64', True, '0'), '--emit=obj', empty, '-o', empty_object])
        self.assertFalse(any(elf(empty_object)[:5]))
        directory = self.work / 'parisc-expansion-only'
        directory.mkdir()
        c, rust = self.sources(directory)
        translated = run([*self.rustc, *self.rustflags('x86_64', True, '2'), '--cfg=CONFIG_PARISC',
                          '-Zunpretty=expanded', rust]).stdout
        original = run([*self.cc, '-E', '-P', '-DCONFIG_PARISC', '-DCONFIG_64BIT',
                        '-DCONFIG_HAVE_ARCH_PREL32_RELOCATIONS', '-I' + str(self.include.parent),
                        '-I' + str(ROOT / 'include'), c]).stdout
        for name, _, _, _, function in ENTRIES:
            expected = (('P%' if function else '') + name).encode()
            self.assertIn(expected, translated)
            self.assertIn(expected, original)

        records = directory / 'normalized-records.c'
        records.write_text('#include <linux/export-internal.h>\n' + NORMALIZED_RECORDS +
                           '\nKSYMTAB_FUNC(target, "");\n')
        normalized = run([*self.cc, '-E', '-P', '-DCONFIG_PARISC', '-DCONFIG_64BIT',
                          '-I' + str(self.include.parent), '-I' + str(ROOT / 'include'), records]).stdout.decode()
        line = next(line for line in normalized.splitlines() if line.startswith('LUPOS_EXPORT_'))
        self.assertEqual(normalized_fields(line), ['target', 'P%target', ''])


if __name__ == '__main__':
    unittest.main()
