# SPDX-License-Identifier: LGPL-2.1-or-later
"""OpenSSL signing and byte-for-byte compatibility with the C sign-file tool."""

import os
from pathlib import Path
import shlex
import struct
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
MARKER = b"~Module signature appended~\n"


class SignFileTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="sign-file-tests-")
        cls.directory = Path(cls.temporary.name)
        cls.c = cls.directory / "sign-file-c"
        cls.rust = cls.directory / "sign-file-rust"
        compiler = shlex.split(os.environ.get("HOSTCC", "cc"))
        pkgconfig = shlex.split(os.environ.get("HOSTPKG_CONFIG", "pkg-config"))
        cls.cflags = shlex.split(subprocess.check_output([*pkgconfig, "--cflags", "libcrypto"], text=True))
        libraries = shlex.split(subprocess.check_output([*pkgconfig, "--libs", "libcrypto"], text=True))
        subprocess.run([*compiler, "-O2", "-Wno-deprecated-declarations", *cls.cflags,
                        "-I", str(ROOT / "tools/include/uapi"), str(ROOT / "scripts/sign-file.c"),
                        "-o", str(cls.c), *libraries], check=True)
        cls.flags = shlex.split(subprocess.check_output([
            "python3", str(ROOT / "scripts/openssl_config.py"), "--cc", shlex.join(compiler),
            "--cflags=" + shlex.join(cls.cflags)], text=True))
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O",
                        "-Wmissing-docs", "-Wrust_2018_idioms", "-Wunreachable_pub", "-Dwarnings",
                        *cls.flags, "-Clink-args=" + shlex.join(libraries), str(ROOT / "scripts/sign-file.rs"),
                        "-o", str(cls.rust)], check=True)
        cls.key = cls.directory / "rsa.key"
        cls.cert = cls.directory / "rsa.pem"
        cls.der = cls.directory / "rsa.der"
        cls.ec_key = cls.directory / "ec.key"
        cls.ec_cert = cls.directory / "ec.pem"
        for key, cert, algorithm in [(cls.key, cls.cert, ["rsa:2048"]),
                                     (cls.ec_key, cls.ec_cert, ["ec", "-pkeyopt", "ec_paramgen_curve:P-256"])]:
            cls.openssl("req", "-new", "-x509", "-newkey", *algorithm, "-nodes", "-days", "1",
                        "-subj", "/CN=Rust host signing test", "-set_serial", "12345", "-keyout", str(key),
                        "-out", str(cert))
        cls.openssl("x509", "-in", str(cls.cert), "-outform", "DER", "-out", str(cls.der))
        cls.encrypted = cls.directory / "encrypted.key"
        cls.openssl("pkey", "-in", str(cls.key), "-aes-256-cbc", "-passout", "pass:correct-pin", "-out", str(cls.encrypted))

    @classmethod
    def openssl(cls, *arguments):
        return subprocess.run(["openssl", *arguments], capture_output=True, check=True)

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def compare(self, arguments, files=None, environment=None, deterministic=True):
        files = files or {}
        env = os.environ.copy()
        env.pop("KBUILD_SIGN_PIN", None)
        env.pop("POSIXLY_CORRECT", None)
        env.update(environment or {})
        results = []
        with tempfile.TemporaryDirectory(dir=self.directory) as work:
            work = Path(work)
            for binary in (self.c, self.rust):
                for child in work.iterdir():
                    child.unlink()
                for name, content in files.items():
                    (work / name).write_bytes(content)
                process = subprocess.run(["sign-file", *map(str, arguments)], executable=str(binary),
                                         cwd=work, env=env, capture_output=True, timeout=15)
                output = {child.name: child.read_bytes() for child in work.iterdir()}
                results.append((process.returncode, process.stdout, process.stderr, output))
        self.assertEqual(results[0][:3], results[1][:3], arguments)
        if deterministic:
            self.assertEqual(results[0][3], results[1][3], arguments)
        return results

    def signature(self, signed, original):
        self.assertTrue(signed.endswith(MARKER))
        trailer = signed[-len(MARKER)-12:-len(MARKER)]
        self.assertEqual(trailer[:8], bytes([0, 0, 2, 0, 0, 0, 0, 0]))
        length = struct.unpack(">I", trailer[8:])[0]
        self.assertEqual(signed[:-len(MARKER)-12-length], original)
        return signed[-len(MARKER)-12-length:-len(MARKER)-12]

    def verify(self, signature, original, certificate):
        with tempfile.TemporaryDirectory(dir=self.directory) as work:
            work = Path(work)
            (work / "signature").write_bytes(signature)
            (work / "content").write_bytes(original)
            self.openssl("cms", "-verify", "-binary", "-inform", "DER", "-in", str(work / "signature"),
                         "-content", str(work / "content"), "-certfile", str(certificate), "-noverify",
                         "-out", os.devnull)

    def test_rsa_hash_algorithms_and_der_certificate(self):
        original = bytes(range(256)) * 33 + b"module bytes\0\xff"
        for digest in ["sha1", "sha224", "sha256", "sha384", "sha512", "sha3-256"]:
            for certificate in [self.cert, self.der]:
                with self.subTest(digest=digest, certificate=certificate.name):
                    result = self.compare([digest, self.key, certificate, "module", "signed"], {"module": original})[1]
                    self.assertEqual(result[0], 0)
                    self.verify(self.signature(result[3]["signed"], original), original, self.cert)

    def test_signature_modes(self):
        original = b"module input\n" * 300
        for mode in [[], ["-p"], ["-d"], ["-k"], ["-pk"], ["-dk"], ["-dp"], ["-dpk"]]:
            for destination in [[], ["module"], ["signed"]]:
                with self.subTest(mode=mode, destination=destination):
                    result = self.compare([*mode, "sha256", self.key, self.cert, "module", *destination], {"module": original})[1]
                    self.assertEqual(result[0], 0)
                    if any("d" in item[1:] for item in mode):
                        self.assertEqual(result[3]["module"], original)
                        self.assertNotIn("signed", result[3])
                        self.verify(result[3]["module.p7s"], original, self.cert)
                    else:
                        signed = result[3][destination[0] if destination else "module"]
                        signature = self.signature(signed, original)
                        self.verify(signature, original, self.cert)
                        if "module.p7s" in result[3]:
                            self.assertEqual(signature, result[3]["module.p7s"])

    def test_ec_signatures_verify(self):
        original = b"elliptic-curve module\0"
        for mode in [[], ["-p"], ["-k"], ["-dk"]]:
            with self.subTest(mode=mode):
                results = self.compare([*mode, "sha256", self.ec_key, self.ec_cert, "module"],
                                       {"module": original}, deterministic=False)
                for result in results:
                    self.assertEqual(result[0], 0)
                    signature = result[3]["module.p7s"] if mode == ["-dk"] else self.signature(result[3]["module"], original)
                    self.verify(signature, original, self.ec_cert)

    def test_raw_signature_mode(self):
        original = b"unsigned module\0\xff"
        for raw in [b"", b"not required to be DER", bytes(range(256)) * 40]:
            for mode in ["-s", "-sd", "-sp", "-sk"]:
                with self.subTest(length=len(raw), mode=mode):
                    result = self.compare([mode, "signature", "ignored-hash", "missing-cert", "module"],
                                          {"module": original, "signature": raw})[1]
                    self.assertEqual(result[0], 0)
                    self.assertEqual(self.signature(result[3]["module"], original), raw)

    def test_encrypted_private_key(self):
        for pin in [None, "correct-pin", "wrong-pin", "", "x" * 1024]:
            with self.subTest(pin=None if pin is None else len(pin)):
                result = self.compare(["sha256", self.encrypted, self.cert, "module"], {"module": b"module"},
                                      {} if pin is None else {"KBUILD_SIGN_PIN": pin})[1]
                self.assertEqual(result[0], 0 if pin == "correct-pin" else 1)
                if result[0] == 0:
                    self.verify(self.signature(result[3]["module"], b"module"), b"module", self.cert)

    def test_input_and_output_errors(self):
        cases = [(["sha256", self.key, self.cert, "missing"], {}),
                 (["sha256", "missing-key", self.cert, "module"], {"module": b"m"}),
                 (["sha256", "bad-key", self.cert, "module"], {"module": b"m", "bad-key": b"bad"}),
                 (["sha256", self.key, "missing-cert", "module"], {"module": b"m"}),
                 (["unknown-hash", self.key, self.cert, "module"], {"module": b"m"}),
                 (["sha256", self.key, self.ec_cert, "module"], {"module": b"m"}),
                 (["sha256", self.key, self.cert, "module", "missing/dest"], {"module": b"m"}),
                 (["-s", "missing-raw", "hash", "cert", "module", "signed"], {"module": b"m"})]
        for length in [0, 1, 2, 64]:
            cases.append((["sha256", self.key, "bad-cert", "module"],
                          {"module": b"m", "bad-cert": b"x" * length}))
        cases.append((["sha256", self.key, "bad-cert", "module"], {"module": b"m", "bad-cert": b"\x30\x82\0\x01x"}))
        for arguments, files in cases:
            with self.subTest(arguments=arguments):
                self.assertEqual(self.compare(arguments, files)[1][0], 1)

    def test_cli(self):
        for arguments in [[], ["-h"], ["--help"], ["-z"], ["--"], ["sha256"], ["a", "b", "c"],
                          ["a", "b", "c", "d", "e", "f"],
                          ["sha256", self.key, self.cert, "module", "-p"],
                          ["--", "sha256", self.key, self.cert, "module"]]:
            with self.subTest(arguments=arguments):
                self.compare(arguments, {"module": b"module"})

    def test_empty_module(self):
        result = self.compare(["sha256", self.key, self.cert, "module"], {"module": b""})[1]
        self.assertEqual(result[0], 0)
        self.verify(self.signature(result[3]["module"], b""), b"", self.cert)

    def test_non_utf8_paths(self):
        name = os.fsdecode(b"module-\xff")
        result = self.compare(["-p", "sha256", self.key, self.cert, name], {name: b"module"})[1]
        self.assertEqual(result[0], 0)
        self.verify(self.signature(result[3][name], b"module"), b"module", self.cert)

    def test_pkcs11_provider_failure(self):
        self.compare(["sha256", "pkcs11:token=missing", self.cert, "module"], {"module": b"module"},
                     {"OPENSSL_MODULES": str(self.directory / "missing-provider-dir")})

    def test_buffered_output_failures(self):
        if not Path("/dev/full").exists():
            self.skipTest("requires /dev/full")
        with tempfile.TemporaryDirectory(dir=self.directory) as work:
            work = Path(work)
            module = work / "module"
            module.write_bytes(b"original module")
            for options in [[], ["-d"], ["-p"]]:
                with self.subTest(options=options):
                    detached = work / "module.p7s"
                    if detached.exists():
                        detached.unlink()
                    if options:
                        detached.symlink_to("/dev/full")
                    arguments = [*options, "sha256", str(self.key), str(self.cert), str(module)]
                    if not options:
                        arguments.append("/dev/full")
                    result = subprocess.run([self.rust, *arguments], capture_output=True)
                    self.assertEqual(result.returncode, 1)
                    self.assertEqual(module.read_bytes(), b"original module")
                    self.assertFalse((work / "module.~signed~").exists())

    def test_missing_abi_probe_is_a_build_error(self):
        result = subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021",
                                 str(ROOT / "scripts/sign-file.rs"), "-o", str(self.directory / "unconfigured")],
                                capture_output=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(b"OpenSSL ABI configuration is missing", result.stderr)

    def test_legacy_engine_branch_with_current_library(self):
        if "--cfg=ossl300" not in self.flags:
            self.skipTest("branch-compatibility check uses current OpenSSL 3 exports")
        binary = self.directory / "sign-file-engine"
        flags = [flag for flag in self.flags if flag != "--cfg=ossl300"] + ["--cfg=ossl_engine"]
        subprocess.run([*shlex.split(os.environ.get("HOSTRUSTC", "rustc")), "--edition=2021", "-O",
                        "-Dwarnings", *flags, str(ROOT / "scripts/sign-file.rs"), "-o", str(binary)], check=True)
        module = self.directory / "engine-module"
        module.write_bytes(b"module")
        result = subprocess.run([binary, "sha256", "pkcs11:token=missing", self.cert, module],
                                env={**os.environ, "OPENSSL_ENGINES": str(self.directory / "absent-engines")},
                                capture_output=True, timeout=10)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b"Load PKCS#11 ENGINE", result.stderr)


if __name__ == "__main__":
    unittest.main()
