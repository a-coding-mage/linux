#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Selectable final-vmlinux metadata: original frontend fields, Rust tables.

All C fixtures and compiled artifacts live in temporary test directories.
The unchanged original modpost/C output is the oracle. Optional existing-kernel
objects are read only. Explicit i686 sysroot errors fail rather than skip.
"""
import os
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[2]
from modpost_test_support import Elf,build,modpost_tools
import test_modpost
from test_kunit_parameters import macro
from rust_exports_test_support import rust_targets

import test_export_internal as metadata
HEADER=ROOT/'include/linux/export-internal_header.rs'
tools=modpost_tools


class DefaultCParity(test_modpost.ModpostTests):
    @classmethod
    def setUpClass(cls): cls.tools=tools()


class RustVmlinuxTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='vmlinux-export-proof-')
        self.addCleanup(self.tmp.cleanup)
        self.work=Path(self.tmp.name)
        self.tools=tools()
        self.frontend_extra=[]

    def fixture(self,bits,alias='platform-name',builtin='builtin',namespace='NS_Δ {braces}',extra=False):
        elf=Elf(bits)
        elf.export('function',namespace,gpl=True)
        elf.export('data',kind=1)
        if extra:elf.export('module_layout')
        size=self.tools[bits][2]['SIZE_platform_device_id']
        name=alias.encode()
        if len(name)>=20: raise AssertionError('platform name fixture too long')
        data=name+bytes(size*2-len(name))
        section=elf.section('.rodata',data)
        elf.symbol(f'__mod_device_table__kmod_{builtin}__platform__ids',section,size=len(data),kind=1)
        files={'vmlinux.o':elf.build(),'.vmlinux.objs':'source.o\n',
            '.source.o.cmd':'\n#SYMVER function 0x12345678\n#SYMVER data 0x87654321\n#SYMVER module_layout 0x11223344\n'}
        if extra:
            module=Elf(bits); module.module_info(imports=(namespace,)); module.export('other');module.symbol('function')
            files['other.o']=module.build();files['other.mod']='part.o\n';files['.part.o.cmd']='\n#SYMVER other 0xaabbccdd\n'
        return files

    def emit(self,bits,files,rust,args=()):
        out=self.work/('rust' if rust else 'c')
        out.mkdir(exist_ok=True)
        for name,data in files.items():
            p=out/name;p.parent.mkdir(parents=True,exist_ok=True)
            p.write_bytes(data.encode() if isinstance(data,str) else data)
        command=[str(self.tools[bits][int(rust)]),'-M',*(['--rust-vmlinux-export'] if rust else []),
            *args,'vmlinux.o',*(['other.o'] if 'other.o' in files else []),'-o','Module.symvers']
        result=subprocess.run(command,cwd=out,capture_output=True)
        return out,result

    def compile(self,out,bits,rust,prel=False,opt='2',cc='gcc'):
        include=out/'include/linux';include.mkdir(parents=True,exist_ok=True)
        (include/'compiler.h').write_text('#define __used __attribute__((used))\n#define __section(x) __attribute__((section(x)))\n#define __aligned(x) __attribute__((aligned(x)))\n')
        (include/'types.h').write_text('typedef unsigned int u32;\n')
        (include/'module.h').write_text('#include <linux/compiler.h>\n#include <linux/stringify.h>\n#define static_assert _Static_assert\n'
            '#define JOIN_(a,b) a##b\n#define JOIN(a,b) JOIN_(a,b)\n#define __UNIQUE_ID(x) JOIN(x,__COUNTER__)\n#define __MODULE_INFO_PREFIX ""\n'+
            macro((ROOT/'include/linux/moduleparam.h').read_text(),'MODULE_INFO'))
        obj=out/f'meta-{cc}-{bits}-{int(prel)}-{opt}.o'
        cfg=(['CONFIG_64BIT'] if bits==64 else [])+(['CONFIG_HAVE_ARCH_PREL32_RELOCATIONS'] if prel else [])
        frontend=[cc,f'-m{bits}','-O'+opt,'-I'+str(include.parent),'-I'+str(ROOT/'include')]
        if cc=='gcc':frontend+=['-fno-toplevel-reorder']
        frontend+=['-D'+flag for flag in cfg]+self.frontend_extra
        if rust:
            build(frontend+['-x','c','-fsyntax-only','.vmlinux.export.h'],cwd=out)
            records=subprocess.run(frontend+['-x','c','-E','-P','-DLUPOS_RUST_VMLINUX_RECORDS',
                '-fmacro-prefix-map=.vmlinux.export.h=.vmlinux.export.c','.vmlinux.export.h'],cwd=out,capture_output=True)
            self.assertEqual(records.returncode,0,records.stderr)
            parsed=subprocess.run([str(self.tools[bits][1]),'--rust-vmlinux-records'],input=records.stdout,capture_output=True)
            self.assertEqual(parsed.returncode,0,parsed.stderr)
            (out/'.vmlinux.export.rs').write_bytes(parsed.stdout)
            wrapper=out/'wrapper.rs'
            wrapper.write_text('#![no_std]\n#[path="'+str(HEADER)+'"] mod internal;\ninclude!(env!("VMLINUX_EXPORT_DATA"));\n')
            cmd=shlex.split(os.environ.get('HOSTRUSTC','rustc'))+['--edition=2021','--crate-name=vmlinux_export','--crate-type=rlib',
                '--emit=obj','-Copt-level='+opt,'-Cpanic=abort','-Dwarnings',*rust_targets()[bits]]
            for flag in cfg:cmd+=['--cfg',flag]
            build(cmd+[str(wrapper),'-o',str(obj)],env={**os.environ,'VMLINUX_EXPORT_DATA':str(out/'.vmlinux.export.rs')})
        else:
            build(frontend+['-c','.vmlinux.export.c','-o',str(obj)],cwd=out)
        return obj

    def records(self,obj):
        data=obj.read_bytes();wide=data[4]==2;order='<' if data[5]==1 else '>'
        unpack=lambda fmt,at:struct.unpack_from(order+fmt,data,at)
        offset=unpack('Q' if wide else 'I',40 if wide else 32)[0]
        size,count,names=unpack('HHH',58 if wide else 46)
        sections=[unpack('IIQQQQIIQQ' if wide else '10I',offset+i*size) for i in range(count)]
        raw=lambda s:data[s[4]:s[4]+s[5]]
        names=raw(sections[names])
        return [raw(s) for s in sections if names[s[0]:names.index(0,s[0])]==b'.modinfo']

    def test_selection_module_outputs_versions_trim_and_noop(self):
        for bits in (32,64):
            for flags in ([],['-m'],['-mt','-u','white']):
                with self.subTest(bits=bits,flags=flags):
                    files=self.fixture(bits,extra=True);files['white']='function\n'
                    c,cr=self.emit(bits,files,False,flags);r,rr=self.emit(bits,files,True,flags)
                    self.assertEqual((cr.returncode,rr.returncode),(0,0),(cr.stderr,rr.stderr))
                    self.assertEqual(cr.stderr,rr.stderr)
                    self.assertEqual((c/'Module.symvers').read_bytes(),(r/'Module.symvers').read_bytes())
                    self.assertEqual((c/'other.mod.c').read_bytes(),(r/'other.mod.c').read_bytes())
                    self.assertFalse((r/'.vmlinux.export.c').exists())
                    data=r/'.vmlinux.export.h';old=(data.read_bytes(),data.stat().st_mtime_ns)
                    self.emit(bits,files,True,flags)
                    self.assertEqual(old,(data.read_bytes(),data.stat().st_mtime_ns))

    def test_real_32_64_prel_absolute_tables_alias_bytes_and_no_owner_artifacts(self):
        for bits in rust_targets():
            files=self.fixture(bits)
            c,cr=self.emit(bits,files,False,['-m']);r,rr=self.emit(bits,files,True,['-m'])
            self.assertEqual((cr.returncode,rr.returncode),(0,0))
            for prel in (False,True):
                for opt in ('0','2','s'):
                    native=self.compile(r,bits,True,prel,opt)
                    for cc in ('gcc','clang'):
                        original=self.compile(c,bits,False,prel,opt,cc)
                        self.assertEqual(metadata.elf(original)[:3],metadata.elf(native)[:3])
                        self.assertEqual(self.records(original),self.records(native))
                    self.assertNotIn(b'__IS_RUST_MODULE',native.read_bytes())
                    self.assertFalse({b'function',b'data'} & metadata.elf(native)[4])

    def test_accepted_alias_C_escapes_match_original_MODULE_INFO(self):
        for alias in ('plain','UTF8_Δ{}',r'a\nb',r'a\tb',r'a\\b',r'a\"b',r'a\101b',r'a\x41-z',r'a\?b',r'big\777',r'big\x123',r'wide\u00e9',r'ext\e',r'unknown\q'):
            with self.subTest(alias=alias):
                files=self.fixture(64,alias=alias)
                c,cr=self.emit(64,files,False);r,rr=self.emit(64,files,True)
                self.assertEqual((cr.returncode,rr.returncode),(0,0),rr.stderr)
                for cc in ('gcc','clang'):
                    if cc=='clang' and alias in (r'big\777',r'big\x123'):
                        with self.assertRaises(RuntimeError):self.compile(c,64,False,cc=cc)
                        with self.assertRaises(RuntimeError):self.compile(r,64,True,cc=cc)
                        continue
                    native=self.compile(r,64,True,cc=cc)
                    original=self.compile(c,64,False,cc=cc)
                    self.assertEqual(self.records(original),self.records(native))

    def test_namespace_preserves_C_literal_then_assembler_escaping(self):
        for namespace in ('UTF8_Δ{}',r'ns\tend',r'ns\\tend',r'ns\101end',r'ns\\101end',r'ns\\\\end'):
            with self.subTest(namespace=namespace):
                files=self.fixture(64,namespace=namespace)
                c,cr=self.emit(64,files,False);r,rr=self.emit(64,files,True)
                self.assertEqual((cr.returncode,rr.returncode),(0,0),rr.stderr)
                native=self.compile(r,64,True)
                for cc in ('gcc','clang'):
                    original=self.compile(c,64,False,cc=cc)
                    self.assertEqual(metadata.elf(original)[:3],metadata.elf(native)[:3])

    def test_frontend_rejects_invalid_alias_literals(self):
        for alias in (r'null\0',r'null\x00','quote"','trail\\'):
            with self.subTest(alias=alias):
                out,result=self.emit(64,self.fixture(64,alias=alias),True)
                self.assertEqual(result.returncode,0,result.stderr)
                with self.assertRaises(RuntimeError): self.compile(out,64,True)

    def test_macro_collisions_counters_and_logical_source_names(self):
        for name in ('linux','__GNUC__','__COUNTER__','__LINE__','__FILE__','__BASE_FILE__','HEADER_MACRO'):
            with self.subTest(name=name):
                self.frontend_extra=['-DHEADER_MACRO=expanded']
                files=self.fixture(64,builtin=name)
                c,cr=self.emit(64,files,False);r,rr=self.emit(64,files,True)
                self.assertEqual((cr.returncode,rr.returncode),(0,0),rr.stderr)
                # Multiple declarations expose expansion-state drift.
                cdata=(c/'.vmlinux.export.c').read_text()
                lines=[line for line in cdata.splitlines() if line.startswith('MODULE_INFO(')]
                for out,suffix in ((c,'c'),(r,'h')):
                    path=out/('.vmlinux.export.'+suffix)
                    data=path.read_text()
                    if suffix=='h':
                        data=data.replace('LUPOS_EXPORT_COUNT 5;','LUPOS_EXPORT_COUNT 7;')
                        data=data.replace('#ifdef LUPOS_RUST_VMLINUX_RECORDS\nLUPOS_EXPORT_END;', '\n'.join(lines*2)+'\n#ifdef LUPOS_RUST_VMLINUX_RECORDS\nLUPOS_EXPORT_END;')
                    else:data+='\n'.join(lines*2)+'\n'
                    path.write_text(data)
                for cc in ('gcc','clang'):
                    native=self.compile(r,64,True,cc=cc)
                    original=self.compile(c,64,False,cc=cc)
                    self.assertEqual(self.records(original),self.records(native))

    def test_export_macro_collision_preserves_distinct_table_and_CRC_names(self):
        for cc in ('gcc','clang'):
            files=self.fixture(64)
            c,_=self.emit(64,files,False,['-m']);r,_=self.emit(64,files,True,['-m'])
            self.frontend_extra=['-Dfunction=linux','-Ddata=renamed_data']
            original=self.compile(c,64,False,cc=cc)
            native=self.compile(r,64,True,cc=cc)
            self.assertEqual(metadata.elf(original)[:3],metadata.elf(native)[:3])

    def test_record_protocol_rejects_missing_duplicate_wrong_charset_and_NUL(self):
        valid=b'LUPOS_EXPORT_COUNT 1; LUPOS_EXPORT_CHARSET "UTF-8"; LUPOS_EXPORT_INPUT_UTF8 "\xc3\xa9"; LUPOS_EXPORT_LOGICAL_SOURCE "logical.c", "logical.c"; LUPOS_EXPORT_ALIAS "x.alias=y"; LUPOS_EXPORT_END;'
        command=[str(self.tools[64][1]),'--rust-vmlinux-records']
        self.assertEqual(subprocess.run(command,input=valid,capture_output=True).returncode,0)
        for bad in (valid.replace(b'COUNT 1',b'COUNT 2'),valid.replace(b'LUPOS_EXPORT_END;',b''),
            valid+valid,valid.replace(b'UTF-8',b'ISO-8859-1'),valid.replace(b'\xc3\xa9',b'\xc3\x83\xc2\xa9'),
            valid.replace(b'x.alias=y',b'x.alias=\\0y'),valid.replace(b'ALIAS',b'UNKNOWN'),
            valid.replace(b'"logical.c", "logical.c"',b'"logical.c", "logical.h"'),
            valid.replace(b'ALIAS "x.alias=y";',b'ALIAS "unterminated;')):
            with self.subTest(bad=bad):self.assertNotEqual(subprocess.run(command,input=bad,capture_output=True).returncode,0)

    def test_actual_frontend_charset_and_filename_mappings_fail_closed(self):
        for flags,message in ((['-fexec-charset=ISO-8859-1'],'execution encoding'),
                (['-finput-charset=ISO-8859-1'],'input encoding'),
                (['-fmacro-prefix-map=.vmlinux.export.c=renamed.c'],'source filename mapping')):
            with self.subTest(flags=flags):
                out,result=self.emit(64,self.fixture(64,namespace='ASCII'),True)
                self.assertEqual(result.returncode,0,result.stderr)
                self.frontend_extra=flags
                with self.assertRaisesRegex(AssertionError,message):self.compile(out,64,True)

    def test_empty_export_set_and_parser_mode_argument_contract(self):
        files={'vmlinux.o':Elf(64).build(),'.vmlinux.objs':''}
        c,cr=self.emit(64,files,False);r,rr=self.emit(64,files,True)
        self.assertEqual((cr.returncode,rr.returncode),(0,0),(cr.stderr,rr.stderr))
        original=self.compile(c,64,False);native=self.compile(r,64,True)
        self.assertEqual(metadata.elf(original)[:3],metadata.elf(native)[:3])
        result=subprocess.run([str(self.tools[64][1]),'--rust-vmlinux-records','unexpected'],capture_output=True)
        self.assertNotEqual(result.returncode,0)
        self.assertIn(b'accepts no arguments',result.stderr)


if __name__=='__main__': unittest.main()
