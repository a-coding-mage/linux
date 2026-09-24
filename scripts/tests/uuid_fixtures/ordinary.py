#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Secondary genuine-width host ABI tests; explicit bad inputs are failures."""
import json
import shlex

def main(n, bits, sysroot):
    make = (n.ROOT/"Makefile").read_text()
    common = make.split("export rust_common_flags := ",1)[1].split("\n\n",1)[0].replace("\\\n", " ")
    flags = shlex.split(common)+["-Aclippy::precedence", "-Cstrip=debuginfo", "-Zallow-features=", "-Dwarnings",
        "-Dunsafe_op_in_unsafe_fn", "-Cpanic=abort", "-Cdebug-assertions=n", "-Coverflow-checks=y"]
    results = []
    failed = False
    for bits in [bits]:
        target = [] if bits == 64 else ["--target=i686-unknown-linux-gnu", "--sysroot="+str(sysroot)]
        for opt in ["0", "2", "s"]:
            name = f"host{bits}-O{opt}"
            d = n.OUT/name
            d.mkdir(exist_ok=True)
            try:
                rf = [n.RUST, *flags, *target, "-Copt-level="+opt, "-Ldependency="+str(d)]
                n.command([*rf, "--crate-type=rlib", "--crate-name=ffi", n.ROOT/"rust/ffi.rs",
                           "-o", d/"libffi.rlib"], name+"-ffi")
                n.command([*rf, "--crate-type=rlib", "--crate-name=kernel", n.FIXTURES/"kernel_fixture.rs",
                           "--extern=ffi="+str(d/"libffi.rlib"), "-o", d/"libkernel.rlib"], name+"-kernel")
                n.command([*rf, "--crate-type=staticlib", "--crate-name=uuid_host", n.FIXTURES/"host.rs",
                           "--extern=kernel="+str(d/"libkernel.rlib"),
                           "--emit=link="+str(d/"rust.a")+",dep-info="+str(d/"rust.d")], name+"-rust")
                cf = ["clang", "-m"+str(bits), "-O"+opt, "-funsigned-char", "-fno-pie",
                      "-ffreestanding", "-fno-builtin", "-fno-stack-protector", "-Wall", "-Wextra", "-Werror"]
                for src, obj in [("ordinary_oracle.c", "c"), ("ordinary_ctype.c","ctype"),
                                 ("ordinary_hex.c", "hex"), ("differential.c","driver")]:
                    extra = ["-D"+s+"=c_"+s for s in n.SYMBOLS] if obj == "c" else ["-DORDINARY"]
                    n.command([*cf, *extra, "-c", n.FIXTURES/src, "-o", d/(obj+".o")], name+"-"+obj)
                extra = ["-no-pie"] if bits == 64 else ["-nostdlib", "-static", "-Wl,-e,_start", n.FIXTURES/"freestanding32.c"]
                n.command([*cf, *extra, "-Wl,--gc-sections", d/"c.o", d/"ctype.o", d/"hex.o", d/"driver.o", d/"rust.a", "-o", d/"differential"], name+"-link")
                assert (d/"differential").read_bytes()[4] == (1 if bits == 32 else 2)
                output = n.command([d/"differential"], name+"-run")
                assert "UUID_DIFFERENTIAL_OK" in output
                results.append({"bits":bits, "opt":opt, "runtime":True, "output":output})
                print(name+" passed", flush=True)
            except Exception as error:
                failed = True
                results.append({"bits":bits,"opt":opt,"runtime":False,"error":str(error)})
                print(name+" FAILED: "+str(error), flush=True)
    (n.OUT/"ordinary-report.json").write_text(json.dumps(results,indent=2)+"\n")
    if failed:
        raise SystemExit(1)
