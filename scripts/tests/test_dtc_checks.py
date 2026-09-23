#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-or-later
"""DTC validation ordering, every check family, and source-reference fixups."""

import os
from pathlib import Path
import random
import re
import shlex
import struct
import subprocess
import tempfile
import unittest

from dtc_test_support import SOURCE, build_c, build_rust


def cells(*values):
    return b"".join(struct.pack(">I", value & 0xffffffff) for value in values)


def node(name=b"", properties=(), children=()):
    return (name.encode() if isinstance(name, str) else name, list(properties), list(children))


def blob(root):
    strings, structure, offsets = bytearray(), bytearray(), {}

    def emit(current):
        name, properties, children = current
        structure.extend(cells(1) + name + b"\0")
        structure.extend(bytes(-len(structure) % 4))
        for key, value in properties:
            key = key.encode() if isinstance(key, str) else key
            if key not in offsets:
                offsets[key] = len(strings)
                strings.extend(key + b"\0")
            structure.extend(cells(3, len(value), offsets[key]) + value)
            structure.extend(bytes(-len(structure) % 4))
        for child in children:
            emit(child)
        structure.extend(cells(2))

    emit(root)
    structure.extend(cells(9))
    header = cells(0xd00dfeed, 56 + len(structure) + len(strings), 56, 56 + len(structure),
                   40, 17, 16, 0, len(strings), len(structure))
    return header + bytes(16) + structure + strings


class DtcChecksTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="dtc-checks-")
        cls.addClassCleanup(cls.temporary.cleanup)
        cls.work = Path(cls.temporary.name)
        cls.c = Path(os.environ["DTC_C"]) if "DTC_C" in os.environ else build_c(cls.work)
        cls.rust = Path(os.environ["DTC_RUST"]) if "DTC_RUST" in os.environ else build_rust(cls.work)
        table = (SOURCE / "checks.c").read_text().split("static struct check *check_table[] = {", 1)[1].split("};", 1)[0]
        cls.checks = re.findall(r"&(\w+)", table)

    def compare(self, data, options=(), source=False, output="null", status=None):
        path = self.work / ("input.dts" if source else "input.dtb")
        path.write_bytes(data)
        results = []
        for tool in (self.c, self.rust):
            result = subprocess.run([str(tool), "-I", "dts" if source else "dtb", "-O", output,
                                     *options, str(path)], capture_output=True, timeout=30)
            results.append((result.returncode, result.stdout, result.stderr))
        self.assertEqual(results[0], results[1], f"options={options}, source={source}\n{data[:500]!r}")
        if status is not None:
            self.assertEqual(results[0][0], status, results[0][2])
        return results[1]

    def tree(self, root, options=(), **kwargs):
        return self.compare(blob(root), options, **kwargs)

    def test_every_check_option_and_enable_disable_spelling(self):
        for check in self.checks:
            for switch in ("-W", "-E"):
                for prefix in ("", "no-", "no_"):
                    with self.subTest(check=check, switch=switch, prefix=prefix):
                        self.tree(node(), (switch, prefix + check))
        for check in ("unknown", "no-unknown", "no_unknown", "", "ALL", "no-no-always_fail"):
            with self.subTest(check=check):
                self.tree(node(), ("-W", check), status=1)

    def test_error_order_force_quiet_and_prerequisite_propagation(self):
        root = node(children=[node("bad@name@again", [("name", b"wrong\0"), ("reg", b"x")]),
                              node("same"), node("same"), node("bad!name")])
        options = [(), ("-f",), ("-W", "always_fail"), ("-E", "always_fail"),
                   ("-W", "no-duplicate_node_names"), ("-E", "no-duplicate_node_names"),
                   ("-E", "no-duplicate_node_names", "-E", "phandle_references"),
                   ("-W", "no-reg_format", "-W", "reg_format"),
                   ("-E", "no-name_is_string", "-E", "name_properties"),
                   ("-E", "no-node_name_chars", "-W", "node_name_format")]
        for args in options:
            for quiet in range(4):
                with self.subTest(args=args, quiet=quiet):
                    self.tree(root, (*args, *(["-q"] * quiet)))
        root = node(children=[node("invalid@1", [("#address-cells", b"wrong"), ("reg", cells(1))])])
        for args in ((), ("-E", "reg_format"), ("-W", "no-address_cells_is_cell"),
                     ("-W", "no-address_cells_is_cell", "-W", "reg_format")):
            self.tree(root, args)

    def test_names_properties_strings_labels_and_binary_names(self):
        cases = [node(children=[node(name)]) for name in ("bad!", "many@@", "@12", "name_underscore", "okay+name", b"bad\xff")]
        cases += [node(properties=[(name, b"value")]) for name in ("bad@", "okay*", "bad_", "vendor,#cells", "wrong#cells", "#good", b"bad\xfe")]
        cases += [node(properties=[("dup", b"1"), ("dup", b"2")]),
                  node(properties=[("same", b"")], children=[node("same")])]
        # Scalar strings require one trailing NUL; string lists may be empty.
        for name in ("name", "model", "device_type", "status", "label", "compatible", "clock-names"):
            for value in (b"", b"x", b"\0", b"a\0b\0", b"a\0bad", b"\xff\0"):
                cases.append(node(properties=[(name, value)]))
        for index, root in enumerate(cases):
            with self.subTest(index=index):
                self.tree(root, ("-W", "node_name_chars_strict", "-W", "property_name_chars_strict"))
        for value in (b"a\0", b"wrong\0"):
            self.tree(node(children=[node("a@1", [("name", value)])]), output="dtb")

    def test_explicit_phandles_values_duplicates_and_mismatches(self):
        for value in (0, 1, 2, 0xfffffffe, 0xffffffff):
            for name in ("phandle", "linux,phandle"):
                self.tree(node(children=[node("a", [(name, cells(value))]), node("b", [("phandle", cells(1))])]))
        for length in (0, 1, 3, 5, 8):
            self.tree(node(children=[node("a", [("phandle", bytes(length))])]))
        for linux in (1, 2, 0xffffffff):
            self.tree(node(children=[node("a", [("phandle", cells(1)), ("linux,phandle", cells(linux))])]))

    def test_reg_ranges_defaults_and_unit_address_styles(self):
        for address, size in ((None, None), (0, 0), (1, 0), (1, 1), (2, 1), (3, 2)):
            properties = ([] if address is None else [("#address-cells", cells(address))])
            properties += ([] if size is None else [("#size-cells", cells(size))])
            for length in (0, 1, 4, 8, 12, 20):
                for name in ("reg", "ranges", "dma-ranges"):
                    root = node(properties=properties, children=[node("dev@0x001", [(name, bytes(length))])])
                    with self.subTest(address=address, size=size, length=length, name=name):
                        self.tree(root)
        self.tree(node(properties=[("reg", cells(0)), ("ranges", b""), ("dma-ranges", b"")]))
        for children in ([node("one@0"), node("two@0")],
                         [node("one@0", [("status", b"disabled\0")]), node("two@0")]):
            root = node(properties=[("#address-cells", cells(1)), ("#size-cells", cells(0))], children=children)
            self.tree(root)
            self.tree(root, ("-W", "no-unique_unit_address", "-W", "unique_unit_address_if_enabled"))
        self.tree(node(children=[node("bus", [("#address-cells", cells(1)), ("#size-cells", cells(1))], [node("child")])]))

    def test_pci_bridges_address_fields_and_bus_ranges(self):
        base = [("device_type", b"pci\0"), ("#address-cells", cells(3)), ("#size-cells", cells(2)), ("ranges", b"")]
        for name in ("pci", "pcie", "wrong"):
            for busrange in (None, b"", cells(5), cells(5, 2), cells(0, 256), cells(3, 7)):
                properties = base + ([] if busrange is None else [("bus-range", busrange)])
                children = [node("dev@1", [("reg", cells(value, 0, 0, 0, 0))])
                            for value in (0, 0x800, 0xf900, 0x03000800, 0x010801, 0x040800)]
                self.tree(node(children=[node(name, properties, children)]))
        for values in ((0, 1, 0, 0, 0), (0, 0, 1, 0, 0)):
            self.tree(node(children=[node("pci", base, [node("dev@0,0", [("reg", cells(*values))])])]))

    def test_simple_i2c_spi_bus_detection_and_addresses(self):
        for busname, compatible in (("bus", b"simple-bus\0"), ("i2c", None), ("i2c-bus", None),
                                     ("i2c-arb", None), ("spi", None), ("not-spi", None)):
            properties = [("#address-cells", cells(1)), ("#size-cells", cells(0))]
            if compatible:
                properties.append(("compatible", compatible))
            children = [node("device@00", [("reg", cells(value)), ("spi-max-frequency", cells(100))])
                        for value in (0, 1, 0x7f, 0x80, 0x40000042, 0x800003ff, 0x80000400)]
            children.append(node("missing"))
            self.tree(node(children=[node(busname, properties, children)]))
        self.tree(node(children=[node("i2c", children=[node("i2c-bus", [("#address-cells", cells(1)), ("#size-cells", cells(0))], [node("a@7f", [("reg", cells(0x7f))])])])]))
        self.tree(node(children=[node("spi", [("spi-slave", b""), ("#address-cells", cells(0)), ("#size-cells", cells(0))], [node("slave")])]))
        self.tree(node(children=[node("bus", [("compatible", b"simple-bus\0"), ("#address-cells", cells(2)), ("#size-cells", cells(1))], [node("bridge@100000002", [("reg", cells(1, 2, 4))])])]))

    def test_chosen_aliases_and_deprecated_properties(self):
        for value in (b"", b"wrong", b"valid\0", b"one\0two\0"):
            self.tree(node(children=[node("parent", children=[node("chosen", [("bootargs", value), ("linux,stdout-path", value)])])]))
        self.tree(node(children=[node("chosen", [("interrupt-controller", b""), ("stdout-path", b"/serial\0")]), node("serial")]))
        for name, value in (("serial0", b"/serial\0"), ("Serial0", b"/serial\0"), ("missing", b"/absent\0"), ("bad", b"")):
            self.tree(node(children=[node("aliases", [(name, value)]), node("serial")]))
        self.tree(node(properties=[("reset-gpio", cells(0)), ("vendor,nr-gpios", cells(4))]), ("-W", "deprecated_gpio_property"))

    def test_every_provider_cell_family_and_optional_msi(self):
        providers = (("clocks", "#clock-cells"), ("cooling-device", "#cooling-cells"), ("dmas", "#dma-cells"),
                     ("hwlocks", "#hwlock-cells"), ("interrupts-extended", "#interrupt-cells"),
                     ("io-channels", "#io-channel-cells"), ("iommus", "#iommu-cells"), ("mboxes", "#mbox-cells"),
                     ("msi-parent", "#msi-cells"), ("mux-controls", "#mux-control-cells"), ("phys", "#phy-cells"),
                     ("power-domains", "#power-domain-cells"), ("pwms", "#pwm-cells"), ("resets", "#reset-cells"),
                     ("sound-dai", "#sound-dai-cells"), ("thermal-sensors", "#thermal-sensor-cells"), ("gpios", "#gpio-cells"))
        for name, cellname in providers:
            for count in (None, 0, 1, 2, 4):
                properties = [("phandle", cells(1))] + ([] if count is None else [(cellname, cells(count))])
                for value in (b"x", cells(0, -1, 1), cells(1, 7), cells(99)):
                    with self.subTest(name=name, count=count, value=value):
                        self.tree(node(children=[node("provider", properties), node("consumer", [(name, value)])]))
        self.tree(node(properties=[("gpio-hog", b""), ("gpios", cells(99))]))

    def test_interrupt_parents_providers_maps_and_masks(self):
        provider = node("irq", [("phandle", cells(1)), ("interrupt-controller", b""), ("#interrupt-cells", cells(2)), ("#address-cells", cells(0))])
        for parent in (None, 0, 1, 2, 0xffffffff):
            for value in (b"x", b"", cells(1), cells(1, 2), cells(1, 2, 3)):
                props = [("interrupts", value)] + ([] if parent is None else [("interrupt-parent", cells(parent))])
                self.tree(node(children=[provider, node("child", props)]))
        self.tree(node(children=[node("irq", provider[1], [node("child", [("interrupts", cells(1))])])]))
        for mapping in (b"x", b"", cells(0), cells(0, 2, 1, 10, 20), cells(0, 2, 99), cells(0, 2, 0), cells(0, 2, 1, 10)):
            for mask in (b"", cells(0), cells(0, 0)):
                props = [("#address-cells", cells(1)), ("#interrupt-cells", cells(1)), ("interrupt-map", mapping), ("interrupt-map-mask", mask)]
                self.tree(node(children=[provider, node("map", props)]))

    def test_overflowing_provider_cell_count_terminates_without_panic(self):
        # The C loop wraps its unsigned cell index and hangs when zero/-1
        # entries precede this provider. Check the safe implementation alone.
        root = node(children=[node("provider", [("phandle", cells(1)), ("#clock-cells", cells(0xffffffff))]),
                              node("consumer", [("clocks", cells(0, -1, 1))])])
        path = self.work / "overflow.dtb"
        path.write_bytes(blob(root))
        result = subprocess.run([str(self.rust), "-I", "dtb", "-O", "null", str(path)],
                                capture_output=True, timeout=5)
        self.assertEqual(result.returncode, 0)
        self.assertIn(b"property size (12) too small for cell size 4294967295", result.stderr)
        self.assertNotIn(b"panicked", result.stderr)

    def test_graph_ports_endpoints_reciprocity_and_malformed_reg(self):
        for remote in (0, 1, 2, 99):
            end1 = node("endpoint@1", [("phandle", cells(1)), ("remote-endpoint", cells(remote)), ("reg", cells(0))])
            end2 = node("endpoint", [("phandle", cells(2)), ("remote-endpoint", cells(1))])
            self.tree(node(children=[node("ports", children=[node("port@0", [("reg", cells(0))], [end1]), node("wrongport", children=[end2])])]))
        self.tree(node(children=[node("endpoint", [("remote-endpoint", cells(0))])]))
        self.tree(node(children=[node("device", children=[node("port", children=[node("wrongendpoint", [("remote-endpoint", cells(0)), ("reg", b"x")])])])]))

    def test_source_label_duplicates_phandle_paths_and_omit_fixups(self):
        fixtures = [
            b'/dts-v1/; / { a: node { }; other { ptr = <&a>; path = &a; }; };',
            b'/dts-v1/; / { a: node { phandle = <&a>; }; };',
            b'/dts-v1/; / { a: node { }; other { phandle = <&a>; }; };',
            b'/dts-v1/; / { a: first { }; a: second { }; };',
            b'/dts-v1/; / { a: prop; a: node { }; };',
            b'/dts-v1/; / { prop = a: <1>; a: node { }; };',
            b'/dts-v1/; / { props = a: <1>, a: <2>; };',
            b'/dts-v1/; / { other { ptr = <&missing>; path = &missing; }; };',
            b'/dts-v1/; /plugin/; / { fragment@0 { target = <&external>; __overlay__ { ptr = <&external>; }; }; };',
            b'/dts-v1/; / { ptr = <&live>; /omit-if-no-ref/ dead: gone { }; /omit-if-no-ref/ live: kept { }; };',
            b'/dts-v1/; / { a: first { }; other { paths = &a, "middle", &a; }; };',
            b'/dts-v1/; / { provider: clock { #clock-cells = <1>; }; client { clocks = <&provider 3>; }; };',
        ]
        for index, data in enumerate(fixtures):
            for opts in ((), ("-f",), ("-@",), ("-H", "both")):
                with self.subTest(index=index, opts=opts):
                    self.compare(data, opts, source=True, output="dtb")

    def test_diagnostic_source_ranges_merged_nodes_and_marker_checks(self):
        fixtures = [
            b'/dts-v1/;\n/ {\n a: item@0 { reg = <1>; };\n};\n&a { new; };\n',
            b'/dts-v1/; / { irq: irq { interrupt-controller; #interrupt-cells = <2>; };\n child { interrupt-parent = <&irq>; interrupts = <1>; }; };',
            b'/dts-v1/; / { clock { phandle = <1>; #clock-cells = <0>; }; client { clocks = <1>; }; };',
            b'/dts-v1/; / { x: node { }; }; / { x: other { }; };',
        ]
        for data in fixtures:
            self.compare(data, source=True)
            self.compare(data, ("-f", "-E", "reg_format"), source=True)

    def test_random_valid_cell_and_provider_trees(self):
        rng = random.Random(0xD7C)
        for index in range(100):
            provider = node("clock", [("phandle", cells(1)), ("#clock-cells", cells(rng.randrange(4)))])
            children = [provider]
            for child in range(rng.randrange(1, 8)):
                children.append(node(f"device@{child:x}", [("reg", cells(*[rng.randrange(100) for _ in range(rng.randrange(5))])),
                                                          ("clocks", cells(*[rng.randrange(4) for _ in range(rng.randrange(8))]))]))
            with self.subTest(index=index):
                self.tree(node(properties=[("#address-cells", cells(rng.randrange(4))), ("#size-cells", cells(rng.randrange(3)))], children=children))

    def test_real_kernel_board_sources_and_complete_dtb_output(self):
        root = SOURCE.parents[1]
        boards = (
            "arch/arm/boot/dts/broadcom/bcm2835-rpi-b.dts",
            "arch/arm64/boot/dts/allwinner/sun50i-a64-pine64.dts",
            "arch/riscv/boot/dts/starfive/jh7100-beaglev-starlight.dts",
            "arch/powerpc/boot/dts/gamecube.dts",
        )
        for board in boards:
            command = shlex.split(os.environ.get("HOSTCC", "cc")) + [
                "-E", "-nostdinc", "-undef", "-D__DTS__", "-x", "assembler-with-cpp",
                "-I", str(SOURCE / "include-prefixes"), str(root / board)]
            result = subprocess.run(command, capture_output=True, cwd=root, timeout=30)
            self.assertEqual(result.returncode, 0, result.stderr)
            with self.subTest(board=board):
                self.compare(result.stdout, source=True, output="dtb", status=0)


if __name__ == "__main__":
    unittest.main()
