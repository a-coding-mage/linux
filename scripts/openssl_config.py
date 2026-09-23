#!/usr/bin/env python3
# SPDX-License-Identifier: LGPL-2.1-or-later
"""Select the installed OpenSSL ABI for Rust host tools, without a C shim."""

import argparse
import os
import shlex
import subprocess
import sys


PROBE = r"""
#include <stdint.h>
#include <openssl/opensslv.h>
#include <openssl/bio.h>
#include <openssl/cms.h>
#if OPENSSL_VERSION_NUMBER >= 0x10100000L && !defined(LIBRESSL_VERSION_NUMBER)
rustcfg_ossl110
#endif
#if OPENSSL_VERSION_NUMBER >= 0x30000000L
rustcfg_ossl300
#elif !defined(OPENSSL_NO_ENGINE) && !defined(OPENSSL_NO_DEPRECATED_3_0)
rustcfg_ossl_engine
#endif
#if OPENSSL_VERSION_NUMBER >= 0x30000000L && OPENSSL_VERSION_NUMBER < 0x40000000L
rustcfg_ossl3series
#endif
#ifdef OPENSSL_LOAD_CONF
rustcfg_ossl_load_conf
#endif
#ifdef CMS_NO_SIGNING_TIME
rustcfg_ossl_no_signing_time
#if CMS_NO_SIGNING_TIME != 0x400000
#error Unsupported CMS_NO_SIGNING_TIME value
#endif
#endif
"""

CFG = ("openssl_configured", "ossl110", "ossl300", "ossl3series", "ossl_engine", "ossl_load_conf",
       "ossl_no_signing_time", "ossl_bio_u64")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cc", default=os.environ.get("HOSTCC", "cc"))
    parser.add_argument("--cflags", default="")
    args = parser.parse_args()
    compiler = shlex.split(args.cc) + shlex.split(args.cflags)
    result = subprocess.run([*compiler, "-E", "-P", "-x", "c", "-"],
                            input=PROBE, text=True, capture_output=True)
    if result.returncode:
        sys.stderr.write(result.stderr)
        return result.returncode
    features = [line.removeprefix("rustcfg_") for line in result.stdout.splitlines()
                if line.startswith("rustcfg_")]
    # BIO_number_written changed from unsigned long to uint64_t. Check the
    # installed declaration, including LibreSSL, without relying on layouts.
    size = subprocess.run([*compiler, "-fsyntax-only", "-x", "c", "-"],
                          input='#include <openssl/bio.h>\n'
                                '_Static_assert(sizeof(BIO_number_written((BIO *)0)) == 8, "width");\n',
                          text=True, capture_output=True)
    if size.returncode == 0:
        features.append("ossl_bio_u64")
    else:
        narrow = subprocess.run([*compiler, "-fsyntax-only", "-x", "c", "-"],
                                input='#include <openssl/bio.h>\n'
                                      '_Static_assert(sizeof(BIO_number_written((BIO *)0)) == sizeof(unsigned long), "width");\n',
                                text=True, capture_output=True)
        if narrow.returncode:
            sys.stderr.write(narrow.stderr)
            return narrow.returncode
    features.append("openssl_configured")
    flags = [f"--check-cfg=cfg({feature})" for feature in CFG]
    flags += [f"--cfg={feature}" for feature in features]
    print(shlex.join(flags))
    return 0


if __name__ == "__main__":
    sys.exit(main())
