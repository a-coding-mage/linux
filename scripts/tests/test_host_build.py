# SPDX-License-Identifier: GPL-2.0-only
"""Verify Rust selection and dependency tracking through the real Kbuild rules."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

from gendwarf_test_support import cflags as gendwarf_cflags, libraries as gendwarf_libraries, require_c_headers
from test_migration_invariants import environment


ROOT = Path(__file__).resolve().parents[2]
TOOLS = (
    "scripts/basic/fixdep",
    "scripts/kallsyms",
    "scripts/tracepoint-update",
    "scripts/gen_packed_field_checks",
    "scripts/asn1_compiler",
    "scripts/unifdef",
    "scripts/sorttable",
    "scripts/recordmcount",
    "scripts/insert-sys-cert",
    "scripts/sign-file",
    "certs/extract-cert",
    "scripts/mod/mk_elfconfig",
    "scripts/mod/modpost",
    "scripts/selinux/mdp/mdp",
    "security/selinux/genheaders",
    "scripts/ipe/polgen/polgen",
    "scripts/kconfig/conf",
    "scripts/genksyms/genksyms",
    "scripts/gendwarfksyms/gendwarfksyms",
    "scripts/dtc/dtc",
    "scripts/dtc/fdtoverlay",
    "scripts/dtc/fdtget",
    "scripts/dtc/fdtput",
    "arch/x86/tools/vdso2c",
    "arch/x86/tools/relocs",
    "arch/x86/tools/insn_decoder_test",
    "arch/x86/tools/insn_sanity",
    "arch/x86/boot/compressed/mkpiggy",
    "arch/x86/boot/mkcpustr",
    "arch/powerpc/boot/addnote",
    "arch/powerpc/boot/hack-coff",
    "arch/powerpc/boot/mktree",
    "arch/arm/vdso/vdsomunge",
    "arch/arm64/kernel/pi/relacheck",
    "arch/arm64/kvm/hyp/nvhe/gen-hyprel",
    "arch/alpha/boot/tools/mkbb",
    "arch/alpha/boot/tools/objstrip",
    "arch/mips/tools/elf-entry",
    "arch/mips/tools/loongson3-llsc-check",
    "arch/mips/boot/elf2ecoff",
    "arch/mips/boot/tools/relocs",
    "arch/mips/boot/compressed/calc_vmlinuz_load_addr",
    "arch/mips/vdso/genvdso",
    "arch/s390/tools/gen_facilities",
    "arch/s390/tools/gen_opcode_table",
    "arch/s390/tools/relocs",
    "arch/sparc/boot/piggyback",
    "arch/sparc/vdso/vdso2c",
    "usr/gen_init_cpio",
    "lib/raid/raid6/mktables",
    "lib/crc/gen_crc32table",
    "lib/crc/gen_crc64table",
    "fs/unicode/mkutf8data",
    "drivers/accessibility/speakup/makemapdata",
    "drivers/accessibility/speakup/genmap",
)


class RustHostBuildTest(unittest.TestCase):
    def test_out_of_tree_build_and_incremental_dependencies(self):
        env = os.environ.copy()
        # This suite also runs inside make rust-host-tests. Start an independent
        # make invocation so the caller's recursion and output paths cannot leak.
        for key in list(env):
            if key.startswith("KBUILD_") or key in (
                    "MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
                    "sub_make_done", "srctree", "srcroot", "objtree", "VPATH"):
                env.pop(key)
        env["LC_ALL"] = "C"
        with tempfile.TemporaryDirectory(prefix="rust-kbuild-") as tmp:
            command = shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", str(ROOT), "O=" + tmp, "rust-host-tools",
                "HOST_TOOLS_LANG=rust",
                "HOSTRUSTFLAGS=-Dwarnings",
                "HOSTLDLIBS_gendwarfksyms=" + shlex.join(gendwarf_libraries()),
                "HOSTCC=" + os.environ.get("HOSTCC", "cc"),
                "HOSTRUSTC=" + os.environ.get("HOSTRUSTC", "rustc")]

            def make(*args):
                result = subprocess.run(command + list(args), env=env,
                                        capture_output=True, text=True, timeout=120)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                return result.stdout

            first = make("-j2")
            self.assertNotIn("  HOSTCC ", first)
            self.assertNotIn("  HOSTLD ", first)
            for tool in TOOLS:
                binary = Path(tmp) / tool
                self.assertTrue(binary.is_file(), tool)
                depfile = binary.with_name("." + binary.name + ".cmd")
                deps = depfile.read_text()
                self.assertIn(str(ROOT / (tool + ".rs")), deps)
                self.assertIn("-Dwarnings", deps)
            tracepoint = Path(tmp) / "scripts/.tracepoint-update.cmd"
            self.assertIn(str(ROOT / "scripts/elf-parse.rs"), tracepoint.read_text())
            sorttable = Path(tmp) / "scripts/.sorttable.cmd"
            self.assertIn(str(ROOT / "scripts/elf-parse.rs"), sorttable.read_text())
            recordmcount = Path(tmp) / "scripts/.recordmcount.cmd"
            self.assertIn(str(ROOT / "scripts/elf-parse.rs"), recordmcount.read_text())
            self.assertIn(str(ROOT / "scripts/recordmcount_header.rs"), recordmcount.read_text())
            genheaders = (Path(tmp) / "security/selinux/.genheaders.cmd").read_text()
            selinux_headers = [ROOT / "security/selinux/include" / header
                               for header in ("classmap_header.rs", "initial_sid_to_string_header.rs")]
            for header in selinux_headers:
                self.assertIn(str(header), genheaders)
            unicode = (Path(tmp) / "fs/unicode/.mkutf8data.cmd").read_text()
            unicode_modules = [ROOT / ("fs/unicode/mkutf8data_" + name + ".rs")
                               for name in ("data", "io", "model", "trie", "verify")]
            for dependency in unicode_modules:
                self.assertIn(str(dependency), unicode)
            speakup_dir = Path("drivers/accessibility/speakup")
            speakup_utils = ROOT / speakup_dir / "utils_header.rs"
            for tool in ("makemapdata", "genmap"):
                deps = (Path(tmp) / speakup_dir / ("." + tool + ".cmd")).read_text()
                self.assertIn(str(speakup_utils), deps)
            self.assertIn(str(Path(tmp) / speakup_dir / "mapdata.bin"),
                          (Path(tmp) / speakup_dir / ".genmap.cmd").read_text())
            for target in ("makemapdata.frontend", "mapdata.bin"):
                deps = (Path(tmp) / speakup_dir / ("." + target + ".cmd")).read_text()
                self.assertIn(str(ROOT / speakup_dir / "mapdata_frontend.py"), deps)
                self.assertIn(str(ROOT / speakup_dir / "utils.h"), deps)
            speakup_outputs = [Path(tmp) / speakup_dir / name for name in (
                "mapdata.h", "mapdata.bin", "makemapdata.frontend", "speakupmap.h")]
            speakup_mtimes = [path.stat().st_mtime_ns for path in speakup_outputs]
            crc32_module = ROOT / "lib/crc/../../include/linux/crc32poly_header.rs"
            self.assertIn(str(crc32_module),
                          (Path(tmp) / "lib/crc/.gen_crc32table.cmd").read_text())
            certificate = Path(tmp) / "scripts/.insert-sys-cert.cmd"
            self.assertIn(str(ROOT / "scripts/elf-parse.rs"), certificate.read_text())
            sign_file = Path(tmp) / "scripts/.sign-file.cmd"
            self.assertIn(str(ROOT / "scripts/ssl-common_header.rs"), sign_file.read_text())
            self.assertIn(str(ROOT / "scripts/../tools/include/uapi/linux/module_signature_header.rs"), sign_file.read_text())
            extract = Path(tmp) / "certs/.extract-cert.cmd"
            extract_ssl_module = ROOT / "certs/../scripts/ssl-common_header.rs"
            self.assertIn(str(extract_ssl_module), extract.read_text())
            modpost = Path(tmp) / "scripts/mod/.modpost.cmd"
            for module in ("modpost_header", "file2alias", "sumversion", "symsearch",
                           "c_literal", "vmlinux_export_data", "module_metadata_data"):
                self.assertIn(str(ROOT / ("scripts/mod/" + module + ".rs")), modpost.read_text())
            modpost_elf_module = ROOT / "scripts/mod/../elf-parse.rs"
            self.assertIn(str(modpost_elf_module), modpost.read_text())
            conf = Path(tmp) / "scripts/kconfig/.conf.cmd"
            for module in ("confdata", "expr", "lexer", "menu", "model", "parser", "preprocess", "symbol"):
                self.assertIn(str(ROOT / ("scripts/kconfig/" + module + ".rs")), conf.read_text())
            genksyms = Path(tmp) / "scripts/genksyms/.genksyms.cmd"
            for module in ("genksyms_header", "keywords", "lexer", "parser", "parser_tables"):
                self.assertIn(str(ROOT / ("scripts/genksyms/" + module + ".rs")), genksyms.read_text())
            gendwarf = (Path(tmp) / "scripts/gendwarfksyms/.gendwarfksyms.cmd").read_text()
            for module in ("cache", "die", "dwarf", "gendwarfksyms_header", "kabi", "reader", "symbols", "types"):
                self.assertIn(str(ROOT / ("scripts/gendwarfksyms/" + module + ".rs")), gendwarf)
            gendwarf_elf_module = ROOT / "scripts/gendwarfksyms/../elf-parse.rs"
            self.assertIn(str(gendwarf_elf_module), gendwarf)
            dtc = Path(tmp) / "scripts/dtc/.dtc.cmd"
            for module in ("checks", "data", "dtc_header", "flattree", "fstree", "lexer",
                           "livetree", "parser", "srcpos", "treesource", "util", "version_gen_header"):
                self.assertIn(str(ROOT / ("scripts/dtc/" + module + ".rs")), dtc.read_text())
            self.assertIn(str(ROOT / "scripts/dtc/libfdt/fdt_header.rs"), dtc.read_text())
            for tool in ("fdtoverlay", "fdtget", "fdtput"):
                deps = (Path(tmp) / ("scripts/dtc/." + tool + ".cmd")).read_text()
                for module in ("mod", "fdt_header", "libfdt_header", "libfdt_env_header",
                               "libfdt_internal_header", "fdt", "fdt_ro", "fdt_rw", "fdt_sw",
                               "fdt_wip", "fdt_addresses", "fdt_empty_tree", "fdt_overlay",
                               "fdt_strerror", "tools"):
                    self.assertIn(str(ROOT / ("scripts/dtc/libfdt/" + module + ".rs")), deps)
            vdso = Path(tmp) / "arch/x86/tools/.vdso2c.cmd"
            self.assertIn(str(ROOT / "arch/x86/tools/vdso2c_header.rs"), vdso.read_text())
            vdso_elf_module = ROOT / "arch/x86/tools/../../../scripts/elf-parse.rs"
            self.assertIn(str(vdso_elf_module), vdso.read_text())
            sparc_vdso = Path(tmp) / "arch/sparc/vdso/.vdso2c.cmd"
            sparc_vdso_module = ROOT / "arch/sparc/vdso/vdso2c_header.rs"
            self.assertIn(str(sparc_vdso_module), sparc_vdso.read_text())
            relocs = Path(tmp) / "arch/x86/tools/.relocs.cmd"
            for module in ("relocs_common", "relocs_header", "relocs_32", "relocs_64"):
                self.assertIn(str(ROOT / ("arch/x86/tools/" + module + ".rs")), relocs.read_text())
            self.assertIn(str(vdso_elf_module), relocs.read_text())
            decoder_dependencies = (
                "arch/x86/tools/insn_test_common.rs", "tools/arch/x86/lib/insn.rs",
                "arch/x86/lib/insn.rs", "arch/x86/lib/inat.rs", "arch/x86/lib/inat_tables.rs",
                "arch/x86/lib/x86-opcode-map.txt", "arch/x86/include/asm/insn_header.rs",
                "arch/x86/include/asm/emulate_prefix_header.rs",
                "arch/x86/include/asm/inat_header.rs", "arch/x86/include/asm/inat_types_header.rs")
            decoder_paths = {}
            for tool in ("insn_decoder_test", "insn_sanity"):
                deps = (Path(tmp) / ("arch/x86/tools/." + tool + ".cmd")).read_text()
                # Nested path imports retain ../ spelling in rustc's depfile.
                paths = {os.path.normpath(token): token for token in deps.split()
                         if token.startswith("/") and not token.endswith(":")}
                for dependency in decoder_dependencies:
                    path = str(ROOT / dependency)
                    self.assertIn(path, paths)
                    decoder_paths[path] = paths[path]
            relacheck = Path(tmp) / "arch/arm64/kernel/pi/.relacheck.cmd"
            arm_elf_module = ROOT / "arch/arm64/kernel/pi/../../../../scripts/elf-parse.rs"
            self.assertIn(str(arm_elf_module), relacheck.read_text())
            hyprel = Path(tmp) / "arch/arm64/kvm/hyp/nvhe/.gen-hyprel.cmd"
            hyp_elf_module = ROOT / "arch/arm64/kvm/hyp/nvhe/../../../../../scripts/elf-parse.rs"
            self.assertIn(str(hyp_elf_module), hyprel.read_text())
            s390_relocs = Path(tmp) / "arch/s390/tools/.relocs.cmd"
            s390_elf_module = ROOT / "arch/s390/tools/../../../scripts/elf-parse.rs"
            self.assertIn(str(s390_elf_module), s390_relocs.read_text())
            mips_vdso = Path(tmp) / "arch/mips/vdso/.genvdso.cmd"
            mips_vdso_elf = ROOT / "arch/mips/vdso/../../../scripts/elf-parse.rs"
            self.assertIn(str(mips_vdso_elf), mips_vdso.read_text())
            self.assertIn(str(ROOT / "arch/mips/vdso/genvdso_header.rs"), mips_vdso.read_text())
            mips_relocs = (Path(tmp) / "arch/mips/boot/tools/.relocs.cmd").read_text()
            mips_relocs_elf = ROOT / "arch/mips/boot/tools/../../../../scripts/elf-parse.rs"
            self.assertIn(str(mips_relocs_elf), mips_relocs)
            for module in ("relocs_32", "relocs_64", "relocs_header", "relocs_main"):
                self.assertIn(str(ROOT / ("arch/mips/boot/tools/" + module + ".rs")), mips_relocs)
            llsc_elf_module = ROOT / "arch/mips/tools/../../../scripts/elf-parse.rs"
            self.assertIn(str(llsc_elf_module),
                          (Path(tmp) / "arch/mips/tools/.loongson3-llsc-check.cmd").read_text())
            cpustr = Path(tmp) / "arch/x86/boot/.mkcpustr.cmd"
            self.assertIn(str(ROOT / "arch/x86/boot/../include/asm/cpufeatures_header.rs"), cpustr.read_text())
            powerpc_module = ROOT / "arch/powerpc/boot/host_tool.rs"
            alpha_module = ROOT / "arch/alpha/boot/tools/bootblock.rs"
            for tool in ("mkbb", "objstrip"):
                depfile = Path(tmp) / ("arch/alpha/boot/tools/." + tool + ".cmd")
                self.assertIn(str(alpha_module), depfile.read_text())
            for tool in ("addnote", "hack-coff", "mktree"):
                depfile = Path(tmp) / ("arch/powerpc/boot/." + tool + ".cmd")
                self.assertIn(str(powerpc_module), depfile.read_text())
            mips_modules = (
                ("arch/mips/tools/.elf-entry.cmd", "arch/mips/tools/../boot/host_tool.rs"),
                ("arch/mips/tools/.loongson3-llsc-check.cmd", "arch/mips/tools/../boot/host_tool.rs"),
                ("arch/mips/boot/.elf2ecoff.cmd", "arch/mips/boot/host_tool.rs"),
                ("arch/mips/boot/compressed/.calc_vmlinuz_load_addr.cmd",
                 "arch/mips/boot/compressed/../host_tool.rs"),
            )
            for depfile, module in mips_modules:
                self.assertIn(str(ROOT / module), (Path(tmp) / depfile).read_text())
            mtimes = [(Path(tmp) / tool).stat().st_mtime_ns for tool in TOOLS]
            second = make()
            self.assertNotIn("HOSTRUSTC", second)
            self.assertEqual(mtimes, [(Path(tmp) / tool).stat().st_mtime_ns
                                      for tool in TOOLS])
            self.assertEqual(speakup_mtimes,
                             [path.stat().st_mtime_ns for path in speakup_outputs])
            recursive = shlex.split(os.environ.get("MAKE", "make")) + ["-W", str(speakup_utils)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            for tool in ("makemapdata", "genmap"):
                self.assertIn("--emit=link=" + str(speakup_dir / tool), rebuild)
            for dependency in (ROOT / speakup_dir / "mapdata_frontend.py",
                               ROOT / "include/uapi/linux/input-event-codes.h"):
                recursive = shlex.split(os.environ.get("MAKE", "make")) + ["-W", str(dependency)]
                rebuild = make("-n", "MAKE=" + shlex.join(recursive))
                self.assertIn("mapdata_frontend.py", rebuild)
                self.assertIn("--emit=link=" + str(speakup_dir / "genmap"), rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                "-W", str(crc32_module)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            self.assertIn("--emit=link=lib/crc/gen_crc32table", rebuild)
            self.assertNotIn("--emit=link=lib/crc/gen_crc64table", rebuild)
            for dependency in unicode_modules:
                recursive = shlex.split(os.environ.get("MAKE", "make")) + ["-W", str(dependency)]
                rebuild = make("-n", "MAKE=" + shlex.join(recursive))
                self.assertIn("--emit=link=fs/unicode/mkutf8data", rebuild)
            for header in selinux_headers:
                recursive = shlex.split(os.environ.get("MAKE", "make")) + ["-W", str(header)]
                rebuild = make("-n", "MAKE=" + shlex.join(recursive))
                self.assertIn("--emit=link=security/selinux/genheaders", rebuild)
            # Pretend the imported module changed; do not edit the source tree.
            # GNU make does not propagate -W through MAKEFLAGS. Supply it to
            # each recursive make, which is where the source dependencies live.
            recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                "-W", str(ROOT / "scripts/elf-parse.rs"),
                "-W", str(modpost_elf_module),
                "-W", str(gendwarf_elf_module),
                "-W", str(vdso_elf_module),
                "-W", str(arm_elf_module),
                "-W", str(hyp_elf_module),
                "-W", str(s390_elf_module),
                "-W", str(llsc_elf_module),
                "-W", str(mips_relocs_elf),
                "-W", str(mips_vdso_elf)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            self.assertIn("--emit=link=scripts/tracepoint-update", rebuild)
            self.assertIn("--emit=link=scripts/sorttable", rebuild)
            self.assertIn("--emit=link=scripts/recordmcount", rebuild)
            self.assertIn("--emit=link=scripts/insert-sys-cert", rebuild)
            self.assertIn("--emit=link=scripts/mod/modpost", rebuild)
            self.assertIn("--emit=link=scripts/gendwarfksyms/gendwarfksyms", rebuild)
            self.assertIn("--emit=link=arch/x86/tools/vdso2c", rebuild)
            self.assertIn("--emit=link=arch/x86/tools/relocs", rebuild)
            self.assertIn("--emit=link=arch/arm64/kernel/pi/relacheck", rebuild)
            self.assertIn("--emit=link=arch/arm64/kvm/hyp/nvhe/gen-hyprel", rebuild)
            self.assertIn("--emit=link=arch/s390/tools/relocs", rebuild)
            self.assertIn("--emit=link=arch/mips/vdso/genvdso", rebuild)
            self.assertIn("--emit=link=arch/mips/boot/tools/relocs", rebuild)
            self.assertIn("--emit=link=arch/mips/tools/loongson3-llsc-check", rebuild)
            self.assertNotIn("--emit=link=scripts/kallsyms", rebuild)
            for module in ("reader", "dwarf", "types"):
                recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                    "-W", str(ROOT / ("scripts/gendwarfksyms/" + module + ".rs"))]
                rebuild = make("-n", "MAKE=" + shlex.join(recursive))
                self.assertIn("--emit=link=scripts/gendwarfksyms/gendwarfksyms", rebuild)
                self.assertNotIn("--emit=link=scripts/genksyms/genksyms", rebuild)
            for dependency in ("arch/x86/lib/insn.rs", "arch/x86/lib/x86-opcode-map.txt"):
                recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                    "-W", decoder_paths[str(ROOT / dependency)]]
                rebuild = make("-n", "MAKE=" + shlex.join(recursive))
                for tool in ("insn_decoder_test", "insn_sanity"):
                    self.assertIn("--emit=link=arch/x86/tools/" + tool, rebuild)
                self.assertNotIn("--emit=link=arch/x86/tools/relocs", rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make")) + ["-W", str(sparc_vdso_module)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            self.assertIn("--emit=link=arch/sparc/vdso/vdso2c", rebuild)
            self.assertNotIn("--emit=link=arch/sparc/boot/piggyback", rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make")) + ["-W", str(powerpc_module)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            for tool in ("addnote", "hack-coff", "mktree"):
                self.assertIn("--emit=link=arch/powerpc/boot/" + tool, rebuild)
            self.assertNotIn("--emit=link=arch/x86/tools/relocs", rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make")) + ["-W", str(alpha_module)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            for tool in ("mkbb", "objstrip"):
                self.assertIn("--emit=link=arch/alpha/boot/tools/" + tool, rebuild)
            self.assertNotIn("--emit=link=arch/x86/tools/relocs", rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make"))
            for _, module in mips_modules:
                recursive += ["-W", str(ROOT / module)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            for tool in ("arch/mips/tools/elf-entry", "arch/mips/boot/elf2ecoff",
                         "arch/mips/tools/loongson3-llsc-check",
                         "arch/mips/boot/compressed/calc_vmlinuz_load_addr"):
                self.assertIn("--emit=link=" + tool, rebuild)
            self.assertNotIn("--emit=link=arch/x86/tools/relocs", rebuild)
            # ARM64's compat vDSO borrows the ARM helper through a relative
            # hostprogs path. Exercise that actual rule, not only the ARM one.
            borrowed = subprocess.run(shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", tmp, "-f", str(ROOT / "scripts/Makefile.build"),
                "srctree=" + str(ROOT), "srcroot=" + str(ROOT), "objtree=.",
                "obj=arch/arm64/kernel/vdso32", "CC=" + os.environ.get("HOSTCC", "cc"),
                "HOSTCC=" + os.environ.get("HOSTCC", "cc"),
                "HOSTRUSTC=" + os.environ.get("HOSTRUSTC", "rustc"),
                "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings",
                "arch/arm64/kernel/vdso32/../../../arm/vdso/vdsomunge"],
                env=env, capture_output=True, text=True, timeout=30)
            self.assertEqual(borrowed.returncode, 0, borrowed.stdout + borrowed.stderr)
            self.assertNotIn("HOSTCC", borrowed.stdout)
            # A parent make exports its quiet mode. The selected source lives
            # in the command/dependency record regardless of log verbosity.
            self.assertIn("vdsomunge.rs", (Path(tmp) / "arch/arm/vdso/.vdsomunge.cmd").read_text())
            # MIPS compressed boot also borrows a host program from its parent.
            borrowed = subprocess.run(shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", tmp, "-f", str(ROOT / "scripts/Makefile.build"),
                "srctree=" + str(ROOT), "srcroot=" + str(ROOT), "objtree=.",
                "obj=arch/mips/boot/compressed", "CC=" + os.environ.get("HOSTCC", "cc"),
                "HOSTCC=" + os.environ.get("HOSTCC", "cc"),
                "HOSTRUSTC=" + os.environ.get("HOSTRUSTC", "rustc"),
                "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings",
                "arch/mips/boot/compressed/../elf2ecoff"],
                env=env, capture_output=True, text=True, timeout=30)
            self.assertEqual(borrowed.returncode, 0, borrowed.stdout + borrowed.stderr)
            self.assertNotIn("HOSTCC", borrowed.stdout)
            self.assertIn("elf2ecoff.rs", (Path(tmp) / "arch/mips/boot/.elf2ecoff.cmd").read_text())
            recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                "-W", str(ROOT / "scripts/ssl-common_header.rs"), "-W", str(extract_ssl_module)]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            self.assertIn("--emit=link=scripts/sign-file", rebuild)
            self.assertIn("--emit=link=certs/extract-cert", rebuild)
            self.assertNotIn("--emit=link=scripts/kallsyms", rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                "-W", str(ROOT / "scripts/dtc/dtc_header.rs")]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            self.assertIn("--emit=link=scripts/dtc/dtc", rebuild)
            self.assertNotIn("--emit=link=scripts/genksyms/genksyms", rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                "-W", str(ROOT / "scripts/dtc/libfdt/fdt_ro.rs")]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            for tool in ("fdtoverlay", "fdtget", "fdtput"):
                self.assertIn("--emit=link=scripts/dtc/" + tool, rebuild)
            self.assertNotIn("--emit=link=scripts/dtc/dtc", rebuild)
            recursive = shlex.split(os.environ.get("MAKE", "make")) + [
                "-W", str(ROOT / "scripts/openssl_config.py")]
            rebuild = make("-n", "MAKE=" + shlex.join(recursive))
            self.assertIn("--emit=link=scripts/sign-file", rebuild)
            self.assertIn("--emit=link=certs/extract-cert", rebuild)

            # ARM64's current Kconfig marks big-endian kernels BROKEN. Still
            # verify the real host rule selects the target byte-order branch,
            # independently of the host and without forcing a broken kernel.
            big = make("CONFIG_CPU_BIG_ENDIAN=y")
            self.assertIn("HOSTRUSTC arch/arm64/kvm/hyp/nvhe/gen-hyprel", big)
            self.assertIn("--cfg CONFIG_CPU_BIG_ENDIAN", hyprel.read_text())
            little = make()
            self.assertIn("HOSTRUSTC arch/arm64/kvm/hyp/nvhe/gen-hyprel", little)
            self.assertNotIn("--cfg CONFIG_CPU_BIG_ENDIAN", hyprel.read_text())

            # Keep the original C decoder rules usable and verify switching
            # languages in the same object tree, including command tracking.
            tables = Path(tmp) / "arch/x86/lib/inat-tables.c"
            tables.parent.mkdir(parents=True, exist_ok=True)
            generated = subprocess.run(shlex.split(os.environ.get("AWK", "awk")) + [
                "-f", str(ROOT / "arch/x86/tools/gen-insn-attr-x86.awk"),
                str(ROOT / "arch/x86/lib/x86-opcode-map.txt")],
                env=env, capture_output=True, timeout=30)
            self.assertEqual(generated.returncode, 0, generated.stderr)
            tables.write_bytes(generated.stdout)
            base = shlex.split(os.environ.get("MAKE", "make")) + [
                "-C", tmp, "-f", str(ROOT / "scripts/Makefile.build"),
                "srctree=" + str(ROOT), "srcroot=" + str(ROOT), "objtree=.",
                "VPATH=" + str(ROOT), "building_out_of_srctree=1", "SUBARCH=x86",
                "obj=arch/x86/tools", "CC=" + os.environ.get("HOSTCC", "cc"),
                "HOSTCC=" + os.environ.get("HOSTCC", "cc"),
                "HOSTRUSTC=" + os.environ.get("HOSTRUSTC", "rustc"),
                "KBUILD_HOSTCFLAGS=-O2", "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings",
                "arch/x86/tools/insn_decoder_test", "arch/x86/tools/insn_sanity"]
            for language in ("c", "rust", "c", "rust"):
                result = subprocess.run(base + ["HOST_TOOLS_LANG=" + language], env=env,
                                        capture_output=True, text=True, timeout=30)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                for tool in ("insn_decoder_test", "insn_sanity"):
                    binary = Path(tmp) / ("arch/x86/tools/" + tool)
                    record = binary.with_name("." + tool + ".cmd").read_text()
                    self.assertIn(tool + (".rs" if language == "rust" else ".c"), record)
                before = [(Path(tmp) / ("arch/x86/tools/" + tool)).stat().st_mtime_ns
                          for tool in ("insn_decoder_test", "insn_sanity")]
                result = subprocess.run(base + ["HOST_TOOLS_LANG=" + language], env=env,
                                        capture_output=True, text=True, timeout=30)
                self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                self.assertEqual(before, [(Path(tmp) / ("arch/x86/tools/" + tool)).stat().st_mtime_ns
                                          for tool in ("insn_decoder_test", "insn_sanity")])

    def test_gendwarfksyms_c_rust_switch(self):
        require_c_headers()
        env = environment()
        make = shlex.split(os.environ.get("MAKE", "make"))
        cc = os.environ.get("HOSTCC", "cc")
        rustc = os.environ.get("HOSTRUSTC", "rustc")
        with tempfile.TemporaryDirectory(prefix="gendwarf-kbuild-switch-") as tmp:
            work = Path(tmp)
            bootstrap = subprocess.run(make + ["-C", tmp, "-f", str(ROOT / "scripts/Makefile.build"),
                "srctree=" + str(ROOT), "srcroot=" + str(ROOT), "objtree=.",
                "VPATH=" + str(ROOT), "building_out_of_srctree=1", "CONFIG_SHELL=/bin/sh",
                "HOST_TOOLS_LANG=c", "HOSTCC=" + cc, "HOSTRUSTC=false",
                "KBUILD_HOSTCFLAGS=-O2 -I" + str(ROOT / "scripts/include"),
                "obj=scripts/basic", "scripts/basic/fixdep"],
                env=env, capture_output=True, text=True, timeout=60)
            self.assertEqual(bootstrap.returncode, 0, bootstrap.stdout + bootstrap.stderr)
            target = "scripts/gendwarfksyms/gendwarfksyms"
            base = make + ["-C", tmp, "-f", str(ROOT / "scripts/Makefile.build"),
                "srctree=" + str(ROOT), "srcroot=" + str(ROOT), "objtree=.",
                "VPATH=" + str(ROOT), "building_out_of_srctree=1", "obj=scripts/gendwarfksyms",
                "CC=" + cc, "HOSTCC=" + cc,
                "KBUILD_HOSTCFLAGS=-O2 -I" + str(ROOT / "scripts/include"),
                "HOST_EXTRACFLAGS=" + shlex.join(gendwarf_cflags()),
                "KBUILD_HOSTRUSTFLAGS=--edition=2021 -O -Dwarnings -Wmissing-docs -Wunreachable-pub -Wrust_2018_idioms",
                "HOSTLDLIBS_gendwarfksyms=" + shlex.join(gendwarf_libraries()), target]
            obj = work / "fixture.o"
            compiled = subprocess.run([*shlex.split(cc), "-g", "-c", "-x", "c", "-",
                                       "-o", str(obj)], input=b"int exported;\n", env=env,
                                      capture_output=True, timeout=30)
            self.assertEqual(compiled.returncode, 0, compiled.stderr)
            expected = None
            for language in ("c", "rust", "c", "rust"):
                with self.subTest(language=language):
                    command = base + ["HOST_TOOLS_LANG=" + language,
                                      "HOSTRUSTC=" + (rustc if language == "rust" else "false")]
                    result = subprocess.run(command, env=env, capture_output=True, text=True, timeout=60)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    binary = work / target
                    record = binary.with_name(".gendwarfksyms.cmd").read_text()
                    if language == "rust":
                        self.assertIn("gendwarfksyms.rs", record)
                        self.assertIn("--emit=link=" + target, record)
                    else:
                        self.assertNotIn("--emit=link=", record)
                        for unit in ("gendwarfksyms", "cache", "die", "dwarf", "kabi", "symbols", "types"):
                            self.assertIn("scripts/gendwarfksyms/" + unit + ".o", record)
                    symtypes = work / "fixture.symtypes"
                    ran = subprocess.run([binary, "-T", symtypes, obj], input=b"exported\n",
                                         env=env, capture_output=True, timeout=10)
                    actual = (ran.returncode, ran.stdout, ran.stderr, symtypes.read_bytes())
                    self.assertEqual(ran.returncode, 0, ran.stderr)
                    if expected is None:
                        expected = actual
                    self.assertEqual(actual, expected)
                    stamp = binary.stat().st_mtime_ns
                    result = subprocess.run(command, env=env, capture_output=True, text=True, timeout=60)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertEqual(binary.stat().st_mtime_ns, stamp)


if __name__ == "__main__":
    unittest.main()
