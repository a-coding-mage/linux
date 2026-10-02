# SPDX-License-Identifier: GPL-2.0-only
"""Transport unchanged static test executables into the isolated boot guest."""

import hashlib
import json
from pathlib import Path
import re
import struct
import tempfile

from kernel_console import normalize_console_transport


def static_test_arch(data, arch, configuration):
    """Validate executable and program-header bounds without running host code."""
    if len(data) < 52 or data[:4] != b"\x7fELF" or data[4:7] not in (b"\x01\x01\x01", b"\x02\x01\x01"):
        raise ValueError("userspace tests require a little-endian static ELF executable")
    wide = data[4] == 2
    machine = struct.unpack_from("<H", data, 18)[0]
    actual = {(True, 62): "x86_64", (False, 3): "i686", (True, 183): "aarch64"}.get((wide, machine))
    if actual != arch and not (arch == "x86_64" and actual == "i686"):
        raise ValueError("userspace test architecture does not match the guest")
    if actual == "i686" and "CONFIG_IA32_EMULATION=y" not in configuration:
        raise ValueError("i686 userspace tests require CONFIG_IA32_EMULATION=y")
    if len(data) < (64 if wide else 52) or struct.unpack_from("<H", data, 16)[0] != 2:
        raise ValueError("userspace tests require statically linked ET_EXEC files")
    offset = struct.unpack_from("<Q" if wide else "<I", data, 32 if wide else 28)[0]
    stride, count = struct.unpack_from("<HH", data, 54 if wide else 42)
    minimum = 56 if wide else 32
    if not count or stride < minimum or offset > len(data) or count > (len(data) - offset) // stride:
        raise ValueError("truncated userspace test program headers")
    executable = False
    for index in range(count):
        values = struct.unpack_from("<IIQQQQQQ" if wide else "<IIIIIIII", data, offset + index * stride)
        if wide:
            kind, flags, start, _, _, size, memory, _ = values
        else:
            kind, start, _, _, size, memory, flags, _ = values
        if start > len(data) or size > len(data) - start or size > memory:
            raise ValueError("truncated userspace test segment")
        if kind in (2, 3):
            raise ValueError("userspace tests must not require a dynamic loader")
        executable |= kind == 1 and bool(flags & 1) and size > 0
    if not executable:
        raise ValueError("userspace test has no executable load segment")
    return actual


def prepare_userspace_tests(paths, arch, configuration):
    if len(paths) > 256:
        raise ValueError("at most 256 userspace tests may be requested")
    entries = []
    for path in paths:
        source = Path(path).resolve(strict=True)
        data = source.read_bytes()
        actual = static_test_arch(data, arch, configuration)
        entries.append((source, data, actual))
    return entries


def userspace_test_manifest(work, files):
    if not files:
        return ""
    folder = work / "userspace-inputs"
    folder.mkdir(exist_ok=True)
    entries = "dir /userspace-tests 0755 0 0\n"
    provenance = []
    for index, (source, data, arch) in enumerate(files):
        target = folder / str(index)
        target.write_bytes(data)
        guest = f"/userspace-tests/{index}"
        entries += f"file {guest} {target} 0755 0 0\n"
        provenance.append({"index": index, "source": str(source), "guest": guest,
                           "arch": arch, "size": len(data), "sha256": hashlib.sha256(data).hexdigest()})
    plan = work / "userspace-plan"
    plan.write_text(f"userspace-v1\n{len(files)}\n")
    entries += f"file /userspace-plan {plan} 0600 0 0\n"
    (work / "userspace-manifest.json").write_text(json.dumps(provenance, indent=2) + "\n")
    return entries


def userspace_test_results(console, count):
    """Decode lossless byte records and require every requested exit status."""
    events = []
    for line in normalize_console_transport(console).splitlines():
        line = re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip(), count=1)
        if b"LUPOS_USERSPACE_" in line:
            events.append(line)
    position, results = 0, []
    for index in range(count):
        if position >= len(events) or events[position] != f"LUPOS_USERSPACE_BEGIN {index}".encode():
            raise ValueError("missing or unordered userspace test begin record")
        position += 1
        streams = {}
        for stream in ("STDOUT", "STDERR"):
            data, sequence = bytearray(), 0
            prefix = f"LUPOS_USERSPACE_{stream} {index} ".encode()
            while position < len(events) and events[position].startswith(prefix):
                record = re.fullmatch(re.escape(prefix) + rb"(0|[1-9][0-9]*) ([0-9a-f]{2,512})", events[position])
                if not record or int(record[1]) != sequence or len(record[2]) % 2:
                    raise ValueError("malformed userspace output chunk")
                data.extend(bytes.fromhex(record[2].decode()))
                position += 1
                sequence += 1
            streams[stream.lower()] = bytes(data)
        if position >= len(events):
            raise ValueError("missing userspace test exit record")
        exit_record = re.fullmatch(f"LUPOS_USERSPACE_EXIT {index} ".encode() +
                                  rb"(0|[1-9][0-9]*) (0|[1-9][0-9]*) (0|[1-9][0-9]*)", events[position])
        if not exit_record:
            raise ValueError("malformed or unordered userspace test exit record")
        status, outlen, errlen = map(int, exit_record.groups())
        if status > 65535 or status & 255 == 127 or outlen != len(streams["stdout"]) or errlen != len(streams["stderr"]):
            raise ValueError("invalid userspace exit status or truncated output")
        position += 1
        results.append({"index": index, "status": status,
                        "exit_code": status >> 8 if status & 127 == 0 else None,
                        "signal": status & 127 or None, **streams})
    if position != len(events):
        raise ValueError("duplicate or unrequested userspace test records")
    return results


def save_userspace_results(work, results):
    records = []
    for result in results:
        record = {key: value for key, value in result.items() if key not in ("stdout", "stderr")}
        for stream in ("stdout", "stderr"):
            path = work / f"userspace-{result['index']}.{stream}"
            path.write_bytes(result[stream])
            record[stream] = path.name
        records.append(record)
    (work / "userspace-results.json").write_text(json.dumps(records, indent=2) + "\n")


def archive_previous_userspace_results(work):
    """Retain old decoded evidence without exposing it as a new run's result."""
    previous = [path for path in work.iterdir()
                if path.name == "userspace-results.json" or
                re.fullmatch(r"userspace-[0-9]+\.(?:stdout|stderr)", path.name)]
    if not previous:
        return None
    archive = Path(tempfile.mkdtemp(prefix="previous-userspace-results-", dir=work))
    for path in previous:
        path.rename(archive / path.name)
    return archive


def require_userspace_success(results):
    if any(result["status"] != 0 for result in results):
        raise ValueError("a requested userspace test failed, skipped, or terminated by signal")
