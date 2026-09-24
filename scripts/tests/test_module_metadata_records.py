#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Fail-closed checks for the target-preprocessed module metadata protocol."""

import subprocess
import unittest

from modpost_test_support import modpost_tools


OWNER = 'LUPOS_MODULE_OWNER .name="fixture", .arch={}, };'


def records(body=OWNER, count=1):
    return (f'LUPOS_MODULE_COUNT {count};\n'
            'LUPOS_MODULE_CHARSET "UTF-8";\n'
            'LUPOS_MODULE_INPUT_UTF8 "é";\n'
            'LUPOS_MODULE_LOGICAL_SOURCE "same.mod.c", "same.mod.c";\n'
            + body + '\nLUPOS_MODULE_END;\n').encode()


class ModuleMetadataRecords(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.binary = modpost_tools()[64][1]

    def invoke(self, data, *arguments, **kwargs):
        return subprocess.run([str(self.binary), '--rust-module-records', *arguments],
                              input=data, stderr=subprocess.PIPE,
                              timeout=30, **kwargs)

    def accept(self, data):
        result = self.invoke(data, stdout=subprocess.PIPE)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, b'')
        return result.stdout

    def reject(self, data, message=None):
        result = self.invoke(data, stdout=subprocess.PIPE)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stdout, b'', 'failed parse must not publish partial Rust')
        self.assertTrue(result.stderr)
        if message is not None:
            self.assertIn(message, result.stderr)

    def test_required_guards_and_exact_record_count(self):
        valid = records()
        self.assertIn(b'pub static mut __this_module', self.accept(valid))
        for old, new in (
                (b'LUPOS_MODULE_COUNT 1;', b''),
                (b'COUNT 1;', b'COUNT 0;'),
                (b'COUNT 1;', b'COUNT 2;'),
                (b'COUNT 1;', b'COUNT -1;'),
                (b'COUNT 1;', b'COUNT 1; LUPOS_MODULE_COUNT 1;'),
                (b'LUPOS_MODULE_CHARSET "UTF-8";', b''),
                (b'"UTF-8"', b'"ISO-8859-1"'),
                ('"é"'.encode(), b'"x"'),
                (b'"same.mod.c", "same.mod.c"', b'"a", "b"'),
                (b'LUPOS_MODULE_OWNER', b'LUPOS_MODULE_CHARSET "UTF-8"; LUPOS_MODULE_OWNER')):
            with self.subTest(old=old, new=new):
                self.reject(valid.replace(old, new))

    def test_owner_and_terminal_are_unique_and_required(self):
        self.reject(records('LUPOS_MODULE_INFO "name=fixture";'))
        self.reject(records(OWNER + OWNER, 2))
        for old, new in ((b'.arch={}', b'.arch={1}'),
                         (b'.arch={}', b'.unknown=1, .arch={}'),
                         (b'LUPOS_MODULE_END;', b'')):
            self.reject(records().replace(old, new))
        for suffix in (b'LUPOS_MODULE_END;', b'LUPOS_MODULE_INFO "x=y";',
                       b'"unterminated'):
            self.reject(records() + suffix)
        self.reject(records(OWNER + 'LUPOS_MODULE_UNKNOWN 1;', 2))

    def test_extended_names_require_explicit_final_terminator(self):
        prefix = OWNER + 'LUPOS_MODULE_EXT_CRCS 0x1, }; LUPOS_MODULE_EXT_NAMES '
        output = self.accept(records(prefix + '"alpha\\0";', 3))
        self.assertIn(b'[97, 108, 112, 104, 97, 0, 0]', output)
        # Both bad strings have the old accepted NUL count. C would append an
        # implicit NUL, but their final name lacks its explicit terminator.
        for value in ('"alpha\\0junk"', '"\\0alpha"'):
            with self.subTest(value=value):
                self.reject(records(prefix + value + ';', 3),
                            b'extended version names lack final explicit NUL')

    def test_extended_names_crc_count_and_pairing(self):
        prefix = OWNER + 'LUPOS_MODULE_EXT_CRCS 0x1, 0x2, }; '
        self.accept(records(prefix + 'LUPOS_MODULE_EXT_NAMES "alpha\\0" "beta\\0";', 3))
        self.reject(records(prefix + 'LUPOS_MODULE_EXT_NAMES "alpha\\0";', 3))
        self.reject(records(OWNER + 'LUPOS_MODULE_EXT_CRCS 0x1, }; '
                            'LUPOS_MODULE_EXT_NAMES "\\0";', 3),
                    b'extended version name is empty')
        self.reject(records(prefix, 2))
        self.reject(records(OWNER + 'LUPOS_MODULE_EXT_NAMES "alpha\\0";', 2))
        self.accept(records(OWNER + 'LUPOS_MODULE_EXT_CRCS }; LUPOS_MODULE_EXT_NAMES "";', 3))

    def test_quoted_marker_and_escaped_bytes_are_data(self):
        output = self.accept(records(OWNER +
            'LUPOS_MODULE_INFO "alias=LUPOS_MODULE_END;\\200\\377";', 2))
        self.assertIn(b'128, 255, 0]', output)
        self.assertIn(b'__MODULE_INFO_1', output)
        self.reject(records(OWNER + 'LUPOS_MODULE_INFO "name=a\\0hidden";', 2))

    def test_cli_rejects_extra_arguments_and_write_failure(self):
        for arguments in (('extra',), ('--help',)):
            result = self.invoke(records(), *arguments, stdout=subprocess.PIPE)
            self.assertNotEqual(result.returncode, 0)
        with open('/dev/full', 'wb') as full:
            result = self.invoke(records(), stdout=full)
        self.assertNotEqual(result.returncode, 0)
        self.assertTrue(result.stderr)


if __name__ == '__main__':
    unittest.main()
