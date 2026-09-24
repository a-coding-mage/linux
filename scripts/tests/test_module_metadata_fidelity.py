#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Private per-module metadata fidelity gates; never a native-load claim.

The original declarations and compiler macros remain the C authority. These
tests construct metadata objects, not replacement module owners. Requested
target sysroots fail if unusable. The accepted contract is natural alignment,
not reproduction of a frontend's optional preferred array over-alignment.
"""

import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys
import tempfile
import unittest
from unittest import mock
from concurrent.futures import ThreadPoolExecutor


ROOT = next(path for path in (Path(__file__).resolve().parents[2], Path.cwd())
            if (path / "scripts/mod/modpost.c").is_file())
sys.path.insert(0, str(ROOT / "scripts/tests"))
from test_module_common import elf
from modpost_test_support import Elf, modpost_tools

WORK = Path(os.environ.get('MODULE_METADATA_TEST_WORK', tempfile.gettempdir()))
if not WORK.is_dir():
    raise RuntimeError(f'explicit test work directory is not a directory: {WORK}')
CANDIDATE = Path(os.environ.get('MODULE_METADATA_CANDIDATE', str(ROOT)))
SECTIONS = ("__versions", "__version_ext_crcs", "__version_ext_names")
OPTS = ("0", "2", "s")


def command(name, default):
    return shlex.split(os.environ.get(name, default))


def run(args, **kwargs):
    result = subprocess.run(list(map(str, args)), capture_output=True,
                            timeout=90, **kwargs)
    if result.returncode:
        raise AssertionError(shlex.join(map(str, args)) + "\n" +
                             result.stderr.decode(errors="replace"))
    return result.stdout


def symbols(parsed):
    table = parsed["sections"].get(".symtab")
    if table is None:
        return []
    names = next(section["data"] for section in parsed["sections"].values()
                 if section["index"] == table["header"][6])
    result = []
    for at in range(0, len(table["data"]), table["header"][9]):
        if parsed["bits"] == 64:
            name, info, other, index, value, size = struct.unpack_from(
                parsed["order"] + "IBBHQQ", table["data"], at)
        else:
            name, value, size, info, other, index = struct.unpack_from(
                parsed["order"] + "IIIBBH", table["data"], at)
        result.append((names[name:names.index(0, name)], info, other,
                       index, value, size))
    return result


def relocations(parsed, target):
    """Resolve relocation identities, not ELF symbol-table indices."""
    table = symbols(parsed)
    result = []
    index = parsed["sections"][target]["index"]
    for section in parsed["sections"].values():
        if section["type"] not in (4, 9) or section["header"][7] != index:
            continue
        wide, explicit = parsed["bits"] == 64, section["type"] == 4
        fmt = ("QQ" if wide else "II") + (("q" if wide else "i") if explicit else "")
        for at in range(0, len(section["data"]), section["header"][9]):
            values = struct.unpack_from(parsed["order"] + fmt, section["data"], at)
            offset, info = values[:2]
            symbol = table[info >> (32 if wide else 8)][0]
            kind = info & (0xffffffff if wide else 0xff)
            result.append((offset, kind, symbol, values[2] if explicit else None))
    return sorted(result)


def first_saved_command(path):
    saved = path.read_text().splitlines()[0].split(' := ', 1)[1]
    lexer = shlex.shlex(saved, posix=True, punctuation_chars=';')
    lexer.whitespace_split = True
    arguments = []
    for token in lexer:
        if token and set(token) == {';'}:
            break
        arguments.append(token)
    return arguments


def native_arguments(build, rust):
    """Read original saved metadata/owner flags, redirect all outputs privately.

    The caller must supply output/source/env flags. Nothing is run or written
    here, and no native object command is evaluated by a shell. This is a
    replay transport for real configuration-specific headers/rmeta, not a
    fabricated kernel or bindings definition.
    """
    stem = 'prime_numbers' if rust else 'prime_numbers.mod'
    path = build / ('lib/math/.' + stem + '.o.cmd')
    args = first_saved_command(path)
    if not rust and any(value.startswith('MODULE_METADATA_DATA=') or
                        value.endswith('/module-metadata.rs') for value in args):
        # After the optional path is selected, .mod.o is Rust. Its .mod.rs
        # prerequisite records the same authoritative original C frontend
        # flags, used for validation before preprocessing the record stream.
        path = build / 'lib/math/.prime_numbers.mod.rs.cmd'
        args = first_saved_command(path)
        if '-fsyntax-only' not in args:
            raise AssertionError(f'expected original metadata C validation command: {path}')
    if rust:
        while '=' in args[0] and not args[0].startswith('-'):
            args.pop(0)
    result = []
    at = 0
    while at < len(args):
        value = args[at]
        at += 1
        if value in ('-o', '--out-dir', '--crate-name', '-x'):
            at += 1
            continue
        if value == '-fsyntax-only' or value.endswith('.mod.h'):
            continue
        if value.startswith('-fmacro-prefix-map='):
            source, separator, destination = value[len('-fmacro-prefix-map='):].partition('=')
            if separator and source.endswith('.mod.h') and destination == source[:-1] + 'c':
                continue
        if value.startswith(('-Wp,-MMD,', '--emit=', '--crate-name=')) or value.endswith(('.c', '.rs')):
            continue
        if value.startswith(('-Copt-level=', '-Zallow-features=', '-Zcrate-attr=')) or (value.startswith('-O') and not rust):
            continue
        if value in ('-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers'):
            continue
        if value.startswith('-I.'):
            value = '-I' + str(build / value[2:])
        elif value.startswith('--target=.'):
            value = '--target=' + str(build / value.split('=', 1)[1])
        elif value.startswith('@.'):
            value = '@' + str(build / value[1:])
        elif value == '-L' and args[at].startswith('.'):
            result += ['-L', str(build / args[at])]
            at += 1
            continue
        result.append(value)
    if not rust and '-c' not in result:
        result.append('-c')
    return result


def version_payloads(path, expected, natural):
    parsed = elf(path)
    sections = parsed["sections"]
    assert set(sections) & set(SECTIONS) == set(expected)
    for name, payload in expected.items():
        section = sections[name]
        assert (section["type"], section["flags"], section["data"]) == (1, 2, payload)
        assert section["align"] >= natural[name]
        assert section["header"][4] % natural[name] == 0
        for relocation in sections.values():
            assert not (relocation["type"] in (4, 9) and
                        relocation["header"][7] == section["index"])
    assert not any(section["flags"] & 4 and section["header"][5]
                   for section in sections.values())
    assert not any(name and index == 0 for name, _, _, index, _, _ in symbols(parsed))
    assert not any(name in (b"__IS_RUST_MODULE", b"__this_module")
                   for name, *_ in symbols(parsed))
    assert not any(name.startswith((".export_symbol", "__ksymtab")) for name in sections)
    return parsed


def records():
    # Distinct values expose endian/order mistakes; all names fit both widths.
    return [(0x12345678, b"first"), (0xffffffff, b"zeta"),
            (0, b"zero_crc"), (0x80000000, b"middle"),
            (0xaabbccdd, b"module_layout")]


def bytes_literal(raw):
    return '"' + ''.join('\\%03o' % value for value in raw) + '"'


class VersionArrayFidelity(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="versions-", dir=WORK)
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.rustc = command("HOSTRUSTC", "rustc")
        cls.bindgen = command("BINDGEN", "bindgen")
        cls.cc = command("HOSTCC", "cc")
        cls.clang = command("CLANG", "clang")
        cls.linker = command("LD_LLD", "ld.lld")
        declaration = re.search(r"struct modversion_info \{.*?\n\};",
            (ROOT / "include/linux/module.h").read_text(), re.S)[0]
        name_length = re.search(r"^#define __MODULE_NAME_LEN .*$",
            (ROOT / "include/linux/moduleparam.h").read_text(), re.M)[0]
        cls.header = cls.work / "original_types.h"
        cls.header.write_text(name_length + "\n#define MODULE_NAME_LEN __MODULE_NAME_LEN\n" +
                              declaration + "\n")
        cls.targets = {}
        for name, bits, ctarget, target, environment, cc in (
            ("x86_64", 64, "x86_64-linux-gnu", "x86_64-unknown-linux-gnu", None, cls.cc + ["-m64"]),
            ("i686", 32, "i686-linux-gnu", "i686-unknown-linux-gnu", "INT_MATH_I686_SYSROOT", cls.cc + ["-m32"]),
            ("arm64", 64, "aarch64-linux-gnu", "aarch64-unknown-linux-musl", "MODULE_COMMON_ARM64_SYSROOT", cls.clang + ["--target=aarch64-linux-gnu"]),
            ("powerpc", 32, "powerpc-linux-gnu", "powerpc-unknown-linux-gnu", "MODULE_COMMON_POWERPC_SYSROOT", cls.clang + ["--target=powerpc-linux-gnu"]),
        ):
            flags = ["--target=" + target]
            requested = os.environ.get(environment) if environment else None
            if environment and environment in os.environ and not requested:
                raise AssertionError(f'explicit {environment} must not be empty')
            if requested:
                flags += ["--sysroot", requested]
            libdir = Path(run(cls.rustc + flags + ["--print=target-libdir"]).decode().strip())
            if not any(libdir.glob("libcore*.rlib")):
                if requested:
                    raise AssertionError(f"requested {environment} lacks target core: {libdir}")
                continue
            directory = cls.work / name
            directory.mkdir()
            run(cls.bindgen + [cls.header, "--use-core", "--ctypes-prefix", "core::ffi",
                "--no-layout-tests", "--no-doc-comments", "--allowlist-type", "modversion_info",
                "-o", directory / "bindings.rs", "--", "--target=" + ctarget])
            cls.targets[name] = dict(bits=bits, cc=cc, flags=flags, directory=directory)

    def require(self, target):
        if target not in self.targets:
            self.skipTest(f"matching {target} Rust core unavailable")
        return self.targets[target]

    def pair(self, target, name, basic, extended, opt="0", bad=None):
        case = self.require(target)
        directory = case["directory"]
        cc = '#include "original_types.h"\n'
        rust = "//! Actual-binding version metadata fixture.\n#![no_std]\n"
        rust += "#[allow(dead_code, non_camel_case_types, unreachable_pub)] mod bindings;\n"
        rust += "const _: () = assert!(core::mem::size_of::<bindings::modversion_info>() == 64);\n"
        expected = {}
        order = ">" if target == "powerpc" else "<"
        if basic is not None:
            cc += 'static const struct modversion_info ____versions[] __attribute__((used,section("__versions"))) = {\n'
            cc += ''.join('{0x%08x,%s},\n' % (crc, bytes_literal(symbol)) for crc, symbol in basic) + '};\n'
            entries = []
            payload = b""
            for crc, symbol in basic:
                raw = symbol + bytes(64 - case["bits"] // 8 - len(symbol))
                entries.append('bindings::modversion_info { crc: %d, name: [%s] }' %
                    (crc, ','.join(str(value) + " as core::ffi::c_char" for value in raw)))
                payload += struct.pack(order + ("Q" if case["bits"] == 64 else "I"), crc) + raw
            rust += '#[used]\n#[link_section="__versions"]\nstatic VERSIONS: [bindings::modversion_info;%d] = [%s];\n' % (len(entries), ','.join(entries))
            expected[SECTIONS[0]] = payload
        if extended is not None:
            crcs = [crc for crc, _ in extended]
            names = b''.join(symbol + b'\0' for _, symbol in extended) + b'\0'
            cc += 'static const unsigned int ____version_ext_crcs[] __attribute__((used,section("__version_ext_crcs"))) = {'
            cc += ','.join(hex(crc) for crc in crcs) + '};\n'
            cc += 'static const char ____version_ext_names[] __attribute__((used,section("__version_ext_names"))) =\n'
            cc += '\n'.join(bytes_literal(symbol + b'\0') for _, symbol in extended) + ';\n'
            rust_names = names[:-1] if bad == "missing-final-nul" else names
            rust_crcs = list(reversed(crcs)) if bad == "crc-order" else crcs
            if bad == "tail-padding":
                rust += '#[repr(C, align(16))] struct Wrong([u32;%d]);\n' % len(crcs)
                rust += '#[used]\n#[link_section="__version_ext_crcs"]\nstatic CRCS: Wrong = Wrong([%s]);\n' % ','.join(map(str, rust_crcs))
            else:
                rust += '#[used]\n#[link_section="__version_ext_crcs"]\nstatic CRCS: [u32;%d] = [%s];\n' % (len(crcs), ','.join(map(str, rust_crcs)))
            rust += '#[used]\n#[link_section="__version_ext_names"]\nstatic NAMES: [u8;%d] = [%s];\n' % (len(rust_names), ','.join(map(str, rust_names)))
            expected[SECTIONS[1]] = b''.join(struct.pack(order + "I", crc) for crc in crcs)
            expected[SECTIONS[2]] = names
        csrc, rsrc = directory / (name + '.c'), directory / (name + '.rs')
        cobj, robj = directory / (name + '-c.o'), directory / (name + '-rust.o')
        csrc.write_text(cc)
        rsrc.write_text(rust)
        run(case["cc"] + ["-ffreestanding", "-fno-pic", "-fno-pie", "-I", self.work,
            "-O" + opt, "-c", csrc, "-o", cobj])
        run(self.rustc + case["flags"] + ["--edition=2021", "--crate-type=rlib",
            "--crate-name=version_metadata", "-Cpanic=abort", "-Copt-level=" + opt,
            "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub", "-Wrust-2018-idioms",
            "--emit=obj", rsrc, "-o", robj])
        natural = dict(zip(SECTIONS, (case["bits"] // 8, 4, 1)))
        version_payloads(cobj, expected, natural)
        return cobj, robj, expected, natural

    def matrix(self, target):
        self.require(target)
        for index, (basic, extended) in enumerate(((records(), None), (None, records()),
                (records(), records()), (records()[:1], records()[:1]))):
            for opt in OPTS:
                with self.subTest(target=target, variant=index, opt=opt):
                    _, rust, expected, natural = self.pair(target, f'm{index}-{opt}', basic, extended, opt)
                    parsed = version_payloads(rust, expected, natural)
                    for section in expected:
                        self.assertEqual(parsed["sections"][section]["align"], natural[section])
                    linked = rust.with_suffix('.linked.o')
                    run(self.linker + ["-r", "--build-id=none", rust, "-o", linked])
                    version_payloads(linked, expected, natural)

    def test_x86_64_natural_alignment(self):
        self.matrix("x86_64")

    def test_i686_natural_alignment(self):
        self.matrix("i686")

    def test_arm64_natural_alignment(self):
        self.matrix("arm64")

    def test_powerpc32_big_endian_natural_alignment(self):
        self.matrix("powerpc")

    def test_long_names_filtering_and_order(self):
        for target in self.targets:
            width = self.targets[target]["bits"]
            short = (7, b'a' * (64 - width // 8 - 1))
            long = (9, b'b' * (64 - width // 8))
            extended = [records()[0], long, short, records()[-1]]
            basic = [record for record in extended if record is not long]
            _, rust, expected, natural = self.pair(target, 'long-names', basic, extended)
            version_payloads(rust, expected, natural)

    def test_empty_basic_array_and_original_invalid_empty_extended_names(self):
        _, rust, expected, natural = self.pair('x86_64', 'empty-basic', [], None)
        version_payloads(rust, expected, natural)
        # Original modpost emits '= ;' when extended imports are empty. That
        # frontend error must remain a validation error, not silently become a
        # fabricated one-byte names table in the translated parser.
        source = self.work / 'empty-extended.c'
        source.write_text('static const char names[] __attribute__((used)) = ;\n')
        result = subprocess.run(self.cc + ['-fsyntax-only', str(source)],
                                capture_output=True, timeout=30)
        self.assertNotEqual(result.returncode, 0)
        self.assertTrue(result.stderr)

    def test_corruption_controls_are_object_checks_not_compile_failures(self):
        for mutation in ('missing-final-nul', 'crc-order', 'tail-padding'):
            _, rust, expected, natural = self.pair('x86_64', mutation,
                records(), records(), bad=mutation)
            with self.assertRaises(AssertionError):
                version_payloads(rust, expected, natural)


class PowerPcModuleSections(unittest.TestCase):
    def matrix(self, wide):
        rustc = command('HOSTRUSTC', 'rustc')
        target = ['--target=' + ('powerpc64le-unknown-linux-gnu' if wide else 'powerpc-unknown-linux-gnu')]
        variable = 'MODULE_METADATA_POWERPC64_SYSROOT' if wide else 'MODULE_COMMON_POWERPC_SYSROOT'
        requested = os.environ.get(variable)
        if variable in os.environ and not requested:
            self.fail(f'explicit {variable} must not be empty')
        if requested:
            target += ['--sysroot', requested]
        libdir = Path(run(rustc + target + ['--print=target-libdir']).decode().strip())
        if not any(libdir.glob('libcore*.rlib')):
            if requested:
                self.fail(f'explicit {variable} lacks matching core')
            self.skipTest('matching PPC64LE core unavailable' if wide else 'matching PPC32 core unavailable')
        original = (ROOT / 'arch/powerpc/include/asm/module.h').read_text()
        start = original.index('#ifdef __powerpc64__', original.index('Make empty sections'))
        end = original.index('\n#ifdef CONFIG_DYNAMIC_FTRACE', start)
        fragment = original[start:end]
        self.assertEqual(fragment.count('asm('), 4)
        for pcrel in ((False, True) if wide else (False,)):
            with self.subTest(wide=wide, pcrel=pcrel), tempfile.TemporaryDirectory(prefix='ppc-sections-', dir=WORK) as temporary:
                self.section_case(Path(temporary), fragment, rustc, target, wide, pcrel)

    def section_case(self, directory, fragment, rustc, target, wide, pcrel):
            csrc, rsrc = directory / 'original.c', directory / 'metadata.rs'
            cobj, robj = directory / 'original.o', directory / 'metadata.o'
            csrc.write_text(fragment)
            wrapper = (CANDIDATE / 'scripts/module-metadata.rs').read_text()
            self.assertEqual(wrapper.count('// Original asm/module.h creates'), 1)
            self.assertEqual(wrapper.count('include!(env!("MODULE_METADATA_DATA"))'), 1)
            # Compile the exact candidate's section side effects, without
            # inventing a PPC module binding. Full PPC owner layout is a
            # separate future native gate, not claimed by this fragment test.
            rust_fragment = wrapper.split('// Original asm/module.h creates', 1)[1]
            rust_fragment = '// Original asm/module.h creates' + rust_fragment.split('include!(env!', 1)[0]
            rsrc.write_text('//! Private exact candidate section-fragment proof.\n#![no_std]\n'
                '#![feature(asm_experimental_arch)]\n' + rust_fragment)
            run(command('CLANG', 'clang') + ['--target=' + ('powerpc64le-linux-gnu' if wide else 'powerpc-linux-gnu'), '-DMODULE',
                *(['-DCONFIG_PPC_KERNEL_PCREL'] if pcrel else []),
                '-ffreestanding', '-c', csrc, '-o', cobj])
            run(rustc + target + ['--edition=2021', '-Dwarnings', '-Wmissing-docs',
                '-Wunreachable-pub', '-Cpanic=abort', '--crate-type=rlib', '--emit=obj',
                '--cfg=CONFIG_PPC', *(['--cfg=CONFIG_PPC_KERNEL_PCREL'] if pcrel else []),
                rsrc, '-o', robj], env=dict(os.environ, RUSTC_BOOTSTRAP='1'))
            a, b = elf(cobj), elf(robj)
            self.assertEqual((a['machine'], a['order'], a['bits']), (21, '<', 64) if wide else (20, '>', 32))
            self.assertEqual((a['machine'], a['order'], a['bits']),
                             (b['machine'], b['order'], b['bits']))
            expected = ({'.stubs': 6, **({'.mygot': 2} if pcrel else {})} if wide else
                        {'.plt': 6, '.init.plt': 6})
            all_names = {'.stubs', '.mygot', '.plt', '.init.plt'}
            self.assertEqual(set(a['sections']) & all_names, set(expected))
            self.assertEqual(set(b['sections']) & all_names, set(expected))
            for name, flags in expected.items():
                original_section, section = a['sections'][name], b['sections'][name]
                self.assertEqual(tuple(original_section[k] for k in ('type', 'flags', 'align')),
                                 (8, flags, 8))
                self.assertEqual(tuple(section[k] for k in ('type', 'flags', 'align')), (8, flags, 8))
                self.assertEqual(original_section['header'][5], 0)
                self.assertEqual(section['header'][5], 0)
            self.assertFalse(any(name and index == 0 for name, _, _, index, _, _ in symbols(b)))
            self.assertFalse(any(section['flags'] & 4 and section['header'][5]
                                 for section in b['sections'].values()))

    def test_original_ppc32_mandatory_empty_sections(self):
        self.matrix(False)

    def test_original_ppc64le_mandatory_sections_with_and_without_pcrel(self):
        self.matrix(True)


class NativeCommandTransport(unittest.TestCase):
    def test_c_frontend_from_original_and_selected_rust_metadata_commands(self):
        with tempfile.TemporaryDirectory(prefix='saved-commands-', dir=WORK) as temporary:
            build = Path(temporary)
            directory = build / 'lib/math'
            directory.mkdir(parents=True)
            common = ['clang', '-I./include', '-DMODULE', '-D__DISABLE_EXPORTS',
                      '-fmacro-prefix-map=/keep/original=/keep/mapping',
                      '-DCHECK_TEXT="quoted;semicolon"', '-O2', '-c']
            command_file = directory / '.prime_numbers.mod.o.cmd'
            command_file.write_text('savedcmd_target := ' + shlex.join(common + ['-Wp,-MMD,private.d',
                '-o', 'lib/math/prime_numbers.mod.o', 'lib/math/prime_numbers.mod.c']) + '\n')
            expected = native_arguments(build, False)
            command_file.write_text('savedcmd_target := MODULE_METADATA_DATA=/private/data.rs rustc '
                                    '--crate-name=module_metadata /source/scripts/module-metadata.rs\n')
            data_command = directory / '.prime_numbers.mod.rs.cmd'
            data_command.write_text('savedcmd_target := ' + shlex.join([arg for arg in common if arg != '-c'] + ['-Wp,-MMD,other.d',
                '-fmacro-prefix-map=lib/math/prime_numbers.mod.h=lib/math/prime_numbers.mod.c',
                '-fsyntax-only', '-x', 'c', 'lib/math/prime_numbers.mod.h']) + '; mv other.d other.d.validate; modpost --rust-module-records < input > output\n')
            actual = native_arguments(build, False)
            self.assertEqual(expected, actual)
            self.assertIn('-D__DISABLE_EXPORTS', actual)
            self.assertIn('-fmacro-prefix-map=/keep/original=/keep/mapping', actual)
            data_command.write_text('savedcmd_target := rustc /bad/source.rs\n')
            with self.assertRaisesRegex(AssertionError, 'C validation command'):
                native_arguments(build, False)


class CandidateMetadataParity(unittest.TestCase):
    """Actual candidate wrapper plus actual native-header binding metadata.

    Synthetic input ELF controls modpost policy, while compiled metadata uses
    genuine current target headers/rmeta. Native trees are input-only; this
    does not load or link against a running kernel.
    """

    maxDiff = None

    @classmethod
    def setUpClass(cls):
        cls.builds = {}
        for arch, variable in (('x86', 'MODULE_METADATA_X86_BUILD'),
                               ('arm64', 'MODULE_METADATA_ARM64_BUILD')):
            if variable not in os.environ:
                continue
            value = os.environ[variable]
            build = Path(value)
            if not value or not build.is_dir():
                raise AssertionError(f'explicit {variable} is not a directory: {value!r}')
            for relative in ('lib/math/.prime_numbers.o.cmd', 'lib/math/.prime_numbers.mod.o.cmd',
                             'rust/libkernel.rmeta', 'rust/libbindings.rmeta', 'rust/libpin_init.rmeta',
                             'rust/bindings/.bindings_generated.rs.cmd',
                             'include/generated/autoconf.h', 'include/generated/rustc_cfg'):
                if not (build / relative).is_file():
                    raise AssertionError(f'explicit {variable} lacks {relative}: {build}')
            cls.builds[arch] = build.resolve()
        if not cls.builds:
            raise unittest.SkipTest('read-only native metadata gates need MODULE_METADATA_X86_BUILD or MODULE_METADATA_ARM64_BUILD')
        cls.temporary = tempfile.TemporaryDirectory(prefix='candidate-', dir=WORK)
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        # The process-wide cached oracle tools outlive this test class. Their
        # own atexit cleanup must not be nested under this class's temporary
        # directory, or subsequent suites receive paths to deleted binaries.
        cls.tools = modpost_tools()
        cls.candidate = CANDIDATE
        cls.binary = cls.work / 'modpost'
        cls.wrapper = cls.candidate / 'scripts/module-metadata.rs'
        cls.source = cls.candidate / 'scripts/mod/modpost.rs'
        run(command('HOSTRUSTC', 'rustc') + ['--edition=2021', '-Dwarnings',
            '-Wmissing-docs', '-Wunreachable-pub', '-Wrust-2018-idioms', '-O',
            '--cfg=modpost_target_offsets', cls.source, '-o', cls.binary],
            env=dict(os.environ, MODPOST_DEVICETABLE_OFFSETS=str(cls.tools[64][0].parent /
                                                                  'devicetable-offsets.h')))
        cls.unload_variants = {}

    def inputs(self, init=False, cleanup=False, rust_owner=False, license='GPL', long=False,
               alias=None, unversioned=False):
        owner = Elf()
        owner.module_info(license=license, imports=('NS',), version='1.2')
        text = owner.section('.text', bytes(16), flags=6)
        if init:
            owner.symbol('init_module', text, kind=2)
        if cleanup:
            owner.symbol('cleanup_module', text, value=8, kind=2)
        if rust_owner:
            owner.symbol('__IS_RUST_MODULE', owner.section('.data', b'\0'), kind=1)
        owner.export('local_function', namespace='NS', gpl=True)
        owner.export('local_data', kind=1)
        if alias is not None:
            size = self.tools[64][2]['SIZE_platform_device_id']
            name = alias.encode()
            self.assertLess(len(name), 20)
            data = name + bytes(2 * size - len(name))
            section = owner.section('.rodata', data)
            owner.symbol('__mod_device_table__kmod_fixture__platform__ids',
                         section, size=len(data), kind=1)
        imports = [('zeta', 'drivers/z', 0x12345678), ('alpha', 'drivers/a', 0xabcdef01),
                   ('again_z', 'drivers/z', 0), ('builtin', 'vmlinux', 0xffffffff)]
        if long:
            imports.insert(1, ('long_' + 'x' * 64, 'drivers/long', 0x80000000))
        for name, _, _ in imports:
            owner.symbol(name)
        if unversioned:
            owner.symbol('unversioned')
        owner.symbol('absent_weak', binding=2)
        dump = ''.join(f'0x{crc:08x}\t{name}\t{provider}\tEXPORT_SYMBOL\t\n'
                       for name, provider, crc in imports)
        dump += '0x11223344\tmodule_layout\tvmlinux\tEXPORT_SYMBOL\t\n'
        result = {'fixture.o': owner.build(), 'input.symvers': dump,
                'fixture.mod': 'part.o\n', 'part.c': 'int untouched_reference;\n',
                '.part.o.cmd': 'source_part.o := part.c\n\n#SYMVER local_function 0xa1b2c3d4\n#SYMVER local_data 0xfedcba98\n'}
        if unversioned:
            provider = Elf()
            provider.module_info()
            provider.export('unversioned')
            result.update({'provider.o': provider.build(), 'provider.mod': 'provider-part.o\n',
                           '.provider-part.o.cmd': '\n'})
        return result

    def emit(self, label, files, flags=(), status=0):
        outputs, outcomes = [], []
        for language, tool in (('c', self.tools[64][0]), ('rust', self.binary)):
            directory = self.work / f'{label}-{language}'
            directory.mkdir()
            for name, data in files.items():
                path = directory / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(data.encode() if isinstance(data, str) else data)
            args = [tool, '-M', '-i', 'input.symvers', *flags,
                    *(['--rust-module-metadata'] if language == 'rust' else []),
                    'fixture.o', *(['provider.o'] if 'provider.o' in files else []),
                    '-o', 'Module.symvers']
            result = subprocess.run(list(map(str, args)), cwd=directory,
                                    capture_output=True, timeout=30)
            outputs.append(directory)
            outcomes.append(result)
        self.assertEqual(tuple(outcome.returncode for outcome in outcomes), (status, status),
                         tuple(outcome.stderr for outcome in outcomes))
        self.assertEqual(outcomes[0].stderr, outcomes[1].stderr)
        self.assertEqual(outcomes[0].stdout, outcomes[1].stdout)
        self.last_stderr = outcomes[0].stderr
        if status == 0:
            self.assertEqual((outputs[0] / 'Module.symvers').read_bytes(),
                             (outputs[1] / 'Module.symvers').read_bytes())
            self.assertFalse((outputs[1] / 'fixture.mod.c').exists())
        return outputs

    def unload_disabled(self, arch):
        if arch in self.unload_variants:
            return self.unload_variants[arch]
        build = self.builds[arch]
        original = (build / 'include/generated/autoconf.h').read_text()
        if '#define CONFIG_MODULE_UNLOAD 1\n' not in original:
            self.unload_variants[arch] = None
            return None
        directory = self.work / (arch + '-unload-disabled')
        include = directory / 'include/generated'
        include.mkdir(parents=True)
        (include / 'autoconf.h').write_text(original.replace('#define CONFIG_MODULE_UNLOAD 1\n', ''))
        cfg = directory / 'rustc_cfg'
        cfg.write_text(''.join(line for line in (build / 'include/generated/rustc_cfg').read_text().splitlines(keepends=True)
                              if not line.startswith('--cfg=CONFIG_MODULE_UNLOAD')))
        args = [value.replace('$(pound)', '#') for value in
                first_saved_command(build / 'rust/bindings/.bindings_generated.rs.cmd')]
        boundary = args.index('--')
        front, clang = args[:boundary], args[boundary + 1:]
        output = directory / 'bindings.rs'
        front[front.index('-o') + 1] = str(output)
        front += ['--allowlist-type', '^(module|modversion_info)$',
                  '--allowlist-function', '^(init_module|cleanup_module)$']
        changed = ['-I' + str(include.parent)]
        for argument in clang:
            if argument.startswith('-Wp,-MMD,'):
                continue
            if argument.startswith('-I') and not argument[2:].startswith('/'):
                argument = '-I' + str(build / argument[2:])
            changed.append(argument)
        run(front + ['--'] + changed)
        text = output.read_text()
        self.assertIn('pub struct module {', text)
        self.assertIn('pub struct modversion_info {', text)
        self.assertNotIn('pub exit:', text.split('pub struct module {', 1)[1].split('\n}', 1)[0])
        source = directory / 'kernel_bindings.rs'
        source.write_text('''//! Private actual-header bindings for MODULE_UNLOAD=n.
#![no_std]
#![feature(cfi_encoding)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs,
    dead_code, unreachable_pub, unsafe_op_in_unsafe_fn, improper_ctypes)]
pub mod bindings {
    use pin_init::{MaybeZeroable, Zeroable};
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    // Exact generic binding implementation from rust/bindings/lib.rs.
    unsafe impl<Storage> Zeroable for __BindgenBitfieldUnit<Storage> where Storage: Zeroable {}
    include!(env!("MODULE_METADATA_TEST_BINDINGS"));
}
''')
        args = [('@' + str(cfg)) if value.startswith('@') else value
                for value in native_arguments(build, True)]
        metadata = directory / 'libkernel_metadata_fixture.rmeta'
        run(args + ['--extern', 'ffi', '--crate-name=kernel_metadata_fixture',
            '--out-dir=' + str(directory), '--emit=metadata=' + str(metadata), source],
            env=dict(os.environ, RUSTC_BOOTSTRAP='1', MODULE_METADATA_TEST_BINDINGS=str(output)))
        self.unload_variants[arch] = (directory, cfg, metadata)
        return self.unload_variants[arch]

    def compile(self, outputs, arch, opt='0', identity=None, extra_fragment='', unload=True):
        build = self.builds[arch]
        cargs, rargs = native_arguments(build, False), native_arguments(build, True)
        if not unload:
            variant = self.unload_disabled(arch)
            if variant is not None:
                directory, cfg, metadata = variant
                cargs.insert(1, '-I' + str(directory / 'include'))
                rargs = [('@' + str(cfg)) if value.startswith('@') else value for value in rargs]
                for at, value in enumerate(rargs):
                    if at and rargs[at - 1] == '--extern' and value == 'kernel':
                        rargs[at] = 'kernel=' + str(metadata)
        cdir, rdir = outputs
        cobj, robj = cdir / f'{arch}-{opt}.o', rdir / f'{arch}-{opt}.o'
        run(cargs + ['-O' + opt, '-c', 'fixture.mod.c', '-o', cobj], cwd=cdir)
        run(cargs + ['-O' + opt, '-x', 'c', '-fsyntax-only', 'fixture.mod.h'], cwd=rdir)
        records = run(cargs + ['-O' + opt, '-x', 'c', '-E', '-P',
            '-DLUPOS_RUST_MODULE_RECORDS', '-fmacro-prefix-map=fixture.mod.h=fixture.mod.c',
            'fixture.mod.h'], cwd=rdir)
        generated = run([self.binary, '--rust-module-records'], input=records)
        fragment = rdir / 'fixture.mod.rs'
        fragment.write_bytes(generated + extra_fragment.encode())
        run(rargs + ['-Copt-level=' + opt, '--crate-name=module_metadata_fixture',
            *(['-Cmetadata=' + identity] if identity else []),
            '--out-dir=' + str(rdir), '--emit=obj=' + str(robj), self.wrapper], cwd=rdir,
            env=dict(os.environ, RUSTC_BOOTSTRAP='1', OBJTREE=str(build),
                     MODULE_METADATA_DATA=str(fragment)))
        return cobj, robj

    def compare(self, cobj, robj):
        a, b = elf(cobj), elf(robj)
        self.assertEqual((a['bits'], a['order'], a['machine']), (b['bits'], b['order'], b['machine']))
        metadata = {name for name, section in a['sections'].items()
                    if section['flags'] & 2 and section['header'][5]}
        actual = {name for name, section in b['sections'].items()
                  if section['flags'] & 2 and section['header'][5]}
        self.assertEqual(metadata, actual)
        for name in metadata:
            original, translated = a['sections'][name], b['sections'][name]
            self.assertEqual((original['type'], original['flags']),
                             (translated['type'], translated['flags']), name)
            if name == '.modinfo':
                self.assertEqual(sorted(original['data'].split(b'\0')),
                                 sorted(translated['data'].split(b'\0')))
            else:
                self.assertEqual(original['data'], translated['data'], name)
            self.assertEqual(relocations(a, name), relocations(b, name), name)
            natural = dict(zip(SECTIONS, (8, 4, 1)))
            if name in natural:
                self.assertEqual(translated['align'], natural[name])
                self.assertEqual(translated['header'][4] % natural[name], 0)
                self.assertEqual(relocations(b, name), [])
            else:
                self.assertEqual(original['align'], translated['align'], name)
        expected_imports = {name for name, _, _, index, _, _ in symbols(a) if name and index == 0}
        actual_imports = {name for name, _, _, index, _, _ in symbols(b) if name and index == 0}
        self.assertEqual(expected_imports, actual_imports)
        self.assertNotIn(b'__IS_RUST_MODULE', {name for name, *_ in symbols(b)})
        def definitions(parsed):
            sections = {section['index']: name for name, section in parsed['sections'].items()}
            return {name: (info, other, sections.get(index, index), value, size)
                    for name, info, other, index, value, size in symbols(parsed)
                    if name and index and info >> 4 in (1, 2)}
        c_definitions, r_definitions = definitions(a), definitions(b)
        self.assertEqual(c_definitions, {name: value for name, value in r_definitions.items()
                                       if not name.startswith(b'_R')})
        self.assertEqual(set(c_definitions), {b'__this_module'})
        self.assertEqual(r_definitions[b'__this_module'][:3],
                         (17, 0, '.gnu.linkonce.this_module'))
        # #[used] private Rust statics retain crate-mangled global ELF names.
        # They are metadata, not exported C implementations. Permit only the
        # exact generated families and their full, already-compared bytes.
        suffixes = {b'_____VERSIONS': '__versions',
                    b'_____VERSION_EXT_CRCS': '__version_ext_crcs',
                    b'_____VERSION_EXT_NAMES': '__version_ext_names'}
        for name, (info, other, section, value, size) in r_definitions.items():
            if not name.startswith(b'_R'):
                continue
            self.assertIn(b'module_metadata_fixture', name)
            self.assertEqual((info, other), (17, 0))
            matching = [target for suffix, target in suffixes.items() if name.endswith(suffix)]
            if matching:
                self.assertEqual(section, matching[0])
                self.assertEqual((value, size), (0, len(b['sections'][section]['data'])))
            else:
                self.assertRegex(name, rb'___MODULE_INFO_[0-9]+$')
                self.assertEqual(section, '.modinfo')
                raw = b['sections'][section]['data'][value:value + size]
                self.assertEqual(len(raw), size)
                self.assertEqual(raw[-1:], b'\0')
                self.assertNotIn(b'\0', raw[:-1])
        self.assertFalse(any(section['flags'] & 4 and section['header'][5]
                             for section in b['sections'].values()))

    def test_native_binding_lifecycle_and_versions(self):
        for init in (False, True):
            for cleanup in (False, True):
                for versions in ((), ('-mb',), ('-mx',), ('-mbx',)):
                    label = f'life-{int(init)}-{int(cleanup)}-{len(versions) and versions[0] or "none"}'
                    outputs = self.emit(label, self.inputs(init, cleanup), versions)
                    for arch in self.builds:
                        for opt in ('0', '2'):
                            with self.subTest(init=init, cleanup=cleanup, versions=versions, arch=arch, opt=opt):
                                self.compare(*self.compile(outputs, arch, opt))

    def test_fresh_actual_header_bindings_without_module_unload(self):
        for init in (False, True):
            for cleanup in (False, True):
                outputs = self.emit(f'no-unload-{init}-{cleanup}', self.inputs(init, cleanup), ('-mbx',))
                for arch in self.builds:
                    for opt in ('0', '2'):
                        with self.subTest(init=init, cleanup=cleanup, arch=arch, opt=opt):
                            objects = self.compile(outputs, arch, opt, unload=False)
                            self.compare(*objects)
                            self.assertNotIn(b'cleanup_module', {name for name, _, _, index, _, _
                                             in symbols(elf(objects[1])) if name and index == 0})

    def test_long_import_filtered_only_from_basic_versions(self):
        outputs = self.emit('long', self.inputs(long=True), ('-mbx',))
        for arch in self.builds:
            self.compare(*self.compile(outputs, arch))
        original = (outputs[0] / 'fixture.mod.c').read_text()
        basic = original.split('____versions[]', 1)[1].split('};', 1)[0]
        self.assertNotIn('long_x', basic)
        self.assertIn('long_x', original.split('____version_ext_names[]', 1)[1])
        self.emit('too-long', self.inputs(long=True), ('-mb',), status=1)

    def test_C_Rust_owner_marker_does_not_leak_into_metadata(self):
        for owner in (False, True):
            for license in ('GPL', 'Proprietary', 'Dual BSD/GPL'):
                outputs = self.emit(f'owner-{owner}-{license.replace("/", "-").replace(" ", "-")}',
                    self.inputs(rust_owner=owner, license=license), ('-mbx', '-e'))
                for arch in self.builds:
                    self.compare(*self.compile(outputs, arch))
                # Licenses and original marker belong to owner .o only; the
                # independent metadata object must not invent either.
                fragment = (outputs[1] / 'fixture.mod.rs').read_text()
                self.assertNotIn('__IS_RUST_MODULE', fragment)

    def test_alias_dependency_srcversion_and_original_warning_filters(self):
        for number, alias in enumerate(('platform-name', r'octal\101-name', 'UTF8-Δ{}')):
            outputs = self.emit(f'fields-{number}', self.inputs(alias=alias, unversioned=True), ('-mbx',))
            self.assertEqual(self.last_stderr.count(b"symbol 'unversioned' has no CRC!"), 2)
            self.assertIn(b"EXPORT symbol 'unversioned' version generation failed", self.last_stderr)
            original = (outputs[0] / 'fixture.mod.c').read_text()
            self.assertIn('MODULE_INFO(depends, "z,a,provider");', original)
            # Original caller passes sizeof(srcversion)-1 (24) to snprintf,
            # preserving 23 hexadecimal characters plus the terminator.
            self.assertRegex(original, r'MODULE_INFO\(srcversion, "[0-9A-F]{23}"\);')
            self.assertNotIn('unversioned', original.split('____versions[]', 1)[1].split('};', 1)[0])
            for arch in self.builds:
                self.compare(*self.compile(outputs, arch))

    def test_final_relocatable_link_payload_relocations_and_offsets(self):
        outputs = self.emit('linked', self.inputs(True, True, rust_owner=True,
                            alias='metadata-linked'), ('-mbx',))
        for arch in self.builds:
            objects = self.compile(outputs, arch, '2')
            linked = [path.with_suffix('.linked.o') for path in objects]
            for source, destination in zip(objects, linked):
                run(command('LD_LLD', 'ld.lld') + ['-r', '--build-id=none', source, '-o', destination])
            self.compare(*linked)
            parsed = elf(linked[1])
            for name in ('.gnu.linkonce.this_module', *SECTIONS):
                section = parsed['sections'][name]
                self.assertEqual(section['header'][4] % section['align'], 0, name)

    def test_parallel_metadata_identity_isolation_and_link(self):
        if 'x86' not in self.builds:
            self.skipTest('x86 objcopy link-isolation gate requires MODULE_METADATA_X86_BUILD')
        outputs = [self.emit('parallel-' + str(index), self.inputs(init=bool(index),
                   cleanup=not bool(index)), ('-mbx',)) for index in range(2)]
        with ThreadPoolExecutor(max_workers=2) as pool:
            futures = [pool.submit(self.compile, output, 'x86', '2',
                       str(output[1] / 'fixture.mod.o')) for output in outputs]
            objects = [future.result() for future in futures]
        for pair in objects:
            self.compare(*pair)
        public_names, mangled_names = [], []
        for _, path in objects:
            definitions = {name for name, info, _, index, _, _ in symbols(elf(path))
                           if name and index and info >> 4 in (1, 2)}
            public_names.append({name for name in definitions if not name.startswith(b'_R')})
            mangled_names.append(definitions - public_names[-1])
        self.assertEqual(public_names, [{b'__this_module'}] * 2)
        self.assertTrue(all(mangled_names))
        self.assertFalse(mangled_names[0] & mangled_names[1])
        # Separate real modules each own __this_module. Rename only that
        # deliberately shared owner in private copies to prove all other
        # metadata symbols can link together without accidental coalescing.
        copies = []
        for index, (_, path) in enumerate(objects):
            copy = path.with_suffix('.renamed.o')
            run(command('OBJCOPY', 'objcopy') + ['--redefine-sym',
                f'__this_module=__this_module_{index}', path, copy])
            copies.append(copy)
        linked = self.work / 'parallel-linked.o'
        run(command('LD_LLD', 'ld.lld') + ['-r', '--build-id=none', *copies, '-o', linked])
        result = elf(linked)
        defined = {name for name, info, _, index, _, _ in symbols(result)
                   if name and index and info >> 4 in (1, 2)}
        self.assertEqual(defined, mangled_names[0] | mangled_names[1] |
                         {b'__this_module_0', b'__this_module_1'})
        for name in SECTIONS:
            self.assertEqual(result['sections'][name]['data'],
                b''.join(elf(path)['sections'][name]['data'] for path in copies))
            self.assertEqual(relocations(result, name), [])
        self.assertEqual({name for name, _, _, index, _, _ in symbols(result) if name and index == 0},
                         {b'local_function', b'local_data', b'init_module', b'cleanup_module'})
        print('parallel identity: distinct mangled data symbols', tuple(map(len, mangled_names)),
              '; st_info=17 STB_GLOBAL/STT_OBJECT, st_other=0 STV_DEFAULT; sole original unmangled owner __this_module')

    def test_unexpected_unmangled_global_is_rejected_after_successful_compile(self):
        outputs = self.emit('rogue', self.inputs(), ('-mbx',))
        pair = self.compile(outputs, next(iter(self.builds)), '2', extra_fragment='''
/// Deliberate test-only symbol-table corruption, with no extra payload.
#[used]
#[no_mangle]
pub static UNEXPECTED_METADATA_OWNER: [u8; 0] = [];
''')
        with self.assertRaises(AssertionError):
            self.compare(*pair)


if __name__ == '__main__':
    unittest.main(verbosity=2)
