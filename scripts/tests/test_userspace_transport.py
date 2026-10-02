# SPDX-License-Identifier: GPL-2.0-only
"""Static ELF admission and lossless guest-test execution evidence."""

from contextlib import redirect_stderr
import io
import json
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from unittest import mock

import boot_kernel as boot
import userspace_transport as transport


def executable(wide=True, machine=62):
    size, stride = (64, 56) if wide else (52, 32)
    data = bytearray(size + stride + 4)
    data[:7] = b"\x7fELF" + bytes([2 if wide else 1, 1, 1])
    struct.pack_into("<HHI", data, 16, 2, machine, 1)
    struct.pack_into("<Q" if wide else "<I", data, 32 if wide else 28, size)
    struct.pack_into("<HH", data, 54 if wide else 42, stride, 1)
    if wide:
        struct.pack_into("<IIQQQQQQ", data, size, 1, 5, size + stride, 0x400000, 0x400000, 4, 4, 4096)
    else:
        struct.pack_into("<IIIIIIII", data, size, 1, size + stride, 0x400000, 0x400000, 4, 4, 5, 4096)
    return bytes(data)


class UserspaceTransportTests(unittest.TestCase):
    def test_native_and_compat_architectures_and_config_gate(self):
        for wide, machine, host, result in [(True, 62, "x86_64", "x86_64"),
                                             (False, 3, "x86_64", "i686"),
                                             (True, 183, "aarch64", "aarch64")]:
            self.assertEqual(transport.static_test_arch(executable(wide, machine), host,
                                                       ["CONFIG_IA32_EMULATION=y"]), result)
        for data, arch, config in [(executable(False, 3), "x86_64", []),
                                   (executable(), "aarch64", []),
                                   (executable(True, 183), "x86_64", []),
                                   (executable(False, 3), "aarch64", ["CONFIG_IA32_EMULATION=y"])]:
            with self.assertRaises(ValueError):
                transport.static_test_arch(data, arch, config)

    def test_truncation_dynamic_loader_and_nonexecutables_are_rejected(self):
        good = executable()
        for end in range(len(good)):
            with self.assertRaises(ValueError):
                transport.static_test_arch(good[:end], "x86_64", [])
        for offset, value in [(4, 0), (5, 2), (6, 0), (16, 3), (18, 3), (54, 8), (56, 0)]:
            changed = bytearray(good)
            changed[offset] = value
            with self.assertRaises(ValueError):
                transport.static_test_arch(changed, "x86_64", [])
        for kind in (2, 3, 0):
            changed = bytearray(good)
            struct.pack_into("<I", changed, 64, kind)
            with self.assertRaises(ValueError):
                transport.static_test_arch(changed, "x86_64", [])
        for offset, value in [(32, 2**63), (72, 2**63), (96, 2**63), (104, 0)]:
            changed = bytearray(good)
            struct.pack_into("<Q", changed, offset, value)
            with self.assertRaises(ValueError):
                transport.static_test_arch(changed, "x86_64", [])

    def test_manifest_stages_exact_validated_bytes_in_argument_order(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            paths = [root / "native", root / "compat"]
            paths[0].write_bytes(executable())
            paths[1].write_bytes(executable(False, 3))
            files = transport.prepare_userspace_tests(paths, "x86_64", ["CONFIG_IA32_EMULATION=y"])
            paths[0].write_bytes(b"changed after validation")
            entries = transport.userspace_test_manifest(root, files)
            self.assertEqual((root / "userspace-plan").read_text(), "userspace-v1\n2\n")
            self.assertEqual((root / "userspace-inputs/0").read_bytes(), executable())
            self.assertEqual((root / "userspace-inputs/1").read_bytes(), executable(False, 3))
            manifest = json.loads((root / "userspace-manifest.json").read_text())
            self.assertEqual([item["arch"] for item in manifest], ["x86_64", "i686"])
            self.assertEqual([item["index"] for item in manifest], [0, 1])
            self.assertIn("file /userspace-tests/1 ", entries)
            self.assertEqual(transport.userspace_test_manifest(root, []), "")

    def test_exact_streams_empty_output_and_signals_are_preserved(self):
        data = (b"[    1.0] LUPOS_USERSPACE_BEGIN 0\n"
                b"[    1.1] LUPOS_USERSPACE_STDOUT 0 0 68690a00ff\n"
                b"LUPOS_USERSPACE_STDERR 0 0 6261640d0a\n"
                b"LUPOS_USERSPACE_EXIT 0 1024 5 5\n"
                b"LUPOS_USERSPACE_BEGIN 1\nLUPOS_USERSPACE_EXIT 1 139 0 0\n")
        results = transport.userspace_test_results(data, 2)
        self.assertEqual(results[0]["stdout"], b"hi\n\0\xff")
        self.assertEqual(results[0]["stderr"], b"bad\r\n")
        self.assertEqual(results[0]["exit_code"], 4)
        self.assertEqual(results[1]["signal"], 11)
        self.assertIsNone(results[1]["exit_code"])
        with self.assertRaises(ValueError):
            transport.require_userspace_success(results)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            transport.save_userspace_results(root, results)
            self.assertEqual((root / "userspace-0.stdout").read_bytes(), results[0]["stdout"])
            self.assertEqual(json.loads((root / "userspace-results.json").read_text())[1]["status"], 139)

    def test_missing_duplicate_reordered_and_truncated_output_never_passes(self):
        good = b"LUPOS_USERSPACE_BEGIN 0\nLUPOS_USERSPACE_STDOUT 0 0 ff\nLUPOS_USERSPACE_EXIT 0 0 1 0\n"
        transport.require_userspace_success(transport.userspace_test_results(good, 1))
        bad = [b"", good + good, good.replace(b"BEGIN 0", b"BEGIN 1"),
               good.replace(b"0 0 ff", b"0 1 ff"), good.replace(b"0 0 ff", b"0 00 ff"),
               good.replace(b"ff", b"f"), good.replace(b"ff", b"FF"),
               good.replace(b" 1 0\n", b" 2 0\n"), good.replace(b"EXIT 0 0", b"EXIT 0 127"),
               good.replace(b"EXIT 0 0", b"EXIT 0 65536"), good.replace(b"LUPOS_USERSPACE_BEGIN", b"xLUPOS_USERSPACE_BEGIN")]
        for data in bad:
            with self.subTest(data=data), self.assertRaises(ValueError):
                transport.userspace_test_results(data, 1)
        with self.assertRaises(ValueError):
            transport.userspace_test_results(good, 0)
        for status in (256, 1024, 9, 139, 32512):
            data = good.replace(b"EXIT 0 0", f"EXIT 0 {status}".encode())
            with self.assertRaises(ValueError):
                transport.require_userspace_success(transport.userspace_test_results(data, 1))

    def test_previous_decoded_results_are_preserved_but_cannot_mask_a_failed_run(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            previous = {"userspace-results.json": b"old successful result\n",
                        "userspace-0.stdout": b"old output\x00\xff",
                        "userspace-0.stderr": b"old error\n"}
            for name, data in previous.items():
                (root / name).write_bytes(data)
            (root / "userspace-plan").write_text("userspace-v1\n1\n")
            archive = transport.archive_previous_userspace_results(root)
            for name, data in previous.items():
                self.assertFalse((root / name).exists())
                self.assertEqual((archive / name).read_bytes(), data)
            with self.assertRaises(ValueError):
                transport.userspace_test_results(b"LUPOS_USERSPACE_BEGIN 0\n", 1)
            self.assertFalse((root / "userspace-results.json").exists())
            self.assertEqual((root / "userspace-plan").read_text(), "userspace-v1\n1\n")
            self.assertIsNone(transport.archive_previous_userspace_results(root))

    def test_cli_rejects_wrong_binary_before_writes_or_guest_commands(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b"not executable")
            (root / ".config").write_text("CONFIG_X86_64=y\nCONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\n")
            binary = root / "test"
            binary.write_bytes(executable(False, 3))
            with mock.patch.object(sys, "argv", ["boot", "--build", temporary, "--userspace-test", str(binary)]), \
                 mock.patch.object(boot.subprocess, "run") as run, \
                 mock.patch.object(boot.subprocess, "Popen") as popen, \
                 redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                boot.main()
            run.assert_not_called()
            popen.assert_not_called()
            self.assertFalse((root / "rust-boot-test").exists())

    def test_cli_adds_input_device_and_ordered_plan_for_real_execution(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b"not executable")
            (root / ".config").write_text("CONFIG_X86_64=y\nCONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\n"
                                          "CONFIG_PROC_FS=y\nCONFIG_BINFMT_ELF=y\n")
            binary = root / "test"
            binary.write_bytes(executable())
            with mock.patch.object(sys, "argv", ["boot", "--build", temporary,
                                                 "--userspace-test", str(binary), "--userspace-test", str(binary)]), \
                 mock.patch.object(boot.subprocess, "run"), \
                 mock.patch.object(boot.subprocess, "Popen", side_effect=RuntimeError("before QEMU")), \
                 self.assertRaisesRegex(RuntimeError, "before QEMU"):
                boot.main()
            manifest = (root / "rust-boot-test/manifest").read_text()
            self.assertEqual(manifest.count("nod /dev/null 0666 0 0 c 1 3\n"), 1)
            self.assertEqual((root / "rust-boot-test/userspace-plan").read_text(), "userspace-v1\n2\n")
            self.assertLess(manifest.index("file /userspace-tests/0 "), manifest.index("file /userspace-tests/1 "))


if __name__ == "__main__":
    unittest.main()
