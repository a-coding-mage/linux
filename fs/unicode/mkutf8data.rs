// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2014 SGI.
// All rights reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation.
// This program is distributed in the hope that it would be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License
// for more details.
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783

//! Generate and verify compact Unicode normalization tries from UCD text files.
//!
//! C startup preserves inherited signals and stdio. Host lexical adapters are
//! narrow; all Unicode algorithms use owned Rust records, slices and indices.
#![no_main]

use std::ffi::{c_char, c_int, CStr};
use std::fmt::Write as _;
use std::os::unix::ffi::OsStrExt;
#[path = "mkutf8data_data.rs"]
mod data;
#[path = "mkutf8data_io.rs"]
mod io;
#[path = "mkutf8data_model.rs"]
mod model;
#[path = "mkutf8data_trie.rs"]
mod trie;
#[path = "mkutf8data_verify.rs"]
mod verify;
use data::Inputs;
use io::{output, Result};

extern "C" {
    fn getopt(argc: c_int, argv: *const *mut c_char, options: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
}
const HELP: &std::ffi::CStr = c"Usage: %s [options]\n\nThis program creates an a data trie used for parsing and\nnormalization of UTF-8 strings. The trie is derived from\na set of input files from the Unicode character database\nfound at: http://www.unicode.org/Public/UCD/latest/ucd/\n\nThe generated tree supports two normalization forms:\n\n\tnfdi:\n\t- Apply unicode normalization form NFD.\n\t- Remove any Default_Ignorable_Code_Point.\n\n\tnfdicf:\n\t- Apply unicode normalization form NFD.\n\t- Remove any Default_Ignorable_Code_Point.\n\t- Apply a full casefold (C + F).\n\nThese forms were chosen as being most useful when dealing\nwith file names: NFD catches most cases where characters\nshould be considered equivalent. The ignorables are mostly\ninvisible, making names hard to type.\n\nThe options to specify the files to be used are listed\nbelow with their default values, which are the names used\nby version 11.0.0 of the Unicode Character Database.\n\nThe input files:\n\t-a DerivedAge.txt\n\t-c DerivedCombiningClass.txt\n\t-p DerivedCoreProperties.txt\n\t-d UnicodeData.txt\n\t-f CaseFolding.txt\n\t-n NormalizationCorrections.txt\n\nAdditionally, the generated tables are tested using:\n\t-t NormalizationTest.txt\n\nFinally, the output file:\n\t-o utf8data.c\n\n";

fn help(program: &CStr) {
    let bytes = HELP.to_bytes();
    let at = bytes
        .windows(2)
        .position(|w| w == b"%s")
        .expect("help program placeholder");
    output(&bytes[..at]);
    output(program.to_bytes());
    output(&bytes[at + 2..]);
}

fn hexadecimal(value: u32) -> String {
    if value == 0 {
        "0".into()
    } else {
        format!("{value:#x}")
    }
}

fn write_file(inputs: &Inputs, db: &model::Database, forest: &trie::Forest) -> Result<()> {
    if inputs.verbose > 0 {
        output(b"Writing ");
        output(inputs.output.to_bytes());
        output(b"\n");
    }
    let mut file = io::File::open(&inputs.output, true)?;
    let mut text = String::from("/* This file is generated code, do not edit. */\n\n#include <linux/module.h>\n#include <linux/kernel.h>\n#include \"utf8n.h\"\n\nstatic const unsigned int utf8agetab[] = {\n");
    for &age in &db.ages {
        writeln!(
            text,
            "\t{}{}",
            hexadecimal(age),
            if age == db.max_age { "" } else { "," }
        )
        .unwrap();
    }
    text.push_str("};\n\n");
    for (name, start) in [("nfdicf", 0), ("nfdi", 1)] {
        writeln!(text, "static const struct utf8data utf8{name}data[] = {{").unwrap();
        let mut t = start;
        for &age in &db.ages {
            writeln!(
                text,
                "\t{{ {}, {} }}{}",
                hexadecimal(age),
                forest.trees[t].index,
                if age == db.max_age { "" } else { "," }
            )
            .unwrap();
            if forest.trees[t].max_age == age {
                t += 2;
            }
        }
        text.push_str("};\n\n");
    }
    writeln!(
        text,
        "static const unsigned char utf8data[{}] = {{",
        forest.bytes.len()
    )
    .unwrap();
    let mut t = 0;
    for (line, bytes) in forest.bytes.chunks_exact(16).enumerate() {
        let index = line * 16;
        if index == forest.trees[t].index {
            writeln!(
                text,
                "\t/* {}_{:x} */",
                forest.trees[t].name(),
                forest.trees[t].max_age
            )
            .unwrap();
            if t < forest.trees.len() - 1 {
                t += 1;
            }
        }
        text.push('\t');
        for (j, byte) in bytes.iter().enumerate() {
            write!(
                text,
                "0x{byte:02x}{}",
                if index + j < forest.bytes.len() - 1 {
                    ","
                } else {
                    ""
                }
            )
            .unwrap();
        }
        text.push('\n');
    }
    text.push_str("};\n\nconst struct utf8data_table utf8_data_table = {\n\t.utf8agetab = utf8agetab,\n\t.utf8agetab_size = ARRAY_SIZE(utf8agetab),\n\n\t.utf8nfdicfdata = utf8nfdicfdata,\n\t.utf8nfdicfdata_size = ARRAY_SIZE(utf8nfdicfdata),\n\n\t.utf8nfdidata = utf8nfdidata,\n\t.utf8nfdidata_size = ARRAY_SIZE(utf8nfdidata),\n\n\t.utf8data = utf8data,\n};\nEXPORT_SYMBOL_GPL(utf8_data_table);\nMODULE_DESCRIPTION(\"UTF8 data table\");\nMODULE_LICENSE(\"GPL v2\");\n");
    file.write(text.as_bytes());
    Ok(())
}

// SAFETY: called only by the platform C runtime with its valid argc/argv.
unsafe fn run(argc: c_int, argv: *mut *mut c_char) -> Result<i32> {
    // SAFETY: the entrypoint contract guarantees argv[0] is a live C string.
    let program = unsafe { CStr::from_ptr(*argv) };
    let mut inputs = Inputs::default();
    loop {
        // SAFETY: argc/argv satisfy getopt's runtime contract and options lives
        // for the whole call. getopt may reorder the writable pointer array.
        let option = unsafe { getopt(argc, argv, c"a:c:d:f:hn:o:p:t:v".as_ptr()) };
        match option {
            -1 => break,
            118 => inputs.verbose += 1,
            104 => {
                help(program);
                return Ok(0);
            }
            97 | 99 | 100 | 102 | 110 | 111 | 112 | 116 => {
                // SAFETY: successful required-argument options set optarg to a
                // live argv string. Own a copy before the next getopt call.
                let value = unsafe { CStr::from_ptr(optarg) }.to_owned();
                match option {
                    97 => inputs.age = value,
                    99 => inputs.ccc = value,
                    100 => inputs.data = value,
                    102 => inputs.fold = value,
                    110 => inputs.norm = value,
                    111 => inputs.output = value,
                    112 => inputs.prop = value,
                    116 => inputs.test = value,
                    _ => unreachable!(),
                }
            }
            _ => {
                help(program);
                return Ok(1);
            }
        }
    }
    if inputs.verbose > 1 {
        help(program);
    }
    let db = data::load(&inputs)?;
    let forest = trie::build(&db, inputs.verbose);
    verify::trees_verify(&forest, &db, inputs.verbose);
    if inputs.verbose > 2 {
        forest.dump(&db);
    }
    let test_path = std::path::Path::new(std::ffi::OsStr::from_bytes(inputs.test.to_bytes()));
    match verify::normalization_test(&forest, &db, test_path, inputs.verbose) {
        Ok(()) => (),
        Err(verify::TestError::File) => return Err(io::file_error(&inputs.test)),
        Err(verify::TestError::Open(error)) => return Err(io::open_error(&inputs.test, error)),
    }
    write_file(&inputs, &db, &forest)?;
    Ok(0)
}

/// C startup boundary, preserving the caller's signal and stdio dispositions.
///
/// # Safety
/// The platform supplies its valid, NUL-terminated argument vector.
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    // SAFETY: these are exactly the platform's argc/argv, passed on unchanged.
    match unsafe { run(argc, argv) } {
        Ok(status) => status,
        Err(error) => {
            output(&error);
            1
        }
    }
}
