# SPDX-License-Identifier: GPL-2.0-only
"""Guest architecture selection for the shared real-kernel boot fixture."""

from contextlib import redirect_stderr, redirect_stdout
import io
import os
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import boot_kernel as boot


class BootArchitectureTests(unittest.TestCase):
    def test_matching_image_target_machine_and_console(self):
        expected = {
            "x86_64": ("arch/x86/boot/bzImage", "x86_64-unknown-linux-gnu",
                       ["-machine", "pc"], "ttyS0"),
            "aarch64": ("arch/arm64/boot/Image", "aarch64-unknown-linux-musl",
                        ["-machine", "virt", "-cpu", "cortex-a57"], "ttyAMA0"),
        }
        for arch, settings in expected.items():
            self.assertEqual(boot.architecture(arch), settings)
        with self.assertRaises(ValueError):
            boot.architecture("mips")

    def test_same_real_pid_one_source_and_explicit_cross_linking(self):
        compiler = ["compiler-wrapper", "rustc-1.85"]
        for arch, target in (("x86_64", "x86_64-unknown-linux-gnu"),
                             ("aarch64", "aarch64-unknown-linux-musl")):
            command = boot.init_command(compiler, arch, Path("guest-init"))
            self.assertEqual(command[:2], compiler)
            self.assertEqual([item for item in command if item.startswith("--target=")],
                             ["--target=" + target])
            self.assertIn(str(boot.ROOT / "scripts/tests/boot_init.rs"), command)
            self.assertIn("-Ctarget-feature=+crt-static", command)
            self.assertIn("-Dwarnings", command)
            self.assertEqual(command[command.index("-o") + 1], "guest-init")
            self.assertEqual("-Clinker=rust-lld" in command, arch == "aarch64")
            self.assertEqual("-Clink-self-contained=yes" in command, arch == "aarch64")

    def test_default_and_overridden_qemu_commands(self):
        for arch, console, machine in (("x86_64", "ttyS0", "pc"),
                                        ("aarch64", "ttyAMA0", "virt")):
            command = boot.qemu_command(None, arch, Path("image"), Path("archive"))
            self.assertEqual(command[0], "qemu-system-" + arch)
            self.assertEqual(command[command.index("-machine") + 1], machine)
            self.assertEqual(command[command.index("-kernel") + 1], "image")
            self.assertEqual(command[command.index("-initrd") + 1], "archive")
            self.assertEqual(command[command.index("-append") + 1],
                             f"console={console} rdinit=/init panic=-1 nokaslr printk.devkmsg=on loglevel=7")
            self.assertEqual(command[command.index("-accel") + 1], "tcg")
            self.assertEqual("-cpu" in command, arch == "aarch64")
            custom = boot.qemu_command("'a path/qemu' -no-user-config", arch,
                                       Path("image"), Path("archive"), Path("firmware"))
            self.assertEqual(custom[:2], ["a path/qemu", "-no-user-config"])
            self.assertEqual(custom[-2:], ["-L", str(Path("firmware").resolve())])

    def test_module_syscall_numbers_follow_actual_architecture_headers(self):
        source = (boot.ROOT / "scripts/tests/boot_init.rs").read_text()
        aarch64 = (boot.ROOT / "include/uapi/asm-generic/unistd.h").read_text()
        x86 = (boot.ROOT / "arch/x86/entry/syscalls/syscall_64.tbl").read_text()
        for name in ("init_module", "delete_module"):
            generic = re.search(r"(?m)^#define __NR_" + name + r"\s+(\d+)\s*$", aarch64)
            native = re.search(r"(?m)^(\d+)\s+\w+\s+" + name + r"\s", x86)
            self.assertIsNotNone(generic)
            self.assertIsNotNone(native)
            for arch, number in (("aarch64", generic[1]), ("x86_64", native[1])):
                self.assertIn(f'#[cfg(target_arch = "{arch}")]\n'
                              f'const {name.upper()}: std::ffi::c_long = {number};', source)

    def test_wrong_guest_config_is_rejected_before_compiler_or_writes(self):
        for arch, wrong in (("aarch64", "X86_64"), ("x86_64", "ARM64")):
            with self.subTest(arch=arch), tempfile.TemporaryDirectory(prefix="boot-arch-") as tmp:
                build = Path(tmp)
                for name in (boot.architecture(arch)[0], "usr/gen_init_cpio"):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(b"must not execute")
                (build / ".config").write_text(f"CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_{wrong}=y\n")
                error = io.StringIO()
                with mock.patch.object(sys, "argv", ["boot_kernel", "--build", tmp, "--arch", arch]), \
                     mock.patch.object(boot.subprocess, "run") as run, \
                     mock.patch.object(boot.subprocess, "Popen") as popen, \
                     redirect_stderr(error), self.assertRaises(SystemExit) as caught:
                    boot.main()
                self.assertEqual(caught.exception.code, 2)
                self.assertIn("require CONFIG_", error.getvalue())
                run.assert_not_called()
                popen.assert_not_called()
                self.assertFalse((build / "rust-boot-test").exists())

    def test_printk_is_required_before_any_compiler_or_output_write(self):
        for arch in ("x86_64", "aarch64"):
            with self.subTest(arch=arch), tempfile.TemporaryDirectory(prefix="boot-printk-") as tmp:
                build = Path(tmp)
                for name in (boot.architecture(arch)[0], "usr/gen_init_cpio"):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(b"must not execute")
                selected = "ARM64" if arch == "aarch64" else "X86_64"
                (build / ".config").write_text(f"CONFIG_MULTIUSER=y\nCONFIG_{selected}=y\n# CONFIG_PRINTK is not set\n")
                error = io.StringIO()
                with mock.patch.object(sys, "argv", ["boot_kernel", "--build", tmp, "--arch", arch]), \
                     mock.patch.object(boot.subprocess, "run") as run, \
                     mock.patch.object(boot.subprocess, "Popen") as popen, \
                     redirect_stderr(error), self.assertRaises(SystemExit) as caught:
                    boot.main()
                self.assertEqual(caught.exception.code, 2)
                self.assertIn("require CONFIG_PRINTK=y", error.getvalue())
                run.assert_not_called()
                popen.assert_not_called()
                self.assertFalse((build / "rust-boot-test").exists())

    def test_real_device_and_supported_unlimited_logging_parameter(self):
        memory = (boot.ROOT / "drivers/char/mem.c").read_text()
        printk = (boot.ROOT / "kernel/printk/printk.c").read_text()
        self.assertRegex(memory, r'\[11\]\s*=\s*\{\s*"kmsg",\s*&kmsg_fops')
        self.assertIn('__setup("printk.devkmsg=", control_devkmsg)', printk)
        self.assertIn('str_has_prefix(str, "on")', printk)
        with tempfile.TemporaryDirectory(prefix="boot-kmsg-manifest-") as tmp:
            build = Path(tmp)
            for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
                path = build / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b"must not execute")
            (build / ".config").write_text("CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_X86_64=y\n")
            with mock.patch.object(sys, "argv", ["boot_kernel", "--build", tmp]), \
                 mock.patch.object(boot.subprocess, "run") as run, \
                 mock.patch.object(boot.subprocess, "Popen", side_effect=RuntimeError("stop before QEMU")), \
                 self.assertRaisesRegex(RuntimeError, "stop before QEMU"):
                boot.main()
            self.assertEqual(run.call_count, 2)
            manifest = (build / "rust-boot-test/manifest").read_text().splitlines()
            self.assertEqual(manifest.count("nod /dev/kmsg 0600 0 0 c 1 11"), 1)
            self.assertEqual(manifest.count("nod /dev/console 0600 0 0 c 5 1"), 1)

    @staticmethod
    def events():
        return [b"LUPOS_RUST_MODULE_REJECT_OK 0", b"LUPOS_RUST_PRELOAD_OK 0", b"LUPOS_RUST_PRELOAD_OK 1",
                b"LUPOS_RUST_MODULE_LOAD_OK", b"LUPOS_RUST_MODULE_UNLOAD_OK 2",
                b"LUPOS_RUST_MODULE_UNLOAD_OK 1", b"LUPOS_RUST_MODULE_UNLOAD_OK 0",
                b"LUPOS_RUST_MODULE_RELOAD_OK 0", b"LUPOS_RUST_MODULE_RELOAD_OK 1",
                b"LUPOS_RUST_MODULE_RELOAD_OK 2", boot.MARKER]

    def verify(self, lines):
        boot.verify_module_events(b"\n".join(lines) + b"\n", preloads=2, module=True, rejected=1, reload=True)

    def test_timestamped_bare_and_mixed_complete_events_are_exact(self):
        events = self.events()
        for mode in ("plain", "printk", "mixed"):
            lines = []
            for index, event in enumerate(events):
                lines.append(b"[    1.234567] ordinary module printk")
                prefix = b"[    12.000001] " if mode == "printk" or mode == "mixed" and index % 2 else b""
                lines.append(prefix + event + b"\r")
            with self.subTest(mode=mode):
                self.verify(lines)

    def test_corrupted_interleaved_or_decorated_records_are_not_repaired(self):
        events = self.events()
        corruptions = (
            b"LUPOS_RUST_PRELOAD_OK 1test_mul_u64_u64_div_u64: Completed test_32bit",
            b"module printk LUPOS_RUST_PRELOAD_OK 1",
            b"[broken] LUPOS_RUST_PRELOAD_OK 1",
            b"[ 1.000000] [ 2.000000] LUPOS_RUST_PRELOAD_OK 1",
            b"<6>LUPOS_RUST_PRELOAD_OK 1",
            b"LUPOS_RUST_PRELOAD_OK 1 LUPOS_RUST_MODULE_LOAD_OK",
        )
        for corrupted in corruptions:
            with self.subTest(record=corrupted), self.assertRaises(ValueError):
                self.verify([*events[:2], corrupted, *events[3:]])
            # A clean later record must not hide the earlier corrupted event.
            with self.subTest(extra=corrupted), self.assertRaises(ValueError):
                self.verify([*events, corrupted])

    def test_timestamps_do_not_weaken_missing_duplicate_or_order_validation(self):
        events = [b"[ 1.000000] " + event for event in self.events()]
        for index in range(len(events)):
            for mutated in (events[:index] + events[index + 1:], events[:index] + events[index:] + [events[index]]):
                with self.subTest(index=index), self.assertRaises(ValueError):
                    self.verify(mutated)
        for index in range(len(events) - 1):
            changed = events.copy()
            changed[index:index + 2] = reversed(changed[index:index + 2])
            with self.subTest(swapped=index), self.assertRaises(ValueError):
                self.verify(changed)

    def test_console_reader_waits_for_a_complete_final_record(self):
        for suffix, success in ((b"\r\n", True), (b"_CORRUPTED\n", False), (b"", False)):
            with self.subTest(suffix=suffix), tempfile.TemporaryDirectory(prefix="boot-record-stream-") as tmp:
                build = Path(tmp)
                for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(b"must not execute")
                (build / ".config").write_text("CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_X86_64=y\n")
                process = mock.Mock()
                process.stdout.read1.side_effect = [b"[  1.234567] " + boot.MARKER, suffix, b""]
                process.poll.return_value = None
                process.wait.return_value = 0
                selector = mock.MagicMock()
                selector.__enter__.return_value = selector
                selector.select.return_value = [(None, None)]
                with mock.patch.object(sys, "argv", ["boot_kernel", "--build", tmp]), \
                     mock.patch.object(boot.subprocess, "run"), \
                     mock.patch.object(boot.subprocess, "Popen", return_value=process), \
                     mock.patch.object(boot.selectors, "DefaultSelector", return_value=selector), \
                     redirect_stdout(io.StringIO()), redirect_stderr(io.StringIO()):
                    if success:
                        boot.main()
                    else:
                        with self.assertRaises(SystemExit):
                            boot.main()
                self.assertGreaterEqual(process.stdout.read1.call_count, 2)
                process.terminate.assert_called_once()
                process.wait.assert_called_once_with(timeout=10)
                process.stdout.close.assert_called_once()

    def test_actual_rust_record_writer_short_write_and_interrupted_negative_controls(self):
        compiler = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        env = {key: value for key, value in os.environ.items()
               if key not in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS")}
        with tempfile.TemporaryDirectory(prefix="boot-record-writer-") as tmp:
            binary = Path(tmp) / "record-tests"
            result = subprocess.run([*compiler, "--edition=2021", "--test", "-Dwarnings",
                                     boot.ROOT / "scripts/tests/boot_init.rs", "-o", binary],
                                    capture_output=True, env=env, timeout=120)
            self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
            result = subprocess.run([binary], capture_output=True, timeout=30)
            self.assertEqual(result.returncode, 0, result.stdout.decode() + result.stderr.decode())
            self.assertIn(b"8 passed; 0 failed", result.stdout)
        source = (boot.ROOT / "scripts/tests/boot_init.rs").read_text().split("#[cfg(test)]", 1)[0]
        self.assertNotIn("println!", source)
        self.assertNotIn("io::stdout", source)
        self.assertIn('open("/dev/kmsg")', source)

    def test_failslab_setup_marker_is_exact_unique_and_before_modules(self):
        good = b"[ 1.000001] " + boot.FAILSLAB_MARKER + b"\nLUPOS_RUST_PRELOAD_OK 0\n" + boot.MARKER + b"\n"
        boot.verify_failslab_setup(good, True)
        boot.verify_failslab_setup(boot.MARKER + b"\n", False)
        bad = [good.replace(boot.FAILSLAB_MARKER, b""), good + boot.FAILSLAB_MARKER + b"\n",
               good.replace(boot.FAILSLAB_MARKER, boot.FAILSLAB_MARKER + b" extra"),
               good.replace(boot.FAILSLAB_MARKER, b"prefix " + boot.FAILSLAB_MARKER),
               b"LUPOS_RUST_PRELOAD_OK 0\n" + good,
               good.replace(boot.FAILSLAB_MARKER, b"LUPOS_FAILSLAB_SETUP_FAILED")]
        for console in bad:
            with self.subTest(console=console), self.assertRaises(ValueError):
                boot.verify_failslab_setup(console, True)
        with self.assertRaises(ValueError):
            boot.verify_failslab_setup(good, False)

    def test_failslab_required_configs_fail_before_compilation_or_writes(self):
        for missing in boot.FAILSLAB_CONFIG:
            with self.subTest(missing=missing), tempfile.TemporaryDirectory(prefix="boot-failslab-config-") as tmp:
                build = Path(tmp)
                for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(b"must not execute")
                (build / ".config").write_text("CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_X86_64=y\n" +
                    "".join("CONFIG_" + option + "=y\n" for option in boot.FAILSLAB_CONFIG if option != missing))
                error = io.StringIO()
                with mock.patch.object(sys, "argv", ["boot_kernel", "--build", tmp, "--prepare-failslab"]), \
                     mock.patch.object(boot.subprocess, "run") as run, \
                     mock.patch.object(boot.subprocess, "Popen") as popen, \
                     redirect_stderr(error), self.assertRaises(SystemExit) as caught:
                    boot.main()
                self.assertEqual(caught.exception.code, 2)
                self.assertIn("CONFIG_" + missing + "=y", error.getvalue())
                run.assert_not_called()
                popen.assert_not_called()
                self.assertFalse((build / "rust-boot-test").exists())

    def test_failslab_guest_manifest_is_only_added_when_requested(self):
        for requested, stack_filter in ((False, False), (True, False), (True, True)):
            with self.subTest(requested=requested), tempfile.TemporaryDirectory(prefix="boot-failslab-manifest-") as tmp:
                build = Path(tmp)
                for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(b"must not execute")
                (build / ".config").write_text("CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_X86_64=y\n" +
                    "".join("CONFIG_" + option + "=y\n" for option in boot.FAILSLAB_CONFIG) +
                    ("CONFIG_FAULT_INJECTION_STACKTRACE_FILTER=y\n" if stack_filter else ""))
                argv = ["boot_kernel", "--build", tmp] + (["--prepare-failslab"] if requested else [])
                with mock.patch.object(sys, "argv", argv), mock.patch.object(boot.subprocess, "run"), \
                     mock.patch.object(boot.subprocess, "Popen", side_effect=RuntimeError("stop before QEMU")), \
                     self.assertRaisesRegex(RuntimeError, "stop before QEMU"):
                    boot.main()
                manifest = (build / "rust-boot-test/manifest").read_text()
                self.assertEqual("file /failslab-setup " in manifest, requested)
                setup = build / "rust-boot-test/failslab-setup"
                self.assertEqual(setup.exists(), requested)
                if requested:
                    self.assertEqual(setup.read_text(), f"failslab-v1\nstacktrace-filter={int(stack_filter)}\n")

    def test_actual_root_target_selects_image_guest_and_emulator(self):
        source = (boot.ROOT / "Makefile").read_text()
        begin = source.index("# Keep the image and emulator consistent with the two supported VM fixtures.")
        fragment = source[begin:source.index("# ---------------------------------------------------------------------------", begin)]
        env = {key: value for key, value in os.environ.items()
               if key not in ("MAKEFLAGS", "MFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "CARGO_MAKEFLAGS", "QEMU", "QEMU_DATA")}
        with tempfile.TemporaryDirectory(prefix="boot-root-target-") as tmp:
            makefile = Path(tmp) / "Makefile"
            makefile.write_text(f"include {boot.ROOT}/scripts/Kbuild.include\n"
                                f"srctree := {boot.ROOT}\nobjtree := {tmp}\n"
                                "PYTHON3 := printf 'BOOT_ARG %s\\n'\nQ := @\n" + fragment +
                                "\n.PHONY: rust-boot-test bzImage Image usr_gen_init_cpio\n"
                                "bzImage Image usr_gen_init_cpio:\n\t@printf 'PREREQUISITE %s\\n' '$@'\n")
            for arch, image, guest in (("x86", "bzImage", "x86_64"), ("arm64", "Image", "aarch64")):
                for override in (False, True):
                    command = [*shlex.split(os.environ.get("MAKE", "make")), "--no-print-directory", "-rR",
                               "-f", str(makefile), "SRCARCH=" + arch, "rust-boot-test"]
                    if override:
                        command += ["QEMU=custom-qemu --no-user-config", "QEMU_DATA=firmware's directory"]
                    result = subprocess.run(command, cwd=tmp, env=env, capture_output=True, timeout=30)
                    self.assertEqual(result.returncode, 0, result.stderr.decode(errors="replace"))
                    lines = result.stdout.decode().splitlines()
                    self.assertEqual([line for line in lines if line.startswith("PREREQUISITE ")],
                                     ["PREREQUISITE " + image, "PREREQUISITE usr_gen_init_cpio"])
                    args = [line.removeprefix("BOOT_ARG ") for line in lines if line.startswith("BOOT_ARG ")]
                    self.assertEqual(args[:5], [str(boot.ROOT / "scripts/tests/boot_kernel.py"), "--build", tmp,
                                                "--arch", guest])
                    self.assertEqual(args[5:7], ["--qemu", "custom-qemu --no-user-config" if override else "qemu-system-" + guest])
                    self.assertEqual(args[7:], ["--qemu-data", "firmware's directory"] if override else [])
            result = subprocess.run(["make", "--no-print-directory", "-rR", "-f", str(makefile),
                                     "SRCARCH=riscv", "rust-boot-test"], cwd=tmp, env=env,
                                    capture_output=True, timeout=30)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"supports only x86-64 and ARM64", result.stderr)
            self.assertNotIn(b"PREREQUISITE", result.stdout)
            self.assertNotIn(b"BOOT_ARG", result.stdout)


if __name__ == "__main__":
    unittest.main()
