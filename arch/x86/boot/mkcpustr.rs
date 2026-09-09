// SPDX-License-Identifier: GPL-2.0-or-later
/* ----------------------------------------------------------------------- *
 *
 *   Copyright 2008 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * This is a host program to preprocess the CPU strings into a
 * compact format suitable for the setup code.
 */

use std::ffi::{c_char, CStr};

// The following names are supplied by the translated CPU feature dependencies.
// C preprocessor constants and included declarations are intentionally retained
// as external source-level references.
extern "C" {
    static x86_cap_flags: *const *const c_char;
}

fn main() {
    let mut i: usize;
    let mut j: usize;
    let mut str_: *const c_char;

    print!("#include <asm/cpufeaturemasks.h>\n\n");
    print!("static const char x86_cap_strs[] =\n");

    i = 0;
    while i < NCAPINTS {
        j = 0;
        while j < 32 {
            unsafe {
                str_ = *x86_cap_flags.add(i * 32 + j);

                if i == NCAPINTS - 1 && j == 31 {
                    /* The last entry must be unconditional; this
                       also consumes the compiler-added null
                       character */
                    if str_.is_null() {
                        str_ = b"\0".as_ptr() as *const c_char;
                    }
                    print!(
                        "\t\\x{:02x}\\x{:02x}\"\"{}\"\n",
                        i,
                        j,
                        CStr::from_ptr(str_).to_string_lossy()
                    );
                } else if !str_.is_null() {
                    print!(
                        "#if REQUIRED_MASK{} & (1 << {})\n\t\"\\x{:02x}\\x{:02x}\"\"{}\\0\"\n#endif\n",
                        i,
                        j,
                        i,
                        j,
                        CStr::from_ptr(str_).to_string_lossy()
                    );
                }
            }
            j += 1;
        }
        i += 1;
    }
    print!("\t;\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
