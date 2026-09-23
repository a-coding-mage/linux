# SPDX-License-Identifier: GPL-2.0
"""Alpha boot-tool parity against the original C and target UAPI layouts."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "arch/alpha/boot/tools"


def elf(payload, bss=0, *, address=0xfffffc0000310000, entry=None,
        offset=512, phoff=64, phnum=1, machine=0x9026, kind=2):
    if entry is None:
        entry = address
    header = struct.pack("<16sHHIQQQIHHHHHH", b"\x7fELF\2\1\1"+bytes(9),
                         kind, machine, 1, entry, phoff, 0, 0, 64, 56, phnum, 64, 0, 0)
    program = struct.pack("<II6Q", 1, 7, offset, address, address,
                          len(payload), len(payload)+bss, 8192)
    image = bytearray(max(offset+len(payload), phoff+56, 64))
    image[:64] = header
    image[phoff:phoff+56] = program
    image[offset:offset+len(payload)] = payload
    return bytes(image)


def ecoff(text, data=b"", bss=0, sections=3, address=0xfffffc0000310000,
          magic=0o407, flags=2, optional_size=80):
    file = struct.pack("<HHIQIHH", 0x183, sections, 0, 0, 0, optional_size, flags)
    aout = struct.pack("<8QIIQ", magic, len(text), len(data), bss, address,
                       address, address+len(text), address+len(text)+len(data), 0, 0, 0)
    offset = (len(file)+len(aout)+sections*64+15)&~15
    return (file+aout).ljust(offset, b"\0")+text+data


class AlphaBootTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temp = tempfile.TemporaryDirectory(prefix="alpha-tools-")
        cls.addClassCleanup(cls.temp.cleanup)
        work = Path(cls.temp.name)
        cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        cls.tools = {}
        for tool in ("mkbb", "objstrip"):
            c, rust = work / (tool+"-c"), work / (tool+"-rust")
            command = cc+["-O2", "-D__EXPORTED_HEADERS__", "-I", str(ROOT / "arch/alpha/include/uapi"),
                          "-I", str(ROOT / "include/uapi"), str(SOURCE / (tool+".c")), "-o", str(c)]
            try:
                subprocess.run(command, check=True, capture_output=True)
                subprocess.run(rustc+["--edition=2021", "-O", "-Dwarnings", "-Wmissing_docs",
                                      "-Wrust_2018_idioms", "-Wunreachable_pub",
                                      str(SOURCE / (tool+".rs")), "-o", str(rust)],
                               check=True, capture_output=True)
            except subprocess.CalledProcessError as exc:
                raise RuntimeError(exc.stderr.decode()) from exc
            cls.tools[tool] = c, rust

    def compare(self, tool, args, files=None):
        with tempfile.TemporaryDirectory(prefix="alpha-case-") as temp:
            work = Path(temp)
            results = []
            for binary in self.tools[tool]:
                for name, content in (files or {}).items():
                    (work / os.fsdecode(name)).write_bytes(content)
                result = subprocess.run([tool]+args, executable=binary, cwd=work,
                                        capture_output=True, timeout=5,
                                        env=dict(os.environ, LC_ALL="C"))
                tree = {os.fsencode(path.name): path.read_bytes() for path in work.iterdir()}
                results.append((result.returncode, result.stdout, result.stderr, tree))
                for path in work.iterdir():
                    path.unlink()
            self.assertEqual(results[0][:3], results[1][:3], (tool, args))
            self.assertEqual(results[0][3].keys(), results[1][3].keys())
            for name in results[0][3]:
                self.assertEqual(results[0][3][name], results[1][3][name], (tool, args, name))
            return results[0]

    def test_primary_bootblocks_lengths_and_checksum(self):
        rng = random.Random(0xA1FA)
        for size in (0, 1, 20, 511, 512, 513, 8191, 8192, 8193, 65537):
            result = self.compare("objstrip", ["-p", "input", "output"], {"input": rng.randbytes(size)})
            words = struct.unpack("<64Q", result[3][b"output"])
            self.assertEqual(words[60], (size+511)//512)
            self.assertEqual(words[61:63], (1, 0))
            self.assertEqual(words[63], sum(words[:63])&((1<<64)-1))
            self.assertEqual(result[1], str((size+511)&~511).encode()+b"\n")

    def test_mkbb_disklabel_merge_and_tail_preservation(self):
        rng = random.Random(0xBB)
        for size in (512, 513, 8192):
            for case in range(20):
                old = rng.randbytes(size)
                loader = rng.randbytes(512+case)
                result = self.compare("mkbb", ["disk", "loader"], {"disk": old, "loader": loader})
                actual = result[3][b"disk"]
                self.assertEqual(actual[64:340], old[64:340])
                self.assertEqual(actual[512:], old[512:])
                self.assertEqual(actual[:64]+actual[340:504], loader[:64]+loader[340:504])
                words = struct.unpack("<64Q", actual[:512])
                self.assertEqual(words[63], sum(words[:63])&((1<<64)-1))

    def test_elf_extraction_padding_and_entry_adjustment(self):
        rng = random.Random(0xE1F)
        for size in (0, 1, 7, 511, 512, 513, 8191, 8192, 8193, 16555):
            for bss in (0, 1, 513, 9000):
                payload = rng.randbytes(size)
                for flag in ("-v", "-b", "-vb", "-vvb"):
                    with self.subTest(size=size, bss=bss, flag=flag):
                        result = self.compare("objstrip", [flag, "input", "output"], {"input": elf(payload, bss)})
                        expected = size+bss
                        if "b" in flag:
                            expected = (expected+511)&~511
                        self.assertEqual(result[3][b"output"], payload+bytes(expected-size))
        for delta in (0, 1, 32, 511, 1024):
            payload = rng.randbytes(1024)
            result = self.compare("objstrip", ["-vb", "input"],
                                  {"input": elf(payload, 2048, address=0x1000, entry=0x1000+delta)})
            self.assertEqual(result[1][:1024-delta], payload[delta:])
        for phnum in (0, 2, 8):
            self.compare("objstrip", ["-v", "input"], {"input": elf(b"payload", phnum=phnum)})

    def test_ecoff_sections_sizes_and_verbose_ranges(self):
        rng = random.Random(42)
        for sections in (0, 1, 3, 17, 65535):
            for address in (0, 1, 0x1234, 0xfffffc0000310000):
                for flag in ("-v", "-vb"):
                    text, data = rng.randbytes(9001), rng.randbytes(73)
                    image = ecoff(text, data, 517, sections, address)
                    result = self.compare("objstrip", [flag, "input", "output"], {"input": image})
                    self.assertEqual(result[3][b"output"][:len(text)+len(data)], text+data)

    def test_cli_error_ordering_and_unknown_flags(self):
        for tool in self.tools:
            for args in ([], ["-h"], ["missing"], ["missing", "other"], ["-"], ["--"]):
                self.compare(tool, args)
        for args in (["-p", "input"], ["-vp", "input", "output", "ignored"],
                     ["-xyzvb", "input", "output"], ["--", "input"], ["-v", "-v", "input"],
                     ["input", "missing/output"]):
            self.compare("objstrip", args, {"input": elf(b"payload", 5)})
        self.compare("mkbb", ["disk", "missing"], {"disk": bytes(512)})
        for size in (0, 1, 511):
            self.compare("mkbb", ["disk", "loader"], {"disk": bytes(512), "loader": bytes(size)})
            self.compare("mkbb", ["disk", "loader"], {"disk": bytes(size), "loader": bytes(512)})

    def test_format_diagnostics_and_short_reads(self):
        for image in (elf(b"abc", kind=1), elf(b"abc", machine=62),
                      ecoff(b"abc", flags=0), ecoff(b"abc", optional_size=79),
                      ecoff(b"abc", magic=0o413)):
            self.compare("objstrip", ["input", "output"], {"input": image, "output": b"previous"})
        self.compare("objstrip", ["-v", "input", "output"], {"input": elf(bytes(100))[:-1]})
        image = bytearray(elf(bytes(100)))
        struct.pack_into("<Q", image, 32, len(image)-20)
        self.compare("objstrip", ["-v", "input", "output"], {"input": image})

    def test_non_utf8_paths_and_diagnostics(self):
        self.compare("objstrip", [b"-v", b"input\xff", b"output\xfe"], {b"input\xff": elf(b"abc", 7)})
        self.compare("objstrip", [b"-v", b"input\xff"], {b"input\xff": ecoff(b"abc", flags=0)})
        self.compare("mkbb", [b"disk\xff", b"loader\xfe"], {b"disk\xff": bytes(600), b"loader\xfe": bytes(512)})
        self.compare("mkbb", [b"missing\xff", b"loader\xfe"])

    def test_malformed_input_safely_rejected(self):
        malformed = [b"", b"\x7fELF", bytes(103)]
        malformed.append(elf(b"x", entry=0xfffffc0000320000))
        image = bytearray(elf(b"ab"))
        struct.pack_into("<Q", image, 64+40, 1)
        malformed.append(bytes(image))
        image = bytearray(ecoff(b"ab"))
        struct.pack_into("<Q", image, 32, 0xffffffffffffffff)
        struct.pack_into("<Q", image, 40, 1)
        malformed.append(bytes(image))
        for data in malformed:
            with tempfile.TemporaryDirectory(prefix="alpha-invalid-") as temp:
                path = Path(temp) / "input"
                path.write_bytes(data)
                result = subprocess.run([self.tools["objstrip"][1], path], capture_output=True, timeout=5)
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertNotIn(b"panicked", result.stderr)

    def test_large_sparse_primary_uses_full_file_size(self):
        with tempfile.TemporaryDirectory(prefix="alpha-sparse-") as temp:
            work = Path(temp)
            source = work / "input"
            size = (1 << 40)+123
            with source.open("wb") as stream:
                stream.truncate(size)
            outputs = []
            for binary in self.tools["objstrip"]:
                output = work / "block"
                result = subprocess.run(["objstrip", "-p", source, output], executable=binary,
                                        capture_output=True, timeout=5)
                self.assertEqual(result.returncode, 0, result.stderr)
                outputs.append((result.stdout, result.stderr, output.read_bytes()))
            self.assertEqual(outputs[0], outputs[1])
            words = struct.unpack("<64Q", outputs[1][2])
            self.assertEqual(words[60], (size+511)//512)

    def test_directory_and_write_failures(self):
        with tempfile.TemporaryDirectory(prefix="alpha-errors-") as temp:
            work = Path(temp)
            (work / "dir").mkdir()
            (work / "input").write_bytes(elf(bytes(100), 17))
            for tool, args in (("mkbb", ["dir", "input"]), ("mkbb", ["input", "dir"]),
                               ("objstrip", ["dir", "output"]), ("objstrip", ["input", "dir"]),
                               ("objstrip", ["-p", "input", "/dev/full"]),
                               ("objstrip", ["input", "/dev/full"])):
                if "/dev/full" in args and not Path("/dev/full").exists():
                    continue
                results = [subprocess.run([tool]+args, executable=binary, cwd=work,
                                          capture_output=True, timeout=5)
                           for binary in self.tools[tool]]
                self.assertEqual((results[0].returncode, results[0].stdout, results[0].stderr),
                                 (results[1].returncode, results[1].stdout, results[1].stderr), (tool,args))


if __name__ == "__main__":
    unittest.main()
