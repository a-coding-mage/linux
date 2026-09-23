// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! In-place, layout-preserving device-tree mutation.
// Copyright (C) 2006 David Gibson, IBM Corporation.
use super::*;

fn blocks_misordered(h: Header, reserve: usize, structure: usize) -> bool {
    h.off_mem_rsvmap < 40
        || h.off_mem_rsvmap
            .checked_add(reserve)
            .is_none_or(|end| h.off_dt_struct < end)
        || h.off_dt_struct
            .checked_add(structure)
            .is_none_or(|end| h.off_dt_strings < end)
        || h.off_dt_strings
            .checked_add(h.size_dt_strings)
            .is_none_or(|end| h.totalsize < end)
}
fn rw_probe(data: &mut [u8]) -> Result<Header> {
    let mut h = ro_probe(data)?;
    if h.version < 17 {
        return Err(Error::BadVersion);
    }
    if blocks_misordered(h, 16, h.size_dt_struct) {
        return Err(Error::BadLayout);
    }
    if h.version > 17 {
        put32(data, 20, 17)?;
        h.version = 17;
    }
    Ok(h)
}
fn splice(data: &mut [u8], at: usize, old: usize, new: usize) -> Result<()> {
    let h = Header::read(data)?;
    let size = h
        .off_dt_strings
        .checked_add(h.size_dt_strings)
        .ok_or(Error::BadOffset)?;
    let end = at.checked_add(old).ok_or(Error::BadOffset)?;
    if end > size {
        return Err(Error::BadOffset);
    }
    let newsize = size
        .checked_sub(old)
        .and_then(|v| v.checked_add(new))
        .ok_or(Error::BadOffset)?;
    if newsize > h.totalsize {
        return Err(Error::NoSpace);
    }
    bytes(data, 0, size.max(newsize))?;
    data.copy_within(end..size, at + new);
    Ok(())
}
fn splice_struct(data: &mut [u8], at: usize, old: usize, new: usize) -> Result<()> {
    let h = Header::read(data)?;
    splice(data, at, old, new)?;
    put32(data, 36, (h.size_dt_struct - old + new) as u32)?;
    put32(data, 12, (h.off_dt_strings - old + new) as u32)
}
fn splice_reserve(data: &mut [u8], at: usize, old: usize, new: usize) -> Result<()> {
    let h = Header::read(data)?;
    splice(data, at, old, new)?;
    put32(data, 8, (h.off_dt_struct - old + new) as u32)?;
    put32(data, 12, (h.off_dt_strings - old + new) as u32)
}
fn add_string(data: &mut [u8], name: &[u8]) -> Result<(usize, bool)> {
    let h = Header::read(data)?;
    let table = bytes(data, h.off_dt_strings, h.size_dt_strings)?;
    if let Some(index) = find_string(table, name) {
        return Ok((index, false));
    }
    let at = h.off_dt_strings + h.size_dt_strings;
    splice(data, at, 0, name.len() + 1)?;
    put32(data, 32, (h.size_dt_strings + name.len() + 1) as u32)?;
    put(data, at, name)?;
    data[at + name.len()] = 0;
    Ok((h.size_dt_strings, true))
}
fn add_property(data: &mut [u8], node: i32, name: &[u8], len: usize) -> Result<usize> {
    let next = check_node_offset(data, node)?;
    let (nameoff, allocated) = add_string(data, name)?;
    let at = struct_abs(data, next, 0)?;
    let size = 12usize.checked_add(align(len, 4)?).ok_or(Error::NoSpace)?;
    if let Err(err) = splice_struct(data, at, 0, size) {
        if allocated {
            let h = Header::read(data)?;
            put32(data, 32, (h.size_dt_strings - name.len() - 1) as u32)?;
        }
        return Err(err);
    }
    put32(data, at, FDT_PROP)?;
    put32(data, at + 4, len as u32)?;
    put32(data, at + 8, nameoff as u32)?;
    Ok(at + 12)
}
pub(crate) fn add_mem_rsv(data: &mut [u8], address: u64, size: u64) -> Result<()> {
    let h = rw_probe(data)?;
    let at = h.off_mem_rsvmap + num_mem_rsv(data)? as usize * 16;
    splice_reserve(data, at, 0, 16)?;
    put64(data, at, address)?;
    put64(data, at + 8, size)
}
pub(crate) fn del_mem_rsv(data: &mut [u8], index: i32) -> Result<()> {
    let h = rw_probe(data)?;
    if index < 0 || index >= num_mem_rsv(data)? {
        return Err(Error::NotFound);
    }
    splice_reserve(data, h.off_mem_rsvmap + index as usize * 16, 16, 0)
}
pub(crate) fn set_name(data: &mut [u8], node: i32, name: &[u8]) -> Result<()> {
    rw_probe(data)?;
    let old = get_name(data, node)?.len();
    let at = struct_abs(data, node + 4, old + 1)?;
    splice_struct(data, at, align(old + 1, 4)?, align(name.len() + 1, 4)?)?;
    put(data, at, name)?;
    data[at + name.len()] = 0;
    Ok(())
}
pub(crate) fn setprop_placeholder<'a>(
    data: &'a mut [u8],
    node: i32,
    name: &[u8],
    len: usize,
) -> Result<&'a mut [u8]> {
    rw_probe(data)?;
    if len > i32::MAX as usize {
        return Err(Error::NoSpace);
    }
    let at = match get_property(data, node, name) {
        Ok(prop) => {
            let at = prop.data_offset;
            let old = prop.data.len();
            splice_struct(data, at, align(old, 4)?, align(len, 4)?)?;
            put32(data, at - 8, len as u32)?;
            at
        }
        Err(Error::NotFound) => add_property(data, node, name, len)?,
        Err(err) => return Err(err),
    };
    Ok(&mut data[at..at + len])
}
pub(crate) fn setprop(data: &mut [u8], node: i32, name: &[u8], value: &[u8]) -> Result<()> {
    setprop_placeholder(data, node, name, value.len())?.copy_from_slice(value);
    Ok(())
}
pub(crate) fn appendprop(data: &mut [u8], node: i32, name: &[u8], value: &[u8]) -> Result<()> {
    rw_probe(data)?;
    match get_property(data, node, name) {
        Ok(prop) => {
            let at = prop.data_offset;
            let old = prop.data.len();
            let len = old.checked_add(value.len()).ok_or(Error::NoSpace)?;
            splice_struct(data, at, align(old, 4)?, align(len, 4)?)?;
            put32(data, at - 8, len as u32)?;
            put(data, at + old, value)
        }
        Err(_) => {
            let at = add_property(data, node, name, value.len())?;
            put(data, at, value)
        }
    }
}
pub(crate) fn delprop(data: &mut [u8], node: i32, name: &[u8]) -> Result<()> {
    rw_probe(data)?;
    let prop = get_property(data, node, name)?;
    let at = prop.data_offset - 12;
    let len = 12 + align(prop.data.len(), 4)?;
    splice_struct(data, at, len, 0)
}
pub(crate) fn add_subnode(data: &mut [u8], parent: i32, name: &[u8]) -> Result<i32> {
    rw_probe(data)?;
    match subnode_offset(data, parent, name) {
        Ok(_) => return Err(Error::Exists),
        Err(Error::NotFound) => {}
        Err(err) => return Err(err),
    }
    let mut next = check_node_offset(data, parent)?;
    let offset = loop {
        let at = next;
        let (tag, result) = next_tag(data, at);
        if tag != FDT_PROP && tag != FDT_NOP {
            break at;
        }
        next = result?;
    };
    let at = struct_abs(data, offset, 0)?;
    let namelen = align(name.len() + 1, 4)?;
    let len = 8 + namelen;
    splice_struct(data, at, 0, len)?;
    put32(data, at, FDT_BEGIN_NODE)?;
    data[at + 4..at + 4 + namelen].fill(0);
    put(data, at + 4, name)?;
    put32(data, at + 4 + namelen, FDT_END_NODE)?;
    Ok(offset)
}
pub(crate) fn del_node(data: &mut [u8], node: i32) -> Result<()> {
    rw_probe(data)?;
    let end = node_end_offset(data, node)?;
    splice_struct(data, struct_abs(data, node, 0)?, (end - node) as usize, 0)
}
fn packblocks(data: &[u8], dest: &mut [u8], reserve: usize, structure: usize) -> Result<()> {
    let h = Header::read(data)?;
    let structoff = 40 + reserve;
    let stringoff = structoff + structure;
    put(dest, 40, bytes(data, h.off_mem_rsvmap, reserve)?)?;
    put32(dest, 16, 40)?;
    put(dest, structoff, bytes(data, h.off_dt_struct, structure)?)?;
    put32(dest, 8, structoff as u32)?;
    put32(dest, 36, structure as u32)?;
    put(
        dest,
        stringoff,
        bytes(data, h.off_dt_strings, h.size_dt_strings)?,
    )?;
    put32(dest, 12, stringoff as u32)?;
    put32(dest, 32, h.size_dt_strings as u32)
}
pub(crate) fn open_into(data: &[u8], dest: &mut [u8]) -> Result<()> {
    let h = ro_probe(data)?;
    if dest.len() > i32::MAX as usize {
        return Err(Error::NoSpace);
    }
    let reserve = (num_mem_rsv(data)? as usize + 1) * 16;
    let structure = if h.version >= 17 {
        h.size_dt_struct
    } else if h.version == 16 {
        let mut at = 0;
        loop {
            let (tag, next) = next_tag(data, at);
            at = next?;
            if tag == FDT_END {
                break at as usize;
            }
        }
    } else {
        return Err(Error::BadVersion);
    };
    if !blocks_misordered(h, reserve, structure) {
        move_into(data, dest)?;
        put32(dest, 20, 17)?;
        put32(dest, 36, structure as u32)?;
        return put32(dest, 4, dest.len() as u32);
    }
    let size = 40usize
        .checked_add(reserve)
        .and_then(|n| n.checked_add(structure))
        .and_then(|n| n.checked_add(h.size_dt_strings))
        .ok_or(Error::NoSpace)?;
    if dest.len() < size {
        return Err(Error::NoSpace);
    }
    packblocks(data, dest, reserve, structure)?;
    put32(dest, 0, FDT_MAGIC)?;
    put32(dest, 4, dest.len() as u32)?;
    put32(dest, 20, 17)?;
    put32(dest, 24, 16)?;
    put32(dest, 28, h.boot_cpuid_phys)
}
pub(crate) fn open_inplace(data: &mut [u8]) -> Result<()> {
    let h = ro_probe(data)?;
    let reserve = (num_mem_rsv(data)? as usize + 1) * 16;
    let structure = if h.version >= 17 {
        h.size_dt_struct
    } else if h.version == 16 {
        let mut at = 0;
        loop {
            let (tag, next) = next_tag(data, at);
            at = next?;
            if tag == FDT_END {
                break at as usize;
            }
        }
    } else {
        return Err(Error::BadVersion);
    };
    let old = data.to_vec();
    if !blocks_misordered(h, reserve, structure) {
        return open_into(&old, data);
    }
    let size = 40usize
        .checked_add(reserve)
        .and_then(|v| v.checked_add(structure))
        .and_then(|v| v.checked_add(h.size_dt_strings))
        .ok_or(Error::NoSpace)?;
    let end = h.totalsize.checked_add(size).ok_or(Error::NoSpace)?;
    if end > data.len() {
        return Err(Error::NoSpace);
    }
    // The C in-place API stages reordered blocks after the old blob. Keep
    // its documented scratch-space requirement and byte-level side effects.
    packblocks(&old, &mut data[h.totalsize..], reserve, structure)?;
    data.copy_within(h.totalsize..end, 0);
    put32(data, 0, FDT_MAGIC)?;
    put32(data, 4, data.len() as u32)?;
    put32(data, 20, 17)?;
    put32(data, 24, 16)
}
pub(crate) fn pack(data: &mut [u8]) -> Result<()> {
    let h = rw_probe(data)?;
    let reserve = (num_mem_rsv(data)? as usize + 1) * 16;
    let old = data.to_vec();
    packblocks(&old, data, reserve, h.size_dt_struct)?;
    put32(
        data,
        4,
        (40 + reserve + h.size_dt_struct + h.size_dt_strings) as u32,
    )
}
