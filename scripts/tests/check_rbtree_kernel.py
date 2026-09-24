#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit selected rbtree linkage and run private GPL callers in a guest.

Kernel/source trees are read-only inputs. Only a new external --work directory
may receive logs, module objects, compiler temporaries or boot artifacts.
"""
import argparse
from contextlib import contextmanager
import json
import math
import os
from pathlib import Path
import re
import selectors
import shlex
import shutil
import struct
import subprocess
import sys
import tempfile
import time

from boot_kernel import (module_name,verify_module_events,init_command,qemu_command,
    architecture as boot_architecture,MARKER)
from check_cmdline_kernel import verify_external_command, verify_strict_lints
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_int_log_kernel import normalize_console_transport,kunit_runs,verify_kunit_warnings
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer, verify_module
from check_prime_numbers_kernel import (provider_type_ids, verify_module_import_versions,
    verify_rust_entrypoints, verify_guarded_calls)
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports
from rbtree_native.transport import (NativeWriteWatch, compiler_environment, explicit_path,
    native_flags, outside, require_rustc, verify_flag_policy,validate_compiler_outputs)
from rbtree_runtime_fixtures import (EXPORTS,GPL_ONLY,DESCRIPTION,marker,caller_source,
    reference_source,init_source,STEPS)

ROOT = Path(os.environ.get('RBTREE_RUNTIME_SOURCE_ROOT', Path(__file__).resolve().parents[2])).resolve()
WRAPPERS = tuple(('rbtree_call_'+name,name) for name in EXPORTS)


@contextmanager
def observed_build(build,work):
    watch=NativeWriteWatch(build)
    try:
        with watch:
            yield watch
    finally:
        observation=dict(root=str(build),directories=len(getattr(watch,'paths',{})),writes=watch.events)
        (work/'donor-observation.json').write_text(json.dumps(observation,indent=2)+'\n')
        if watch.events: raise ValueError('selected audit wrote to native donor')


def linked_type_ids(path,names,arch,*,locations=None):
    """Read real KCFI prefixes from ET_EXEC, preserving virtual/file offsets.

    The shared module decoder intentionally accepts only ET_REL, so final
    vmlinux needs this bounded ELF64 little-endian symbol/section reader.
    """
    if arch not in ('x86_64','aarch64'): raise ValueError('unsupported final ELF architecture')
    data=path.read_bytes()
    if len(data)<64 or data[:6]!=b'\x7fELF\x02\x01' or struct.unpack_from('<HH',data,16)!=(2,183 if arch=='aarch64' else 62):
        raise ValueError('wrong final vmlinux ELF architecture/type')
    offset=struct.unpack_from('<Q',data,40)[0]
    stride,count=struct.unpack_from('<HH',data,58)
    if stride!=64 or offset>len(data)-64: raise ValueError('invalid final ELF section table')
    if count==0: count=struct.unpack_from('<Q',data,offset+32)[0]
    if count>(len(data)-offset)//stride: raise ValueError('truncated final ELF sections')
    sections=[struct.unpack_from('<IIQQQQIIQQ',data,offset+i*stride) for i in range(count)]
    def payload(section):
        start,size=section[4:6]
        if start>len(data) or size>len(data)-start: raise ValueError('truncated final ELF section')
        return data[start:start+size]
    tables=[section for section in sections if section[1]==2]
    if len(tables)!=1: raise ValueError('missing/ambiguous final ELF symbol table')
    table=tables[0]
    if table[6]>=count or sections[table[6]][1]!=3 or table[9]!=24:
        raise ValueError('invalid final ELF symbols/string-table link')
    strings=payload(sections[table[6]]); symbols=payload(table)
    if len(symbols)%24: raise ValueError('truncated final symbol table')
    result={}
    for at in range(0,len(symbols),24):
        name,info,_,index,value,size=struct.unpack_from('<IBBHQQ',symbols,at)
        end=strings.find(b'\0',name)
        if name>=len(strings) or end<0: raise ValueError('invalid final symbol name')
        symbol=strings[name:end].decode(errors='surrogateescape')
        if symbol not in names or info&15!=2 or info>>4 not in (1,2): continue
        if symbol in result or not 0<index<count: raise ValueError('duplicate/undefined final export')
        section=sections[index]; relative=value-section[3]; content=payload(section)
        if section[1]!=1 or section[2]&6!=6: raise ValueError('final function section is not allocated executable PROGBITS')
        if relative<4 or size==0 or relative>len(content) or size>len(content)-relative:
            raise ValueError('missing final KCFI prefix or invalid function size')
        result[symbol]=int.from_bytes(content[relative-4:relative],'little')
        if locations is not None:
            locations[symbol]={'prefix':section[4]+relative-4,'symbol':table[4]+at,
                'section':offset+index*stride,'strings':offset+table[6]*stride}
    if set(result)!=set(names): raise ValueError('missing final KCFI functions')
    return result


def verify_final_types(path,arch,expected):
    if linked_type_ids(path,expected,arch)!=expected:
        raise ValueError('final rbtree KCFI identity differs from selected owner')


def command_words(value,name):
    if not value.strip() or '\0' in value: raise ValueError(name+' must be nonempty and contain no NUL')
    words=shlex.split(value)
    if not words or not words[0]: raise ValueError(name+' must name an executable')
    executable=shutil.which(words[0])
    if executable is None: raise ValueError(name+' executable does not exist')
    return [os.path.abspath(executable),*words[1:]]


def run_logged(command,log,*,input=None,env=None):
    work=log.parent.resolve()
    temporary=work/'tmp'; temporary.mkdir(exist_ok=True)
    environment=compiler_environment(work,dict(os.environ) if env is None else env)
    header=('cwd='+str(work)+'\nTMPDIR='+str(temporary)+'\n'+shlex.join([str(p) for p in command])+'\n').encode()
    log.write_bytes(header)
    try:
        result=subprocess.run([str(p) for p in command],cwd=work,env=environment,input=input,
            stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=240)
    except (OSError,subprocess.TimeoutExpired) as error:
        log.write_bytes(header+(getattr(error,'stdout',None) or b'')+b'\nstderr:\n'+
            (getattr(error,'stderr',None) or b'')+('\ntransport-error='+str(error)+'\n').encode())
        raise ValueError('command transport failed; retained '+str(log)) from error
    log.write_bytes(header+result.stdout+b'\nstderr:\n'+result.stderr+f'\nexit={result.returncode}\n'.encode())
    if result.returncode: raise ValueError('command failed; retained '+str(log))
    return result


def verify_source(root):
    original=(root/'lib/rbtree.c').read_text()
    records=re.findall(r'^EXPORT_SYMBOL(_GPL)?\((\w+)\);',original,re.M)
    if [name for _,name in records]!=list(EXPORTS) or {name for gpl,name in records if gpl}!=GPL_ONLY:
        raise ValueError('original eleven rbtree exports/licenses changed')
    test=(root/'lib/rbtree_test.c').read_text()
    if not re.search(r'basic_check\(\);\s*augmented_check\(\);\s*kfree\(nodes\);\s*return -EAGAIN;',test):
        raise ValueError('original rbtree test lifecycle changed')
    if 'kunit_test_suite' in test or 'MODULE_LICENSE("GPL")' not in test:
        raise ValueError('original rbtree test is not the expected GPL non-KUnit module')


def provider(config):
    architecture(config)
    if config.get('UML')=='y': raise ValueError('system QEMU cannot boot UML')
    if config.get('RUST_RBTREE','n') not in ('n','y'): raise ValueError('invalid rbtree selection')
    if config.get('RUST_RBTREE')=='y' and config.get('RUST')!='y': raise ValueError('Rust rbtree requires RUST')
    return 'Rust' if config.get('RUST_RBTREE')=='y' else 'C'


def owner_path(build,selection):
    if selection not in ('Rust','C'): raise ValueError('unknown rbtree owner')
    return build/('lib/rbtree_rust.o' if selection=='Rust' else 'lib/rbtree.o')


def selected_versions(build):
    records=[line.split() for line in (build/'Module.symvers').read_bytes().splitlines()]
    if any(len(row)>1 and row[1] in (b'rb_first',b'rb_last') for row in records):
        raise ValueError('inline rb_first/rb_last must not become exports')
    versions={}
    for name in EXPORTS:
        rows=[row for row in records if len(row)>1 and row[1]==name.encode()]
        license=b'EXPORT_SYMBOL_GPL' if name in GPL_ONLY else b'EXPORT_SYMBOL'
        if len(rows)!=1 or len(rows[0])!=4 or rows[0][2:]!=[b'vmlinux',license] or not re.fullmatch(rb'0x[0-9a-fA-F]{8}',rows[0][0]):
            raise ValueError('wrong defining owner/license/namespace/CRC for '+name)
        versions[name.encode()]=rows[0][0].lower()
    return versions


def verify_bindings(build,root):
    generated=build/'rust/bindings/bindings_generated.rs'
    headers=[root/'include/linux'/name for name in ('rbtree.h','rbtree_types.h','rbtree_augmented.h')]
    deps=verify_external_command(build,generated,root/'rust/bindings/bindings_helper.h',headers)
    newer(generated,[*deps,root/'rust/bindgen_parameters'])
    text=generated.read_text()
    if re.search(r'pub fn __(?:rb_insert_augmented|rb_erase_color)\(',text):
        raise ValueError('selected bindings retain incorrect nominal Option declarations')
    if any('pub struct '+name not in text for name in ('rb_node','rb_root','rb_node_linked','rb_root_linked','rb_augment_callbacks')):
        raise ValueError('selected actual generated rbtree structures missing')
    obj=build/'rust/bindings.o'; metadata=build/'rust/libbindings.rmeta'
    required=[generated,root/'rust/bindings/rbtree.rs']
    deps=verify_external_command(build,obj,root/'rust/bindings/lib.rs',required)
    flags=compilation_flags(obj)
    emissions=[part.split('=',1)[1] for flag in flags if flag.startswith('--emit=')
               for part in flag[7:].split(',') if part.startswith('metadata=')]
    if len(emissions)!=1 or (build/emissions[0]).resolve()!=metadata.resolve():
        raise ValueError('selected bindings command emits different metadata')
    newer(metadata,[*deps,build/'include/generated/rustc_cfg'])
    newer(build/'rust/libkernel.rmeta',[metadata])


def verify_original_test(build,root,arch):
    obj=build/'lib/rbtree_test.o'; module=obj.with_suffix('.ko')
    verify_build_command(build,obj,root/'lib/rbtree_test.c',
        [root/'include/linux/rbtree.h',root/'include/linux/rbtree_augmented.h'])
    verify_flag_policy(native_flags(compilation_flags(obj)[1:],build,'c'),'c')
    verify_module(build,module,obj,'rbtree_test',arch)
    verify_module_metadata(build,module)
    verify_module_import_versions(build,module)
    fields=metadata_fields(module)
    require_metadata_field(fields,b'license',b'GPL')
    require_metadata_field(fields,b'description',b'Red Black Tree test')
    if read_exports(module): raise ValueError('original rbtree test exports symbols')
    verify_references(module,('__rb_insert_augmented','__rb_erase_color','rb_insert_color',
        'rb_erase','rb_next','rb_first_postorder','rb_next_postorder'))
    return module


def verify_selected(build,*,source=None,work=None,allow_c=False):
    """Read-only selected-kernel gate; output only retained private DWARF logs."""
    build=explicit_path(build,'build'); root=explicit_path(source or ROOT,'source')
    verify_source(root)
    config=configuration(build); arch=architecture(config); selection=provider(config)
    if selection=='C' and not allow_c: raise ValueError('requires CONFIG_RUST_RBTREE=y')
    for name in ('CFI','MODVERSIONS','GENDWARFKSYMS','MODULES'):
        if config.get(name)!='y': raise ValueError('requires CONFIG_'+name+'=y; no incomplete CFI/version proof')
    if config.get('RBTREE_TEST')!='m': raise ValueError('requires original CONFIG_RBTREE_TEST=m')
    if config.get('CFI_PERMISSIVE')=='y': raise ValueError('strict non-permissive CFI required')
    if selection=='Rust' and int(config.get('RUSTC_VERSION','0'))<108500:
        raise ValueError('selected Rust owner requires native Rust >=1.85')
    owner=owner_path(build,selection)
    image=build/('arch/arm64/boot/Image' if arch=='aarch64' else 'arch/x86/boot/bzImage')
    for path in (owner,build/'lib/lib.a',build/'vmlinux.a',build/'vmlinux.o',build/'vmlinux',
                 build/'Module.symvers',image,build/'lib/rbtree_test.ko'):
        if not path.is_file(): raise ValueError('missing selected artifact: '+str(path))
    if work is None:
        parent=explicit_path(os.environ.get('RBTREE_RUNTIME_LOG_DIR',tempfile.gettempdir()),'log parent')
        outside(parent,(build,root,Path(__file__).resolve().parents[2]))
        if not parent.is_dir(): raise ValueError('private log parent must already exist')
        work=Path(tempfile.mkdtemp(prefix='rbtree-selected-',dir=parent))
    else:
        work=outside(explicit_path(work,'work'),(build,root))
        if not work.is_dir(): raise ValueError('private verification work must exist')
    print('Retained rbtree selected audit: '+str(work),flush=True)
    with observed_build(build,work):
        candidates={owner_path(build,kind).resolve() for kind in ('C','Rust')}
        for archive in (build/'lib/lib.a',build/'vmlinux.a'):
            members=[(build/os.fsdecode(line)).resolve() for line in tool('ar','t',archive).splitlines()]
            if [path for path in members if path in candidates]!=[owner.resolve()]:
                raise ValueError('selected rbtree archive owner absent/mixed/duplicated')
            if archive.name=='lib.a' and [str(p) for p in members]!=sorted(set(str(p) for p in members)):
                raise ValueError('lib-y archive lost original unique sorted order')
            newer(archive,[owner])
        source_file=root/('lib/rbtree_rust.rs' if selection=='Rust' else 'lib/rbtree.c')
        deps=[root/'include/linux'/name for name in ('rbtree.h','rbtree_types.h','rbtree_augmented.h')]
        if selection=='Rust':
            deps=[root/path for path in ('lib/rbtree.rs','include/linux/rbtree_header.rs',
                'include/linux/rbtree_types_header.rs','include/linux/rbtree_augmented_header.rs',
                'rust/ffi_export.rs','include/linux/export_header.rs')]
            deps += [build/'rust/libkernel.rmeta',build/'rust/libbindings.rmeta']
        verify_build_command(build,owner,source_file,deps)
        flags=compilation_flags(owner)
        command=flags.copy()
        while command and '=' in command[0] and not command[0].startswith('-'): command.pop(0)
        real_flags=native_flags(command[1:],build,'rust' if selection=='Rust' else 'c')
        verify_flag_policy(real_flags,'rust' if selection=='Rust' else 'c')
        if selection=='Rust':
            verify_strict_lints(real_flags)
            if [f for f in flags if f.startswith('RUST_MODFILE=')]!=['RUST_MODFILE=lib/rbtree']:
                raise ValueError('selected rbtree lost original modfile ownership')
            require_rustc(command[0],work,compiler_environment(work,dict(os.environ)))
            verify_bindings(build,root)
        elf_target(owner,arch)
        exports=read_exports(owner)
        if len(exports)!=11 or {row['name'] for row in exports}!=set(EXPORTS):
            raise ValueError('selected defining object lacks exactly eleven exports')
        for row in exports:
            expected=dict(license='GPL' if row['name'] in GPL_ONLY else '',namespace='',
                relocation_target=row['name'],relocation_addend=0,pointer_width=8,
                relocation_kind=257 if arch=='aarch64' else 1,label_binding=0,label_kind=0,
                section_flags=2,section_alignment=8)
            if any(row[key]!=value for key,value in expected.items()):
                raise ValueError('rbtree actual export metadata differs: '+row['name'])
        types=provider_type_ids(owner,names=EXPORTS)
        for path in (owner,build/'vmlinux.o',build/'vmlinux'):
            definitions=[row.split() for row in tool('nm','-g','--defined-only',path).splitlines()
                         if row.split() and row.split()[-1].decode() in EXPORTS]
            if len(definitions)!=11 or {row[-1] for row in definitions}!={n.encode() for n in EXPORTS} or any(row[-2]!=b'T' for row in definitions):
                raise ValueError('missing/duplicate/non-function final rbtree definition')
            if path.name=='vmlinux':
                verify_final_types(path,arch,types)
                continue
            actual=provider_type_ids(path,names=EXPORTS)
            if actual!=types:
                raise ValueError('final rbtree KCFI identity differs from selected owner')
        versions=selected_versions(build)
        if version_records(owner.with_name('.'+owner.name+'.cmd'))!=versions:
            raise ValueError('selected rbtree versions differ from actual defining-command records')
        proof=run_logged([build/'scripts/gendwarfksyms/gendwarfksyms','--symtypes',work/'rbtree.symtypes',owner],
            work/'defining-dwarf.log',input=b''.join(n.encode()+b'\n' for n in EXPORTS))
        rows=[line.split() for line in proof.stdout.splitlines()]
        if proof.stderr or len(rows)!=11 or any(len(row)!=3 or row[0]!=b'#SYMVER' for row in rows) or {row[1]:row[2].lower() for row in rows}!=versions:
            raise ValueError('real rbtree defining-object DWARF does not reproduce selected versions')
        if not (work/'rbtree.symtypes').read_bytes(): raise ValueError('missing actual rbtree DWARF types')
        original=verify_original_test(build,root,arch)
        newer(build/'Module.symvers',[owner,build/'lib/rbtree_test.o'])
        newer(build/'vmlinux.o',[build/'lib/lib.a',build/'vmlinux.a'])
        newer(build/'vmlinux',[build/'vmlinux.o']); newer(image,[build/'vmlinux'])
    result=dict(build=str(build),source=str(root),selection=selection,architecture=arch,owner=str(owner),
        original_test=str(original),versions={k.decode():v.decode() for k,v in versions.items()},
        kcfi=types,work=str(work),runtime=False)
    (work/'selected.json').write_text(json.dumps(result,indent=2)+'\n')
    return result


def caller_names(caller):
    marker(caller)
    return ('rbtree_rust_abi','rbtree_rust_main') if caller=='rust' else ('rbtree_abi','rbtree_c_main')


def verify_rust_workload(obj,module=None):
    imports={row.split()[-1] for row in tool('nm','-u',obj).splitlines() if row.split()}
    required={name.encode() for name in EXPORTS}|{('oracle_'+name).encode() for name in EXPORTS}
    required|={b'oracle_edge',b'oracle_add_linked',b'oracle_erase_aug',b'_printk'}
    intrinsic={name for name in imports if re.fullmatch(rb'_R\w*4core\w+',name)}
    memory={b'memset',b'memcpy',b'memmove',b'bcmp',b'memcmp'}
    atomic={('rust_helper_atomic_ptr_'+operation).encode() for operation in ('read','read_acquire','set','set_release')}
    if not required<=imports or imports-required-intrinsic-memory-atomic:
        raise ValueError('Rust rbtree imports indicate missing workload or C dispatch delegation')
    for path in (obj,*([module] if module is not None else [])):
        definitions={row.split()[-1] for row in tool('nm','-g','--defined-only',path).splitlines() if row.split()}
        if b'rbtree_rust_exercise' not in definitions or b'test_main' in definitions:
            raise ValueError('Rust caller does not own its independent state machine')


def verify_consumer(build,work,caller,*,source=None):
    root=source or ROOT; name,stem=caller_names(caller)
    obj=work/(stem+'.o');module=work/(name+'.ko');oracle=work/'rbtree_reference.o'
    config=configuration(build);arch=architecture(config)
    supplied=obj.with_suffix('.rs' if caller=='rust' else '.c')
    if supplied.read_text()!=caller_source(root,caller) or oracle.with_suffix('.c').read_text()!=reference_source(root):
        raise ValueError('independent caller/original C oracle source changed')
    deps=[root/'include/linux/rbtree.h',root/'include/linux/rbtree_augmented.h']
    if caller=='rust': deps=[build/'rust/libkernel.rmeta',build/'rust/libbindings.rmeta']+[
        root/'include/linux'/name for name in ('rbtree_header.rs','rbtree_types_header.rs','rbtree_augmented_header.rs')]
    verify_external_command(work,obj,supplied,deps)
    verify_external_command(work,oracle,oracle.with_suffix('.c'),[root/'lib/rbtree.c',root/'include/linux/rbtree.h'])
    members=[(work/os.fsdecode(row)).resolve() for row in module.with_suffix('.mod').read_bytes().splitlines()]
    if members!=[obj.resolve(),oracle.resolve()]: raise ValueError('unexpected private rbtree module constituents')
    if [(work/os.fsdecode(row)).resolve() for row in (work/'modules.order').read_bytes().splitlines()]!=[module.with_suffix('.o').resolve()]:
        raise ValueError('unexpected external rbtree module order')
    if module_name(module)!=name: raise ValueError('incorrect rbtree caller module identity')
    types=provider_type_ids(owner_path(build,provider(config)),names=EXPORTS)
    for path in (obj,module):
        elf_target(path,arch);verify_references(path,EXPORTS)
        verify_guarded_calls(path,arch,types,wrappers=WRAPPERS)
        if read_exports(path): raise ValueError('private rbtree caller exports symbols')
        if caller=='rust': verify_rust_entrypoints(path,arch)
    verify_module_import_versions(build,module)
    require_metadata_field(metadata_fields(module),b'license',b'GPL')
    require_metadata_field(metadata_fields(module),b'description',DESCRIPTION.encode())
    verify_module_metadata(build,module,work=work,require_c_suppression=True)
    verify_common_metadata(build,work,flags=compilation_flags,exports=read_exports)
    newer(module,[obj,oracle,selected_metadata(build,module)])
    for path,language in ((obj,'rust' if caller=='rust' else 'c'),(oracle,'c')):
        flags=compilation_flags(path);command=flags.copy()
        while command and '=' in command[0] and not command[0].startswith('-'): command.pop(0)
        real=native_flags(command[1:],work,language)
        verify_flag_policy(real,language)
        if language=='c' and ('-D__DISABLE_EXPORTS' not in real or '-U__DISABLE_EXPORTS' in real or read_exports(path)):
            raise ValueError('private original-C fixture retains incidental exports')
        if language=='rust': require_rustc(command[0],work,compiler_environment(work,dict(os.environ)))
    if caller=='rust': verify_rust_workload(obj,module)
    return module


ORIGINAL_LINES=(
    rb'rbtree testing',rb'-> test 1 \(latency of nnodes insert\+delete\): \d+ cycles',
    rb'-> test 2 \(latency of nnodes cached insert\+delete\): \d+ cycles',
    rb'-> test 3 \(latency of inorder traversal\): \d+ cycles',
    rb'-> test 4 \(latency to fetch first node\)',rb'non-cached: \d+ cycles',rb'cached: \d+ cycles',
    rb'augmented rbtree testing',rb'-> test 1 \(latency of nnodes insert\+delete\): \d+ cycles',
    rb'-> test 2 \(latency of nnodes cached insert\+delete\): \d+ cycles')


def verify_console(console,caller,*,reload=False,config=None):
    if re.search(rb'CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version',console,re.I):
        raise ValueError('rbtree kernel fault/test failure')
    normalized=normalize_console_transport(console)
    lines=[re.sub(rb'^\[\s*\d+\.\d+\]\s*',b'',line.strip()).strip() for line in normalized.splitlines()]
    allowed=set()
    if config and config.get('INT_LOG_KUNIT_TEST')=='y':
        allowed=set(verify_kunit_warnings(lines,kunit_runs(console,1),
            'Rust' if config.get('RUST_INT_LOG')=='y' else 'C',config.get('BUG')=='y'))
    if {i for i,line in enumerate(lines) if b'WARNING:' in line}!=allowed:
        raise ValueError('warning outside configured original integer-log zero cases')
    events=[]
    for line in lines:
        # KERN_CONT-style first heading/test may be joined or separated by printk.
        joined=re.fullmatch(rb'((?:augmented )?rbtree testing)\s+(-> test .*)',line)
        pieces=joined.groups() if joined else (line,)
        for piece in pieces:
            piece=re.sub(rb'^rbtree_(?:rust_)?abi: ',b'',piece)
            if (b'LUPOS_' in piece or b'rbtree testing' in piece or b'-> test ' in piece or
                    b'non-cached:' in piece or piece.startswith(b'cached:') or piece==b'test exit'):
                events.append(piece)
    expected=[]
    for attempt in range(2):
        expected.extend(ORIGINAL_LINES)
        expected.append(re.escape(f'LUPOS_RBTREE_ORIGINAL_EAGAIN_OK {attempt}'.encode()))
    expected+=[re.escape(marker(caller)),rb'LUPOS_RUST_MODULE_LOAD_OK']
    if reload: expected += [rb'LUPOS_RUST_MODULE_UNLOAD_OK 0',re.escape(marker(caller)),rb'LUPOS_RUST_MODULE_RELOAD_OK 0']
    expected.append(re.escape(MARKER))
    if len(events)!=len(expected) or any(re.fullmatch(pattern,event) is None for pattern,event in zip(expected,events)):
        raise ValueError('original rbtree suite/caller/result/lifecycle events differ')
    verify_module_events(console,module=True,preloads=0,reload=reload)
    return 2 if reload else 1


def clean_environment(environment):
    compiler_inputs={'CC','CXX','CPP','LD','AR','NM','STRIP','OBJCOPY','OBJDUMP','READELF',
        'HOSTCC','HOSTCXX','HOSTRUSTC','RUSTC','BINDGEN','LLVM','LLVM_IAS','ARCH','CROSS_COMPILE',
        'CLANG_FLAGS','CLANG_TRIPLE','CFLAGS','CXXFLAGS','CPPFLAGS','LDFLAGS','KCFLAGS','KAFLAGS',
        'KCPPFLAGS','KRUSTFLAGS','RUSTFLAGS','RUSTFLAGS_MODULE','AFLAGS_MODULE','CFLAGS_MODULE',
        'HOSTCFLAGS','HOSTCXXFLAGS','HOSTLDFLAGS','HOSTLDLIBS','HOSTRUSTFLAGS','BINDGEN_EXTRA_CLANG_ARGS',
        'RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','CCC_OVERRIDE_OPTIONS','CLANG_MODULE_CACHE_PATH',
        'COMPILER_PATH','GCC_EXEC_PREFIX','LIBRARY_PATH','C_INCLUDE_PATH','CPATH','CPLUS_INCLUDE_PATH',
        'OBJC_INCLUDE_PATH','LLVM_PROFILE_FILE'}
    return {name:value for name,value in environment.items() if not name.startswith(('CONFIG_','KCONFIG_','KBUILD_'))
        and name not in compiler_inputs and name not in ('MAKEFLAGS','MFLAGS','GNUMAKEFLAGS','CARGO_MAKEFLAGS','MAKELEVEL','MAKEOVERRIDES',
            'srctree','srcroot','objtree','VPATH','sub_make_done','DEPENDENCIES_OUTPUT','SUNPRO_DEPENDENCIES')}


def make_arguments(arguments,arch):
    """Only compiler selection/build verbosity; no output or policy overrides."""
    tools={'CC','RUSTC','BINDGEN','HOSTCC','HOSTCXX','HOSTRUSTC','LD','AR','NM','STRIP','OBJCOPY','OBJDUMP','READELF'}
    result=[];seen=set()
    for argument in arguments:
        match=re.fullmatch(r'([A-Z_]+)=([^\0\r\n$`]+)',argument)
        if not match or match[1] in seen: raise ValueError('invalid/duplicate make assignment')
        name,value=match.groups();seen.add(name)
        if name in tools:
            words=command_words(value,name)
            if len(words)!=1: raise ValueError('compiler selection must name one executable, not extra flags')
            value=words[0]
        elif name=='ARCH':
            if value!=('arm64' if arch=='aarch64' else 'x86'): raise ValueError('make ARCH differs from selected kernel')
        elif name=='LLVM':
            if value!='1' and not re.fullmatch(r'-\d+',value): raise ValueError('LLVM must select real LLVM tools')
        elif name=='LLVM_IAS':
            if value!='1': raise ValueError('integrated assembler required')
        elif name=='V':
            if value not in ('0','1'): raise ValueError('invalid make verbosity')
        else: raise ValueError('make argument may not override policy, roots, inputs or outputs: '+name)
        result.append(name+'='+value)
    return result


def validate_options(args,environment=None):
    env=os.environ if environment is None else environment
    build=explicit_path(args.build,'build');root=explicit_path(args.source,'source')
    verify_source(root);config=configuration(build);arch=architecture(config);selection=provider(config)
    if selection=='C' and not args.allow_c_baseline: raise ValueError('requires CONFIG_RUST_RBTREE=y')
    parent=outside(explicit_path(args.output_parent,'output parent'),(root,build,Path(__file__).resolve().parents[2]))
    if not parent.is_dir() or not os.access(parent,os.W_OK): raise ValueError('output parent must exist and be writable')
    if any(c.isspace() for c in str(parent)): raise ValueError('initramfs manifest output paths cannot contain whitespace')
    make=command_words(env.get('MAKE','make'),'MAKE')
    if any(not re.fullmatch(r'(?:-j[1-9]\d*|--jobs=[1-9]\d*|--no-print-directory)',part) for part in make[1:]):
        raise ValueError('MAKE argv may only select job count or suppress directory messages')
    for name in ('QEMU','HOSTRUSTC'):
        if name in env: command_words(env[name],name)
    qemu=command_words(args.qemu,'--qemu') if args.qemu is not None else command_words(env['QEMU'],'QEMU') if 'QEMU' in env else None
    data=explicit_path(args.qemu_data,'qemu-data') if args.qemu_data is not None else None
    if data is not None and not data.is_dir(): raise ValueError('qemu-data directory does not exist')
    make_args=make_arguments(args.make_arg,arch)
    selected_rust=None
    if config.get('RUST')=='y':
        saved=compilation_flags(owner_path(build,selection) if selection=='Rust' else build/'rust/kernel.o')
        while saved and '=' in saved[0] and not saved[0].startswith('-'): saved.pop(0)
        if not saved: raise ValueError('missing selected Rust compiler command')
        selected_rust=command_words(saved[0],'selected Rust compiler')[0]
    if not args.audit_only:
        defaults={}
        if selected_rust:
            defaults.update(RUSTC=selected_rust,HOSTRUSTC=selected_rust)
            binding=compilation_flags(build/'rust/bindings/bindings_generated.rs')
            defaults['BINDGEN']=command_words(binding[0],'selected bindgen')[0]
        present={item.split('=',1)[0] for item in make_args}
        make_args += [name+'='+value for name,value in defaults.items() if name not in present]
    if not math.isfinite(args.timeout) or args.timeout<=0: raise ValueError('timeout must be finite and positive')
    if not args.audit_only:
        for name in ('MULTIUSER','PRINTK','MODULES'):
            if config.get(name)!='y': raise ValueError('runtime requires CONFIG_'+name+'=y')
        if args.reload_modules and config.get('MODULE_UNLOAD')!='y': raise ValueError('reload requires MODULE_UNLOAD=y')
        if config.get('MODULE_SIG_FORCE')=='y': raise ValueError('unsigned caller requires disabled signature enforcement')
        if args.caller=='rust' and config.get('RUST')!='y': raise ValueError('Rust caller requires RUST=y')
        if not args.build_only and qemu is None: qemu=command_words('qemu-system-'+arch,'default QEMU')
        for relative in ('usr/gen_init_cpio','scripts/gendwarfksyms/gendwarfksyms'):
            if not (build/relative).is_file(): raise ValueError('missing completed native artifact '+relative)
    host=command_words(env.get('HOSTRUSTC',selected_rust or 'rustc'),'HOSTRUSTC')
    if len(host)!=1: raise ValueError('HOSTRUSTC must name one compiler without output flags')
    return dict(build=build,root=root,config=config,arch=arch,selection=selection,parent=parent,
                make=make,make_args=make_args,qemu=qemu,data=data,host=host)


def private_copy(build,destination,root):
    # Never share writable output inode/symlink targets with the native donor.
    shutil.copytree(build,destination,symlinks=False,
        ignore=lambda directory,names:['source'] if Path(directory)==build and 'source' in names else [])
    (destination/'source').symlink_to(root,target_is_directory=True)


def prepare_boot(build,work,module,arch,host,root,*,reload=False):
    work.mkdir()
    source=work/'boot_init.rs';source.write_text(init_source(root));init=work/'init'
    command=init_command(host,arch,init)
    command=[str(source) if word.endswith('/scripts/tests/boot_init.rs') else word for word in command]
    command+=['--out-dir='+str(work)]
    require_rustc(host[0],work,compiler_environment(work,clean_environment(os.environ)))
    validate_compiler_outputs(command,work,work,(build,root))
    run_logged(command,work/'init-compile.log',env=clean_environment(os.environ))
    fixture=work/'fixture';fixture.write_bytes(b'Rust-generated initramfs fixture\n\0with binary data\xff')
    original=build/'lib/rbtree_test.ko'
    for path in (module,original):
        if any(c.isspace() for c in str(path)): raise ValueError('initramfs module path contains whitespace')
    manifest=work/'manifest'
    entries=('dir /dev 0755 0 0\nnod /dev/console 0600 0 0 c 5 1\nnod /dev/kmsg 0600 0 0 c 1 11\n'+
        f'file /init {init} 0755 0 0\nfile /fixture {fixture} 0640 123 456 /hardlink\n'+
        'slink /symlink /fixture 0777 0 0\n'+f'file /test-module.ko {module} 0600 0 0\n'+
        f'file /original-rbtree-test.ko {original} 0600 0 0\n')
    if reload:
        plan=work/'reload-plan';plan.write_text('/test-module.ko\t'+module_name(module)+'\n')
        entries+=f'file /reload-plan {plan} 0600 0 0\n'
    manifest.write_text(entries);archive=work/'initramfs.cpio'
    run_logged([build/'usr/gen_init_cpio','-t','0','-c','-o',archive,manifest],work/'initramfs.log')
    return archive


def execute_guest(command,work,timeout):
    """Root-owned invocation only; all guest process state/logs remain private."""
    environment=compiler_environment(work,clean_environment(os.environ))
    (work/'qemu-command.json').write_text(json.dumps(dict(argv=command,cwd=str(work),
        temporary=environment['TMPDIR']),indent=2)+'\n')
    deadline=time.monotonic()+timeout;tail=b'';found=False;status={}
    with (work/'console.log').open('wb') as log,selectors.DefaultSelector() as selector:
        process=subprocess.Popen(command,cwd=work,env=environment,stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,stdin=subprocess.DEVNULL)
        try:
            selector.register(process.stdout,selectors.EVENT_READ)
            while time.monotonic()<deadline:
                ready=selector.select(timeout=min(1,max(0,deadline-time.monotonic())))
                if not ready:
                    if process.poll() is not None: break
                    continue
                data=process.stdout.read1(65536)
                if not data: break
                log.write(data);log.flush();tail=(tail+data)[-65536:]
                if MARKER+b'\n' in tail or MARKER+b'\r\n' in tail:
                    found=True;break
        finally:
            if process.poll() is None: process.terminate()
            try: process.wait(timeout=10)
            except subprocess.TimeoutExpired: process.kill();process.wait()
            process.stdout.close()
            status=dict(returncode=process.returncode,marker_seen=found,timeout=timeout)
            (work/'qemu-status.json').write_text(json.dumps(status,indent=2)+'\n')
    if not found: raise ValueError('guest did not finish; retained '+str(work/'console.log'))
    return (work/'console.log').read_bytes()


def main(argv=None):
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build')
    parser.add_argument('--source',default=str(ROOT))
    parser.add_argument('--audit-only',action='store_true')
    parser.add_argument('--allow-c-baseline',action='store_true')
    parser.add_argument('--caller',choices=('c','rust'),default='c')
    parser.add_argument('--reload-modules',action='store_true')
    parser.add_argument('--build-only',action='store_true',help='private callers/PID1 only, no guest or runtime claim')
    parser.add_argument('--output-parent',default=os.environ.get('RBTREE_RUNTIME_LOG_DIR',tempfile.gettempdir()))
    parser.add_argument('--make-arg',action='append',default=[])
    parser.add_argument('--qemu')
    parser.add_argument('--qemu-data')
    parser.add_argument('--timeout',type=float,default=300)
    args=parser.parse_args(argv)
    try:
        values=validate_options(args)
        build,root=values['build'],values['root']
        work=Path(tempfile.mkdtemp(prefix='rbtree-runtime-',dir=values['parent']))
        print('Retained rbtree runtime evidence: '+str(work),flush=True)
        audit=work/'audit';audit.mkdir()
        result=verify_selected(build,source=root,work=audit,allow_c=args.allow_c_baseline)
        if args.audit_only:
            print(json.dumps(result,indent=2));return
        with observed_build(build,work):
            private=work/'build';private_copy(build,private,root)
            consumer=work/'consumer';consumer.mkdir();name,stem=caller_names(args.caller)
            (consumer/(stem+('.rs' if args.caller=='rust' else '.c'))).write_text(caller_source(root,args.caller))
            (consumer/'rbtree_reference.c').write_text(reference_source(root))
            (consumer/'Makefile').write_text(f'obj-m := {name}.o\n{name}-y := {stem}.o rbtree_reference.o\n')
            environment=clean_environment(os.environ)
            run_logged([*values['make'],'-C',root,'O='+str(private),'M='+str(consumer),*values['make_args'],
                'CFLAGS_MODULE+=-D__DISABLE_EXPORTS','RUSTFLAGS_MODULE+=-Dwarnings -Dunsafe_op_in_unsafe_fn','modules'],
                work/'external-module.log',env=environment)
            module=verify_consumer(private,consumer,args.caller,source=root)
            boot=work/'boot'
            archive=prepare_boot(private,boot,module,values['arch'],values['host'],root,reload=args.reload_modules)
            result.update(caller=args.caller,consumer=str(module),boot=str(boot),runtime=False)
            if not args.build_only:
                kernel=build/boot_architecture(values['arch'])[0]
                command=qemu_command(shlex.join(values['qemu']),values['arch'],kernel,archive,values['data'])
                console=execute_guest(command,boot,args.timeout)
                runs=verify_console(console,args.caller,reload=args.reload_modules,config=values['config'])
                result.update(runtime=True,loads=runs,steps_per_load=STEPS,original_attempts=2)
        (work/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    except (OSError,ValueError,KeyError,IndexError,struct.error,subprocess.SubprocessError) as error:
        parser.error(str(error))
    print(json.dumps(result,indent=2))


if __name__=='__main__': main()
