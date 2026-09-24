# SPDX-License-Identifier: GPL-2.0-only
"""Isolated native rbtree fixture transport; no donor writes."""
import ctypes
import os
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess

NATIVE_REQUIRED = ('rust/libkernel.rmeta', 'rust/libbindings.rmeta', 'rust/core.o',
    'rust/compiler_builtins.o', 'lib/.list_sort_rust.o.cmd', 'lib/.scatterlist.o.cmd',
    'rust/bindings/.bindings_generated.rs.cmd', 'rust/.bindings.o.cmd')


def explicit_path(value, name):
    if not str(value).strip() or '\0' in str(value):
        raise ValueError(name + ' must be a nonempty path without NUL')
    return Path(value).resolve()


def outside(path, roots):
    path = path.resolve()
    if any(path == root or root in path.parents for root in roots):
        raise ValueError('output/cwd must be outside source/native trees: ' + str(path))
    return path


def target_rlib(sysroot, crate):
    directory = sysroot / 'lib/rustlib/i686-unknown-linux-gnu/lib'
    found = [p for p in directory.glob('lib' + crate + '*.rlib')
             if re.fullmatch(r'lib' + re.escape(crate) + r'(?:-[0-9a-f]+)?\.rlib', p.name) and p.is_file()]
    if len(found) != 1:
        raise ValueError('i686 sysroot needs exactly one real ' + crate + ' rlib: ' + str(sysroot))
    return found[0]


def program(value, name, required):
    if not value.strip() or '\0' in value:
        raise ValueError(name + ' must name a nonempty executable without NUL')
    executable = shutil.which(value)
    if executable is None:
        if required:
            raise ValueError('explicit ' + name + ' is not executable')
        return None
    return Path(executable).absolute()


def validate_settings(root, environment):
    """Validate all explicit inputs before creating output or launching tools."""
    root = root.resolve()
    source = explicit_path(environment.get('RBTREE_SOURCE', root), 'RBTREE_SOURCE')
    for name in ('lib/rbtree.c', 'include/linux/rbtree.h', 'include/linux/rbtree_types.h',
                 'include/linux/rbtree_augmented.h'):
        if not (source / name).is_file():
            raise ValueError('RBTREE_SOURCE lacks original ' + name)
    for name in ('RBTREE_NATIVE', 'NATIVE_RUST_KERNEL_BUILD', 'RBTREE_ARM64',
                 'RBTREE_TEST_ARTIFACTS', 'RBTREE_I686_CORE', 'INT_MATH_I686_SYSROOT'):
        if name in environment:
            explicit_path(environment[name], name)
    native_name = 'RBTREE_NATIVE' if 'RBTREE_NATIVE' in environment else 'NATIVE_RUST_KERNEL_BUILD'
    native = explicit_path(environment.get(native_name, '/nonexistent-rbtree-native'), native_name)
    arm = explicit_path(environment['RBTREE_ARM64'], 'RBTREE_ARM64') if 'RBTREE_ARM64' in environment else None
    supplied_builds = [explicit_path(environment[name], name) for name in
        ('RBTREE_NATIVE', 'NATIVE_RUST_KERNEL_BUILD', 'RBTREE_ARM64') if name in environment]
    for name in ('RBTREE_NATIVE', 'NATIVE_RUST_KERNEL_BUILD', 'RBTREE_ARM64'):
        if name not in environment:
            continue
        build = explicit_path(environment[name], name)
        if not all((build / item).is_file() for item in NATIVE_REQUIRED):
            raise ValueError('explicit ' + name + ' lacks actual native metadata/commands: ' + str(build))
    parent = explicit_path(environment.get('RBTREE_TEST_ARTIFACTS', '/tmp'), 'RBTREE_TEST_ARTIFACTS')
    tools = {name: program(environment.get(name, default), name, name in environment)
             for name, default in (('HOSTRUSTC', 'rustc'), ('BINDGEN', 'bindgen'))}
    sysroot_name = 'RBTREE_I686_CORE' if 'RBTREE_I686_CORE' in environment else 'INT_MATH_I686_SYSROOT'
    sysroot = explicit_path(environment[sysroot_name], sysroot_name) if sysroot_name in environment else None
    supplied_sysroots = []
    for name in ('RBTREE_I686_CORE', 'INT_MATH_I686_SYSROOT'):
        if name in environment:
            supplied = explicit_path(environment[name], name)
            target_rlib(supplied, 'core')
            target_rlib(supplied, 'compiler_builtins')
            supplied_sysroots.append(supplied)
    protected = (root, source, native, *supplied_builds, *supplied_sysroots)
    outside(parent, protected)
    if not parent.is_dir() or not os.access(parent, os.W_OK):
        raise ValueError('artifact parent must already exist and be writable')
    return dict(source=source, native=native, arm=arm, parent=parent, protected=protected,
                rustc=tools['HOSTRUSTC'], bindgen=tools['BINDGEN'], sysroot=sysroot)


_VERSIONS = {}
def require_rustc(compiler, cwd, environment):
    compiler = program(str(compiler), 'Rust compiler', True)
    if compiler not in _VERSIONS:
        result = subprocess.run([str(compiler), '--version'], cwd=cwd, env=environment,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, check=True)
        match = re.match(r'rustc (\d+)\.(\d+)\.(\d+)', result.stdout)
        if match is None or tuple(map(int, match.groups())) < (1, 85, 0):
            raise ValueError('rbtree fixtures require real Rust >= 1.85: ' + result.stdout)
        _VERSIONS[compiler] = result.stdout.strip()
    return _VERSIONS[compiler]

def compiler_environment(work, environment):
    temporary = Path(work) / "tmp"
    temporary.mkdir(parents=True, exist_ok=True)
    environment = {key: value for key, value in environment.items()
                   if key not in ('MAKEFLAGS', 'MFLAGS', 'GNUMAKEFLAGS',
                                  'DEPENDENCIES_OUTPUT', 'SUNPRO_DEPENDENCIES')}
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


def validate_compiler_outputs(arguments, cwd, private, protected):
    """Reject output escapes even in added flags or nested response files."""
    language = 'rust' if Path(arguments[0]).name == 'rustc' else 'c'
    arguments = list(response_flags(arguments[1:], cwd, language))
    private = private.resolve()
    def output(value):
        path = outside(explicit_path(cwd / value, 'compiler output'), protected)
        if path != private and private not in path.parents:
            raise ValueError('compiler output escapes private fixture: ' + str(path))
    iterator = iter(arguments)
    for flag in iterator:
        if flag in ('-o', '--out-dir', '-MF'):
            value = next(iterator, None)
            if not value:
                raise ValueError('missing compiler output argument: ' + flag)
            output(value)
        elif flag.startswith(('--out-dir=', '-Wp,-MMD,', '-Wp,-MD,')):
            output(flag.split('=', 1)[1] if '=' in flag else flag.split(',', 2)[2])
        elif flag.startswith('-MF') and len(flag) > 3:
            output(flag[3:])
        elif flag.startswith('-o') and len(flag) > 2:
            output(flag[2:])
        elif flag == '--emit' or flag.startswith('--emit='):
            value = next(iterator, '') if flag == '--emit' else flag.split('=', 1)[1]
            if not value:
                raise ValueError('missing Rust emit argument')
            for emission in value.split(','):
                if '=' in emission:
                    output(emission.split('=', 1)[1])
        elif flag.startswith(('-fprofile-', '-fmodules-cache-path', '-foptimization-record-file',
                              '-ftime-trace=', '-serialize-diagnostics', '--serialize-diagnostics',
                              '-dependency-file', '-MJ')):
            raise ValueError('unsupported additional compiler output option: ' + flag)
        elif flag in ('-C', '--codegen', '-Z') or flag.startswith(('-C', '--codegen=', '-Z')):
            value = next(iterator, '') if flag in ('-C', '--codegen', '-Z') else flag[2:]
            if any(part in value for part in ('incremental=', 'profile-generate=', 'profile-dir=',
                                             'dump-mir-dir=', 'self-profile=')):
                raise ValueError('unsupported additional Rust output option: ' + flag)


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


def verify_strict_lints(flags):
    """Interpret rustc's ordered lint levels and first global lint cap."""
    levels = {"A": "allow", "W": "warn", "D": "deny", "F": "forbid"}
    warnings, cap = None, None
    iterator = iter(flags)
    for flag in iterator:
        level, value = None, None
        if flag in ("--cap-lints", "--force-warn", "--allow", "--warn", "--deny", "--forbid"):
            level, value = flag[2:], next(iterator, None)
        elif any(flag.startswith("--" + name + "=") for name in ("cap-lints", "force-warn", "allow", "warn", "deny", "forbid")):
            level, value = flag[2:].split("=", 1)
        elif len(flag) >= 2 and flag[0] == "-" and flag[1] in levels:
            level, value = levels[flag[1]], flag[2:] or next(iterator, None)
        if level is None: continue
        if not value or value.startswith("="): raise ValueError("malformed Rust lint option")
        if level == "cap-lints":
            if value not in ("allow", "warn", "deny", "forbid"):
                raise ValueError("invalid Rust lint cap")
            if cap is None: cap = value
        elif level == "force-warn":
            raise ValueError("Rust force-warn bypasses strict warnings")
        elif value == "warnings" and warnings != "forbid":
            warnings = level
    if warnings not in ("deny", "forbid") or cap not in (None, "deny", "forbid"):
        raise ValueError("Rust saved command does not have effective strict warnings")


def verify_flag_policy(flags, language):
    enabled, normalized = False, False
    iterator = iter(flags)
    for flag in iterator:
        if language == "rust":
            value = next(iterator, "") if flag == "-Z" else flag[2:] if flag.startswith("-Z") else ""
            if value.startswith("sanitizer="):
                selected = set(value.split("=", 1)[1].split(","))
                if selected & {"", "none"}: raise ValueError("invalid Rust sanitizer selection")
                enabled |= "kcfi" in selected
            elif value == "sanitizer-cfi-normalize-integers": normalized = True
            elif value.startswith("sanitizer-cfi-normalize-integers="):
                setting = value.split("=", 1)[1]
                if setting not in ("yes", "true", "y", "1", "no", "false", "n", "0"):
                    raise ValueError("invalid Rust CFI normalization option")
                normalized = setting in ("yes", "true", "y", "1")
        elif language == "c":
            if flag.startswith("-fsanitize=") and "kcfi" in flag.split("=", 1)[1].split(","): enabled = True
            elif flag.startswith("-fno-sanitize=") and {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")): enabled = False
            elif flag == "-fsanitize-cfi-icall-experimental-normalize-integers": normalized = True
            elif flag == "-fno-sanitize-cfi-icall-experimental-normalize-integers": normalized = False
        else: raise ValueError("unknown compiler language")
    if not enabled: raise ValueError("missing or disabled KCFI in actual saved command")
    if not normalized: raise ValueError("missing actual cross-language CFI integer normalization")
    if language == "rust": verify_strict_lints(flags)
