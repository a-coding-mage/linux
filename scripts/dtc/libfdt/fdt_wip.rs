// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Mutations that preserve structure offsets.
// Copyright (C) 2006 David Gibson, IBM Corporation.
use super::*;

pub(crate) fn setprop_inplace_partial(
    data: &mut [u8],
    node: i32,
    name: &[u8],
    index: usize,
    value: &[u8],
) -> Result<()> {
    let prop = property(data, node, name)?;
    if index
        .checked_add(value.len())
        .is_none_or(|end| end > prop.data.len())
    {
        return Err(Error::NoSpace);
    }
    put(data, prop.data_offset + index, value)
}
pub(crate) fn setprop_inplace(data: &mut [u8], node: i32, name: &[u8], value: &[u8]) -> Result<()> {
    if getprop(data, node, name)?.len() != value.len() {
        return Err(Error::NoSpace);
    }
    setprop_inplace_partial(data, node, name, 0, value)
}
fn nop_region(data: &mut [u8], start: usize, len: usize) -> Result<()> {
    let len = align(len, 4)?;
    bytes(data, start, len)?;
    for at in (start..start + len).step_by(4) {
        put32(data, at, FDT_NOP)?;
    }
    Ok(())
}
pub(crate) fn nop_property(data: &mut [u8], node: i32, name: &[u8]) -> Result<()> {
    let prop = get_property(data, node, name)?;
    nop_region(data, prop.data_offset - 12, prop.data.len() + 12)
}
pub(crate) fn nop_node(data: &mut [u8], node: i32) -> Result<()> {
    let end = node_end_offset(data, node)?;
    nop_region(data, struct_abs(data, node, 0)?, (end - node) as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
