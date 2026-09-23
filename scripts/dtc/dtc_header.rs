// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005.
 */

//! Owned compiler data model. Arena indices remain valid when trees are merged.

use std::borrow::Cow;

pub(crate) type NodeId = usize;
pub(crate) type PropId = usize;
pub(crate) const DTSF_V1: u32 = 1;
pub(crate) const DTSF_PLUGIN: u32 = 2;
pub(crate) const PHANDLE_LEGACY: u32 = 1;
pub(crate) const PHANDLE_EPAPR: u32 = 2;
pub(crate) const PHANDLE_BOTH: u32 = 3;

pub(crate) fn display(bytes: &[u8]) -> Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

pub(crate) fn phandle_is_valid(value: u32) -> bool {
    value != 0 && value != u32::MAX
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MarkerKind {
    None,
    RefPhandle,
    RefPath,
    Label,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    String,
}

impl MarkerKind {
    pub(crate) fn is_type(self) -> bool {
        matches!(
            self,
            Self::Uint8 | Self::Uint16 | Self::Uint32 | Self::Uint64 | Self::String
        )
    }

    pub(crate) fn bits(self) -> Option<usize> {
        match self {
            Self::Uint8 => Some(8),
            Self::Uint16 => Some(16),
            Self::Uint32 => Some(32),
            Self::Uint64 => Some(64),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Marker {
    pub(crate) kind: MarkerKind,
    pub(crate) offset: usize,
    pub(crate) reference: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Data {
    pub(crate) bytes: Vec<u8>,
    pub(crate) markers: Vec<Marker>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SourcePos {
    pub(crate) file: Vec<u8>,
    // Live lexer locations refer to a source-file slot until copied into a node.
    pub(crate) file_id: Option<usize>,
    pub(crate) first_line: i32,
    pub(crate) first_column: i32,
    pub(crate) last_line: i32,
    pub(crate) last_column: i32,
}

#[derive(Clone, Debug)]
pub(crate) struct Label {
    pub(crate) name: Vec<u8>,
    pub(crate) deleted: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Property {
    pub(crate) deleted: bool,
    pub(crate) name: Vec<u8>,
    pub(crate) data: Data,
    pub(crate) labels: Vec<Label>,
    pub(crate) srcpos: Vec<SourcePos>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Node {
    pub(crate) deleted: bool,
    pub(crate) name: Vec<u8>,
    pub(crate) properties: Vec<Property>,
    pub(crate) children: Vec<NodeId>,
    pub(crate) parent: Option<NodeId>,
    pub(crate) fullpath: Vec<u8>,
    pub(crate) basenamelen: usize,
    pub(crate) phandle: u32,
    pub(crate) addr_cells: i32,
    pub(crate) size_cells: i32,
    pub(crate) labels: Vec<Label>,
    pub(crate) bus: Option<&'static str>,
    pub(crate) srcpos: Vec<SourcePos>,
    pub(crate) omit_if_unused: bool,
    pub(crate) is_referenced: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Reserve {
    pub(crate) address: u64,
    pub(crate) size: u64,
    pub(crate) labels: Vec<Label>,
}

#[derive(Clone, Debug)]
pub(crate) struct DtInfo {
    pub(crate) nodes: Vec<Node>,
    pub(crate) root: NodeId,
    pub(crate) reserves: Vec<Reserve>,
    pub(crate) dtsflags: u32,
    pub(crate) boot_cpuid_phys: u32,
    pub(crate) outname: Vec<u8>,
    pub(crate) initial_path: Vec<u8>,
    pub(crate) dependencies: Vec<Vec<u8>>,
    pub(crate) next_phandle: u32,
    pub(crate) next_orphan_fragment: u32,
}

impl Default for DtInfo {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            root: 0,
            reserves: Vec::new(),
            dtsflags: DTSF_V1,
            boot_cpuid_phys: 0,
            outname: b"-".to_vec(),
            initial_path: Vec::new(),
            dependencies: Vec::new(),
            next_phandle: 1,
            next_orphan_fragment: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Options {
    pub(crate) quiet: u32,
    pub(crate) reservenum: u32,
    pub(crate) minsize: i32,
    pub(crate) padsize: i32,
    pub(crate) alignsize: i32,
    pub(crate) phandle_format: u32,
    pub(crate) generate_symbols: bool,
    pub(crate) generate_fixups: bool,
    pub(crate) auto_label_aliases: bool,
    pub(crate) annotate: u32,
    pub(crate) force: bool,
    pub(crate) sort: bool,
    pub(crate) version: i32,
    pub(crate) include_paths: Vec<Vec<u8>>,
    pub(crate) checks: Vec<(bool, bool, Vec<u8>)>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            quiet: 0,
            reservenum: 0,
            minsize: 0,
            padsize: 0,
            alignsize: 0,
            phandle_format: PHANDLE_EPAPR,
            generate_symbols: false,
            generate_fixups: false,
            auto_label_aliases: false,
            annotate: 0,
            force: false,
            sort: false,
            version: 17,
            include_paths: Vec::new(),
            checks: Vec::new(),
        }
    }
}

/// Diagnostics are accumulated as bytes so filenames need not be UTF-8.
#[derive(Default)]
pub(crate) struct Diagnostics {
    pub(crate) bytes: Vec<u8>,
    pub(crate) source_error: bool,
    pub(crate) input_dependencies: Vec<Vec<u8>>,
}

impl Diagnostics {
    pub(crate) fn raw(&mut self, message: impl AsRef<[u8]>) {
        self.bytes.extend_from_slice(message.as_ref());
    }

    pub(crate) fn source(&mut self, prefix: &[u8], pos: &SourcePos, message: impl AsRef<[u8]>) {
        self.raw(prefix);
        self.raw(b": ");
        self.raw(pos.render());
        self.raw(b" ");
        self.raw(message);
        self.raw(b"\n");
        self.source_error = true;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
