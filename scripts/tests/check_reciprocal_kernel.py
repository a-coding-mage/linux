#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Check reciprocal constructors/division and real SLUB caches inside QEMU.

The completed little-endian x86-64 or ARM64 kernel may use C or native Rust.
Fixtures are private external modules, never production code or host loads.
Both callers exercise the real C aggregate-return ABI. The Rust caller also
checks the independent safe API. A shared temporary C workload uses actual
kernel slab APIs; no allocator structures are re-declared in either language.
The proprietary C ABI caller uses a separate GPL allocator preload, so incidental
page-helper dependencies from allocator headers cannot weaken the export test.
"""

import argparse
import os
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys
import tempfile

from boot_kernel import verify_module_events
from check_div64_kernel import architecture, configuration, verify_build_command


ROOT = Path(__file__).resolve().parents[2]
BASIC_COUNT = 69728
ADVANCED_COUNT = 16811
SLUB_MARKER = b"LUPOS_RECIPROCAL_SLUB_OK caches=24 allocations=3456"
RESULT = b"basic=69728 divide=557824 advanced=16811"
ORPHAN_METADATA_IMPORTS = frozenset({b"__clear_pages_unrolled", b"copy_page", b"__memset", b"__memmove"})


def module_elf(path):
    """Read the private ELF64 module and retain every structural comparison."""
    data = path.read_bytes()
    if data[:6] != b"\x7fELF\x02\x01":
        raise ValueError("fixture cleanup requires a little-endian ELF64 module")
    header = struct.unpack_from("<16sHHIQQQIHHHHHH", data)
    if header[1] != 1 or header[2] not in (62, 183) or header[10] or not header[12]:
        raise ValueError("unexpected private module ELF header")
    if header[11] != 64 or header[13] >= header[12]:
        raise ValueError("unsupported private module section table")
    sections = [struct.unpack_from("<IIQQQQIIQQ", data, header[6] + index * 64)
                for index in range(header[12])]
    # Fresh private modules are unsigned. Do not let objcopy silently discard
    # an appended signature or opaque non-section data from an arbitrary file.
    regions = [(0, 64), (header[6], header[6] + header[12] * 64)]
    regions += [(section[4], section[4] + section[5]) for section in sections
                if section[1] != 8 and section[5]]
    end = 0
    for start, stop in sorted(regions):
        if start > end and any(data[end:start]):
            raise ValueError("private module contains non-section payload")
        end = max(end, stop)
    if any(data[end:]):
        raise ValueError("private module contains a signature or trailing payload")

    def payload(section):
        if section[1] == 8:  # NOBITS has a size but no file payload.
            return b""
        start, size = section[4:6]
        if start + size > len(data):
            raise ValueError("truncated private module section")
        return data[start:start + size]

    names = payload(sections[header[13]])

    def string(table, offset):
        if offset >= len(table):
            raise ValueError("bad private module string offset")
        return table[offset:table.index(0, offset)]

    labels = [string(names, section[0]) for section in sections]
    tables = [index for index, section in enumerate(sections) if section[1] == 2]
    if len(tables) != 1 or any(section[1] == 11 for section in sections):
        raise ValueError("unexpected private module symbol tables")
    table_index = tables[0]
    table = sections[table_index]
    if table[9] != 24 or table[5] % 24 or table[6] >= len(sections):
        raise ValueError("invalid private module symbol table")
    strings = payload(sections[table[6]])
    symbols = []
    for offset in range(0, table[5], 24):
        name, info, other, section, value, size = struct.unpack_from("<IBBHQQ", payload(table), offset)
        symbols.append((string(strings, name), info, other, section, value, size))
    referenced = set()
    normalized = []
    imports = set()
    for index, section in enumerate(sections):
        name, kind = labels[index], section[1]
        content = payload(section)
        if kind in (4, 9):
            stride = 24 if kind == 4 else 16
            if section[6] != table_index or section[9] != stride or section[5] % stride:
                raise ValueError("unexpected private module relocation table")
            relocations = []
            for offset in range(0, len(content), stride):
                address, info = struct.unpack_from("<QQ", content, offset)
                symbol, relocation = info >> 32, info & 0xffffffff
                if symbol >= len(symbols):
                    raise ValueError("invalid private module relocation symbol")
                referenced.add(symbol)
                addend = struct.unpack_from("<q", content, offset + 16)[0] if kind == 4 else None
                relocations.append((address, relocation, symbols[symbol], addend))
            content = tuple(relocations)
        if name == b"__version_ext_names":
            imports.update(content.rstrip(b"\0").split(b"\0"))
        elif name == b"__versions":
            if len(content) % 64:
                raise ValueError("unexpected legacy module version records")
            imports.update(content[offset + 8:offset + 64].split(b"\0", 1)[0]
                           for offset in range(0, len(content), 64))
        if index in (table_index, table[6], header[13]):
            # Symbol/string offsets and lengths necessarily change after a
            # symbol deletion. Names, all symbol fields and section metadata
            # are compared independently below; no executable bytes skipped.
            content = None
        normalized.append((name, kind, section[2], section[3],
                           None if content is None else section[5], section[6],
                           section[7], section[8], section[9], content))
    identity = (header[:6], header[7:11], header[11:])
    return identity, normalized, symbols, referenced, imports


def clean_private_metadata_orphans(module):
    """Remove only proven orphan names created by metadata-only C includes.

    module-common.c and generated .mod.c include KCFI_REFERENCE page helpers
    after modpost. The final linker discards their addressable sections but
    both GNU ld and LLD retain the now-unreferenced undefined names. This
    fixture-only cleanup never drops a relocation or a versioned import and
    never exempts the actual constructors from GPL checks or protected calls.
    """
    before = module_elf(module)
    identity, sections, symbols, referenced, imports = before
    candidates = [(index, symbol) for index, symbol in enumerate(symbols)
                  if symbol[0] in ORPHAN_METADATA_IMPORTS and symbol[3] == 0]
    if not candidates:
        return []
    names = {symbol[0] for _, symbol in candidates}
    if len(names) != len(candidates) or any(sum(symbol[0] == name for symbol in symbols) != 1 for name in names):
        raise ValueError("ambiguous private metadata orphan symbol")
    if any(index in referenced or symbol[0] in imports for index, symbol in candidates):
        raise ValueError("refusing to strip referenced or versioned private module import")
    with tempfile.TemporaryDirectory(prefix="reciprocal-orphans-", dir=module.parent) as temporary:
        output = Path(temporary) / module.name
        command = [*shlex.split(os.environ.get("OBJCOPY", "llvm-objcopy")),
                   *("--strip-symbol=" + name.decode("ascii") for name in sorted(names)), str(module), str(output)]
        subprocess.run(command, check=True, capture_output=True)
        after = module_elf(output)
        expected_symbols = [symbol for symbol in symbols if symbol[0] not in names]
        if (after[0] != identity or after[1] != sections or after[2] != expected_symbols or after[4] != imports):
            raise ValueError("orphan cleanup changed unrelated module contents or structure")
        output.replace(module)
    return sorted(name.decode("ascii") for name in names)


def module_fixture_header():
    """Extract unchanged module macros without unrelated heavyweight includes.

    module.h transitively emits GPL-only page-helper KCFI address references.
    The proprietary ABI test needs the real init/exit aliases and metadata,
    not those helpers. This generated private header copies the original
    macro definitions verbatim; it defines no substitute kernel types.
    """
    module = (ROOT / "include/linux/module.h").read_text()
    parameters = (ROOT / "include/linux/moduleparam.h").read_text()

    def definition(name):
        starts = list(re.finditer(r"(?m)^#define " + name + r"\(", module))
        if not starts:
            raise ValueError("missing original module macro: " + name)
        lines = module[starts[-1].start():].splitlines(keepends=True)
        selected = []
        for line in lines:
            selected.append(line)
            if not line.rstrip().endswith("\\"):
                return "".join(selected)
        raise ValueError("unterminated original module macro: " + name)

    info = parameters[parameters.index("#ifdef MODULE\n#define MODULE_PARAM_PREFIX"):
                      parameters.index("#define __MODULE_PARM_TYPE")]
    file = re.search(r"(?s)#ifdef MODULE\n#define MODULE_FILE\n.*?#endif", module)
    if file is None:
        raise ValueError("missing original MODULE_FILE definition")
    return ("/* SPDX-License-Identifier: GPL-2.0-only */\n"
            "/* Private fixture: verbatim module.h/moduleparam.h macros. */\n"
            "#ifndef MODULE\n#error This fixture must be built as a module\n#endif\n" + info +
            "\n" + file[0] + "\n" + "\n".join(definition(name) for name in
                ("module_init", "module_exit", "MODULE_LICENSE", "MODULE_DESCRIPTION")))


REFERENCE_SOURCE = r'''
#include <linux/reciprocal_div.h>
#include <linux/export.h>

/* Rename linkage, not C struct tags. Both aggregate types remain the actual
 * original header declarations, with no duplicate or invented ABI layout. */
struct reciprocal_value reciprocal_value(u32 d) __asm__("reciprocal_reference_value");
struct reciprocal_value_adv reciprocal_value_adv(u32 d, u8 prec)
    __asm__("reciprocal_reference_value_adv");
#undef EXPORT_SYMBOL
#define EXPORT_SYMBOL(symbol)
#include "@SOURCE@"
'''

SLUB_SOURCE = r'''
#include <linux/errno.h>
#include <linux/kernel.h>
#include <linux/slab.h>

int reciprocal_slub_exercise(void);

#define OBJECTS 96

static unsigned char pattern(unsigned int size, unsigned int object,
                             unsigned int byte, unsigned int generation)
{
    return size + object * 37 + byte * 13 + generation * 97;
}

static int exercise_cache(unsigned int size, unsigned int alignment, unsigned int *allocations)
{
    struct kmem_cache *cache;
    unsigned char *objects[OBJECTS] = { NULL };
    char name[48];
    unsigned int i, j, pass;
    int result = -ENOMEM;

    snprintf(name, sizeof(name), "lupos-reciprocal-%u-%u", size, alignment);
    cache = kmem_cache_create(name, size, alignment, SLAB_NO_MERGE, NULL);
    if (!cache)
        return -ENOMEM;
    if (kmem_cache_size(cache) != size) {
        result = -EINVAL;
        goto out;
    }
    for (pass = 0; pass < 2; pass++) {
        /* Pass 2 keeps every odd allocation live, replacing only even ones. */
        for (i = 0; i < OBJECTS; i++) {
            if (objects[i])
                continue;
            objects[i] = kmem_cache_alloc(cache, GFP_KERNEL);
            if (!objects[i])
                goto out;
            ++*allocations;
            if ((unsigned long)objects[i] & (alignment - 1)) {
                result = -EINVAL;
                goto out;
            }
            for (j = 0; j < size; j++)
                objects[i][j] = pattern(size, i, j, pass);
        }
        /* Check survivors as well as replacements, across full object sizes. */
        for (i = 0; i < OBJECTS; i++)
            for (j = 0; j < size; j++)
                if (objects[i][j] != pattern(size, i, j, pass && !(i & 1))) {
                    result = -EINVAL;
                    goto out;
                }
        if (pass == 0)
            for (i = 0; i < OBJECTS; i += 2) {
                kmem_cache_free(cache, objects[i]);
                objects[i] = NULL;
            }
    }
    result = 0;
out:
    for (i = OBJECTS; i > 0; i--)
        if (objects[i - 1])
            kmem_cache_free(cache, objects[i - 1]);
    kmem_cache_destroy(cache);
    return result;
}

int reciprocal_slub_exercise(void)
{
    static const unsigned int sizes[] = { 24, 40, 72, 136, 264, 520, 1000, 2040 };
    static const unsigned int alignments[] = { 8, 16, 64 };
    unsigned int size, alignment, caches = 0, allocations = 0;

    for (size = 0; size < ARRAY_SIZE(sizes); size++)
        for (alignment = 0; alignment < ARRAY_SIZE(alignments); alignment++) {
            int result = exercise_cache(sizes[size], alignments[alignment], &allocations);

            if (result) {
                pr_err("LUPOS_RECIPROCAL_FAILED SLUB size=%u align=%u error=%d\n",
                       sizes[size], alignments[alignment], result);
                return result;
            }
            caches++;
        }
    pr_info("LUPOS_RECIPROCAL_SLUB_OK caches=%u allocations=%u\n", caches, allocations);
    return 0;
}
'''

ALLOCATOR_SOURCE = r'''
#include <linux/init.h>
#include <linux/module.h>

int reciprocal_slub_exercise(void);
static int __init reciprocal_allocator_init(void) { return reciprocal_slub_exercise(); }
static void __exit reciprocal_allocator_exit(void) {}
module_init(reciprocal_allocator_init);
module_exit(reciprocal_allocator_exit);
MODULE_LICENSE("GPL");
MODULE_DESCRIPTION("Independent real SLUB workload for reciprocal ABI test");
'''

C_SOURCE = r'''
#include <linux/array_size.h>
#include <linux/bitops.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/printk.h>
#include <linux/reciprocal_div.h>
#include "reciprocal_module.h"

struct reciprocal_value reciprocal_reference_value(u32 d);
struct reciprocal_value_adv reciprocal_reference_value_adv(u32 d, u8 precision);

/* Volatile targets force both actual aggregate-return ABIs through the
 * original C signatures, including their KCFI checks when configured. */
static struct reciprocal_value (*volatile basic)(u32) = reciprocal_value;
static struct reciprocal_value_adv (*volatile advanced)(u32, u8) = reciprocal_value_adv;
static unsigned int basic_count, divide_count, advanced_count;

static int check_basic(u32 d)
{
    struct reciprocal_value actual = basic(d), reference = reciprocal_reference_value(d);
    u32 inputs[] = { 0, 1, d - 1, d, d + 1, ~0U, 0x80000000U, d * 1664525U };
    unsigned int i;

    /* Never compare indeterminate C padding: test every declared field. */
    if (actual.m != reference.m || actual.sh1 != reference.sh1 || actual.sh2 != reference.sh2)
        return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(inputs); i++)
        if (reciprocal_divide(inputs[i], actual) != inputs[i] / d ||
            reciprocal_divide(inputs[i], actual) != reciprocal_divide(inputs[i], reference))
            return -EINVAL;
    basic_count++;
    divide_count += ARRAY_SIZE(inputs);
    return 0;
}

static int check_advanced(u32 d, u8 precision)
{
    struct reciprocal_value_adv actual = advanced(d, precision);
    struct reciprocal_value_adv reference = reciprocal_reference_value_adv(d, precision);

    if (actual.m != reference.m || actual.sh != reference.sh ||
        actual.exp != reference.exp || actual.is_wide_m != reference.is_wide_m)
        return -EINVAL;
    advanced_count++;
    return 0;
}

#define CHECK(expression) do { int error = (expression); if (error) return error; } while (0)

static int __init reciprocal_abi_init(void)
{
    u32 d, state = 0x72656369;
    unsigned int bit, i, precision;

    for (d = 1; d <= 65535; d++)
        CHECK(check_basic(d));
    for (bit = 1; bit < 32; bit++) {
        d = 1U << bit;
        CHECK(check_basic(d - 1)); CHECK(check_basic(d)); CHECK(check_basic(d + 1));
    }
    CHECK(check_basic(~0U)); CHECK(check_basic(~0U - 1));
    CHECK(check_basic(0x80000000U)); CHECK(check_basic(0x80000001U));
    for (i = 0; i < 4096; i++) {
        state = state * 1664525U + 1013904223U;
        CHECK(check_basic(state | 1U));
    }
    /* Exactly the defined original domain, including precision-zero wrapping:
     * d in 1..2^31, precision <= 32 + ceil(log2(d)). No UB oracle calls. */
    for (d = 1; d <= 256; d++)
        for (precision = 0; precision <= 32 + fls(d - 1); precision++)
            CHECK(check_advanced(d, precision));
    for (bit = 1; bit < 32; bit++)
        for (d = (1U << bit) - 1; d <= (1U << bit) + 1 && d <= 0x80000000U; d++)
            for (precision = 0; precision <= 32 + fls(d - 1); precision++)
                CHECK(check_advanced(d, precision));
    for (i = 0; i < 2048; i++) {
        state = state * 1664525U + 1013904223U;
        d = (state & 0x7fffffffU) + 1;
        state = state * 1664525U + 1013904223U;
        CHECK(check_advanced(d, state % (33 + fls(d - 1))));
    }
    pr_info("LUPOS_RECIPROCAL_ABI_OK basic=%u divide=%u advanced=%u\n",
            basic_count, divide_count, advanced_count);
    return 0;
}

static void __exit reciprocal_abi_exit(void) {}
module_init(reciprocal_abi_init);
module_exit(reciprocal_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Independent proprietary C reciprocal ABI caller");
'''

RUST_SOURCE = r'''
// SPDX-License-Identifier: GPL-2.0-only
//! Real C aggregate ABI, independent safe math API, and actual SLUB workload.

use kernel::{bindings, math};
use kernel::prelude::*;

module! {
    type: ReciprocalRustAbi,
    name: "reciprocal_rust_abi",
    authors: ["Lupos"],
    description: "Reciprocal constructors and SLUB against original C",
    license: "GPL",
}

unsafe extern "C" {
    fn reciprocal_reference_value(d: u32) -> bindings::reciprocal_value;
    fn reciprocal_reference_value_adv(d: u32, precision: u8) -> bindings::reciprocal_value_adv;
    fn reciprocal_slub_exercise() -> core::ffi::c_int;
}

fn check_basic(d: u32) -> Result {
    // SAFETY: d is nonzero. Both declarations use the actual generated C
    // aggregate layout; constructors retain no pointers or external state.
    let (native, reference) = unsafe { (bindings::reciprocal_value(d), reciprocal_reference_value(d)) };
    let pure = math::reciprocal_value(d).ok_or(EINVAL)?;
    if (native.m, native.sh1, native.sh2) != (reference.m, reference.sh1, reference.sh2) ||
       (pure.m, pure.sh1, pure.sh2) != (reference.m, reference.sh1, reference.sh2) {
        return Err(EINVAL);
    }
    for a in [0, 1, d - 1, d, d.wrapping_add(1), u32::MAX, 0x80000000, d.wrapping_mul(1664525)] {
        if math::reciprocal_divide(a, pure) != Some(a / d) {
            return Err(EINVAL);
        }
    }
    Ok(())
}

fn check_advanced(d: u32, precision: u8) -> Result {
    // SAFETY: The caller enumerates only nonzero d <= 2^31 and precision <=
    // 32 + ceil(log2(d)), preserving defined shifts in the unchanged C oracle.
    let (native, reference) = unsafe {
        (bindings::reciprocal_value_adv(d, precision), reciprocal_reference_value_adv(d, precision))
    };
    let pure = math::reciprocal_value_adv(d, precision).ok_or(EINVAL)?;
    if (native.m, native.sh, native.exp, native.is_wide_m) !=
       (reference.m, reference.sh, reference.exp, reference.is_wide_m) ||
       (pure.m, pure.sh, pure.exp, pure.is_wide_m) !=
       (reference.m, reference.sh, reference.exp, reference.is_wide_m) {
        return Err(EINVAL);
    }
    Ok(())
}

struct ReciprocalRustAbi;

impl kernel::Module for ReciprocalRustAbi {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let (mut basic_count, mut advanced_count) = (0u32, 0u32);
        let mut basic = |d| -> Result { check_basic(d)?; basic_count += 1; Ok(()) };
        let mut advanced = |d, precision| -> Result {
            check_advanced(d, precision)?; advanced_count += 1; Ok(())
        };
        const ZERO_REJECTED: bool = math::reciprocal_value(0).is_none();
        if !ZERO_REJECTED || math::reciprocal_value_adv(0, 32).is_some() ||
           math::reciprocal_value_adv(0x80000001, 32).is_some() ||
           math::reciprocal_value_adv(1, 33).is_some() ||
           math::reciprocal_divide(1, math::ReciprocalValue { m: 1, sh1: 32, sh2: 0 }).is_some() ||
           math::reciprocal_divide(1, math::ReciprocalValue { m: 1, sh1: 0, sh2: 32 }).is_some() {
            return Err(EINVAL);
        }
        for d in 1..=65535 { basic(d)?; }
        for bit in 1..32 {
            let d = 1u32 << bit;
            basic(d - 1)?; basic(d)?; basic(d + 1)?;
        }
        for d in [u32::MAX, u32::MAX - 1, 0x80000000, 0x80000001] { basic(d)?; }
        let mut state = 0x72656369u32;
        for _ in 0..4096 {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            basic(state | 1)?;
        }
        for d in 1..=256u32 {
            for precision in 0..=(64 - (d - 1).leading_zeros()) {
                advanced(d, precision as u8)?;
            }
        }
        for bit in 1..32 {
            for d in ((1u32 << bit) - 1)..=((1u32 << bit) + 1) {
                if d <= 0x80000000 {
                    for precision in 0..=(64 - (d - 1).leading_zeros()) {
                        advanced(d, precision as u8)?;
                    }
                }
            }
        }
        for _ in 0..2048 {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            let d = (state & 0x7fffffff) + 1;
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            advanced(d, (state % (65 - (d - 1).leading_zeros())) as u8)?;
        }
        // SAFETY: This private test function exclusively owns and cleans up
        // its real kernel slab caches and returns a conventional error code.
        if unsafe { reciprocal_slub_exercise() } != 0 { return Err(EINVAL); }
        pr_info!("LUPOS_RECIPROCAL_RUST_API_OK basic={} divide={} advanced={}\n",
                 basic_count, basic_count * 8, advanced_count);
        Ok(Self)
    }
}
'''


def verify_linked_implementation(build, selection):
    """Check actual selected reciprocal owner and real allocator linkage."""
    config = configuration(build)
    arch = architecture(config)
    if selection not in ("C", "Rust") or selection != ("Rust" if config.get("RUST_RECIPROCAL_DIV") == "y" else "C"):
        raise ValueError("reciprocal provider does not match kernel configuration")
    archive = build / "vmlinux.a"
    listed = subprocess.run([*shlex.split(os.environ.get("AR", "ar")), "t", archive],
                            check=True, capture_output=True).stdout
    objects = {(build / os.fsdecode(line)).resolve() for line in listed.splitlines()}
    original, translated = [build / ("lib/math/" + name + ".o")
                            for name in ("reciprocal_div", "reciprocal_div_rust")]
    owner = translated if selection == "Rust" else original
    if selection not in ("Rust", "C") or objects & {original.resolve(), translated.resolve()} != {owner.resolve()}:
        raise ValueError("linked reciprocal objects do not match the " + selection + " configuration")
    for name in ("mm/slub.o", "mm/slab_common.o"):
        if (build / name).resolve() not in objects:
            raise ValueError("missing actual built-in slab allocator object: " + name)
    symbols = subprocess.run([*shlex.split(os.environ.get("NM", "nm")), "--undefined-only", build / "mm/slub.o"],
                             check=True, capture_output=True).stdout.splitlines()
    if not any(line.split()[-1:] == [b"reciprocal_value"] for line in symbols):
        raise ValueError("selected SLUB object does not consume the actual reciprocal_value export")
    sources = ["lib/math/reciprocal_div.c", "include/linux/reciprocal_div.h"] if selection == "C" else [
        "lib/math/reciprocal_div.rs", "lib/math/reciprocal_div_rust.rs",
        "rust/ffi_export.rs", "include/linux/export_header.rs"]
    if owner.stat().st_mtime_ns < max((ROOT / name).stat().st_mtime_ns for name in sources):
        raise ValueError("selected reciprocal owner is older than its source; rebuild the kernel")
    if selection == "Rust":
        # The native result types come from these actual binding metadata
        # crates. Require tracked, current dependencies rather than accepting
        # an ABI/KCFI type retained from an older generated header.
        verify_build_command(build, owner, ROOT / "lib/math/reciprocal_div_rust.rs",
                             [ROOT / name for name in sources] +
                             [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"])
    if archive.stat().st_mtime_ns < owner.stat().st_mtime_ns:
        raise ValueError("vmlinux.a is older than the selected reciprocal owner")
    image = build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage")
    if image.stat().st_mtime_ns < archive.stat().st_mtime_ns:
        raise ValueError(image.name + " is older than vmlinux.a; finish the kernel build")


def verify_console(console, caller, reload=False):
    lines = [re.sub(rb"^\[\s*\d+\.\d+\]\s?", b"", line).strip().removeprefix(
        b"reciprocal_rust_abi: ") for line in console.splitlines()]
    marker = b"LUPOS_RECIPROCAL_" + (b"RUST_API" if caller == "rust" else b"ABI") + b"_OK " + RESULT
    preloads = int(caller == "c")
    expected = [SLUB_MARKER] + ([b"LUPOS_RUST_PRELOAD_OK 0"] if preloads else [])
    expected += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {index}".encode() for index in range(preloads, -1, -1)]
        expected += [SLUB_MARKER] + ([b"LUPOS_RUST_MODULE_RELOAD_OK 0"] if preloads else [])
        expected += [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected += [b"LUPOS_RUST_BUILD_BOOT_OK"]
    if [line for line in lines if line.startswith(b"LUPOS_")] != expected:
        raise ValueError("missing, failed, duplicate or reordered reciprocal/SLUB guest checks")
    verify_module_events(console, module=True, preloads=preloads, reload=reload)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--qemu", default=os.environ.get("QEMU"))
    parser.add_argument("--qemu-data", type=Path)
    parser.add_argument("--make-arg", action="append", default=[])
    parser.add_argument("--allow-c-baseline", action="store_true")
    parser.add_argument("--caller", choices=("c", "rust"), default="c")
    parser.add_argument("--reload-modules", action="store_true")
    args = parser.parse_args()
    build = args.build.resolve()
    try:
        config = configuration(build)
        arch = architecture(config)
    except (OSError, ValueError) as error:
        parser.error(str(error))
    selection = "Rust" if config.get("RUST_RECIPROCAL_DIV") == "y" else "C"
    if selection == "C" and not args.allow_c_baseline:
        parser.error("requires CONFIG_RUST_RECIPROCAL_DIV=y (or --allow-c-baseline)")
    if selection == "Rust" and config.get("RUST") != "y":
        parser.error("CONFIG_RUST_RECIPROCAL_DIV=y requires CONFIG_RUST=y")
    for option in ("MODULES", "PRINTK", "MULTIUSER", "SLUB"):
        if config.get(option) != "y":
            parser.error("requires CONFIG_" + option + "=y")
    if args.caller == "rust" and config.get("RUST") != "y":
        parser.error("--caller rust requires CONFIG_RUST=y")
    if args.reload_modules and config.get("MODULE_UNLOAD") != "y":
        parser.error("--reload-modules requires CONFIG_MODULE_UNLOAD=y")
    if config.get("MODULE_SIG_FORCE") == "y":
        parser.error("the temporary test module requires a build without forced module signatures")
    try:
        verify_linked_implementation(build, selection)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-reciprocal-test"
    work.mkdir(exist_ok=True)
    (work / "reciprocal_reference.c").write_text(REFERENCE_SOURCE.replace("@SOURCE@", str(ROOT / "lib/math/reciprocal_div.c")))
    (work / "reciprocal_slub.c").write_text(SLUB_SOURCE)
    rust = args.caller == "rust"
    stem, source = ("reciprocal_rust_abi", "reciprocal_rust_main") if rust else ("reciprocal_abi", "reciprocal_c_main")
    (work / (source + (".rs" if rust else ".c"))).write_text(RUST_SOURCE if rust else C_SOURCE)
    if rust:
        makefile = f"obj-m := {stem}.o\n{stem}-y := {source}.o reciprocal_reference.o reciprocal_slub.o\n"
    else:
        (work / "reciprocal_module.h").write_text(module_fixture_header())
        (work / "reciprocal_allocator_main.c").write_text(ALLOCATOR_SOURCE)
        makefile = (f"obj-m := {stem}.o reciprocal_allocator.o\n"
                    f"{stem}-y := {source}.o reciprocal_reference.o\n"
                    "reciprocal_allocator-y := reciprocal_allocator_main.o reciprocal_slub.o\n")
    (work / "Makefile").write_text(makefile)
    env = {name: value for name, value in os.environ.items()
           if not name.startswith(("CONFIG_", "KCONFIG_", "KBUILD_")) and name not in
           ("MAKEFLAGS", "MFLAGS", "CARGO_MAKEFLAGS", "MAKELEVEL", "MAKEOVERRIDES",
            "srctree", "srcroot", "objtree", "VPATH", "sub_make_done")}
    subprocess.run([*shlex.split(os.environ.get("MAKE", "make")), "-C", str(ROOT),
                    "O=" + str(build), "M=" + str(work), *args.make_arg, "modules"], env=env, check=True)
    if not rust:
        removed = clean_private_metadata_orphans(work / (stem + ".ko"))
        if removed:
            print("Private fixture: removed only relocation-free, unversioned metadata imports: " + ", ".join(removed))
    command = [sys.executable, str(ROOT / "scripts/tests/boot_kernel.py"),
               "--build", str(build), "--arch", arch, "--module", str(work / (stem + ".ko")),
               "--qemu", args.qemu or "qemu-system-" + arch]
    if not rust:
        command += ["--preload-module", str(work / "reciprocal_allocator.ko")]
    if args.reload_modules:
        command.append("--reload-modules")
    if args.qemu_data:
        command += ["--qemu-data", str(args.qemu_data)]
    subprocess.run(command, env=env, check=True)
    verify_console((build / "rust-boot-test/console.log").read_bytes(), args.caller, args.reload_modules)
    print(f"{selection} reciprocal provider passed {BASIC_COUNT} basic/{ADVANCED_COUNT} advanced "
          f"{args.caller} inputs and 24 real SLUB caches/3456 allocations per load in QEMU.")


if __name__ == "__main__":
    main()
