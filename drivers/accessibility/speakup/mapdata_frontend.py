#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0
"""Extract data, never code, from the selected host C frontend.

No C helper is linked or executed. Original sources receive syntax validation;
their preprocessed declarations and data are compiled after removing functions.
The object reader deliberately fails closed on unknown object/relocation ABIs.
"""
import argparse
from pathlib import Path
import re
import shlex
import shutil
import struct
import subprocess
import sys
import tempfile


class Object:
    def __init__(self, data):
        self.data = data
        if data[:4] != b"\x7fELF" or data[4] not in (1, 2) or data[5] not in (1, 2):
            raise ValueError("unsupported host object format")
        self.wide = data[4] == 2
        self.endian = "<" if data[5] == 1 else ">"
        header = self.unpack("HHIQQQIHHHHHH" if self.wide else "HHIIIIIHHHHHH", 16)
        if header[0] != 1:
            raise ValueError("expected a relocatable data object (LTO is unsupported)")
        self.machine = header[1]
        self.sections = [self.unpack("IIQQQQIIQQ" if self.wide else "IIIIIIIIII",
                                     header[5] + i * header[10]) for i in range(header[11])]
        self.symbols = {}
        self.tables = {}
        for index, section in enumerate(self.sections):
            if section[1] != 2:
                continue
            strings = self.section(section[6])
            symbols = []
            for at in range(section[4], section[4] + section[5], section[9]):
                fields = self.unpack("IBBHQQ" if self.wide else "IIIBBH", at)
                name, info, other, shndx, value, size = fields if self.wide else (
                    fields[0], fields[3], fields[4], fields[5], fields[1], fields[2])
                symbol = (shndx, value, size)
                symbols.append(symbol)
                self.symbols[strings[name:].split(b"\0", 1)[0]] = symbol
            self.tables[index] = symbols

    def unpack(self, fmt, at):
        return struct.unpack_from(self.endian + fmt, self.data, at)

    def section(self, index):
        section = self.sections[index]
        if section[1] == 8:
            raise ValueError("unexpected zero-fill data")
        return self.data[section[4]:section[4] + section[5]]

    def symbol(self, name):
        if name not in self.symbols:
            raise ValueError("missing data symbol (LTO or incompatible flags)")
        section, offset, size = self.symbols[name]
        return self.section(section)[offset:offset + size]

    def names(self, section_index, begin=0, end=None):
        # Absolute pointer relocations only. Unknown semantics must not become
        # silently wrong name bytes. No machine instructions are interpreted.
        absolute = {3: (1, 4), 62: (1, 8), 183: (257, 8), 40: (2, 4),
                    243: (2 if self.wide else 1, 8 if self.wide else 4)}
        if self.machine not in absolute:
            raise ValueError("unsupported host ELF relocation architecture")
        kind, width = absolute[self.machine]
        result = {}
        for section in self.sections:
            if section[1] not in (4, 9) or section[7] != section_index:
                continue
            for at in range(section[4], section[4] + section[5], section[9]):
                offset, info = self.unpack("QQ" if self.wide else "II", at)
                if offset < begin or (end is not None and offset >= end):
                    continue
                symbol_index = info >> (32 if self.wide else 8)
                relocation = info & (0xffffffff if self.wide else 0xff)
                if relocation != kind:
                    raise ValueError("unsupported host pointer relocation")
                if section[1] == 4:
                    addend, = self.unpack("q" if self.wide else "i", at + width * 2)
                else:
                    addend = int.from_bytes(self.section(section_index)[offset:offset + width],
                                            "little" if self.endian == "<" else "big", signed=True)
                target, value, _ = self.tables[section[6]][symbol_index]
                if target == 0 or target >= len(self.sections):
                    raise ValueError("initializer needs an external symbol")
                content = self.section(target)
                start = value + addend
                if start < 0 or start >= len(content) or b"\0" not in content[start:]:
                    raise ValueError("initializer is not a terminated string")
                result[offset] = content[start:].split(b"\0", 1)[0] + b"\0"
        return result


def declarations(text):
    """Keep original preprocessed tokens, replacing function bodies by ';'.

    Expansion has already happened in the original TU (including headers,
    __LINE__, __FILE__, include depth and counter state). This is a bounded
    declaration extractor, not a C expression evaluator. Unsupported syntax
    must fail compilation or the executable-section guard, never run as C.
    """
    tokens = re.finditer(
        rb'^\s*\#[^\n]*|/\*.*?\*/|//[^\n]*|'
        rb'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|'
        rb'[A-Za-z_][A-Za-z_0-9]*|[^\s]', text, re.M | re.S)
    tokens = [match for match in tokens
              if not match.group().lstrip().startswith((b'#', b'/*', b'//'))]
    words = [match.group() for match in tokens] + [b'']
    pairs = {}
    stack = []
    for i, token in enumerate(words[:-1]):
        if token in (b'(', b'[', b'{'):
            stack.append(i)
        elif token in (b')', b']', b'}'):
            if not stack or words[stack[-1]] != {b')': b'(', b']': b'[', b'}': b'{'}[token]:
                raise ValueError("unbalanced preprocessed declaration")
            pairs[stack.pop()] = i + 1
    if stack:
        raise ValueError("unbalanced preprocessed declaration")

    def fail(i):
        raise ValueError("unsupported preprocessed declaration near " +
                         repr(b' '.join(words[max(0, i - 4):i + 5])))

    def identifier(i):
        return re.fullmatch(rb'[A-Za-z_][A-Za-z_0-9]*', words[i]) is not None

    attributes = {b'__attribute__', b'__attribute', b'__declspec'}
    qualifiers = {b'const', b'volatile', b'restrict', b'__const', b'__const__',
                  b'__volatile', b'__volatile__', b'__restrict', b'__restrict__', b'_Atomic'}

    def decorations(i, asm=False):
        while words[i] in attributes or (asm and words[i] in (b'asm', b'__asm', b'__asm__')):
            i += 1
            if words[i] != b'(':
                fail(i)
            i = pairs[i]
        return i

    def declarator(i):
        # Derived operators ordered outwards from the actual declared name:
        # f(...) is a function; (*f)(...) is an object holding a pointer.
        pointers = []
        i = decorations(i)
        while words[i] == b'*':
            pointers.append(b'pointer')
            i += 1
            while words[i] in qualifiers or words[i] in attributes:
                i = decorations(i) if words[i] in attributes else i + 1
        if words[i] == b'(':
            end = pairs[i]
            i, derived = declarator(i + 1)
            if i != end - 1:
                fail(i)
            i = end
        elif identifier(i):
            i, derived = i + 1, []
        else:
            fail(i)
        while True:
            i = decorations(i, asm=True)
            if words[i] not in (b'(', b'['):
                break
            derived.append(b'function' if words[i] == b'(' else b'array')
            i = pairs[i]
        return i, derived + pointers

    storage = {b'typedef', b'extern', b'static', b'auto', b'register', b'inline',
               b'__inline', b'__inline__', b'_Noreturn', b'_Thread_local', b'__thread', b'__extension__'}
    builtin = {b'void', b'char', b'short', b'int', b'long', b'float', b'double',
               b'signed', b'unsigned', b'_Bool', b'_Complex', b'__int128', b'__builtin_va_list'}
    edits = []
    i = 0
    while words[i]:
        if words[i] == b';':
            i += 1
            continue
        if words[i] in (b'_Static_assert', b'static_assert'):
            i += 1
            if words[i] != b'(':
                fail(i)
            i = pairs[i]
            if words[i] != b';':
                fail(i)
            continue
        have_type = False
        while True:
            i = decorations(i)
            token = words[i]
            if token in (b'struct', b'union', b'enum'):
                i = decorations(i + 1)
                if identifier(i):
                    i = decorations(i + 1)
                if words[i] == b'{':
                    i = pairs[i]
                have_type = True
            elif token in (b'typeof', b'__typeof', b'__typeof__', b'_Atomic') and words[i + 1] == b'(':
                i = pairs[i + 1]
                have_type = True
            elif token in builtin:
                have_type = True
                i += 1
            elif token in storage or token in qualifiers:
                i += 1
            elif not have_type and identifier(i):
                # A typedef name is a type specifier, never a special-cased
                # system-header type. The original compiler validated it.
                have_type = True
                i += 1
            else:
                break
        if not have_type:
            fail(i)
        if words[i] == b';':
            continue
        while True:
            i, derived = declarator(i)
            if words[i] == b'{':
                if not derived or derived[0] != b'function':
                    fail(i)
                end = pairs[i]
                edits.append((tokens[i].start(), tokens[end - 1].end()))
                i = end
                break
            if words[i] == b'=':
                i += 1
                while words[i] not in (b',', b';', b''):
                    i = pairs[i] if i in pairs else i + 1
            if words[i] == b',':
                i += 1
                continue
            if words[i] != b';':
                fail(i)
            i += 1
            break
    for start, end in reversed(edits):
        text = text[:start] + b';' + b'\n' * text[start:end].count(b'\n') + text[end:]
    return text


def preprocess(compiler, source):
    result = subprocess.run(compiler + ["-w", "-E", str(source)], stdout=subprocess.PIPE)
    if result.returncode:
        sys.exit(result.returncode if result.returncode > 0 else 1)
    return result.stdout


def run(command, data_probe=False):
    result = subprocess.run(command)
    if result.returncode:
        if data_probe:
            print("Speakup data/context probe failed (requires actual frontend UTF-8 charsets); "
                  "use retained C selection HOST_TOOLS_LANG=c", file=sys.stderr)
        sys.exit(result.returncode if result.returncode > 0 else 1)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--mapdata", type=Path)
    parser.add_argument("--depfile", type=Path)
    parser.add_argument("compiler", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    compiler = args.compiler
    if compiler[:1] == ["--"]:
        compiler = compiler[1:]
    if not compiler:
        parser.error("selected host compiler command is required")
    # The same original frontend owns acceptance, warnings and -Werror.
    dependencies = ["-MD", "-MF", str(args.depfile)] if args.depfile else []
    run(compiler + dependencies + ["-fsyntax-only", str(args.source)])
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".speakup-data-", dir=args.output.parent) as tmp:
        source = Path(tmp) / "data.c"
        obj = Path(tmp) / "data.o"
        text = ('const char speakup_charset_probe[] = "é𐀀" "\\u00e9\\U00010000";\n'
                '#ifdef __GNUC_EXECUTION_CHARSET_NAME\n'
                'const char speakup_execution_charset[] = __GNUC_EXECUTION_CHARSET_NAME;\n'
                '#elif defined(__clang__)\n'
                'const char speakup_execution_charset[] = "UTF-8";\n'
                '#else\n#error Unknown frontend execution charset; use HOST_TOOLS_LANG=c\n#endif\n')
        source.write_bytes(text.encode("utf-8", errors="surrogateescape"))
        def data_object(original):
            preprocessed = Path(tmp) / "data.i"
            preprocessed.write_bytes(declarations(preprocess(compiler, original)))
            # .i prevents a second macro expansion / forced include pass.
            # Diagnostics already came from the original translation unit.
            run(compiler + ["-w", "-c", "-o", str(obj), str(preprocessed)], data_probe=True)
            data = Object(obj.read_bytes())
            if any(section[2] & 4 and section[5] for section in data.sections):
                raise ValueError("unexpected executable content in declaration object")
            return data

        # Separate objects avoid duplicating declarations from forced headers.
        data = data_object(source)
        charset = data.symbol(b"speakup_execution_charset").rstrip(b"\0").lower().replace(b"-", b"")
        if (charset != b"utf8" or
                data.symbol(b"speakup_charset_probe") != (b"\xc3\xa9\xf0\x90\x80\x80" * 2 + b"\0")):
            raise ValueError("Rust Speakup requires actual frontend UTF-8 input and execution charsets")
        output = bytearray(b"SKMP0001")
        if args.mapdata:
            # The original source owns all expansion points and declarations.
            # No function body reaches the compiler's code generation pass.
            data = data_object(args.source)
            section, offset, size = data.symbols[b"init_key_data"]
            table = data.symbol(b"init_key_data")
            names = data.names(section, offset, offset + size)
            width = 8 if data.wide else 4
            stride = width + 8
            if size % stride:
                raise ValueError("incompatible host initializer layout")
            sentinel = False
            for at in range(0, size, stride):
                name = names.get(offset + at)
                if name is None:
                    raise ValueError("missing name relocation")
                if name.startswith(b"."):
                    sentinel = True
                    break
                value, shift = struct.unpack_from(data.endian + "ii", table, at + width)
                output += struct.pack("<Iii", len(name), value, shift) + name
            if not sentinel:
                raise ValueError("missing initializer sentinel")
    if args.depfile:
        # Driver depfiles omit response files and compiler wrappers. Track the
        # visible files as well, so changing a charset inside either reruns the
        # actual frontend guard without making successful no-op builds rebuild.
        extra = set()

        def command_dependencies(arguments):
            for argument in arguments:
                if argument.startswith("@"):
                    path = Path(argument[1:]).resolve()
                    if path not in extra:
                        extra.add(path)
                        command_dependencies(shlex.split(path.read_text()))
                elif not argument.startswith("-"):
                    executable = shutil.which(argument)
                    if executable:
                        extra.add(Path(executable).absolute())

        command_dependencies(compiler)
        escaped = [str(path).replace("$", "$$").replace("#", "\\#").replace(" ", "\\ ")
                   for path in sorted(extra)]
        with args.depfile.open("a", encoding="utf-8", errors="surrogateescape") as depfile:
            # fixdep treats the first prerequisite of each rule as its source.
            depfile.write(str(args.output) + ": " + str(args.source) + " " +
                          " ".join(escaped) + "\n")
    # Do not refresh a success target if even dependency collection failed.
    args.output.write_bytes(output)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, IndexError, struct.error) as error:
        sys.exit(f"Speakup frontend data: {error}; use retained C selection HOST_TOOLS_LANG=c")
