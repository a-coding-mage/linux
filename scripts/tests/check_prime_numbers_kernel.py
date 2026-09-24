#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check selected native prime providers, original KUnit, and non-GPL callers.

Only external test fixtures are generated. Both caller languages retain a
Proprietary license and call the real public exports. This gate proves public
ABI/results, selected KUnit lifecycle and module reload, not allocator-failure
injection or reader/writer/RCU reclamation; those require a separate GPL fixture.
No module is ever loaded on the host.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

from boot_kernel import module_name, verify_module_events
from check_cordic_kernel import imported_crc, tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_module_common import verify_common_metadata
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_reciprocal_kernel import module_elf as structural_elf
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports


ROOT = Path(__file__).resolve().parents[2]
PUBLIC = ("is_prime_number", "next_prime_number")
PRIVATE = ("with_primes", "slow_is_prime_number")
KUNIT_IMPORTS = ("__kunit_do_failed_assertion", "kunit_binary_assert_format")
SUITE = b"math-prime_numbers"
CASE = b"prime_numbers_test"
COUNT = 65536
ITERATIONS = ((0, 0), (0, 1), (0, 16), (1, 16), (2, 16), (4, 16),
              (9, 32), (17, 17), (18, 17), (31, 64), (64, 100))
PRIVATE_C_FLAGS = "CFLAGS_MODULE+=-D__DISABLE_EXPORTS"


def reference_source():
    """Copy exact original fallback bodies, changing only private identifiers.

    Copying the whole provider would introduce a second stateful RCU cache and
    its lifetime obligations. These original fallback functions are instead an
    independent, allocation-free oracle. Preserve slow(1)==true, and correct for
    the public bitmap's exclusion of one only at the caller comparison.
    """
    source = (ROOT / "lib/math/prime_numbers.c").read_text()
    functions = []
    for signature in ("bool slow_is_prime_number(unsigned long x)",
                      "static unsigned long slow_next_prime_number(unsigned long x)"):
        if source.count(signature) != 1: raise ValueError("original prime fallback signature changed")
        start = source.index(signature)
        opening = source.index("{", start)
        depth, end = 1, opening + 1
        while depth:
            if end >= len(source): raise ValueError("unterminated original prime fallback")
            depth += (source[end] == "{") - (source[end] == "}")
            end += 1
        functions.append(source[start:end].removeprefix("static "))
    body = "\n\n".join(functions).replace("slow_is_prime_number", "prime_reference_is").replace(
        "slow_next_prime_number", "prime_reference_next")
    return ("#include <linux/types.h>\n#include <linux/math.h>\n#include <linux/limits.h>\n"
            "bool prime_reference_is(unsigned long);\nunsigned long prime_reference_next(unsigned long);\n" + body + "\n")


C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/module.h>
#include <linux/prime_numbers.h>
#include <linux/printk.h>

bool prime_reference_is(unsigned long);
unsigned long prime_reference_next(unsigned long);
/* Distinct, non-inlined call sites permit the artifact gate to prove both
 * native prototypes retain their own indirect-call KCFI guards. */
static noinline bool primes_call_is(unsigned long value)
{
    bool (*volatile actual_is)(unsigned long) = is_prime_number;
    return actual_is(value);
}
static noinline unsigned long primes_call_next(unsigned long value)
{
    unsigned long (*volatile actual_next)(unsigned long) = next_prime_number;
    return actual_next(value);
}

static unsigned long expected_next(unsigned long value)
{
    /* The slow fallback deliberately regards one as prime; the public bitmap
     * excludes zero and one. Keep the copied oracle untouched. */
    return prime_reference_next(value ? value : 1);
}

#define PRIME_FAIL(stage, value) do { \
    pr_err("LUPOS_PRIMES_FAIL stage=%s value=%lu\n", stage, (unsigned long)(value)); \
    return -EINVAL; \
} while (0)

static int __init primes_abi_init(void)
{
    static const unsigned long bounds[][2] = { @BOUNDS@ };
    unsigned long value, prime, expected;
    unsigned int i, seen;

    if (!prime_reference_is(1) || prime_reference_is(0) ||
        prime_reference_next(0) != 1 || prime_reference_next(1) != 2) PRIME_FAIL("oracle_boundary", 0);
    for (value = 0; value < 65536; value++) {
        if (primes_call_is(value) != (value > 1 && prime_reference_is(value)) ||
            primes_call_next(value) != expected_next(value)) PRIME_FAIL("public_values", value);
    }
    if (primes_call_next(~0UL) != ~0UL) PRIME_FAIL("sentinel", ~0UL);
    for (i = 0; i < ARRAY_SIZE(bounds); i++) {
        expected = bounds[i][0];
        seen = 0;
        for_each_prime_number_from(prime, bounds[i][0], bounds[i][1]) {
            if (prime != expected || ++seen > 101) PRIME_FAIL("iterator_from", prime);
            expected = expected_next(expected);
        }
        if (expected <= bounds[i][1]) PRIME_FAIL("iterator_end", expected);
    }
    expected = 2;
    for_each_prime_number(prime, 100) {
        if (prime != expected) PRIME_FAIL("iterator_default", prime);
        expected = expected_next(expected);
    }
    if (expected != 101) PRIME_FAIL("iterator_default_end", expected);
    pr_info("LUPOS_PRIMES_ABI_OK values=65536 next=65537 iterators=12\n");
    return 0;
}
static void __exit primes_abi_exit(void) {}
module_init(primes_abi_init);
module_exit(primes_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Non-GPL prime-number public ABI check");
'''

RUST_SOURCE = r'''// SPDX-License-Identifier: GPL-2.0-only
//! Disposable non-GPL caller using the actual public bindings and inline facade.
use kernel::{bindings, ffi, primes};

unsafe extern "C" {
    fn prime_reference_is(value: ffi::c_ulong) -> bool;
    fn prime_reference_next(value: ffi::c_ulong) -> ffi::c_ulong;
}

#[inline(never)]
#[no_mangle]
/// Preserve a distinct, genuinely indirect call to the native bool prototype.
pub extern "C" fn primes_call_is(value: ffi::c_ulong) -> bool {
    let pointer: unsafe extern "C" fn(ffi::c_ulong) -> bool = bindings::is_prime_number;
    // SAFETY: The volatile copy retains the actual native function pointer.
    unsafe { core::ptr::read_volatile(&pointer)(value) }
}

#[inline(never)]
#[no_mangle]
/// Preserve a distinct, genuinely indirect call to the native long prototype.
pub extern "C" fn primes_call_next(value: ffi::c_ulong) -> ffi::c_ulong {
    let pointer: unsafe extern "C" fn(ffi::c_ulong) -> ffi::c_ulong = bindings::next_prime_number;
    // SAFETY: The volatile copy retains the actual native function pointer.
    unsafe { core::ptr::read_volatile(&pointer)(value) }
}

#[inline(always)]
fn expected_next(value: ffi::c_ulong) -> ffi::c_ulong {
    // SAFETY: Preserve the original slow oracle, while accounting for the
    // public bitmap excluding one even though slow_is_prime_number(1) is true.
    unsafe { prime_reference_next(if value == 0 { 1 } else { value }) }
}

fn failure(stage: &'static core::ffi::CStr, value: ffi::c_ulong) -> ffi::c_int {
    // SAFETY: Static format/stage strings and an exact C unsigned-long value.
    unsafe { bindings::_printk(c"\x013LUPOS_PRIMES_FAIL stage=%s value=%lu\n".as_ptr().cast(),
        stage.as_ptr().cast::<ffi::c_char>(), value); }
    -22
}

#[no_mangle]
#[link_section = ".init.text"]
/// Run the private non-GPL ABI probe at module insertion.
pub extern "C" fn init_module() -> ffi::c_int {
    // SAFETY: All functions accept any unsigned long by value. Reference
    // functions are private unchanged original fallbacks, including slow(1).
    unsafe {
        if !prime_reference_is(1) || prime_reference_is(0) ||
            prime_reference_next(0) != 1 || prime_reference_next(1) != 2 { return failure(c"oracle_boundary", 0); }
        let mut value = 0;
        while value < 65536 {
            let expected = value > 1 && prime_reference_is(value);
            let later = expected_next(value);
            if primes_call_is(value) != expected || primes_call_next(value) != later ||
                primes::is_prime_number(value) != expected ||
                primes::next_prime_number(value) != later { return failure(c"public_values", value); }
            value += 1;
        }
        if bindings::next_prime_number(ffi::c_ulong::MAX) != ffi::c_ulong::MAX { return failure(c"sentinel", ffi::c_ulong::MAX); }
        // Borrow static data: by-value array IntoIter can introduce an
        // out-of-line GPL-only core iterator import in size-optimized kernels.
        const BOUNDS: &[(ffi::c_ulong, ffi::c_ulong)] = &[@BOUNDS@];
        let mut index = 0;
        while index < BOUNDS.len() {
            let (from, maximum) = BOUNDS[index];
            let mut expected = from;
            let mut seen = 0;
            for value in primes::for_each_prime_number_from(from, maximum) {
                seen += 1;
                if value != expected || seen > 101 { return failure(c"iterator_from", value); }
                expected = expected_next(expected);
            }
            if expected <= maximum { return failure(c"iterator_end", expected); }
            index += 1;
        }
        let mut expected = 2;
        for value in primes::for_each_prime_number(100) {
            if value != expected { return failure(c"iterator_default", value); }
            expected = expected_next(expected);
        }
        if expected != 101 { return failure(c"iterator_default_end", expected); }
        bindings::_printk(c"\x016LUPOS_PRIMES_RUST_ABI_OK values=65536 next=65537 iterators=12\n".as_ptr().cast());
    }
    0
}

#[no_mangle]
#[link_section = ".exit.text"]
/// There is no retained fixture state to release.
pub extern "C" fn cleanup_module() {}

// Same typed addressability records as the loadable-module arm of module!.
// The module loader calls these entrypoints indirectly; objtool must not seal
// their ENDBR instructions. No registration/validation is disabled here.
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn() -> ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=Proprietary\0description=Non-GPL prime-number public ABI check\0";
#[used]
#[link_section = ".modinfo"]
static MODINFO: [u8; INFO.len()] = {
    let mut bytes = [0; INFO.len()];
    let mut i = 0;
    while i < bytes.len() { bytes[i] = INFO.as_bytes()[i]; i += 1; }
    bytes
};
#[used]
static __IS_RUST_MODULE: () = ();
'''


def sources(caller):
    if caller == "c":
        return C_SOURCE.replace("@BOUNDS@", ", ".join("{%d, %d}" % pair for pair in ITERATIONS))
    if caller == "rust":
        return RUST_SOURCE.replace("@BOUNDS@", ", ".join("(%d, %d)" % pair for pair in ITERATIONS))
    raise ValueError("unknown prime caller")


def states(config):
    provider, suite, framework = (config.get(name, "n") for name in
                                  ("PRIME_NUMBERS", "PRIME_NUMBERS_KUNIT_TEST", "KUNIT"))
    if provider not in ("y", "m") or suite not in ("n", "y", "m"):
        raise ValueError("requires enabled prime provider and valid suite state")
    if suite != "n" and (framework not in ("y", "m") or suite == "y" and
                          (provider == "m" or framework == "m")):
        raise ValueError("incompatible prime provider/suite/framework states")
    if any(config.get(name) == "y" for name in ("RUST_PRIME_NUMBERS", "RUST_PRIME_NUMBERS_KUNIT_TEST")) and config.get("RUST") != "y":
        raise ValueError("Rust prime selections require CONFIG_RUST=y")
    return provider, suite, framework


def selected_versions(build):
    config = configuration(build)
    provider, suite, framework = states(config)
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    owner = b"vmlinux" if provider == "y" else b"lib/math/prime_numbers"
    expected = {name: (owner, b"EXPORT_SYMBOL") for name in PUBLIC + (PRIVATE if suite != "n" else ())}
    if suite != "n":
        expected.update({name: (b"lib/kunit/kunit" if framework == "m" else b"vmlinux", b"EXPORT_SYMBOL_GPL")
                         for name in KUNIT_IMPORTS})
    result = {}
    for symbol in PUBLIC + PRIVATE + (KUNIT_IMPORTS if suite != "n" else ()):
        matches = [row for row in rows if len(row) >= 2 and row[1] == symbol.encode()]
        if symbol not in expected:
            if matches: raise ValueError("conditional prime test export present with suite disabled")
            continue
        if (len(matches) != 1 or len(matches[0]) != 4 or tuple(matches[0][2:]) != expected[symbol]
                or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", matches[0][0])):
            raise ValueError("wrong prime/KUnit symbol owner, license, namespace or CRC: " + symbol)
        result[symbol] = matches[0][0].lower()
    return result


def verify_metadata(obj, prefix=""):
    data = metadata_fields(obj)
    prefix = (prefix + ".").encode() if prefix else b""
    for name, value in ((b"license", b"GPL"), (b"author", b"Intel Corporation"),
                        (b"description", b"Prime number library")):
        require_metadata_field(data, prefix + name, value)


def verify_import_versions(build, generated, symbols, versions):
    if configuration(build).get("MODVERSIONS") == "y":
        for name in symbols:
            if imported_crc(generated.read_text(), name) != versions[name]:
                raise ValueError("prime caller/suite CRC differs from actual selected provider: " + name)


def verify_module_import_versions(build, module):
    """Check the final ELF's active version records, not generated-C text.

    Reject even unreferenced strong imports without versions: the loader still
    resolves them. In particular, metadata-only KCFI orphans have no exemption.
    A truly unresolved weak symbol remains optional, as in modpost/the loader;
    a weak symbol found in Module.symvers still needs its correct CRC.
    """
    config = configuration(build)
    if config.get("MODVERSIONS") != "y": return {}
    _, sections, symbols, _, _ = structural_elf(module)

    def section(name):
        matches = [entry[-1] for entry in sections if entry[0] == name]
        if len(matches) != 1 or not isinstance(matches[0], bytes):
            raise ValueError("missing or duplicate module version section: " + os.fsdecode(name))
        return matches[0]

    if config.get("EXTENDED_MODVERSIONS") == "y":
        crcs, names = section(b"__version_ext_crcs"), section(b"__version_ext_names")
        if len(crcs) % 4 or not names.endswith(b"\0"):
            raise ValueError("malformed extended module versions")
        values = [value[0] for value in struct.iter_unpack("<I", crcs)]
        records = []
        for crc in values:
            name, terminator, names = names.partition(b"\0")
            if not terminator: raise ValueError("extended module version name/CRC count mismatch")
            records.append((name, crc))
        # modpost emits concatenated "symbol\0" C literals: their final
        # implicit terminator is not another version record. No other trailing
        # payload (including additional empty records) is accepted.
        if names not in (b"", b"\0"):
            raise ValueError("extended module version name/CRC count mismatch")
    else:
        data = section(b"__versions")
        if len(data) % 64: raise ValueError("malformed basic module versions")
        records = []
        for offset in range(0, len(data), 64):
            crc = struct.unpack_from("<Q", data, offset)[0]
            raw = data[offset + 8:offset + 64]
            if b"\0" not in raw or any(raw[raw.index(0):]) or crc > 0xffffffff:
                raise ValueError("malformed basic module version record")
            records.append((raw.split(b"\0", 1)[0], crc))
    versions = {}
    for name, crc in records:
        if not name or name in versions: raise ValueError("empty or duplicate module version name")
        versions[name] = crc
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    exports = {}
    for name in set(versions) | {symbol[0] for symbol in symbols if symbol[3] == 0 and symbol[0]} | {b"module_layout"}:
        matches = [row for row in rows if len(row) >= 2 and row[1] == name]
        if not matches: continue
        if (len(matches) != 1 or len(matches[0]) not in (4, 5) or
                not re.fullmatch(rb"0x[0-9a-fA-F]{8}", matches[0][0]) or
                matches[0][3] not in (b"EXPORT_SYMBOL", b"EXPORT_SYMBOL_GPL")):
            raise ValueError("ambiguous or invalid selected symbol version: " + os.fsdecode(name))
        if name == b"module_layout" and matches[0][2] != b"vmlinux":
            raise ValueError("module_layout is not supplied by vmlinux")
        exports[name] = int(matches[0][0], 16)
    for name, crc in versions.items():
        if exports.get(name) != crc:
            raise ValueError("final module CRC differs from selected Module.symvers: " + os.fsdecode(name))
    required = {b"module_layout"}
    seen = set()
    for name, info, _, index, _, _ in symbols:
        if index or not name: continue
        if name in seen or info >> 4 not in (1, 2):
            raise ValueError("ambiguous or unsupported undefined module symbol")
        seen.add(name)
        if info >> 4 == 2 and name not in exports: continue
        required.add(name)
    for name in required:
        if name not in exports or name not in versions:
            raise ValueError("unversioned final module import: " + os.fsdecode(name))
    return versions


def verify_rust_api(build, *, kunit=False, members=None):
    required = [ROOT / "include/linux/prime_numbers_header.rs"]
    if kunit: required.append(ROOT / "rust/kernel/kunit.rs")
    obj = build / "rust/kernel.o"
    if members is None:
        members = {(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", build / "vmlinux.a").splitlines()}
    if obj.resolve() not in members: raise ValueError("actual kernel::primes is not linked")
    verify_build_command(build, obj, ROOT / "rust/kernel/lib.rs", required)
    newer(build / "rust/libkernel.rmeta", [ROOT / "rust/kernel/lib.rs", *required])
    binding_headers = [ROOT / "include/linux/prime_numbers.h"]
    if kunit:
        binding_headers += [ROOT / "lib/math/prime_numbers_private.h", ROOT / "include/kunit/test.h"]
    newer(build / "rust/libbindings.rmeta", binding_headers)
    newer(build / "vmlinux.a", [obj])


def verify_linked_implementation(build, selection):
    config = configuration(build)
    provider, suite, framework = states(config)
    if selection != ("Rust" if config.get("RUST_PRIME_NUMBERS") == "y" else "C"):
        raise ValueError("prime provider selection contradicts configuration")
    arch = architecture(config)
    archive = build / "vmlinux.a"
    listing = tool("ar", "t", archive).splitlines()
    members = {(build / os.fsdecode(line)).resolve() for line in listing}
    if len(members) != len(listing): raise ValueError("duplicate vmlinux archive members")
    owner = (build / "lib/math/prime_numbers.o").resolve()
    test = (build / "lib/math/tests/prime_numbers_kunit.o").resolve()
    if (owner in members) != (provider == "y") or (test in members) != (suite == "y"):
        raise ValueError("prime provider/suite archive selection mismatch")
    source = ROOT / "lib/math" / ("prime_numbers.rs" if selection == "Rust" else "prime_numbers.c")
    deps = ([ROOT / "lib/math/prime_numbers_mutex.rs", ROOT / "rust/ffi_export.rs", ROOT / "include/linux/export_header.rs",
             build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust" else
            [ROOT / "include/linux/prime_numbers.h", ROOT / "lib/math/prime_numbers_private.h"])
    verify_build_command(build, owner, source, deps)
    elf_target(owner, arch)
    symbols = PUBLIC + (PRIVATE if suite != "n" else ())
    records = read_exports(owner)
    if sorted(record["name"] for record in records) != sorted(symbols):
        raise ValueError("prime owner has wrong conditional exports")
    for record in records:
        expected = dict(license="", namespace="", relocation_target=record["name"], relocation_addend=0,
                        pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1,
                        label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
        if any(record[key] != value for key, value in expected.items()):
            raise ValueError("invalid prime export metadata/relocation")
    defined = [line.split() for line in tool("nm", "--defined-only", owner).splitlines()]
    for name in symbols:
        matches = [row for row in defined if row[-1:] == [name.encode()]]
        if len(matches) != 1 or matches[0][-2] != b"T": raise ValueError("prime export is not a global owner function")
    versions = selected_versions(build)
    if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("." + owner.name + ".cmd")) != {
            name.encode(): versions[name] for name in symbols}:
        raise ValueError("prime version CRCs do not come from the selected defining object")
    verify_metadata(owner, "prime_numbers" if provider == "y" else "")
    if provider == "y": newer(archive, [owner])
    modules = [verify_framework_module(build, members, arch)] if suite != "n" and framework == "m" else []
    if suite != "n":
        if framework == "y":
            for name in ("test", "assert"):
                obj = build / "lib/kunit" / (name + ".o")
                if obj.resolve() not in members: raise ValueError("builtin KUnit framework is not linked")
                verify_build_command(build, obj, ROOT / "lib/kunit" / (name + ".c"))
                newer(archive, [obj])
        rust_tests = config.get("RUST_PRIME_NUMBERS_KUNIT_TEST") == "y"
        test_source = ROOT / "lib/math/tests" / ("prime_numbers_kunit.rs" if rust_tests else "prime_numbers_kunit.c")
        test_deps = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta",
                      ROOT / "lib/math/prime_numbers_private_header.rs"] if rust_tests else
                     [ROOT / "include/kunit/test.h", ROOT / "include/linux/prime_numbers.h", ROOT / "lib/math/prime_numbers_private.h"])
        verify_build_command(build, test, test_source, test_deps)
        elf_target(test, arch)
        verify_references(test, (*PUBLIC, *PRIVATE, *KUNIT_IMPORTS))
        sizes = re.findall(rb"\]\s+\.kunit_test_suites\s+PROGBITS\s+[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\b", tool("readelf", "-SW", test))
        if len(sizes) != 1 or int(sizes[0], 16) != 8: raise ValueError("prime KUnit needs exactly one suite registration")
        verify_metadata(test, "prime_numbers_kunit" if suite == "y" else "")
        if rust_tests: verify_rust_api(build, kunit=True, members=members)
        if suite == "y": newer(archive, [test])
    for state, stem, obj, name in ((provider, "lib/math/prime_numbers", owner, "prime_numbers"),
                                  (suite, "lib/math/tests/prime_numbers_kunit", test, "prime_numbers_kunit")):
        if state != "m": continue
        module = build / (stem + ".ko")
        verify_module(build, module, obj, name, arch)
        newer(module, [module.with_suffix(".mod.c")])
        verify_metadata(module)
        verify_module_import_versions(build, module)
        if name == "prime_numbers_kunit":
            verify_references(module, (*PUBLIC, *PRIVATE, *KUNIT_IMPORTS))
            verify_import_versions(build, module.with_suffix(".mod.c"), (*PUBLIC, *PRIVATE, *KUNIT_IMPORTS), versions)
        modules.append(module)
    newer(build / "Module.symvers", [owner, *([test] if suite != "n" else [])])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return modules


def verify_consumer(build, work, caller):
    name = "primes_rust_abi" if caller == "rust" else "primes_abi"
    obj = work / ("primes_rust_main.o" if caller == "rust" else "primes_c_main.o")
    module, reference = work / (name + ".ko"), work / "prime_reference.o"
    config, arch = configuration(build), architecture(configuration(build))
    for path in (obj, module):
        elf_target(path, arch)
        verify_references(path, PUBLIC)
    if module_name(module) != name: raise ValueError("wrong prime consumer module identity")
    deps = ([build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if caller == "rust" else
            [ROOT / "include/linux/prime_numbers.h"])
    verify_build_command(work, obj, obj.with_suffix(".rs" if caller == "rust" else ".c"), deps)
    if obj.with_suffix(".rs" if caller == "rust" else ".c").read_text() != sources(caller):
        raise ValueError("private prime caller source differs from the checked fixture")
    verify_build_command(work, reference, reference.with_suffix(".c"), [ROOT / "include/linux/math.h"])
    if reference.with_suffix(".c").read_text() != reference_source(): raise ValueError("private prime oracle differs from original fallback")
    newer(reference, [ROOT / "lib/math/prime_numbers.c"])
    newer(module, [obj, reference, module.with_suffix(".mod.c")])
    verify_import_versions(build, module.with_suffix(".mod.c"), PUBLIC, selected_versions(build))
    verify_module_import_versions(build, module)
    require_metadata_field(metadata_fields(module), b"license", b"Proprietary")
    require_metadata_field(metadata_fields(module), b"description", b"Non-GPL prime-number public ABI check")
    if read_exports(module): raise ValueError("private prime caller unexpectedly exports symbols")
    undefined = {line.split()[-1] for line in tool("nm", "-u", module).splitlines() if line.split()}
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    if any(len(row) >= 4 and row[1] in undefined and row[3] != b"EXPORT_SYMBOL" for row in rows):
        raise ValueError("Proprietary prime caller imports a GPL-only symbol")
    if undefined & {name.encode() for name in PRIVATE}: raise ValueError("public caller depends on conditional prime test exports")
    if caller == "rust":
        imports = {line.split()[-1] for line in tool("nm", "-u", obj).splitlines() if line.split()}
        if imports != {b"_printk", b"is_prime_number", b"next_prime_number", b"prime_reference_is", b"prime_reference_next"}:
            raise ValueError("Rust Proprietary caller has unexpected runtime/core imports")
        for path in (obj, module): verify_rust_entrypoints(path, arch)
    # Every generated C translation unit must suppress only incidental module
    # header addressability records, including when the main caller is Rust.
    for path in (reference, work / (name + ".mod.o"), *([obj] if caller == "c" else [])):
        flags = compilation_flags(path)
        if "-D__DISABLE_EXPORTS" not in flags or "-U__DISABLE_EXPORTS" in flags or read_exports(path):
            raise ValueError("private prime C fixture contains incidental export metadata")
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    if config.get("CFI") == "y":
        flags = compilation_flags(obj)
        expected = "-Zsanitizer=kcfi" if caller == "rust" else "-fsanitize=kcfi"
        if expected not in flags or any(flag.startswith("-fno-sanitize=") and
                {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")) for flag in flags):
            raise ValueError("prime indirect caller lacks actual KCFI compilation")
        types = provider_type_ids(build / "lib/math/prime_numbers.o")
        for path in (obj, module):
            verify_guarded_calls(path, arch, types)


def verify_rust_entrypoints(obj, arch):
    """Require module!'s real entrypoint addressability, not an objtool bypass."""
    _, sections, symbols, _, _ = structural_elf(obj)
    for section_name, name in ((b".init.data", b"init_module"), (b".exit.data", b"cleanup_module")):
        functions = [symbol for symbol in symbols if symbol[0] == name and symbol[1] == 0x12 and symbol[3]]
        if len(functions) != 1: raise ValueError("missing native Rust module entrypoint")
        data = [(i, section) for i, section in enumerate(sections) if section[0] == section_name]
        if len(data) != 1 or data[0][1][4] != 8 or data[0][1][7] != 8:
            raise ValueError("missing typed Rust module entrypoint addressability record")
        relocations = [relocation for section in sections if section[1] == 4 and section[6] == data[0][0]
                       for relocation in section[-1]]
        if (len(relocations) != 1 or relocations[0][0] != 0 or relocations[0][3] != 0 or
                relocations[0][1] != (257 if arch == "aarch64" else 1) or relocations[0][2] != functions[0]):
            raise ValueError("Rust module entrypoint addressability points to the wrong function")


def provider_type_ids(owner, *, names=PUBLIC):
    """Read the actual KCFI prefix immediately before each native definition."""
    _, sections, symbols, _, _ = structural_elf(owner)
    result = {}
    for name in names:
        matches = [symbol for symbol in symbols if symbol[0] == name.encode() and symbol[1] & 15 == 2]
        if len(matches) != 1: raise ValueError("missing native prime KCFI function")
        _, _, _, index, value, _ = matches[0]
        if not 0 < index < len(sections): raise ValueError("undefined native prime KCFI function")
        section = sections[index]
        offset = value - section[3]
        data = section[-1]
        if not isinstance(data, bytes) or offset < 4 or offset > len(data):
            raise ValueError("missing native prime KCFI type prefix")
        result[name] = int.from_bytes(data[offset - 4:offset], "little")
    return result


def verify_guarded_calls(obj, arch, types, *, wrappers=(("primes_call_is", PUBLIC[0]), ("primes_call_next", PUBLIC[1]))):
    """Prove the named import sites retain their actual, correctly typed guard.

    Check emitted instructions, register identity, trap/branch destination and
    the defining provider's type hash, in the input object AND final module.
    A sanitizer flag or an unrelated trap section alone is not evidence.
    """
    if not wrappers or len({wrapper for wrapper, _ in wrappers}) != len(wrappers):
        raise ValueError("missing or duplicate prime guarded wrappers")
    if any(native not in types for _, native in wrappers):
        raise ValueError("missing defining provider KCFI type")
    assembly = subprocess.run([*shlex.split(os.environ.get("LLVM_OBJDUMP", "llvm-objdump")),
        "-dr", "--no-show-raw-insn", str(obj)], check=True, capture_output=True, timeout=120,
        env={**os.environ, "LC_ALL": "C"}).stdout.decode()
    blocks = {}
    current = None
    for line in assembly.splitlines():
        label = re.fullmatch(r"[0-9a-f]+ <([^>]+)>:", line)
        if label:
            current = label[1]
            if current in blocks: raise ValueError("duplicate prime function in disassembly")
            blocks[current] = []
        elif line.startswith("Disassembly of section"):
            current = None
        elif current is not None:
            blocks[current].append(line)
    for wrapper, native in wrappers:
        lines = blocks.get(wrapper, [])
        references = re.findall(r"\bR_\w+\s+(\S+)", "\n".join(lines))
        if not references or any(reference != native for reference in references):
            raise ValueError("prime guarded wrapper does not reference its native import: " + wrapper)
        instructions = []
        for line in lines:
            match = re.match(r"\s*([0-9a-f]+):\s+(\S+)(?:\s+(.*?))?\s*$", line)
            if match and not match[2].startswith("R_"):
                args = re.split(r"\s+(?:#|//) ", match[3] or "", maxsplit=1)[0]
                instructions.append((int(match[1], 16), match[2], args))
        protected = 0
        indirect = 0
        for i, (address, op, args) in enumerate(instructions):
            if arch == "x86_64":
                if op not in ("call", "callq", "jmp", "jmpq") or not args.startswith("*"): continue
                indirect += 1
                if i < 4: continue
                load, add, branch, trap = instructions[i - 4:i]
                register = re.fullmatch(r"\*(%r\w+)", args)
                immediate = re.fullmatch(r"\$(0x[0-9a-f]+|\d+), (%r\w+d)", load[2])
                if not register or not immediate or load[1] != "movl": continue
                expected_add = "-0x4(" + register[1] + "), " + immediate[2]
                if add[1:] != ("addl", expected_add) or ((-int(immediate[1], 0)) & 0xffffffff) != types[native]: continue
                if branch[1] not in ("je", "jz") or trap[1] != "ud2": continue
            elif arch == "aarch64":
                if op not in ("blr", "br"): continue
                indirect += 1
                if i < 6: continue
                load, low, high, compare, branch, trap = instructions[i - 6:i]
                type_load = re.fullmatch(r"(w\d+), \[" + re.escape(args) + r", #-(?:0x4|4)\]", load[2])
                low_value = re.fullmatch(r"(w\d+), #(0x[0-9a-f]+|\d+)", low[2])
                high_value = re.fullmatch(r"(w\d+), #(0x[0-9a-f]+|\d+), lsl #16", high[2])
                if not type_load or not low_value or not high_value or load[1] != "ldur": continue
                if low[1] not in ("mov", "movk", "movz") or high[1] != "movk" or low_value[1] != high_value[1]: continue
                if int(low_value[2], 0) | (int(high_value[2], 0) << 16) != types[native]: continue
                if compare[1:] != ("cmp", type_load[1] + ", " + low_value[1]): continue
                if branch[1] != "b.eq" or trap[1] != "brk" or not re.fullmatch(r"#0x8[0-3][0-9a-f]{2}", trap[2]): continue
            else:
                raise ValueError("unsupported prime KCFI architecture")
            destination = re.match(r"0x([0-9a-f]+)\b", branch[2])
            if destination and int(destination[1], 16) == address: protected += 1
        if protected != 1 or indirect != 1:
            raise ValueError("prime wrapper lacks exactly one correctly typed protected indirect call: " + wrapper)
    if arch == "x86_64":
        _, sections, _, _, _ = structural_elf(obj)
        traps = [section for section in sections if section[0] in (b".kcfi_traps", b"__kcfi_traps")]
        # Clang emits distinct SHF_LINK_ORDER trap sections for .text and
        # .init.text. A final module linker may merge them into __kcfi_traps.
        # Validate every section, not merely a sum that could hide malformed
        # entries or a link-order association with non-executable storage.
        if not traps or any(section[1] != 1 or section[2] & ~0x82 or not section[2] & 2 or
                            not section[4] or section[4] % 4 for section in traps):
            raise ValueError("prime indirect caller lacks emitted KCFI trap records")
        for section in traps:
            if section[2] & 0x80 and (not 0 < section[5] < len(sections) or
                                      sections[section[5]][2] & 6 != 6):
                raise ValueError("prime KCFI traps link to a non-executable section")
        if sum(section[4] for section in traps) < 4 * len(wrappers):
            raise ValueError("prime indirect caller lacks emitted KCFI trap records")


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).strip() for line in console.splitlines()]


def kunit_runs(console, runs=1):
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: " + SUITE]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+" + SUITE + rb"(?:\s.*)?", line)]
    if runs not in (0, 1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("missing or duplicate prime KUnit suites")
    intervals = []
    for start, end in zip(starts, ends):
        if start >= end or intervals and start <= intervals[-1][1]: raise ValueError("overlapping prime suites")
        plans, results, dumps, summaries = [], [], [], set()
        for i in range(start + 1, end + 1):
            line = lines[i]
            if re.match(rb"\d+\.\.", line):
                if line != b"1..1" or plans or results: raise ValueError("wrong or misplaced prime KUnit plan")
                plans.append(i)
            elif line.startswith(b"# Subtest:"): raise ValueError("unexpected parameterization of prime case")
            elif re.match(rb"(?:not )?ok\s", line):
                if line == b"ok 1 " + CASE and plans and not results:
                    results.append(i)
                elif i == end and re.fullmatch(rb"ok [1-9]\d* " + SUITE, line):
                    if len(results) != 1 or len(dumps) != 1: raise ValueError("premature prime suite completion")
                else: raise ValueError("wrong, failed, duplicate or skipped prime case")
            elif b"primes.{" in line:
                match = re.fullmatch(rb"(?:# math-prime_numbers: )?primes\.\{last=(\d+), \.sz=(\d+), \.primes\[\]=\.\.\.x([0-9a-f]+)\} = ([0-9,-]+)", line)
                if not match or not results or dumps or summaries & {SUITE, b"Totals"}:
                    raise ValueError("missing, malformed or misplaced prime suite-exit dump")
                last, size = map(int, match.groups()[:2])
                final_word = int(match[3], 16)
                if (size < 65536 or size % 64 or last >= size or last < 65521 or
                        not 0 < final_word < (1 << 64) or size - 64 + final_word.bit_length() - 1 != last):
                    raise ValueError("prime suite-exit cache fields contradict completed corpus")
                dumps.append(i)
            elif re.match(rb"# (?:prime_numbers_test|math-prime_numbers|Totals):", line):
                match = re.fullmatch(rb"# (prime_numbers_test|math-prime_numbers|Totals): pass:1 fail:0 skip:0 total:1", line)
                if not match or match[1] in summaries: raise ValueError("contradictory or duplicate prime totals")
                label = match[1]
                if (label == CASE and results) or (label != CASE and not dumps): raise ValueError("misplaced prime totals")
                summaries.add(label)
            elif line.startswith(b"# module:") and line != b"# module: prime_numbers_kunit":
                raise ValueError("wrong prime KUnit module identity")
        if len(plans) != 1 or len(results) != 1 or len(dumps) != 1: raise ValueError("incomplete prime KUnit lifecycle")
        intervals.append((start, end))
    for i, line in enumerate(lines):
        if (b"primes.{" in line or re.fullmatch(rb"(?:not )?ok\s+\d+\s+prime_numbers_test(?:\s.*)?", line)) and not any(a < i < b for a, b in intervals):
            raise ValueError("prime case or cache dump outside its suite")
    return intervals


def result_marker(caller):
    if caller not in ("c", "rust"): raise ValueError("unknown prime caller")
    return b"LUPOS_PRIMES_" + (b"RUST_ABI" if caller == "rust" else b"ABI") + b"_OK values=65536 next=65537 iterators=12"


def verify_console(console, caller, *, provider_module=False, suite="y", framework_module=False, reload=False):
    if suite not in ("n", "y", "m") or (suite == "y" and (provider_module or framework_module)) or (suite == "n" and framework_module):
        raise ValueError("incompatible prime runtime module states")
    if re.search(rb"CFI failure|BUG:|WARNING:|Oops:|Kernel panic|UBSAN:|KASAN:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s", console, re.I):
        raise ValueError("unexpected prime kernel fault or KUnit failure")
    lines = console_lines(console)
    ranges = kunit_runs(console, 0 if suite == "n" else 2 if suite == "m" and reload else 1)
    preloads = int(framework_module) + int(provider_module) + int(suite == "m")
    marker = result_marker(caller)
    expected = [b"KUNIT"] if suite == "y" else []
    for index in range(preloads):
        if suite == "m" and index == preloads - 1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for index in range(preloads):
            if suite == "m" and index == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(b"LUPOS_RUST_BUILD_BOOT_OK")
    actual = []
    for i, line in enumerate(lines):
        if any(i == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start <= i <= end for start, end in ranges): raise ValueError("loader event inside unfinished prime suite")
            actual.append(line)
    if actual != expected: raise ValueError("prime caller/suite/dependency events are missing, duplicated or reordered")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)
    return len(ranges)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--reload-modules", action="store_true")
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=Path)
    args = parser.parse_args()
    build = args.build.resolve()
    try:
        config = configuration(build)
        provider, suite, framework = states(config)
        selection = "Rust" if config.get("RUST_PRIME_NUMBERS") == "y" else "C"
        if selection == "C" and not args.allow_c_baseline: raise ValueError("requires RUST_PRIME_NUMBERS=y or --allow-c-baseline")
        for option in ("MODULES", "PRINTK", "MULTIUSER"):
            if config.get(option) != "y": raise ValueError("requires CONFIG_" + option + "=y")
        if args.caller == "rust" and config.get("RUST") != "y": raise ValueError("Rust caller requires RUST=y")
        if args.reload_modules and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires MODULE_UNLOAD=y")
        if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("disposable modules require signatures not forced")
        arch = architecture(config)
        preloads = verify_linked_implementation(build, selection)
        if args.caller == "rust": verify_rust_api(build)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-prime-numbers-test"
    work.mkdir(exist_ok=True)
    name, source = ("primes_rust_abi", "primes_rust_main") if args.caller == "rust" else ("primes_abi", "primes_c_main")
    (work / "prime_reference.c").write_text(reference_source())
    (work / (source + (".rs" if args.caller == "rust" else ".c"))).write_text(sources(args.caller))
    (work / "Makefile").write_text(f"obj-m := {name}.o\n{name}-y := {source}.o prime_reference.o\n")
    env = {key: value for key, value in os.environ.items() if not key.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and key not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES", "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT), "O=" + str(build), "M=" + str(work),
                    *args.make_arg, PRIVATE_C_FLAGS, "modules"], env=env, check=True)
    verify_consumer(build, work, args.caller)
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"), "--build", str(build), "--arch", arch,
               "--module", str(work / (name + ".ko")), "--qemu", args.qemu or ("qemu-system-aarch64" if arch == "aarch64" else "qemu-system-x86_64")]
    for module in preloads: command += ["--preload-module", str(module)]
    if args.reload_modules: command += ["--reload-modules"]
    if args.qemu_data: command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    count = verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller,
        provider_module=provider == "m", suite=suite, framework_module=suite != "n" and framework == "m", reload=args.reload_modules)
    print(f"{selection} primes: Proprietary {args.caller} caller passed {COUNT} primality and {COUNT + 1} next-prime checks, "
          f"12 iterator cases/load; {count} selected KUnit runs on {arch}. Allocation-failure/RCU stress gate not run.")


if __name__ == "__main__":
    main()
