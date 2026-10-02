# SPDX-License-Identifier: GPL-2.0-only
"""Guest architecture selection for the shared real-kernel boot fixture."""

from contextlib import redirect_stderr, redirect_stdout
import io
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import boot_kernel as boot
from kernel_console import REPLAY_MARKER


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
        for name in ("init_module", "delete_module", "uname"):
            generic = re.search(r"(?m)^#define __NR_" + name + r"\s+(\d+)\s*$", aarch64)
            native = re.search(r"(?m)^(\d+)\s+\w+\s+" + name + r"\s", x86)
            self.assertIsNotNone(generic)
            self.assertIsNotNone(native)
            for arch, number in (("aarch64", generic[1]), ("x86_64", native[1])):
                self.assertRegex(source, re.escape(f'#[cfg(target_arch = "{arch}")]') +
                                 r'\s+' + re.escape(f'const {name.upper()}: std::ffi::c_long = {number};'))

    def test_explicit_cpu_and_boot_options_reach_qemu_without_default_override(self):
        for arch in ("x86_64", "aarch64"):
            with self.subTest(arch=arch), tempfile.TemporaryDirectory(prefix="boot-options-") as tmp:
                build = Path(tmp)
                for name in (boot.architecture(arch)[0], "usr/gen_init_cpio"):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_bytes(b"must not execute")
                selected = "ARM64" if arch == "aarch64" else "X86_64"
                (build / ".config").write_text(f"CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_{selected}=y\n")
                args = ["boot_kernel", "--build", tmp, "--arch", arch, "--cpu", "max",
                        "--kernel-arg", "initcall_debug", "--kernel-arg", "randomize_kstack_offset=off"]
                with mock.patch.object(sys, "argv", args), \
                     mock.patch.object(boot.subprocess, "run"), \
                     mock.patch.object(boot.subprocess, "Popen", side_effect=RuntimeError("before QEMU")) as popen, \
                     self.assertRaisesRegex(RuntimeError, "before QEMU"):
                    boot.main()
                command = popen.call_args.args[0]
                self.assertEqual(command.count("-cpu"), 1)
                self.assertEqual(command[command.index("-cpu") + 1], "max")
                self.assertEqual(command.count("-append"), 1)
                arguments = command[command.index("-append") + 1].split()
                self.assertEqual(arguments[-2:], ["initcall_debug", "randomize_kstack_offset=off"])
                self.assertIn("rdinit=/init", arguments)

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

    def test_expected_init_errno_has_distinct_exact_events_and_reexecutes(self):
        for preloads in (0, 2):
            for reload in (False, True):
                events = [f"LUPOS_RUST_PRELOAD_OK {i}".encode() for i in range(preloads)]
                events += [b"LUPOS_RUST_MODULE_INIT_ERRNO_OK 11"]
                if reload:
                    events += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads))]
                    events += [f"LUPOS_RUST_MODULE_RELOAD_OK {i}".encode() for i in range(preloads)]
                    events += [b"LUPOS_RUST_MODULE_REINIT_ERRNO_OK 11"]
                events += [boot.MARKER]
                kwargs = dict(preloads=preloads, module=True, reload=reload, module_init_errno=11)
                good = b"\n".join(events) + b"\n"
                boot.verify_module_events(good, **kwargs)
                for index, event in enumerate(events):
                    for changed in (events[:index] + events[index + 1:],
                                    events[:index] + [event, event] + events[index + 1:]):
                        with self.subTest(preloads=preloads, reload=reload, index=index), self.assertRaises(ValueError):
                            boot.verify_module_events(b"\n".join(changed) + b"\n", **kwargs)
                for bad in (good.replace(b"ERRNO_OK 11", b"ERRNO_OK 12"),
                            good.replace(b"LUPOS_RUST_MODULE_INIT_ERRNO_OK 11", b"LUPOS_RUST_MODULE_LOAD_OK"),
                            good.replace(boot.MARKER, b"LUPOS_RUST_MODULE_UNLOAD_OK " + str(preloads).encode() + b"\n" + boot.MARKER)):
                    with self.assertRaises(ValueError):
                        boot.verify_module_events(bad, **kwargs)
                with self.assertRaises(ValueError):
                    boot.verify_module_events(good, preloads=preloads, module=True, reload=reload)

    def test_expected_init_errno_requires_module_and_valid_errno_before_writes(self):
        for extra in (["--module-init-errno", "11"],
                      *(["--module", "test.ko", "--module-init-errno", number] for number in ("0", "-11", "4096"))):
            with mock.patch.object(sys, "argv", ["boot_kernel", "--build", "/missing-build", *extra]), \
                 mock.patch.object(boot.subprocess, "run") as run, \
                 redirect_stderr(io.StringIO()) as error, self.assertRaises(SystemExit) as caught:
                boot.main()
            self.assertEqual(caught.exception.code, 2)
            self.assertIn("--module-init-errno requires", error.getvalue())
            run.assert_not_called()

    def test_expected_init_errno_manifest_excludes_failed_module_from_reload_plan(self):
        with tempfile.TemporaryDirectory(prefix="boot-init-errno-") as tmp:
            build = Path(tmp)
            for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio", "dependency.ko", "test.ko"):
                path = build / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b"must not execute")
            (build / ".config").write_text("CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_X86_64=y\n"
                                          "CONFIG_MODULES=y\nCONFIG_MODULE_UNLOAD=y\nCONFIG_PROC_FS=y\n")
            args = ["boot_kernel", "--build", tmp, "--module", str(build / "test.ko"),
                    "--preload-module", str(build / "dependency.ko"), "--reload-modules",
                    "--module-init-errno", "11"]
            with mock.patch.object(sys, "argv", args), mock.patch.object(boot.subprocess, "run"), \
                 mock.patch.object(boot, "module_name", side_effect=["dependency", "rbtree_test"]), \
                 mock.patch.object(boot.subprocess, "Popen", side_effect=RuntimeError("stop before QEMU")), \
                 self.assertRaisesRegex(RuntimeError, "stop before QEMU"):
                boot.main()
            work = build / "rust-boot-test"
            self.assertEqual((work / "module-init-errno").read_text(), "rbtree_test\t11\n")
            self.assertEqual((work / "reload-plan").read_text(), "/preload-module.0\tdependency\n")
            self.assertIn(f"file /module-init-errno {work}/module-init-errno 0600 0 0\n",
                          (work / "manifest").read_text())

    def test_extra_initramfs_entries_reach_generator_and_missing_input_fails_early(self):
        with tempfile.TemporaryDirectory(prefix="boot-extra-manifest-") as tmp:
            build = Path(tmp)
            for name in ("arch/x86/boot/bzImage", "usr/gen_init_cpio"):
                path = build / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b"must not execute")
            (build / ".config").write_text("CONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\nCONFIG_X86_64=y\n")
            extra = build / "extra.manifest"
            args = ["boot_kernel", "--build", tmp, "--extra-initramfs-manifest", str(extra)]
            with mock.patch.object(sys, "argv", args), \
                 mock.patch.object(boot.subprocess, "run") as run, \
                 mock.patch.object(boot.subprocess, "Popen") as popen, \
                 redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as caught:
                boot.main()
            self.assertEqual(caught.exception.code, 2)
            run.assert_not_called()
            popen.assert_not_called()
            self.assertFalse((build / "rust-boot-test").exists())
            extra.write_text("dir /extra-fixture 0755 0 0")
            with mock.patch.object(sys, "argv", args), \
                 mock.patch.object(boot.subprocess, "run") as run, \
                 mock.patch.object(boot.subprocess, "Popen", side_effect=RuntimeError("stop before QEMU")), \
                 self.assertRaisesRegex(RuntimeError, "stop before QEMU"):
                boot.main()
            manifest = build / "rust-boot-test/manifest"
            self.assertTrue(manifest.read_text().endswith("dir /extra-fixture 0755 0 0\n"))
            self.assertEqual(run.call_args_list[-1].args[0][-1], str(manifest))

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

    def test_exact_marked_replays_of_every_guest_event_are_accepted(self):
        for stamped in (False, True):
            events = [(b"[ 1.000000] " if stamped else b"") + event for event in self.events()]
            for index, event in enumerate(events):
                for length in range(1, len(event) + 1):
                    with self.subTest(stamped=stamped, index=index, length=length):
                        self.verify([*events[:index], event[:length], REPLAY_MARKER, *events[index:]])

    def test_replay_cannot_hide_mismatched_or_duplicate_guest_events(self):
        events = self.events()
        for index, event in enumerate(events):
            malformed = ([event, event], [b"wrong", REPLAY_MARKER, event],
                         [event, b" " + REPLAY_MARKER, event], [REPLAY_MARKER, event],
                         [event, REPLAY_MARKER], [event, REPLAY_MARKER, event, event])
            for replacement in malformed:
                with self.subTest(index=index, replacement=replacement), self.assertRaises(ValueError):
                    self.verify([*events[:index], *replacement, *events[index + 1:]])

    def test_failslab_setup_replay_preserves_exact_count_and_order(self):
        records = [boot.FAILSLAB_MARKER, b"LUPOS_RUST_MODULE_LOAD_OK", boot.MARKER]
        original = b"\n".join(records) + b"\n"
        for length in range(1, len(boot.FAILSLAB_MARKER) + 1):
            replay = b"\n".join([boot.FAILSLAB_MARKER[:length], REPLAY_MARKER, *records]) + b"\n"
            boot.verify_failslab_setup(replay, True)
            with self.assertRaises(ValueError): boot.verify_failslab_setup(replay, False)
        for changed in (boot.FAILSLAB_MARKER + b"\n" + original,
                        b"\n".join([records[1], records[0], records[2]]) + b"\n"):
            with self.assertRaises(ValueError): boot.verify_failslab_setup(changed, True)

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
        self.assertIn(b"23 passed; 0 failed", result.stdout)
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

    def test_cpuset_markers_require_both_original_tests_and_restored_state(self):
        records = [*boot.CPUSET_MARKERS, boot.MARKER]
        good = b'\n'.join(b'[ 1.123456] ' + line for line in records) + b'\n'
        boot.verify_cpuset_events(good, True)
        boot.verify_cpuset_events(boot.MARKER + b'\n', False)
        for index in range(len(records)):
            for changed in (records[:index] + records[index + 1:],
                            records[:index] + [records[index], records[index]] + records[index + 1:],
                            records[:index] + [records[index] + b' extra'] + records[index + 1:]):
                with self.subTest(index=index, changed=changed), self.assertRaises(ValueError):
                    boot.verify_cpuset_events(b'\n'.join(changed) + b'\n', True)
        for changed in (list(reversed(records)), [boot.MARKER, *boot.CPUSET_MARKERS],
                        [*records, b'LUPOS_CPUSET_V1_SKIPPED'], [*records, b'prefix ' + boot.CPUSET_MARKERS[0]]):
            with self.assertRaises(ValueError):
                boot.verify_cpuset_events(b'\n'.join(changed) + b'\n', True)
        with self.assertRaises(ValueError):
            boot.verify_cpuset_events(good, False)
        for index, record in enumerate(records):
            replay = [*records[:index], record[:5], REPLAY_MARKER, *records[index:]]
            boot.verify_cpuset_events(b'\n'.join(replay) + b'\n', True)

    def test_cpuset_scheduler_domain_prerequisite_and_explicit_core_topology(self):
        # The original base script writes sched_relax_domain_level=2. A
        # singleton SMT level below the package supplies domain level >=1;
        # two hardware threads of one core would not satisfy that premise.
        self.assertIn('SCHED_SMT', boot.CPUSET_CONFIG)
        for arch in ('x86_64', 'aarch64'):
            ordinary = boot.qemu_command(None, arch, Path('/image'), Path('/archive'))
            selected = boot.qemu_command(None, arch, Path('/image'), Path('/archive'), cpuset=True)
            self.assertEqual(ordinary.count('-smp'), 1)
            position = ordinary.index('-smp') + 1
            self.assertEqual(ordinary[position], '2')
            self.assertEqual(selected[position], '2,sockets=1,cores=2,threads=1')
            self.assertEqual(ordinary[:position], selected[:position])
            self.assertEqual(ordinary[position + 1:], selected[position + 1:])

    def test_cpuset_prerequisites_and_arm_userspace_fail_before_output_creation(self):
        for arch, missing in [('aarch64', None)] + [('x86_64', name) for name in boot.CPUSET_CONFIG]:
            with self.subTest(arch=arch, missing=missing), tempfile.TemporaryDirectory(prefix='boot-cpuset-config-') as tmp:
                build = Path(tmp)
                for name in (boot.architecture(arch)[0], 'usr/gen_init_cpio'):
                    artifact = build / name
                    artifact.parent.mkdir(parents=True, exist_ok=True)
                    artifact.write_bytes(b'must not execute')
                configuration = ['MULTIUSER', 'PRINTK', 'ARM64' if arch == 'aarch64' else 'X86_64']
                configuration += [name for name in boot.CPUSET_CONFIG if name != missing]
                (build / '.config').write_text(''.join('CONFIG_' + name + '=y\n' for name in configuration))
                error = io.StringIO()
                with mock.patch.object(sys, 'argv', ['boot_kernel', '--build', tmp, '--arch', arch, '--cpuset-v1-selftests']), \
                     mock.patch.object(boot.subprocess, 'run') as run, \
                     mock.patch.object(boot.subprocess, 'Popen') as popen, \
                     mock.patch.object(boot, 'cpuset_userspace') as userspace, \
                     redirect_stderr(error), self.assertRaises(SystemExit) as caught:
                    boot.main()
                self.assertEqual(caught.exception.code, 2)
                self.assertIn('ARM userspace' if arch == 'aarch64' else 'CONFIG_' + missing + '=y', error.getvalue())
                run.assert_not_called()
                popen.assert_not_called()
                userspace.assert_not_called()
                self.assertFalse((build / 'rust-boot-test').exists())

    @unittest.skipUnless(os.uname().machine == 'x86_64' and Path('/usr/bin/busybox').exists(),
                         'matching local x86 Bash and static BusyBox are required')
    def test_cpuset_stages_verified_original_script_bytes_and_runtime_hashes(self):
        files = boot.cpuset_userspace('x86_64')
        guests = [guest for guest, _, _ in files]
        for name in ('/bin/bash', '/bin/busybox', '/cpuset-tests/test_cpuset_v1_base.sh',
                     '/cpuset-tests/test_cpuset_v1_hp.sh'):
            self.assertIn(name, guests)
        with tempfile.TemporaryDirectory(prefix='boot-cpuset-runtime-') as tmp:
            work = Path(tmp)
            manifest = boot.cpuset_manifest(work, files)
            provenance = json.loads((work / 'cpuset-inputs/provenance.json').read_text())
            for index, (guest, source, data) in enumerate(files):
                self.assertEqual(data, source.read_bytes())
                self.assertEqual((work / 'cpuset-inputs' / str(index)).read_bytes(), data)
                self.assertEqual(provenance[index], dict(guest=guest, source=str(source),
                                                        sha256=hashlib.sha256(data).hexdigest()))
            for name in boot.CPUSET_APPLETS:
                self.assertIn(f'slink /bin/{name} /bin/busybox 0777 0 0\n', manifest)
            self.assertIn('nod /dev/null 0666 0 0 c 1 3\n', manifest)
            self.assertIn('slink /etc/mtab /proc/mounts 0777 0 0\n', manifest)
            self.assertEqual((work / 'cpuset-inputs/setup').read_bytes(), b'cpuset-v1\n')
            # Staging consumes already verified bytes, even if an input file
            # changes after preflight. The original tests are never patched.
            self.assertEqual([line.split()[1] for line in manifest.splitlines() if line.startswith('file /cpuset-tests/')],
                             ['/cpuset-tests/test_cpuset_v1_base.sh', '/cpuset-tests/test_cpuset_v1_hp.sh'])

    def test_cpuset_elf_validation_rejects_wrong_target_and_truncated_inputs(self):
        with self.assertRaisesRegex(ValueError, '--cpuset-userspace-root'):
            boot.cpuset_userspace('aarch64')
        data = Path('/bin/bash').read_bytes()
        for broken in (b'', data[:63], b'not-ELF' + data[7:], data[:18] + b'\xb7\x00' + data[20:]):
            with self.assertRaises(ValueError):
                boot.cpuset_interpreter(broken, 'x86_64')
        broken = bytearray(data)
        broken[32:40] = b'\xff' * 8
        with self.assertRaisesRegex(ValueError, 'program headers'):
            boot.cpuset_interpreter(broken, 'x86_64')

    @unittest.skipUnless(os.environ.get('CPUSET_AARCH64_USERSPACE_ROOT'),
                         'CPUSET_AARCH64_USERSPACE_ROOT supplies target Bash/BusyBox/runtime')
    def test_cpuset_cross_root_merged_usr_dependencies_are_complete_without_target_execution(self):
        root = Path(os.environ['CPUSET_AARCH64_USERSPACE_ROOT']).resolve()
        actual_run = subprocess.run
        with mock.patch.object(boot.subprocess, 'run', wraps=actual_run) as run:
            files = boot.cpuset_userspace('aarch64', userspace_root=root)
        self.assertTrue(run.call_args_list)
        self.assertEqual({call.args[0][0] for call in run.call_args_list}, {'readelf'})
        guests = {guest: (source, data) for guest, source, data in files}
        self.assertEqual(set(guests), {
            '/bin/bash', '/bin/busybox', '/lib/ld-linux-aarch64.so.1',
            '/cpuset-runtime/ld-linux-aarch64.so.1', '/cpuset-runtime/libc.so.6',
            '/cpuset-runtime/libtinfo.so.6', '/cpuset-tests/test_cpuset_v1_base.sh',
            '/cpuset-tests/test_cpuset_v1_hp.sh'})
        for guest, (source, data) in guests.items():
            if not guest.startswith('/cpuset-tests/'):
                self.assertTrue(source.is_relative_to(root))
                boot.cpuset_interpreter(data, 'aarch64')
            self.assertEqual(data, source.read_bytes())
        with tempfile.TemporaryDirectory(prefix='boot-cpuset-cross-') as tmp:
            work = Path(tmp)
            manifest = boot.cpuset_manifest(work, files)
            self.assertIn('dir /lib 0755 0 0\n', manifest)
            self.assertIn('file /lib/ld-linux-aarch64.so.1 ', manifest)
            records = json.loads((work / 'cpuset-inputs/provenance.json').read_text())
            self.assertEqual({record['guest'] for record in records}, set(guests))
            for record in records:
                self.assertEqual(record['sha256'], hashlib.sha256(guests[record['guest']][1]).hexdigest())

    @unittest.skipUnless(os.environ.get('CPUSET_AARCH64_USERSPACE_ROOT'),
                         'CPUSET_AARCH64_USERSPACE_ROOT supplies target Bash/BusyBox/runtime')
    def test_cpuset_cross_root_rejects_missing_wrong_target_and_escaping_dependencies(self):
        source = Path(os.environ['CPUSET_AARCH64_USERSPACE_ROOT']).resolve()
        with tempfile.TemporaryDirectory(prefix='boot-cpuset-cross-invalid-') as tmp:
            root = Path(tmp) / 'root'
            (root / 'usr/bin').mkdir(parents=True)
            for name in ('bash', 'busybox'):
                shutil.copyfile(source / 'usr/bin' / name, root / 'usr/bin' / name)
            shutil.copytree(source / 'usr/lib', root / 'usr/lib', symlinks=True)
            # A DT_NEEDED dependency is essential even though the ELF loader exists.
            libc = root / 'usr/lib/aarch64-linux-gnu/libc.so.6'
            original = libc.read_bytes()
            libc.unlink()
            with self.assertRaisesRegex(ValueError, 'unresolved.*libc.so.6'):
                boot.cpuset_userspace('aarch64', userspace_root=root)
            wrong = bytearray(original)
            wrong[18:20] = b'\x3e\x00'
            libc.write_bytes(wrong)
            with self.assertRaisesRegex(ValueError, 'actual aarch64 ELF'):
                boot.cpuset_userspace('aarch64', userspace_root=root)
            libc.unlink()
            libc.symlink_to(source / 'usr/lib/aarch64-linux-gnu/libc.so.6')
            with self.assertRaisesRegex(ValueError, 'escapes supplied root'):
                boot.cpuset_userspace('aarch64', userspace_root=root)
            libc.unlink()
            libc.write_bytes(original)
            box = root / 'usr/bin/busybox'
            original_box = box.read_bytes()
            shutil.copyfile(root / 'usr/lib/aarch64-linux-gnu/libtinfo.so.6', box)
            with self.assertRaisesRegex(ValueError, 'static BusyBox'):
                boot.cpuset_userspace('aarch64', userspace_root=root)
            box.write_bytes(original_box)
            (root / 'usr/lib/ld-linux-aarch64.so.1').unlink()
            with self.assertRaisesRegex(FileNotFoundError, 'ld-linux-aarch64'):
                boot.cpuset_userspace('aarch64', userspace_root=root)

    def test_cpuset_explicit_root_cli_is_opt_in_and_forwarded_for_both_guests(self):
        with mock.patch.object(sys, 'argv', ['boot_kernel', '--build', '/missing', '--cpuset-userspace-root', '/target']), \
             redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            boot.main()
        for arch in ('x86_64', 'aarch64'):
            with self.subTest(arch=arch), tempfile.TemporaryDirectory(prefix='boot-cpuset-root-cli-') as tmp:
                build = Path(tmp)
                for name in (boot.architecture(arch)[0], 'usr/gen_init_cpio'):
                    artifact = build / name
                    artifact.parent.mkdir(parents=True, exist_ok=True)
                    artifact.write_bytes(b'must not execute')
                settings = ['MULTIUSER', 'PRINTK', 'ARM64' if arch == 'aarch64' else 'X86_64', *boot.CPUSET_CONFIG]
                (build / '.config').write_text(''.join('CONFIG_' + name + '=y\n' for name in settings))
                argv = ['boot_kernel', '--build', tmp, '--arch', arch, '--cpuset-v1-selftests',
                        '--cpuset-userspace-root', '/target']
                with mock.patch.object(sys, 'argv', argv), redirect_stderr(io.StringIO()), \
                     mock.patch.object(boot, 'cpuset_userspace', side_effect=ValueError('preflight only')) as userspace, \
                     mock.patch.object(boot.subprocess, 'run') as run, self.assertRaises(SystemExit):
                    boot.main()
                userspace.assert_called_once_with(arch, userspace_root=Path('/target'))
                run.assert_not_called()
                self.assertFalse((build / 'rust-boot-test').exists())

    def test_cpuset_mode_is_opt_in_and_bad_userspace_fails_before_writes(self):
        for requested in (False, True):
            with self.subTest(requested=requested), tempfile.TemporaryDirectory(prefix='boot-cpuset-opt-in-') as tmp:
                build = Path(tmp)
                for name in ('arch/x86/boot/bzImage', 'usr/gen_init_cpio'):
                    artifact = build / name
                    artifact.parent.mkdir(parents=True, exist_ok=True)
                    artifact.write_bytes(b'must not execute')
                (build / '.config').write_text('CONFIG_X86_64=y\nCONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\n' +
                    ''.join('CONFIG_' + name + '=y\n' for name in boot.CPUSET_CONFIG))
                argv = ['boot_kernel', '--build', tmp] + (['--cpuset-v1-selftests'] if requested else [])
                with mock.patch.object(sys, 'argv', argv), redirect_stderr(io.StringIO()), \
                     mock.patch.object(boot, 'cpuset_userspace', side_effect=ValueError('invalid userspace')) as userspace, \
                     mock.patch.object(boot.subprocess, 'run') as run, \
                     mock.patch.object(boot.subprocess, 'Popen', side_effect=RuntimeError('before QEMU')):
                    with self.assertRaises(SystemExit if requested else RuntimeError):
                        boot.main()
                if requested:
                    userspace.assert_called_once_with('x86_64', userspace_root=None)
                    run.assert_not_called()
                    self.assertFalse((build / 'rust-boot-test').exists())
                else:
                    userspace.assert_not_called()
                    guests = [line.split()[1] for line in (build / 'rust-boot-test/manifest').read_text().splitlines()]
                    self.assertFalse(any(name.startswith('/cpuset') for name in guests))


class LegacyInitrdTests(unittest.TestCase):
    def archive(self, work):
        generator = work / 'gen_init_cpio'
        subprocess.run([*shlex.split(os.environ.get('HOSTCC', 'cc')), '-O2',
                        boot.ROOT / 'usr/gen_init_cpio.c', '-o', generator], check=True, capture_output=True)
        payload = work / 'payload'
        payload.write_bytes(bytes(range(256)) + b'\0binary\xff')
        manifest = work / 'manifest'
        manifest.write_text('dir /dev 0755 0 0\n'
            'nod /dev/console 0600 0 0 c 5 1\n'
            'nod /dev/kmsg 0600 0 0 c 1 11\n'
            'nod /dev/ram0 0600 0 0 b 1 0\n'
            f'file /fixture {payload} 0640 123 456 /hardlink\n'
            f'file /init {payload} 0755 0 0\n'
            'slink /symlink /fixture 0777 0 0\n'
            'pipe /fifo 0620 123 456\n'
            'sock /socket 0600 123 456\n')
        archive = work / 'initramfs.cpio'
        subprocess.run([generator, '-t', '0', '-c', '-o', archive, manifest], check=True, capture_output=True)
        return archive, payload

    def test_real_unprivileged_ext2_preserves_guest_fixture_metadata(self):
        if not all(shutil.which(tool) for tool in ('mkfs.ext2', 'debugfs', 'e2fsck')):
            self.skipTest('e2fsprogs is required for the real legacy initrd image check')
        with tempfile.TemporaryDirectory(prefix='legacy-initrd-image-') as tmp:
            work = Path(tmp)
            archive, payload = self.archive(work)
            image = work / 'initrd.ext2'
            with mock.patch.object(os, 'mknod', side_effect=AssertionError('host device creation is forbidden')):
                size = boot.legacy_initrd(archive, image)
            self.assertEqual(image.stat().st_size, size * 1024)
            self.assertGreater(size, 16 * 1024)
            checked = subprocess.run(['e2fsck', '-fn', image], capture_output=True)
            self.assertEqual(checked.returncode, 0, checked.stdout + checked.stderr)

            def inspect(request):
                return subprocess.run(['debugfs', '-R', request, image],
                                      check=True, capture_output=True).stdout

            regular = inspect('stat /fixture')
            link = inspect('stat /hardlink')
            self.assertEqual(re.search(rb'Inode: (\d+)', regular)[1], re.search(rb'Inode: (\d+)', link)[1])
            self.assertIn(b'Mode:  0640', regular)
            self.assertRegex(regular, rb'User:\s+123\s+Group:\s+456')
            self.assertIn(b'Links: 2', regular)
            self.assertIn(b'Mode:  0755', inspect('stat /init'))
            self.assertIn(b'Fast link dest: "/fixture"', inspect('stat /symlink'))
            for path, kind, device in (('/dev/console', b'character special', b'05:01'),
                                       ('/dev/kmsg', b'character special', b'01:11'),
                                       ('/dev/ram0', b'block special', b'01:00')):
                record = inspect('stat ' + path)
                self.assertIn(kind, record)
                self.assertIn(b'Mode:  0600', record)
                self.assertIn(b'Device major/minor number: ' + device, record)
            self.assertIn(b'Type: FIFO', inspect('stat /fifo'))
            self.assertIn(b'Type: socket', inspect('stat /socket'))
            dump = work / 'dump'
            for path in ('/fixture', '/hardlink', '/init'):
                inspect(f'dump {path} {dump}')
                self.assertEqual(dump.read_bytes(), payload.read_bytes())

    def test_shared_cpio_decoder_rejects_bounds_checksum_and_trailing_data(self):
        with tempfile.TemporaryDirectory(prefix='legacy-initrd-invalid-') as tmp:
            archive, _ = self.archive(Path(tmp))
            valid = archive.read_bytes()
            records = boot.initramfs_entries(valid)
            payload = next(offset for name, _, data, offset in records if name == b'hardlink' and data)
            checksum = bytearray(valid)
            checksum[payload] ^= 1
            name_size = bytearray(valid)
            name_size[94:102] = b'FFFFFFFF'
            for invalid in (b'', valid[:50], valid[:-1], b'bad' + valid[3:], bytes(checksum),
                            bytes(name_size), valid + bytes(511) + b'x'):
                with self.subTest(length=len(invalid)), self.assertRaises(ValueError):
                    boot.initramfs_entries(invalid)

    def test_legacy_config_and_argument_gates_precede_output_writes(self):
        base = ['CONFIG_X86_64=y', 'CONFIG_MULTIUSER=y', 'CONFIG_PRINTK=y', 'CONFIG_EXT2_FS=y',
                *['CONFIG_' + name + '=y' for name in boot.LEGACY_INITRD_CONFIG]]
        cases = [(set(base) - {setting}, []) for setting in base[3:]]
        cases += [(set(base) | {'CONFIG_INITRAMFS_FORCE=y'}, [])]
        cases += [(set(base), ['--kernel-arg=' + argument]) for argument in
                  ('root=/dev/sda', 'rdinit=/init', 'init=/bin/sh', 'noinitrd', 'ramdisk_size=1', 'ro', '--')]
        for config, extra in cases:
            with self.subTest(config=config, extra=extra), tempfile.TemporaryDirectory(prefix='legacy-initrd-gate-') as tmp:
                build = Path(tmp)
                for name in (boot.architecture('x86_64')[0], 'usr/gen_init_cpio'):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.touch()
                (build / '.config').write_text('\n'.join(config) + '\n')
                with mock.patch.object(sys, 'argv', ['boot_kernel', '--build', tmp, '--legacy-initrd', *extra]), \
                     mock.patch.object(boot.subprocess, 'run') as run, \
                     mock.patch.object(boot.subprocess, 'Popen') as popen, \
                     redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as result:
                    boot.main()
                self.assertEqual(result.exception.code, 2)
                run.assert_not_called()
                popen.assert_not_called()
                self.assertFalse((build / 'rust-boot-test').exists())

    def test_archive_paths_are_checked_before_image_or_tool_writes(self):
        with tempfile.TemporaryDirectory(prefix='legacy-initrd-paths-') as tmp:
            work = Path(tmp)
            archive, _ = self.archive(work)
            valid = archive.read_bytes()
            cases = [(b'fixture\0', name) for name in (b'../oops\0', b'no/file\0', b'fixtur"\0')]
            cases.append((b'hardlink\0', b'fixture\0\0'))
            for original, replacement in cases:
                with self.subTest(replacement=replacement):
                    archive.write_bytes(valid.replace(original, replacement, 1))
                    image = work / 'invalid.ext2'
                    with mock.patch.object(boot.subprocess, 'run') as run, self.assertRaises(ValueError):
                        boot.legacy_initrd(archive, image)
                    run.assert_not_called()
                    self.assertFalse(image.exists())

    def test_legacy_image_and_original_module_plan_reach_both_guests(self):
        for arch in ('x86_64', 'aarch64'):
            with self.subTest(arch=arch), tempfile.TemporaryDirectory(prefix='legacy-initrd-route-') as tmp:
                build = Path(tmp)
                for name in (boot.architecture(arch)[0], 'usr/gen_init_cpio', 'test.ko'):
                    path = build / name
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.touch()
                selected = 'ARM64' if arch == 'aarch64' else 'X86_64'
                (build / '.config').write_text(f'CONFIG_{selected}=y\nCONFIG_MULTIUSER=y\nCONFIG_PRINTK=y\n'
                    'CONFIG_MODULES=y\nCONFIG_MODULE_UNLOAD=y\nCONFIG_EXT4_FS=y\nCONFIG_EXT4_USE_FOR_EXT2=y\n' +
                    ''.join('CONFIG_' + name + '=y\n' for name in boot.LEGACY_INITRD_CONFIG))
                args = ['boot_kernel', '--build', tmp, '--arch', arch, '--legacy-initrd',
                        '--module', str(build / 'test.ko'), '--reload-modules', '--kernel-arg=initcall_debug']
                with mock.patch.object(sys, 'argv', args), mock.patch.object(boot.subprocess, 'run'), \
                     mock.patch.object(boot.shutil, 'which', return_value='/tools/present'), \
                     mock.patch.object(boot, 'module_name', return_value='test'), \
                     mock.patch.object(boot, 'legacy_initrd', return_value=49152) as generate, \
                     mock.patch.object(boot.subprocess, 'Popen', side_effect=RuntimeError('before QEMU')) as popen, \
                     self.assertRaisesRegex(RuntimeError, 'before QEMU'):
                    boot.main()
                work = build / 'rust-boot-test'
                generate.assert_called_once_with(work / 'initramfs.cpio', work / 'initrd.ext2')
                command = popen.call_args.args[0]
                self.assertEqual(command[command.index('-initrd') + 1], str(work / 'initrd.ext2'))
                options = command[command.index('-append') + 1].split()
                for option in ('rdinit=' + boot.LEGACY_RDINIT, 'root=/dev/ram0', 'rootfstype=ext2',
                               'rw', 'init=/init', 'ramdisk_size=49152', 'initcall_debug'):
                    self.assertIn(option, options)
                self.assertNotIn('rdinit=/init', options)
                self.assertEqual((work / 'reload-plan').read_text(), '/test-module.ko\ttest\n')
                self.assertIn('file /test-module.ko ', (work / 'manifest').read_text())

    def test_legacy_success_requires_load_mount_pivot_and_pid_one_in_order(self):
        events = [b'RAMDISK: ext2 filesystem found at block 0',
                  b'using deprecated initrd support, will be removed in January 2027; use initramfs instead',
                  b'VFS: Mounted root (ext2 filesystem) on device 1:0.',
                  b'VFS: Pivoted into new rootfs', b'Run /init as init process', boot.MARKER]
        console = b'\r\n'.join(b'[    1.234] ' + event for event in events)
        boot.verify_legacy_initrd(console, True)
        boot.verify_legacy_initrd(console.replace(b'ext2 filesystem)', b'ext4 filesystem)'), True)
        boot.verify_legacy_initrd(boot.MARKER, False)
        invalid = [events[:index] + events[index + 1:] for index in range(len(events))]
        invalid += [events + [events[2]], list(reversed(events)), events + [events[3] + b' malformed']]
        for lines in invalid:
            with self.subTest(lines=lines), self.assertRaises(ValueError):
                boot.verify_legacy_initrd(b'\n'.join(lines), True)
        for original, replacement in ((b'1:0', b'8:0'), (b'filesystem)', b'filesystem) readonly')):
            with self.subTest(replacement=replacement), self.assertRaises(ValueError):
                boot.verify_legacy_initrd(console.replace(original, replacement), True)


class RootDiskTests(unittest.TestCase):
    def configuration(self, arch):
        return ['CONFIG_MULTIUSER=y', 'CONFIG_PRINTK=y', 'CONFIG_EXT2_FS=y',
                'CONFIG_' + ('X86_64' if arch == 'x86_64' else 'ARM64') + '=y',
                *['CONFIG_' + name + '=y' for name in boot.ROOT_DISK_CONFIG],
                *(['CONFIG_PCI=y', 'CONFIG_VIRTIO_PCI=y'] if arch == 'x86_64'
                  else ['CONFIG_VIRTIO_MMIO=y'])]

    def artifacts(self, directory, arch, configuration):
        build = Path(directory)
        for name in (boot.architecture(arch)[0], 'usr/gen_init_cpio'):
            path = build / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.touch()
        (build / '.config').write_text('\n'.join(configuration) + '\n')
        return build

    def test_root_disk_requires_no_initramfs_and_builtin_block_transport(self):
        for arch in ('x86_64', 'aarch64'):
            good = self.configuration(arch)
            invalid = [good + ['CONFIG_BLK_DEV_INITRD=y']]
            invalid += [[setting for setting in good if setting != missing]
                        for missing in good if any(name in missing for name in ('BLOCK', 'VIRTIO', 'EXT2'))]
            for configuration in invalid:
                with self.subTest(arch=arch, configuration=configuration), tempfile.TemporaryDirectory() as directory:
                    build = self.artifacts(directory, arch, configuration)
                    with mock.patch.object(sys, 'argv', ['boot_kernel', '--build', directory, '--arch', arch, '--root-disk']), \
                         mock.patch.object(boot.subprocess, 'run') as run, \
                         mock.patch.object(boot.subprocess, 'Popen') as popen, \
                         redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as failure:
                        boot.main()
                    self.assertEqual(failure.exception.code, 2)
                    run.assert_not_called()
                    popen.assert_not_called()
                    self.assertFalse((build / 'rust-boot-test').exists())

    def test_initrd_enabled_disk_mode_requires_initrd_and_builtin_transport(self):
        for arch in ('x86_64', 'aarch64'):
            good = self.configuration(arch) + ['CONFIG_BLK_DEV_INITRD=y']
            invalid = [[setting for setting in good if setting != missing]
                       for missing in good if any(name in missing for name in ('BLK_DEV_INITRD', 'BLOCK', 'VIRTIO', 'EXT2'))]
            for configuration in invalid:
                with self.subTest(arch=arch, configuration=configuration), tempfile.TemporaryDirectory() as directory:
                    build = self.artifacts(directory, arch, configuration)
                    with mock.patch.object(sys, 'argv', ['boot_kernel', '--build', directory, '--arch', arch,
                                                        '--root-disk-initrd-enabled']), \
                         mock.patch.object(boot.subprocess, 'run') as run, \
                         mock.patch.object(boot.subprocess, 'Popen') as popen, \
                         redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as failure:
                        boot.main()
                    self.assertEqual(failure.exception.code, 2)
                    run.assert_not_called()
                    popen.assert_not_called()
                    self.assertFalse((build / 'rust-boot-test').exists())

    def test_initrd_enabled_disk_mode_preserves_transport_and_mode_exclusion(self):
        for arch, device in (('x86_64', 'virtio-blk-pci'), ('aarch64', 'virtio-blk-device')):
            with self.subTest(arch=arch), tempfile.TemporaryDirectory() as directory:
                configuration = self.configuration(arch) + ['CONFIG_BLK_DEV_INITRD=y']
                build = self.artifacts(directory, arch, configuration)
                with mock.patch.object(sys, 'argv', ['boot_kernel', '--build', directory, '--arch', arch,
                                                    '--root-disk-initrd-enabled', '--cpu', 'max']), \
                     mock.patch.object(boot.subprocess, 'run'), \
                     mock.patch.object(boot.shutil, 'which', return_value='/tools/present'), \
                     mock.patch.object(boot, 'legacy_initrd', return_value=32768) as generate, \
                     mock.patch.object(boot.subprocess, 'Popen', side_effect=RuntimeError('before QEMU')) as popen, \
                     self.assertRaisesRegex(RuntimeError, 'before QEMU'):
                    boot.main()
                work = build / 'rust-boot-test'
                generate.assert_called_once_with(work / 'initramfs.cpio', work / 'root.ext2')
                command = popen.call_args.args[0]
                self.assertNotIn('-initrd', command)
                self.assertEqual(command[command.index('-device') + 1], device + ',drive=lupos-root')
                self.assertEqual(command[command.index('-drive') + 1],
                                 f'file={work / "root.ext2"},format=raw,if=none,id=lupos-root,snapshot=on')
                options = command[command.index('-append') + 1].split()
                for option in ('root=/dev/vda', 'rootfstype=ext2', 'rw', 'init=/init',
                               'rdinit=' + boot.LEGACY_RDINIT):
                    self.assertIn(option, options)
                self.assertFalse(any(option.startswith('ramdisk_size=') for option in options))
        for other in ('--root-disk', '--legacy-initrd'):
            with self.subTest(other=other), \
                 mock.patch.object(sys, 'argv', ['boot_kernel', '--build', '/unused',
                                                '--root-disk-initrd-enabled', other]), \
                 mock.patch.object(boot.subprocess, 'run') as run, \
                 mock.patch.object(boot.subprocess, 'Popen') as popen, \
                 redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as failure:
                boot.main()
            self.assertEqual(failure.exception.code, 2)
            run.assert_not_called()
            popen.assert_not_called()

    def test_disk_image_reaches_matching_virtio_device_with_snapshot_and_no_initrd(self):
        for arch, device in (('x86_64', 'virtio-blk-pci'), ('aarch64', 'virtio-blk-device')):
            with self.subTest(arch=arch), tempfile.TemporaryDirectory() as directory:
                build = self.artifacts(directory, arch, self.configuration(arch))
                with mock.patch.object(sys, 'argv', ['boot_kernel', '--build', directory, '--arch', arch,
                                                    '--root-disk', '--cpu', 'max']), \
                     mock.patch.object(boot.subprocess, 'run'), \
                     mock.patch.object(boot.shutil, 'which', return_value='/tools/present'), \
                     mock.patch.object(boot, 'legacy_initrd', return_value=32768) as generate, \
                     mock.patch.object(boot.subprocess, 'Popen', side_effect=RuntimeError('before QEMU')) as popen, \
                     self.assertRaisesRegex(RuntimeError, 'before QEMU'):
                    boot.main()
                work = build / 'rust-boot-test'
                generate.assert_called_once_with(work / 'initramfs.cpio', work / 'root.ext2')
                command = popen.call_args.args[0]
                self.assertNotIn('-initrd', command)
                self.assertEqual(command[command.index('-device') + 1], device + ',drive=lupos-root')
                self.assertEqual(command[command.index('-drive') + 1],
                                 f'file={work / "root.ext2"},format=raw,if=none,id=lupos-root,snapshot=on')
                options = command[command.index('-append') + 1].split()
                for option in ('root=/dev/vda', 'rootfstype=ext2', 'rw', 'init=/init'):
                    self.assertIn(option, options)
                self.assertNotIn('rdinit=/init', options)
                self.assertFalse(any(option.startswith('ramdisk_size=') for option in options))
                self.assertEqual(command[command.index('-cpu') + 1], 'max')
        with self.assertRaises(ValueError):
            boot.qemu_command(None, 'x86_64', Path('/kernel'), Path('/image'), ramdisk_kib=1, root_disk=True)
        with self.assertRaises(ValueError):
            boot.qemu_command(None, 'x86_64', Path('/kernel'), Path('/image,option'), root_disk=True)

    def test_disk_boot_requires_device_mount_pivot_and_pid_one(self):
        events = [b'virtio_blk virtio0: [vda] 65536 512-byte logical blocks (33.6 MB/32.0 MiB)',
                  b'VFS: Mounted root (ext2 filesystem) on device 254:0.',
                  b'VFS: Pivoted into new rootfs', b'Run /init as init process', boot.MARKER]
        console = b'\n'.join(events)
        boot.verify_root_disk(console, True)
        boot.verify_root_disk(b'[1.234] ' + console.replace(b'\n', b'\r\n[1.234] '), True)
        boot.verify_root_disk(boot.MARKER, False)
        invalid = [events[:index] + events[index + 1:] for index in range(len(events))]
        invalid += [list(reversed(events)), events + [events[2]], events + [events[2] + b' malformed'],
                    [b'RAMDISK: ext2 filesystem found at block 0', *events]]
        for lines in invalid:
            with self.subTest(lines=lines), self.assertRaises(ValueError):
                boot.verify_root_disk(b'\n'.join(lines), True)
        for original, replacement in ((b'254:0', b'1:0'), (b'[vda]', b'[vdb]'), (b'512-byte', b'0-byte')):
            with self.subTest(replacement=replacement), self.assertRaises(ValueError):
                boot.verify_root_disk(console.replace(original, replacement), True)


if __name__ == "__main__":
    unittest.main()
