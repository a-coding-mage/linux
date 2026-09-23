// SPDX-License-Identifier: GPL-2.0-or-later
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005.

//! Checked flattened-tree input and binary/assembler serialization.

use crate::dtc_header::{
    Data, Diagnostics, DtInfo, Label, MarkerKind, Node, NodeId, Options, Property, Reserve,
    DTSF_PLUGIN,
};
use crate::srcpos::{errno_text, Sources};
use std::io::{self, Read};

#[allow(dead_code)]
#[path = "libfdt/fdt_header.rs"]
mod format;
use format::*;

#[derive(Clone, Copy)]
struct Version {
    number: u32,
    compatible: u32,
    header_size: usize,
    full_paths: bool,
    variable_alignment: bool,
    name_properties: bool,
}

fn version(number: i32) -> Result<Version, Vec<u8>> {
    let (compatible, header_size) = match number {
        1 => (1, FDT_V1_SIZE),
        2 => (1, FDT_V2_SIZE),
        3 => (1, FDT_V3_SIZE),
        16 => (16, FDT_V16_SIZE),
        17 => (16, FDT_V17_SIZE),
        _ => return Err(format!("Unknown device tree blob version {number}\n").into_bytes()),
    };
    Ok(Version {
        number: number as u32,
        compatible,
        header_size,
        full_paths: number < 16,
        variable_alignment: number < 16,
        name_properties: number < 16,
    })
}

fn align(value: usize, alignment: usize) -> Result<usize, Vec<u8>> {
    if !alignment.is_power_of_two() {
        return Err(b"Invalid device tree alignment".to_vec());
    }
    value
        .checked_add(alignment - 1)
        .map(|value| value & !(alignment - 1))
        .ok_or_else(|| b"Device tree size overflow".to_vec())
}

fn word(value: usize) -> Result<u32, Vec<u8>> {
    u32::try_from(value).map_err(|_| b"Device tree size exceeds 32-bit format".to_vec())
}

trait Emitter {
    fn cell(&mut self, value: u32);
    fn string(&mut self, value: &[u8], length: usize);
    fn align(&mut self, alignment: usize) -> Result<(), Vec<u8>>;
    fn data(&mut self, value: &Data);
    fn tag(&mut self, value: u32, labels: &[Label]);
}

#[derive(Default)]
struct Binary(Vec<u8>);

impl Emitter for Binary {
    fn cell(&mut self, value: u32) {
        self.0.extend_from_slice(&value.to_be_bytes());
    }
    fn string(&mut self, value: &[u8], length: usize) {
        self.0
            .extend_from_slice(if length == 0 { value } else { &value[..length] });
        self.0.push(0);
    }
    fn align(&mut self, alignment: usize) -> Result<(), Vec<u8>> {
        self.0.resize(align(self.0.len(), alignment)?, 0);
        Ok(())
    }
    fn data(&mut self, value: &Data) {
        self.0.extend_from_slice(&value.bytes);
    }
    fn tag(&mut self, value: u32, _labels: &[Label]) {
        self.cell(value);
    }
}

#[derive(Default)]
struct Assembly(Vec<u8>);

impl Assembly {
    fn write(&mut self, value: impl AsRef<[u8]>) {
        self.0.extend_from_slice(value.as_ref());
    }
    fn label(&mut self, name: &str) {
        self.write(format!("\t.globl\tdt_{name}\ndt_{name}:\n_dt_{name}:\n"));
    }
    fn expression(&mut self, expression: &str) {
        for shift in [24, 16, 8] {
            self.write(format!("\t.byte\t(({expression}) >> {shift}) & 0xff\n"));
        }
        self.write(format!("\t.byte\t({expression}) & 0xff\n"));
    }
    fn named_label(&mut self, name: &[u8], suffix: &[u8]) {
        self.write(b"\t.globl\t");
        self.write(name);
        self.write(suffix);
        self.write(b"\n");
        self.write(name);
        self.write(suffix);
        self.write(b":\n");
    }
}

impl Emitter for Assembly {
    fn cell(&mut self, value: u32) {
        for byte in value.to_be_bytes() {
            self.write(format!("\t.byte\t0x{byte:02x}\n"));
        }
    }
    fn string(&mut self, value: &[u8], length: usize) {
        self.write(b"\t.asciz\t\"");
        self.write(if length == 0 { value } else { &value[..length] });
        self.write(b"\"\n");
    }
    fn align(&mut self, alignment: usize) -> Result<(), Vec<u8>> {
        self.write(format!("\t.balign\t{alignment}, 0\n"));
        Ok(())
    }
    fn data(&mut self, value: &Data) {
        for marker in value
            .markers
            .iter()
            .filter(|marker| marker.kind == MarkerKind::Label)
        {
            let name = marker.reference.as_deref().unwrap_or_default();
            self.write(b"\t.globl\t");
            self.write(name);
            self.write(b"\n");
            self.write(name);
            self.write(format!("\t= . + {}\n", marker.offset));
        }
        let mut words = value.bytes.chunks_exact(4);
        for bytes in &mut words {
            self.cell(u32::from_be_bytes(bytes.try_into().unwrap()));
        }
        for byte in words.remainder() {
            self.write(format!("\t.byte\t0x{byte:x}\n"));
        }
    }
    fn tag(&mut self, value: u32, labels: &[Label]) {
        if value != FDT_END_NODE {
            for label in labels.iter().filter(|label| !label.deleted) {
                self.named_label(&label.name, b"");
            }
        }
        let comment = match value {
            FDT_BEGIN_NODE => "FDT_BEGIN_NODE",
            FDT_END_NODE => "FDT_END_NODE",
            FDT_PROP => "FDT_PROP",
            _ => "FDT_END",
        };
        self.write(format!("\t/* {comment} */\n"));
        self.cell(value);
        if value == FDT_END_NODE {
            for label in labels.iter().filter(|label| !label.deleted) {
                self.named_label(&label.name, b"_end");
            }
        }
    }
}

fn intern(strings: &mut Vec<u8>, name: &[u8]) -> Result<u32, Vec<u8>> {
    let needle = [name, b"\0"].concat();
    if let Some(offset) = strings
        .windows(needle.len())
        .position(|window| window == needle)
    {
        return word(offset);
    }
    let offset = word(strings.len())?;
    strings.extend_from_slice(&needle);
    Ok(offset)
}

fn flatten(
    tree: &DtInfo,
    id: NodeId,
    output: &mut impl Emitter,
    strings: &mut Vec<u8>,
    version: Version,
) -> Result<(), Vec<u8>> {
    let mut pending = vec![(id, false)];
    while let Some((id, end)) = pending.pop() {
        let node = &tree.nodes[id];
        if node.deleted {
            continue;
        }
        if end {
            output.tag(FDT_END_NODE, &node.labels);
            continue;
        }
        output.tag(FDT_BEGIN_NODE, &node.labels);
        output.string(
            if version.full_paths {
                &node.fullpath
            } else {
                &node.name
            },
            0,
        );
        output.align(4)?;
        let mut has_name = false;
        for property in node.properties.iter().filter(|property| !property.deleted) {
            has_name |= property.name == b"name";
            let name_offset = intern(strings, &property.name)?;
            output.tag(FDT_PROP, &property.labels);
            output.cell(word(property.data.bytes.len())?);
            output.cell(name_offset);
            if version.variable_alignment && property.data.bytes.len() >= 8 {
                output.align(8)?;
            }
            output.data(&property.data);
            output.align(4)?;
        }
        if version.name_properties && !has_name {
            output.tag(FDT_PROP, &[]);
            output.cell(word(node.basenamelen + 1)?);
            output.cell(intern(strings, b"name")?);
            if version.variable_alignment && node.basenamelen + 1 >= 8 {
                output.align(8)?;
            }
            output.string(&node.name, node.basenamelen);
            output.align(4)?;
        }
        pending.push((id, true));
        for &child in node.children.iter().rev() {
            pending.push((child, false));
        }
    }
    Ok(())
}

pub(crate) fn to_blob(
    tree: &DtInfo,
    options: &Options,
    diagnostics: &mut Diagnostics,
) -> Result<Vec<u8>, Vec<u8>> {
    let version = version(options.version)?;
    let mut structure = Binary::default();
    let mut strings = Vec::new();
    flatten(tree, tree.root, &mut structure, &mut strings, version)?;
    structure.cell(FDT_END);
    let reserve_offset = align(version.header_size, 8)?;
    let count = tree
        .reserves
        .len()
        .checked_add(options.reservenum as usize)
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| b"Device tree size overflow".to_vec())?;
    let structure_offset = count
        .checked_mul(16)
        .and_then(|value| value.checked_add(reserve_offset))
        .ok_or_else(|| b"Device tree size overflow".to_vec())?;
    let strings_offset = structure_offset
        .checked_add(structure.0.len())
        .ok_or_else(|| b"Device tree size overflow".to_vec())?;
    let size = strings_offset
        .checked_add(strings.len())
        .ok_or_else(|| b"Device tree size overflow".to_vec())?;
    let mut padding = 0;
    if options.minsize > 0 {
        padding = (options.minsize as usize).saturating_sub(size);
        if size > options.minsize as usize && options.quiet < 1 {
            diagnostics.raw(format!(
                "Warning: blob size {} >= minimum size {}\n",
                word(size)?,
                options.minsize
            ));
        }
    }
    if options.padsize > 0 {
        padding = options.padsize as usize;
    }
    let mut total = size
        .checked_add(padding)
        .ok_or_else(|| b"Device tree size overflow".to_vec())?;
    if options.alignsize > 0 {
        total = align(total, options.alignsize as usize)?;
    }
    let header = [
        FDT_MAGIC,
        word(total)?,
        word(structure_offset)?,
        word(strings_offset)?,
        word(reserve_offset)?,
        version.number,
        version.compatible,
        tree.boot_cpuid_phys,
        word(strings.len())?,
        word(structure.0.len())?,
    ];
    let mut output = Vec::new();
    output
        .try_reserve_exact(total)
        .map_err(|_| b"Out of memory allocating device tree blob".to_vec())?;
    for field in &header[..version.header_size / 4] {
        output.extend_from_slice(&field.to_be_bytes());
    }
    output.resize(reserve_offset, 0);
    for reserve in &tree.reserves {
        output.extend_from_slice(&reserve.address.to_be_bytes());
        output.extend_from_slice(&reserve.size.to_be_bytes());
    }
    output.resize(structure_offset, 0);
    output.extend_from_slice(&structure.0);
    output.extend_from_slice(&strings);
    output.resize(total, 0);
    Ok(output)
}

pub(crate) fn to_asm(
    tree: &DtInfo,
    options: &Options,
    _diagnostics: &mut Diagnostics,
) -> Result<Vec<u8>, Vec<u8>> {
    let version = version(options.version)?;
    let mut output = Assembly::default();
    let mut strings = Vec::new();
    output.write(b"/* autogenerated by dtc, do not edit */\n\n");
    output.label("blob_start");
    output.label("header");
    output.write(b"\t/* magic */\n");
    output.cell(FDT_MAGIC);
    for (field, end) in [
        ("totalsize", "blob_abs_end"),
        ("off_dt_struct", "struct_start"),
        ("off_dt_strings", "strings_start"),
        ("off_mem_rsvmap", "reserve_map"),
    ] {
        output.write(format!("\t/* {field} */\n"));
        output.expression(&format!("_dt_{end} - _dt_blob_start"));
    }
    output.write(b"\t/* version */\n");
    output.cell(version.number);
    output.write(b"\t/* last_comp_version */\n");
    output.cell(version.compatible);
    if version.number >= 2 {
        output.write(b"\t/* boot_cpuid_phys */\n");
        output.cell(tree.boot_cpuid_phys);
    }
    if version.number >= 3 {
        output.write(b"\t/* size_dt_strings */\n");
        output.expression("_dt_strings_end - _dt_strings_start");
    }
    if version.number >= 17 {
        output.write(b"\t/* size_dt_struct */\n");
        output.expression("_dt_struct_end - _dt_struct_start");
    }
    output.align(8)?;
    output.label("reserve_map");
    output.write(b"/* Memory reserve map from source file */\n");
    for reserve in &tree.reserves {
        for label in reserve.labels.iter().filter(|label| !label.deleted) {
            output.named_label(&label.name, b"");
        }
        for value in [
            (reserve.address >> 32) as u32,
            reserve.address as u32,
            (reserve.size >> 32) as u32,
            reserve.size as u32,
        ] {
            output.expression(&format!("0x{value:08x}"));
        }
    }
    for _ in 0..=options.reservenum {
        output.write(b"\t.long\t0, 0\n\t.long\t0, 0\n");
    }
    output.label("struct_start");
    flatten(tree, tree.root, &mut output, &mut strings, version)?;
    output.tag(FDT_END, &[]);
    output.label("struct_end");
    output.label("strings_start");
    for name in strings.split_inclusive(|&byte| byte == 0) {
        output.write(b"\t.asciz \"");
        output.write(&name[..name.len() - 1]);
        output.write(b"\"\n");
    }
    output.label("strings_end");
    output.label("blob_end");
    if options.minsize > 0 {
        output.write(format!(
            "\t.space\t{} - (_dt_blob_end - _dt_blob_start), 0\n",
            options.minsize
        ));
    }
    if options.padsize > 0 {
        output.write(format!("\t.space\t{}, 0\n", options.padsize));
    }
    if options.alignsize > 0 {
        output.align(options.alignsize as usize)?;
    }
    output.label("blob_abs_end");
    Ok(output.0)
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, size: usize) -> Result<&'a [u8], Vec<u8>> {
        let end = self
            .offset
            .checked_add(size)
            .ok_or_else(|| b"Premature end of data parsing flat device tree\n".to_vec())?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| b"Premature end of data parsing flat device tree\n".to_vec())?;
        self.offset = end;
        Ok(value)
    }
    fn word(&mut self) -> Result<u32, Vec<u8>> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn align(&mut self, alignment: usize) -> Result<(), Vec<u8>> {
        let offset = align(self.offset, alignment)?;
        self.take(offset - self.offset)?;
        Ok(())
    }
    fn string(&mut self) -> Result<Vec<u8>, Vec<u8>> {
        let length = self.bytes[self.offset..]
            .iter()
            .position(|&byte| byte == 0)
            .ok_or_else(|| b"Premature end of data parsing flat device tree\n".to_vec())?;
        let name = self.take(length)?.to_vec();
        self.take(1)?;
        self.align(4)?;
        Ok(name)
    }
}

fn read_node(
    reader: &mut Reader<'_>,
    parent_name: &[u8],
    old: bool,
    parent: Option<NodeId>,
    tree: &mut DtInfo,
) -> Result<(NodeId, Vec<u8>), Vec<u8>> {
    let flat_name = reader.string()?;
    let name = if old {
        if !flat_name.starts_with(parent_name) {
            return Err([
                b"Path \"".as_slice(),
                &flat_name,
                b"\" is not valid as a child of \"",
                parent_name,
                b"\"\n",
            ]
            .concat());
        }
        let start = parent_name.len() + usize::from(parent_name != b"/");
        flat_name
            .get(start..)
            .ok_or_else(|| b"Invalid full-path node name\n".to_vec())?
            .to_vec()
    } else {
        flat_name.clone()
    };
    let id = tree.nodes.len();
    tree.nodes.push(Node {
        name,
        parent,
        ..Node::default()
    });
    Ok((id, flat_name))
}

fn unflatten(
    reader: &mut Reader<'_>,
    strings: &[u8],
    old: bool,
    tree: &mut DtInfo,
    diagnostics: &mut Diagnostics,
) -> Result<NodeId, Vec<u8>> {
    let root = read_node(reader, b"", old, None, tree)?;
    let root_id = root.0;
    let mut pending = vec![root];
    while let Some((id, flat_name)) = pending.last() {
        let id = *id;
        match reader.word()? {
            FDT_PROP => {
                if !tree.nodes[id].children.is_empty() {
                    diagnostics
                        .raw(b"Warning: Flat tree input has subnodes preceding a property.\n");
                }
                let size = reader.word()? as usize;
                let name_offset = reader.word()?;
                let tail = strings.get(name_offset as usize..).ok_or_else(|| {
                    format!(
                        "String offset {} overruns string table\n",
                        name_offset as i32
                    )
                    .into_bytes()
                })?;
                let length = tail.iter().position(|&byte| byte == 0).ok_or_else(|| {
                    format!(
                        "String offset {} overruns string table\n",
                        name_offset as i32
                    )
                    .into_bytes()
                })?;
                let name = tail[..length].to_vec();
                if old && size >= 8 {
                    reader.align(8)?;
                }
                let bytes = reader.take(size)?.to_vec();
                if size != 0 {
                    reader.align(4)?;
                }
                tree.nodes[id].properties.push(Property {
                    name,
                    data: Data {
                        bytes,
                        markers: Vec::new(),
                    },
                    ..Property::default()
                });
            }
            FDT_BEGIN_NODE => {
                let child = read_node(reader, flat_name, old, Some(id), tree)?;
                tree.nodes[id].children.push(child.0);
                pending.push(child);
            }
            FDT_END_NODE => {
                pending.pop();
            }
            FDT_END => return Err(b"Premature FDT_END in device tree blob\n".to_vec()),
            FDT_NOP => {
                if old {
                    diagnostics.raw(b"Warning: NOP tag found in flat tree version <16\n");
                }
            }
            value => {
                return Err(
                    format!("Invalid opcode word {value:08x} in device tree blob\n").into_bytes(),
                )
            }
        }
    }
    Ok(root_id)
}

pub(crate) fn from_blob(
    filename: &[u8],
    options: &Options,
    diagnostics: &mut Diagnostics,
) -> Result<DtInfo, Vec<u8>> {
    let mut sources = Sources::new(options.include_paths.clone());
    let (_, mut input) = sources.open_stream(filename)?;
    diagnostics.input_dependencies = sources.dependencies.clone();
    let mut header = [0u8; 8];
    let read_header = |input: &mut dyn Read, buffer: &mut [u8], field: &str| {
        input.read_exact(buffer).map_err(|error| {
            if error.kind() == io::ErrorKind::UnexpectedEof {
                format!("EOF reading DT blob {field}\n").into_bytes()
            } else {
                format!("Error reading DT blob {field}: {}\n", errno_text(&error)).into_bytes()
            }
        })
    };
    read_header(&mut input, &mut header[..4], "magic number")?;
    if u32::from_be_bytes(header[..4].try_into().unwrap()) != FDT_MAGIC {
        return Err(b"Blob has incorrect magic number\n".to_vec());
    }
    read_header(&mut input, &mut header[4..], "size")?;
    let total = u32::from_be_bytes(header[4..].try_into().unwrap()) as usize;
    if total < FDT_V1_SIZE {
        return Err(format!("DT blob size ({total}) is too small\n").into_bytes());
    }
    // Read only the declared image. This handles pipes without waiting for
    // EOF and does not allocate an attacker-controlled size before data arrives.
    let mut bytes = header.to_vec();
    input
        .take((total - header.len()) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("Error reading DT blob: {}\n", errno_text(&error)).into_bytes())?;
    if bytes.len() != total {
        return Err(format!("EOF before reading {total} bytes of DT blob\n").into_bytes());
    }
    let bytes = bytes.as_slice();
    let read = |offset: usize| -> Result<u32, Vec<u8>> {
        let value = bytes
            .get(offset..offset + 4)
            .ok_or_else(|| b"Truncated device tree header\n".to_vec())?;
        Ok(u32::from_be_bytes(value.try_into().unwrap()))
    };
    let structure_offset = read(8)? as usize;
    let strings_offset = read(12)? as usize;
    let reserve_offset = read(16)? as usize;
    let version = read(20)?;
    if reserve_offset >= total {
        return Err(b"Mem Reserve structure offset exceeds total size\n".to_vec());
    }
    if structure_offset >= total {
        return Err(b"DT structure offset exceeds total size\n".to_vec());
    }
    if strings_offset > total {
        return Err(b"String table offset exceeds total size\n".to_vec());
    }
    let strings_end = if version >= 3 {
        strings_offset
            .checked_add(read(32)? as usize)
            .filter(|&end| end <= total)
            .ok_or_else(|| b"String table extends past total size\n".to_vec())?
    } else {
        total
    };
    if version >= 17 {
        structure_offset
            .checked_add(read(36)? as usize)
            .filter(|&end| end <= total)
            .ok_or_else(|| b"Structure block extends past total size\n".to_vec())?;
    }
    // C reads the v2 field even in v1 input, where those bytes are header
    // padding. Preserve that behavior when present, without out-of-bounds reads.
    let boot_cpuid_phys = read(28)?;
    let mut tree = DtInfo {
        boot_cpuid_phys,
        dependencies: sources.dependencies,
        ..DtInfo::default()
    };
    let mut reserves = Reader {
        bytes: &bytes[reserve_offset..],
        offset: 0,
    };
    loop {
        let address = u64::from_be_bytes(reserves.take(8)?.try_into().unwrap());
        let size = u64::from_be_bytes(reserves.take(8)?.try_into().unwrap());
        if size == 0 {
            break;
        }
        tree.reserves.push(Reserve {
            address,
            size,
            ..Reserve::default()
        });
    }
    let mut reader = Reader {
        bytes: &bytes[structure_offset..],
        offset: 0,
    };
    let first = reader.word()?;
    if first != FDT_BEGIN_NODE {
        return Err(format!(
            "Device tree blob doesn't begin with FDT_BEGIN_NODE (begins with 0x{first:08x})\n"
        )
        .into_bytes());
    }
    tree.root = unflatten(
        &mut reader,
        &bytes[strings_offset..strings_end],
        version < 16,
        &mut tree,
        diagnostics,
    )?;
    if reader.word()? != FDT_END {
        return Err(b"Device tree blob doesn't end with FDT_END\n".to_vec());
    }
    if tree.nodes[tree.root].children.iter().any(|&id| {
        matches!(
            tree.nodes[id].name.as_slice(),
            b"__fixups__" | b"__local_fixups__"
        )
    }) {
        tree.dtsflags |= DTSF_PLUGIN;
    }
    Ok(tree)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
