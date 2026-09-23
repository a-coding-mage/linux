#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Certificate insertion parity and malformed-image write protection."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
WIDE = struct.calcsize("P") == 8
ORDER = "<" if sys.byteorder == "little" else ">"
BASE = 0xffffffff81000000 if WIDE else 0xc1000000
NAMES = ("system_extra_cert", "system_extra_cert_used", "system_certificate_list_size")


def fixture(reserve=64, previous=b"", list_size=100, symbols=True, extended=False,
            missing=(), undefined=(), used=None, overrides=None):
    header_size, section_size, symbol_size = (64, 64, 24) if WIDE else (52, 40, 16)
    data = bytearray(header_size)
    offsets = [len(data), len(data) + reserve, len(data) + reserve + 8]
    data.extend(previous.ljust(reserve, b"\xa5"))
    data.extend(struct.pack(ORDER + "i", len(previous) if used is None else used))
    data.extend(bytes(4))
    data.extend(struct.pack(ORDER + ("Q" if WIDE else "I"), list_size))
    data_size = len(data) - header_size
    strings = bytearray(b"\0")
    table = bytearray(symbol_size)
    for index, name in enumerate(NAMES):
        if name in missing:
            continue
        name_offset = len(strings)
        strings.extend(name.encode() + b"\0")
        address, size, section = BASE + offsets[index] - header_size, (reserve, 4, 8 if WIDE else 4)[index], 1
        if name in undefined:
            section = 0
        address, size, section = (overrides or {}).get(name, (address, size, section))
        table.extend(struct.pack(ORDER + "IBBHQQ", name_offset, 0x11, 0, section, address, size) if WIDE else
                     struct.pack(ORDER + "IIIBBH", name_offset, address, size, 0x11, 0, section))
    table_offset = len(data)
    data.extend(table)
    strings_offset = len(data)
    data.extend(strings)
    section_offset = len(data)
    sections = [(0, 0, 0, 4 if extended else 0, 0, 0),
                (1, BASE, header_size, data_size, 0, 0),
                (2 if symbols else 1, 0, table_offset, len(table), 3, symbol_size),
                (3, 0, strings_offset, len(strings), 0, 0)]
    for kind, address, offset, size, link, entry_size in sections:
        fields = (0, kind, 3 if address else 0, address, offset, size, link, 0, 1, entry_size)
        data.extend(struct.pack(ORDER + ("IIQQQQIIQQ" if WIDE else "IIIIIIIIII"), *fields))
    ident = b"\x7fELF" + bytes((2 if WIDE else 1, 1 if sys.byteorder == "little" else 2, 1)) + bytes(9)
    fields = (ident, 2, 62 if WIDE else 3, 1, 0, 0, section_offset, 0, header_size, 0, 0,
              section_size, 0 if extended else 4, 0)
    data[:header_size] = struct.pack(ORDER + ("16sHHIQQQIHHHHHH" if WIDE else "16sHHIIIIIHHHHHH"), *fields)
    map_data = b"".join(f"{BASE + offset - header_size:016x} D {name}\n".encode()
                        for name, offset in zip(NAMES, offsets))
    return bytes(data), offsets, map_data


class InsertSysCertTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="insert-cert-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        work = Path(cls.temporary.name)
        cls.c, cls.rust = work / "insert-cert-c", work / "insert-cert-rust"
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        subprocess.run(cls.cc + ["-O2", str(ROOT / "scripts/insert-sys-cert.c"), "-o", str(cls.c)],
                       check=True, capture_output=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
            str(ROOT / "scripts/insert-sys-cert.rs"), "-o", str(cls.rust)], check=True, capture_output=True)

    def compare(self, image, cert, map_data=None, status=0, extra=(), name="certificate"):
        with tempfile.TemporaryDirectory(prefix="insert-cert-case-") as temporary:
            work = Path(temporary)
            (work / name).write_bytes(cert)
            if map_data is not None:
                (work / "System.map").write_bytes(map_data)
            results = []
            for tool in (self.c, self.rust):
                (work / "vmlinux").write_bytes(image)
                arguments = [str(tool), "-b", "vmlinux", "-c", name, *extra]
                if map_data is not None:
                    arguments += ["-s", "System.map"]
                result = subprocess.run(arguments, cwd=work, capture_output=True, timeout=10)
                results.append((result.returncode, result.stdout.replace(os.fsencode(tool), b"TOOL"),
                                result.stderr.replace(os.fsencode(tool), b"TOOL"), (work / "vmlinux").read_bytes()))
            self.assertEqual(results[0][0], status, results[0][:3])
            self.assertEqual(results[0], results[1])
            return results[1][3]

    def test_insert_replace_clear_and_exact_capacity(self):
        rng = random.Random(7183)
        for reserve in (8, 64, 4096):
            for length in (0, 1, reserve // 2, reserve):
                for previous in (b"", b"old-cert"):
                    for extended in (False, True):
                        with self.subTest(reserve=reserve, length=length, previous=previous, extended=extended):
                            image, offsets, _ = fixture(reserve, previous, extended=extended)
                            cert = rng.randbytes(length)
                            output = self.compare(image, cert)
                            if not cert and not previous:  # C leaves unused reserve untouched on a no-op.
                                self.assertEqual(output, image)
                            else:
                                self.assertEqual(output[offsets[0]:offsets[1]], cert.ljust(reserve, b"\0"))
                            self.assertEqual(struct.unpack_from(ORDER + "i", output, offsets[1])[0], length)
                            self.assertEqual(struct.unpack_from(ORDER + ("Q" if WIDE else "I"), output, offsets[2])[0],
                                             100 + length - len(previous))

    def test_system_map_and_ignored_map(self):
        for extended in (False, True):
            for symbols in (False, True):
                image, _, map_data = fixture(symbols=symbols, extended=extended)
                self.compare(image, b"certificate\0\xff", map_data)
                if symbols:
                    self.compare(image, b"certificate", extra=("-s", "missing-map"))
                else:
                    self.compare(image, b"certificate", status=1)
                    self.compare(image, b"certificate", map_data=b"00 D unrelated\n", status=1)
                    self.compare(image, b"certificate", map_data=b"a" * 100, status=1)
                    self.compare(image, b"certificate", map_data=b"unterminated", status=1)

    def test_identical_noop_preserves_inode_mode_and_time(self):
        cert = b"a\0b\xffz"
        image, _, _ = fixture(previous=cert)
        for tool in (self.c, self.rust):
            with tempfile.TemporaryDirectory(prefix="insert-cert-noop-") as temporary:
                work = Path(temporary)
                target = work / "vmlinux"
                target.write_bytes(image)
                target.chmod(0o640)
                os.utime(target, ns=(1_000_000_000, 1_000_000_000))
                os.link(target, work / "hardlink")
                before = target.stat()
                (work / "certificate").write_bytes(cert)
                result = subprocess.run([str(tool), "-b", "vmlinux", "-c", "certificate"], cwd=work, capture_output=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertIn(b"Certificate was already inserted.", result.stdout)
                self.assertEqual(target.read_bytes(), image)
                after = target.stat()
                self.assertEqual((before.st_ino, before.st_mode, before.st_mtime_ns),
                                 (after.st_ino, after.st_mode, after.st_mtime_ns))

    def test_missing_symbols_undefined_and_oversize(self):
        self.compare(fixture()[0], bytes(65), status=1)
        for name in NAMES:
            self.compare(fixture(missing=(name,))[0], b"cert", status=1)
            self.compare(fixture(undefined=(name,))[0], b"cert", status=1)
        self.compare(fixture(missing=NAMES)[0], b"cert", status=1)

    def test_replacement_preserves_hardlinks_and_permissions(self):
        with tempfile.TemporaryDirectory(prefix="insert-cert-inplace-") as temporary:
            work = Path(temporary)
            image, offsets, _ = fixture()
            (work / "vmlinux").write_bytes(image)
            (work / "vmlinux").chmod(0o640)
            os.link(work / "vmlinux", work / "linked")
            (work / "symlink").symlink_to("vmlinux")
            (work / "certificate").write_bytes(b"cert")
            before = (work / "vmlinux").stat()
            result = subprocess.run([str(self.rust), "-b", "symlink", "-c", "certificate"], cwd=work, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            after = (work / "vmlinux").stat()
            self.assertEqual((before.st_ino, before.st_mode, before.st_nlink),
                             (after.st_ino, after.st_mode, after.st_nlink))
            self.assertEqual((work / "linked").read_bytes()[offsets[0]:offsets[0] + 4], b"cert")

    def test_posix_operand_terminates_options(self):
        environment = dict(os.environ, POSIXLY_CORRECT="")
        results = []
        for tool in (self.c, self.rust):
            result = subprocess.run([str(tool), "operand", "-b", "missing", "-c", "missing"],
                                    env=environment, capture_output=True)
            results.append((result.returncode, result.stdout.replace(os.fsencode(tool), b"TOOL"), result.stderr))
        self.assertEqual(results[0], results[1])

    def test_argument_forms_and_non_utf8_paths(self):
        image = fixture()[0]
        for name in ("certificate with spaces", "日本語.der", os.fsdecode(b"cert-\xff.der")):
            self.compare(image, b"DER\0CERT", name=name)
        self.compare(image, b"cert", extra=("ignored-operand", "-b" + "vmlinux", "-c" + "certificate"))
        self.compare(image, b"cert", extra=("-qz",))
        self.compare(image, b"cert", extra=("-s",))
        for arguments in ([], ["-b"], ["-b", "missing"], ["-c", "missing"], ["-q"], ["--", "-b", "x"]):
            results = []
            for tool in (self.c, self.rust):
                result = subprocess.run([str(tool), *arguments], capture_output=True)
                results.append((result.returncode, result.stdout.replace(os.fsencode(tool), b"TOOL"),
                                result.stderr.replace(os.fsencode(tool), b"TOOL")))
            self.assertEqual(results[0], results[1])

    def test_header_diagnostics(self):
        image = fixture()[0]
        variants = [bytes(16), b"bad!" + image[4:]]
        for offset, value in ((4, 1 if WIDE else 2), (5, 2 if sys.byteorder == "little" else 1)):
            changed = bytearray(image)
            changed[offset] = value
            variants.append(changed)
        changed = bytearray(image)
        struct.pack_into(ORDER + ("Q" if WIDE else "I"), changed, 40 if WIDE else 32, len(image) + 1)
        variants.append(changed)
        for variant in variants:
            self.compare(variant, b"cert", status=1)

    def test_binary_nul_replacement_and_exact_map_symbols(self):
        # Intentional fixes for two C bugs: strncmp on binary certificates and
        # substring symbol matching in an unsorted System.map.
        image, offsets, map_data = fixture(previous=b"prefix\0old")
        cert = b"prefix\0new"
        for symbols in (True, False):
            if not symbols:
                image, offsets, map_data = fixture(previous=b"prefix\0old", symbols=False)
            with tempfile.TemporaryDirectory(prefix="insert-cert-binary-") as temporary:
                work = Path(temporary)
                (work / "vmlinux").write_bytes(image)
                (work / "certificate").write_bytes(cert)
                (work / "System.map").write_bytes(b"".join(reversed(map_data.splitlines(keepends=True))))
                result = subprocess.run([str(self.rust), "-b", "vmlinux", "-c", "certificate", "-s", "System.map"],
                                        cwd=work, capture_output=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertIn(b"Replacing previously", result.stdout)
                self.assertEqual((work / "vmlinux").read_bytes()[offsets[0]:offsets[0] + len(cert)], cert)

    def test_malformed_images_never_modify_input(self):
        image = fixture()[0]
        variants = [image[:length] for length in range(len(image))]
        variants += [fixture(used=-1)[0], fixture(used=65)[0], fixture(previous=b"old", list_size=1)[0],
                     fixture(list_size=(1 << (64 if WIDE else 32)) - 1)[0],
                     fixture(overrides={NAMES[0]: (BASE, 100000, 1)})[0],
                     fixture(overrides={NAMES[1]: (BASE + 2, 4, 1)})[0],
                     fixture(overrides={NAMES[2]: (BASE + 64, 8 if WIDE else 4, 1)})[0],
                     fixture(overrides={NAMES[0]: (BASE, 64, 0xfff1)})[0]]
        with tempfile.TemporaryDirectory(prefix="insert-cert-invalid-") as temporary:
            work = Path(temporary)
            (work / "certificate").write_bytes(b"cert")
            for index, variant in enumerate(variants):
                with self.subTest(index=index):
                    (work / "vmlinux").write_bytes(variant)
                    result = subprocess.run([str(self.rust), "-b", "vmlinux", "-c", "certificate"], cwd=work,
                                            capture_output=True, timeout=10)
                    self.assertEqual(result.returncode, 1, result.stderr)
                    self.assertNotIn(b"panicked", result.stderr)
                    self.assertEqual((work / "vmlinux").read_bytes(), variant)

    def test_real_linked_elf(self):
        with tempfile.TemporaryDirectory(prefix="insert-cert-linked-") as temporary:
            work = Path(temporary)
            assembly = ".data\n.global system_extra_cert, system_extra_cert_used, system_certificate_list_size\n"
            assembly += "system_extra_cert:\n.space 64\n.size system_extra_cert, 64\n"
            assembly += "system_extra_cert_used:\n.long 0\n.size system_extra_cert_used, 4\n.balign 8\n"
            assembly += "system_certificate_list_size:\n" + (".quad 100\n" if WIDE else ".long 100\n")
            assembly += f".size system_certificate_list_size, {8 if WIDE else 4}\n"
            (work / "cert.S").write_text(assembly)
            subprocess.run(self.cc + ["-nostdlib", "-no-pie", "-Wl,-e,0", str(work / "cert.S"), "-o", str(work / "vmlinux")],
                           check=True, capture_output=True)
            self.compare((work / "vmlinux").read_bytes(), b"\x30\x82\0\xffDER certificate")

    def test_optional_built_kernel_and_stripped_map(self):
        directory = os.environ.get("INSERT_CERT_KERNEL_BUILD")
        if not directory:
            self.skipTest("set INSERT_CERT_KERNEL_BUILD to a SYSTEM_EXTRA_CERTIFICATE=y build")
        build = Path(directory)
        image = (build / "vmlinux").read_bytes()
        for certificate in (b"\x30\x82\0\x10temporary test certificate", bytes(range(256))):
            self.compare(image, certificate)
        with tempfile.TemporaryDirectory(prefix="insert-cert-real-kernel-") as temporary:
            stripped = Path(temporary) / "stripped"
            subprocess.run(shlex.split(os.environ.get("OBJCOPY", "objcopy")) + [
                "--strip-all", str(build / "vmlinux"), str(stripped)], check=True, capture_output=True)
            self.compare(stripped.read_bytes(), b"temporary test certificate", map_data=(build / "System.map").read_bytes())


if __name__ == "__main__":
    unittest.main()
