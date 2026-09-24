#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Audit native list_sort and run a non-GPL caller only inside a disposable VM.

The original C algorithm remains the independent private oracle. Both callers
exercise the selected export through a volatile, KCFI-checked function pointer;
the Rust caller also supplies an actual Rust comparator with the original ABI.
No module is loaded on the host. Generated C files are disposable test inputs,
not replacement production sources.
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

from boot_kernel import module_name, verify_module_events
from check_cordic_kernel import tool
from check_div64_kernel import architecture, configuration, verify_build_command
from check_int_math_kernel import verify_references
from check_int_log_kernel import (normalize_console_transport, kunit_runs as log_kunit_runs,
                                  verify_kunit_warnings as log_kunit_warnings)
from check_module_common import verify_common_metadata
from check_module_metadata import selected_metadata, verify_module_metadata
from check_polynomial_kernel import elf_target, newer, verify_framework_module, verify_module
from check_prime_numbers_kernel import (provider_type_ids, verify_guarded_calls,
                                       verify_module_import_versions, verify_rust_entrypoints)
from check_rational_kernel import compilation_flags, metadata_fields, require_metadata_field
from check_reciprocal_kernel import module_elf
from check_rust_exports_bridge import version_records
from rust_exports_test_support import read_exports

ROOT = Path(__file__).resolve().parents[2]
SUITE, CASE = b"list_sort", b"list_sort_test"
MAXIMUM = 1025
LENGTHS = tuple(n for n in range(MAXIMUM + 1) if n <= 260 or n == 642 or
                not n & (n - 1) or not (n - 1) & (n - 2) or not (n + 1) & n)
COUNT = len(LENGTHS) * 8 * 3 + 3
DESCRIPTION = b"Non-GPL list_sort public ABI check"
PRIVATE_C_FLAGS = "CFLAGS_MODULE+=-D__DISABLE_EXPORTS"


def reference_source():
    """Compile the unchanged original algorithm under a private symbol only."""
    return ('#include <linux/list_sort.h>\n#include <linux/export.h>\n'
            'void list_sort_reference(void *, struct list_head *, list_cmp_func_t);\n'
            '#undef EXPORT_SYMBOL\n#define EXPORT_SYMBOL(symbol)\n'
            '#define list_sort list_sort_reference\n'
            '#include "' + str(ROOT / 'lib/list_sort.c') + '"\n')


WORKLOAD = r'''
#include <linux/errno.h>
#include <linux/kernel.h>
#include <linux/list_sort.h>
#include <linux/printk.h>
#include <linux/stddef.h>

#define MAXIMUM 1025
#define TRACES (MAXIMUM * 16)
#define POISON1 0xdeadbeefU
#define POISON2 0xa324354cU
typedef void (*sort_fn)(void *, struct list_head *, list_cmp_func_t);
void list_sort_reference(void *, struct list_head *, list_cmp_func_t);
int list_sort_compare(void *, const struct list_head *, const struct list_head *);
int list_sort_exercise(sort_fn, list_cmp_func_t);

struct node { u32 poison1; struct list_head link; u32 poison2; int key, id; };
struct trace { int a, b, an, ap, bn, bp, result; };
struct context {
    struct node nodes[MAXIMUM];
    struct list_head head;
    struct trace trace[TRACES];
    unsigned int calls, style, size;
    int error;
};
static struct context original, candidate;
static struct context *active;

static int index_of(struct context *c, const struct list_head *p)
{
    unsigned long address = (unsigned long)p;
    unsigned long base = (unsigned long)&c->nodes[0].link;
    unsigned long distance;
    if (!p) return -1;
    if (p == &c->head) return -2;
    if (address < base) { c->error = 1; return -3; }
    distance = address - base;
    if (distance % sizeof(struct node) || distance / sizeof(struct node) >= c->size) {
        c->error = 1; return -3;
    }
    return distance / sizeof(struct node);
}

int list_sort_compare(void *opaque, const struct list_head *a, const struct list_head *b)
{
    struct context *c = active;
    const struct node *na, *nb;
    struct trace *t;
    int ai, bi, result;
    if (opaque != c) { c->error = 2; return 0; }
    ai = index_of(c, a); bi = index_of(c, b);
    if (ai < 0 || bi < 0 || ai >= bi || c->calls == TRACES) { c->error = 3; return 0; }
    na = &c->nodes[ai]; nb = &c->nodes[bi];
    if (na->id != ai || nb->id != bi || na->poison1 != POISON1 || nb->poison1 != POISON1 ||
        na->poison2 != POISON2 || nb->poison2 != POISON2) { c->error = 4; return 0; }
    if (c->style == 1) result = na->key > nb->key;
    else if (c->style == 2) result = (na->key < nb->key) - (na->key > nb->key);
    else result = (na->key > nb->key) - (na->key < nb->key);
    t = &c->trace[c->calls++];
    *t = (struct trace){ai, bi, index_of(c, a->next), index_of(c, a->prev),
                       index_of(c, b->next), index_of(c, b->prev), result};
    return result;
}

static void initialize(struct context *c, int n, int pattern, unsigned int style)
{
    unsigned int random = 0x194537abU + (unsigned int)n;
    int i;
    c->calls = 0; c->error = 0; c->style = style; c->size = n;
    c->head.next = n ? &c->nodes[0].link : &c->head;
    c->head.prev = n ? &c->nodes[n-1].link : &c->head;
    for (i = 0; i < n; ++i) {
        struct node *p = &c->nodes[i];
        random = random * 1664525U + 1013904223U;
        p->id = i; p->poison1 = POISON1; p->poison2 = POISON2;
        switch (pattern) {
        case 0: p->key = i; break;
        case 1: p->key = n-i; break;
        case 2: p->key = 0; break;
        case 3: p->key = i%3; break;
        case 4: p->key = (int)(random%17); break;
        case 5: p->key = (int)(random & 0x7fffffffU); break;
        case 6: p->key = i < n/2 ? i : n-i; break;
        default: p->key = i%2 ? n-i : i; break;
        }
        p->link.prev = i ? &c->nodes[i-1].link : &c->head;
        p->link.next = i+1 < n ? &c->nodes[i+1].link : &c->head;
    }
}

static int check(int n)
{
    struct list_head *a = original.head.next, *b = candidate.head.next;
    struct list_head *previous = &candidate.head;
    unsigned int k;
    int i;
    if (original.error || candidate.error || original.calls != candidate.calls) return 1;
    for (k = 0; k < original.calls; ++k) {
        const struct trace *x = &original.trace[k], *y = &candidate.trace[k];
        if (x->a != y->a || x->b != y->b || x->an != y->an || x->ap != y->ap ||
            x->bn != y->bn || x->bp != y->bp || x->result != y->result) return 2;
    }
    for (i = 0; i < n; ++i) {
        int ai = index_of(&original, a), bi = index_of(&candidate, b);
        struct node *nb;
        if (ai < 0 || bi < 0 || ai != bi) return 3;
        nb = &candidate.nodes[bi];
        if (nb->id != bi || nb->poison1 != POISON1 || nb->poison2 != POISON2 ||
            b->prev != previous) return 4;
        if (i) {
            int pi = index_of(&candidate, previous);
            struct node *p;
            if (pi < 0) return 5;
            p = &candidate.nodes[pi];
            if ((candidate.style == 2 ? p->key < nb->key : p->key > nb->key) ||
                (p->key == nb->key && p->id >= nb->id)) return 6;
        }
        previous = b; a = a->next; b = b->next;
    }
    if (a != &original.head || b != &candidate.head || candidate.head.prev != previous) return 7;
    for (i = 0; i < n; ++i) {
        b = b->prev;
        if (index_of(&candidate, b) < 0) return 8;
    }
    if (b != candidate.head.next || original.error || candidate.error) return 9;
    return 0;
}

static int compare_no_priv(void *opaque, const struct list_head *a, const struct list_head *b)
{
    int ai = index_of(active, a), bi = index_of(active, b);
    if (opaque || ai < 0 || bi < 0) { active->error = 10; return 0; }
    return active->nodes[ai].key > active->nodes[bi].key;
}

int list_sort_exercise(sort_fn selected, list_cmp_func_t cmp)
{
    int n, pattern, stage, cases = 0;
    unsigned int style;
    for (n = 0; n <= MAXIMUM; ++n) {
        if (n > 260 && n != 642 && (n & (n-1)) && ((n-1) & (n-2)) && ((n+1) & n)) continue;
        for (pattern = 0; pattern < 8; ++pattern) for (style = 0; style < 3; ++style) {
            initialize(&original, n, pattern, style); initialize(&candidate, n, pattern, style);
            active = &original; list_sort_reference(active, &original.head, list_sort_compare);
            active = &candidate; selected(active, &candidate.head, cmp);
            stage = check(n);
            if (stage) goto fail;
            ++cases;
        }
    }
    for (n = 0; n <= 2; ++n) {
        initialize(&original, n, 1, 1); initialize(&candidate, n, 1, 1);
        active = &original; list_sort_reference(NULL, &original.head, compare_no_priv);
        active = &candidate; selected(NULL, &candidate.head, compare_no_priv);
        stage = check(n);
        if (stage) goto fail;
        ++cases;
    }
    active = NULL;
    return cases;
fail:
    pr_err("LUPOS_LIST_SORT_FAIL stage=%d length=%d original=%d candidate=%d\n",
           stage, n, original.error, candidate.error);
    active = NULL;
    return -EINVAL;
}
'''

C_SOURCE = r'''
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/list_sort.h>
#include <linux/module.h>
#include <linux/printk.h>
typedef void (*sort_fn)(void *, struct list_head *, list_cmp_func_t);
int list_sort_exercise(sort_fn, list_cmp_func_t);
int list_sort_compare(void *, const struct list_head *, const struct list_head *);
static noinline void list_sort_call(void *ctx, struct list_head *head, list_cmp_func_t cmp)
{
    sort_fn volatile selected = list_sort;
    selected(ctx, head, cmp);
}
static int __init list_sort_abi_init(void)
{
    int result = list_sort_exercise(list_sort_call, list_sort_compare);
    if (result != @COUNT@) return -EINVAL;
    pr_info("LUPOS_LIST_SORT_ABI_OK cases=@COUNT@ callbacks=exact links=bidirectional stable=yes\n");
    return 0;
}
static void __exit list_sort_abi_exit(void) {}
module_init(list_sort_abi_init);
module_exit(list_sort_abi_exit);
MODULE_LICENSE("Proprietary");
MODULE_DESCRIPTION("Non-GPL list_sort public ABI check");
'''

RUST_SOURCE = r'''// SPDX-License-Identifier: GPL-2.0-only
//! Disposable non-GPL native caller, using the original nonnullable callback ABI.
use core::ffi::c_void;
use kernel::{bindings, ffi};
#[path = "@HEADER@"]
pub mod declarations;
use declarations::{list_cmp_func_t, list_head};
type Sort = unsafe extern "C" fn(*mut c_void, *mut list_head, list_cmp_func_t);
unsafe extern "C" {
    fn list_sort_exercise(sort: Sort, cmp: list_cmp_func_t) -> ffi::c_int;
    fn list_sort_compare(ctx: *mut c_void, a: *const list_head, b: *const list_head) -> ffi::c_int;
}
/// Forward the native comparator arguments unchanged into the checked workload.
///
/// # Safety
/// The private workload owns the live nodes and context for this callback.
#[inline(never)]
pub unsafe extern "C" fn comparator(ctx: *mut c_void, a: *const list_head, b: *const list_head) -> ffi::c_int {
    // SAFETY: The caller maintains the same original comparator preconditions.
    unsafe { list_sort_compare(ctx, a, b) }
}
/// Genuinely indirect invocation of the actual native list_sort export.
///
/// # Safety
/// Head and comparator satisfy the original list_sort contract.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn list_sort_call(ctx: *mut c_void, head: *mut list_head, cmp: list_cmp_func_t) {
    let pointer: Sort = declarations::list_sort;
    // SAFETY: Volatile loading preserves the original typed, nonnull function.
    unsafe { core::ptr::read_volatile(&pointer)(ctx, head, cmp) }
}
/// Execute the stateless public ABI workload at insertion.
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn init_module() -> ffi::c_int {
    // SAFETY: The C workload owns every actual-header list node and lifetime.
    let result = unsafe { list_sort_exercise(list_sort_call, comparator) };
    if result != @COUNT@ { return -22; }
    // SAFETY: Static format with no variadic arguments.
    unsafe { bindings::_printk(c"\x016LUPOS_LIST_SORT_RUST_ABI_OK cases=@COUNT@ callbacks=exact links=bidirectional stable=yes\n".as_ptr().cast()); }
    0
}
/// The fixture retains no live external resources.
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section = ".init.data"]
static ADDRESSABLE_INIT_MODULE: extern "C" fn() -> ffi::c_int = init_module;
#[used]
#[link_section = ".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE: extern "C" fn() = cleanup_module;
const INFO: &str = "license=Proprietary\0description=Non-GPL list_sort public ABI check\0";
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
    if caller not in ("c", "rust"):
        raise ValueError("unknown list_sort caller")
    return (C_SOURCE if caller == "c" else RUST_SOURCE).replace("@COUNT@", str(COUNT)).replace(
        "@HEADER@", str(ROOT / "include/linux/list_sort_header.rs"))


def states(config):
    suite, framework = (config.get(name, "n") for name in ("TEST_LIST_SORT", "KUNIT"))
    if suite not in ("n", "y", "m") or suite != "n" and (
            framework not in ("y", "m") or suite == "y" and framework == "m"):
        raise ValueError("incompatible list_sort suite/framework configuration")
    if any(config.get(name) == "y" for name in ("RUST_LIST_SORT", "RUST_LIST_SORT_KUNIT_TEST")) and config.get("RUST") != "y":
        raise ValueError("Rust list_sort selections require CONFIG_RUST=y")
    return suite, framework


def selected_version(build):
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()
            if len(line.split()) > 1 and line.split()[1] == b"list_sort"]
    if (len(rows) != 1 or len(rows[0]) != 4 or rows[0][1:] != [b"list_sort", b"vmlinux", b"EXPORT_SYMBOL"]
            or not re.fullmatch(rb"0x[0-9a-fA-F]{8}", rows[0][0])):
        raise ValueError("wrong list_sort owner, license, namespace or CRC")
    return rows[0][0].lower()


def verify_dwarf(build, owner, crc):
    """Recompute this defining unit's CRC, not a fabricated declaration type."""
    config = configuration(build)
    if config.get("MODVERSIONS") != "y": return
    if config.get("GENDWARFKSYMS") != "y":
        raise ValueError("list_sort native type proof requires GENDWARFKSYMS=y")
    executable = build / "scripts/gendwarfksyms/gendwarfksyms"
    with tempfile.TemporaryDirectory(prefix="list-sort-dwarf-") as temporary:
        symtypes = Path(temporary) / "types"
        result = subprocess.run([str(executable), "--symtypes", str(symtypes), str(owner)],
                                input=b"list_sort\n", capture_output=True, check=True, timeout=180)
        if result.stderr or result.stdout.split() != [b"#SYMVER", b"list_sort", crc]:
            raise ValueError("list_sort actual defining-object DWARF CRC differs")
        types = symtypes.read_bytes()
        if b"list_head" not in types or b"list_sort" not in types:
            raise ValueError("list_sort defining-object DWARF lacks actual list/callback type")


def verify_suite_metadata(obj, builtin):
    prefix = b"test_list_sort." if builtin else b""
    fields = metadata_fields(obj)
    for name, value in ((b"license", b"GPL"), (b"description", b"list_sort() KUnit test suite")):
        require_metadata_field(fields, prefix + name, value)


def verify_linked_implementation(build, selection):
    config = configuration(build)
    suite, framework = states(config)
    if selection != ("Rust" if config.get("RUST_LIST_SORT") == "y" else "C"):
        raise ValueError("list_sort provider selection differs from configuration")
    arch, archive = architecture(config), build / "vmlinux.a"
    ordered = [(build / os.fsdecode(line)).resolve() for line in tool("ar", "t", archive).splitlines()]
    members = set(ordered)
    owner = build / ("lib/list_sort_rust.o" if selection == "Rust" else "lib/list_sort.o")
    other = build / ("lib/list_sort.o" if selection == "Rust" else "lib/list_sort_rust.o")
    if ordered.count(owner.resolve()) != 1 or other.resolve() in members:
        raise ValueError("list_sort archive owner missing, duplicated or mixed")
    source = ROOT / ("lib/list_sort_rust.rs" if selection == "Rust" else "lib/list_sort.c")
    deps = ([ROOT / p for p in ("lib/list_sort.rs", "include/linux/list_sort_header.rs",
             "rust/ffi_export.rs", "include/linux/export_header.rs")] +
            [build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"] if selection == "Rust" else
            [ROOT / "include/linux/list_sort.h", ROOT / "include/linux/list.h"])
    verify_build_command(build, owner, source, deps)
    elf_target(owner, arch)
    expected = dict(name="list_sort", license="", namespace="", relocation_target="list_sort",
                    relocation_addend=0, pointer_width=8, relocation_kind=257 if arch == "aarch64" else 1,
                    label_binding=0, label_kind=0, section_flags=2, section_alignment=8)
    records = read_exports(owner)
    if len(records) != 1 or any(records[0][key] != value for key, value in expected.items()):
        raise ValueError("wrong list_sort export metadata or relocation")
    definitions = [line.split() for line in tool("nm", "-g", "--defined-only", owner).splitlines()
                   if line.split()[-1:] == [b"list_sort"]]
    if len(definitions) != 1 or definitions[0][-2] != b"T":
        raise ValueError("list_sort is not an actual global function in its owner")
    crc = selected_version(build)
    if config.get("MODVERSIONS") == "y" and version_records(owner.with_name("." + owner.name + ".cmd")) != {b"list_sort": crc}:
        raise ValueError("list_sort version does not originate in the selected defining object")
    verify_dwarf(build, owner, crc)
    if config.get("CFI") == "y":
        required = "-Zsanitizer=kcfi" if selection == "Rust" else "-fsanitize=kcfi"
        if required not in compilation_flags(owner): raise ValueError("selected list_sort owner lacks KCFI")
        provider_type_ids(owner, names=("list_sort",))
    newer(archive, [owner])
    test = build / "lib/tests/test_list_sort.o"
    if ordered.count(test.resolve()) != int(suite == "y"):
        raise ValueError("list_sort KUnit archive membership differs from configuration")
    modules = []
    if suite != "n":
        if framework == "m":
            module = verify_framework_module(build, members, arch)
            verify_module_metadata(build, module)
            verify_module_import_versions(build, module)
            modules.append(module)
        else:
            for name in ("test", "assert"):
                obj = build / ("lib/kunit/" + name + ".o")
                if obj.resolve() not in members: raise ValueError("builtin KUnit framework is not linked")
                verify_build_command(build, obj, ROOT / ("lib/kunit/" + name + ".c"))
                newer(archive, [obj])
        rust_test = config.get("RUST_LIST_SORT_KUNIT_TEST") == "y"
        test_source = ROOT / ("lib/tests/test_list_sort.rs" if rust_test else "lib/tests/test_list_sort.c")
        test_deps = ([ROOT / "include/linux/list_sort_header.rs", build / "rust/libkernel.rmeta",
                      build / "rust/libbindings.rmeta"] if rust_test else
                     [ROOT / "include/kunit/test.h", ROOT / "include/linux/list_sort.h", ROOT / "include/linux/list.h"])
        verify_build_command(build, test, test_source, test_deps)
        elf_target(test, arch)
        verify_references(test, ("list_sort", "__kunit_do_failed_assertion"))
        _, sections, _, _, _ = module_elf(test)
        registration = [s for s in sections if s[0] == b".kunit_test_suites"]
        if len(registration) != 1 or registration[0][1] != 1 or registration[0][4] != 8:
            raise ValueError("list_sort test must register exactly one real KUnit suite")
        verify_suite_metadata(test, suite == "y")
        if rust_test:
            kernel = build / "rust/kernel.o"
            if kernel.resolve() not in members: raise ValueError("Rust KUnit API is not linked")
            verify_build_command(build, kernel, ROOT / "rust/kernel/lib.rs", [ROOT / "rust/kernel/kunit.rs"])
            newer(build / "rust/libkernel.rmeta", [ROOT / "rust/kernel/kunit.rs"])
            newer(build / "rust/libbindings.rmeta", [ROOT / "include/kunit/test.h", ROOT / "include/linux/list.h"])
        if suite == "y": newer(archive, [test])
        else:
            module = test.with_suffix(".ko")
            verify_module(build, module, test, "test_list_sort", arch)
            verify_module_metadata(build, module)
            verify_module_import_versions(build, module)
            verify_references(module, ("list_sort", "__kunit_do_failed_assertion"))
            verify_suite_metadata(module, False)
            modules.append(module)
    newer(build / "Module.symvers", [owner, *([test] if suite != "n" else [])])
    newer(build / "vmlinux.o", [archive])
    newer(build / "vmlinux", [build / "vmlinux.o"])
    newer(build / ("arch/arm64/boot/Image" if arch == "aarch64" else "arch/x86/boot/bzImage"), [build / "vmlinux"])
    return modules


def verify_consumer(build, work, caller):
    name = "list_sort_rust_abi" if caller == "rust" else "list_sort_abi"
    obj = work / ("list_sort_rust_main.o" if caller == "rust" else "list_sort_c_main.o")
    module = work / (name + ".ko")
    config, arch = configuration(build), architecture(configuration(build))
    for path in (obj, module):
        elf_target(path, arch)
        verify_references(path, ("list_sort",))
    if module_name(module) != name: raise ValueError("wrong list_sort consumer module identity")
    deps = ([ROOT / "include/linux/list_sort_header.rs", build / "rust/libkernel.rmeta", build / "rust/libbindings.rmeta"]
            if caller == "rust" else [ROOT / "include/linux/list_sort.h"])
    verify_build_command(work, obj, obj.with_suffix(".rs" if caller == "rust" else ".c"), deps)
    if obj.with_suffix(".rs" if caller == "rust" else ".c").read_text() != sources(caller):
        raise ValueError("private list_sort caller differs from checked source")
    inputs = []
    for stem, expected, dependencies in (("list_sort_reference", reference_source(), [ROOT / "lib/list_sort.c"]),
                                        ("list_sort_workload", WORKLOAD, [ROOT / "include/linux/list_sort.h"])):
        path = work / (stem + ".o")
        if path.with_suffix(".c").read_text() != expected: raise ValueError("private list_sort workload/oracle source differs")
        verify_build_command(work, path, path.with_suffix(".c"), dependencies)
        inputs.append(path)
    newer(module, [obj, *inputs, selected_metadata(build, module)])
    verify_module_metadata(build, module, work=work, require_c_suppression=True)
    verify_module_import_versions(build, module)
    require_metadata_field(metadata_fields(module), b"license", b"Proprietary")
    require_metadata_field(metadata_fields(module), b"description", DESCRIPTION)
    if read_exports(module): raise ValueError("private list_sort caller unexpectedly exports symbols")
    undefined = {line.split()[-1] for line in tool("nm", "-u", module).splitlines() if line.split()}
    rows = [line.split() for line in (build / "Module.symvers").read_bytes().splitlines()]
    if any(len(row) >= 4 and row[1] in undefined and row[3] != b"EXPORT_SYMBOL" for row in rows):
        raise ValueError("Proprietary list_sort caller imports a GPL-only symbol")
    if caller == "rust":
        imports = {line.split()[-1] for line in tool("nm", "-u", obj).splitlines() if line.split()}
        if imports != {b"_printk", b"list_sort", b"list_sort_compare", b"list_sort_exercise"}:
            raise ValueError("Rust list_sort caller has unexpected runtime/core imports")
        for path in (obj, module): verify_rust_entrypoints(path, arch)
    for path in [*inputs, *([obj] if caller == "c" else [])]:
        flags = compilation_flags(path)
        if "-D__DISABLE_EXPORTS" not in flags or "-U__DISABLE_EXPORTS" in flags or read_exports(path):
            raise ValueError("private list_sort C fixture contains incidental export metadata")
    verify_common_metadata(build, work, flags=compilation_flags, exports=read_exports)
    if config.get("CFI") == "y":
        flags = compilation_flags(obj)
        required = "-Zsanitizer=kcfi" if caller == "rust" else "-fsanitize=kcfi"
        if required not in flags or any(flag.startswith("-fno-sanitize=") and
                {"all", "kcfi"} & set(flag.split("=", 1)[1].split(",")) for flag in flags):
            raise ValueError("list_sort caller lacks actual KCFI compilation")
        owner = build / ("lib/list_sort_rust.o" if config.get("RUST_LIST_SORT") == "y" else "lib/list_sort.o")
        types = provider_type_ids(owner, names=("list_sort",))
        for path in (obj, module): verify_guarded_calls(path, arch, types, wrappers=(("list_sort_call", "list_sort"),))


def console_lines(console):
    return [re.sub(rb"^\[\s*\d+\.\d+\]\s*", b"", line.strip()).strip()
            for line in normalize_console_transport(console).splitlines()]


def kunit_runs(console, runs):
    lines = console_lines(console)
    starts = [i for i, line in enumerate(lines) if line == b"# Subtest: " + SUITE]
    ends = [i for i, line in enumerate(lines) if re.fullmatch(rb"(?:not )?ok\s+\d+\s+list_sort(?:\s.*)?", line)]
    if runs not in (0, 1, 2) or len(starts) != runs or len(ends) != runs:
        raise ValueError("missing or duplicate list_sort suites")
    intervals = []
    for start, end in zip(starts, ends):
        if start >= end or intervals and start <= intervals[-1][1]: raise ValueError("overlapping list_sort suites")
        plan, result, summaries = False, False, set()
        for i in range(start + 1, end + 1):
            line = lines[i]
            if re.match(rb"\d+\.\.", line):
                if line != b"1..1" or plan or result: raise ValueError("wrong or misplaced list_sort plan")
                plan = True
            elif line.startswith(b"# Subtest:"): raise ValueError("unexpected list_sort parameter suite")
            elif re.match(rb"(?:not )?ok\s", line):
                if line == b"ok 1 " + CASE and plan and not result: result = True
                elif i == end and re.fullmatch(rb"ok [1-9]\d* list_sort", line) and result: pass
                else: raise ValueError("wrong, duplicate, skipped or premature list_sort result")
            elif re.match(rb"# (?:list_sort_test|list_sort|Totals):", line):
                match = re.fullmatch(rb"# (list_sort_test|list_sort|Totals): pass:1 fail:0 skip:0 total:1", line)
                if not match or match[1] in summaries: raise ValueError("contradictory list_sort totals")
                if (match[1] == CASE and result) or (match[1] != CASE and not result):
                    raise ValueError("misplaced list_sort totals")
                summaries.add(match[1])
            elif line.startswith(b"# module:") and line != b"# module: test_list_sort":
                raise ValueError("wrong list_sort KUnit module identity")
        if not plan or not result: raise ValueError("incomplete list_sort suite")
        intervals.append((start, end))
    for i, line in enumerate(lines):
        if re.fullmatch(rb"(?:not )?ok\s+\d+\s+list_sort_test(?:\s.*)?", line) and not any(a < i < b for a, b in intervals):
            raise ValueError("list_sort case outside its suite")
    return intervals


def result_marker(caller):
    if caller not in ("c", "rust"): raise ValueError("unknown list_sort caller")
    return (b"LUPOS_LIST_SORT_" + (b"RUST_ABI" if caller == "rust" else b"ABI") +
            f"_OK cases={COUNT} callbacks=exact links=bidirectional stable=yes".encode())


def verify_console(console, caller, *, suite="y", framework_module=False, reload=False, config=None):
    if suite not in ("n", "y", "m") or framework_module and suite != "m":
        raise ValueError("incompatible list_sort runtime states")
    if re.search(rb"CFI failure|BUG:|Oops:|Kernel panic|UBSAN:|KASAN:|general protection fault|Unable to handle|Internal error:|(?:EXPECTATION|ASSERTION) FAILED|\bnot ok\s|no (?:extended )?symbol version", console, re.I):
        raise ValueError("unexpected list_sort kernel fault or KUnit failure")
    lines = console_lines(console)
    allowed = set()
    if config and config.get("INT_LOG_KUNIT_TEST") == "y":
        log_ranges = log_kunit_runs(console, 1)
        allowed = set(log_kunit_warnings(lines, log_ranges,
                      "Rust" if config.get("RUST_INT_LOG") == "y" else "C", config.get("BUG") == "y"))
    if {i for i, line in enumerate(lines) if b"WARNING:" in line} != allowed:
        raise ValueError("warning outside configured original integer-log zero cases")
    ranges = kunit_runs(console, 0 if suite == "n" else 2 if suite == "m" and reload else 1)
    preloads = int(framework_module) + int(suite == "m")
    marker = result_marker(caller)
    expected = [b"KUNIT"] if suite == "y" else []
    for index in range(preloads):
        if index == preloads - 1: expected.append(b"KUNIT")
        expected.append(f"LUPOS_RUST_PRELOAD_OK {index}".encode())
    expected += [marker, b"LUPOS_RUST_MODULE_LOAD_OK"]
    if reload:
        expected += [f"LUPOS_RUST_MODULE_UNLOAD_OK {i}".encode() for i in reversed(range(preloads + 1))]
        for index in range(preloads):
            if index == preloads - 1: expected.append(b"KUNIT")
            expected.append(f"LUPOS_RUST_MODULE_RELOAD_OK {index}".encode())
        expected += [marker, f"LUPOS_RUST_MODULE_RELOAD_OK {preloads}".encode()]
    expected.append(b"LUPOS_RUST_BUILD_BOOT_OK")
    actual = []
    for i, line in enumerate(lines):
        if any(i == end for _, end in ranges): actual.append(b"KUNIT")
        if b"LUPOS_" in line:
            if any(start <= i <= end for start, end in ranges): raise ValueError("loader event inside unfinished list_sort suite")
            actual.append(re.sub(rb"^list_sort_(?:rust_)?abi: ", b"", line))
    if actual != expected: raise ValueError("list_sort caller/suite events missing, duplicated or reordered")
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
        suite, framework = states(config)
        selection = "Rust" if config.get("RUST_LIST_SORT") == "y" else "C"
        if selection == "C" and not args.allow_c_baseline: raise ValueError("requires RUST_LIST_SORT=y or --allow-c-baseline")
        for option in ("MODULES", "PRINTK", "MULTIUSER"):
            if config.get(option) != "y": raise ValueError("requires CONFIG_" + option + "=y")
        if args.caller == "rust" and config.get("RUST") != "y": raise ValueError("Rust caller requires RUST=y")
        if args.reload_modules and config.get("MODULE_UNLOAD") != "y": raise ValueError("reload requires MODULE_UNLOAD=y")
        if config.get("MODULE_SIG_FORCE") == "y": raise ValueError("disposable modules require signatures not forced")
        arch = architecture(config)
        preloads = verify_linked_implementation(build, selection)
    except (OSError, ValueError, IndexError, KeyError, struct.error, subprocess.CalledProcessError) as error:
        parser.error(str(error))
    work = build / "rust-list-sort-test"
    work.mkdir(exist_ok=True)
    name, source = ("list_sort_rust_abi", "list_sort_rust_main") if args.caller == "rust" else ("list_sort_abi", "list_sort_c_main")
    (work / "list_sort_reference.c").write_text(reference_source())
    (work / "list_sort_workload.c").write_text(WORKLOAD)
    (work / (source + (".rs" if args.caller == "rust" else ".c"))).write_text(sources(args.caller))
    (work / "Makefile").write_text(f"obj-m := {name}.o\n{name}-y := {source}.o list_sort_workload.o list_sort_reference.o\n")
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
                           suite=suite, framework_module=suite != "n" and framework == "m", reload=args.reload_modules,
                           config=config)
    print(f"{selection} list_sort: Proprietary {args.caller} caller passed {COUNT} cases/load with exact comparator traces, "
          f"stable ordering and bidirectional links; {count} selected KUnit runs on {arch}.")


if __name__ == "__main__":
    main()
