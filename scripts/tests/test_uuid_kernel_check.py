#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""UUID runtime protocol and actual-flag private caller proofs; never boot."""
from contextlib import redirect_stderr
import io
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import check_uuid_kernel as check
import test_uuid as native_fixture

ROOT=Path(os.environ.get("UUID_RUNTIME_SOURCE_ROOT",Path(__file__).resolve().parents[2])).resolve()
check.ROOT=ROOT


def suite_log():
    return (b"KTAP version 1\n# Subtest: uuid\n# module: uuid_kunit\n1..8\n"+
            b"".join(b"ok "+str(i+1).encode()+b" "+case+b"\n" for i,case in enumerate(check.CASES))+
            b"ok 9 uuid\n")


def console(caller="c",suite="y",framework=False,reload=False):
    data=suite_log() if suite=="y" else b""
    count=int(framework)+int(suite=="m")
    for i in range(count):
        if i==count-1: data+=suite_log()
        data+=f"LUPOS_RUST_PRELOAD_OK {i}\n".encode()
    data+=check.result_marker(caller)+b"\nLUPOS_RUST_MODULE_LOAD_OK\n"
    if reload:
        for i in reversed(range(count+1)): data+=f"LUPOS_RUST_MODULE_UNLOAD_OK {i}\n".encode()
        for i in range(count):
            if i==count-1: data+=suite_log()
            data+=f"LUPOS_RUST_MODULE_RELOAD_OK {i}\n".encode()
        data+=check.result_marker(caller)+f"\nLUPOS_RUST_MODULE_RELOAD_OK {count}\n".encode()
    return data+b"LUPOS_RUST_BUILD_BOOT_OK\n"


class ProtocolTests(unittest.TestCase):
    def test_all_callers_frameworks_and_reload(self):
        for caller in ("c","rust"):
            for suite,framework in (("y",False),("m",False),("m",True)):
                for reload in (False,True):
                    self.assertEqual(check.verify_console(console(caller,suite,framework,reload),caller,
                        suite=suite,framework_module=framework,reload=reload),2 if suite=="m" and reload else 1)

    def test_exact_original_cases(self):
        source=(ROOT/"lib/tests/uuid_kunit.c").read_text()
        cases=re.findall(r"KUNIT_CASE\((\w+)\)",source)
        self.assertEqual([x.decode() for x in check.CASES],cases)
        data=console()
        for case in check.CASES:
            line=next(row for row in data.splitlines() if row.endswith(case))
            for variant in (data.replace(line+b"\n",b""),data.replace(line,line+b"\n"+line),
                            data.replace(line,line+b" # SKIP"),data.replace(line,b"not "+line),
                            data.replace(line,line.replace(b"uuid_test_",b"wrong_test_"))):
                with self.assertRaises(ValueError): check.verify_console(variant,"c")
        for old,new in ((b"1..8",b"1..7"),(b"1..8",b"1..8 # TODO"),(b"ok 9 uuid",b"ok 9 uuid # SKIP"),
                        (b"# module: uuid_kunit",b"# module: other")):
            with self.assertRaises(ValueError): check.verify_console(data.replace(old,new),"c")
        with self.assertRaises(ValueError): check.verify_console(data+suite_log(),"c")
        with self.assertRaises(ValueError): check.verify_console(data+b"ok 1 uuid_test_guid_valid\n","c")

    def test_summaries_are_exact(self):
        data=console().replace(b"ok 9 uuid",b"# uuid: pass:8 fail:0 skip:0 total:8\n# Totals: pass:8 fail:0 skip:0 total:8\nok 9 uuid")
        self.assertEqual(check.verify_console(data,"c"),1)
        for old,new in ((b"pass:8",b"pass:7"),(b"fail:0",b"fail:1"),(b"skip:0",b"skip:1"),(b"total:8",b"total:7")):
            with self.assertRaises(ValueError): check.verify_console(data.replace(old,new,1),"c")

    def test_missing_duplicate_reordered_events(self):
        data=console("rust","m",True,True)
        for line in (x for x in data.splitlines() if b"LUPOS_" in x):
            for variant in (data.replace(line+b"\n",b"",1),data.replace(line,line+b"\n"+line,1)):
                with self.assertRaises(ValueError): check.verify_console(variant,"rust",suite="m",framework_module=True,reload=True)
        variant=data.replace(b"ok 9 uuid\nLUPOS_RUST_PRELOAD_OK 1",b"LUPOS_RUST_PRELOAD_OK 1\nok 9 uuid")
        with self.assertRaises(ValueError): check.verify_console(variant,"rust",suite="m",framework_module=True,reload=True)

    def test_timestamp_and_exact_replay(self):
        data=console("rust")
        data=data.replace(check.result_marker("rust"),b"uuid_rust_abi: "+check.result_marker("rust"))
        data=b"".join(b"[    1.234567] "+row+b"\n" for row in data.splitlines())
        self.assertEqual(check.verify_console(data,"rust"),1)
        marker=check.result_marker("c")
        data=console().replace(marker,marker[:20]+b"\n** replaying previous printk message **\n"+marker)
        self.assertEqual(check.verify_console(data,"c"),1)
        with self.assertRaises(ValueError): check.verify_console(data.replace(marker[:20],b"wrong",1),"c")

    def test_faults_and_false_success(self):
        for fault in (b"BUG:",b"WARNING:",b"Oops:",b"CFI failure",b"Kernel panic",b"KASAN:",b"UBSAN:",
                      b"not ok 3 other",b"no symbol version",b"no extended symbol version",b"LUPOS_UUID_FAILURE stage=1 record=0"):
            with self.assertRaises(ValueError): check.verify_console(console()+fault+b"\n","c")
        with self.assertRaises(ValueError): check.verify_console(console().replace(b"parser=9217",b"parser=9216"),"c")
        with self.assertRaises(ValueError): check.verify_console(console(),"rust")
        with self.assertRaises(ValueError): check.verify_console(console()+b"not ok\n** replaying previous printk message **\nnot okay\n","c")

    def test_states_require_original_suite(self):
        for framework,suite in (("y","y"),("y","m"),("m","m")):
            self.assertEqual(check.states(dict(KUNIT=framework,UUID_KUNIT_TEST=suite)),(suite,framework))
        for cfg in ({},{"KUNIT":"m","UUID_KUNIT_TEST":"y"},{"KUNIT":"y","UUID_KUNIT_TEST":"n"},
                    {"KUNIT":"y","UUID_KUNIT_TEST":"y","RUST_UUID":"y"}):
            with self.assertRaises(ValueError): check.states(cfg)

    def test_independent_rust_suite_selector(self):
        for provider in ("n","y"):
            for language in ("n","y"):
                for framework,suite in (("y","y"),("y","m"),("m","m")):
                    cfg=dict(RUST="y",RUST_UUID=provider,RUST_UUID_KUNIT_TEST=language,
                             KUNIT=framework,UUID_KUNIT_TEST=suite)
                    self.assertEqual(check.states(cfg),(suite,framework))
        for language in ("m","invalid","y"):
            with self.assertRaises(ValueError):
                check.states(dict(RUST="n",KUNIT="y",UUID_KUNIT_TEST="y",RUST_UUID_KUNIT_TEST=language))

    def test_cli_rejects_before_writes(self):
        with tempfile.TemporaryDirectory() as directory:
            build=Path(directory)
            for cfg in ({},dict(KUNIT="y",UUID_KUNIT_TEST="y"),dict(KUNIT="y",UUID_KUNIT_TEST="y",
                        RUST="y",RUST_UUID="y",MODULES="y",PRINTK="y",MULTIUSER="y",MODULE_SIG_FORCE="y")):
                with mock.patch.object(sys,"argv",["check",str(build)]),mock.patch.object(check,"configuration",return_value=cfg),redirect_stderr(io.StringIO()):
                    with self.assertRaises(SystemExit) as error: check.main()
                    self.assertEqual(error.exception.code,2)
                self.assertEqual(list(build.iterdir()),[])


class MetadataTests(unittest.TestCase):
    def test_every_export_record_field_is_enforced(self):
        for arch,reloc in (("x86_64",1),("aarch64",257)):
            valid=[dict(name=name,license="GPL" if name in check.GPL else "",namespace="",relocation_target=name,
                relocation_addend=0,pointer_width=8,relocation_kind=reloc,label_binding=0,label_kind=0,
                section_flags=2,section_alignment=8) for name in check.PUBLIC]
            with mock.patch.object(check,"read_exports",return_value=valid): check.verify_exports(Path("owner.o"),arch)
            variants=[valid[:-1],valid+[valid[0]]]
            for field,value in (("namespace","WRONG"),("license","GPL"),("relocation_target","wrong"),
                                ("relocation_addend",1),("pointer_width",4),("relocation_kind",999),
                                ("label_binding",1),("label_kind",1),("section_flags",3),("section_alignment",4)):
                variants.append([{**valid[0],field:value},*valid[1:]])
            for variant in variants:
                with mock.patch.object(check,"read_exports",return_value=variant),self.assertRaises(ValueError):
                    check.verify_exports(Path("owner.o"),arch)

    def test_dwarf_is_recomputed_not_just_saved_crc(self):
        with tempfile.TemporaryDirectory() as directory:
            build=Path(directory)
            versions={n.encode():b"0x12345678" for n in check.PUBLIC}
            output=b"".join(b"#SYMVER "+n+b" "+crc+b"\n" for n,crc in versions.items())
            def run(args,**kwargs):
                Path(args[2]).write_bytes(b"s#guid_t s#uuid_t")
                return subprocess.CompletedProcess(args,0,output,b"")
            with mock.patch.object(check,"configuration",return_value={"MODVERSIONS":"y","GENDWARFKSYMS":"y"}),mock.patch.object(check.subprocess,"run",side_effect=run):
                check.verify_dwarf(build,build/"owner.o",versions)
                versions[b"guid_null"]=b"0x00000000"
                with self.assertRaises(ValueError): check.verify_dwarf(build,build/"owner.o",versions)

    def test_exact_mixed_exports_and_no_private_indexes(self):
        with tempfile.TemporaryDirectory() as temporary:
            build=Path(temporary)
            valid=b"".join(b"0x12345678\t"+name.encode()+b"\tvmlinux\t"+
                (b"EXPORT_SYMBOL_GPL" if name in check.GPL else b"EXPORT_SYMBOL")+b"\t\n" for name in check.PUBLIC)
            (build/"Module.symvers").write_bytes(valid)
            self.assertEqual(len(check.selected_versions(build)),9)
            for bad in (b"",valid+valid.splitlines(keepends=True)[0],valid.replace(b"\tvmlinux\t",b"\tother\t",1),
                        valid.replace(b"EXPORT_SYMBOL_GPL",b"EXPORT_SYMBOL",1),
                        valid+b"0x12345678\tguid_index\tvmlinux\tEXPORT_SYMBOL\t\n",
                        valid.replace(b"\t\n",b"\tNS\n",1)):
                (build/"Module.symvers").write_bytes(bad)
                with self.subTest(bad=bad),self.assertRaises(ValueError): check.selected_versions(build)

    def test_original_kunit_metadata_exact(self):
        for builtin in (False,True):
            p=b"uuid_kunit." if builtin else b""
            fields=[p+b"license=Dual BSD/GPL",p+b"author=Andy Shevchenko <andriy.shevchenko@linux.intel.com>",
                    p+b"description=Test cases for lib/uuid.c module"]
            with mock.patch.object(check,"metadata_fields",return_value=fields): check.verify_suite_metadata(Path("test.o"),builtin)
            for bad in (fields*2,[fields[0]+b"x",*fields[1:]],fields[:-1]):
                with mock.patch.object(check,"metadata_fields",return_value=bad),self.assertRaises(ValueError):
                    check.verify_suite_metadata(Path("test.o"),builtin)

    def test_license_negatives_require_actual_modpost_diagnostic(self):
        check.verify_license_result(subprocess.CompletedProcess([],0,b"",b""),None)
        for name in check.GPL:
            msg=f'ERROR: modpost: uuid_license.ko: GPL-incompatible module uses GPL-only symbol \'{name}\'\n'.encode()
            check.verify_license_result(subprocess.CompletedProcess([],2,b"",msg),name)
            for rc,data in ((0,msg),(1,b"error: compile failed"),(2,msg+b"error: other")):
                with self.assertRaises(ValueError): check.verify_license_result(subprocess.CompletedProcess([],rc,b"",data),name)
        self.assertIn('MODULE_LICENSE("Proprietary")',check.license_source())
        for name in check.GPL: self.assertIn(name+"(&value)",check.license_source(name))


class SuiteCommandTests(unittest.TestCase):
    """Real saved-command/freshness checks with scoped ELF transport mocks.

    Actual header-derived objects and registration/KCFI are exercised separately
    by NativeCallerTests; these files do not purport to be ABI objects.
    """
    def setUp(self):
        temporary=tempfile.TemporaryDirectory(prefix="uuid-suite-command-")
        self.addCleanup(temporary.cleanup)
        self.build=Path(temporary.name)
        self.obj=self.build/"lib/tests/uuid_kunit.o"
        self.kernel=self.build/"rust/kernel.o"
        self.metadata=[self.build/"rust/libkernel.rmeta",self.build/"rust/libbindings.rmeta"]
        for path in (self.obj,self.kernel,*self.metadata):
            path.parent.mkdir(parents=True,exist_ok=True)
            path.write_bytes(b"transport")
        (self.build/".config").write_text("CONFIG_CFI=y\n")
        for name in ("elf_target","verify_references","verify_suite_registration","verify_suite_metadata"):
            patch=mock.patch.object(check,name)
            patch.start();self.addCleanup(patch.stop)
        self.command(self.kernel,ROOT/"rust/kernel/lib.rs",[ROOT/"rust/kernel/kunit.rs"],True)

    def command(self,obj,source,deps,rust):
        path=obj.with_name("."+obj.name+".cmd")
        key=str(obj.relative_to(self.build))
        flags=["rustc","-Zsanitizer=kcfi"] if rust else ["clang","-fsanitize=kcfi"]
        path.write_text(f"savedcmd_{key} := "+shlex.join([*flags,str(source)])+"\n"+
            f"source_{key} := {source}\n"+f"deps_{key} := "+shlex.join(map(str,deps))+"\n")
        os.utime(obj,ns=(source.stat().st_mtime_ns+10_000_000_000,
                        max(p.stat().st_mtime_ns for p in (source,*deps))+10_000_000_000))
        return path

    def prepare(self,rust):
        source=ROOT/("lib/tests/uuid_kunit.rs" if rust else "lib/tests/uuid_kunit.c")
        deps=[ROOT/"include/linux/uuid_header.rs",*self.metadata] if rust else [ROOT/"include/kunit/test.h",ROOT/"include/linux/uuid.h"]
        return self.command(self.obj,source,deps,rust)

    def verify(self,rust=True,members=None):
        check.verify_suite_object(self.build,self.obj,"x86_64",builtin=True,rust_suite=rust,
            members={self.kernel.resolve()} if members is None else members)

    def test_selected_original_source_and_wrong_language(self):
        for rust in (False,True):
            self.prepare(rust)
            self.verify(rust)
            with self.assertRaises(ValueError): self.verify(not rust)

    def test_missing_dependency_and_stale_source(self):
        for rust in (False,True):
            path=self.prepare(rust)
            original=path.read_text()
            missing="include/linux/uuid_header.rs" if rust else "include/kunit/test.h"
            path.write_text(original.replace(str(ROOT/missing),""))
            with self.assertRaises(ValueError): self.verify(rust)
            path.write_text(original)
            os.utime(self.obj,ns=(1,1))
            with self.assertRaises(ValueError): self.verify(rust)

    def test_real_rust_helper_membership_dependencies_and_rmeta_freshness(self):
        self.prepare(True)
        self.verify()
        with self.assertRaises(ValueError): self.verify(members=set())
        path=self.kernel.with_name("."+self.kernel.name+".cmd")
        text=path.read_text()
        path.write_text(text.replace(str(ROOT/"rust/kernel/kunit.rs"),""))
        with self.assertRaises(ValueError): self.verify()
        path.write_text(text)
        for metadata in self.metadata:
            stamp=metadata.stat().st_mtime_ns
            os.utime(metadata,ns=(1,1))
            with self.assertRaises(ValueError): self.verify()
            os.utime(metadata,ns=(stamp,stamp))

    def test_selected_compile_flags_must_retain_kcfi(self):
        for rust in (False,True):
            path=self.prepare(rust)
            original=path.read_text()
            flag="-Zsanitizer=kcfi" if rust else "-fsanitize=kcfi"
            for replacement in ("",flag+" -fno-sanitize=all",flag+" -fno-sanitize=kcfi"):
                path.write_text(original.replace(flag,replacement))
                with self.assertRaises(ValueError): self.verify(rust)


class NativeCallerTests(unittest.TestCase):
    def setUp(self):
        self.fixture=native_fixture.UUIDTest("test_native_x86")
        self.fixture.setUp()
        self.addCleanup(self.fixture.doCleanups)

    def compile(self,arch,execute=False):
        fixture=self.fixture
        fixture.require(arch)
        n=fixture.n
        build=fixture.builds[arch]
        n.BUILDS={arch:build}
        rf,cf=n.flags(build,True),n.flags(build,False)
        n.env.update(OBJTREE=str(build),RUST_MODFILE="uuid_runtime")
        provider=fixture.out/"provider-c.o"
        n.command(["clang",*cf,"-O2","-D__DISABLE_EXPORTS","-c",ROOT/"lib/uuid.c","-o",provider],arch+"-provider-c",build)
        check.verify_statics(provider)
        check.verify_rng_calls(provider,"aarch64" if arch=="arm64" else "x86_64")
        n.command([n.RUST,*rf,"-Copt-level=2",ROOT/"lib/uuid_rust.rs","--emit=obj="+str(fixture.out/"provider-rust.o")],arch+"-provider-rust")
        check.verify_statics(fixture.out/"provider-rust.o")
        check.verify_rng_calls(fixture.out/"provider-rust.o","aarch64" if arch=="arm64" else "x86_64")
        types=check.provider_type_ids(provider,names=check.FUNCTIONS)
        for caller in ("c","rust"):
            source=fixture.out/("uuid_"+caller+(".rs" if caller=="rust" else ".c"))
            source.write_text(check.sources(caller))
            obj=source.with_suffix(".o")
            if caller=="rust":
                n.command([n.RUST,*rf,"-Copt-level=2","--cfg","MODULE",source,"--emit=obj="+str(obj)],arch+"-"+caller)
                check.verify_rust_workload(obj)
            else:
                n.command(["clang",*cf,"-O2","-DMODULE","-D__DISABLE_EXPORTS","-c",source,"-o",obj],arch+"-"+caller,build)
            n.command(["llvm-objdump","-dr","--no-show-raw-insn",obj],arch+"-"+caller+"-disassembly")
            check.verify_guarded_calls(obj,"aarch64" if arch=="arm64" else "x86_64",types,wrappers=check.WRAPPERS)
            linked=source.with_suffix(".linked.o")
            n.command(["ld.lld","-r",obj,"-o",linked],arch+"-"+caller+"-link")
            check.verify_guarded_calls(linked,"aarch64" if arch=="arm64" else "x86_64",types,wrappers=check.WRAPPERS)
            assembly=n.command(["llvm-objdump","-dr","--no-show-raw-insn",obj],arch+"-"+caller+"-negative-disassembly")
            old="ud2" if arch=="x86" else "brk"
            self.assertIn(old,assembly)
            altered=assembly.replace(old,"nop")
            with mock.patch.object(check.subprocess,"run",return_value=subprocess.CompletedProcess([],0,altered.encode(),b"")):
                with self.assertRaises(ValueError):
                    check.verify_guarded_calls(obj,"aarch64" if arch=="arm64" else "x86_64",types,wrappers=check.WRAPPERS)
        for stem,text in (("reference",check.reference_source()),("workload",check.WORKLOAD),
                          ("license-public",check.license_source()),
                          ("license-guid",check.license_source("guid_gen")),("license-uuid",check.license_source("uuid_gen"))):
            source=fixture.out/(stem+".c");source.write_text(text)
            n.command(["clang",*cf,"-O2","-DMODULE","-D__DISABLE_EXPORTS","-c",source,"-o",source.with_suffix(".o")],arch+"-"+stem,build)
            if stem.startswith("license-"):
                directory=fixture.out/stem
                directory.mkdir()
                obj=directory/"uuid_license.o"
                obj.write_bytes(source.with_suffix(".o").read_bytes())
                obj.with_suffix(".mod").write_text(str(obj)+"\n")
                (directory/".uuid_license.o.cmd").write_text("")
                result=subprocess.run([str(build/"scripts/mod/modpost"),"-e","-M","-m","-x",
                    "-i",str(build/"Module.symvers"),"-o",str(directory/"Module.symvers"),str(obj)],
                    env=n.env,capture_output=True,timeout=60)
                (fixture.out/(arch+"-"+stem+"-modpost.log")).write_bytes(result.stdout+result.stderr)
                forbidden={"license-public":None,"license-guid":"guid_gen","license-uuid":"uuid_gen"}[stem]
                check.verify_license_result(result,forbidden)
        if execute: self.execute(build,rf,cf)

    def test_actual_x86_protected_callers(self): self.compile("x86")
    def test_actual_arm64_protected_callers(self): self.compile("arm64")

    def test_actual_c_rust_suite_registration_lifecycles_and_kcfi(self):
        fixture=self.fixture
        n=fixture.n
        for arch in ("x86","arm64"):
            fixture.require(arch)
            build=fixture.builds[arch]
            n.BUILDS={arch:build}
            rf,cf=n.flags(build,True),n.flags(build,False)
            # The ARM donor may itself be modular. Exercise both UUID suite
            # lifecycles explicitly, retaining every other actual target flag.
            rf=[arg for i,arg in enumerate(rf) if arg!="--cfg=MODULE" and
                not (arg=="--cfg" and i+1<len(rf) and rf[i+1]=="MODULE") and
                not (arg=="MODULE" and i>0 and rf[i-1]=="--cfg")]
            cf=[arg.replace('"lib/uuid"','"lib/tests/uuid_kunit"').replace('"uuid"','"uuid_kunit"')
                .replace('=uuid','=uuid_kunit') for arg in cf]
            makefile=(ROOT/"lib/tests/Makefile").read_text()
            self.assertNotRegex(makefile,r'(?m)^\s*CFLAGS_(?:REMOVE_)?uuid_kunit\.o\s*[:+?]?=')
            n.env.update(OBJTREE=str(build),RUST_MODFILE="lib/tests/uuid_kunit")
            rust_source=fixture.out/"suite.rs"
            text=(ROOT/"lib/tests/uuid_kunit.rs").read_text()
            self.assertEqual(text.count('../../include/linux/uuid_header.rs'),1)
            rust_source.write_text(text.replace('../../include/linux/uuid_header.rs',str(ROOT/"include/linux/uuid_header.rs")))
            for builtin in (False,True):
                ids=[]
                for language in ("c","rust"):
                    tag=f"suite-{arch}-{builtin}-{language}"
                    obj=fixture.out/(tag+".o")
                    if language=="c":
                        n.command(["clang",*cf,"-O2",*(["-DMODULE"] if not builtin else []),
                            "-c",ROOT/"lib/tests/uuid_kunit.c","-o",obj],tag,build)
                    else:
                        n.command([n.RUST,*rf,"-Copt-level=2",*(["--cfg=MODULE"] if not builtin else []),
                            rust_source,"--emit=obj="+str(obj)],tag)
                    n.command(["nm",obj],tag+"-symbols")
                    callbacks=check.verify_suite_registration(obj,"x86_64" if arch=="x86" else "aarch64",
                        rust_suite=language=="rust",builtin=builtin,cfi=True)
                    check.verify_references(obj,("guid_parse","uuid_parse","guid_gen","uuid_gen",
                        "generate_random_uuid","generate_random_guid","__kunit_do_failed_assertion",
                        "kunit_binary_assert_format","kunit_unary_assert_format"))
                    check.verify_suite_metadata(obj,builtin)
                    values=check.provider_type_ids(obj,names=tuple(s[0].decode() for s in callbacks))
                    ids.append(list(values.values()))
                    self.registration_negatives(obj,arch,language=="rust",builtin)
                    linked=fixture.out/(tag+".linked.o")
                    n.command(["ld.lld","-r",obj,"-o",linked],tag+"-link")
                    check.verify_suite_registration(linked,"x86_64" if arch=="x86" else "aarch64",
                        rust_suite=language=="rust",builtin=builtin,cfi=True)
                self.assertEqual(ids[0],ids[1],"original C and actual-binding Rust callback KCFI")

    def registration_negatives(self,obj,arch,rust_suite,builtin):
        arch="x86_64" if arch=="x86" else "aarch64"
        parsed=check.module_elf(obj)
        sections=parsed[1]
        registration=next(i for i,s in enumerate(sections) if s[0]==b".kunit_test_suites")
        relocation=next(i for i,s in enumerate(sections) if s[1]==4 and s[6]==registration)
        def reject(changed):
            with mock.patch.object(check,"module_elf",return_value=(parsed[0],changed,*parsed[2:])),self.assertRaises(ValueError):
                check.verify_suite_registration(obj,arch,rust_suite=rust_suite,builtin=builtin,cfi=True)
        for field,value in ((1,8),(2,2),(4,16),(7,4)):
            changed=list(sections);section=list(changed[registration]);section[field]=value;changed[registration]=tuple(section)
            reject(changed)
        for payload in ((),sections[relocation][-1]*2):
            changed=list(sections);section=list(changed[relocation]);section[-1]=payload;changed[relocation]=tuple(section)
            reject(changed)
        for offset,kind in ((4,1),(0,999)):
            changed=list(sections);section=list(changed[relocation]);r=list(section[-1][0]);r[0]=offset;r[1]=kind
            section[-1]=(tuple(r),);changed[relocation]=tuple(section);reject(changed)
        changed=list(sections);section=list(changed[relocation]);r=list(section[-1][0]);symbol=list(r[2])
        symbol[3]=0;r[2]=tuple(symbol);section[-1]=(tuple(r),);changed[relocation]=tuple(section)
        reject(changed)
        callbacks=check.verify_suite_registration(obj,arch,rust_suite=rust_suite,builtin=builtin,cfi=True)
        callback_addresses={(s[3],s[4]) for s in callbacks}
        for i,s in enumerate(sections):
            if s[1]!=4 or not 0<s[6]<len(sections) or not sections[s[6]][2]&2 or sections[s[6]][2]&4: continue
            matches=[j for j,r in enumerate(s[-1]) if (r[2][3],r[2][4]+r[3]) in callback_addresses]
            if len(matches)!=8: continue
            for mode in ("missing","reordered","duplicate"):
                records=list(s[-1]);a,b=matches[:2]
                if mode=="missing": records.pop(a)
                elif mode=="duplicate": records.append(records[a])
                else:
                    first,second=records[a],records[b]
                    records[a]=(first[0],*second[1:]);records[b]=(second[0],*first[1:])
                changed=list(sections);changed[i]=(*s[:-1],tuple(records));reject(changed)
            break
        else: self.fail("actual callback registration relocation table missing")
        for name in (b"init_module",b"cleanup_module"):
            symbols=[*parsed[2],(name,*callbacks[0][1:])]
            with mock.patch.object(check,"module_elf",return_value=(parsed[0],sections,symbols,*parsed[3:])),self.assertRaises(ValueError):
                check.verify_suite_registration(obj,arch,rust_suite=rust_suite,builtin=builtin,cfi=True)
        # The same real object must fail the opposite language/module marker gate.
        if not builtin:
            with self.assertRaises(ValueError):
                check.verify_suite_registration(obj,arch,rust_suite=not rust_suite,builtin=builtin,cfi=True)
        with mock.patch.object(check,"read_exports",return_value=[{"name":"unexpected"}]),self.assertRaises(ValueError):
            check.verify_suite_registration(obj,arch,rust_suite=rust_suite,builtin=builtin,cfi=True)
        with mock.patch.object(check,"provider_type_ids",return_value={str(i):i+1 for i in range(8)}),self.assertRaises(ValueError):
            check.verify_suite_registration(obj,arch,rust_suite=rust_suite,builtin=builtin,cfi=True)

    def execute(self,build,rf,cf):
        fixture=self.fixture
        n=fixture.n
        transport=fixture.out/"transport.c"
        transport.write_text("""
#include <linux/printk.h>
#include <linux/random.h>
extern void exit(int);
int uuid_exercise(void);
int main(void);
static unsigned state, calls;
int _printk(const char *fmt, ...) { return 0; }
void get_random_bytes(void *p, size_t size)
{
    unsigned char *b=p;
    unsigned i;
    if (size!=16) exit(98);
    calls++;
    for(i=0;i<size;i++) { state=state*1664525U+1013904223U; b[i]=state>>24; }
}
int main(void) { int n=uuid_exercise(); return n==9217 && calls==1024 ? 0 : 87; }
""")
        original_transport=transport.read_text()
        for caller in ("c","rust"):
            source=fixture.out/("transport-"+caller+".c")
            source.write_text(original_transport if caller=="c" else original_transport.replace("uuid_exercise","uuid_rust_exercise"))
            n.command(["clang",*cf,"-O2","-c",source,"-o",source.with_suffix(".o")],"transport-"+caller,build)
        for stem,source in (("ctype",ROOT/"lib/ctype.c"),("hex",n.FIXTURES/"hex.c")):
            n.command(["clang",*cf,"-O2","-D__DISABLE_EXPORTS","-c",source,"-o",fixture.out/(stem+".o")],stem,build)
        n.command([n.RUST,*rf,"-Copt-level=2",n.FIXTURES/"panic.rs","--emit=obj="+str(fixture.out/"panic.o")],"panic")
        shared=[fixture.out/(stem+".o") for stem in ("reference","ctype","hex","panic")]
        for provider in ("c","rust"):
            for caller in ("c","rust"):
                inputs=[*shared,fixture.out/("transport-"+caller+".o")]
                if caller=="c": inputs.append(fixture.out/"workload.o")
                executable=fixture.out/(provider+"-"+caller)
                n.command(["clang","-no-pie","-Wl,-T,"+str(n.FIXTURES/"discard.lds"),
                    fixture.out/("provider-"+provider+".o"),fixture.out/("uuid_"+caller+".o"),
                    *inputs,build/"rust/core.o",build/"rust/compiler_builtins.o","-o",executable],provider+"-"+caller+"-executable")
                n.command([executable],provider+"-"+caller+"-execute")
        prefix='pub extern "C" fn uuid_rust_exercise() -> ffi::c_int {'
        rust_source=check.sources("rust")
        self.assertEqual(rust_source.count(prefix),1)
        for label,statement,extra,expected in (
                ("constant-result","return 9217;","",87),
                ("c-delegation",'unsafe extern "C" { fn uuid_exercise() -> ffi::c_int; } return unsafe { uuid_exercise() };',"workload",0)):
            source=fixture.out/(label+".rs")
            source.write_text(rust_source.replace(prefix,prefix+'\n if core::hint::black_box(true) { '+statement+' }'))
            obj=source.with_suffix(".o")
            n.command([n.RUST,*rf,"-Copt-level=2","--cfg","MODULE",source,"--emit=obj="+str(obj)],label+"-compile")
            if label=="c-delegation":
                with self.assertRaises(ValueError): check.verify_rust_workload(obj)
            executable=fixture.out/label
            inputs=[*shared,fixture.out/"transport-rust.o"]
            if extra: inputs.append(fixture.out/(extra+".o"))
            n.command(["clang","-no-pie","-Wl,-T,"+str(n.FIXTURES/"discard.lds"),fixture.out/"provider-c.o",obj,
                *inputs,build/"rust/core.o",build/"rust/compiler_builtins.o","-o",executable],label+"-link")
            n.command([executable],label+"-execute",expected=expected)
        inputs=[*shared,fixture.out/"transport-rust.o"]
        original=(ROOT/"lib/uuid.c").read_text()
        for label,old,new,expected in (
                ("no-write","return __uuid_parse(uuid, u->b, uuid_index);","return 0;",87),
                ("bad-null","const guid_t guid_null;","const guid_t guid_null = {{1}};",87),
                ("wrong-rng-size","get_random_bytes(uuid, 16);","get_random_bytes(uuid, 15);",98),
                ("wrong-variant","uuid[8] = (uuid[8] & 0x3F) | 0x80;","uuid[8] = 0;",87)):
            self.assertEqual(original.count(old),1)
            source=fixture.out/(label+".c")
            source.write_text(original.replace(old,new))
            obj=source.with_suffix(".o")
            n.command(["clang",*cf,"-O2","-D__DISABLE_EXPORTS","-c",source,"-o",obj],label+"-compile",build)
            executable=fixture.out/label
            n.command(["clang","-no-pie","-Wl,-T,"+str(n.FIXTURES/"discard.lds"),
                obj,fixture.out/"uuid_rust.o",*inputs,build/"rust/core.o",build/"rust/compiler_builtins.o",
                "-o",executable],label+"-link")
            n.command([executable],label+"-execute",expected=expected)

    def test_actual_workload_both_languages_both_providers_executes(self):
        self.compile("x86",execute=True)


if __name__=="__main__":
    unittest.main()
