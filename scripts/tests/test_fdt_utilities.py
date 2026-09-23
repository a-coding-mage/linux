# SPDX-License-Identifier: GPL-2.0-or-later
"""C parity and safety regressions for fdtget, fdtput and fdtoverlay."""

import os
from pathlib import Path
import struct
import shlex
import subprocess
import tempfile
import unittest

from libfdt_test_support import cached_fdt_utilities
import test_libfdt


class FdtUtilitiesTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        try:
            cls.tools = cached_fdt_utilities()
        except subprocess.CalledProcessError as exc:
            raise RuntimeError(exc.stderr.decode()) from exc

    def compare(self, tool, args, files=None, stdin=b"", env=None):
        with tempfile.TemporaryDirectory(prefix="fdt-cli-") as directory:
            work = Path(directory)
            results = []
            for binary in self.tools[tool]:
                for name, data in (files or {}).items():
                    (work / name).write_bytes(data)
                result = subprocess.run([tool] + args, executable=binary, cwd=work,
                                        input=stdin, capture_output=True, timeout=10,
                                        env=dict(os.environ, LC_ALL="C", **(env or {})))
                tree = {os.fsencode(p.name): p.read_bytes() for p in work.iterdir() if p.is_file()}
                results.append((result.returncode, result.stdout, result.stderr, tree))
                for p in work.iterdir():
                    p.unlink()
            self.assertEqual(results[0][:3], results[1][:3], (tool, args))
            for name, data in results[0][3].items():
                self.assertEqual(data, results[1][3].get(name), (tool, args, name))
            self.assertEqual(results[0][3].keys(), results[1][3].keys())
            return results[0]

    @staticmethod
    def sample(capacity=4096):
        return test_libfdt.dtb((b"", [(b"text", b"hello\0world\0"),
                                      (b"empty", b""), (b"ints", struct.pack(">III", 0, 0xffffffff, 0x12345678)),
                                      (b"bytes", bytes(range(7))), (b"unterminated", b"abc"),
                                      (b"emptystrings", b"\0x\0"), (b"odd", b"abcde\0")], [
            (b"bus", [], [(b"child@1", [(b"value", b"v\0")], [])]),
            (b"second", [], [])]), capacity=capacity)

    def test_cli_help_usage_options_and_errors(self):
        for tool in self.tools:
            for args in ([], ["-h"], ["-?"], ["-t"], ["--bad"], ["-h", "--bad"]):
                with self.subTest(tool=tool, args=args):
                    self.compare(tool, args)
        for args in (["file"], ["file", "/"], ["-t", "invalid", "file"], ["missing", "/", "x"],
                     ["-p", "missing", "/"], ["-t", "hhhx", "file"], ["-t", "", "file"]):
            self.compare("fdtget", args)
            self.compare("fdtput", args)
        for args in (["--help"], ["--version"], ["--input=base"], ["-i", "base", "-o", "out"],
                     ["-i", "base", "-o", "out", "overlay"], ["--verbose=no"], ["--input"],
                     ["--out", "out", "-i", "base", "overlay"], ["--v"], ["--ver"], ["--h=foo"]):
            self.compare("fdtoverlay", args)

    def test_get_values_formats_defaults_and_lists(self):
        files = {"tree": self.sample()}
        for kind in (None, "s", "i", "u", "x", "bx", "hhi", "hx", "li", "Lx", "llu", "bbs"):
            for prop in ("text", "empty", "ints", "bytes", "unterminated", "emptystrings", "odd", "missing"):
                args = (["-t", kind] if kind else []) + ["tree", "/", prop]
                with self.subTest(args=args):
                    self.compare("fdtget", args, files)
        for args in (["-p", "tree", "/", "/bus"], ["-l", "tree", "/", "/bus"],
                     ["-d", "fallback", "tree", "/", "missing", "/no", "x", "/", "text"],
                     ["tree", "/", "text", "-t", "s"], ["-l", "tree", "/none"],
                     ["-p", "-d", "absent", "tree", "/none"]):
            self.compare("fdtget", args, files)
        self.compare("fdtget", ["-", "/", "text"], stdin=self.sample())

    def test_put_values_creation_and_failures(self):
        for kind in (None, "s", "i", "u", "x", "bx", "hhi", "li", "Lx", "llu"):
            values = ["hello", "", "two"] if kind == "s" else ["1", "-2", "0x1234", "0123", "4294967295"]
            args = ["-v"] + (["-t", kind] if kind else []) + ["tree", "/", "new"] + values
            with self.subTest(kind=kind):
                self.compare("fdtput", args, {"tree": self.sample()})
        for args in (["tree", "/", "new"], ["-c", "tree", "/new", "/new/child"],
                     ["-c", "-p", "tree", "/a/b/c", "/a/b/d"],
                     ["-c", "tree", "/bus"], ["-c", "tree", "bad"],
                     ["-c", "tree", "/missing/child"], ["-c", "-p", "tree", "/a//b/"],
                     ["-p", "tree", "/a/b", "new", "12"], ["tree", "/missing", "new", "12"],
                     ["-c", "tree"], ["tree", "/bus", "new", "1", "2"]):
            self.compare("fdtput", args, {"tree": self.sample()})
        self.compare("fdtput", ["tree", "/", "new", "1"], {"tree": self.sample(0)})
        self.compare("fdtput", ["-t", "s", "-", "/", "new", "hello"], stdin=self.sample())

    def test_non_utf8_names_and_paths(self):
        name = b"tree\xff"
        blob = test_libfdt.dtb((b"", [(b"prop\xff", b"value\xfe\0")], [(b"node\xfe", [], [])]), capacity=512)
        # Python's surrogateescape retains these path bytes through pathlib.
        files = {os.fsdecode(name): blob}
        self.compare("fdtget", [name, b"/", b"prop\xff", "-t", "s"], files)
        self.compare("fdtget", ["-l", name, "/"], files)
        self.compare("fdtput", ["-t", "s", name, b"/node\xfe", b"prop\xff", b"value\xfd"], files)
        self.compare("fdtget", [b"missing\xff", "/", "x"])

    def test_overlay_cli_growth_chaining_and_diagnostics(self):
        base = test_libfdt.dtb((b"", [], []))
        overlay = test_libfdt.dtb((b"", [], [(b"f", [(b"target-path", b"/\0")], [
            (b"__overlay__", [(b"new-property", bytes(range(128))*20)], [(b"child", [], [])])])]))
        overlay2 = test_libfdt.dtb((b"", [], [(b"f", [(b"target-path", b"/child\0")], [
            (b"__overlay__", [(b"more", b"test")], [])])]))
        files = {"base": base, "overlay": overlay, "overlay2": overlay2}
        for args in (["-i", "base", "-o", "out", "overlay"],
                     ["-v", "-i", "base", "-o", "out", "overlay", "overlay2"],
                     ["--input=base", "--output=out", "--verbose", "overlay"],
                     ["-i", "base", "-o", "out", "overlay2"],
                     ["-i", "base", "-o", "missing/out", "overlay"],
                     ["-i", "base", "-o", "out", "missing"]):
            self.compare("fdtoverlay", args, files)
        self.compare("fdtoverlay", ["-i", "-", "-o", "-", "overlay"], {"overlay": overlay}, stdin=base)
        self.compare("fdtoverlay", ["-i", "-", "-o", "out", "-"], stdin=base)

    def test_safe_fixes_for_undefined_legacy_formats(self):
        with tempfile.TemporaryDirectory(prefix="fdt-safe-") as temp:
            tree = Path(temp) / "tree"
            tree.write_bytes(self.sample())
            rust = self.tools["fdtput"][1]
            result = subprocess.run([rust, "-t", "hx", tree, "/", "halfwords", "1234", "abcd"], capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            get = subprocess.run([self.tools["fdtget"][1], "-t", "hx", tree, "/", "halfwords"], capture_output=True)
            self.assertEqual(get.stdout, b"1234 abcd\n")
            result = subprocess.run([rust, tree, "/", "bad", "not-an-integer"], capture_output=True)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"Invalid integer", result.stderr)
            result = subprocess.run([rust, "-t", "r", tree, "/", "raw", "one", "two"], capture_output=True)
            self.assertEqual(result.returncode, 0)
            get = subprocess.run([self.tools["fdtget"][1], "-t", "r", tree, "/", "raw"], capture_output=True)
            self.assertEqual(get.stdout, b"onetwo\n")

    def test_compiled_dts_overlay_pipeline(self):
        import dtc_test_support
        with tempfile.TemporaryDirectory(prefix="dtc-overlay-") as temp:
            work = Path(temp)
            dtc = dtc_test_support.build_c(work)
            sources = {
                "base": b'''/dts-v1/;
                    /memreserve/ 0x12345678 0x1000;
                    / { compatible = "test,base"; #address-cells = <1>; #size-cells = <1>;
                        bus: bus { status = "disabled";
                            dev: device@100 { reg = <0x100 0x20>; phandle = <10>; }; };
                    };''',
                "first": b'''/dts-v1/; /plugin/;
                    &bus { status = "okay";
                        added: child { compatible = "test,added"; target = <&dev>;
                            local = <&added>; other = <&second>; };
                        second: second { bytes = [00 01 ff 05]; };
                    };
                    &dev { new = <1 2 3>; };''',
                "second": b'''/dts-v1/; /plugin/;
                    &added { more = "overlay two"; };
                    &{/bus} { another { data = /bits/ 64 <0x123456789abcdef>; }; };''',
                "conflict": b'''/dts-v1/; /plugin/;
                    &bus { dev_new: device@100 { reg = <0x100 0x40>; phandle = <7>; };
                        ref { target = <&dev_new>; };
                    };''',
            }
            files = {}
            for name, source in sources.items():
                path = work / (name + ".dts")
                path.write_bytes(source)
                blob = work / (name + ".dtb")
                result = subprocess.run([dtc, "-@", "-q", "-I", "dts", "-O", "dtb", "-o", blob, path], capture_output=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                files[name] = blob.read_bytes()
            self.compare("fdtoverlay", ["-v", "-i", "base", "-o", "out", "first", "second", "conflict"], files)

    def test_kernel_of_overlay_fixtures(self):
        import dtc_test_support
        root = dtc_test_support.ROOT
        directory = root / "drivers/of/unittest-data"
        with tempfile.TemporaryDirectory(prefix="of-overlays-") as temp:
            work = Path(temp)
            dtc = dtc_test_support.build_c(work)
            def compile_blob(path):
                source = subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) +
                                        ["-E", "-P", "-nostdinc", "-undef", "-D__DTS__",
                                         "-x", "assembler-with-cpp", "-I", str(root / "include"),
                                         "-I", str(directory), str(path)],
                                        check=True, capture_output=True).stdout
                result = subprocess.run([dtc, "-@", "-q", "-I", "dts", "-O", "dtb"],
                                        input=source, capture_output=True)
                self.assertEqual(result.returncode, 0, (path, result.stderr))
                return result.stdout
            base = compile_blob(directory / "testcases.dtso")
            for path in sorted(directory.glob("overlay*.dtso")):
                with self.subTest(overlay=path.name):
                    overlay = compile_blob(path)
                    self.compare("fdtoverlay", ["-i", "base", "-o", "out", "overlay"],
                                 {"base": base, "overlay": overlay})


if __name__ == "__main__":
    unittest.main()
