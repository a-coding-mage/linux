// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Borrowed property/name access and tree searches.
// Copyright (C) 2006 David Gibson, IBM Corporation.
use super::*;

pub(crate) fn get_string(data: &[u8], offset: i32) -> Result<&[u8]> {
    let h = ro_probe(data)?;
    let abs = (h.off_dt_strings as u32).wrapping_add(offset as u32) as usize;
    if abs >= h.totalsize {
        return Err(Error::BadOffset);
    }
    let mut len = h.totalsize - abs;
    if h.magic == FDT_MAGIC {
        if offset < 0 {
            return Err(Error::BadOffset);
        }
        if h.version >= 17 {
            if offset as usize >= h.size_dt_strings {
                return Err(Error::BadOffset);
            }
            len = len.min(h.size_dt_strings - offset as usize);
        }
    } else {
        if offset >= 0 || offset.unsigned_abs() as usize > h.size_dt_strings {
            return Err(Error::BadOffset);
        }
        len = len.min(offset.unsigned_abs() as usize);
    }
    cstr(bytes(data, abs, len)?)
}
pub(crate) fn get_mem_rsv(data: &[u8], n: i32) -> Result<(u64, u64)> {
    let h = ro_probe(data)?;
    let n = usize::try_from(n).map_err(|_| Error::BadOffset)?;
    let at = h
        .off_mem_rsvmap
        .checked_add(n.checked_mul(16).ok_or(Error::BadOffset)?)
        .ok_or(Error::BadOffset)?;
    if at.checked_add(16).is_none_or(|end| end > h.totalsize) {
        return Err(Error::BadOffset);
    }
    Ok((u64_at(data, at)?, u64_at(data, at + 8)?))
}
pub(crate) fn num_mem_rsv(data: &[u8]) -> Result<i32> {
    // This accessor does not perform the read-only state probe in libfdt.
    let h = Header::read(data)?;
    let mut n = 0;
    let mut at = h.off_mem_rsvmap;
    while at
        .checked_add(16)
        .is_some_and(|end| end <= h.totalsize && end <= data.len())
    {
        if u64_at(data, at + 8)? == 0 {
            return Ok(n);
        }
        at += 16;
        n += 1;
    }
    Err(Error::Truncated)
}
pub(crate) fn get_name(data: &[u8], node: i32) -> Result<&[u8]> {
    let h = ro_probe(data)?;
    let next = check_node_offset(data, node)?;
    let name = cstr(offset_ptr(data, node + 4, (next - node - 4) as usize)?)?;
    if h.version < 16 {
        let leaf = name
            .iter()
            .rposition(|&c| c == b'/')
            .ok_or(Error::BadStructure)?;
        Ok(&name[leaf + 1..])
    } else {
        Ok(name)
    }
}
pub(crate) fn subnode_offset(data: &[u8], mut parent: i32, name: &[u8]) -> Result<i32> {
    ro_probe(data)?;
    if parent < 0 {
        return Err(Error::from_code(parent));
    }
    let mut depth = 0;
    loop {
        if depth == 1 {
            if let Ok(found) = get_name(data, parent) {
                if found.starts_with(name)
                    && (found.len() == name.len()
                        || (!name.contains(&b'@') && found.get(name.len()) == Some(&b'@')))
                {
                    return Ok(parent);
                }
            }
        }
        parent = next_node(data, parent, Some(&mut depth))?;
        if depth < 0 {
            return Err(Error::NotFound);
        }
    }
}
pub(crate) fn path_offset(data: &[u8], path: &[u8]) -> Result<i32> {
    path_offset_inner(data, path, 0)
}
fn path_offset_inner(data: &[u8], mut path: &[u8], aliases: usize) -> Result<i32> {
    ro_probe(data)?;
    if path.is_empty() {
        return Err(Error::BadPath);
    }
    // Cyclic aliases are invalid; bound recursion for hostile blobs.
    if aliases > data.len() / 4 {
        return Err(Error::BadPath);
    }
    let mut offset = 0;
    if path[0] != b'/' {
        let end = path.iter().position(|&c| c == b'/').unwrap_or(path.len());
        let alias = get_alias(data, &path[..end]).ok_or(Error::BadPath)?;
        offset = path_offset_inner(data, alias, aliases + 1)?;
        path = &path[end..];
    }
    for part in path.split(|&c| c == b'/').filter(|s| !s.is_empty()) {
        offset = subnode_offset(data, offset, part)?;
    }
    Ok(offset)
}
fn nextprop(data: &[u8], mut at: i32) -> Result<i32> {
    loop {
        let (tag, next) = next_tag(data, at);
        match tag {
            FDT_PROP => return Ok(at),
            FDT_NOP => at = next?,
            FDT_END => return Err(next.err().unwrap_or(Error::BadStructure)),
            _ => return Err(Error::NotFound),
        }
    }
}
pub(crate) fn first_property_offset(data: &[u8], node: i32) -> Result<i32> {
    nextprop(data, check_node_offset(data, node)?)
}
pub(crate) fn next_property_offset(data: &[u8], offset: i32) -> Result<i32> {
    nextprop(data, check_prop_offset(data, offset)?)
}
pub(super) fn property_by_offset(data: &[u8], offset: i32) -> Result<Property<'_>> {
    check_prop_offset(data, offset)?;
    let header = offset_ptr(data, offset, 12)?;
    let len = u32_at(header, 4)? as usize;
    let mut at = offset.checked_add(12).ok_or(Error::BadOffset)?;
    if Header::read(data)?.version < 16 && len >= 8 && at % 8 != 0 {
        at += 4;
    }
    let range = struct_range(data, at, len)?;
    Ok(Property {
        offset,
        name_offset: u32_at(header, 8)? as i32,
        data_offset: range.start,
        data: &data[range],
    })
}
pub(crate) fn get_property_by_offset(data: &[u8], offset: i32) -> Result<Property<'_>> {
    if Header::read(data)?.version < 16 {
        return Err(Error::BadVersion);
    }
    property_by_offset(data, offset)
}
pub(super) fn property<'a>(data: &'a [u8], node: i32, name: &[u8]) -> Result<Property<'a>> {
    let mut at = first_property_offset(data, node)?;
    loop {
        let prop = property_by_offset(data, at).map_err(|_| Error::Internal)?;
        if get_string(data, prop.name_offset).is_ok_and(|s| s == name) {
            return Ok(prop);
        }
        at = next_property_offset(data, at)?;
    }
}
pub(crate) fn get_property<'a>(data: &'a [u8], node: i32, name: &[u8]) -> Result<Property<'a>> {
    if Header::read(data)?.version < 16 {
        return Err(Error::BadVersion);
    }
    property(data, node, name)
}
pub(crate) fn getprop<'a>(data: &'a [u8], node: i32, name: &[u8]) -> Result<&'a [u8]> {
    Ok(property(data, node, name)?.data)
}
pub(crate) fn getprop_by_offset(data: &[u8], offset: i32) -> Result<(&[u8], &[u8])> {
    let prop = property_by_offset(data, offset)?;
    Ok((get_string(data, prop.name_offset)?, prop.data))
}
pub(crate) fn get_phandle(data: &[u8], node: i32) -> u32 {
    for name in [b"phandle".as_slice(), b"linux,phandle"] {
        if let Ok(value) = getprop(data, node, name) {
            if value.len() == 4 {
                return u32::from_be_bytes(value.try_into().unwrap());
            }
        }
    }
    0
}
pub(crate) fn get_alias<'a>(data: &'a [u8], name: &[u8]) -> Option<&'a [u8]> {
    let node = path_offset(data, b"/aliases").ok()?;
    let value = getprop(data, node, name).ok()?;
    if value.first() != Some(&b'/') || value.last() != Some(&0) {
        return None;
    }
    cstr(value).ok()
}
pub(crate) fn get_symbol<'a>(data: &'a [u8], name: &[u8]) -> Option<&'a [u8]> {
    getprop(data, path_offset(data, b"/__symbols__").ok()?, name).ok()
}
pub(crate) fn find_max_phandle(data: &[u8]) -> Result<u32> {
    let mut at = -1;
    let mut max = 0;
    loop {
        match next_node(data, at, None) {
            Ok(next) => {
                at = next;
                max = max.max(get_phandle(data, at));
            }
            Err(Error::NotFound) => return Ok(max),
            Err(err) => return Err(err),
        }
    }
}
pub(crate) fn generate_phandle(data: &[u8]) -> Result<u32> {
    let max = find_max_phandle(data)?;
    if max == MAX_PHANDLE {
        Err(Error::NoPhandles)
    } else {
        Ok(max.wrapping_add(1))
    }
}
pub(crate) fn supernode_atdepth_offset(data: &[u8], node: i32, wanted: i32) -> Result<(i32, i32)> {
    ro_probe(data)?;
    if wanted < 0 {
        return Err(Error::NotFound);
    }
    let mut at = 0;
    let mut depth = 0;
    let mut parent = Err(Error::Internal);
    loop {
        if at > node {
            return Err(Error::BadOffset);
        }
        if depth == wanted {
            parent = Ok(at);
        }
        if at == node {
            if wanted > depth {
                return Err(Error::NotFound);
            }
            return Ok((parent?, depth));
        }
        at = next_node(data, at, Some(&mut depth)).map_err(|err| match err {
            Error::NotFound => Error::BadOffset,
            Error::BadOffset => Error::BadStructure,
            err => err,
        })?;
    }
}
pub(crate) fn node_depth(data: &[u8], node: i32) -> Result<i32> {
    let (root, depth) = supernode_atdepth_offset(data, node, 0)?;
    if root != 0 {
        return Err(Error::Internal);
    }
    Ok(depth)
}
pub(crate) fn parent_offset(data: &[u8], node: i32) -> Result<i32> {
    Ok(supernode_atdepth_offset(data, node, node_depth(data, node)? - 1)?.0)
}
pub(crate) fn get_path(data: &[u8], node: i32) -> Result<Vec<u8>> {
    ro_probe(data)?;
    let mut at = 0;
    let mut depth = 0;
    let mut names = Vec::new();
    loop {
        if at > node {
            return Err(Error::BadOffset);
        }
        let name = get_name(data, at)?;
        if depth < 0 {
            return Err(Error::BadStructure);
        }
        names.truncate(depth as usize);
        names.push(name);
        if at == node {
            let mut path = vec![b'/'];
            for (i, name) in names.iter().skip(1).enumerate() {
                if i != 0 {
                    path.push(b'/');
                }
                path.extend_from_slice(name);
            }
            return Ok(path);
        }
        at = next_node(data, at, Some(&mut depth)).map_err(|err| match err {
            Error::NotFound => Error::BadOffset,
            Error::BadOffset => Error::BadStructure,
            err => err,
        })?;
    }
}
pub(crate) fn get_path_into(data: &[u8], node: i32, buf: &mut [u8]) -> Result<()> {
    ro_probe(data)?;
    if buf.len() < 2 {
        return Err(Error::NoSpace);
    }
    let path = get_path(data, node)?;
    if path.len() >= buf.len() {
        return Err(Error::NoSpace);
    }
    buf[..path.len()].copy_from_slice(&path);
    buf[path.len()] = 0;
    Ok(())
}
pub(crate) fn node_offset_by_prop_value(
    data: &[u8],
    mut at: i32,
    name: &[u8],
    value: &[u8],
) -> Result<i32> {
    ro_probe(data)?;
    loop {
        at = next_node(data, at, None)?;
        if getprop(data, at, name).is_ok_and(|v| v == value) {
            return Ok(at);
        }
    }
}
pub(crate) fn node_offset_by_phandle(data: &[u8], phandle: u32) -> Result<i32> {
    if phandle == 0 || phandle == u32::MAX {
        return Err(Error::BadPhandle);
    }
    ro_probe(data)?;
    let mut at = -1;
    loop {
        at = next_node(data, at, None)?;
        if get_phandle(data, at) == phandle {
            return Ok(at);
        }
    }
}
pub(crate) fn stringlist_contains(mut list: &[u8], string: &[u8]) -> bool {
    while let Ok(value) = cstr(list) {
        if value == string {
            return true;
        }
        list = &list[value.len() + 1..];
    }
    false
}
pub(crate) fn stringlist_count(data: &[u8], node: i32, name: &[u8]) -> Result<i32> {
    let mut list = getprop(data, node, name)?;
    let mut count = 0;
    while !list.is_empty() {
        let len = cstr(list).map_err(|_| Error::BadValue)?.len() + 1;
        list = &list[len..];
        count += 1;
    }
    Ok(count)
}
pub(crate) fn stringlist_search(data: &[u8], node: i32, name: &[u8], string: &[u8]) -> Result<i32> {
    let mut list = getprop(data, node, name)?;
    let mut index = 0;
    while !list.is_empty() {
        let value = cstr(list).map_err(|_| Error::BadValue)?;
        if value == string {
            return Ok(index);
        }
        list = &list[value.len() + 1..];
        index += 1;
    }
    Err(Error::NotFound)
}
pub(crate) fn stringlist_get<'a>(
    data: &'a [u8],
    node: i32,
    name: &[u8],
    mut index: i32,
) -> Result<&'a [u8]> {
    let mut list = getprop(data, node, name)?;
    while !list.is_empty() {
        let value = cstr(list).map_err(|_| Error::BadValue)?;
        if index == 0 {
            return Ok(value);
        }
        list = &list[value.len() + 1..];
        index -= 1;
    }
    Err(Error::NotFound)
}
pub(crate) fn node_check_compatible(data: &[u8], node: i32, compatible: &[u8]) -> Result<bool> {
    Ok(stringlist_contains(
        getprop(data, node, b"compatible")?,
        compatible,
    ))
}
pub(crate) fn node_offset_by_compatible(
    data: &[u8],
    mut at: i32,
    compatible: &[u8],
) -> Result<i32> {
    ro_probe(data)?;
    loop {
        at = next_node(data, at, None)?;
        match node_check_compatible(data, at, compatible) {
            Ok(true) => return Ok(at),
            Ok(false) | Err(Error::NotFound) => {}
            Err(err) => return Err(err),
        }
    }
}
// The iteration helpers preserve the C for_each_* convention: traversal ends
// at the first negative result. Individual offset accessors expose errors.
pub(crate) fn subnodes(data: &[u8], parent: i32) -> Vec<i32> {
    let mut out = Vec::new();
    let mut at = first_subnode(data, parent);
    while let Ok(node) = at {
        out.push(node);
        at = next_subnode(data, node);
    }
    out
}
pub(crate) fn property_offsets(data: &[u8], node: i32) -> Vec<i32> {
    let mut out = Vec::new();
    let mut at = first_property_offset(data, node);
    while let Ok(prop) = at {
        out.push(prop);
        at = next_property_offset(data, prop);
    }
    out
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
