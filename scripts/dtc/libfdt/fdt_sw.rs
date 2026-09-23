// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Sequential construction with libfdt's reservation/structure/completion states.
// Copyright (C) 2006 David Gibson, IBM Corporation.
use super::*;

fn sw_probe(data: &[u8]) -> Result<Header> {
    let h = Header::read(data)?;
    match h.magic {
        FDT_SW_MAGIC => {
            if h.totalsize > data.len() {
                return Err(Error::Truncated);
            }
            Ok(h)
        }
        FDT_MAGIC => Err(Error::BadState),
        _ => Err(Error::BadMagic),
    }
}
fn sw_structure(data: &[u8]) -> Result<Header> {
    let h = sw_probe(data)?;
    if h.off_dt_strings != h.totalsize {
        return Err(Error::BadState);
    }
    Ok(h)
}
fn grab_space(data: &mut [u8], len: usize) -> Result<usize> {
    let h = Header::read(data)?;
    let at = h
        .off_dt_struct
        .checked_add(h.size_dt_struct)
        .ok_or(Error::NoSpace)?;
    let end = at.checked_add(len).ok_or(Error::NoSpace)?;
    if end
        > h.totalsize
            .checked_sub(h.size_dt_strings)
            .ok_or(Error::NoSpace)?
        || end > data.len()
    {
        return Err(Error::NoSpace);
    }
    put32(data, 36, (h.size_dt_struct + len) as u32)?;
    Ok(at)
}
pub(crate) fn create_with_flags(data: &mut [u8], flags: u32) -> Result<()> {
    if data.len() < 48 || data.len() > i32::MAX as usize {
        return Err(Error::NoSpace);
    }
    if flags & !CREATE_FLAG_NO_NAME_DEDUP != 0 {
        return Err(Error::BadFlags);
    }
    data.fill(0);
    put32(data, 0, FDT_SW_MAGIC)?;
    put32(data, 20, 17)?;
    put32(data, 24, flags)?;
    put32(data, 4, data.len() as u32)?;
    put32(data, 16, 48)?;
    put32(data, 8, 48)
}
pub(crate) fn create(data: &mut [u8]) -> Result<()> {
    create_with_flags(data, 0)
}
pub(crate) fn resize(data: &[u8], dest: &mut [u8]) -> Result<()> {
    let h = sw_probe(data)?;
    let head = h
        .off_dt_struct
        .checked_add(h.size_dt_struct)
        .ok_or(Error::Internal)?;
    let tail = h.size_dt_strings;
    let total = head.checked_add(tail).ok_or(Error::Internal)?;
    if total > h.totalsize {
        return Err(Error::Internal);
    }
    if total > dest.len() || dest.len() > i32::MAX as usize {
        return Err(Error::NoSpace);
    }
    put(dest, 0, bytes(data, 0, head)?)?;
    put(
        dest,
        dest.len() - tail,
        bytes(data, h.totalsize - tail, tail)?,
    )?;
    put32(dest, 4, dest.len() as u32)?;
    if h.off_dt_strings != 0 {
        put32(dest, 12, dest.len() as u32)?;
    }
    Ok(())
}
pub(crate) fn add_reservemap_entry(data: &mut [u8], address: u64, size: u64) -> Result<()> {
    let h = sw_probe(data)?;
    if h.off_dt_strings != 0 {
        return Err(Error::BadState);
    }
    if h.off_dt_struct
        .checked_add(16)
        .is_none_or(|end| end > h.totalsize || end > data.len())
    {
        return Err(Error::NoSpace);
    }
    put64(data, h.off_dt_struct, address)?;
    put64(data, h.off_dt_struct + 8, size)?;
    put32(data, 8, (h.off_dt_struct + 16) as u32)
}
pub(crate) fn finish_reservemap(data: &mut [u8]) -> Result<()> {
    add_reservemap_entry(data, 0, 0)?;
    put32(data, 12, Header::read(data)?.totalsize as u32)
}
pub(crate) fn begin_node(data: &mut [u8], name: &[u8]) -> Result<()> {
    sw_structure(data)?;
    let at = grab_space(data, 4 + align(name.len() + 1, 4)?)?;
    put32(data, at, FDT_BEGIN_NODE)?;
    put(data, at + 4, name)?;
    data[at + 4 + name.len()] = 0;
    Ok(())
}
pub(crate) fn end_node(data: &mut [u8]) -> Result<()> {
    sw_structure(data)?;
    let at = grab_space(data, 4)?;
    put32(data, at, FDT_END_NODE)
}
fn add_string(data: &mut [u8], name: &[u8], dedup: bool) -> Result<(i32, bool)> {
    let h = Header::read(data)?;
    let start = h
        .totalsize
        .checked_sub(h.size_dt_strings)
        .ok_or(Error::NoSpace)?;
    if dedup {
        if let Some(offset) = find_string(bytes(data, start, h.size_dt_strings)?, name) {
            return Ok((
                (offset as i32).wrapping_sub(h.size_dt_strings as i32),
                false,
            ));
        }
    }
    let newsize = h
        .size_dt_strings
        .checked_add(name.len() + 1)
        .ok_or(Error::NoSpace)?;
    let at = h.totalsize.checked_sub(newsize).ok_or(Error::NoSpace)?;
    if at < h.off_dt_struct + h.size_dt_struct {
        return Err(Error::NoSpace);
    }
    put(data, at, name)?;
    data[at + name.len()] = 0;
    put32(data, 32, newsize as u32)?;
    Ok((-(newsize as i32), true))
}
pub(crate) fn property_placeholder<'a>(
    data: &'a mut [u8],
    name: &[u8],
    len: usize,
) -> Result<&'a mut [u8]> {
    let h = sw_structure(data)?;
    if len > i32::MAX as usize {
        return Err(Error::NoSpace);
    }
    let (nameoff, allocated) = add_string(
        data,
        name,
        h.last_comp_version & CREATE_FLAG_NO_NAME_DEDUP == 0,
    )?;
    let at = match grab_space(data, 12 + align(len, 4)?) {
        Ok(at) => at,
        Err(err) => {
            if allocated {
                put32(data, 32, h.size_dt_strings as u32)?;
            }
            return Err(err);
        }
    };
    put32(data, at, FDT_PROP)?;
    put32(data, at + 4, len as u32)?;
    put32(data, at + 8, nameoff as u32)?;
    Ok(&mut data[at + 12..at + 12 + len])
}
pub(crate) fn property_write(data: &mut [u8], name: &[u8], value: &[u8]) -> Result<()> {
    property_placeholder(data, name, value.len())?.copy_from_slice(value);
    Ok(())
}
pub(crate) fn finish(data: &mut [u8]) -> Result<()> {
    sw_structure(data)?;
    let at = grab_space(data, 4)?;
    put32(data, at, FDT_END)?;
    let h = Header::read(data)?;
    let old = h.totalsize - h.size_dt_strings;
    let new = h.off_dt_struct + h.size_dt_struct;
    bytes(data, old, h.size_dt_strings)?;
    bytes(data, new, h.size_dt_strings)?;
    data.copy_within(old..old + h.size_dt_strings, new);
    put32(data, 12, new as u32)?;
    let mut at = 0;
    loop {
        let (tag, next) = next_tag(data, at);
        if tag == FDT_END {
            next?;
            break;
        }
        if tag == FDT_PROP {
            let offset = struct_abs(data, at + 8, 4)?;
            let name = u32_at(data, offset)?.wrapping_add(h.size_dt_strings as u32);
            put32(data, offset, name)?;
        }
        at = next?;
    }
    put32(data, 4, (new + h.size_dt_strings) as u32)?;
    put32(data, 24, 16)?;
    put32(data, 0, FDT_MAGIC)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
