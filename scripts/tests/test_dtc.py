# SPDX-License-Identifier: GPL-2.0-or-later
"""Full compiler differential tests, retaining the C grammar as the authority."""

import difflib
import os
from pathlib import Path
import random
import shlex
import subprocess
import tempfile
import unittest

from dtc_test_support import ROOT, build_c, build_rust


class DtcTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="dtc-tests-")
        cls.directory = Path(cls.temporary.name)
        cls.c = build_c(cls.directory)
        cls.rust = build_rust(cls.directory)

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def compare(self, source, options=(), inputs=(), env=None, outputs=()):
        if isinstance(source, str):
            source = source.encode()
        results = []
        for binary in (self.c, self.rust):
            for filename in outputs:
                path = self.directory / filename
                if path.exists():
                    path.unlink()
            result = subprocess.run(["dtc", *options, *inputs], executable=binary,
                                    input=source, capture_output=True, cwd=self.directory,
                                    env=env, timeout=20)
            extra = tuple((self.directory / name).read_bytes()
                          if (self.directory / name).exists() else None for name in outputs)
            results.append((result.returncode, result.stdout, result.stderr, extra))
        for index, name in enumerate(("status", "stdout", "stderr", "files")):
            if results[0][index] != results[1][index]:
                value = results[0][index]
                diff = ""
                if isinstance(value, bytes):
                    diff = "".join(difflib.unified_diff(
                        value.decode(errors="backslashreplace").splitlines(True),
                        results[1][index].decode(errors="backslashreplace").splitlines(True)))
                self.assertEqual(results[0][index], results[1][index],
                                 f"{name}: options={options!r}, source={source!r}\n{diff}")
        return results[0]

    def test_basic_trees_and_formats(self):
        fixtures = [
            "/dts-v1/; / {};",
            '/dts-v1/; / { compatible = "vendor,board"; #address-cells=<1>; #size-cells=<1>; model="A", "B"; n@0 { reg=<0 4>; }; };',
            "/dts-v1/; /memreserve/ 0x1000 0x200; reserve: /memreserve/ (1<<32) 3; / { empty; };",
            '/dts-v1/; / { raw=[00 12 34 ff]; values=<0 1 0xffffffff>; wide=/bits/ 64 <0x1122334455667788>; little=/bits/ 16 <1 65535>; bytes=/bits/ 8 <1 2 255>; };',
            '/dts-v1/; / { mixed="start", <1 2>, [11 22], /bits/ 16 <0x9876>, "end"; };',
            '/dts-v1/; / { a: first { x: p=<label: 1 last:>, finish: "abc"; }; b: second {}; };',
            '/dts-v1/; / { cpus { cpu@4 { device_type="cpu"; reg=<4>; }; }; };',
        ]
        for source in fixtures:
            for output in ("dts", "dtb", "asm", "null"):
                with self.subTest(source=source, output=output):
                    self.compare(source, ("-O", output))

    def test_integer_expression_grammar(self):
        values = ["0", "010", "0x12UL", "123ULL", "'a'", "'\\n'", "'\\x7f'",
                  "(-1)", "(~0)", "(!1)", "(2 + 3 * 7)", "(1<<63)", "(1<<64)",
                  "(0xffffffffffffffff + 2)", "(7/2)", "(7%2)", "(1 < 2)", "(2>=2)",
                  "(3 == 2)", "(3 != 2)", "(1|2^3&4)", "(1&&0||4)", "(1?4:5)",
                  "(0?1:1?2:3)", "(1 ? (0?3:4) : 5)", "(0?1/0:5)", "(1/0)", "(2%0)"]
        for value in values:
            for bits in (8, 16, 32, 64):
                with self.subTest(value=value, bits=bits):
                    self.compare(f"/dts-v1/; / {{ p = /bits/ {bits} <{value}>; }};", ("-O", "dts"))

    def test_posix_whitespace_and_include_whitespace(self):
        (self.directory / "whitespace.dtsi").write_bytes(b"/ { included; };")
        for whitespace in (b" ", b"\t", b"\n", b"\v", b"\f", b"\r", b" \t\n\v\f\r"):
            for source in (b"/dts-v1/; / { x; };", b"/dts-v1/; / { x= [00 ff]; };",
                           b"/dts-v1/; /include/ \"whitespace.dtsi\"",
                           b"/dts-v1/; / { x=<1 invalid>; };"):
                with self.subTest(whitespace=whitespace, source=source):
                    self.compare(source.replace(b" ", whitespace), ("-O", "dts"))
            for option, value in ((b"-V", b"17"), (b"-R", b"2"), (b"-S", b"100"),
                                  (b"-p", b"32"), (b"-a", b"8"), (b"-b", b"3"),
                                  (b"-b", b"-9223372036854775809")):
                self.compare(b"/dts-v1/; / {};", (option, whitespace + value))

    def test_reference_merging_and_deletion(self):
        fixtures = [
            '/dts-v1/; / { a: first { p=<1>; child {}; }; second { ref=<&a>; path=&a; }; }; &a { p=<2>; new; };',
            '/dts-v1/; / { a: first { p=<1>; child {}; }; }; / { first { /delete-property/ p; /delete-node/ child; }; };',
            '/dts-v1/; / { a: first { p=<1>; child {}; }; }; /delete-node/ &a; / { first { p=<2>; }; };',
            '/dts-v1/; / { a: first { b: child {}; }; }; &{a/child} { p="merged"; };',
            '/dts-v1/; / { a: first {}; }; second: &a { p; }; / { use=<&second>; };',
            '/dts-v1/; / { /omit-if-no-ref/ unused {}; /omit-if-no-ref/ a: used {}; ref=<&a>; };',
            '/dts-v1/; / { ref=<&a>; /omit-if-no-ref/ unused {}; /omit-if-no-ref/ a: used {}; };',
            '/dts-v1/; / { a: first {}; }; /omit-if-no-ref/ &a;',
            '/dts-v1/; / { a: first { b: second {}; }; }; /delete-node/ &{a/second};',
        ]
        for source in fixtures:
            for flags in [(), ("-@",), ("-A",), ("-L",), ("-s",), ("-H", "both")]:
                with self.subTest(source=source, flags=flags):
                    self.compare(source, ("-O", "dts", *flags))

    def test_overlays(self):
        fixtures = [
            '/dts-v1/; /plugin/; &external { compatible="test"; };',
            '/dts-v1/; /plugin/; / { a: local {}; other { ref=<&a &external>; }; };',
            '/dts-v1/; /plugin/; &{/soc} { x; }; &second { y; };',
            '/dts-v1/; /plugin/; / { a: n {}; }; &a { ref=<&external>; };',
            '/dts-v1/; /plugin/; / { a: n {}; }; &{/n} { x; };',
            '/dts-v1/; /plugin/; / { a: n { c {}; }; }; &{a/c} { x; };',
        ]
        for source in fixtures:
            for output in ("dts", "dtb", "asm"):
                for flags in [(), ("-@",), ("-s", "-@")]:
                    with self.subTest(source=source, output=output, flags=flags):
                        self.compare(source, ("-O", output, *flags))

    def test_lexical_errors_and_locations(self):
        fixtures = [b"", b" ", b"\n", b"/dts-v1/;", b"/dts-v1/; / {", b"/dts-v1/; / {}; @",
                    b"/dts-v1/; / { p=<MISSING_MACRO>; };", b"/dts-v1/; / { p=<08>; };",
                    b"/dts-v1/; / { p=<0xffffffffffffffffffff>; };", b"/dts-v1/; / { p=<>; };",
                    b"/dts-v1/; / { p=<''>; };", b"/dts-v1/; / { p=<'abc'>; };",
                    b'/dts-v1/; / { p="\\x"; };', b"/dts-v1/; / { p=[a]; };",
                    b"/dts-v1/; / { p=/bits/ 7 <1>; };", b"/dts-v1/; / { p=/bits/ 64 <&x>; };",
                    b"/dts-v1/; / { n {}; p=<1>; };", b"/dts-v1/; / { a: n {}; b: p; };",
                    b"/dts-v1/; / { p=<1> \n another; };", b"/dts-v1/; / { p=; };",
                    b"/dts-v1/; / { p=<1,2>; };", b"/dts-v1/; / { p=<1+2>; };",
                    b"/dts-v1/; / { p=/bits/ 'a' <1>; };", b"/dts-v1/; / { p=[ff /*x*/ ee]; };",
                    b"/dts-v1/; / {}; // no newline", b"/dts-v1/; / {}; /* bad",
                    b"/dts-v1/; / {}; \0 junk", b"/dts-v1/; / { p=<1LU>; };",
                    b"/dts-v1/; / { p=<(-0xffffffffffffffff)>; };",
                    b"/dts-v1/; /plugin/; /dts-v1/; / {};",
                    b'/dts-v1/; / { p="a\x80\xff"; };',
                    b'# 8 "input.dts" 1\n/dts-v1/; / { p=<macro>; };',
                    b'#line 42 "input.dts"\n/dts-v1/; / { p; };',
                    b'# 8 "input\\000.dts"\n/dts-v1/; / {};']
        for source in fixtures:
            with self.subTest(source=source):
                self.compare(source, ("-O", "dts"))

    def test_annotations_and_line_markers(self):
        source = '# 1 "path/main.dts"\n/dts-v1/;\n/ { p=<1>;\n# 27 "path/other.dtsi" 1\na: child { x; };\n};\n&a { y; };'
        for flags in [("-T",), ("-TT",), ("-TTT",), ("-T", "-@"), ("-TT", "-A")]:
            self.compare(source, ("-O", "dts", *flags))

    def test_include_incbin_and_dependencies(self):
        (self.directory / "root.dts").write_bytes(b'/dts-v1/; /include/ "included.dtsi"\n/ { data=/incbin/("data.bin",2,5); full=/incbin/("data.bin"); };')
        (self.directory / "included.dtsi").write_bytes(b'/ { a: node { p; }; };')
        (self.directory / "data.bin").write_bytes(bytes(range(32)))
        for flags in [(), ("-T",), ("-TT",), ("-@",)]:
            self.compare(b"", ("-O", "dts", "-d", "deps", *flags), ("root.dts",), outputs=("deps",))
        for offset, length in [(0, 0), (32, 8), (1000, 20), (4, 0xffffffffffffffff)]:
            self.compare(f'/dts-v1/; / {{ p=/incbin/("data.bin",{offset},{length}); }};', ("-O", "dts"))
        for source in [b'/dts-v1/; / { p=; };',
                       b'/dts-v1/; /include/ "missing-file" / {};',
                       b'/dts-v1/; / { p=/incbin/("missing-file"); };',
                       b'/dts-v1/; / { p=/incbin/("data.bin",(-1),1); };']:
            self.compare(source, ("-O", "dts", "-d", "deps"), outputs=("deps",))

    def test_include_search_limits_and_binary_paths(self):
        directory = self.directory / "search"
        directory.mkdir(exist_ok=True)
        (directory / "found.dtsi").write_bytes(b'/ { included; };')
        self.compare('/dts-v1/; /include/ "found.dtsi"',
                     ('-O', 'dts', '-i', 'missing', '-i', 'search', '-d', 'deps'), outputs=('deps',))
        path = os.fsencode(self.directory) + b'/name-\xff.dts'
        with open(path, 'wb') as file:
            file.write(b'/dts-v1/; / { p=<1>; };')
        self.compare(b'', ('-O', 'dts', '-TT', '-d', 'deps'), (path,), outputs=('deps',))
        (self.directory / 'recursive.dtsi').write_bytes(b'/include/ "recursive.dtsi"')
        self.compare('/dts-v1/; /include/ "recursive.dtsi"', ('-d', 'deps'), outputs=('deps',))
        (self.directory / 'empty.dtsi').write_bytes(b'')
        self.compare('/dts-v1/; ' + '/include/ "empty.dtsi"\n' * 200 + '/ {};',
                     ('-d', 'deps'), outputs=('deps',))

    def test_deep_nodes_and_expressions(self):
        source = '/dts-v1/; / {' + ' n {' * 3000 + ' };' * 3000 + ' };'
        for output in ('dtb', 'dts', 'asm'):
            self.compare(source, ('-O', output))
        self.compare(source + ' / {' + ' n {' * 3000 + ' changed;' + ' };' * 3000 + ' };',
                     ('-O', 'dtb'))
        for expression in ['(' * 5000 + '1' + ')' * 5000, '!' * 8000 + '1']:
            self.compare('/dts-v1/; / { p=<(' + expression + ')>; };', ('-O', 'dtb'))
        # Rust uses heap frames rather than C's fixed 10,000-entry Bison stack.
        source = ('/dts-v1/; / { p=<(' + '(' * 12000 + '1' + ')' * 12000 + ')>; };').encode()
        result = subprocess.run([self.rust, '-O', 'dts'], input=source, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(b'p = <0x01>;', result.stdout)

    def test_lexical_and_semantic_diagnostic_order(self):
        self.compare('/dts-v1/; / { p=<(2/includ\nnone)>; };', ('-O', 'dts'))
        self.compare('/dts-v1/; / { a { p=<1>; }; __local_fixups__ { '
                     'a { missing=<0>; }; absent {}; }; };', ('-O', 'dts'))
        for source in ['# 1 "foo"',
                       '/dts-v1/; / { p=<(1/\n# 80 "foo"\n0)>; };',
                       '/dts-v1/; / {}; &missing {\n# 80 "foo"\np; };',
                       '/dts-v1/; / { p=<1> /*a*//*b*/']:
            self.compare(source, ('-O', 'dts'))

    def test_command_line(self):
        for args in [[], ["-h"], ["--help"], ["-v"], ["--version"], ["-z"], ["-O"],
                     ["--unknown"], ["--out-f"], ["--out-f", "dts"], ["--out-v=17"],
                     ["--out-"], ["--help=x"], ["--", "-z"], ["one", "two"],
                     ["-a", "3"], ["-a", "0"], ["-H", "bad"], ["-S", "10", "-p", "4"],
                     ["-O", "yaml"], ["-O", "wrong"], ["-I", "wrong"], ["-T"],
                     ["-W", "unknown", "-h"], ["-I", "fs", "-T"], ["-V", "4"],
                     ["-V", "17trailing"], ["-a", "0x20"], ["-b", "-1"], ["-b", "0xffffffff"],
                     ["-R", "2", "-p", "13"], ["-S", "256", "-a", "64"],
                     ["-d", "missing/d"], ["-o", "missing/out"], ["-o", "result.DTS"]]:
            with self.subTest(args=args):
                self.compare("/dts-v1/; / {};", args)

    def test_generated_trees(self):
        randomizer = random.Random(417123)
        for number in range(100):
            children = []
            for index in range(randomizer.randrange(1, 10)):
                value = randomizer.getrandbits(64)
                children.append(f"label{index}: node{index} {{ p=/bits/ 64 <0x{value:x}>; }};")
            source = "/dts-v1/; / { rootprop=\"test\"; " + " ".join(children) + " };"
            source += f" &label{randomizer.randrange(len(children))} {{ extra = <{number}>; }};"
            with self.subTest(number=number):
                self.compare(source, ("-O", randomizer.choice(("dts", "dtb", "asm")), "-@", "-L"))

    def test_generated_malformed_grammar(self):
        randomizer = random.Random(41)
        tokens = ['a', 'aa:', '<', '>', '0', '1', ';', ':', ',', '(', ')', '{', '}',
                  '"s"', "'a'", '&a', '&{/x}', '/bits/', '/delete-node/',
                  '/omit-if-no-ref/', '[', ']', '/dts-v1/', '/plugin/', '/', 'p=',
                  '\n', '//c\n', '/*a*/']
        for number in range(400):
            source = '/dts-v1/; / { ' + ' '.join(randomizer.choices(
                tokens, k=randomizer.randrange(1, 35))) + ' };'
            with self.subTest(number=number):
                self.compare(source, ("-O", "dts"))
        self.compare('/dts-v1/; / { p=/bits/ 0 ) ; };', ("-O", "dts"))

    def test_real_kernel_device_trees(self):
        files = [
            ROOT / 'arch/arm64/boot/dts/broadcom/bcm2711-rpi-4-b.dts',
            ROOT / 'arch/arm64/boot/dts/rockchip/rk3399-rockpro64.dts',
            ROOT / 'arch/arm64/boot/dts/freescale/imx8mp-evk.dts',
            ROOT / 'arch/riscv/boot/dts/starfive/jh7110-starfive-visionfive-2-v1.3b.dts',
            ROOT / 'arch/arm/boot/dts/ti/omap/am335x-boneblack.dts',
        ]
        files += sorted((ROOT / 'drivers/of/unittest-data').glob('*.dts'))
        files += sorted((ROOT / 'drivers/of/unittest-data').glob('*.dtso'))
        for path in files:
            source = subprocess.run([
                *shlex.split(os.environ.get('HOSTCC', 'cc')), '-E', '-nostdinc',
                '-I', str(ROOT / 'scripts/dtc/include-prefixes'), '-undef', '-D__DTS__',
                '-x', 'assembler-with-cpp', str(path)], capture_output=True, check=True).stdout
            for output in ('dtb', 'dts'):
                with self.subTest(path=path, output=output):
                    result = self.compare(source, ('-O', output, '-@', '-f', '-i', str(path.parent)))
                    self.assertEqual(result[0], 0)


if __name__ == "__main__":
    unittest.main()
