#!/usr/bin/env python3
"""Actual normalized C/Rust outer+inner rbtree KCFI and nullable domains.

No kernel/VM writes. Real native types/flags and unchanged C are read-only
inputs; new candidate binding declarations are compiled separately. The raw
Option negative is freshly generated from original C, so it remains a negative
after the real selected bindings have been repaired.
"""
import json
import os
from pathlib import Path
import re
import signal
import unittest

from test_rbtree_native import (ROOT,SOURCE,NATIVE,ART,FIXTURES,EXPORTS,run, PrivateTestCase,
    native_command,binding_facade,bindgen_command,c_compile,atomic_helpers, execute as private_execute)


def ids(file):
    text=file.read_text()
    values={n:int(v)&0xffffffff for n,v in re.findall(r'^!(\d+) = !\{i32 (-?\d+)\}',text,re.M)}
    return {n:values[v] for n,v in re.findall(r'^define [^\n]*?@([^ (]+)\([^\n]*!kcfi_type !(\d+)',text,re.M)}


def compile_rust(native,source,out,facade,opt,protected,raw,raw_original):
    args,env=native_command(out,source,native)
    args=['kernel='+str(facade) if a=='kernel' else a for a in args]
    if not protected:
        args=[a for a in args if a not in ('-Zsanitizer=kcfi','-Zsanitizer-cfi-normalize-integers')]
    args+=['-Copt-level='+opt,'--crate-name='+out.stem.replace('-','_').lower(),
           '--emit=llvm-ir='+str(out.with_suffix('.ll'))]
    if source.name=='callback_panic.rs':args+=['--extern','build_error']
    env.update(RBTREE_RAW_BINDINGS=str(raw),RBTREE_RAW_ORIGINAL_BINDINGS=str(raw_original))
    run(args,out.stem+'-compile',env=env)


def compile_c(native,source,out,opt,protected,rename=False,ir=False):
    extra=['-O'+opt]
    if rename:extra+=['-D'+name+'=oracle_'+name for name in EXPORTS]
    if ir:extra+=['-S','-emit-llvm']
    c_compile(source,out,extra,native=native,remove=() if protected else
              ('-fsanitize=kcfi','-fsanitize-cfi-icall-experimental-normalize-integers'))


def execute(binary,args,expected):
    proc=private_execute(binary,args)
    if proc.returncode!=expected:
        raise AssertionError((str(binary),args,proc.returncode,expected,proc.stdout))
    return proc.returncode


class RbtreeCallbacks(PrivateTestCase):
    def prove(self,native,runtime):
        folder=ART/('callbacks-'+native.name);folder.mkdir(parents=True,exist_ok=True)
        raw=folder/'raw.rs'
        run(bindgen_command(native,FIXTURES/'callback_types.c',raw,raw=True),
            'raw-bindgen-'+native.name)
        text=raw.read_text()
        self.assertEqual(text.count('pub fn __rb_'),2)
        self.assertEqual(text.count('::core::option::Option<'),2)
        raw_original=folder/'raw-original.rs'
        raw_original.write_text(re.sub(r'pub fn (__(?:rb_insert_augmented|rb_erase_color))\(',
            lambda m:'#[link_name="oracle_'+m[1]+'"]\npub fn '+m[1]+'(',text))
        facade=binding_facade(native)
        compile_c(native,FIXTURES/'callback_types.c',folder/'types.o','2',True)
        compile_c(native,FIXTURES/'callback_types.c',folder/'types.ll','2',True,ir=True)
        reference=ids(folder/'types.ll');self.assertIn('probe_callback',reference)
        # Compiler-derived normalized grammar, never invented numeric IDs.
        c_compile(FIXTURES/'callback_types.c',folder/'type-grammar.ll',
            ['-flto','-fsanitize=cfi-icall','-S','-emit-llvm'],native=native,remove=('-fsanitize=kcfi',))
        self.assertIn('_ZTSFvP7rb_nodeP7rb_rootPFvS0_S0_EE.normalized',
                      (folder/'type-grammar.ll').read_text())
        result={'runtime':[],'ids':{},'native':str(native)}
        helpers=atomic_helpers(native)
        for opt in ('0','2','s'):
            for protected in (False,True):
                label='O'+opt+('-protected' if protected else '-ordinary')
                original=folder/(label+'-original.o')
                compile_c(native,SOURCE/'lib/rbtree.c',original,opt,protected,rename=True)
                compile_c(native,SOURCE/'lib/rbtree.c',original.with_suffix('.ll'),opt,protected,rename=True,ir=True)
                objects=[]
                for source,name in [(ROOT/'lib/rbtree_rust.rs','owner'),
                                    (FIXTURES/'callback_consumer.rs','consumer')]:
                    obj=folder/(label+'-'+name+'.o');objects.append(obj)
                    compile_rust(native,source,obj,facade,opt,protected,raw,raw_original)
                # Exact pointer/Option representation and real rb_node/root.
                import sys
                sys.path.insert(0,str(SOURCE/'scripts/tests'))
                from test_ctype_translation import elf_symbol
                self.assertEqual(elf_symbol(folder/'types.o',b'c_layout')[1:],
                                 elf_symbol(objects[1],b'rust_layout')[1:])
                if protected:
                    ci,ri=ids(original.with_suffix('.ll')),ids(objects[0].with_suffix('.ll'))
                    body=objects[0].with_suffix('.ll').read_text()
                    functions=dict(re.findall(r'^define [^\n]*?@([^ (]+)\([^\n]*\{\n(.*?)^\}',body,re.M|re.S))
                    for name,helper in [('__rb_insert_augmented','___rb_insert'),
                                        ('__rb_erase_color','_____rb_erase_color')]:
                        self.assertEqual(ci['oracle_'+name],ri[name])
                        reached=functions[name]
                        for callee in re.findall(r'call (?:fastcc )?void @([^ (]+)\(',reached):
                            if helper in callee:reached+=functions[callee]
                        inner={int(n)&0xffffffff for n in re.findall(r'"kcfi"\(i32 (-?\d+)\)',reached)}
                        self.assertIn(reference['probe_callback'],inner)
                    result['ids'][opt]={'outer':ri['__rb_insert_augmented'],'inner':reference['probe_callback']}
                if not runtime:continue
                driver=folder/(label+'-driver.o');panic=folder/(label+'-panic.o')
                compile_c(native,FIXTURES/'callback_driver.c',driver,opt,protected)
                compile_rust(native,FIXTURES/'callback_panic.rs',panic,facade,opt,protected,raw,raw_original)
                binary=folder/label
                run(['clang','-no-pie','-nostartfiles','-Wl,--gc-sections','-Wl,-T,'+str(FIXTURES/'discard.lds'),
                     '-Wl,-e,_start','-o',binary,driver,original,*objects,panic,helpers,
                     native/'rust/core.o',native/'rust/compiler_builtins.o',native/'rust/build_error.o','-lc'],label+'-link')
                execute(binary,[],95)
                execute(binary,['7','c','i'],97) # genuine core panic, not a fake stub
                execute(binary,['8','c','i'],97) # actual build_error.rs, no no-op stub
                for provider in ('c','r'):
                    for operation in ('i','e'):
                        for mode in range(7):
                            expected=-signal.SIGILL if protected and mode in (4,5,6) else 0
                            status=execute(binary,[str(mode),provider,operation],expected)
                            result['runtime'].append([opt,protected,provider,operation,mode,status])
        (folder/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
        if runtime:
            self.assertEqual(len(result['runtime']),168)
            self.assertEqual(sum(item[-1]==-signal.SIGILL for item in result['runtime']),36)

    def test_x86_actual_nullable_and_inner_controls(self):
        self.prove(NATIVE,True)

    def test_arm64_actual_nullable_and_inner_codegen(self):
        if 'RBTREE_ARM64' not in os.environ:self.skipTest('optional RBTREE_ARM64 not supplied')
        if not os.environ['RBTREE_ARM64']:self.fail('RBTREE_ARM64 explicitly empty')
        self.prove(Path(os.environ['RBTREE_ARM64']),False)


if __name__=='__main__':unittest.main()
