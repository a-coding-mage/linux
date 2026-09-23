// SPDX-License-Identifier: GPL-2.0-or-later
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005.

//! Read the directory representation of a device tree.

use crate::dtc_header::{Data, Diagnostics, DtInfo, Node, NodeId, Options, Property};
use crate::srcpos::{errno_text, join_path};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Read;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;

fn path_error(prefix: &[u8], path: &[u8], suffix: &[u8], error: &std::io::Error) -> Vec<u8> {
    [prefix, path, suffix, errno_text(error).as_bytes(), b"\n"].concat()
}

fn read_directory(
    directory: &[u8],
    tree: &mut DtInfo,
    parent: Option<NodeId>,
    name: Vec<u8>,
    active: &mut HashSet<(u64, u64)>,
    diagnostics: &mut Diagnostics,
) -> Result<NodeId, Vec<u8>> {
    let path = OsStr::from_bytes(directory);
    let entries = fs::read_dir(path)
        .map_err(|error| path_error(b"Couldn't opendir() \"", directory, b"\": ", &error))?;
    let metadata =
        fs::metadata(path).map_err(|error| path_error(b"stat(", directory, b"): ", &error))?;
    let identity = (metadata.dev(), metadata.ino());
    if !active.insert(identity) {
        return Err([
            b"Directory cycle reading filesystem tree: ".as_slice(),
            directory,
            b"\n",
        ]
        .concat());
    }
    let id = tree.nodes.len();
    tree.nodes.push(Node {
        parent,
        name,
        ..Node::default()
    });
    for entry in entries {
        let entry = entry.map_err(|error| path_error(b"readdir(", directory, b"): ", &error))?;
        let name = entry.file_name().as_bytes().to_vec();
        let full = join_path(directory, &name);
        let path = OsStr::from_bytes(&full);
        let metadata =
            fs::metadata(path).map_err(|error| path_error(b"stat(", &full, b"): ", &error))?;
        if metadata.is_file() {
            match File::open(path) {
                Ok(file) => {
                    let mut bytes = Vec::new();
                    file.take(metadata.len())
                        .read_to_end(&mut bytes)
                        .map_err(|error| {
                            format!("Error reading file into data: {}", errno_text(&error))
                                .into_bytes()
                        })?;
                    tree.nodes[id].properties.push(Property {
                        name,
                        data: Data {
                            bytes,
                            markers: Vec::new(),
                        },
                        ..Property::default()
                    });
                }
                Err(error) => {
                    diagnostics.raw(path_error(b"WARNING: Cannot open ", &full, b": ", &error))
                }
            }
        } else if metadata.is_dir() {
            let child = read_directory(&full, tree, Some(id), name, active, diagnostics)?;
            tree.nodes[id].children.push(child);
        }
    }
    active.remove(&identity);
    Ok(id)
}

pub(crate) fn from_fs(
    filename: &[u8],
    _options: &Options,
    diagnostics: &mut Diagnostics,
) -> Result<DtInfo, Vec<u8>> {
    let mut tree = DtInfo::default();
    tree.root = read_directory(
        filename,
        &mut tree,
        None,
        Vec::new(),
        &mut HashSet::new(),
        diagnostics,
    )?;
    tree.boot_cpuid_phys = tree.guess_boot_cpuid();
    Ok(tree)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
