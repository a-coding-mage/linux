#!/usr/bin/env python3
# SPDX-License-Identifier: LGPL-2.1-or-later
"""Compare certificate extraction with the original OpenSSL-backed tool."""

import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class ExtractCertTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="extract-cert-tools-")
        cls.addClassCleanup(cls.temporary.cleanup)
        work = Path(cls.temporary.name)
        cls.c, cls.rust = work / "extract-cert-c", work / "extract-cert-rust"
        cls.cc = shlex.split(os.environ.get("HOSTCC", "cc"))
        pkg_config = shlex.split(os.environ.get("HOSTPKG_CONFIG", "pkg-config"))
        cflags = subprocess.run(pkg_config + ["--cflags", "libcrypto"], text=True, capture_output=True)
        cflags = shlex.split(cflags.stdout) if cflags.returncode == 0 else []
        libs = subprocess.run(pkg_config + ["--libs", "libcrypto"], text=True, capture_output=True)
        libs = shlex.split(libs.stdout) if libs.returncode == 0 else ["-lcrypto"]
        subprocess.run(cls.cc + ["-O2", "-Wno-deprecated-declarations", *cflags, "-I" + str(ROOT / "scripts"),
                                str(ROOT / "certs/extract-cert.c"), "-o", str(cls.c), *libs],
                       check=True, capture_output=True)
        flags = subprocess.run(["python3", str(ROOT / "scripts/openssl_config.py"),
                                "--cc=" + shlex.join(cls.cc), "--cflags=" + shlex.join(cflags)],
                               text=True, capture_output=True, check=True)
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
            *shlex.split(flags.stdout), *["-Clink-arg=" + arg for arg in libs],
            str(ROOT / "certs/extract-cert.rs"), "-o", str(cls.rust)], check=True, capture_output=True)
        cls.pem, cls.der = [], []
        for index, subject in enumerate(("/CN=Temporary test certificate/O=Lupos", "/CN=Second/C=JP", "/CN=" + "x" * 64)):
            pem, key, der = (work / f"cert{index}.pem", work / f"key{index}.pem", work / f"cert{index}.der")
            subprocess.run(["openssl", "req", "-new", "-newkey", "rsa:1024", "-nodes", "-x509", "-days", "1",
                            "-subj", subject, "-keyout", str(key), "-out", str(pem)], check=True, capture_output=True)
            subprocess.run(["openssl", "x509", "-in", str(pem), "-outform", "DER", "-out", str(der)],
                           check=True, capture_output=True)
            cls.pem.append(pem.read_bytes())
            cls.der.append(der.read_bytes())

    def compare(self, data, source="input.pem", destination="output.der", status=0, verbose="", extra_env=None):
        with tempfile.TemporaryDirectory(prefix="extract-cert-case-") as temporary:
            work = Path(temporary)
            environment = dict(os.environ, KBUILD_VERBOSE=verbose)
            environment.update(extra_env or {})
            results = []
            for tool in (self.c, self.rust):
                if source and data is not None and not source.startswith("pkcs11:"):
                    (work / source).write_bytes(data)
                output = work / destination
                if destination == "directory":
                    output.mkdir(exist_ok=True)
                elif "/" not in destination and destination != source:
                    output.write_bytes(b"PREVIOUS OUTPUT")
                result = subprocess.run([str(tool), source, destination], env=environment,
                                        cwd=work, capture_output=True, timeout=10)
                result_data = output.read_bytes() if output.is_file() else None
                results.append((result.returncode, result.stdout,
                                result.stderr.replace(os.fsencode(tool.name), b"extract-cert"), result_data))
            self.assertEqual(results[0][0], status, results[0][:3])
            self.assertEqual(results[0], results[1])
            return results[1]

    def test_single_and_multiple_certificates(self):
        for count in (1, 2, 3, 12):
            certificates = [index % len(self.pem) for index in range(count)]
            for verbose in ("", "0", "1", "21", "verbose=1"):
                with self.subTest(count=count, verbose=verbose):
                    result = self.compare(b"".join(self.pem[index] for index in certificates), verbose=verbose)
                    self.assertEqual(result[3], b"".join(self.der[index] for index in certificates))
                    self.assertEqual(result[2].count(b"Extracted cert:"), count if "1" in verbose else 0)

    def test_comments_noise_and_line_endings(self):
        for pem in (b"comment\n" + self.pem[0], self.pem[0] + b"trailing garbage\0\xff",
                    self.pem[0].replace(b"\n", b"\r\n"), self.pem[0].rstrip(b"\n"),
                    b"-" * 10000 + b"\n" + self.pem[0]):
            self.assertEqual(self.compare(pem)[3], self.der[0])
        result = self.compare(self.pem[0] + b"garbage\n" + self.pem[1])
        self.assertEqual(result[3], self.der[0] + self.der[1])

    def test_empty_source_creates_or_truncates(self):
        self.assertEqual(self.compare(None, source="")[3], b"")
        self.compare(None, source="", destination="directory", status=1)
        self.compare(None, source="", destination="missing/output", status=1)

    def test_invalid_input_preserves_previous_output(self):
        for data in (b"", b"junk", self.der[0], b"-----BEGIN CERTIFICATE-----\ninvalid\n-----END CERTIFICATE-----\n",
                     self.pem[0][:len(self.pem[0]) // 2], self.pem[0].replace(b"CERTIFICATE", b"OTHER")):
            result = self.compare(data, status=1)
            self.assertEqual(result[3], b"PREVIOUS OUTPUT")

    def test_error_after_first_certificate_preserves_partial_output(self):
        result = self.compare(self.pem[0] + b"-----BEGIN CERTIFICATE-----\ninvalid\n-----END CERTIFICATE-----\n", status=1)
        self.assertEqual(result[3], self.der[0])

    def test_paths_and_file_errors(self):
        for source, destination in (("input with spaces", "output with spaces"), ("入力.pem", "出力.der"),
                                    (os.fsdecode(b"input-\xff.pem"), os.fsdecode(b"output-\xfe.der"))):
            self.compare(self.pem[0], source=source, destination=destination)
            self.compare(b"invalid", source=source, destination=destination, status=1)
        self.compare(None, source="missing", status=1)
        self.compare(self.pem[0], destination="directory", status=1)
        self.compare(self.pem[0], destination="missing/output", status=1)
        self.compare(self.pem[0], destination="input.pem")

    def test_symlink_and_hardlink_output(self):
        with tempfile.TemporaryDirectory(prefix="extract-cert-links-") as temporary:
            work = Path(temporary)
            (work / "input").write_bytes(self.pem[0])
            (work / "output").write_bytes(b"PREVIOUS")
            (work / "output").chmod(0o640)
            before = (work / "output").stat()
            os.link(work / "output", work / "hardlink")
            (work / "symlink").symlink_to("output")
            result = subprocess.run([str(self.rust), "input", "symlink"], cwd=work, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            after = (work / "output").stat()
            self.assertEqual((before.st_ino, before.st_mode), (after.st_ino, after.st_mode))
            self.assertEqual((work / "hardlink").read_bytes(), self.der[0])

    def test_missing_pkcs11_provider(self):
        with tempfile.TemporaryDirectory(prefix="extract-cert-providers-") as temporary:
            self.compare(None, source="pkcs11:object=does-not-exist", status=1,
                         extra_env={"OPENSSL_MODULES": temporary, "OPENSSL_ENGINES": temporary})

    def test_usage_and_buffered_write_failure(self):
        for tool in (self.c, self.rust):
            for arguments in ([], ["one"], ["one", "two", "three"]):
                result = subprocess.run([str(tool), *arguments], capture_output=True)
                self.assertEqual(result.returncode, 2)
                self.assertEqual(result.stdout, b"")
                self.assertEqual(result.stderr, b"Usage: extract-cert <source> <dest>\n")
        if Path("/dev/full").exists():
            with tempfile.TemporaryDirectory(prefix="extract-cert-full-") as temporary:
                source = Path(temporary) / "input.pem"
                source.write_bytes(self.pem[0])
                result = subprocess.run([str(self.rust), str(source), "/dev/full"], capture_output=True)
                self.assertEqual(result.returncode, 1)
                self.assertIn(b"/dev/full", result.stderr)


if __name__ == "__main__":
    unittest.main()
