#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""Device-tree binary, assembly, source and filesystem I/O comparisons."""

import os
from pathlib import Path
import random
import shlex
import struct
import subprocess
import tempfile
import unittest

from dtc_test_support import ROOT, build_c, build_rust

SOURCE = b'''/dts-v1/;
/memreserve/ 0x123456789abcdef0 0x9988776655443322;
/memreserve/ 0 1;
/ {
    empty;
    text = "alpha", "beta", "";
    bytes = [00 12 80 ff 32];
    words = <0 1 0xffffffff>;
    half = /bits/ 16 <1 0xffff>;
    wide = /bits/ 64 <0x123456789abcdef0>;
    prefix-name = "value";
    name = "";
    child@0 { name = "child"; suffix-name = [33]; deeper { flag; }; };
};
'''

TYPED = br'''/dts-v1/;
reserve_label: /memreserve/ 0x1234 0x8000;
/ {
    prop_label: bytes = [12 ff 00];
    half = /bits/ 16 <0x12 0xffff>;
    words = <word_label: 1 0xabcd>;
    wide = /bits/ 64 <0xffffffffffffffff>;
    string = "a\0b\x80\xff\n\"\\";
    child_label: child@0 {};
};
'''


class DtcEmitTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="dtc-emit-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.c = build_c(cls.work)
        cls.rust = build_rust(cls.work, ROOT / "scripts/tests/dtc_emit_harness.rs")
        cls.rust = cls.rust.rename(cls.work / "dtc-emit-rust")
        cls.compiler = build_rust(cls.work)

    def reference(self, source=SOURCE, version=17):
        result = subprocess.run([str(self.c), "-q", "-q", "-I", "dts", "-O", "dtb", "-V", str(version), "-"],
                                input=source, capture_output=True, check=True)
        return result.stdout

    def compare(self, blob, output="dtb", arguments=(), status=0, input_format="dtb", path="-"):
        results = []
        for binary in (self.c, self.rust):
            command = [str(binary), "-I", input_format, "-O", output, *arguments, path]
            if binary == self.c:
                # This harness isolates I/O; the complete compiler owns checks
                # that otherwise remove valid legacy `name` properties.
                command[1:1] = ["-q", "-q", "-Eno-name_properties"]
            result = subprocess.run(command, input=blob, capture_output=True, timeout=10)
            results.append((result.returncode, result.stdout, result.stderr))
        self.assertEqual(results[0][0], status, results[0][2])
        self.assertEqual(results[0], results[1])
        return results[1][1]

    def test_all_input_and_output_versions(self):
        for input_version in (1, 2, 3, 16, 17):
            blob = self.reference(version=input_version)
            for output_version in (1, 2, 3, 16, 17):
                for output in ("dtb", "asm", "dts"):
                    with self.subTest(input_version=input_version, output_version=output_version, output=output):
                        self.compare(blob, output, ("-V", str(output_version)))

    def test_reserves_padding_alignment_boot_and_sort(self):
        blob = self.reference()
        for arguments in (("-R", "0"), ("-R", "5"), ("-S", "4096"), ("-p", "37"),
                          ("-a", "16"), ("-a", "4096"), ("-p", "7", "-a", "64"),
                          ("-S", "2000", "-a", "256"), ("-b", "0xabcdef"), ("-s",),
                          ("-R", "11", "-b", "123", "-p", "511", "-a", "1024")):
            for output in ("dtb", "asm", "dts"):
                with self.subTest(arguments=arguments, output=output):
                    self.compare(blob, output, arguments)

    def test_annotations_without_source_and_blob_overlay_flag(self):
        for source in (SOURCE, b"/dts-v1/; / { __fixups__ {}; };", b"/dts-v1/; / { __local_fixups__ {}; };"):
            blob = self.reference(source)
            self.compare(blob, "dts")
        # The C CLI refuses -T for blob input; call source emitter annotations
        # directly here and verify the empty source-position representation.
        result = subprocess.run([str(self.rust), "-O", "dts", "-T", "-T"], input=self.reference(), capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(b"/* <no-file>:<no-line> */", result.stdout)

    def test_typed_values_labels_and_non_ascii_strings(self):
        for output in ("dtb", "asm", "dts"):
            for version in (1, 2, 3, 16, 17):
                reference = subprocess.run([str(self.c), "-q", "-q", "-I", "dts", "-O", output, "-V", str(version), "-"],
                                           input=TYPED, capture_output=True)
                result = subprocess.run([str(self.rust), "--typed-fixture", "-O", output, "-V", str(version)], capture_output=True)
                with self.subTest(output=output, version=version):
                    self.assertEqual(reference.returncode, 0, reference.stderr)
                    self.assertEqual((reference.returncode, reference.stdout, reference.stderr),
                                     (result.returncode, result.stdout, result.stderr))

    def test_generated_assembly_compiles_to_blob(self):
        blob = self.reference()
        for version in (1, 2, 3, 16, 17):
            arguments = ("-V", str(version), "-p", "64", "-a", "16")
            assembly = self.compare(blob, "asm", arguments)
            expected = self.compare(blob, "dtb", arguments)
            with tempfile.TemporaryDirectory(prefix="dtc-asm-") as temporary:
                work = Path(temporary)
                (work / "tree.S").write_bytes(assembly)
                subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + ["-c", str(work / "tree.S"), "-o", str(work / "tree.o")],
                               check=True, capture_output=True)
                subprocess.run(["objcopy", "-O", "binary", str(work / "tree.o"), str(work / "tree.bin")], check=True, capture_output=True)
                self.assertEqual((work / "tree.bin").read_bytes(), expected)

    def test_byte_values_and_random_property_shapes(self):
        rng = random.Random(7541)
        for length in (0, 1, 2, 3, 4, 5, 7, 8, 13, 31, 256, 4096):
            for data in (rng.randbytes(length), b"A" * max(0, length - 1) + (b"\0" if length else b"")):
                source = b"/dts-v1/; / { data = [" + data.hex(" ").encode() + b"]; };"
                blob = self.reference(source)
                for output in ("dtb", "dts", "asm"):
                    self.compare(blob, output)

    def test_filesystem_order_symlinks_and_boot_cpu(self):
        with tempfile.TemporaryDirectory(prefix="dtc-fstree-") as temporary:
            work = Path(temporary)
            (work / "text").write_bytes(b"string\0")
            (work / "empty").write_bytes(b"")
            (work / "bytes").write_bytes(bytes(range(256)))
            child = work / "cpus" / "cpu@123"
            child.mkdir(parents=True)
            (child / "reg").write_bytes(struct.pack(">I", 123))
            (work / "link").symlink_to("text")
            os.mkfifo(work / "ignored-fifo")
            for output in ("dtb", "dts", "asm"):
                result = self.compare(None, output, ("-s",), input_format="fs", path=str(work))
                if output == "dtb":
                    self.assertEqual(struct.unpack_from(">I", result, 28)[0], 123)
            (work / "cycle").symlink_to(".")
            result = subprocess.run([str(self.rust), "-I", "fs", str(work)], capture_output=True, timeout=5)
            self.assertEqual(result.returncode, 1)
            self.assertIn(b"Directory cycle", result.stderr)

    def test_file_search_paths_and_binary_names(self):
        with tempfile.TemporaryDirectory(prefix="dtc-paths-") as temporary:
            work = Path(temporary)
            name = os.fsdecode(b"blob-\xff.dtb")
            blob = self.reference()
            (work / name).write_bytes(blob)
            self.compare(None, "dts", ("-i", str(work)), path=name)
            self.compare(None, "dtb", path=str(work / name))
            self.compare(None, path=str(work / "missing"), status=1)

    def test_malformed_blob_bounds_and_tags(self):
        blob = self.reference()
        for length in (0, 1, 3, 4, 7, 8, 27, len(blob) - 1):
            self.compare(blob[:length], status=1)
        variants = []
        for offset, value in ((0, 123), (4, 8), (8, len(blob)), (12, len(blob) + 1),
                              (16, len(blob)), (32, 0xffffffff), (36, 0xffffffff)):
            changed = bytearray(blob)
            struct.pack_into(">I", changed, offset, value)
            variants.append(changed)
        structure = struct.unpack_from(">I", blob, 8)[0]
        for offset, tag in ((structure, 9), (structure + 8, 9), (structure + 8, 0xfefefefe)):
            changed = bytearray(blob)
            struct.pack_into(">I", changed, offset, tag)
            variants.append(changed)
        for index, variant in enumerate(variants):
            with self.subTest(index=index):
                self.compare(variant, status=1)

    def test_legacy_nops_and_bad_string_offsets(self):
        for version in (1, 2, 3, 16, 17):
            blob = bytearray(self.reference(b"/dts-v1/; / { value = <1>; };", version))
            structure = struct.unpack_from(">I", blob, 8)[0]
            nop = bytearray(blob[:structure + 8] + struct.pack(">I", 4) + blob[structure + 8:])
            for field in (4, 12):
                struct.pack_into(">I", nop, field, struct.unpack_from(">I", nop, field)[0] + 4)
            if version >= 17:
                struct.pack_into(">I", nop, 36, struct.unpack_from(">I", nop, 36)[0] + 4)
            self.compare(nop, "dts")
            struct.pack_into(">I", blob, structure + 16, 0xfffffffe)
            self.compare(blob, status=1)

    def test_minimum_size_warning_and_quiet(self):
        blob = self.reference(b"/dts-v1/; / {};")
        for quiet in ([], ["-q"], ["-q", "-q"]):
            results = []
            for binary in (self.c, self.rust):
                result = subprocess.run([str(binary), "-I", "dtb", "-O", "dtb", "-S", "1", *quiet, "-"],
                                        input=blob, capture_output=True)
                results.append((result.returncode, result.stdout, result.stderr))
            self.assertEqual(results[0], results[1])
            self.assertEqual(b"Warning: blob size" in results[1][2], not quiet)

    def test_complete_compiler_reads_and_emits_all_versions(self):
        for source in (SOURCE, TYPED):
            for input_version in (1, 2, 3, 16, 17):
                blob = self.reference(source, input_version)
                for output_version in (1, 2, 3, 16, 17):
                    for output in ("dtb", "dts", "asm"):
                        arguments = ["-q", "-q", "-I", "dtb", "-O", output, "-V", str(output_version), "-"]
                        results = []
                        for binary in (self.c, self.compiler):
                            result = subprocess.run([str(binary), *arguments], input=blob, capture_output=True, timeout=10)
                            results.append((result.returncode, result.stdout, result.stderr))
                        with self.subTest(input_version=input_version, output_version=output_version, output=output):
                            self.assertEqual(results[0][0], 0, results[0][2])
                            self.assertEqual(results[0], results[1])

    def test_deep_tree_uses_owned_iteration(self):
        depth = 3000
        structure = struct.pack(">II", 1, 0)
        structure += (struct.pack(">I", 1) + b"n\0\0\0") * depth
        structure += struct.pack(">I", 2) * (depth + 1) + struct.pack(">I", 9)
        blob = struct.pack(">10I", 0xd00dfeed, 56 + len(structure), 56, 56 + len(structure),
                           40, 17, 16, 0, 0, len(structure)) + bytes(16) + structure
        for output in ("dtb", "dts", "asm"):
            self.compare(blob, output)

    def test_blob_stream_does_not_require_eof(self):
        blob = self.reference()
        for binary in (self.c, self.rust, self.compiler):
            process = subprocess.Popen([str(binary), "-q", "-q", "-I", "dtb", "-O", "dtb", "-"],
                                       stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            try:
                process.stdin.write(blob)
                process.stdin.flush()
                # Keeping stdin open catches eager read-to-EOF behavior.
                self.assertEqual(process.wait(timeout=5), 0)
            finally:
                if process.poll() is None:
                    process.kill()
                    process.wait()
                process.stdin.close()
                process.stdout.close()
                process.stderr.close()
        with tempfile.TemporaryDirectory(prefix="dtc-read-error-") as directory:
            self.compare(None, path=directory, status=1)

    def test_random_binary_mutations_fail_without_panics(self):
        blob = self.reference()
        rng = random.Random(561722)
        for case in range(400):
            changed = bytearray(blob)
            for _ in range(rng.randrange(1, 9)):
                changed[rng.randrange(len(changed))] = rng.randrange(256)
            if case % 4 == 0:
                changed = changed[:rng.randrange(len(changed))]
            result = subprocess.run([str(self.rust), "-O", "dtb"], input=changed,
                                    capture_output=True, timeout=5)
            with self.subTest(case=case):
                self.assertIn(result.returncode, (0, 1), result.stderr)
                self.assertNotIn(b"panicked", result.stderr)
                self.assertLess(len(result.stdout), 1_000_000)

    def test_trailing_bytes_are_not_part_of_the_blob(self):
        blob = self.reference()
        for trailing in (bytes(1000), b"second stream segment\xff", self.reference()):
            self.assertEqual(self.compare(blob + trailing), self.compare(blob))


if __name__ == "__main__":
    unittest.main()
