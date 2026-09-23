# SPDX-License-Identifier: GPL-2.0-or-later
"""Byte-for-byte differential tests for the safe flattened-tree library."""

import os
from pathlib import Path
import random
import struct
import subprocess
import tempfile
import unittest

from libfdt_test_support import cached_libfdt_tools, cmd


def dtb(tree, version=17, reserve=(), capacity=0):
    """Independent encoder, also exercising versions predating libfdt writers."""
    strings = bytearray()
    structure = bytearray()
    def word(n):
        structure.extend(struct.pack(">I", n))
    def padding(alignment):
        structure.extend(b"\0" * (-len(structure) % alignment))
    def emit(node, path):
        name, properties, children = node
        full = path + (b"/" if path != b"/" else b"") + name if name else b"/"
        word(1)
        structure.extend((full if version < 16 else name) + b"\0")
        padding(4)
        for key, value in properties:
            needle = key + b"\0"
            at = strings.find(needle)
            if at < 0:
                at = len(strings)
                strings.extend(needle)
            word(3)
            word(len(value))
            word(at)
            if version < 16 and len(value) >= 8:
                padding(8)
            structure.extend(value)
            padding(4)
        for child in children:
            emit(child, full)
        word(2)
    emit(tree, b"")
    word(9)
    reservations = b"".join(struct.pack(">QQ", *item) for item in reserve) + bytes(16)
    structoff = 40 + len(reservations)
    stringoff = structoff + len(structure)
    total = max(capacity, stringoff + len(strings))
    header = struct.pack(">10I", 0xd00dfeed, total, structoff, stringoff, 40,
                         version, min(version, 16), 0x123, len(strings), len(structure))
    return (header + reservations + structure + strings).ljust(total, b"\0")


class LibfdtTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        try:
            cls.tools = cached_libfdt_tools()
        except subprocess.CalledProcessError as exc:
            raise RuntimeError(exc.stderr.decode()) from exc

    def compare(self, commands, blob=b"", capacity=4096):
        with tempfile.TemporaryDirectory(prefix="libfdt-case-") as directory:
            work = Path(directory)
            (work / "in").write_bytes(blob)
            results = []
            for index, tool in enumerate(self.tools):
                out = work / str(index)
                result = subprocess.run([tool, work / "in", out, str(capacity)],
                                        input=("\n".join(commands) + "\n").encode(),
                                        capture_output=True, timeout=10)
                self.assertEqual(result.returncode, 0, (commands, result.stderr))
                results.append((result.stdout, result.stderr, out.read_bytes(),
                                out.with_suffix(".overlay").read_bytes()
                                if out.with_suffix(".overlay").exists() else None))
            self.assertEqual(results[0][:2], results[1][:2], commands)
            if results[0][2] != results[1][2]:
                diffs = [i for i, (a, b) in enumerate(zip(results[0][2], results[1][2])) if a != b]
                self.fail(f"blob differs at {diffs[:30]} for {commands}")
            self.assertEqual(results[0][3], results[1][3], commands)
            return results[0][2], results[0][0]

    def test_sequential_construction_and_states(self):
        for flags in (0, 1, 2, 0xffffffff):
            for cap in (40, 48, 64, 80, 128, 512, 4096):
                with self.subTest(flags=flags, capacity=cap):
                    self.compare([
                        cmd("create", flags), cmd("reserve", 0x123456789abcdef, 0x987654321),
                        "finishreserve", cmd("begin", b""),
                        cmd("property", b"compatible", b"a\0b\0"),
                        cmd("property", b"patible", b"x"), cmd("begin", b"child"),
                        cmd("property", b"compatible", bytes(range(17))), "end", "end", "finish",
                    ], capacity=cap)
        self.compare(["empty", "create 0", "begin 61", "end", "finish", "finishreserve",
                      "finishreserve", "begin -", "end", "finish", "finish", "end"])

    def test_mutations_preserve_complete_buffer(self):
        self.compare(["empty", cmd("set", "/", "first", b"abcdefg"),
                      cmd("set", "/", "second", b"\x00"), cmd("add", "/", "a@0"),
                      cmd("set", "/a", "compatible", b"board,one\0board,two\0"),
                      cmd("add", "/a", "child"), cmd("add", "/", "b"),
                      cmd("rename", "/b", "longer-name"), cmd("set", "/", "first", b"x"),
                      cmd("append", "/", "second", bytes(range(10))),
                      cmd("partial", "/", "second", 3, b"xy"),
                      cmd("inplace", "/", "first", b"z"), cmd("del", "/", "second"),
                      "addreserve 12 34", "addreserve 56 78", "delreserve 0", "walk", "pack"])

    def test_read_queries_old_versions_and_binary_names(self):
        tree = (b"", [(b"compatible", b"a\0bb\0"), (b"phandle", struct.pack(">I", 7)),
                      (b"binary\xff", bytes(range(25)))], [
            (b"aliases", [(b"serial", b"/bus/device@10\0")], []),
            (b"bus", [(b"#address-cells", struct.pack(">I", 1))], [
                (b"device@10", [(b"linux,phandle", struct.pack(">I", 42)),
                                 (b"list", b"zero\0\0two\0")], [])])])
        for version in (2, 3, 15, 16, 17, 18):
            blob = dtb(tree, version, [(0x1234567890, 4096)])
            commands = ["check", "walk", "max", "gen", "rsv 0", "rsv 1", "rsv 100",
                        cmd("path", "/bus/device"), cmd("path", "serial"),
                        cmd("path", "serial/"), cmd("path", "////bus//device@10///"),
                        "findphandle 42", "findphandle 0", "findphandle 4294967295",
                        cmd("findcompat", -1, "bb"), cmd("findvalue", -1, "binary\xff".encode("latin1"), bytes(range(25))),
                        cmd("count", "/bus/device", "list"), cmd("search", "/bus/device", "list", "two"),
                        cmd("listget", "/bus/device", "list", 1),
                        cmd("ac", "/"), cmd("sc", "/"), cmd("ac", "/bus")]
            commands += [cmd(op, off) for op in ("name", "depth", "parent", "firstprop", "nextprop", "firstsub", "nextsub", "tag") for off in (-1, 0, 1, 4, 8, 12, len(blob))]
            commands += [cmd("getpath", off, size) for off in (0, 8, len(blob)) for size in (0, 1, 2, 10, 100)]
            with self.subTest(version=version):
                self.compare(commands, blob, len(blob))

    def test_nops_inplace_and_errors(self):
        self.compare(["empty", cmd("set", "/", "a", b"12345"), cmd("set", "/", "b", b"abcd"),
                      cmd("add", "/", "node"), cmd("add", "/node", "child"),
                      cmd("inplace", "/", "a", b"wrong-size"),
                      cmd("partial", "/", "b", 3, b"zz"),
                      cmd("nopprop", "/", "a"), cmd("nopnode", "/node"), "walk",
                      cmd("add", "/", "node"), cmd("delnode", "/node"), "walk"])
        for size in range(80, 160):
            self.compare(["empty", cmd("set", "/", "new-name", b"x"*19),
                          cmd("set", "/", "new-name", b"a"*5), cmd("add", "/", "long-name"),
                          cmd("append", "/", "new-name", b"z"*13)], capacity=size)

    def test_cells_and_address_ranges(self):
        for ac in (0, 1, 2, 3, 4, 5, 0xffffffff):
            for sc in (0, 1, 2, 3, 5):
                self.compare(["empty", cmd("set", "/", "#address-cells", struct.pack(">I", ac)),
                              cmd("set", "/", "#size-cells", struct.pack(">I", sc)),
                              cmd("addrange", "/", "/", "reg", 0xffffff00, 0x100),
                              cmd("addrange", "/", "/", "reg", 0xffffff00, 0x101),
                              cmd("addrange", "/", "/", "reg", 0x100000000, 0x100000000)])

    def test_deterministic_random_mutations(self):
        rng = random.Random(0xFD7)
        for case in range(35):
            commands = ["empty"]
            for i in range(80):
                name = rng.choice([b"a", b"long-name", b"name", b"x\xff"])
                value = rng.randbytes(rng.randrange(0, 40))
                op = rng.choice(["set", "set", "append", "del", "nopprop", "inplace"])
                commands.append(cmd(op, "/", name, *([value] if op in ("set", "append", "inplace") else [])))
            commands += ["walk", "pack"]
            with self.subTest(case=case):
                self.compare(commands)

    def test_overlay_merge_phandles_fixups_and_symbols(self):
        base = dtb((b"", [], [
            (b"target", [(b"phandle", struct.pack(">I", 10)), (b"old", b"keep\0")], []),
            (b"__symbols__", [(b"target_label", b"/target\0")], [])]), capacity=4096)
        overlay = dtb((b"", [], [
            (b"fragment@0", [(b"target", struct.pack(">I", 0xffffffff))], [
                (b"__overlay__", [(b"phandle", struct.pack(">I", 1)), (b"new", b"data\0")], [
                    (b"child", [(b"phandle", struct.pack(">I", 2)),
                                 (b"reference", struct.pack(">II", 1, 2))], [])])]),
            (b"__fixups__", [(b"target_label", b"/fragment@0:target:0\0")], []),
            (b"__local_fixups__", [], [(b"fragment@0", [], [(b"__overlay__", [], [
                (b"child", [(b"reference", struct.pack(">II", 0, 4))], [])])])]),
            (b"__symbols__", [(b"child_label", b"/fragment@0/__overlay__/child\0"),
                               (b"root_label", b"/fragment@0/__overlay__\0")], [])]))
        with tempfile.TemporaryDirectory(prefix="overlay-input-") as temp:
            path = Path(temp) / "overlay"
            path.write_bytes(overlay)
            self.compare([cmd("overlay", os.fsencode(path)), "walk", "pack"], base)
            for target in (b"/target", b"/", b"/missing"):
                path.write_bytes(dtb((b"", [], [(b"f", [(b"target-path", target+b"\0")], [
                    (b"__overlay__", [(b"value", b"x")], [(b"new-child", [], [])])])])))
                self.compare([cmd("overlay", os.fsencode(path))], base)
            for capacity in (len(dtb((b"", [], []))), 100, 128):
                root = dtb((b"", [], []), capacity=capacity)
                path.write_bytes(dtb((b"", [], [(b"f", [(b"target-path", b"/\0")], [
                    (b"__overlay__", [(b"value", b"x"*100)], [])])])))
                self.compare([cmd("overlay", os.fsencode(path))], root, len(root))

    def test_open_pack_resize_and_reordered_blocks(self):
        tree = (b"", [(b"name", b"root\0")], [(b"child", [(b"v", b"xyz")], [])])
        for version in (15, 16, 17, 18):
            blob = dtb(tree, version, [(42, 99)])
            self.compare(["open", "check", "walk", "pack"], blob)
            for size in (len(blob)-1, len(blob), len(blob)+100):
                self.compare([cmd("copyopen", size)], blob)
            h = list(struct.unpack(">10I", blob[:40]))
            reserve, structure, strings = blob[h[4]:h[2]], blob[h[2]:h[3]], blob[h[3]:]
            reordered = bytearray(blob[:40] + strings)
            reordered.extend(bytes(-len(reordered) % 8))
            h[4] = len(reordered)
            reordered.extend(reserve)
            h[2] = len(reordered)
            reordered.extend(structure)
            h[3] = 40
            h[1] = len(reordered)
            reordered[:40] = struct.pack(">10I", *h)
            for size in (h[1], h[1]+100, h[1]*3):
                self.compare(["open"], bytes(reordered), size)
                self.compare([cmd("copyopen", size)], bytes(reordered), size)
        for size in (48, 80, 100, 300, 4096):
            self.compare(["create 0", "finishreserve", "begin -", cmd("property", "a", bytes(range(7))),
                          cmd("resize", size), "end", "finish"])

    def test_header_validation_and_malformed_tags(self):
        original = dtb((b"", [(b"name", b"text\0")], []), capacity=256)
        for field in range(10):
            for value in (0, 1, 2, 3, 15, 16, 17, 18, 39, 40, 41, 128, 255, 256, 257, 0xffffffff):
                blob = bytearray(original)
                struct.pack_into(">I", blob, field*4, value)
                # The C checker does not know backing allocation length; only
                # compare values within its actual backing memory boundary.
                if field == 1 and 256 < value < 0x80000000:
                    continue
                with self.subTest(field=field, value=value):
                    self.compare(["check"], bytes(blob), 256)
        start = struct.unpack_from(">I", original, 8)[0]
        for relative in (0, 4, 8, 12, 16, 20, 24):
            for value in (0, 1, 2, 3, 4, 9, 0x7fffffff, 0xffffffff):
                blob = bytearray(original)
                struct.pack_into(">I", blob, start+relative, value)
                self.compare(["tag 0", "tag 8", "name 0", "firstprop 0"], bytes(blob), 256)

    def test_hostile_buffers_are_checked_without_panics(self):
        rust = self.tools[1]
        randomizer = random.Random(1234)
        with tempfile.TemporaryDirectory(prefix="fdt-hostile-") as temp:
            work = Path(temp)
            for case in range(200):
                blob = bytearray(randomizer.randbytes(256))
                if case % 2:
                    blob[:40] = dtb((b"", [], []), capacity=256)[:40]
                (work / "in").write_bytes(blob)
                result = subprocess.run([rust, work / "in", work / "out", "256"],
                                        input=b"check\ntag 0\nname 0\npath 2f61\nget 2f 61\nopen\npack\n",
                                        capture_output=True, timeout=5)
                self.assertEqual(result.returncode, 0, (case, result.stderr))
                self.assertNotIn(b"panicked", result.stderr)

    def test_overlay_error_matrix_and_local_bounds(self):
        phandle = lambda n: struct.pack(">I", n)
        base = dtb((b"", [], [(b"target", [(b"phandle", phandle(10))], []),
                              (b"__symbols__", [(b"label", b"/target\0")], [])]), capacity=2048)
        variants = [
            ([(b"target", b"bad")], [], []),
            ([(b"target", phandle(0xffffffff))], [], []),
            ([], [], []),
            ([(b"target", phandle(100))], [], []),
            ([(b"target-path", b"/target\0")], [(b"phandle", b"bad")], []),
            ([(b"target-path", b"/target\0")], [(b"phandle", phandle(0xfffffff9))], []),
        ]
        for fixup in (b"no-colons\0", b"/f:target\0", b"/f::0\0", b"/f:target:0x0\0",
                      b"/f:target:nan\0", b"/missing:target:0\0", b"/f:target:3\0",
                      b"/f:target:0", b"", b"/f:target:+0\0", b"/f:target: 0\0"):
            variants.append(([(b"target", phandle(0xffffffff))], [],
                             [(b"__fixups__", [(b"label", fixup)], [])]))
        for symbol in (b"relative\0", b"/f/__overlay__\0extra", b"/missing/__overlay__\0", b"/f/ignored\0"):
            variants.append(([(b"target-path", b"/target\0")], [],
                             [(b"__symbols__", [(b"name", symbol)], [])]))
        for value in (b"bad", phandle(0)):
            variants.append(([(b"target-path", b"/target\0")], [],
                             [(b"__local_fixups__", [], [(b"f", [], [
                                 (b"__overlay__", [(b"missing", value)], [])])])]))
        with tempfile.TemporaryDirectory(prefix="overlay-invalid-") as temp:
            path = Path(temp) / "overlay"
            for target, props, extras in variants:
                path.write_bytes(dtb((b"", [], [(b"f", target, [(b"__overlay__", props, [])])] + extras)))
                self.compare([cmd("overlay", os.fsencode(path))], base, 2048)
            # Earlier local-fixup siblings must be modified before a later
            # missing sibling fails, preserving C's depth-first failure effects.
            path.write_bytes(dtb((b"", [], [
                (b"first", [(b"ref", phandle(1))], []),
                (b"__local_fixups__", [], [(b"first", [(b"ref", phandle(0))], []),
                                             (b"missing", [], [])])])) )
            self.compare([cmd("overlay", os.fsencode(path))], base, 2048)
            # Original libfdt blindly dereferences these local fixup offsets.
            # Rust bounds checks both the unaligned read and write instead.
            for offset in (1, 4, 0xfffffffc, 0xffffffff):
                path.write_bytes(dtb((b"", [], [(b"f", [(b"target-path", b"/target\0")], [
                    (b"__overlay__", [(b"ref", phandle(1))], [])]),
                    (b"__local_fixups__", [], [(b"f", [], [
                        (b"__overlay__", [(b"ref", phandle(offset))], [])])])])))
                infile = Path(temp) / "base"
                infile.write_bytes(base)
                out = Path(temp) / "out"
                result = subprocess.run([self.tools[1], infile, out, "2048"],
                                        input=(cmd("overlay", os.fsencode(path))+"\n").encode(),
                                        capture_output=True, timeout=5)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(result.stdout, b"-16\n")
                self.assertEqual(out.read_bytes()[:4], b"\xff"*4)

    def test_malformed_sequential_headers_never_index_past_slice(self):
        with tempfile.TemporaryDirectory(prefix="fdt-sw-hostile-") as temp:
            work = Path(temp)
            for total in (100, 101, 0xffffffff):
                for structoff in (64, 100, 101, 0xffffffff):
                    blob = struct.pack(">10I", 0x2ff20112, total, structoff, total,
                                       48, 17, 0, 0, 0, 0).ljust(100, b"\0")
                    (work / "in").write_bytes(blob)
                    result = subprocess.run([self.tools[1], work / "in", work / "out", "100"],
                                            input=b"property 61 -\nbegin 61\nend\nfinish\n",
                                            capture_output=True, timeout=5)
                    self.assertEqual(result.returncode, 0, (total, structoff, result.stderr))

    def test_deep_overlay_uses_explicit_stacks(self):
        overlay = bytearray(dtb((b"", [], [(b"f", [(b"target-path", b"/\0")],
                                                [(b"__overlay__", [], [])])])) )
        h = list(struct.unpack(">10I", overlay[:40]))
        start = overlay.index(b"__overlay__\0") + len(b"__overlay__\0")
        start = (start+3) & ~3
        depth = 2000
        nested = (struct.pack(">I", 1)+b"n\0\0\0")*depth + struct.pack(">I", 2)*depth
        overlay[start:start] = nested
        h[1] += len(nested)
        h[3] += len(nested)
        h[9] += len(nested)
        overlay[:40] = struct.pack(">10I", *h)
        with tempfile.TemporaryDirectory(prefix="overlay-deep-") as temp:
            path = Path(temp) / "overlay"
            path.write_bytes(overlay)
            self.compare([cmd("overlay", os.fsencode(path)), "pack"],
                         dtb((b"", [], []), capacity=30000), 30000)


if __name__ == "__main__":
    unittest.main()
