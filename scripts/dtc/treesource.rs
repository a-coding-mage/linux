// SPDX-License-Identifier: GPL-2.0-or-later
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005.

//! Byte-preserving DTS output and inferred property-value markers.

use crate::dtc_header::{
    Diagnostics, DtInfo, Label, Marker, MarkerKind, NodeId, Options, Property, SourcePos,
    DTSF_PLUGIN,
};
use crate::srcpos;

pub(crate) fn property_add_marker(
    property: &mut Property,
    kind: MarkerKind,
    offset: usize,
    reference: Option<Vec<u8>>,
) {
    let markers = &mut property.data.markers;
    let mut index = markers.partition_point(|marker| marker.offset < offset);
    if let Some(marker) = markers.get(index) {
        if marker.offset == offset && marker.kind.is_type() {
            if kind.is_type() {
                return;
            }
            index += 1;
        }
    }
    if markers
        .get(index)
        .is_some_and(|marker| marker.offset == offset && marker.kind == kind)
    {
        return;
    }
    markers.insert(
        index,
        Marker {
            kind,
            offset,
            reference,
        },
    );
}

pub(crate) fn add_phandle_marker(
    tree: &mut DtInfo,
    node: NodeId,
    property: usize,
    offset: usize,
    options: &Options,
    diagnostics: &mut Diagnostics,
) {
    let prop = &tree.nodes[node].properties[property];
    let Some(bytes) = offset
        .checked_add(4)
        .and_then(|end| prop.data.bytes.get(offset..end))
    else {
        if options.quiet < 1 {
            diagnostics.raw(b"Warning: property ");
            diagnostics.raw(&prop.name);
            diagnostics.raw(format!(
                " too short to contain a phandle at offset {offset}\n"
            ));
        }
        return;
    };
    let phandle = u32::from_be_bytes(bytes.try_into().unwrap());
    let reference = tree.node_by_phandle(phandle).map(|id| {
        let candidate = &tree.nodes[id];
        candidate
            .labels
            .first()
            .map_or_else(|| candidate.fullpath.clone(), |label| label.name.clone())
    });
    if let Some(reference) = reference {
        property_add_marker(
            &mut tree.nodes[node].properties[property],
            MarkerKind::RefPhandle,
            offset,
            Some(reference),
        );
    } else if options.quiet < 1 {
        diagnostics.raw(format!(
            "Warning: node referenced by phandle 0x{phandle:x} in property "
        ));
        diagnostics.raw(&prop.name);
        diagnostics.raw(b" not found\n");
    }
}

fn guess_range(property: &mut Property, start: usize, end: usize) -> Result<(), Vec<u8>> {
    if start == end {
        return Ok(());
    }
    let bytes = property
        .data
        .bytes
        .get(start..end)
        .ok_or_else(|| b"Invalid property marker offset".to_vec())?;
    let zeroes = bytes.iter().filter(|&&byte| byte == 0).count();
    let string = bytes.last() == Some(&0)
        && zeroes <= bytes.len() - zeroes
        && bytes
            .iter()
            .all(|byte| matches!(byte, 0 | 7..=13 | 32..=126));
    let kind = if string {
        MarkerKind::String
    } else if bytes.len() % 4 == 0 {
        MarkerKind::Uint32
    } else {
        MarkerKind::Uint8
    };
    let starts = if string {
        bytes
            .iter()
            .enumerate()
            .filter_map(|(index, &byte)| {
                (byte == 0 && index + 1 < bytes.len()).then_some(start + index + 1)
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    property_add_marker(property, kind, start, None);
    for offset in starts {
        property_add_marker(property, MarkerKind::String, offset, None);
    }
    Ok(())
}

fn infer_types(property: &mut Property) -> Result<(), Vec<u8>> {
    if property
        .data
        .markers
        .iter()
        .any(|marker| marker.kind.is_type())
    {
        return Ok(());
    }
    let mut offset = 0;
    for marker in property.data.markers.clone() {
        if marker.offset > offset {
            guess_range(property, offset, marker.offset)?;
            offset = marker.offset;
        }
        if marker.kind == MarkerKind::RefPhandle {
            property_add_marker(property, MarkerKind::Uint32, offset, None);
            offset = offset
                .checked_add(4)
                .ok_or_else(|| b"Invalid property marker offset".to_vec())?;
        }
    }
    if offset < property.data.bytes.len() {
        guess_range(property, offset, property.data.bytes.len())?;
    }
    Ok(())
}

fn write_string(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), Vec<u8>> {
    if bytes.is_empty() {
        return Ok(());
    }
    if bytes.last() != Some(&0) {
        return Err(b"Unterminated string property marker".to_vec());
    }
    output.push(b'"');
    for &byte in &bytes[..bytes.len() - 1] {
        let escape: Option<&[u8]> = match byte {
            0 => Some(b"\\0"),
            7 => Some(b"\\a"),
            8 => Some(b"\\b"),
            9 => Some(b"\\t"),
            10 => Some(b"\\n"),
            11 => Some(b"\\v"),
            12 => Some(b"\\f"),
            13 => Some(b"\\r"),
            b'\\' => Some(b"\\\\"),
            b'"' => Some(b"\\\""),
            _ => None,
        };
        if let Some(escape) = escape {
            output.extend_from_slice(escape);
        } else if (32..=126).contains(&byte) {
            output.push(byte);
        } else {
            // Match the C tool's char promotion, including long hexadecimal
            // escapes on signed-char hosts. Its parser truncates them to u8.
            let promoted = byte as std::ffi::c_char as i32 as u32;
            output.extend_from_slice(format!("\\x{promoted:02x}").as_bytes());
        }
    }
    output.push(b'"');
    Ok(())
}

fn write_integers(output: &mut Vec<u8>, bytes: &[u8], width: usize) -> Result<(), Vec<u8>> {
    if bytes.len() % width != 0 {
        return Err(b"Unaligned integer property marker".to_vec());
    }
    for (index, bytes) in bytes.chunks_exact(width).enumerate() {
        if index != 0 {
            output.push(b' ');
        }
        let value = bytes
            .iter()
            .fold(0u64, |value, &byte| (value << 8) | u64::from(byte));
        if width != 1 {
            output.extend_from_slice(b"0x");
        }
        output.extend_from_slice(format!("{value:02x}").as_bytes());
    }
    Ok(())
}

fn delimiters(kind: MarkerKind) -> (&'static [u8], &'static [u8]) {
    match kind {
        MarkerKind::Uint8 => (b"[", b"]"),
        MarkerKind::Uint16 => (b"/bits/ 16 <", b">"),
        MarkerKind::Uint32 => (b"<", b">"),
        MarkerKind::Uint64 => (b"/bits/ 64 <", b">"),
        _ => (b"", b""),
    }
}

fn annotation(
    output: &mut Vec<u8>,
    positions: &[SourcePos],
    tree: &DtInfo,
    options: &Options,
    first: bool,
) {
    if options.annotate != 0 {
        if let Some(comment) =
            srcpos::comment(positions, &tree.initial_path, first, options.annotate)
        {
            output.extend_from_slice(b" /* ");
            output.extend_from_slice(&comment);
            output.extend_from_slice(b" */");
        }
    }
}

fn write_value(
    output: &mut Vec<u8>,
    property: &Property,
    tree: &DtInfo,
    options: &Options,
) -> Result<(), Vec<u8>> {
    let mut property = property.clone();
    let length = property.data.bytes.len();
    if length != 0 {
        output.extend_from_slice(b" =");
        infer_types(&mut property)?;
        let markers = &property.data.markers;
        let mut emit = MarkerKind::None;
        for (index, marker) in markers.iter().enumerate() {
            let end = markers.get(index + 1).map_or(length, |next| next.offset);
            let data_end = markers[index + 1..]
                .iter()
                .find(|next| next.kind.is_type())
                .map_or(length, |next| next.offset);
            let chunk = property
                .data
                .bytes
                .get(marker.offset..end)
                .ok_or_else(|| b"Invalid property marker offset".to_vec())?;
            if marker.kind.is_type() {
                emit = marker.kind;
                output.push(b' ');
                output.extend_from_slice(delimiters(emit).0);
            } else if marker.kind == MarkerKind::Label {
                output.push(b' ');
                output.extend_from_slice(marker.reference.as_deref().unwrap_or_default());
                output.push(b':');
            }
            if emit == MarkerKind::None || chunk.is_empty() {
                continue;
            }
            match emit {
                MarkerKind::Uint32 => {
                    if let Some(reference) = markers.iter().find(|other| {
                        other.kind == MarkerKind::RefPhandle && other.offset == marker.offset
                    }) {
                        let name = reference.reference.as_deref().unwrap_or_default();
                        output.push(b'&');
                        if name.starts_with(b"/") {
                            output.push(b'{');
                        }
                        output.extend_from_slice(name);
                        if name.starts_with(b"/") {
                            output.push(b'}');
                        }
                        if chunk.len() < 4 {
                            return Err(b"Truncated phandle property marker".to_vec());
                        }
                        if chunk.len() > 4 {
                            output.push(b' ');
                            write_integers(output, &chunk[4..], 4)?;
                        }
                    } else {
                        write_integers(output, chunk, 4)?;
                    }
                    if data_end > end {
                        output.push(b' ');
                    }
                }
                MarkerKind::String => write_string(output, chunk)?,
                kind => write_integers(output, chunk, kind.bits().unwrap_or(8) / 8)?,
            }
            if end == data_end {
                output.extend_from_slice(delimiters(emit).1);
                if end != length {
                    output.push(b',');
                }
                emit = MarkerKind::None;
            }
        }
    }
    output.push(b';');
    annotation(output, &property.srcpos, tree, options, true);
    output.push(b'\n');
    Ok(())
}

fn labels(output: &mut Vec<u8>, labels: &[Label]) {
    for label in labels.iter().filter(|label| !label.deleted) {
        output.extend_from_slice(&label.name);
        output.extend_from_slice(b": ");
    }
}

fn write_node(
    output: &mut Vec<u8>,
    tree: &DtInfo,
    id: NodeId,
    level: usize,
    options: &Options,
) -> Result<(), Vec<u8>> {
    let root = id;
    let mut pending = vec![(id, level, false)];
    while let Some((id, level, end)) = pending.pop() {
        let node = &tree.nodes[id];
        if end {
            output.extend(std::iter::repeat(b'\t').take(level));
            output.extend_from_slice(b"};");
            annotation(output, &node.srcpos, tree, options, false);
            output.push(b'\n');
            continue;
        }
        if id != root {
            output.push(b'\n');
        }
        output.extend(std::iter::repeat(b'\t').take(level));
        labels(output, &node.labels);
        output.extend_from_slice(if node.name.is_empty() {
            b"/"
        } else {
            &node.name
        });
        output.extend_from_slice(b" {");
        annotation(output, &node.srcpos, tree, options, true);
        output.push(b'\n');
        for property in node.properties.iter().filter(|property| !property.deleted) {
            output.extend(std::iter::repeat(b'\t').take(level + 1));
            labels(output, &property.labels);
            output.extend_from_slice(&property.name);
            write_value(output, property, tree, options)?;
        }
        pending.push((id, level, true));
        for &child in node.children.iter().rev() {
            if !tree.nodes[child].deleted {
                pending.push((child, level + 1, false));
            }
        }
    }
    Ok(())
}

pub(crate) fn to_source(
    tree: &DtInfo,
    options: &Options,
    _diagnostics: &mut Diagnostics,
) -> Result<Vec<u8>, Vec<u8>> {
    let mut output = b"/dts-v1/;\n".to_vec();
    if tree.dtsflags & DTSF_PLUGIN != 0 {
        output.extend_from_slice(b"/plugin/;\n");
    }
    output.push(b'\n');
    for reserve in &tree.reserves {
        labels(&mut output, &reserve.labels);
        output.extend_from_slice(
            format!(
                "/memreserve/\t0x{:016x} 0x{:016x};\n",
                reserve.address, reserve.size
            )
            .as_bytes(),
        );
    }
    write_node(&mut output, tree, tree.root, 0, options)?;
    Ok(output)
}
