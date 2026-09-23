#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Compare the Rust initramfs generator with the original C implementation.

Run directly or via ``python3 -m unittest discover -s scripts/tests``.
Both compilers and every generated archive use temporary directories.
"""

import os
from pathlib import Path
import shlex
import shutil
import socket
import stat
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


def entries(archive):
    """Decode newc/crc without relying on either implementation's parser."""
    result = []
    offset = 0
    while True:
        assert archive[offset:offset + 6] in (b"070701", b"070702")
        fields = [int(archive[offset + 6 + i * 8:offset + 14 + i * 8], 16)
                  for i in range(13)]
        offset += 110
        name = archive[offset:offset + fields[11]].split(b"\0", 1)[0]
        offset = (offset + fields[11] + 3) & ~3
        data_offset = offset
        data = archive[offset:offset + fields[6]]
        offset = (offset + fields[6] + 3) & ~3
        result.append((name, fields, data, data_offset))
        if name == b"TRAILER!!!":
            assert len(archive) % 512 == 0
            assert archive[offset:] == bytes(len(archive) - offset)
            return result


class GenInitCpioTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.build = tempfile.TemporaryDirectory(prefix="gen-init-cpio-build-")
        cls.addClassCleanup(cls.build.cleanup)
        cls.c = Path(cls.build.name) / "gen_init_cpio-c"
        cls.rust = Path(cls.build.name) / "gen_init_cpio-rust"
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-Wall", str(ROOT / "usr/gen_init_cpio.c"),
            "-o", str(cls.c)], check=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-Wmissing-docs", "-Dwarnings", "-O",
            str(ROOT / "usr/gen_init_cpio.rs"), "-o", str(cls.rust)], check=True)

    def setUp(self):
        self.work = tempfile.TemporaryDirectory(prefix="gen-init-cpio-test-")
        self.addCleanup(self.work.cleanup)
        self.directory = Path(self.work.name)
        self.payload = self.directory / "payload"
        self.payload.write_bytes(bytes(range(256)) * 513 + b"last bytes\0")
        os.utime(self.payload, (1234567890, 1234567890))

    def compare(self, source=b"", options=(), timestamp=True, list_file=False,
                env=None, expected_status=0, arguments=None):
        options = list(options)
        if timestamp:
            options = ["-t", "123456789"] + options
        if list_file:
            path = self.directory / "list"
            path.write_bytes(source)
            positional = [str(path)]
        else:
            positional = ["-"]
        args = options + positional if arguments is None else arguments
        environment = dict(os.environ, LC_ALL="C")
        environment.pop("POSIXLY_CORRECT", None)
        if env:
            environment.update(env)
        results = []
        for binary in (self.c, self.rust):
            process = subprocess.run([str(binary)] + args, input=source,
                                     capture_output=True, env=environment)
            stderr = process.stderr.replace(os.fsencode(binary), b"gen_init_cpio")
            results.append((process.returncode, process.stdout, stderr))
        self.assertEqual(results[0], results[1])
        self.assertEqual(results[1][0], expected_status, results[1][2])
        return results[1]

    def file_line(self, name=b"/payload", tail=b""):
        return b"file " + name + b" " + os.fsencode(self.payload) + b" 0640 123 456" + tail + b"\n"

    def test_all_node_types_and_hard_links(self):
        source = (b"# the complete file-list grammar\n\n"
                  b"dir /dev 0755 0 0\n"
                  b"nod /dev/console 0600 0 0 c 5 1\n"
                  b"nod /dev/disk 0660 4 5 b 259 65535\n"
                  b"slink /init /bin/init 0777 2 3\n"
                  b"pipe /fifo 0600 6 7\n"
                  b"sock /socket 0600 8 9\n" +
                  self.file_line(tail=b" /hardlink relative-link"))
        _, archive, _ = self.compare(source, list_file=True)
        decoded = entries(archive)
        self.assertEqual([row[1][1] & 0o170000 for row in decoded[:-1]],
                         [stat.S_IFDIR, stat.S_IFCHR, stat.S_IFBLK,
                          stat.S_IFLNK, stat.S_IFIFO, stat.S_IFSOCK,
                          stat.S_IFREG, stat.S_IFREG, stat.S_IFREG])
        links = decoded[-4:-1]
        self.assertEqual([row[1][0] for row in links], [727] * 3)
        self.assertEqual([row[1][4] for row in links], [3] * 3)
        self.assertEqual([row[2] for row in links], [b"", b"", self.payload.read_bytes()])
        self.assertEqual(decoded[3][2], b"/bin/init\0")

    def test_checksum_and_alignment(self):
        source = self.file_line(tail=b" /second /third")
        for alignment in (0, 4, 12, 16, 256, 4096, 8192, 65536):
            with self.subTest(alignment=alignment):
                _, archive, _ = self.compare(source, ["-c", "-a", str(alignment)])
                decoded = entries(archive)
                self.assertEqual(archive[:6], b"070702")
                self.assertEqual(decoded[2][1][12], sum(self.payload.read_bytes()) & 0xffffffff)
                self.assertEqual(decoded[0][1][12], 0)
                if alignment and alignment <= 4096 and alignment & (alignment - 1) == 0:
                    self.assertEqual(decoded[2][3] % alignment, 0)

    def test_alignment_fallback_for_long_name(self):
        source = self.file_line(name=b"/" + b"x" * 4050)
        _, _, stderr = self.compare(source, ["-a", "8192"])
        self.assertIn(b"best-effort alignment 8192 missed", stderr)

    def test_checksum_wraps_at_32_bits(self):
        self.payload.write_bytes(b"\xff" * (17 * 1024 * 1024))
        _, archive, _ = self.compare(self.file_line(), ["-c"])
        self.assertEqual(entries(archive)[0][1][12], (255 * 17 * 1024 * 1024) & 0xffffffff)

    def test_empty_file_and_archive(self):
        self.compare(b"# empty archive\n\n")
        self.payload.write_bytes(b"")
        _, archive, _ = self.compare(self.file_line(), ["-c", "-a", "4096"])
        self.assertEqual(entries(archive)[0][2], b"")

    def test_file_timestamp_and_override(self):
        _, archive, _ = self.compare(self.file_line(), timestamp=False)
        self.assertEqual(entries(archive)[0][1][5], 1234567890)
        _, archive, _ = self.compare(self.file_line(), ["-t", "4294967295"])
        self.assertEqual(entries(archive)[0][1][5], 0xffffffff)

    def test_timestamp_clipping(self):
        for timestamp, stored, warning in ((-1, 0, b"Timestamp negative"),
                                           (4294967296, 4294967295, b"Timestamp exceeds maximum")):
            with self.subTest(timestamp=timestamp):
                os.utime(self.payload, (timestamp, timestamp))
                if int(self.payload.stat().st_mtime) != timestamp:
                    self.skipTest("filesystem cannot represent the test timestamp")
                _, archive, stderr = self.compare(self.file_line(), timestamp=False)
                self.assertEqual(entries(archive)[0][1][5], stored)
                self.assertIn(warning, stderr)

    def test_environment_expansion(self):
        source = b"file /data ${CPIO_TEST_ROOT}/${CPIO_MISSING}${CPIO_TEST_NAME} 0644 0 0\n"
        self.compare(source, env={"CPIO_TEST_ROOT": str(self.directory),
                                  "CPIO_TEST_NAME": "${CPIO_TEST_FINAL}",
                                  "CPIO_TEST_FINAL": "payload", "CPIO_MISSING": ""})
        self.compare(b"file /data ${CPIO_ABSENT_VARIABLE}" + os.fsencode(self.payload) + b" 0644 0 0\n")

    def test_non_unicode_names_and_locations(self):
        path = os.fsencode(self.directory) + b"/payload-\xff"
        with open(path, "wb") as output:
            output.write(b"non-UTF-8 filename")
        source = b"file /name-\xff " + path + b" 0644 0 0 /hard-\xfe\n"
        source += b"slink /link-\xff /target-\xfe 0777 0 0\n"
        self.compare(source, ["-c"])

    def test_output_file_mode_and_content(self):
        output = self.directory / "archive.cpio"
        archives = []
        for binary in (self.c, self.rust):
            process = subprocess.run([str(binary), "-t0", "-c", "-o", str(output), "-"],
                                     input=self.file_line(), capture_output=True)
            self.assertEqual(process.returncode, 0, process.stderr)
            self.assertEqual(process.stdout, b"")
            archives.append(output.read_bytes())
            self.assertEqual(stat.S_IMODE(output.stat().st_mode), 0o600)
            output.unlink()
        self.assertEqual(*archives)

    def test_options_and_help(self):
        self.compare(arguments=["-h"])
        self.compare(self.file_line(), arguments=["-", "-ct0", "-a4"])
        self.compare(self.file_line(), arguments=["-t", "+000", "--", "-"])
        for arguments in ([], ["-x"], ["-t"], ["-o"], ["-a"], ["-t", ""],
                          ["-t", "123x", "-"], ["-t", "-1", "-"],
                          ["-t", "-18446744073709551615", "-"],
                          ["-t", "4294967296", "-"], ["-a", "3", "-"],
                          ["-a", "4 ", "-"], ["-a", "x", "-"], ["-", "extra"]):
            with self.subTest(arguments=arguments):
                self.compare(arguments=arguments, expected_status=1)

    def test_input_errors(self):
        for source in (b"dir /missing-mode\n", b"slink /target\n", b"nod /device 0600 0 0 c\n",
                       b"pipe /pipe 9999 0 0\n", b"sock /sock 0600 bad 0\n",
                       b"file /missing\n", b"file /missing /no/such/file 0644 0 0\n"):
            with self.subTest(source=source):
                self.compare(source, expected_status=255)
        self.compare(arguments=[str(self.directory / "missing")], expected_status=1)
        self.compare(arguments=["-o", str(self.directory / "missing" / "archive"), "-"], expected_status=1)

    @unittest.skipUnless(Path("/dev/full").exists(), "requires /dev/full")
    def test_output_error(self):
        self.compare(self.file_line(), ["-o", "/dev/full"], expected_status=255)

    def test_gen_initramfs_shell_pipeline(self):
        tree = self.directory / "root"
        tree.mkdir()
        (tree / "subdir").mkdir()
        shutil.copyfile(self.payload, tree / "subdir/payload")
        (tree / "link").symlink_to("subdir/payload")
        os.mkfifo(tree / "pipe")
        server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.addCleanup(server.close)
        try:
            server.bind(str(tree / "socket"))
            has_socket = True
        except PermissionError:
            # Restricted build containers may forbid bind even for a filesystem
            # socket. The direct file-list test covers the socket archive entry.
            has_socket = False
        archives = []
        dependencies = []
        for name, binary in (("c", self.c), ("rust", self.rust)):
            build = self.directory / name
            (build / "usr").mkdir(parents=True)
            (build / "usr/gen_init_cpio").symlink_to(binary)
            output = build / "initramfs.cpio"
            dependency = build / "initramfs.d"
            subprocess.run(["sh", str(ROOT / "usr/gen_initramfs.sh"),
                            "-o", str(output), "-l", str(dependency),
                            "-u", "squash", "-g", "squash", "-d", "@123456789",
                            str(tree)], cwd=build, check=True, capture_output=True)
            archives.append(output.read_bytes())
            dependencies.append(dependency.read_bytes())
        self.assertEqual(*archives)
        self.assertEqual(*dependencies)
        self.assertEqual(len(entries(archives[0])), 6 if has_socket else 5)

    def test_large_file_is_rejected_before_reading(self):
        with self.payload.open("wb") as output:
            output.truncate(1 << 32)
        self.compare(self.file_line(), expected_status=255)

    def test_parser_compatibility(self):
        for source in (b"dir /dir 0755 -1 -2 ignored\n", b"dir /dir 0755 0 0",
                       b"pipe /pipe 0900 0 0\n",
                       b"dir /dir\v0755\f0\r0\n",
                       b"dir /dir 0755 0 0\r\n", b"\t\n# comment\n",
                       b"unknown /value\n", b"  dir /indented 0755 0 0\n",
                       b"dir\n", b"#" + b"x" * 9000 + b"\n"):
            with self.subTest(source=source[:80]):
                self.compare(source)


if __name__ == "__main__":
    unittest.main()
