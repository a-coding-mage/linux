# SPDX-License-Identifier: GPL-2.0-only
"""Private glob callers: each language owns every loop and public call.

The retained kernel ABI calls the bounded export glob_match_len, not
glob_match_n. No C dispatcher, data generator or replacement algorithm is used.
"""
import ast
import ctypes
import itertools
import os
from pathlib import Path
import random
import re
import shlex
import struct


def compiler_environment(work, environment):
    temporary = Path(work) / "tmp"
    temporary.mkdir(parents=True, exist_ok=True)
    return {**environment, **{key: str(temporary) for key in ("TMPDIR", "TMP", "TEMP")}}


def response_flags(arguments, build, language, active=()):
    for flag in arguments:
        if not flag.startswith("@"):
            yield flag
            continue
        path = (build / flag[1:]).resolve()
        if path in active:
            raise ValueError("recursive native response file: " + str(path))
        text = path.read_text()
        nested = text.splitlines() if language == "rust" else shlex.split(text)
        yield from response_flags(nested, build, language, (*active, path))


def native_flags(arguments, build, language):
    """Keep saved compiler policy, resolving inputs against its original cwd.

    Expand response files before relocating paths: a response file can itself
    contain donor-relative inputs or output options. Rust response files have
    one argument per line, while C response files use shell-style quoting.
    The fixture supplies its own source and output arguments afterwards.
    """
    build = build.resolve()
    if language not in ("c", "rust"):
        raise ValueError("unknown native compiler language: " + language)

    def absolute(path):
        if not path:
            raise ValueError("empty native input path")
        return str(Path(path) if Path(path).is_absolute() else build / path)

    def include(path):
        # GCC/Clang expand these against --sysroot, not the command cwd.
        return path if path.startswith(("=", "$SYSROOT")) else absolute(path)

    def search(path):
        kind, separator, value = path.partition("=")
        if language == "rust" and separator:
            return kind + separator + absolute(value)
        return include(path) if language == "c" else absolute(path)

    def external(dependency):
        name, separator, value = dependency.partition("=")
        return name + separator + absolute(value) if separator else name

    def target(value):
        return absolute(value) if value.endswith(".json") or "/" in value else value

    inputs = {"-I": include, "-isystem": include, "-iquote": include,
              "-include": absolute, "-imacros": absolute, "-idirafter": include,
              "-include-pch": absolute, "-isysroot": absolute, "--sysroot": absolute,
              "-L": search, "--extern": external, "--target": target}
    joined_inputs = ("-include-pch", "-isysroot", "-isystem", "-iquote", "-idirafter", "-include", "-imacros", "-I", "-L")
    output_options = ("incremental=", "profile-generate=", "profile-dir=", "dump-mir-dir=", "self-profile=")
    flags, iterator = [], iter(response_flags(arguments, build, language))

    def argument(option):
        try:
            return next(iterator)
        except StopIteration:
            raise ValueError("missing native compiler argument for " + option) from None

    for flag in iterator:
        if flag in ("--out-dir", "--emit", "-o", "-MF", "-MT", "-MQ"):
            argument(flag)
        elif flag.startswith(("--out-dir=", "--emit=", "-Wp,-MMD,", "-Wp,-MD,", "-MF", "-MT", "-MQ")):
            continue
        elif flag.startswith("-o") and len(flag) > 2:
            continue
        elif flag in inputs:
            flags += [flag, inputs[flag](argument(flag))]
        elif flag.startswith(("--extern=", "--sysroot=", "--target=")):
            key, value = flag.split("=", 1)
            flags.append(key + "=" + inputs[key](value))
        elif any(flag.startswith(key) and flag != key for key in joined_inputs):
            key = next(key for key in joined_inputs if flag.startswith(key))
            flags.append(key + inputs[key](flag[len(key):]))
        elif flag in ("-C", "--codegen", "-Z"):
            value = argument(flag)
            if value.startswith(output_options):
                raise ValueError("native compiler output option cannot be replayed: " + flag + value)
            flags += [flag, value]
        elif any(flag.startswith(prefix + option) for prefix in ("-C", "--codegen=", "-Z") for option in output_options):
            raise ValueError("native compiler output option cannot be replayed: " + flag)
        elif flag.startswith(("-fprofile-instr-generate=", "-fprofile-generate=", "-fprofile-dir=",
                              "-fmodules-cache-path=", "-foptimization-record-file=", "-ftime-trace=",
                              "-serialize-diagnostics", "--serialize-diagnostics", "-dependency-file", "-MJ")):
            raise ValueError("native compiler output option cannot be replayed: " + flag)
        elif language == "rust" and not flag.startswith("-") and flag.endswith(".rs"):
            continue
        else:
            flags.append(flag)
    return flags


class NativeWriteWatch:
    """Record donor writes, including files deleted before a compiler exits."""
    # MODIFY, ATTRIB, CLOSE_WRITE, MOVED_FROM/TO, CREATE, DELETE, DELETE_SELF,
    # MOVE_SELF. Access/open events are intentionally excluded.
    MASK = 0x00000fce

    def __init__(self, root):
        self.root = root
        self.events = []

    def __enter__(self):
        libc = ctypes.CDLL(None, use_errno=True)
        libc.inotify_init1.argtypes = [ctypes.c_int]
        libc.inotify_init1.restype = ctypes.c_int
        libc.inotify_add_watch.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_uint32]
        libc.inotify_add_watch.restype = ctypes.c_int
        self.fd = libc.inotify_init1(os.O_NONBLOCK | os.O_CLOEXEC)
        if self.fd < 0:
            raise OSError(ctypes.get_errno(), "inotify_init1")
        self.paths = {}
        try:
            def walk_error(error):
                raise error
            for directory, _, _ in os.walk(self.root, onerror=walk_error):
                watch = libc.inotify_add_watch(self.fd, os.fsencode(directory), self.MASK)
                if watch < 0:
                    raise OSError(ctypes.get_errno(), "inotify_add_watch: " + directory)
                self.paths[watch] = Path(directory)
            if not self.paths:
                raise ValueError("missing donor for write observation: " + str(self.root))
        except BaseException:
            os.close(self.fd)
            raise
        return self

    def __exit__(self, *_):
        try:
            while True:
                try:
                    data = os.read(self.fd, 65536)
                except BlockingIOError:
                    break
                offset = 0
                while offset < len(data):
                    watch, mask, _, size = struct.unpack_from("iIII", data, offset)
                    offset += 16
                    name = os.fsdecode(data[offset:offset + size].split(b"\0", 1)[0])
                    offset += size
                    # Queue overflow and invalidated watches also fail the
                    # empty-event assertion; absence of evidence is not proof.
                    self.events.append((str(self.paths.get(watch, self.root) / name), hex(mask)))
        finally:
            os.close(self.fd)


def original_cases(root):
    source = (Path(root) / "lib/tests/glob_kunit.c").read_text()
    rows = re.findall(r'\{\s*\.pat = ("(?:[^"\\]|\\.)*"),\s*\.str = ("(?:[^"\\]|\\.)*"),\s*\.expected = (true|false)\s*\}', source)
    if len(rows) != 64 or re.findall(r'KUNIT_CASE_PARAM\((\w+),\s*(\w+)\)', source) != [("glob_test_match", "glob_gen_params")]:
        raise ValueError("original glob parameter corpus/registration changed")
    return [(ast.literal_eval(p).encode(), ast.literal_eval(s).encode(), e == "true") for p, s, e in rows]


def labels(root):
    return [b'pat:"' + p + b'" str:"' + s + b'"' for p, s, _ in original_cases(root)]


def cases(root):
    rows = [(p, s) for p, s, _ in original_cases(root)]
    alphabet = b'a![]-*?\\\x80\xff'
    inputs = (b'', b'a', b'b', b'[', b']', b'abc', b'aaa', b'\x80', b'\xff', b'a\0z')
    rows += [(bytes(p), s) for n in range(4) for p in itertools.product(alphabet, repeat=n) for s in inputs]
    rows += [(p, s) for p in (b'[!a-z]', b'[]a]', b'[a-]', b'[z-a]', b'[\x80-\xff]', b'*a*b*c', b'*[!a]?', b'a\\', b'[^a]', b'a\0*') for s in inputs]
    rng = random.Random(731)
    rows += [(bytes(rng.choices(alphabet, k=rng.randrange(24))), bytes(rng.choices(alphabet + b'\0', k=rng.randrange(24)))) for _ in range(4096)]
    return rows


def count(root):
    # Unbounded plus every inclusive bound, with exact unterminated copies.
    return sum(len(s) + 2 for _, s in cases(root))


def marker(root, caller):
    if caller not in ("c", "rust"):
        raise ValueError("invalid glob caller")
    return f"LUPOS_GLOB_{'RUST_ABI' if caller == 'rust' else 'ABI'}_OK cases={count(root)}".encode()


def reference_source(root):
    # Whole unchanged C translation unit. Only public names and incidental
    # metadata macros are private; algorithm and header types remain original.
    return '''#include <linux/module.h>
#include <linux/glob.h>
#include <linux/export.h>
bool glob_reference_match(const char *, const char *);
bool glob_reference_len(const char *, const char *, size_t);
#undef MODULE_DESCRIPTION
#undef MODULE_LICENSE
#undef EXPORT_SYMBOL
#define MODULE_DESCRIPTION(x)
#define MODULE_LICENSE(x)
#define EXPORT_SYMBOL(x)
#define glob_match glob_reference_match
#define glob_match_len glob_reference_len
#include "''' + str(Path(root) / 'lib/glob.c') + '''"
'''


C = r'''
#include <linux/glob.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/string.h>
bool glob_reference_match(const char *, const char *);
bool glob_reference_len(const char *, const char *, size_t);
static noinline bool glob_call_match(const char *p, const char *s)
{
    bool (*volatile actual)(const char *, const char *) = glob_match;
    return actual(p, s);
}
static noinline bool glob_call_len(const char *p, const char *s, size_t n)
{
    bool (*volatile actual)(const char *, const char *, size_t) = glob_match_len;
    return actual(p, s, n);
}
static const struct { unsigned char p[32], s[32]; size_t n; } cases[] = { @CASES@ };
static int glob_workload(void)
{
    size_t i, n, checks = 0;
    unsigned char bounded[32];
    for (i = 0; i < sizeof(cases)/sizeof(cases[0]); i++) {
        const char *p = (const char *)cases[i].p, *s = (const char *)cases[i].s;
        if (glob_call_match(p, s) != glob_reference_match(p, s)) return -EINVAL;
        checks++;
        for (n = 0; n <= cases[i].n; n++) {
            memset(bounded, 0xa5, sizeof(bounded));
            memcpy(bounded, s, n);
            if (glob_call_len(p, (char *)bounded, n) != glob_reference_len(p, (char *)bounded, n)) return -EINVAL;
            checks++;
        }
    }
    if (checks != @COUNT@) return -EINVAL;
    return 0;
}
#ifndef GLOB_HOST
static int __init glob_abi_init(void)
{
    int result = glob_workload();
    if (result) return result;
    pr_info("@MARKER@\n"); return 0;
}
static void __exit glob_abi_exit(void) {}
module_init(glob_abi_init);
module_exit(glob_abi_exit);
MODULE_LICENSE("@LICENSE@");
MODULE_DESCRIPTION("Independent glob native ABI test");
#else
int main(void) { return glob_workload() != 0; }
#endif
'''

RUST = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Complete independent Rust glob workload; C supplies only the oracle.
#[cfg(not(glob_host))]
use kernel::{bindings, ffi};
#[cfg(not(glob_host))]
use kernel::prelude::*;
#[cfg(glob_host)]
use core::ffi;
#[allow(dead_code, unreachable_pub)]
#[path = "@HEADER@"]
mod glob;
#[cfg(not(glob_host))]
const _: unsafe extern "C" fn(*const ffi::c_char, *const ffi::c_char) -> bool = bindings::glob_match;
#[cfg(not(glob_host))]
const _: unsafe extern "C" fn(*const ffi::c_char, *const ffi::c_char, usize) -> bool = bindings::glob_match_len;
unsafe extern "C" {
    fn glob_reference_match(p: *const ffi::c_char, s: *const ffi::c_char) -> bool;
    fn glob_reference_len(p: *const ffi::c_char, s: *const ffi::c_char, n: usize) -> bool;
}
/// Protected actual unbounded call.
/// # Safety
/// Both pointers satisfy the public terminated string contract.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn glob_call_match(p: *const ffi::c_char, s: *const ffi::c_char) -> bool {
    let actual: unsafe extern "C" fn(*const ffi::c_char, *const ffi::c_char) -> bool = glob::glob_match;
    unsafe { core::ptr::read_volatile(&actual)(p, s) }
}
/// Protected actual bounded call.
/// # Safety
/// Pattern is terminated; input is readable through its first NUL or n.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn glob_call_len(p: *const ffi::c_char, s: *const ffi::c_char, n: usize) -> bool {
    let actual: unsafe extern "C" fn(*const ffi::c_char, *const ffi::c_char, usize) -> bool = glob::glob_match_len;
    unsafe { core::ptr::read_volatile(&actual)(p, s, n) }
}
static CASES: &[(&[u8], &[u8], usize)] = &[@CASES@];
fn workload() -> bool {
    let mut checks = 0;
    for &(pattern, input, size) in CASES {
        let p = pattern.as_ptr().cast();
        let s = input.as_ptr().cast();
        // SAFETY: Static data has genuine terminators for the unbounded API.
        if unsafe { glob_call_match(p, s) != glob_reference_match(p, s) } { return false; }
        checks += 1;
        for n in 0..=size {
            let mut bounded = [0xa5u8; 32];
            bounded[..n].copy_from_slice(&input[..n]);
            let s = bounded.as_ptr().cast();
            // SAFETY: Every n-byte prefix is readable; no terminator required.
            if unsafe { glob_call_len(p, s, n) != glob_reference_len(p, s, n) } { return false; }
            checks += 1;
        }
    }
    checks == @COUNT@
}
#[cfg(glob_host)]
fn main() { assert!(workload()); }
#[cfg(not(glob_host))]
module! { type: GlobRustAbi, name: "glob_rust_abi", authors: ["Lupos"],
          description: "Independent glob native ABI test", license: "@LICENSE@", }
#[cfg(not(glob_host))]
struct GlobRustAbi;
#[cfg(not(glob_host))]
impl kernel::Module for GlobRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        if !workload() { return Err(EINVAL); }
        pr_info!("@MARKER@\n"); Ok(Self)
    }
}
#[cfg(not(glob_host))]
impl Drop for GlobRustAbi { fn drop(&mut self) {} }
'''


def caller_source(root, caller, license="GPL"):
    if caller not in ("c", "rust"):
        raise ValueError("invalid glob caller")
    if license not in ("GPL", "Proprietary") or caller == "rust" and license != "GPL":
        raise ValueError("Rust kernel support imports require GPL; public-license probe is C")
    def array(data):
        return ','.join(map(str, data + b'\0'))
    rows = cases(root)
    if any(max(len(p), len(s)) >= 32 for p, s in rows):
        raise ValueError("fixture buffer too small")
    values = (','.join('{{' + array(p) + '},{' + array(s) + '},' + str(len(s)) + '}' for p, s in rows) if caller == 'c' else
              ','.join('(&[' + array(p) + '],&[' + array(s) + '],' + str(len(s)) + ')' for p, s in rows))
    return (C if caller == 'c' else RUST).replace('@CASES@', values).replace('@COUNT@', str(count(root))).replace(
        '@MARKER@', marker(root, caller).decode()).replace('@HEADER@', str(Path(root) / 'include/linux/glob_header.rs')).replace('@LICENSE@', license)
