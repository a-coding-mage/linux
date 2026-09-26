# SPDX-License-Identifier: GPL-2.0
"""Compare native and PREL32 initcall decoding with the original kernel header."""

import json
import os
from pathlib import Path
import shlex
import tempfile
import unittest

from rust_exports_test_support import rust_targets
from test_argv_split import function
from test_rational_build import environment, run


ROOT = Path(__file__).resolve().parents[2]


def native_init_includes(work):
    """Keep real init.h/compiler.h behavior and isolate compiler attributes."""
    include = work / "include/linux"
    include.mkdir(parents=True)
    (include / "build_bug.h").write_text("/* No assertions expanded by this consumer. */\n")
    (include / "types.h").write_text("#include <stdbool.h>\n")
    (include / "compiler.h").write_text(
        "#include <linux/compiler_attributes.h>\n"
        "#define __cold __attribute__((__cold__))\n"
        "#define __latent_entropy\n#define __no_kstack_erase\n" +
        function((ROOT / "include/linux/compiler.h").read_text(), "offset_to_ptr"))
    return ["-I" + str(include.parent), "-I" + str(ROOT / "include")]


class InitHeader(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="init-header-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.env = dict(environment(), RUSTC_BOOTSTRAP="1")
        self.rustc = shlex.split(os.environ.get("HOSTRUSTC", "rustc"))
        self.includes = native_init_includes(self.work)
        self.linker = self.work / "entries.lds"
        self.linker.write_text('''SECTIONS {
  .init_header : {
    *(.init_header.before)
    *(.init_header.entries)
    *(.init_header.after)
  }
} INSERT BEFORE .text;
''')
        self.wrapper = self.work / "header.rs"
        self.wrapper.write_text('''#![no_std]
extern crate self as kernel;
pub use ffi;
#[allow(non_camel_case_types)]
#[path = @HEADER@]
pub mod declarations;
use declarations::*;
#[no_mangle]
pub unsafe extern "C" fn rust_decode(entry: *mut initcall_entry_t) -> initcall_t {
    unsafe { initcall_from_entry(entry) }
}
#[no_mangle]
pub unsafe extern "C" fn rust_call(entry: *mut initcall_entry_t) -> core::ffi::c_int {
    match unsafe { initcall_from_entry(entry) } {
        Some(callback) => unsafe { callback() },
        None => -1,
    }
}
#[no_mangle]
pub extern "C" fn rust_entry_size() -> usize { core::mem::size_of::<initcall_entry_t>() }
#[no_mangle]
pub extern "C" fn rust_entry_align() -> usize { core::mem::align_of::<initcall_entry_t>() }
#[cfg(not(MODULE))]
#[no_mangle]
pub extern "C" fn rust_setup_size() -> usize { core::mem::size_of::<obs_kernel_param>() }
#[cfg(not(MODULE))]
#[no_mangle]
pub extern "C" fn rust_setup_align() -> usize { core::mem::align_of::<obs_kernel_param>() }
#[cfg(not(MODULE))]
#[no_mangle]
pub extern "C" fn rust_setup_callback_offset() -> usize {
    core::mem::offset_of!(obs_kernel_param, setup_func)
}
#[cfg(not(MODULE))]
#[no_mangle]
pub extern "C" fn rust_setup_early_offset() -> usize { core::mem::offset_of!(obs_kernel_param, early) }
#[cfg(not(MODULE))]
unsafe extern "C" { fn c_setup_callback(value: *mut ffi::c_char) -> ffi::c_int; }
#[cfg(not(MODULE))]
#[no_mangle]
pub unsafe extern "C" fn rust_setup_callback(value: *mut ffi::c_char) -> ffi::c_int {
    ffi::c_int::from(unsafe { value.read() }).wrapping_add(2)
}
#[cfg(not(MODULE))]
#[no_mangle]
pub unsafe extern "C" fn rust_setup_call(value: *mut ffi::c_char) -> ffi::c_int {
    let setup = obs_kernel_param {
        str_: c"test".as_ptr().cast(), setup_func: Some(c_setup_callback), early: 1,
    };
    let callback = core::hint::black_box(setup.setup_func);
    match callback { Some(callback) => unsafe { callback(value) }, None => -1 }
}
#[no_mangle]
pub extern "C" fn rust_this_module() -> *mut module { THIS_MODULE }
'''.replace("@HEADER@", json.dumps(str(ROOT / "include/linux/init_header.rs"))))

    def check(self, bits, compiler, optimize, *, protected=False):
        targets = rust_targets()
        if bits not in targets:
            self.skipTest("the actual i686 Rust core is required")
        for relative in (False, True):
            for module in (False, True):
                with self.subTest(relative=relative, module=module):
                    config = (["CONFIG_HAVE_ARCH_PREL32_RELOCATIONS"] if relative else [])
                    config += ["MODULE"] if module else []
                    obj = self.work / "rust.o"
                    rflags = [*targets[bits], "--edition=2021", "--crate-type=rlib",
                              "-Dwarnings", "-Wunsafe-op-in-unsafe-fn", "-Wrust-2018-idioms",
                              "-Cpanic=abort", "-Coverflow-checks=yes", "-Crelocation-model=static",
                              "-Copt-level=" + optimize,
                              *[arg for name in config for arg in ("--cfg", name)]]
                    cflags = [*self.includes, "-m" + str(bits), "-O" + optimize,
                              "-Wall", "-Wextra", "-Werror", "-funsigned-char",
                              *["-D" + name for name in config]]
                    if protected:
                        rflags += ["-Zsanitizer=kcfi", "-Zsanitizer-cfi-normalize-integers"]
                        cflags += ["-fsanitize=kcfi", "-fsanitize-cfi-icall-experimental-normalize-integers"]
                    ffi = self.work / "libffi.rlib"
                    run([*self.rustc, *rflags, "--crate-name=ffi", ROOT / "rust/ffi.rs",
                         "-o", ffi], env=self.env)
                    run([*self.rustc, *rflags, "--extern", "ffi=" + str(ffi),
                         "--emit=obj=" + str(obj), self.wrapper], env=self.env)
                    binary = self.work / "compare"
                    run([*compiler, *cflags, ROOT / "scripts/tests/init_header_driver.c", obj,
                         "-no-pie", "-Wl,-T," + str(self.linker), "-o", binary], env=self.env)
                    self.assertEqual(binary.read_bytes()[4], 1 if bits == 32 else 2)
                    run([binary], env=self.env)

    def test_original_decoding_and_declarations(self):
        for compiler in (shlex.split(os.environ.get("HOSTCC", "cc")), ["clang"]):
            for optimize in ("0", "2", "s"):
                with self.subTest(compiler=compiler, optimize=optimize):
                    self.check(64, compiler, optimize)

    def test_real_i686_pointer_width(self):
        compiler = shlex.split(os.environ.get("INIT_HEADER_I686_CC", "cc -m32"))
        for optimize in ("0", "2", "s"):
            self.check(32, compiler, optimize)

    def test_protected_callback_calls(self):
        self.check(64, ["clang"], "2", protected=True)


if __name__ == "__main__":
    unittest.main()
