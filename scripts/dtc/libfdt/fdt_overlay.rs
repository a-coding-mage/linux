// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Overlay relocation, external/local fixups, merge, and symbol propagation.
// Copyright (C) 2016 Free Electrons
// Copyright (C) 2016 NextThing Co.
use super::*;

pub(crate) fn overlay_target_offset<'a>(
    base: &[u8],
    overlay: &'a [u8],
    fragment: i32,
) -> Result<(i32, Option<&'a [u8]>)> {
    let phandle = match getprop(overlay, fragment, b"target") {
        Ok(value) => {
            if value.len() != 4 {
                return Err(Error::BadPhandle);
            }
            let value = u32_at(value, 0)?;
            if value == u32::MAX {
                return Err(Error::BadPhandle);
            }
            value
        }
        Err(_) => 0,
    };
    if phandle != 0 {
        return Ok((node_offset_by_phandle(base, phandle)?, None));
    }
    let value = getprop(overlay, fragment, b"target-path").map_err(|err| {
        if err == Error::NotFound {
            Error::BadOverlay
        } else {
            err
        }
    })?;
    let path = cstr(value).map_err(|_| Error::BadOverlay)?;
    Ok((path_offset(base, path)?, Some(path)))
}
fn adjust_phandles(overlay: &mut [u8], delta: u32) -> Result<()> {
    let mut stack = vec![0];
    while let Some(node) = stack.pop() {
        for name in [b"phandle".as_slice(), b"linux,phandle"] {
            let old = match getprop(overlay, node, name) {
                Ok(v) => {
                    if v.len() != 4 {
                        return Err(Error::BadPhandle);
                    }
                    u32_at(v, 0)?
                }
                Err(Error::NotFound) => continue,
                Err(err) => return Err(err),
            };
            let new = old
                .checked_add(delta)
                .filter(|&n| n != u32::MAX)
                .ok_or(Error::NoPhandles)?;
            setprop_inplace(overlay, node, name, &new.to_be_bytes())?;
        }
        stack.extend(subnodes(overlay, node).into_iter().rev());
    }
    Ok(())
}
fn map_notfound(err: Error) -> Error {
    if err == Error::NotFound {
        Error::BadOverlay
    } else {
        err
    }
}
fn update_references(
    overlay: &mut [u8],
    fixups: i32,
    operation: impl Fn(u32) -> u32,
) -> Result<()> {
    enum Work {
        Node(i32, i32),
        Child(i32, i32),
    }
    let mut stack = vec![Work::Node(0, fixups)];
    while let Some(work) = stack.pop() {
        let (node, fixup) = match work {
            Work::Node(node, fixup) => (node, fixup),
            Work::Child(parent, fixup) => {
                let name = get_name(overlay, fixup)?;
                (
                    subnode_offset(overlay, parent, name).map_err(map_notfound)?,
                    fixup,
                )
            }
        };
        for off in property_offsets(overlay, fixup) {
            let (name, offsets) = getprop_by_offset(overlay, off)?;
            if offsets.len() % 4 != 0 {
                return Err(Error::BadOverlay);
            }
            let name = name.to_vec();
            let offsets = offsets.to_vec();
            let prop = property(overlay, node, &name).map_err(map_notfound)?;
            let at = prop.data_offset;
            let len = prop.data.len();
            for value in offsets.chunks_exact(4) {
                let offset = u32_at(value, 0)? as usize;
                if offset.checked_add(4).is_none_or(|end| end > len) {
                    return Err(Error::BadOverlay);
                }
                let new = operation(u32_at(overlay, at + offset)?);
                put32(overlay, at + offset, new)?;
            }
        }
        for child in subnodes(overlay, fixup).into_iter().rev() {
            stack.push(Work::Child(node, child));
        }
    }
    Ok(())
}
fn local_fixups(overlay: &mut [u8], operation: impl Fn(u32) -> u32) -> Result<()> {
    match path_offset(overlay, b"/__local_fixups__") {
        Ok(fixups) => update_references(overlay, fixups, operation),
        Err(Error::NotFound) => Ok(()),
        Err(err) => Err(err),
    }
}
fn decimal_offset(value: &[u8]) -> Result<usize> {
    let mut at = 0;
    while value
        .get(at)
        .is_some_and(|&b| b == b' ' || (b'\t'..=b'\r').contains(&b))
    {
        at += 1;
    }
    let negative = value.get(at) == Some(&b'-');
    if value.get(at).is_some_and(|b| *b == b'-' || *b == b'+') {
        at += 1;
    }
    let start = at;
    let mut out = 0u64;
    while value.get(at).is_some_and(u8::is_ascii_digit) {
        out = out
            .saturating_mul(10)
            .saturating_add((value[at] - b'0') as u64);
        at += 1;
    }
    if at == start || at != value.len() {
        return Err(Error::BadOverlay);
    }
    if negative {
        out = out.wrapping_neg();
    }
    // C strtoul is converted to int, then the partial-write API's uint32_t.
    Ok(out as u32 as usize)
}
fn external_fixups(base: &[u8], overlay: &mut [u8]) -> Result<()> {
    let fixups = match path_offset(overlay, b"/__fixups__") {
        Ok(node) => node,
        Err(Error::NotFound) => return Ok(()),
        Err(err) => return Err(err),
    };
    let symbols = match path_offset(base, b"/__symbols__") {
        Ok(node) => node,
        Err(Error::NotFound) => Error::NotFound.code(),
        Err(err) => return Err(err),
    };
    for prop in property_offsets(overlay, fixups) {
        let (label, values) = getprop_by_offset(overlay, prop).map_err(|e| {
            if e == Error::NotFound {
                Error::Internal
            } else {
                e
            }
        })?;
        // C passes a missing symbols offset to getprop, yielding BADOFFSET.
        let symbol_path = cstr(getprop(base, symbols, label)?).map_err(|_| Error::BadOverlay)?;
        let node = path_offset(base, symbol_path)?;
        let phandle = get_phandle(base, node);
        if phandle == 0 {
            return Err(Error::NotFound);
        }
        let values = values.to_vec();
        let mut rest = values.as_slice();
        loop {
            let fixup = cstr(rest).map_err(|_| Error::BadOverlay)?;
            let a = fixup
                .iter()
                .position(|&c| c == b':')
                .ok_or(Error::BadOverlay)?;
            if a == fixup.len() - 1 {
                return Err(Error::BadOverlay);
            }
            let b = fixup[a + 1..]
                .iter()
                .position(|&c| c == b':')
                .ok_or(Error::BadOverlay)?
                + a
                + 1;
            if b == a + 1 {
                return Err(Error::BadOverlay);
            }
            let index = decimal_offset(&fixup[b + 1..])?;
            let node = path_offset(overlay, &fixup[..a]).map_err(map_notfound)?;
            setprop_inplace_partial(
                overlay,
                node,
                &fixup[a + 1..b],
                index,
                &phandle.to_be_bytes(),
            )?;
            rest = &rest[fixup.len() + 1..];
            if rest.is_empty() {
                break;
            }
        }
    }
    Ok(())
}
fn prevent_overwrite(base: &[u8], overlay: &mut [u8]) -> Result<()> {
    enum Work {
        Node(i32, i32),
        Child(i32, i32),
    }
    for fragment in subnodes(overlay, 0) {
        let root = match subnode_offset(overlay, fragment, b"__overlay__") {
            Ok(node) => node,
            Err(Error::NotFound) => continue,
            Err(err) => return Err(err),
        };
        let target = match overlay_target_offset(base, overlay, fragment) {
            Ok((node, _)) => node,
            Err(Error::NotFound) => continue,
            Err(err) => return Err(err),
        };
        let mut stack = vec![Work::Node(target, root)];
        while let Some(work) = stack.pop() {
            let (target, node) = match work {
                Work::Node(target, node) => (target, node),
                Work::Child(parent, node) => {
                    let name = get_name(overlay, node)?;
                    match subnode_offset(base, parent, name) {
                        Ok(target) => (target, node),
                        Err(Error::NotFound) => continue,
                        Err(err) => return Err(err),
                    }
                }
            };
            let base_phandle = get_phandle(base, target);
            let overlay_phandle = get_phandle(overlay, node);
            if base_phandle != 0 && overlay_phandle != 0 {
                for name in [b"phandle".as_slice(), b"linux,phandle"] {
                    if getprop(overlay, node, name).is_ok_and(|v| v.len() == 4) {
                        setprop_inplace(overlay, node, name, &base_phandle.to_be_bytes())?;
                    }
                }
                local_fixups(overlay, |value| {
                    if value == overlay_phandle {
                        base_phandle
                    } else {
                        value
                    }
                })?;
            }
            for child in subnodes(overlay, node).into_iter().rev() {
                stack.push(Work::Child(target, child));
            }
        }
    }
    Ok(())
}
fn apply_node(base: &mut [u8], target: i32, overlay: &[u8], node: i32) -> Result<()> {
    enum Work {
        Merge(i32, i32),
        Child(i32, i32),
    }
    let mut stack = vec![Work::Merge(target, node)];
    while let Some(work) = stack.pop() {
        let (target, node) = match work {
            Work::Merge(target, node) => (target, node),
            Work::Child(parent, node) => {
                let name = get_name(overlay, node)?;
                let target = match add_subnode(base, parent, name) {
                    Ok(node) => node,
                    Err(Error::Exists) => subnode_offset(base, parent, name).map_err(|e| {
                        if e == Error::NotFound {
                            Error::Internal
                        } else {
                            e
                        }
                    })?,
                    Err(err) => return Err(err),
                };
                (target, node)
            }
        };
        for off in property_offsets(overlay, node) {
            let (name, value) = getprop_by_offset(overlay, off).map_err(|e| {
                if e == Error::NotFound {
                    Error::Internal
                } else {
                    e
                }
            })?;
            setprop(base, target, name, value)?;
        }
        for child in subnodes(overlay, node).into_iter().rev() {
            stack.push(Work::Child(target, child));
        }
    }
    Ok(())
}
fn merge(base: &mut [u8], overlay: &[u8]) -> Result<()> {
    for fragment in subnodes(overlay, 0) {
        let node = match subnode_offset(overlay, fragment, b"__overlay__") {
            Ok(node) => node,
            Err(Error::NotFound) => continue,
            Err(err) => return Err(err),
        };
        let target = overlay_target_offset(base, overlay, fragment)?.0;
        apply_node(base, target, overlay, node)?;
    }
    Ok(())
}
fn symbol_update(base: &mut [u8], overlay: &[u8]) -> Result<()> {
    let Ok(symbols) = subnode_offset(overlay, 0, b"__symbols__") else {
        return Ok(());
    };
    let root = match subnode_offset(base, 0, b"__symbols__") {
        Ok(node) => node,
        Err(Error::NotFound) => add_subnode(base, 0, b"__symbols__")?,
        Err(err) => return Err(err),
    };
    for off in property_offsets(overlay, symbols) {
        let (name, value) = getprop_by_offset(overlay, off)?;
        let path = cstr(value).map_err(|_| Error::BadValue)?;
        if path.len() + 1 != value.len() || path.first() != Some(&b'/') {
            return Err(Error::BadValue);
        }
        let Some(split) = path[1..].iter().position(|&c| c == b'/').map(|n| n + 1) else {
            continue;
        };
        let suffix = &path[split..];
        let relative = if let Some(path) = suffix.strip_prefix(b"/__overlay__/") {
            path
        } else if suffix == b"/__overlay__" {
            b""
        } else {
            continue;
        };
        let fragment =
            subnode_offset(overlay, 0, &path[1..split]).map_err(|_| Error::BadOverlay)?;
        subnode_offset(overlay, fragment, b"__overlay__").map_err(|_| Error::BadOverlay)?;
        let (target, target_path) = overlay_target_offset(base, overlay, fragment)?;
        let path = if let Some(path) = target_path {
            path.to_vec()
        } else {
            get_path(base, target)?
        };
        let len = path.len() + (path.len() > 1) as usize + relative.len() + 1;
        // Allocate first: this preserves padding and failure-side effects.
        setprop_placeholder(base, root, name, len)?;
        let path = if target_path.is_some() {
            path
        } else {
            get_path(base, overlay_target_offset(base, overlay, fragment)?.0)?
        };
        let mut value = Vec::new();
        if path.len() > 1 {
            value.extend_from_slice(&path);
        }
        value.push(b'/');
        value.extend_from_slice(relative);
        value.push(0);
        setprop_inplace_partial(base, root, name, 0, &value)?;
    }
    Ok(())
}
pub(crate) fn overlay_apply(base: &mut [u8], overlay: &mut [u8]) -> Result<()> {
    ro_probe(base)?;
    ro_probe(overlay)?;
    let result = (|| {
        let delta = find_max_phandle(base)?;
        adjust_phandles(overlay, delta)?;
        local_fixups(overlay, |value| value.wrapping_add(delta))?;
        external_fixups(base, overlay)?;
        prevent_overwrite(base, overlay)?;
        merge(base, overlay)?;
        symbol_update(base, overlay)
    })();
    put32(overlay, 0, u32::MAX)?;
    if result.is_err() {
        put32(base, 0, u32::MAX)?;
    }
    result
}
