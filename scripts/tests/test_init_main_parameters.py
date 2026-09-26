# SPDX-License-Identifier: GPL-2.0-only
"""Original early callback/record semantics, including the real integer parser."""
import json
import os
import shlex
import unittest

from test_argv_split import function
from test_rational_build import run
import test_init_main_command_line as commandline

ROOT = commandline.ROOT
FIXTURE = commandline.FIXTURE


class InitMainParameters(unittest.TestCase):
    prepare = commandline.InitMainCommandLine.prepare
    bindings = commandline.InitMainCommandLine.bindings
    wrapper = commandline.InitMainCommandLine.wrapper
    records = commandline.InitMainCommandLine.records

    def test_original_callbacks_with_malformed_overflow_and_alternate_quiet(self):
        build, work, env, reader, watch = self.prepare('INIT_MAIN_X86_BUILD')
        with watch:
            cflags = reader.native_flags(build, 'init/.main.o.cmd', False)
            original = (ROOT / 'init/main.c').read_text()
            (work / 'parameters.inc').write_text(''.join(function(original, name) for name in (
                'set_reset_devices', 'debug_kernel', 'quiet_kernel', 'loglevel')))
            (work / 'canonical.h').write_text('#include ' + json.dumps(str(commandline.HEADER)) + '\n')
            integers = (ROOT / 'lib/kstrtox.c').read_text()
            printf = (ROOT / 'lib/vsprintf.c').read_text()
            options = (ROOT / 'lib/cmdline.c').read_text()
            (work / 'parser.c').write_text('#include <linux/kernel.h>\n#include <linux/ctype.h>\n'
                '#include <linux/overflow.h>\n#include <linux/limits.h>\n#include ' +
                json.dumps(str(ROOT / 'lib/kstrtox.h')) + '\n' +
                function(integers, '_parse_integer_fixup_radix') + function(integers, '_parse_integer_limit') +
                function(printf, 'simple_strntoull') + function(printf, 'simple_strtoull') +
                function(options, 'get_option'))
            for quiet in (4, 3):
                override = work / 'config.h'
                override.write_text('#undef CONFIG_CONSOLE_LOGLEVEL_QUIET\n'
                                    '#define CONFIG_CONSOLE_LOGLEVEL_QUIET ' + str(quiet) + '\n')
                cfg = ['-include', str(override)]
                generated = self.bindings(build, work, env, reader, cfg)
                for optimization in ('0', '2'):
                    with self.subTest(quiet=quiet, optimization=optimization):
                        objects = []
                        for source in (FIXTURE / 'parameters.c', work / 'parser.c', ROOT / 'lib/ctype.c'):
                            obj = work / (source.stem + '.o')
                            run([*cflags, *cfg, '-I' + str(work), '-O' + optimization,
                                 '-c', source, '-o', obj], cwd=work, env=env)
                            objects.append(obj)
                        rustc = shlex.split(os.environ.get('HOSTRUSTC', 'rustc'))
                        flags = ['--edition=2021', '-Dwarnings', '-Cpanic=abort', '-Ccodegen-units=1',
                                 '-Coverflow-checks=yes', '-Copt-level=' + optimization]
                        ffi = work / 'libffi.rlib'
                        run([*rustc, *flags, '--crate-name=ffi', '--crate-type=rlib',
                             ROOT / 'rust/ffi.rs', '-o', ffi], cwd=work, env=env)
                        source = work / 'driver.rs'
                        source.write_text(self.wrapper(generated, True, 'main_parameters') +
                                          (FIXTURE / 'parameters.rs').read_text())
                        executable = work / 'parameters'
                        run([*rustc, *flags, '--extern', 'ffi=' + str(ffi), source, '-o', executable,
                             '-Clink-arg=-no-pie', *['-Clink-arg=' + str(obj) for obj in objects]], cwd=work, env=env)
                        self.assertEqual(run([executable], cwd=work, env=env).stdout,
                                         b'INIT_MAIN_PARAMETERS_OK cases=4864\n')

    def native(self, variable):
        build, work, env, reader, watch = self.prepare(variable)
        with watch:
            generated = self.bindings(build, work, env, reader)
            flags = reader.native_flags(build, 'lib/.list_sort_rust.o.cmd', True)
            flags = [flag + ',linkage' if flag.startswith('-Zallow-features=') else flag for flag in flags]
            source = work / 'native.rs'
            source.write_text(self.wrapper(generated, False, 'main_parameters'))
            obj = work / 'native.o'
            run([*flags, '--crate-name=init_main_parameters', '--emit=obj', source, '-o', obj], cwd=work, env=env)
            records = ((b'reset_devices', 0), (b'debug', 1), (b'quiet', 1), (b'loglevel', 1))
            self.assertEqual(self.records(build / 'init/main.o', records), self.records(obj, records))
            symbols = run(['llvm-nm', obj], cwd=work, env=env).stdout
            self.assertIn(b'U console_printk', symbols)
            self.assertIn(b'U get_option', symbols)

    def test_native_x86_early_records_and_canonical_calls(self):
        self.native('INIT_MAIN_X86_BUILD')

    def test_native_arm64_early_records_and_canonical_calls(self):
        self.native('INIT_MAIN_ARM64_BUILD')


if __name__ == '__main__':
    unittest.main()
