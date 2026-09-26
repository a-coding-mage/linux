# SPDX-License-Identifier: GPL-2.0-only
"""Original early option routing with canonical boot state and C interfaces."""

import json
import os
from pathlib import Path
import shlex
import re
import subprocess
import unittest

from test_rational_build import run
from test_argv_split import function
from rbtree_native import transport
import test_init_main_command_line as command_line


ROOT = Path(__file__).resolve().parents[2]


class InitMainBootOptions(unittest.TestCase):
    prepare = command_line.InitMainCommandLine.prepare

    def bindings(self, build, work, env, reader, extra=()):
        saved = reader.saved(build, "rust/bindings/.bindings_generated.rs.cmd")
        flags = transport.native_flags(saved[saved.index("--") + 1:], build, "c")
        generated = work / "generated.rs"
        header = work / "canonical.h"
        helper = (ROOT / 'rust/helpers/bug.c').read_text()
        declaration = re.search(r'^__rust_helper __noreturn void rust_helper_BUG\(void\)', helper, re.M).group()
        header.write_text('#include ' + json.dumps(str(ROOT / "rust/bindings/init_main.h")) +
                          '\n#include <linux/sysctl.h>\n#define __rust_helper\n' + declaration + ';\n')
        run([*shlex.split(os.environ.get("BINDGEN", saved[0])), header,
             "--use-core", "--rust-target=1.85", "--ctypes-prefix=kernel::ffi",
             "--no-layout-tests", "--no-doc-comments", "--enable-function-attribute-detection", "--wrap-unsafe-ops",
             "--allowlist-type=^(system_states|ktime_t|initcall_entry_t|list_head|obs_kernel_param|pi_entry)$",
             "--allowlist-var=^(RUST_INIT_MAIN_.*|__initcall.*|__setup_start|__setup_end)$",
             "--allowlist-function=^(strlen|memmove|strncmp|strnchr|parameqn|sysctl_is_alias|_printk|rust_helper_BUG)$",
             "-o", generated, "--", *flags, *extra], cwd=work, env=env)
        return generated

    def wrapper(self, generated, host=False):
        return ('//! Staged boot-option routines using actual declarations and state.\n'
                '#![feature(linkage)]\n'
                '#![allow(dead_code,missing_docs,non_camel_case_types,non_snake_case,non_upper_case_globals)]\n'
                + ('extern crate self as kernel;\npub extern crate ffi;\n' if host else '') +
                'pub mod bindings { include!(' + json.dumps(str(generated)) + ');' +
                ('pub use rust_helper_BUG as BUG;' if host else '') + ' }\n' +
                ''.join('#[path=' + json.dumps(str(ROOT / ("init/" + name + ".rs"))) +
                        '] mod ' + name + ';\n' for name in
                        ("main_globals", "main_printk", "main_bootoptions")) +
                'pub use main_globals::*;\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, "lib/.list_sort_rust.o.cmd", True)
            flags = [flag + ",linkage" if flag.startswith("-Zallow-features=") else flag for flag in flags]
            wrapper = work / "native.rs"
            wrapper.write_text(self.wrapper(generated) + '''
#[no_mangle]
pub unsafe extern "C" fn bootoptions_fixture(
    parameter: *mut kernel::ffi::c_char, value: *mut kernel::ffi::c_char, mode: kernel::ffi::c_int,
) -> kernel::ffi::c_int {
    unsafe {
        match mode {
            0 => { main_bootoptions::repair_env_string(parameter, value); 0 },
            1 => main_bootoptions::set_init_arg(parameter, value, core::ptr::null(), core::ptr::null_mut()),
            2 => main_bootoptions::unknown_bootoption(parameter, value, core::ptr::null(), core::ptr::null_mut()),
            _ => kernel::ffi::c_int::from(main_bootoptions::obsolete_checksetup(parameter)),
        }
    }
}
''')
            obj = work / "native.o"
            run([*flags, "--crate-name=init_main_bootoptions", "--emit=obj", wrapper, "-o", obj],
                cwd=work, env=env)
            symbols = run(["llvm-nm", "-u", obj], cwd=work, env=env).stdout
            for name in (b"rust_helper_BUG", b"parameqn", b"_printk", b"__setup_start", b"__setup_end"):
                self.assertIn(name, symbols)
            for name in (b"pr_warn", b"strstarts", b"console_loglevel"):
                self.assertNotIn(name, symbols)

    def test_native_x86_canonical_calls(self):
        self.native("INIT_MAIN_X86_BUILD")

    def test_native_arm64_canonical_calls(self):
        self.native("INIT_MAIN_ARM64_BUILD")

    def test_original_state_mutation_registry_and_fatal_parser_geometry(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = (ROOT / 'init/main.c').read_text()
            declarations = ''.join(re.search(pattern, original, re.M).group() + '\n' for pattern in (
                r'^static const char \*argv_init\[.*?;', r'^const char \*envp_init\[.*?;',
                r'^static const char \*panic_later, \*panic_param;'))
            oracle = work / 'oracle.c'
            oracle.write_text('#define envp_init original_envp_init\n#include "canonical.h"\n'
                '#define MAX_INIT_ARGS CONFIG_INIT_ENV_ARG_LIMIT\n#define MAX_INIT_ENVS CONFIG_INIT_ENV_ARG_LIMIT\n'
                '#undef BUG\n#define BUG() rust_helper_BUG()\n' + declarations +
                ''.join(function(original, name) for name in (
                    'repair_env_string', 'obsolete_checksetup', 'set_init_arg', 'unknown_bootoption')) + '''
void original_prepare(size_t arguments, size_t environment, size_t replacement, bool pending)
{
    size_t i;
    for (i = 0; i < ARRAY_SIZE(argv_init); i++) argv_init[i] = i < arguments ? "seed" : NULL;
    for (i = 0; i < ARRAY_SIZE(envp_init); i++) envp_init[i] = i < environment ? "other=before" : NULL;
    if (replacement < environment) envp_init[replacement] = "replace=before";
    panic_later = pending ? "pending" : NULL;
    panic_param = pending ? "older" : NULL;
}
int original_run(unsigned int mode, char *parameter, char *value)
{
    switch (mode) {
    case 0: repair_env_string(parameter, value); return 0;
    case 1: return set_init_arg(parameter, value, NULL, NULL);
    case 2: return unknown_bootoption(parameter, value, NULL, NULL);
    default: return obsolete_checksetup(parameter);
    }
}
const char *original_argument(size_t index) { return argv_init[index]; }
const char *original_environment(size_t index) { return envp_init[index]; }
const char *original_panic(void) { return panic_later; }
const char *original_panic_parameter(void) { return panic_param; }
''')
            services = work / 'services.c'
            parameters = (ROOT / 'kernel/params.c').read_text()
            strings = (ROOT / 'lib/string.c').read_text()
            services.write_text('#include <linux/moduleparam.h>\n#include <linux/string.h>\n' +
                function(parameters, 'dash2underscore') + function(parameters, 'parameqn') +
                function(strings, 'strnchr'))
            linker = work / 'registry.lds'
            linker.write_text('SECTIONS { .init.setup : { __setup_start = .; KEEP(*(.init.setup)) '
                              '__setup_end = .; } /DISCARD/ : { *(.discard.*) } } INSERT AFTER .data;\n')
            rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
            for sysctl in (False, True):
                config = work / 'config.h'
                config.write_text('#undef CONFIG_SYSCTL\n' + ('#define CONFIG_SYSCTL 1\n' if sysctl else '') +
                    '#undef CONFIG_INIT_ENV_ARG_LIMIT\n#define CONFIG_INIT_ENV_ARG_LIMIT 7\n'
                    '#undef CONFIG_PRINTK\n#define CONFIG_PRINTK 1\n#undef CONFIG_PRINTK_INDEX\n')
                extra = ['-include', str(config)]
                generated = self.bindings(build, work, env, reader, extra)
                for optimization in ('0', '2'):
                    with self.subTest(sysctl=sysctl, optimization=optimization):
                        objects = []
                        for source in (oracle, services, ROOT / 'scripts/tests/init_main_bootoptions_transport.c'):
                            obj = work / (source.stem + '.o')
                            run([*cflags, *extra, '-I' + str(work), '-O' + optimization,
                                 '-c', source, '-o', obj], cwd=work, env=env)
                            objects.append(obj)
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                                 '-Coverflow-checks=yes', '-Copt-level=' + optimization,
                                 '-Zsanitizer=kcfi', '-Zsanitizer-cfi-normalize-integers']
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        wrapper = work / 'driver.rs'
                        wrapper.write_text(self.wrapper(generated, True) +
                            (ROOT / 'scripts/tests/init_main_bootoptions_driver.rs').read_text())
                        executable = work / 'bootoptions'
                        run([*rustc, *flags, '--cfg=CONFIG_PRINTK', *(['--cfg=CONFIG_SYSCTL'] if sysctl else []),
                             '--extern', 'ffi=' + str(ffi), wrapper, '-o', executable,
                             '-Clink-arg=-no-pie', '-Clink-arg=-Wl,-T,' + str(linker),
                             *['-Clink-arg=' + str(obj) for obj in objects]], cwd=work, env=env)
                        self.assertEqual(run([executable], cwd=work, env=env).stdout,
                                         b'INIT_MAIN_BOOTOPTIONS_OK cases=20160\n')
                        for mode in range(3):
                            for alias in ('alias', 'ordinary'):
                                for pending in ('pending', 'clear'):
                                    with self.subTest(mode=mode, alias=alias, pending=pending):
                                        results = [subprocess.run([executable, owner, str(mode), alias, pending],
                                            cwd=work, env=env, capture_output=True, timeout=10) for owner in ('c', 'rust')]
                                        returned = (mode == 1 and pending == 'pending') or (mode == 2 and alias == 'alias' and sysctl)
                                        expected = (0, b'RETURNED 0\n', b'') if returned else (86, b'', b'')
                                        for result in results:
                                            self.assertEqual((result.returncode, result.stdout, result.stderr), expected)


if __name__ == "__main__":
    unittest.main()
