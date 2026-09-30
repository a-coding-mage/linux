#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Boot an x86-64 or ARM64 kernel with generated RAM or disk root filesystems."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import shlex
import shutil
import stat
import struct
import subprocess
import sys
import time

from kernel_console import normalize_console_transport


ROOT = Path(__file__).resolve().parents[2]
MARKER = b"LUPOS_RUST_BUILD_BOOT_OK"
FAILSLAB_MARKER = b"LUPOS_FAILSLAB_SETUP_OK"
FAILSLAB_CONFIG = ("FAULT_INJECTION", "FAILSLAB", "FAULT_INJECTION_DEBUG_FS", "DEBUG_FS", "SYSFS")
CPUSET_CONFIG = ("SMP", "SCHED_SMT", "CGROUPS", "CPUSETS", "CPUSETS_V1", "PROC_PID_CPUSET",
                 "HOTPLUG_CPU", "PROC_FS", "SYSFS", "BINFMT_ELF")
CPUSET_APPLETS = ("awk", "cat", "grep", "head", "id", "mkdir", "mount", "rmdir", "sleep")
CPUSET_MARKERS = (
    b"LUPOS_CPUSET_V1_SETUP_OK cpus=0-1 mems=0 overlap=0-1 root_load_balance=0",
    b"LUPOS_CPUSET_V1_BASE_OK",
    b"LUPOS_CPUSET_V1_HOTPLUG_OK",
    b"LUPOS_CPUSET_V1_CLEANUP_OK cpu1=online",
)
LEGACY_INITRD_CONFIG = ("BLOCK", "BLK_DEV_INITRD", "BLK_DEV_RAM")
ROOT_DISK_CONFIG = ("BLOCK", "VIRTIO", "VIRTIO_BLK")
LEGACY_RDINIT = "/__lupos_missing_rdinit__"


def initramfs_entries(archive):
    """Decode the generator's newc/crc records with bounded names and payloads."""
    result = []
    offset = 0
    while True:
        header = archive[offset:offset + 110]
        if len(header) != 110 or header[:6] not in (b"070701", b"070702"):
            raise ValueError("invalid or truncated initramfs header")
        if not re.fullmatch(b"[0-9a-fA-F]{104}", header[6:]):
            raise ValueError("invalid initramfs header fields")
        fields = [int(header[6 + index * 8:14 + index * 8], 16) for index in range(13)]
        offset += 110
        name_size, data_size = fields[11], fields[6]
        name_bytes = archive[offset:offset + name_size]
        if not name_size or len(name_bytes) != name_size or not name_bytes.endswith(b"\0"):
            raise ValueError("invalid or truncated initramfs name")
        name, _, padding = name_bytes.partition(b"\0")
        if any(padding):
            raise ValueError("invalid initramfs name padding")
        offset = (offset + name_size + 3) & ~3
        data_offset = offset
        data = archive[offset:offset + data_size]
        if len(data) != data_size:
            raise ValueError("truncated initramfs payload")
        # gen_init_cpio emits CRCs for regular-file payloads only.
        if header[:6] == b"070702" and stat.S_ISREG(fields[1]) and sum(data) & 0xffffffff != fields[12]:
            raise ValueError("invalid initramfs file checksum")
        offset = (offset + data_size + 3) & ~3
        result.append((name, fields, data, data_offset))
        if name == b"TRAILER!!!":
            if len(archive) % 512 or offset > len(archive) or any(archive[offset:]):
                raise ValueError("invalid initramfs trailer padding")
            return result


def legacy_initrd(archive, image):
    """Populate ext2 directly from generated CPIO, without host mounts/mknod."""
    records = initramfs_entries(archive.read_bytes())[:-1]
    paths, regular = {}, {}

    def argument(value):
        # debugfs has its own command parser. The manifest uses whitespace-
        # delimited paths; reject metacharacters instead of inventing quoting.
        value = os.fsdecode(value)
        if not value or any(character.isspace() or character in '\\"\x00' for character in value):
            raise ValueError("unsupported legacy initrd path: " + repr(value))
        return value

    for name, fields, data, _ in records:
        path = "/" + argument(name).lstrip("/")
        if ".." in Path(path).parts or str(Path(path)) != path or path == LEGACY_RDINIT:
            raise ValueError("invalid legacy initrd destination: " + path)
        if path in paths and not (stat.S_ISDIR(fields[1]) and stat.S_ISDIR(paths[path][0][1])):
            raise ValueError("duplicate legacy initrd destination: " + path)
        paths[path] = fields, data
        if stat.S_ISREG(fields[1]):
            key = fields[7], fields[8], fields[0]
            regular.setdefault(key, []).append((path, fields, data))
        elif stat.S_ISLNK(fields[1]):
            argument(data.rstrip(b"\0"))
        elif stat.S_IFMT(fields[1]) not in (stat.S_IFDIR, stat.S_IFCHR, stat.S_IFBLK, stat.S_IFIFO, stat.S_IFSOCK):
            raise ValueError("unsupported legacy initrd inode type")
    for path in paths:
        for parent in Path(path).parents:
            if str(parent) != "/" and (str(parent) not in paths or not stat.S_ISDIR(paths[str(parent)][0][1])):
                raise ValueError("legacy initrd parent is not a directory: " + str(parent))
    for group in regular.values():
        _, first, _ = group[0]
        if len(group) != first[4] or any(fields[1:6] != first[1:6] for _, fields, _ in group):
            raise ValueError("inconsistent legacy initrd hardlink metadata")
        if len({data for _, _, data in group if data}) > 1:
            raise ValueError("inconsistent legacy initrd hardlink payloads")

    # Reserve space for indirect blocks, inode tables, extra directories and
    # writes made by the unchanged guest fixture after mounting the root rw.
    payload = sum(max(len(data) for _, _, data in group) for group in regular.values())
    size_kib = ((payload * 5 // 4 + len(paths) * 4096 + (16 << 20) + (1 << 20) - 1) >> 20) * 1024
    image.parent.mkdir(parents=True, exist_ok=True)
    with image.open("wb") as output:
        output.truncate(size_kib * 1024)
    environment = {**os.environ, "LC_ALL": "C"}
    subprocess.run(["mkfs.ext2", "-q", "-F", "-b", "1024", "-I", "256", "-O", "none", "-m", "0",
                    "-N", str(max(128, len(paths) * 2 + 32)), str(image)],
                   check=True, capture_output=True, env=environment)
    folder = image.parent / "legacy-initrd-inputs"
    folder.mkdir(exist_ok=True)
    empty = folder / "empty"
    empty.write_bytes(b"")
    commands = []
    for path, (fields, _) in sorted(paths.items(), key=lambda item: (item[0].count("/"), item[0])):
        if stat.S_ISDIR(fields[1]) and path != "/":
            commands.append("mkdir " + path)
    for index, group in enumerate(regular.values()):
        path, fields, _ = group[0]
        source = folder / str(index)
        source.write_bytes(max((data for _, _, data in group), key=len))
        commands.append(f"write {argument(str(source))} {path}")
        commands.extend(f"ln {path} {other}" for other, _, _ in group[1:])
        commands.append(f"sif {path} links_count {fields[4]}")
    for path, (fields, data) in paths.items():
        mode = fields[1]
        if stat.S_ISLNK(mode):
            commands.append(f"symlink {path} {argument(data.rstrip(bytes([0])))}")
        elif stat.S_ISCHR(mode) or stat.S_ISBLK(mode):
            parent, name = str(Path(path).parent), Path(path).name
            kind = "c" if stat.S_ISCHR(mode) else "b"
            commands.extend((f"cd {parent}", f"mknod {name} {kind} {fields[9]} {fields[10]}", "cd /"))
        elif stat.S_ISFIFO(mode) or stat.S_ISSOCK(mode):
            # With ext2's filetype feature disabled, the inode mode alone
            # identifies empty FIFO/socket objects; no host special file exists.
            commands.append(f"write {argument(str(empty))} {path}")
        for field, value in (("mode", f"0{mode:o}"), ("uid", fields[2]), ("gid", fields[3]),
                             ("atime", fields[5]), ("mtime", fields[5]), ("ctime", fields[5])):
            commands.append(f"sif {path} {field} {value}")
    batch = folder / "commands"
    batch.write_text("\n".join(commands) + "\n")
    result = subprocess.run(["debugfs", "-w", "-f", str(batch), str(image)],
                            check=True, capture_output=True, env=environment)
    # debugfs can report a failed command while returning status zero.
    if not re.fullmatch(rb"debugfs [^\n]+\n", result.stderr):
        raise ValueError("legacy initrd population failed: " + result.stderr.decode(errors="replace"))
    return size_kib


def verify_legacy_initrd(console, requested):
    """Require the legacy load and root pivot before the existing PID 1 checks."""
    if not requested:
        return
    patterns = (
        rb"RAMDISK: ext2 filesystem found at block 0",
        rb"using deprecated initrd support, will be removed in January 2027; .*",
        rb"VFS: Mounted root \((?:ext2|ext4) filesystem\) on device 1:0\.",
        rb"VFS: Pivoted into new rootfs",
        rb"Run /init as init process",
        MARKER,
    )
    prefixes = (b"RAMDISK: ext2 filesystem found", b"using deprecated initrd support",
                b"VFS: Mounted root", b"VFS: Pivoted into new rootfs", b"Run /init as init process", MARKER)
    verify_root_events(console, prefixes, patterns)


def verify_root_disk(console, requested):
    """Require virtio discovery and root handoff with no initrd boot path."""
    if not requested:
        return
    if b"RAMDISK: ext2 filesystem found" in console or b"using deprecated initrd support" in console:
        raise ValueError("disk-root boot unexpectedly used a legacy initrd")
    prefixes = (b"[vda]", b"VFS: Mounted root", b"VFS: Pivoted into new rootfs",
                b"Run /init as init process", MARKER)
    patterns = (
        rb"virtio_blk [^:]+: \[vda\] [1-9][0-9]* 512-byte logical blocks.*",
        rb"VFS: Mounted root \((?:ext2|ext4) filesystem\) on device (?!1:)[1-9][0-9]*:0\.",
        rb"VFS: Pivoted into new rootfs", rb"Run /init as init process", MARKER,
    )
    verify_root_events(console, prefixes, patterns)


def verify_root_events(console, prefixes, patterns):
    console = normalize_console_transport(console)
    events = []
    for line in console.splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip(), count=1)
        for index, (prefix, pattern) in enumerate(zip(prefixes, patterns)):
            if prefix in line:
                events.append(index if re.fullmatch(pattern, line) else -1)
    if events != list(range(len(patterns))):
        raise ValueError("guest did not complete root loading and namespace handoff")


def cpuset_interpreter(data, arch):
    """Validate the actual target ELF and return its bounded PT_INTERP."""
    machine = {"x86_64": 62, "aarch64": 183}.get(arch)
    if machine is None or len(data) < 64 or data[:6] != b"\x7fELF\x02\x01" or struct.unpack_from("<H", data, 18)[0] != machine:
        raise ValueError("cpuset userspace requires actual " + arch + " ELF files")
    if struct.unpack_from("<H", data, 16)[0] not in (2, 3):
        raise ValueError("cpuset userspace must be executable/shared ELF files")
    offset = struct.unpack_from("<Q", data, 32)[0]
    stride, count = struct.unpack_from("<HH", data, 54)
    if stride < 56 or offset > len(data) or count > (len(data) - offset) // stride:
        raise ValueError("truncated cpuset userspace program headers")
    interpreter = None
    for index in range(count):
        kind, _, start, _, _, size, _, _ = struct.unpack_from("<IIQQQQQQ", data, offset + index * stride)
        if start > len(data) or size > len(data) - start:
            raise ValueError("truncated cpuset userspace segment")
        if kind == 3:
            value = data[start:start + size]
            if interpreter is not None or not value.endswith(b"\0") or b"\0" in value[:-1]:
                raise ValueError("invalid cpuset userspace interpreter")
            interpreter = value[:-1].decode("ascii")
            if not interpreter.startswith("/") or any(c.isspace() for c in interpreter) or ".." in Path(interpreter).parts:
                raise ValueError("invalid cpuset userspace interpreter path")
    return interpreter


def cpuset_root_runtime(root, arch, add):
    """Resolve a supplied target root without executing any of its binaries."""
    root = Path(root).resolve(strict=True)
    if not root.is_dir():
        raise ValueError("cpuset userspace root is not a directory")

    def resolve(guest):
        choices = [guest]
        if guest.split('/')[1] in ('bin', 'sbin', 'lib', 'lib64'):
            choices.append('/usr' + guest)
        for name in choices:
            path = root / name.lstrip('/')
            if path.exists():
                path = path.resolve(strict=True)
                if not path.is_relative_to(root):
                    raise ValueError("cpuset userspace path escapes supplied root: " + guest)
                if path.is_file():
                    return path
        raise FileNotFoundError("cpuset userspace root lacks " + guest)

    multiarch = {'x86_64': 'x86_64-linux-gnu', 'aarch64': 'aarch64-linux-gnu'}[arch]
    search = ('/lib/' + multiarch, '/usr/lib/' + multiarch,
              '/lib64', '/usr/lib64', '/lib', '/usr/lib')
    queue = []
    static_box = None
    for guest in ('/bin/bash', '/bin/busybox'):
        path = resolve(guest)
        data = add(guest, path)
        interpreter = cpuset_interpreter(data, arch)
        if guest.endswith('/busybox') and interpreter is not None:
            raise ValueError("cpuset-v1 selftests require a static BusyBox")
        if guest.endswith('/busybox'):
            static_box = path
        queue.append((path, data))
        if interpreter:
            loader = resolve(interpreter)
            queue.append((loader, add(interpreter, loader)))
    inspected, staged = set(), set()
    while queue:
        path, data = queue.pop(0)
        if path in inspected:
            continue
        inspected.add(path)
        result = subprocess.run(['readelf', '--wide', '--dynamic', str(path)], check=True,
                                capture_output=True, timeout=10, env={**os.environ, 'LC_ALL': 'C'})
        if result.stderr or path.read_bytes() != data:
            raise ValueError("cpuset userspace ELF changed or has invalid dynamic metadata: " + str(path))
        for line in result.stdout.decode().splitlines():
            if '(NEEDED)' not in line:
                continue
            if path == static_box:
                raise ValueError("cpuset-v1 selftests require a static BusyBox")
            match = re.fullmatch(r'\s*0x[0-9a-f]+\s+\(NEEDED\)\s+Shared library: \[([^\]]+)\]\s*', line)
            if not match or not re.fullmatch(r'[A-Za-z0-9_+.-]+', match[1]) or match[1] in ('.', '..'):
                raise ValueError("invalid cpuset userspace DT_NEEDED entry: " + line)
            soname = match[1]
            if soname in staged:
                continue
            for directory in search:
                try:
                    dependency = resolve(directory + '/' + soname)
                    break
                except FileNotFoundError:
                    pass
            else:
                raise ValueError("unresolved cpuset userspace dependency: " + soname)
            staged.add(soname)
            queue.append((dependency, add('/cpuset-runtime/' + soname, dependency)))


def cpuset_userspace(arch, bash=Path("/bin/bash"), busybox=Path("/usr/bin/busybox"), userspace_root=None):
    """Verify target userspace before output writes; execute only native defaults."""
    if arch not in ('x86_64', 'aarch64'):
        raise ValueError("unsupported cpuset userspace architecture: " + arch)
    if userspace_root is None and (arch != "x86_64" or os.uname().machine not in ("x86_64", "amd64")):
        raise ValueError("cpuset-v1 selftests require --cpuset-userspace-root without a native x86-64 host and guest")
    files = []

    def add(guest, path):
        path = Path(path).resolve(strict=True)
        data = path.read_bytes()
        cpuset_interpreter(data, arch)
        files.append((guest, path, data))
        return data

    if userspace_root is not None:
        cpuset_root_runtime(userspace_root, arch, add)
        interpreter = None
    else:
        shell = add("/bin/bash", bash)
        interpreter = cpuset_interpreter(shell, arch)
        box = add("/bin/busybox", busybox)
        if cpuset_interpreter(box, arch) is not None:
            raise ValueError("cpuset-v1 selftests require a static BusyBox")
        applets = subprocess.run([str(busybox), "--list"], check=True, capture_output=True, timeout=10).stdout.splitlines()
        if not set(name.encode() for name in CPUSET_APPLETS) <= set(applets):
            raise ValueError("BusyBox lacks a required cpuset-v1 applet")
        version = subprocess.run([str(bash), "--version"], check=True, capture_output=True, timeout=10).stdout
        if not version.startswith(b"GNU bash, version "):
            raise ValueError("cpuset-v1 selftests require GNU Bash")
    if interpreter:
        add(interpreter, interpreter)
        result = subprocess.run(["ldd", str(bash)], check=True, capture_output=True, timeout=10,
                                env={**os.environ, "LC_ALL": "C"})
        if result.stderr or b"not found" in result.stdout:
            raise ValueError("Bash has unresolved runtime dependencies")
        for line in result.stdout.decode().splitlines():
            match = re.fullmatch(r"\s*(\S+) => (/\S+) \(0x[0-9a-f]+\)\s*", line)
            if match:
                soname, path = match.groups()
                if "/" in soname or soname in (".", ".."):
                    raise ValueError("invalid Bash runtime soname")
                add("/cpuset-runtime/" + soname, path)
            elif not re.fullmatch(r"\s*(?:linux-vdso\.so\.\d+|/\S+) \(0x[0-9a-f]+\)\s*", line):
                raise ValueError("unrecognized Bash runtime dependency: " + line)
    for name in ("test_cpuset_v1_base.sh", "test_cpuset_v1_hp.sh"):
        path = ROOT / "tools/testing/selftests/cgroup" / name
        data = path.read_bytes()
        if not data.startswith(b"#!/bin/bash\n"):
            raise ValueError("unexpected original cpuset-v1 script interpreter")
        files.append(("/cpuset-tests/" + name, path, data))
    if len({guest for guest, _, _ in files}) != len(files):
        raise ValueError("duplicate cpuset userspace destination")
    return files


def cpuset_manifest(work, files):
    """Stage exactly the verified bytes, recording sources and content hashes."""
    folder = work / "cpuset-inputs"
    folder.mkdir(exist_ok=True)
    directories = {"/bin", "/etc", "/cpuset-runtime", "/cpuset-tests"}
    for guest, _, _ in files:
        directories.update(str(parent) for parent in Path(guest).parents if str(parent) != "/")
    entries = ''.join(f"dir {directory} 0755 0 0\n" for directory in sorted(directories, key=lambda x: (x.count('/'), x)))
    provenance = []
    for index, (guest, source, data) in enumerate(files):
        output = folder / str(index)
        output.write_bytes(data)
        entries += f"file {guest} {output} 0755 0 0\n"
        provenance.append(dict(guest=guest, source=str(source), sha256=hashlib.sha256(data).hexdigest()))
    for name in CPUSET_APPLETS:
        entries += f"slink /bin/{name} /bin/busybox 0777 0 0\n"
    entries += "nod /dev/null 0666 0 0 c 1 3\nslink /etc/mtab /proc/mounts 0777 0 0\n"
    setup = folder / "setup"
    setup.write_bytes(b"cpuset-v1\n")
    entries += f"file /cpuset-v1-setup {setup} 0600 0 0\n"
    (folder / "provenance.json").write_text(json.dumps(provenance, indent=2) + "\n")
    return entries


def verify_cpuset_events(console, requested):
    """Reject absent, skipped, duplicate, malformed and late selftest records."""
    events = []
    for line in normalize_console_transport(console).splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip(), count=1)
        if b"LUPOS_CPUSET_" in line or MARKER in line:
            events.append(line)
    if events != [*(CPUSET_MARKERS if requested else ()), MARKER]:
        raise ValueError("guest did not complete original cpuset-v1 tests and cleanup in order")


def architecture(arch):
    """Return the image, Rust userspace target and isolated QEMU machine."""
    if arch == "x86_64":
        return ("arch/x86/boot/bzImage", "x86_64-unknown-linux-gnu",
                ["-machine", "pc"], "ttyS0")
    if arch == "aarch64":
        return ("arch/arm64/boot/Image", "aarch64-unknown-linux-musl",
                ["-machine", "virt", "-cpu", "cortex-a57"], "ttyAMA0")
    raise ValueError(f"unsupported guest architecture: {arch}")


def init_command(compiler, arch, output):
    """Compile the same PID 1 fixture for the actual guest, not the host."""
    _, target, _, _ = architecture(arch)
    command = [*compiler, "--edition=2021", "--target=" + target,
               "-Ctarget-feature=+crt-static", "-Cpanic=abort", "-Cstrip=debuginfo",
               "-O", "-Dwarnings", str(ROOT / "scripts/tests/boot_init.rs"),
               "-o", str(output)]
    if arch == "aarch64":
        # The matching rust-std target includes its musl CRT; Rust's bundled
        # LLD links it without a host-installed cross libc or GCC toolchain.
        command += ["-Clinker=rust-lld", "-Clink-self-contained=yes"]
    return command


def qemu_command(qemu, arch, kernel, archive, data=None, cpuset=False,
                 cpu=None, kernel_args=(), ramdisk_kib=None, root_disk=False):
    """Keep the emulated machine, kernel console and guest target consistent."""
    _, _, machine, console = architecture(arch)
    if cpu is not None:
        if "-cpu" in machine:
            machine[machine.index("-cpu") + 1] = cpu
        else:
            machine += ["-cpu", cpu]
    if root_disk and ramdisk_kib is not None:
        raise ValueError("root disk and legacy initrd are mutually exclusive")
    if root_disk and "," in str(archive):
        raise ValueError("root disk image path must not contain commas")
    root_args = (["rdinit=" + LEGACY_RDINIT, "root=/dev/vda", "rootfstype=ext2", "rw", "init=/init"]
                 if root_disk else
                 ["rdinit=" + LEGACY_RDINIT, "root=/dev/ram0", "rootfstype=ext2", "rw",
                  "init=/init", "ramdisk_size=" + str(ramdisk_kib)] if ramdisk_kib is not None
                 else ["rdinit=/init"])
    command_line = [f"console={console}", *root_args, "panic=-1", "nokaslr",
                    "printk.devkmsg=on", "loglevel=7", *kernel_args]
    command = shlex.split(qemu or "qemu-system-" + arch) + machine + [
        "-accel", "tcg", "-m", "256M", "-smp",
        "2,sockets=1,cores=2,threads=1" if cpuset else "2",
        "-display", "none", "-serial", "stdio", "-monitor", "none",
        "-nic", "none", "-no-reboot", "-kernel", str(kernel),
        "-append", " ".join(command_line)]
    if root_disk:
        transport = "virtio-blk-pci" if arch == "x86_64" else "virtio-blk-device"
        command += ["-drive", f"file={archive},format=raw,if=none,id=lupos-root,snapshot=on",
                    "-device", transport + ",drive=lupos-root"]
    else:
        command += ["-initrd", str(archive)]
    if data:
        command += ["-L", str(data.resolve())]
    return command


def module_name(path):
    """Read the actual module name, rather than guessing from a renamed .ko."""
    data = path.read_bytes()
    if len(data) < 64 or data[:4] != b"\x7fELF" or data[4] not in (1, 2) or data[5] not in (1, 2):
        raise ValueError(f"reload requires an uncompressed ELF module: {path}")
    order, wide = ("<" if data[5] == 1 else ">"), data[4] == 2
    offset = struct.unpack_from(order + ("Q" if wide else "I"), data, 40 if wide else 32)[0]
    stride, count, strings = struct.unpack_from(order + "HHH", data, 58 if wide else 46)
    fmt = order + ("IIQQQQIIQQ" if wide else "IIIIIIIIII")
    if stride < struct.calcsize(fmt) or offset > len(data) or struct.calcsize(fmt) > len(data) - offset:
        raise ValueError(f"invalid ELF section table: {path}")
    first = struct.unpack_from(fmt, data, offset)
    if count == 0:
        count = first[5]
    if strings == 0xffff:
        strings = first[6]
    if stride < struct.calcsize(fmt) or count > (len(data) - offset) // stride or strings >= count:
        raise ValueError(f"invalid ELF section table: {path}")
    sections = [struct.unpack_from(fmt, data, offset + i * stride) for i in range(count)]
    def section_bytes(section):
        start, size = section[4:6]
        if start > len(data) or size > len(data) - start:
            raise ValueError(f"truncated ELF section: {path}")
        return data[start:start + size]

    string_section = sections[strings]
    names = section_bytes(string_section)
    found = set()
    for section in sections:
        end = names.find(b"\0", section[0])
        if end == -1 or names[section[0]:end] != b".modinfo":
            continue
        info = section_bytes(section)
        if not info.endswith(b"\0"):
            raise ValueError(f"unterminated module metadata: {path}")
        for entry in info.split(b"\0"):
            if entry.startswith(b"name="):
                found.add(entry[5:])
    if len(found) != 1:
        raise ValueError(f"reload requires exactly one distinct .modinfo name: {path}")
    name = found.pop()
    if not name or any(byte not in b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-" for byte in name):
        raise ValueError(f"invalid module name in {path}")
    return name.decode("ascii").replace("-", "_")


def verify_module_events(console, *, preloads=0, module=False, rejected=0, reload=False):
    """Require requested actions, not just PID 1's final optional-fixture marker."""
    console = normalize_console_transport(console)
    expected = [f"LUPOS_RUST_MODULE_REJECT_OK {index}" for index in range(rejected)]
    expected += [f"LUPOS_RUST_PRELOAD_OK {index}" for index in range(preloads)]
    if module:
        expected.append("LUPOS_RUST_MODULE_LOAD_OK")
    if reload:
        count = preloads + int(module)
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {index}" for index in reversed(range(count))]
        expected += [f"LUPOS_RUST_MODULE_RELOAD_OK {index}" for index in range(count)]
    expected.append(MARKER.decode())
    prefixes = (b"LUPOS_RUST_MODULE_", b"LUPOS_RUST_PRELOAD_", MARKER)
    actual = []
    for line in console.splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip(), count=1)
        # Reserve every occurrence, not just intact prefixes. Interleaving or
        # extra decoration must fail even if a later clean marker also exists.
        if any(prefix in line for prefix in prefixes):
            actual.append(line)
    if actual != [line.encode() for line in expected]:
        raise ValueError("guest did not complete the requested module checks in order")


def verify_failslab_setup(console, requested):
    """Require one verified setup before any module action, only when requested."""
    console = normalize_console_transport(console)
    records = []
    for line in console.splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip(), count=1)
        if b"LUPOS_FAILSLAB_" in line or b"LUPOS_RUST_" in line:
            records.append(line)
    setups = [line for line in records if b"LUPOS_FAILSLAB_" in line]
    if setups != ([FAILSLAB_MARKER] if requested else []):
        raise ValueError("missing, duplicate, malformed or unrequested failslab setup")
    if requested and (not records or records[0] != FAILSLAB_MARKER):
        raise ValueError("failslab setup did not precede module actions")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", required=True, type=Path,
                        help="kernel output directory containing its boot image and gen_init_cpio")
    parser.add_argument("--arch", choices=("x86_64", "aarch64"), default="x86_64",
                        help="guest architecture (default: x86_64)")
    parser.add_argument("--qemu", default=os.environ.get("QEMU"),
                        help="QEMU command (defaults to QEMU or the guest's qemu-system binary)")
    parser.add_argument("--qemu-data", type=Path, help="optional QEMU firmware directory")
    parser.add_argument("--cpu", help="explicit QEMU CPU model, overriding the architecture default")
    parser.add_argument("--kernel-arg", action="append", default=[],
                        help="append a kernel boot argument (repeatable)")
    parser.add_argument("--timeout", type=float, default=60)
    parser.add_argument("--module", type=Path, help="optional uncompressed module to load inside the VM")
    parser.add_argument("--preload-module", type=Path, action="append", default=[],
                        help="load this dependency before --module, in argument order (repeatable)")
    parser.add_argument("--reload-modules", action="store_true",
                        help="unload successful loads in reverse order, then reload in original order")
    parser.add_argument("--reject-module", type=Path, action="append", default=[],
                        help="module that signature enforcement must reject inside the VM (repeatable)")
    parser.add_argument("--prepare-failslab", action="store_true",
                        help="prepare scoped FAILSLAB controls inside the isolated VM before module loading")
    parser.add_argument("--cpuset-v1-selftests", action="store_true",
                        help="run unchanged legacy cpuset base/hotplug selftests inside the guest")
    parser.add_argument("--cpuset-userspace-root", type=Path,
                        help="explicit target root containing Bash, static BusyBox and their ELF dependencies")
    parser.add_argument("--extra-initramfs-manifest", type=Path,
                        help="append fixture entries in gen_init_cpio manifest syntax")
    root_mode = parser.add_mutually_exclusive_group()
    root_mode.add_argument("--legacy-initrd", action="store_true",
                        help="boot the same fixtures from an ext2 RAM disk through prepare_namespace")
    root_mode.add_argument("--root-disk", action="store_true",
                          help="boot the same fixtures from a virtio disk with initramfs support disabled")
    args = parser.parse_args()
    if args.cpuset_userspace_root is not None and not args.cpuset_v1_selftests:
        parser.error("--cpuset-userspace-root requires --cpuset-v1-selftests")
    build = args.build.resolve()
    work = build / "rust-boot-test"
    image, _, _, _ = architecture(args.arch)
    kernel = build / image
    generator = build / "usr/gen_init_cpio"
    for artifact in (kernel, generator):
        if not artifact.is_file():
            parser.error(f"missing build artifact: {artifact}")
    configuration = (build / ".config").read_text().splitlines()
    required = ["CONFIG_MULTIUSER=y", "CONFIG_PRINTK=y",
                "CONFIG_ARM64=y" if args.arch == "aarch64" else "CONFIG_X86_64=y"]
    loaded = args.preload_module + ([args.module] if args.module else [])
    if loaded or args.reject_module:
        required.append("CONFIG_MODULES=y")
    if args.reload_modules:
        if not loaded:
            parser.error("--reload-modules requires --module or --preload-module")
        required.append("CONFIG_MODULE_UNLOAD=y")
    if args.reject_module:
        required.append("CONFIG_MODULE_SIG_FORCE=y")
    if args.prepare_failslab:
        required.extend("CONFIG_" + name + "=y" for name in FAILSLAB_CONFIG)
    if args.cpuset_v1_selftests:
        if args.arch != "x86_64" and args.cpuset_userspace_root is None:
            parser.error("cpuset-v1 selftests require --cpuset-userspace-root for ARM userspace")
        required.extend("CONFIG_" + name + "=y" for name in CPUSET_CONFIG)
    if args.legacy_initrd:
        required.extend("CONFIG_" + name + "=y" for name in LEGACY_INITRD_CONFIG)
        if "CONFIG_INITRAMFS_FORCE=y" in configuration:
            parser.error("--legacy-initrd requires CONFIG_INITRAMFS_FORCE to be disabled")
    if args.root_disk:
        required.extend("CONFIG_" + name + "=y" for name in ROOT_DISK_CONFIG)
        required.extend(["CONFIG_PCI=y", "CONFIG_VIRTIO_PCI=y"] if args.arch == "x86_64"
                        else ["CONFIG_VIRTIO_MMIO=y"])
        if "CONFIG_BLK_DEV_INITRD=y" in configuration:
            parser.error("--root-disk requires CONFIG_BLK_DEV_INITRD to be disabled")
        if "," in str(work):
            parser.error("--root-disk requires an output path without commas")
    if args.legacy_initrd or args.root_disk:
        option = "--root-disk" if args.root_disk else "--legacy-initrd"
        if not ("CONFIG_EXT2_FS=y" in configuration or
                {"CONFIG_EXT4_FS=y", "CONFIG_EXT4_USE_FOR_EXT2=y"} <= set(configuration)):
            parser.error(option + " requires built-in EXT2_FS or EXT4_FS with EXT4_USE_FOR_EXT2")
        reserved = {"root", "rootfstype", "init", "rdinit", "noinitrd", "ramdisk_size",
                    "ramdisk_start", "brd.rd_size", "initrd", "initrdmem", "ro", "rw", "--"}
        if any(token.split("=", 1)[0] in reserved for argument in args.kernel_arg for token in argument.split()):
            parser.error(option + " owns root, initrd and RAM disk boot arguments")
        for tool in ("mkfs.ext2", "debugfs"):
            if shutil.which(tool) is None:
                parser.error(option + " requires " + tool)
    for setting in required:
        if setting not in configuration:
            parser.error(f"the requested boot checks require {setting}")
    for artifact in loaded + args.reject_module:
        if not artifact.is_file():
            parser.error(f"missing module fixture: {artifact}")
        if any(character.isspace() for character in str(artifact.resolve())):
            parser.error("the initramfs manifest requires module paths without whitespace")
    if any(character.isspace() for character in str(work)):
        parser.error("the initramfs manifest requires an output path without whitespace")
    extra_entries = ""
    if args.extra_initramfs_manifest is not None:
        try:
            extra_entries = args.extra_initramfs_manifest.read_text()
        except (OSError, UnicodeError) as error:
            parser.error(str(error))
        if extra_entries and not extra_entries.endswith("\n"):
            extra_entries += "\n"
    reload_names = []
    if args.reload_modules:
        try:
            reload_names = [module_name(path) for path in loaded]
        except (OSError, ValueError, struct.error) as error:
            parser.error(str(error))
        if len(set(reload_names)) != len(reload_names):
            parser.error("reload module names must be distinct")
    userspace = None
    if args.cpuset_v1_selftests:
        try:
            userspace = cpuset_userspace(args.arch, userspace_root=args.cpuset_userspace_root)
        except (OSError, ValueError, struct.error, subprocess.SubprocessError) as error:
            parser.error(str(error))
    work.mkdir(parents=True, exist_ok=True)

    init = work / "init"
    compiler_env = os.environ.copy()
    # Python closes make's jobserver descriptors for subprocesses. The fixture
    # is a standalone compilation, so it must not advertise those descriptors.
    for name in ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS"):
        compiler_env.pop(name, None)
    subprocess.run(init_command(shlex.split(os.environ.get("HOSTRUSTC", "rustc")),
                                args.arch, init), check=True, env=compiler_env)
    fixture = work / "fixture"
    fixture.write_bytes(b"Rust-generated initramfs fixture\n\0with binary data\xff")
    manifest = work / "manifest"
    entries = (
        "dir /dev 0755 0 0\n"
        "nod /dev/console 0600 0 0 c 5 1\n"
        "nod /dev/kmsg 0600 0 0 c 1 11\n"
        f"file /init {init} 0755 0 0\n"
        f"file /fixture {fixture} 0640 123 456 /hardlink\n"
        "slink /symlink /fixture 0777 0 0\n")
    if userspace is not None:
        entries += cpuset_manifest(work, userspace)
    if args.module:
        entries += f"file /test-module.ko {args.module.resolve()} 0600 0 0\n"
    guest_paths = []
    if args.prepare_failslab:
        setup = work / "failslab-setup"
        stack_filter = int("CONFIG_FAULT_INJECTION_STACKTRACE_FILTER=y" in configuration)
        setup.write_text(f"failslab-v1\nstacktrace-filter={stack_filter}\n")
        entries += f"file /failslab-setup {setup} 0600 0 0\n"
    for index, module in enumerate(args.preload_module):
        guest_paths.append(f"/preload-module.{index}")
        entries += f"file {guest_paths[-1]} {module.resolve()} 0600 0 0\n"
    if args.module:
        guest_paths.append("/test-module.ko")
    if args.reload_modules:
        plan = work / "reload-plan"
        plan.write_text("".join(f"{path}\t{name}\n" for path, name in zip(guest_paths, reload_names)))
        entries += f"file /reload-plan {plan} 0600 0 0\n"
    for index, module in enumerate(args.reject_module):
        entries += f"file /reject-module.{index} {module.resolve()} 0600 0 0\n"
    entries += extra_entries
    manifest.write_text(entries)
    archive = work / "initramfs.cpio"
    subprocess.run([str(generator), "-t", "0", "-c", "-o", str(archive),
                    str(manifest)], check=True)
    ramdisk_kib = None
    if args.legacy_initrd or args.root_disk:
        image = work / ("root.ext2" if args.root_disk else "initrd.ext2")
        size_kib = legacy_initrd(archive, image)
        if args.legacy_initrd:
            ramdisk_kib = size_kib
        archive = image

    command = qemu_command(args.qemu, args.arch, kernel, archive, args.qemu_data,
                           cpuset=args.cpuset_v1_selftests, cpu=args.cpu,
                           kernel_args=args.kernel_arg, ramdisk_kib=ramdisk_kib,
                           root_disk=args.root_disk)
    log_path = work / "console.log"
    deadline = time.monotonic() + args.timeout
    found = False
    tail = b""
    with log_path.open("wb") as log, selectors.DefaultSelector() as selector:
        process = subprocess.Popen(command, stdout=subprocess.PIPE,
                                   stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
        try:
            selector.register(process.stdout, selectors.EVENT_READ)
            while time.monotonic() < deadline:
                ready = selector.select(timeout=min(1, max(0, deadline - time.monotonic())))
                if not ready:
                    if process.poll() is not None:
                        break
                    continue
                data = process.stdout.read1(65536)
                if not data:
                    break
                log.write(data)
                log.flush()
                tail = (tail + data)[-65536:]
                if MARKER + b"\n" in tail or MARKER + b"\r\n" in tail:
                    found = True
                    break
        finally:
            if process.poll() is None:
                process.terminate()
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
            process.stdout.close()
    if not found:
        print(tail.decode(errors="replace"), file=sys.stderr)
        raise SystemExit(f"kernel boot check failed; console output: {log_path}")
    try:
        verify_module_events(log_path.read_bytes(), preloads=len(args.preload_module),
                             module=args.module is not None, rejected=len(args.reject_module),
                             reload=args.reload_modules)
        verify_failslab_setup(log_path.read_bytes(), args.prepare_failslab)
        verify_cpuset_events(log_path.read_bytes(), args.cpuset_v1_selftests)
        verify_legacy_initrd(log_path.read_bytes(), args.legacy_initrd)
        verify_root_disk(log_path.read_bytes(), args.root_disk)
    except ValueError as error:
        raise SystemExit(f"{error}; console output: {log_path}") from error
    medium = ("ext2 root disk" if args.root_disk else "legacy ext2 initrd" if args.legacy_initrd
              else "Rust-generated initramfs")
    print(f"Kernel boot and {medium} checks passed; console: {log_path}")
    if args.cpuset_v1_selftests:
        print("Original cpuset-v1 base/hotplug tests and restored CPU/hierarchy state passed.")
    if args.prepare_failslab:
        print("Scoped FAILSLAB controls verified inside the VM before module loading.")
    if args.module:
        print("Module load passed inside the VM.")
    if args.preload_module:
        print(f"Loaded {len(args.preload_module)} prerequisite module(s) in order inside the VM.")
    if args.reload_modules:
        print("Module unload and reload passed inside the VM.")
    if args.reject_module:
        print(f"Signature enforcement rejected {len(args.reject_module)} module fixture(s) inside the VM.")


if __name__ == "__main__":
    main()
