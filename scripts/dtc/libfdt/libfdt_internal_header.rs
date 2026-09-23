// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Shared checked header access and structure-block indexing.
use super::*;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Header {
    pub(crate) magic: u32,
    pub(crate) totalsize: usize,
    pub(crate) off_dt_struct: usize,
    pub(crate) off_dt_strings: usize,
    pub(crate) off_mem_rsvmap: usize,
    pub(crate) version: u32,
    pub(crate) last_comp_version: u32,
    pub(crate) boot_cpuid_phys: u32,
    pub(crate) size_dt_strings: usize,
    pub(crate) size_dt_struct: usize,
}
impl Header {
    pub(crate) fn read(data: &[u8]) -> Result<Self> {
        let version = u32_at(data, 20)?;
        bytes(data, 0, header_size(version))?;
        Ok(Self {
            magic: u32_at(data, 0)?,
            totalsize: u32_at(data, 4)? as usize,
            off_dt_struct: u32_at(data, 8)? as usize,
            off_dt_strings: u32_at(data, 12)? as usize,
            off_mem_rsvmap: u32_at(data, 16)? as usize,
            version,
            last_comp_version: u32_at(data, 24)?,
            boot_cpuid_phys: if version >= 2 { u32_at(data, 28)? } else { 0 },
            size_dt_strings: if version >= 3 {
                u32_at(data, 32)? as usize
            } else {
                0
            },
            size_dt_struct: if version >= 17 {
                u32_at(data, 36)? as usize
            } else {
                0
            },
        })
    }
}
pub(super) fn ro_probe(data: &[u8]) -> Result<Header> {
    let h = Header::read(data)?;
    match h.magic {
        FDT_MAGIC => {
            if h.version < 2 || h.last_comp_version > 17 {
                return Err(Error::BadVersion);
            }
        }
        FDT_SW_MAGIC => {
            if h.size_dt_struct == 0 {
                return Err(Error::BadState);
            }
        }
        _ => return Err(Error::BadMagic),
    }
    if h.totalsize >= i32::MAX as usize || h.totalsize > data.len() {
        return Err(Error::Truncated);
    }
    Ok(h)
}
pub(super) fn struct_range(data: &[u8], offset: i32, len: usize) -> Result<std::ops::Range<usize>> {
    let h = Header::read(data)?;
    let off = usize::try_from(offset).map_err(|_| Error::Truncated)?;
    let end = off.checked_add(len).ok_or(Error::Truncated)?;
    if h.version >= 17 && end > h.size_dt_struct {
        return Err(Error::Truncated);
    }
    let abs = h.off_dt_struct.checked_add(off).ok_or(Error::Truncated)?;
    let abs_end = abs.checked_add(len).ok_or(Error::Truncated)?;
    if abs_end > h.totalsize || abs_end > data.len() {
        return Err(Error::Truncated);
    }
    Ok(abs..abs_end)
}
pub(super) fn struct_abs(data: &[u8], offset: i32, len: usize) -> Result<usize> {
    Ok(struct_range(data, offset, len)?.start)
}
pub(super) fn check_node_offset(data: &[u8], offset: i32) -> Result<i32> {
    if offset < 0 || offset % 4 != 0 {
        return Err(Error::BadOffset);
    }
    match next_tag(data, offset) {
        (FDT_BEGIN_NODE, Ok(next)) => Ok(next),
        _ => Err(Error::BadOffset),
    }
}
pub(super) fn check_prop_offset(data: &[u8], offset: i32) -> Result<i32> {
    if offset < 0 || offset % 4 != 0 {
        return Err(Error::BadOffset);
    }
    match next_tag(data, offset) {
        (FDT_PROP, Ok(next)) => Ok(next),
        _ => Err(Error::BadOffset),
    }
}
pub(super) fn node_end_offset(data: &[u8], mut offset: i32) -> Result<i32> {
    let mut depth = 0;
    while depth >= 0 {
        offset = next_node(data, offset, Some(&mut depth))?;
    }
    Ok(offset)
}
