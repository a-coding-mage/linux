# SPDX-License-Identifier: GPL-2.0-only
"""Lossless handling of the kernel's explicitly marked console replays."""

REPLAY_MARKER = b"** replaying previous printk message **"


def normalize_console_transport(console):
    """Collapse only explicitly identified, matching single-line nbcon replays.

    nbcon_emit_next_record() (kernel/printk/nbcon.c) detects takeover of the
    same sequence, and console_prepend_replay() (kernel/printk/printk.c) prepends
    this exact, unstamped marker. Compare raw record bytes before stripping
    timestamps or caller prefixes: the interrupted output must be a nonempty
    prefix of the replayed, newline-terminated record. More complex takeover
    output (including multiline records) is deliberately not reconstructed.
    Ordinary duplicates and malformed marker-like output are never discarded.
    """
    records = console.splitlines(keepends=True)
    output = []
    index = 0
    while index < len(records):
        record = records[index]
        line = record.removesuffix(b"\n").removesuffix(b"\r")
        if b"replaying previous printk message" not in line:
            output.append(record)
            index += 1
            continue
        if line != REPLAY_MARKER or not record.endswith(b"\n") or not output or index + 1 >= len(records):
            raise ValueError("malformed or unpaired printk replay marker")
        previous = output[-1].removesuffix(b"\n").removesuffix(b"\r")
        following = records[index + 1]
        replay = following.removesuffix(b"\n").removesuffix(b"\r")
        if (not previous or not following.endswith(b"\n") or not replay.startswith(previous) or
                b"replaying previous printk message" in replay):
            raise ValueError("printk replay does not match the preceding complete/partial record")
        output[-1] = following
        index += 2
    return b"".join(output)
