#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""C/Rust byte-for-byte tests for SPARC boot and vDSO host helpers."""

import os
from pathlib import Path
import random
import shlex
import shutil
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


def vdso_fixture(bits=64, loads=1, load_offset=0, address=0, memsz=32,
                 filesz=32, tags=((0, 0),), symtab=True, dynamic=True):
    wide = bits == 64
    hsize, psize, ssize = (64, 56, 64) if wide else (52, 32, 40)
    count = loads + int(dynamic)
    data = bytearray(hsize + psize * count)
    dynoff = len(data)
    dyn = b"".join(struct.pack(">QQ" if wide else ">II", *tag) for tag in tags)
    data.extend(dyn)
    shoff = len(data)
    data.extend(bytes(ssize * 2))
    ident = b"\x7fELF" + bytes((2 if wide else 1, 2, 1)) + bytes(9)
    data[:hsize] = struct.pack(">16sHHIQQQIHHHHHH" if wide else ">16sHHIIIIIHHHHHH",
                              ident, 3, 43 if wide else 2, 1, 0, hsize, shoff,
                              0, hsize, psize, count, ssize, 2, 0)
    programs = [(1, load_offset, address, filesz, memsz)] * loads
    if dynamic:
        programs.append((2, dynoff, 0, len(dyn), len(dyn)))
    for index, (kind, offset, addr, fsize, msize) in enumerate(programs):
        ph = (struct.pack(">IIQQQQQQ", kind, 5, offset, addr, 0, fsize, msize, 8192)
              if wide else struct.pack(">IIIIIIII", kind, offset, addr, 0, fsize, msize, 5, 8192))
        data[hsize + index * psize:hsize + (index + 1) * psize] = ph
    struct.pack_into(">I", data, shoff + ssize + 4, 2 if symtab else 3)
    return data


def aout_fixture(size=4096, branch=None, marker=40):
    data = bytearray((i * 7 + 19) % 256 for i in range(size))
    data[:4] = b"\x01\x03\x01\x07"
    if branch is not None:
        data[34:36] = struct.pack(">H", branch)
    if marker is not None:
        data[marker:marker + 4] = b"HdrS"
    return data


class SparcToolsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="sparc-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.tools = {}
        for name, directory in (("piggyback", "boot"), ("vdso2c", "vdso")):
            source = ROOT / "arch/sparc" / directory / name
            cls.tools[name] = [cls.work / (name + "-c"), cls.work / (name + "-rust")]
            commands = [shlex.split(os.environ.get("HOSTCC", "cc")) + [
                "-O2", "-I" + str(ROOT / "tools/include"),
                str(source.with_suffix(".c")), "-o", str(cls.tools[name][0])],
                shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
                "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs",
                "-Wunreachable-pub", "-Wrust-2018-idioms",
                str(source.with_suffix(".rs")), "-o", str(cls.tools[name][1])]]
            for command in commands:
                result = subprocess.run(command, capture_output=True, text=True)
                if result.returncode:
                    raise RuntimeError(result.stdout + result.stderr)

    def compare_vdso(self, raw, stripped=bytes(range(128)), name=b"vdso-image.c", status=0,
                     args=None):
        with tempfile.TemporaryDirectory(prefix="sparc-vdso-") as tmp:
            work = Path(tmp)
            (work / "raw").write_bytes(raw)
            (work / "stripped").write_bytes(stripped)
            output = work / os.fsdecode(name)
            results = []
            for tool in self.tools["vdso2c"]:
                run = subprocess.run([tool, *(args if args is not None else [b"raw", b"stripped", name])],
                                     cwd=work, capture_output=True, timeout=5)
                content = output.read_bytes() if output.is_file() else None
                if content is not None:
                    output.unlink()
                results.append((run.returncode, run.stdout,
                                run.stderr.replace(os.fsencode(tool.name), b"vdso2c"), content))
            self.assertEqual(results[0][0], status, results[0][:3])
            self.assertEqual(*results)
            return results[1][3]

    def compare_piggy(self, image, ramdisk=b"ramdisk", bits="32", start=0xf0004000,
                      end=0xf0007000, symbols=None, status=0, names=None):
        symbols = symbols if symbols is not None else f"{start:08x} T _start\n{end:08x} T _end\n".encode()
        names = names or [b"image", b"System.map", b"initrd"]
        with tempfile.TemporaryDirectory(prefix="sparc-piggy-") as tmp:
            work = Path(tmp)
            paths = [work / os.fsdecode(name) for name in names]
            paths[1].write_bytes(symbols)
            paths[2].write_bytes(ramdisk)
            results = []
            for tool in self.tools["piggyback"]:
                paths[0].write_bytes(image)
                paths[0].chmod(0o751)
                run = subprocess.run([tool, bits, *names], cwd=work, capture_output=True, timeout=5)
                self.assertLess(paths[0].stat().st_size, 2**20, "fixture unexpectedly creates a huge sparse file")
                results.append((run.returncode, run.stdout, run.stderr, paths[0].read_bytes(),
                                paths[0].stat().st_mode & 0o777))
            self.assertEqual(results[0][0], status, results[0][:3])
            self.assertEqual(*results)
            return results[1][3]

    def test_vdso_classes_pages_and_raw_outputs(self):
        rng = random.Random(432)
        for bits in (32, 64):
            for size in (32, 33, 8191, 8192, 8193, 16384):
                with self.subTest(bits=bits, size=size):
                    stripped = rng.randbytes(size)
                    output = self.compare_vdso(vdso_fixture(bits), stripped)
                    self.assertIn(f".size = {(size + 8191) // 8192 * 8192},".encode(), output)
                    self.assertEqual(self.compare_vdso(vdso_fixture(bits), stripped, b"image.so"), stripped)

    def test_vdso_load_validation_and_error_order(self):
        for bits in (32, 64):
            for options in ({"loads": 0}, {"loads": 2}, {"load_offset": 1},
                            {"address": 1}, {"memsz": 33}, {"symtab": False}):
                with self.subTest(bits=bits, options=options):
                    self.compare_vdso(vdso_fixture(bits, **options), status=1)
            self.compare_vdso(vdso_fixture(bits), bytes(31), status=1)
            self.compare_vdso(vdso_fixture(bits, dynamic=False))
            self.compare_vdso(vdso_fixture(bits, memsz=0, filesz=0), b"x")

    def test_vdso_all_dynamic_tags_and_termination(self):
        for bits in (32, 64):
            for tag in [*range(40), 0x6fffffff, (1 << bits) - 1]:
                for value in (0, 1):
                    self.compare_vdso(vdso_fixture(bits, tags=((tag, value), (0, 0))),
                                      status=int(tag in (8, 18) and value != 0))
            self.compare_vdso(vdso_fixture(bits, tags=((0, 0), (18, 9))))
            self.compare_vdso(vdso_fixture(bits, tags=()))

    def test_vdso_header_and_stride_compatibility(self):
        for bits in (32, 64):
            raw = vdso_fixture(bits)
            for byte, value in ((0, 0), (5, 0), (5, 1), (6, 0)):
                changed = raw.copy()
                changed[byte] = value
                self.compare_vdso(changed)
            for stride in (0, 1, 32, 56, 0xffff):
                changed = raw.copy()
                struct.pack_into(">H", changed, 54 if bits == 64 else 42, stride)
                self.compare_vdso(changed)
            for stride in (0, 1, 4, 8, 16):
                changed = raw.copy()
                off = struct.unpack_from(">Q" if bits == 64 else ">I", changed,
                                         40 if bits == 64 else 32)[0]
                struct.pack_into(">H", changed, 58 if bits == 64 else 46, stride)
                struct.pack_into(">I", changed, off + 4, 2)
                self.compare_vdso(changed)
            changed = raw.copy()
            changed[4] = 3
            self.compare_vdso(changed, status=1)

    def test_vdso_byte_names_usage_and_io_errors(self):
        raw = vdso_fixture()
        for name in (b"hyphen-name.second.c", b"image", b".hidden.c", b"raw.SO", b"vdso-\xff.c"):
            self.compare_vdso(raw, name=name)
        for args in ([], [b"one"], [b"missing-\xff", b"stripped", b"out.c"],
                     [b"raw", b"missing", b"out.c"], [b"raw", b"stripped", b"missing/out.c"]):
            self.compare_vdso(raw, status=1, args=args)
        self.compare_vdso(b"", status=1)
        self.compare_vdso(raw, b"", status=1)

    def test_vdso_checked_malformed_offsets_and_mutations(self):
        rng = random.Random(713)
        for bits in (32, 64):
            original = vdso_fixture(bits)
            variants = [original[:n] for n in (1, 4, 10, 32, 51, 52)]
            for offset, code in ((32 if bits == 64 else 28, "Q" if bits == 64 else "I"),
                                 (40 if bits == 64 else 32, "Q" if bits == 64 else "I")):
                data = original.copy()
                struct.pack_into(">" + code, data, offset, (1 << bits) - 1)
                variants.append(data)
            for _ in range(250):
                data = original.copy()
                for _ in range(rng.randrange(1, 8)):
                    data[rng.randrange(len(data))] = rng.randrange(256)
                variants.append(data)
            with tempfile.TemporaryDirectory(prefix="sparc-vdso-invalid-") as tmp:
                work = Path(tmp)
                (work / "stripped").write_bytes(bytes(128))
                for data in variants:
                    (work / "raw").write_bytes(data)
                    (work / "image.c").write_bytes(b"previous output")
                    run = subprocess.run([self.tools["vdso2c"][1], "raw", "stripped", "image.c"],
                                         cwd=work, capture_output=True, timeout=3)
                    self.assertIn(run.returncode, (0, 1), run.stderr)
                    self.assertNotIn(b"panicked", run.stderr)
                    if run.returncode:
                        self.assertEqual((work / "image.c").read_bytes(), b"previous output")

    def test_vdso_write_failure_and_input_alias(self):
        with tempfile.TemporaryDirectory(prefix="sparc-vdso-write-") as tmp:
            work = Path(tmp)
            (work / "raw").write_bytes(vdso_fixture())
            (work / "stripped.so").write_bytes(bytes(range(128)))
            if Path("/dev/full").exists():
                run = subprocess.run([self.tools["vdso2c"][1], "raw", "stripped.so", "/dev/full"],
                                     cwd=work, capture_output=True)
                self.assertEqual(run.returncode, 1)
                self.assertIn(b"No space left on device", run.stderr)
            run = subprocess.run([self.tools["vdso2c"][1], "raw", "stripped.so", "stripped.so"],
                                 cwd=work, capture_output=True)
            self.assertEqual(run.returncode, 0, run.stderr)
            self.assertEqual((work / "stripped.so").read_bytes(), bytes(range(128)))

    @unittest.skipUnless(shutil.which("clang") and shutil.which("ld.lld") and shutil.which("llvm-objcopy"),
                         "SPARC compiler/linker tools not installed")
    def test_real_sparc64_vdso_and_generated_c_roundtrip(self):
        with tempfile.TemporaryDirectory(prefix="sparc-vdso-linked-") as tmp:
            work = Path(tmp)
            (work / "input.s").write_text(
                ".global __vdso_clock_gettime\n.type __vdso_clock_gettime, #function\n"
                "__vdso_clock_gettime:\nretl\nnop\n")
            (work / "layout.lds").write_text(
                "PHDRS { text PT_LOAD FILEHDR PHDRS FLAGS(5); dynamic PT_DYNAMIC FLAGS(4); }\n"
                "SECTIONS { . = SIZEOF_HEADERS; .text : { *(.text) } :text "
                ".dynamic : { *(.dynamic) } :text :dynamic }\n")
            for command in (["clang", "--target=sparc64-linux-gnu", "-c", "input.s", "-o", "input.o"],
                            ["ld.lld", "-m", "elf64_sparc", "-shared", "-z", "max-page-size=8192",
                             "--hash-style=both", "--build-id=sha1", "-T", "layout.lds", "input.o", "-o", "raw"],
                            ["llvm-objcopy", "-S", "raw", "stripped"]):
                run = subprocess.run(command, cwd=work, capture_output=True)
                self.assertEqual(run.returncode, 0, run.stderr)
            raw, stripped = (work / "raw").read_bytes(), (work / "stripped").read_bytes()
            self.assertEqual(raw[4:6], b"\x02\x02")
            self.assertEqual(struct.unpack_from(">H", raw, 18)[0], 43)
            generated = self.compare_vdso(raw, stripped, b"linked.c")
            self.assertEqual(self.compare_vdso(raw, stripped, b"linked.so"), stripped)
            # Compile the emitted initializer against the actual SPARC image
            # definition and execute on the host to check padding and bytes.
            (work / "linux").mkdir()
            (work / "linux/cache.h").write_text(
                "#define __ro_after_init\n#define __aligned(x) __attribute__((aligned(x)))\n")
            (work / "linked.c").write_bytes(generated +
                b"#include <stdio.h>\nint main(void) { return fwrite(linked_builtin.data, "
                b"linked_builtin.size, 1, stdout) != 1; }\n")
            subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
                "-Wall", "-Werror", "-I.", "-I" + str(ROOT / "arch/sparc/include"),
                "linked.c", "-o", "roundtrip"], cwd=work, capture_output=True, check=True)
            run = subprocess.run([work / "roundtrip"], capture_output=True, check=True)
            self.assertEqual(run.stdout, stripped + bytes((-len(stripped)) % 8192))

    def test_piggyback_both_widths_alignment_sizes_and_retained_tail(self):
        rng = random.Random(924)
        for bits in ("32", "64", "anything"):
            for end in (0xf0007000, 0xf0007fdf, 0xf0007fe0, 0xf0007fe1, 0xf0008000):
                for length in (0, 1, 1023, 1024, 1025, 16385):
                    with self.subTest(bits=bits, end=end, length=length):
                        self.compare_piggy(aout_fixture(), rng.randbytes(length), bits, end=end)
            self.compare_piggy(aout_fixture(65536), bits=bits)
            self.compare_piggy(aout_fixture(), bits=bits, start=0xfffff000, end=0xffffffe0)

    def test_piggyback_branch_search_signed_char_and_scan_bounds(self):
        # Most host targets have signed char; query the reference compiler so
        # the inherited ld2 low-byte sign extension is tested portably.
        signed = subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-dM", "-E", "-"], input=b"", capture_output=True, check=True).stdout
        signed = b"__CHAR_UNSIGNED__" not in signed
        for branch in (120, 127, 128, 255, 256, 1024, 32767, 32768, 65535):
            low = branch & 255
            decoded = (0xff00 | low) if signed and low >= 128 else branch
            offset = decoded * 4 - 480
            for relative in (0, 4, 508):
                if offset + relative < 36:
                    continue  # Do not overwrite the a.out header or branch.
                self.compare_piggy(aout_fixture(max(4096, offset + 1024), branch, offset + relative))
        for relative in (1, 2, 3, 512):
            self.compare_piggy(aout_fixture(4096, 256, 544 + relative), status=1)
        self.compare_piggy(aout_fixture(4096, 119, None), status=1)

    def test_piggyback_map_columns_duplicates_and_fgets_chunks(self):
        normal = b"f0004000 T _start\nf0007000 T _end\n"
        for symbols in (normal, b"00000000f0004000 T _start\n00000000f0007000 T _end\n",
                        normal + b"f0008000 T _end\n", b"x" * 1023 + normal,
                        b"f0004000 \x00 _start\nf0007000 T _end\n",
                        normal + b"f0008000\n"):
            self.compare_piggy(aout_fixture(), symbols=symbols)
        for symbols in (b"", normal[:-1], normal.replace(b" T ", b"  T "),
                        normal.replace(b"_end", b"_end_extra"), normal + b"0\n",
                        b"00000000 T _start\nf0007000 T _end\n"):
            self.compare_piggy(aout_fixture(), symbols=symbols, status=1)

    def test_piggyback_invalid_headers_short_reads_and_raw_paths(self):
        for length in (0, 1, 511, 512, 1023):
            self.compare_piggy(aout_fixture()[:length], status=1)
        self.compare_piggy(bytes(4096), status=1)
        self.compare_piggy(aout_fixture(branch=256, marker=None), status=1)
        self.compare_piggy(aout_fixture(), names=[b"image-\xff", b"map-\xfe", b"initrd-\xfd"])

    def test_piggyback_usage_and_error_precedence(self):
        with tempfile.TemporaryDirectory(prefix="sparc-piggy-errors-") as tmp:
            work = Path(tmp)
            (work / "initrd").write_bytes(b"x")
            (work / "map").write_bytes(b"f0004000 T _start\nf0007000 T _end\n")
            for args in ([], [b"64"], [b"32", b"image", b"map", b"missing-\xff"],
                         [b"32", b"image", b"missing", b"initrd"],
                         [b"32", b"missing", b"map", b"initrd"],
                         [b"32", b"missing", b".", b"initrd"]):
                results = [subprocess.run([tool, *args], cwd=work, capture_output=True)
                           for tool in self.tools["piggyback"]]
                self.assertEqual([(r.returncode, r.stdout, r.stderr) for r in results],
                                 [(results[0].returncode, results[0].stdout, results[0].stderr)] * 2)
                self.assertEqual(results[0].returncode, 1)

    def test_piggyback_generated_images(self):
        rng = random.Random(7132)
        for _ in range(100):
            start = rng.choice((0x4000, 0xf0004000, 0x80004000))
            end = start + rng.randrange(1, 0x10000)
            self.compare_piggy(aout_fixture(rng.randrange(1024, 0x10000)),
                              rng.randbytes(rng.randrange(4096)), rng.choice(("32", "64")), start, end)

    def test_piggyback_aliases_and_ramdisk_read_errors_are_safe(self):
        with tempfile.TemporaryDirectory(prefix="sparc-piggy-alias-") as tmp:
            work = Path(tmp)
            image = bytes(aout_fixture())
            (work / "image").write_bytes(image)
            (work / "map").write_bytes(b"f0004000 T _start\nf0007000 T _end\n")
            (work / "symlink").symlink_to("image")
            os.link(work / "image", work / "hardlink")
            for name in ("image", "symlink", "hardlink"):
                run = subprocess.run([self.tools["piggyback"][1], "32", "image", "map", name],
                                     cwd=work, capture_output=True, timeout=3)
                self.assertEqual(run.returncode, 1)
                self.assertIn(b"aliases kernel image", run.stderr)
                self.assertEqual((work / "image").read_bytes(), image)
            # The C read result is unsigned and turns -1 into a huge write.
            # Rust reports the actual source read error after the header edit.
            run = subprocess.run([self.tools["piggyback"][1], "32", "image", "map", "."],
                                 cwd=work, capture_output=True, timeout=3)
            self.assertEqual(run.returncode, 1)
            self.assertIn(b"Is a directory", run.stderr)
            self.assertNotEqual((work / "image").read_bytes(), image)


if __name__ == "__main__":
    unittest.main()
