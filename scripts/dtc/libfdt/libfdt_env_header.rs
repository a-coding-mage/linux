// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Checked wire-format accessors replacing native C endian/layout helpers.
use super::{Error, Result};

pub(super) fn bytes(data: &[u8], at: usize, len: usize) -> Result<&[u8]> {
    data.get(at..at.checked_add(len).ok_or(Error::Truncated)?)
        .ok_or(Error::Truncated)
}
pub(super) fn u32_at(data: &[u8], at: usize) -> Result<u32> {
    Ok(u32::from_be_bytes(bytes(data, at, 4)?.try_into().unwrap()))
}
pub(super) fn u64_at(data: &[u8], at: usize) -> Result<u64> {
    Ok(u64::from_be_bytes(bytes(data, at, 8)?.try_into().unwrap()))
}
pub(super) fn put(data: &mut [u8], at: usize, value: &[u8]) -> Result<()> {
    let end = at.checked_add(value.len()).ok_or(Error::Truncated)?;
    data.get_mut(at..end)
        .ok_or(Error::Truncated)?
        .copy_from_slice(value);
    Ok(())
}
pub(super) fn put32(data: &mut [u8], at: usize, value: u32) -> Result<()> {
    put(data, at, &value.to_be_bytes())
}
pub(super) fn put64(data: &mut [u8], at: usize, value: u64) -> Result<()> {
    put(data, at, &value.to_be_bytes())
}
pub(super) fn align(value: usize, by: usize) -> Result<usize> {
    value
        .checked_add(by - 1)
        .map(|x| x & !(by - 1))
        .ok_or(Error::NoSpace)
}
pub(super) fn cstr(data: &[u8]) -> Result<&[u8]> {
    Ok(&data[..data.iter().position(|&c| c == 0).ok_or(Error::Truncated)?])
}
pub(super) fn find_string(table: &[u8], name: &[u8]) -> Option<usize> {
    table
        .windows(name.len().checked_add(1)?)
        .position(|s| s[..name.len()] == *name && s[name.len()] == 0)
}
