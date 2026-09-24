#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Private, read-only native flag replay. All outputs stay in this workspace."""
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import shlex
import subprocess
import sys

WORK = ROOT = FIXTURES = OUT = RUST = None
BUILDS = {}
env = {}
report = {"runs": [], "failures": []}
SYMBOLS = ["guid_null", "uuid_null", "guid_index", "uuid_index",
           "generate_random_uuid", "generate_random_guid", "guid_gen", "uuid_gen",
           "uuid_is_valid", "guid_parse", "uuid_parse"]

def command(args, name, cwd=None, input_text=None, expected=0):
    args = list(map(str, args))
    path = OUT / (name + ".log")
    if path.exists() and not path.read_text().endswith("exit=0\n"):
        index = 1
        while (OUT / (name + f".failure-{index}.log")).exists():
            index += 1
        path.rename(OUT / (name + f".failure-{index}.log"))
    with (OUT / (name + ".log")).open("w") as log:
        log.write("$ " + shlex.join(args) + "\n")
        log.flush()
        p = subprocess.run(args, cwd=cwd or OUT, env=env, input=input_text,
                           text=True, capture_output=True, timeout=55,
                           preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_CORE, (0, 0)))
        log.write(p.stdout + p.stderr + f"\nexit={p.returncode}\n")
    if p.returncode != expected:
        raise RuntimeError(f"{name}: exit={p.returncode}; see {OUT / (name+'.log')}")
    return p.stdout

def flags(build, rust):
    path = "lib/.list_sort_rust.o.cmd" if rust and build == BUILDS.get("x86") else (
        "lib/math/.cordic_rust.o.cmd" if rust else "lib/.scatterlist.o.cmd")
    saved = (build/path).read_text()
    lexer = shlex.shlex(saved.splitlines()[0].split(" := ", 1)[1], posix=True, punctuation_chars=';')
    lexer.whitespace_split = True
    args = []
    for token in lexer:
        if token and set(token) == {';'}:
            break
        args.append(token)
    while re.fullmatch(r'[A-Za-z_][A-Za-z_0-9]*=.*', args[0]):
        args.pop(0)
    args.pop(0)
    if not rust:
        # UUID's C object disappears in a clean Rust-selected build. The
        # always-C sibling supplies identical directory/target compiler flags,
        # but never borrow its object identity or unreviewed per-file settings.
        source = re.search(r'^source_lib/scatterlist\.o := (.+)$', saved, re.M)
        if not source or Path(source[1]).resolve() != (ROOT/'lib/scatterlist.c').resolve():
            raise ValueError('scatterlist donor source identity does not match original C')
        if args.count('-c') != 1 or args.count('-o') != 1:
            raise ValueError('scatterlist donor must be one C compile')
        at = args.index('-c')
        if args[at:] != ['-c', '-o', 'lib/scatterlist.o', source[1]]:
            raise ValueError('scatterlist donor compile source/output identity mismatch')
        makefile = (ROOT/'lib/Makefile').read_text()
        if re.search(r'(?m)^\s*(?:(?:CFLAGS(?:_REMOVE)?|GCOV_PROFILE|KCOV_INSTRUMENT|KASAN_SANITIZE|UBSAN_SANITIZE)_'
                     r'(?:uuid|scatterlist)\.o\b|[^\n]*\b(?:uuid|scatterlist)\.o\s*:)', makefile):
            raise ValueError('UUID/scatterlist per-file compiler settings need explicit audit')
        flags = args[:at]
        identities = {'-DKBUILD_MODFILE="lib/scatterlist"': '-DKBUILD_MODFILE="lib/uuid"',
                      '-DKBUILD_BASENAME="scatterlist"': '-DKBUILD_BASENAME="uuid"',
                      '-DKBUILD_MODNAME="scatterlist"': '-DKBUILD_MODNAME="uuid"',
                      '-D__KBUILD_MODNAME=scatterlist': '-D__KBUILD_MODNAME=uuid'}
        for identity in identities:
            if flags.count(identity) != 1:
                raise ValueError('scatterlist donor source-specific macro mismatch: ' + identity)
        return [identities.get(a, a) for a in flags
                if not a.startswith('-Wp,-MMD,') and a not in ('-Os', '-O2', '-O0')]
    args.pop()
    result = []
    skip = False
    for arg in args:
        if skip:
            skip = False
            continue
        if arg == "--out-dir":
            skip = True
            continue
        if arg.startswith(("--emit=", "-Copt-level=")):
            continue
        if arg.startswith("--target=./"):
            arg = "--target=" + str(build/arg.split("=./")[1])
        elif arg.startswith("@./"):
            arg = "@" + str(build/arg[3:])
        elif arg == "./rust/":
            arg = str(build/"rust")
        result.append(arg)
    return result + ["-Dwarnings", "-Dunsafe_op_in_unsafe_fn"]

def ids(path):
    text = path.read_text()
    values = {n: int(v) & 0xffffffff for n, v in re.findall(r"^!(\d+) = !\{i32 (-?\d+)\}$", text, re.M)}
    return {name: values[n] for name, n in re.findall(r"^define [^\n]*?@([\w]+)\([^\n]*!kcfi_type !(\d+)", text, re.M)}

def main():
    sys.path.insert(0, str(ROOT/"scripts/tests"))
    from rust_exports_test_support import read_exports
    report["rustc"] = command([RUST, "-vV"], "rustc")
    for arch, build in BUILDS.items():
        for required in ["rust/libkernel.rmeta", "rust/libbindings.rmeta", "lib/.scatterlist.o.cmd"]:
            if not (build/required).is_file():
                raise ValueError(f"Explicit native input {build} missing {required}")
        rf, cf = flags(build, True), flags(build, False)
        assert "-Zsanitizer=kcfi" in rf and "-fsanitize=kcfi" in cf
        env.update(OBJTREE=str(build), RUST_MODFILE="lib/uuid_rust")
        report[arch+"_bindings_sha256"] = hashlib.sha256((build/"rust/bindings/bindings_generated.rs").read_bytes()).hexdigest()
        for opt in ["0", "2", "s"]:
            prefix = arch+"-O"+opt
            d = OUT/prefix
            d.mkdir(exist_ok=True)
            command([RUST, *rf, "-Copt-level="+opt, ROOT/"lib/uuid_rust.rs",
                     "--emit=obj="+str(d/"rust.o")+",llvm-ir="+str(d/"rust.ll")+",dep-info="+str(d/"rust.d")], prefix+"-rust")
            command([RUST, *rf, "-Copt-level="+opt, FIXTURES/"header.rs",
                     "--emit=obj="+str(d/"header.o")+",dep-info="+str(d/"header.d")], prefix+"-header")
            renamed = ["-D"+s+"=c_"+s for s in SYMBOLS]
            command(["clang", *cf, "-O"+opt, *renamed, "-c", ROOT/"lib/uuid.c", "-o", d/"c.o"], prefix+"-c", build)
            command(["clang", *cf, "-O"+opt, *renamed, "-S", "-emit-llvm", ROOT/"lib/uuid.c", "-o", d/"c.ll"], prefix+"-c-ir", build)
            ci, ri = ids(d/"c.ll"), ids(d/"rust.ll")
            for name in SYMBOLS[4:]:
                assert ci["c_"+name] == ri[name], (arch, opt, name, ci, ri)
            records = read_exports(d/"rust.o")
            assert {r["name"] for r in records} == set(SYMBOLS)-{"guid_index", "uuid_index"}
            for r in records:
                assert r["license"] == ("GPL" if r["name"] in ["guid_gen", "uuid_gen"] else "")
                assert r["namespace"] == "" and r["relocation_target"] == r["name"]
                assert r["pointer_width"] == 8 and r["relocation_addend"] == 0
                assert r["relocation_kind"] == (257 if arch == "arm64" else 1)
                assert r["section_alignment"] == 8 and r["section_flags"] == 2
                assert r["label_binding"] == 0 and r["label_kind"] == 0
            command(["llvm-dwarfdump", "--debug-info", d/"rust.o"], prefix+"-dwarf")
            symbols = command(["readelf", "-Ws", d/"rust.o"], prefix+"-symbols")
            for symbol in SYMBOLS[:4]:
                assert re.search(r"\b16 OBJECT\s+GLOBAL\s+DEFAULT\s+\d+ "+symbol+r"$", symbols, re.M), symbol
            undefined = command(["nm", "-u", d/"rust.o"], prefix+"-undefined")
            assert "get_random_bytes" in undefined
            assert not any(name in undefined for name in ("isxdigit", "hex_to_bin", "alloc", "kmalloc"))
            command(["readelf", "-rW", "-x", ".export_symbol", d/"rust.o"], prefix+"-exports")
            command([build/"scripts/gendwarfksyms/gendwarfksyms", "--dump-versions", "-T", d/"uuid.symtypes", d/"rust.o"],
                    prefix+"-gendwarfksyms", input_text="\n".join(r["name"] for r in records)+"\n")
            symtypes = (d/"uuid.symtypes").read_text()
            for kind in ("guid", "uuid"):
                assert kind+"_null variable s#bindings::bindings_raw::"+kind+"_t" in symtypes
                assert "s#bindings::bindings_raw::"+kind+"_t structure_type" in symtypes
            assert symtypes.count("byte_size(16) alignment(1)") == 2
            if arch == "x86":
                command([RUST, *rf, "-Copt-level="+opt, FIXTURES/"panic.rs",
                         "--emit=obj="+str(d/"panic.o")], prefix+"-panic")
                driver_flags = [a for a in cf if a not in ["-mskip-rax-setup", "-mstack-alignment=8"]]
                for source, obj in [(ROOT/"lib/ctype.c", "ctype"), (FIXTURES/"hex.c", "hex"),
                                    (FIXTURES/"differential.c", "driver")]:
                    command(["clang", *driver_flags, "-O"+opt, "-c", source, "-o", d/(obj+".o")], prefix+"-"+obj, build)
                command(["clang", "-no-pie", "-Wl,-T,"+str(FIXTURES/"discard.lds"),
                         d/"rust.o", d/"header.o", d/"c.o", d/"ctype.o", d/"hex.o", d/"driver.o", d/"panic.o",
                         build/"rust/core.o", build/"rust/compiler_builtins.o", "-o", d/"differential"], prefix+"-link")
                command([d/"differential"], prefix+"-run")
                command(["clang", *driver_flags, "-O"+opt, "-c", FIXTURES/"kcfi.c", "-o", d/"kcfi.o"], prefix+"-kcfi-caller", build)
                command(["clang", "-no-pie", "-Wl,-T,"+str(FIXTURES/"discard.lds"),
                         d/"rust.o", d/"kcfi.o", d/"panic.o", build/"rust/core.o",
                         build/"rust/compiler_builtins.o", "-o", d/"kcfi"], prefix+"-kcfi-link")
                command([d/"kcfi"], prefix+"-kcfi-good")
                command([d/"kcfi", "bad"], prefix+"-kcfi-bad", expected=-4)
            report["runs"].append({"arch":arch, "opt":opt, "kcfi":{n:ri[n] for n in SYMBOLS[4:]},
                                   "runtime":arch=="x86", "exports":records})
            print(prefix+" passed", flush=True)
