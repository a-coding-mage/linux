// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Header validation and structure-tag traversal.
// Copyright (C) 2006 David Gibson, IBM Corporation.
use super::*;

pub(crate) fn header_size(version: u32) -> usize {
    match version {
        0..=1 => FDT_V1_SIZE,
        2 => FDT_V2_SIZE,
        3..=16 => FDT_V3_SIZE,
        _ => FDT_V17_SIZE,
    }
}
pub(crate) fn check_header(data: &[u8]) -> Result<()> {
    if u32_at(data, 0)? != FDT_MAGIC {
        return Err(Error::BadMagic);
    }
    let h = Header::read(data)?;
    if h.version < 2 || h.last_comp_version > 17 || h.version < h.last_comp_version {
        return Err(Error::BadVersion);
    }
    let hdrsize = header_size(h.version);
    if h.totalsize < hdrsize || h.totalsize > i32::MAX as usize || h.totalsize > data.len() {
        return Err(Error::Truncated);
    }
    if h.off_mem_rsvmap % 8 != 0 || h.off_dt_struct % 4 != 0 {
        return Err(Error::Alignment);
    }
    let check = |base: usize, size: usize| {
        base >= hdrsize && base.checked_add(size).is_some_and(|end| end <= h.totalsize)
    };
    if !check(h.off_mem_rsvmap, 0)
        || !check(
            h.off_dt_struct,
            if h.version >= 17 { h.size_dt_struct } else { 0 },
        )
        || !check(h.off_dt_strings, h.size_dt_strings)
    {
        return Err(Error::Truncated);
    }
    Ok(())
}
pub(crate) fn offset_ptr(data: &[u8], offset: i32, len: usize) -> Result<&[u8]> {
    Ok(&data[struct_range(data, offset, len)?])
}
pub(crate) fn next_tag(data: &[u8], offset: i32) -> (u32, Result<i32>) {
    let read = |at| -> Result<u32> {
        Ok(u32::from_be_bytes(
            offset_ptr(data, at, 4)?.try_into().unwrap(),
        ))
    };
    let Ok(tag) = read(offset) else {
        return (FDT_END, Err(Error::Truncated));
    };
    let scan = || -> Result<i32> {
        let mut at = offset.checked_add(4).ok_or(Error::BadStructure)?;
        match tag {
            FDT_BEGIN_NODE => loop {
                let byte = offset_ptr(data, at, 1).map_err(|_| Error::BadStructure)?[0];
                at = at.checked_add(1).ok_or(Error::BadStructure)?;
                if byte == 0 {
                    break;
                }
            },
            FDT_PROP => {
                let len = read(at).map_err(|_| Error::BadStructure)?;
                if u64::from(len) + at as u64 >= i32::MAX as u64 {
                    return Err(Error::BadStructure);
                }
                at = at
                    .checked_add(8)
                    .and_then(|v| v.checked_add(len as i32))
                    .ok_or(Error::BadStructure)?;
                if Header::read(data)?.version < 16 && len >= 8 && (at - len as i32) % 8 != 0 {
                    at = at.checked_add(4).ok_or(Error::BadStructure)?;
                }
            }
            FDT_END | FDT_END_NODE | FDT_NOP => {}
            _ => return Err(Error::BadStructure),
        }
        offset_ptr(data, offset, (at - offset) as usize).map_err(|_| Error::BadStructure)?;
        i32::try_from(align(at as usize, 4)?).map_err(|_| Error::BadStructure)
    };
    match scan() {
        Ok(next) => (tag, Ok(next)),
        Err(err) => (FDT_END, Err(err)),
    }
}
pub(crate) fn next_node(data: &[u8], offset: i32, mut depth: Option<&mut i32>) -> Result<i32> {
    let mut next = if offset >= 0 {
        check_node_offset(data, offset)?
    } else {
        0
    };
    loop {
        let at = next;
        let (tag, result) = next_tag(data, at);
        match tag {
            FDT_BEGIN_NODE => {
                if let Some(d) = depth.as_deref_mut() {
                    *d += 1;
                }
                return Ok(at);
            }
            FDT_END_NODE => {
                if let Some(d) = depth.as_deref_mut() {
                    *d -= 1;
                    if *d < 0 {
                        return result;
                    }
                }
            }
            FDT_END => {
                return Err(match result {
                    Ok(_) => Error::NotFound,
                    Err(Error::Truncated) if depth.is_none() => Error::NotFound,
                    Err(err) => err,
                })
            }
            _ => {}
        }
        next = result?;
    }
}
pub(crate) fn first_subnode(data: &[u8], offset: i32) -> Result<i32> {
    let mut depth = 0;
    let off = next_node(data, offset, Some(&mut depth)).map_err(|_| Error::NotFound)?;
    if depth == 1 {
        Ok(off)
    } else {
        Err(Error::NotFound)
    }
}
pub(crate) fn next_subnode(data: &[u8], mut offset: i32) -> Result<i32> {
    let mut depth = 1;
    loop {
        offset = next_node(data, offset, Some(&mut depth)).map_err(|_| Error::NotFound)?;
        if depth < 1 {
            return Err(Error::NotFound);
        }
        if depth == 1 {
            return Ok(offset);
        }
    }
}
pub(crate) fn move_into(data: &[u8], dest: &mut [u8]) -> Result<()> {
    let h = ro_probe(data)?;
    if h.totalsize > dest.len() {
        return Err(Error::NoSpace);
    }
    dest[..h.totalsize].copy_from_slice(&data[..h.totalsize]);
    Ok(())
}
