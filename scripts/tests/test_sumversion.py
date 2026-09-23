#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""Compare historical module source checksums with the original C algorithm."""

import os
from pathlib import Path
import random
import shlex
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
C_HARNESS = r'''
#include "sumversion.c"
const char *get_basename(const char *path) {
    const char *slash = strrchr(path, '/');
    return slash ? slash + 1 : path;
}
char *read_text_file(const char *name) {
    FILE *file = fopen(name, "rb");
    if (!file) { perror(name); exit(1); }
    if (fseek(file, 0, SEEK_END)) { perror(name); exit(1); }
    long size = ftell(file);
    rewind(file);
    char *data = calloc(1, size + 1);
    if (fread(data, 1, size, file) != (size_t)size) { perror(name); exit(1); }
    fclose(file);
    return data;
}
char *get_line(char **position) {
    char *line = *position;
    if (!line || !*line) return NULL;
    char *end = strchr(line, '\n');
    if (end) *end++ = 0;
    *position = end;
    return line;
}
void modpost_log(bool error, struct module *module, const char *format, ...) {
    (void)error; (void)module;
    va_list arguments;
    va_start(arguments, format);
    vfprintf(stderr, format, arguments);
    va_end(arguments);
}
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    char sum[25] = { 0 };
    get_src_version(argv[1], sum, sizeof(sum) - 1);
    puts(sum);
    return 0;
}
'''
RUST_HARNESS = r'''
//! Source digest differential harness.
#[path = "@SOURCE@"] mod sumversion;
fn io_error(error: &std::io::Error) -> String {
    error.to_string().split(" (os error ").next().unwrap().to_owned()
}
fn main() {
    match sumversion::get_src_version(&std::env::args().nth(1).unwrap()) {
        Ok(version) => {
            for warning in version.warnings { eprint!("{warning}"); }
            println!("{}", version.checksum.unwrap_or_default());
        }
        Err(error) => { eprint!("{error}"); std::process::exit(1); }
    }
}
'''


class SumversionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory(prefix="sumversion-tools-")
        cls.addClassCleanup(cls.tools.cleanup)
        work = Path(cls.tools.name)
        (work / "elfconfig.h").write_text("#define KERNEL_ELFCLASS ELFCLASS64\n")
        source = work / "harness.c"
        source.write_text(C_HARNESS)
        cls.c, cls.rust = work / "sumversion-c", work / "sumversion-rust"
        subprocess.run(shlex.split(os.environ.get("HOSTCC", "cc")) + [
            "-O2", "-I" + str(work), "-I" + str(ROOT / "scripts/mod"),
            "-I" + str(ROOT / "scripts/include"), str(source), "-o", str(cls.c)],
            check=True, capture_output=True)
        source = work / "harness.rs"
        source.write_text(RUST_HARNESS.replace("@SOURCE@", str(ROOT / "scripts/mod/sumversion.rs")))
        subprocess.run(shlex.split(os.environ.get("HOSTRUSTC", "rustc")) + [
            "--edition=2021", "-O", "-Dwarnings", "-Wmissing-docs", "-Wunreachable-pub",
            str(source), "-o", str(cls.rust)], check=True, capture_output=True)

    def compare(self, files, module="module", status=0):
        with tempfile.TemporaryDirectory(prefix="sumversion-case-") as temporary:
            work = Path(temporary)
            for name, content in files.items():
                path = work / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(content)
            results = []
            for tool in (self.c, self.rust):
                result = subprocess.run([str(tool), module], cwd=work, capture_output=True,
                                        env=dict(os.environ, LC_ALL="C"), timeout=10)
                results.append((result.returncode, result.stdout, result.stderr))
            self.assertEqual(results[0][0], status, results[0])
            self.assertEqual(results[0], results[1])
            return results[1][1]

    @staticmethod
    def files(source, dependencies=b"", object_name="dir/file.o"):
        directory, _, base = object_name.rpartition("/")
        command = f"{directory + '/' if directory else ''}.{base}.cmd"
        return {"module.mod": object_name.encode() + b"\n", "dir/source.c": source,
                command: b"savedcmd_example := cc ...\nsource_example := dir/source.c\n" + dependencies}

    def test_digest_boundaries_and_random_binary_sources(self):
        generator = random.Random(287)
        for length in list(range(0, 130)) + [255, 256, 511, 512, 1023, 4096, 65536]:
            for binary in (False, True):
                with self.subTest(length=length, binary=binary):
                    source = (generator.randbytes(length) if binary else
                              bytes(generator.choice(b"abcdef0123456789") for _ in range(length)))
                    self.compare(self.files(source))

    def test_whitespace_comments_strings_and_splices(self):
        sources = (b"a b\tc\nd\re\vf\fg", b"ab/**/cd/* multi\nline */ef", b"a\\\nb\\\r\nc",
                   b'"a b\tc\n/**/" outside', b'"a\\\" b\\\\" tail"', b'"unterminated',
                   b"before/* unterminated", b"x//not removed\ny", b"prefix\0ignored")
        for source in sources:
            with self.subTest(source=source):
                self.compare(self.files(source))

    def test_same_directory_dependency_selection(self):
        for object_name in ("file.o", "dir/file.o", "sub/dir/file.o"):
            dependencies = b"deps_example := \\\n dir/local.h \\\n elsewhere/dir/local.h \\\n dir/sub/nested.h \\\n sub/dir/local.h \\\n local.h \\\n\nignored/file.h \\\n"
            files = self.files(b"source", dependencies, object_name)
            files.update({"dir/local.h": b"one", "elsewhere/dir/local.h": b"two",
                          "dir/sub/nested.h": b"three", "sub/dir/local.h": b"four",
                          "local.h": b"five"})
            with self.subTest(object_name=object_name):
                self.compare(files)

    def test_object_order_libraries_empty_lines_and_duplicates(self):
        files = self.files(b"first")
        files.update({"dir/.second.o.cmd": b"source_second := dir/second.c\n",
                      "dir/second.c": b"second"})
        for objects in (b"", b"library.a\n", b"dir/file.o\n\ndir/second.o\n",
                        b"dir/second.o\ndir/file.o\n", b"dir/file.o\ndir/file.o\nlib.a\n"):
            with self.subTest(objects=objects):
                files["module.mod"] = objects
                self.compare(files)

    def test_command_whitespace_malformed_lines_and_nul(self):
        for command in (b"source_no_space\n", b"\vsource_x := dir/source.c\n",
                        b"source_x := dir/source.c\0source_y := missing\n",
                        b"deps_x := \\\n\n", b"source_x := dir/source.c\n\tdeps_x := \\\n \vdir/local.h \\\n"):
            files = self.files(b"text")
            files["dir/.file.o.cmd"] = command
            files["dir/local.h"] = b"header"
            with self.subTest(command=command):
                self.compare(files)

    def test_non_utf8_filenames(self):
        object_name = os.fsdecode(b"dir/file-\xff.o")
        source_name = os.fsdecode(b"dir/source-\xfe.c")
        files = {"module.mod": os.fsencode(object_name) + b"\n", source_name: b"source",
                 os.fsdecode(b"dir/.file-\xff.o.cmd"): b"source_x := " + os.fsencode(source_name) + b"\n"}
        self.compare(files)

    def test_missing_inputs_fail(self):
        for files in ({}, {"module.mod": b"dir/missing.o\n"},
                      {"module.mod": b"dir/file.o\n", "dir/.file.o.cmd": b"source_x := missing\n"}):
            with self.subTest(files=files):
                self.compare(files, status=1)


if __name__ == "__main__":
    unittest.main()
