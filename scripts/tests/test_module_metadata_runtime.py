#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Read-only native metadata gates plus private copies for negative controls.

Explicit MODULE_METADATA_X86_BUILD/ARM64_BUILD inputs must be valid completed
selected kernels; absent inputs skip honestly. No compiler, make, VM or native
output write is performed. Mutations change only private copies of real ELF.
"""
import os
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile
import unittest
from unittest import mock

import check_module_metadata as checker
from check_reciprocal_kernel import module_elf


class ModuleMetadataRoutingTests(unittest.TestCase):
    def test_explicit_empty_or_invalid_native_input_is_not_a_skip(self):
        for native in (X86ModuleMetadataRuntimeTests,Arm64ModuleMetadataRuntimeTests):
            for value in ("","/nonexistent-module-metadata-input"):
                with self.subTest(variable=native.variable,value=value),mock.patch.dict(os.environ,{native.variable:value}):
                    with self.assertRaises(ValueError): native.setUpClass()

    def test_c_selection_keeps_original_source_and_suppression(self):
        with tempfile.TemporaryDirectory() as temporary:
            build = Path(temporary)
            (build/".config").write_text("CONFIG_MODULES=y\n")
            module = build/"fixture.ko"
            command = build/".fixture.mod.o.cmd"
            command.write_text("savedcmd_fixture.mod.o := cc -D__DISABLE_EXPORTS fixture.mod.c\n")
            with mock.patch("rust_exports_test_support.read_exports",return_value=[]):
                self.assertEqual(checker.verify_module_metadata(build,module,work=build,require_c_suppression=True),build/"fixture.mod.c")
                self.assertEqual(checker.selected_metadata(build,module),build/"fixture.mod.c")
                for flags in ("cc", "cc -D__DISABLE_EXPORTS -U__DISABLE_EXPORTS"):
                    command.write_text("savedcmd_fixture.mod.o := "+flags+" fixture.mod.c\n")
                    with self.assertRaises(ValueError): checker.verify_module_metadata(build,module,work=build,require_c_suppression=True)
                command.write_text("savedcmd_fixture.mod.o := cc -D__DISABLE_EXPORTS fixture.mod.c\n")
                with mock.patch("rust_exports_test_support.read_exports",return_value=[{"name":"extra"}]),self.assertRaises(ValueError):
                    checker.verify_module_metadata(build,module,work=build,require_c_suppression=True)

    def test_selected_source_never_uses_stale_c_orphan(self):
        with tempfile.TemporaryDirectory() as temporary:
            build=Path(temporary); module=build/"fixture.ko"
            (build/".config").write_text("CONFIG_RUST_MODULE_METADATA=y\n")
            (build/"fixture.mod.c").write_text("stale inactive output")
            self.assertEqual(checker.metadata_source(build,module),build/"fixture.mod.h")
            with self.assertRaises((ValueError,FileNotFoundError)): checker.selected_metadata(build,module)


class NativeMetadataMixin:
    @classmethod
    def setUpClass(cls):
        value=os.environ.get(cls.variable)
        if value is None: raise unittest.SkipTest(cls.variable+" not supplied")
        if not value: raise ValueError("explicit "+cls.variable+" is empty")
        cls.build=Path(value).resolve()
        if not cls.build.is_dir(): raise ValueError("invalid explicit "+cls.variable)
        cls.config=checker.configuration(cls.build)
        if cls.config.get("RUST_MODULE_METADATA")!="y" or cls.config.get(cls.arch)!="y":
            raise ValueError("explicit native input is not the requested selected Rust metadata kernel")
        cls.module=cls.build/"lib/math/prime_numbers.ko"
        cls.obj=cls.module.with_suffix(".mod.o")
        cls.values=checker.generated_values(cls.module.with_suffix(".mod.rs"))
        cls.layout=checker.module_layout(cls.obj)

    def setUp(self):
        temporary=tempfile.TemporaryDirectory(prefix="metadata-runtime-",dir=os.environ.get("MODULE_METADATA_TEST_WORK"))
        self.addCleanup(temporary.cleanup)
        self.work=Path(temporary.name)

    def verify_object(self,path,values=None):
        checker.verify_object(path,self.module.with_suffix(".o"),self.config,values or self.values,self.layout)

    def test_actual_provider_framework_suite_and_external_caller(self):
        for relative in ("lib/math/prime_numbers.ko","lib/math/tests/prime_numbers_kunit.ko","lib/kunit/kunit.ko",
                         "rust-prime-numbers-test/primes_rust_abi.ko"):
            with self.subTest(module=relative):
                module=self.build/relative
                self.assertEqual(checker.verify_module_metadata(self.build,module),module.with_suffix(".mod.h"))

    def copy_inputs(self):
        relative=self.obj.relative_to(self.build)
        destination=self.work/relative
        destination.parent.mkdir(parents=True)
        for suffix in (".ko",".o",".mod.o",".mod.h",".mod.rs"):
            source=self.module.with_suffix(suffix)
            shutil.copy2(source,self.work/source.relative_to(self.build))
        for source in (self.obj,self.module.with_suffix(".mod.rs")):
            source=source.with_name("."+source.name+".cmd")
            target=self.work/source.relative_to(self.build)
            text=source.read_text().replace(str(self.module.with_suffix(".mod.rs")),str(self.work/self.module.relative_to(self.build).with_suffix(".mod.rs")))
            target.write_text(text)
        # Relative generated-header dependencies still refer to the completed
        # original target, not fabricated headers. All writes remain private.
        (self.work/"include").symlink_to(self.build/"include",target_is_directory=True)
        (self.work/"arch").symlink_to(self.build/"arch",target_is_directory=True)
        return self.work/self.module.relative_to(self.build)

    def test_saved_source_cfg_feature_dependency_and_staleness_controls(self):
        module=self.copy_inputs()
        obj=module.with_suffix(".mod.o")
        command=obj.with_name("."+obj.name+".cmd")
        original=command.read_text()
        self.assertEqual(checker.verify_module_metadata(self.build,module),module.with_suffix(".mod.h"))
        replacements=((str(checker.SOURCE),str(checker.ROOT/"scripts/module-common.rs")),
                      ("--crate-name=module_metadata","--crate-name=owner"),
                      ("-Zallow-features= ","-Zallow-features=linkage "),
                      ("--cfg MODULE","--cfg OWNER"),
                      ("@./include/generated/rustc_cfg","@wrong_cfg"),
                      (str(self.build/"rust/libbindings.rmeta"),str(self.build/"rust/libcore.rmeta")))
        for old,new in replacements:
            with self.subTest(change=old):
                self.assertIn(old,original)
                command.write_text(original.replace(old,new))
                with self.assertRaises((ValueError,FileNotFoundError)):
                    checker.verify_module_metadata(self.build,module)
        command.write_text(original+"\n#SYMVER invented 0x00000000\n")
        with self.assertRaises(ValueError): checker.verify_module_metadata(self.build,module)
        command.write_text(original)
        data=module.with_suffix(".mod.rs")
        stamp=obj.stat().st_mtime_ns+1000000
        os.utime(data,ns=(stamp,stamp))
        with self.assertRaisesRegex(ValueError,"older|stale"): checker.verify_module_metadata(self.build,module)

    def section(self,raw,name):
        start=struct.unpack_from("<Q",raw,40)[0]
        count,strings=struct.unpack_from("<HH",raw,60)
        headers=[struct.unpack_from("<IIQQQQIIQQ",raw,start+index*64) for index in range(count)]
        label=headers[strings]; names=raw[label[4]:label[4]+label[5]]
        for index,header in enumerate(headers):
            if names[header[0]:names.index(0,header[0])]==name:
                return start+index*64,header
        raise AssertionError(name)

    def reject_mutation(self,label,mutate):
        path=self.work/(label+".o"); raw=bytearray(self.obj.read_bytes())
        mutate(raw); path.write_bytes(raw)
        # ELF parsing must still succeed; malformed storage is not a substitute
        # for the specific semantic/layout negative being exercised here.
        module_elf(path)
        with self.assertRaises(ValueError): self.verify_object(path)

    def test_allocated_payload_flags_alignment_and_runtime_code_controls(self):
        for name in (b".modinfo",b"__versions",b"__version_ext_crcs",b"__version_ext_names",b".gnu.linkonce.this_module"):
            with self.subTest(section=name):
                self.reject_mutation("payload",lambda raw: raw.__setitem__(self.section(raw,name)[1][4],raw[self.section(raw,name)[1][4]]^1))
                self.reject_mutation("alignment",lambda raw: struct.pack_into("<Q",raw,self.section(raw,name)[0]+48,2))
                self.reject_mutation("executable",lambda raw: struct.pack_into("<Q",raw,self.section(raw,name)[0]+8,self.section(raw,name)[1][2]|4))

    def test_lifecycle_relocation_and_symbol_marker_controls(self):
        def relocation(raw):
            _,header=self.section(raw,b".rela.gnu.linkonce.this_module")
            offset=struct.unpack_from("<Q",raw,header[4])[0]
            struct.pack_into("<Q",raw,header[4],offset+8)
        self.reject_mutation("wrong-lifecycle-field",relocation)
        def rename(raw,replacement):
            _,table=self.section(raw,b".strtab")
            data=raw[table[4]:table[4]+table[5]]
            candidates=[symbol[0] for symbol in module_elf(self.obj)[2]
                        if symbol[0].startswith(b"_RN") and b"___MODULE_INFO_" in symbol[0]]
            original=candidates[0]; offset=data.index(original+b"\0")+table[4]
            raw[offset:offset+len(original)]=replacement+bytes(len(original)-len(replacement))
        for name in (b"__IS_RUST_MODULE",b"unexpected_global",b"_RNmodule_metadata99unexpected"):
            with self.subTest(name=name): self.reject_mutation("owner-global",lambda raw: rename(raw,name))
        def imports(raw):
            _,table=self.section(raw,b".strtab")
            data=raw[table[4]:table[4]+table[5]]
            offset=data.index(b"cleanup_module\0")+table[4]
            raw[offset:offset+14]=b"other___module"
        self.reject_mutation("extra-import",imports)

    def test_generated_metadata_corruption_and_export_owner_controls(self):
        raw=self.module.with_suffix(".mod.rs").read_text()
        data=self.work/"candidate.mod.rs"
        for old,new in (("0x85f0928e","0x85f0928f"),
                        ('"with_primes", "with_primes"','"with_primes", "next_prime_number"')):
            # Do not depend on a particular native CRC: choose the first
            # generated CRC token when testing an arbitrary configured kernel.
            if old.startswith("0x"):
                import re
                old=re.search(r'SYMBOL_CRC_NORMALIZED!\("with_primes", (0x[0-9a-f]+)\)',raw)[1]
                new=hex(int(old,16)^1)
            self.assertIn(old,raw); data.write_text(raw.replace(old,new,1))
            values=checker.generated_values(data)
            with self.assertRaises(ValueError): self.verify_object(self.obj,values)
        for text in (raw+"fn extra_runtime() {}\n",raw.replace("module_value(&[","module_value_mutated(&[",1)):
            data.write_text(text)
            with self.assertRaises(ValueError): checker.generated_values(data)

    def test_final_linked_owner_versions_exports_and_lifecycle_are_not_assumed(self):
        module=self.copy_inputs(); original=module.read_bytes()
        for name in (b".gnu.linkonce.this_module",b"__version_ext_crcs",b"__ksymtab_strings"):
            with self.subTest(section=name):
                raw=bytearray(original); _,section=self.section(raw,name)
                raw[section[4]]^=1; module.write_bytes(raw)
                with self.assertRaisesRegex(ValueError,"final module|final export"):
                    checker.verify_module_metadata(self.build,module)
        raw=bytearray(original)
        _,section=self.section(raw,b".rela.gnu.linkonce.this_module")
        offset=struct.unpack_from("<Q",raw,section[4])[0]
        struct.pack_into("<Q",raw,section[4],offset+8); module.write_bytes(raw)
        with self.assertRaisesRegex(ValueError,"final module changed selected lifecycle"):
            checker.verify_module_metadata(self.build,module)

    def test_consistent_generated_and_linked_lifecycle_omission_is_rejected(self):
        module = self.copy_inputs()
        source = module.with_suffix(".mod.rs")
        original = source.read_text()
        assignment = "    value.exit = Some(bindings::cleanup_module);\n"
        self.assertIn(assignment, original)
        self.assertEqual(self.values["owner"][1], {"exit": "cleanup_module"})
        self.assertIn(".exit = cleanup_module", module.with_suffix(".mod.h").read_text())
        source.write_text(original.replace(assignment, ""))
        for suffix in (".mod.o", ".ko"):
            target = module.with_suffix(suffix)
            args = ["llvm-objcopy", "--remove-section=.rela.gnu.linkonce.this_module"]
            if suffix == ".mod.o":
                args.append("--strip-symbol=cleanup_module")
            subprocess.run([*args, str(target)], check=True, capture_output=True, timeout=30)
            module_elf(target)  # The negative is semantic, not malformed ELF.
        with self.assertRaisesRegex(ValueError, "lifecycle differs from actual owner"):
            checker.verify_module_metadata(self.build, module)

    def test_owner_lifecycle_changes_not_owner_timestamp_determine_metadata(self):
        module = self.copy_inputs()
        owner = module.with_suffix(".o")
        stamp = max(module.with_suffix(suffix).stat().st_mtime_ns
                    for suffix in (".mod.h", ".mod.rs", ".mod.o")) + 1000000
        os.utime(owner, ns=(stamp, stamp))
        os.utime(module, ns=(stamp + 1000000, stamp + 1000000))
        # modpost's write_if_changed deliberately preserves the older header
        # when a rebuilt implementation needs identical loader metadata.
        self.assertEqual(checker.verify_module_metadata(self.build, module),
                         module.with_suffix(".mod.h"))
        subprocess.run(["llvm-objcopy", "--redefine-sym=cleanup_module=other_cleanup",
                        str(owner)], check=True, capture_output=True, timeout=30)
        module_elf(owner)
        with self.assertRaisesRegex(ValueError, "lifecycle differs from actual owner"):
            checker.verify_object(module.with_suffix(".mod.o"), owner,
                                  self.config, self.values, self.layout)


class X86ModuleMetadataRuntimeTests(NativeMetadataMixin,unittest.TestCase):
    variable="MODULE_METADATA_X86_BUILD"
    arch="X86_64"


class Arm64ModuleMetadataRuntimeTests(NativeMetadataMixin,unittest.TestCase):
    variable="MODULE_METADATA_ARM64_BUILD"
    arch="ARM64"


if __name__=="__main__": unittest.main(verbosity=2)
