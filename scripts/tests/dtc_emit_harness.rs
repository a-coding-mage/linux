// SPDX-License-Identifier: GPL-2.0-or-later
//! Test-only entry point for independently exercising device-tree I/O.
#![allow(dead_code)]

#[path = "../dtc/data.rs"]
mod data;
#[path = "../dtc/dtc_header.rs"]
mod dtc_header;
#[path = "../dtc/flattree.rs"]
mod flattree;
#[path = "../dtc/fstree.rs"]
mod fstree;
#[path = "../dtc/livetree.rs"]
mod livetree;
#[path = "../dtc/srcpos.rs"]
mod srcpos;
#[path = "../dtc/treesource.rs"]
mod treesource;
#[path = "../dtc/util.rs"]
mod util;

use dtc_header::*;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;

fn typed_fixture() -> DtInfo {
    let mut tree = DtInfo {
        reserves: vec![Reserve {
            address: 0x1234,
            size: 0x8000,
            labels: vec![Label {
                name: b"reserve_label".to_vec(),
                deleted: false,
            }],
        }],
        ..DtInfo::default()
    };
    tree.root = tree.add_node(Node::default());
    for (name, kind, bytes) in [
        (b"bytes".as_slice(), MarkerKind::Uint8, vec![0x12, 0xff, 0]),
        (b"half", MarkerKind::Uint16, vec![0, 0x12, 0xff, 0xff]),
        (
            b"words",
            MarkerKind::Uint32,
            [1u32.to_be_bytes(), 0xabcd_u32.to_be_bytes()].concat(),
        ),
        (b"wide", MarkerKind::Uint64, u64::MAX.to_be_bytes().to_vec()),
        (
            b"string",
            MarkerKind::String,
            b"a\0b\x80\xff\n\"\\\0".to_vec(),
        ),
    ] {
        tree.nodes[tree.root].properties.push(Property {
            name: name.to_vec(),
            data: Data {
                bytes,
                markers: vec![Marker {
                    kind,
                    offset: 0,
                    reference: None,
                }],
            },
            ..Property::default()
        });
    }
    tree.nodes[tree.root].properties[2]
        .data
        .markers
        .push(Marker {
            kind: MarkerKind::Label,
            offset: 0,
            reference: Some(b"word_label".to_vec()),
        });
    tree.nodes[tree.root].properties[0].labels.push(Label {
        name: b"prop_label".to_vec(),
        deleted: false,
    });
    let child = tree.add_node(Node {
        name: b"child@0".to_vec(),
        labels: vec![Label {
            name: b"child_label".to_vec(),
            deleted: false,
        }],
        ..Node::default()
    });
    tree.add_child(tree.root, child);
    tree
}

fn main() {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    let mut options = Options::default();
    let mut input = b"dtb".to_vec();
    let mut output = b"dtb".to_vec();
    let mut filename = b"-".to_vec();
    let mut boot = None;
    let mut typed = false;
    let mut index = 0;
    while index < arguments.len() {
        let argument = arguments[index].as_bytes();
        index += 1;
        match argument {
            b"--typed-fixture" => typed = true,
            b"-q" => options.quiet += 1,
            b"-T" => options.annotate += 1,
            b"-s" => options.sort = true,
            b"-I" | b"-O" | b"-V" | b"-R" | b"-S" | b"-p" | b"-a" | b"-b" | b"-i" => {
                let value = arguments[index].as_bytes();
                index += 1;
                let integer = || util::strtol(value) as i32;
                match argument {
                    b"-I" => input = value.to_vec(),
                    b"-O" => output = value.to_vec(),
                    b"-V" => options.version = integer(),
                    b"-R" => options.reservenum = integer() as u32,
                    b"-S" => options.minsize = integer(),
                    b"-p" => options.padsize = integer(),
                    b"-a" => options.alignsize = integer(),
                    b"-b" => boot = Some(integer() as u32),
                    b"-i" => options.include_paths.push(value.to_vec()),
                    _ => unreachable!(),
                }
            }
            _ => filename = argument.to_vec(),
        }
    }
    let mut diagnostics = Diagnostics::default();
    let result = (|| {
        let mut tree = if typed {
            typed_fixture()
        } else if input == b"fs" {
            fstree::from_fs(&filename, &options, &mut diagnostics)?
        } else {
            flattree::from_blob(&filename, &options, &mut diagnostics)?
        };
        tree.fill_fullpaths();
        if let Some(boot) = boot {
            tree.boot_cpuid_phys = boot;
        }
        if options.sort {
            tree.sort_tree();
        }
        match output.as_slice() {
            b"dts" => treesource::to_source(&tree, &options, &mut diagnostics),
            b"asm" => flattree::to_asm(&tree, &options, &mut diagnostics),
            _ => flattree::to_blob(&tree, &options, &mut diagnostics),
        }
    })();
    let status = match result {
        Ok(output) => {
            io::stdout().lock().write_all(&output).unwrap();
            0
        }
        Err(error) => {
            diagnostics.raw(b"FATAL ERROR: ");
            diagnostics.raw(error);
            if diagnostics.bytes.last() != Some(&b'\n') {
                diagnostics.raw(b"\n");
            }
            1
        }
    };
    io::stderr().lock().write_all(&diagnostics.bytes).unwrap();
    std::process::exit(status);
}
