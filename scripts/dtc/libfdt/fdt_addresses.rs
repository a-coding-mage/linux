// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Address and size cell validation and range encoding.
// Copyright (C) 2014 David Gibson <david@gibson.dropbear.id.au>
// Copyright (C) 2018 embedded brains GmbH
use super::*;

fn cells(data: &[u8], node: i32, name: &[u8]) -> Result<u32> {
    let value = getprop(data, node, name)?;
    if value.len() != 4 {
        return Err(Error::BadNCells);
    }
    let n = u32_at(value, 0)?;
    if n > MAX_NCELLS {
        return Err(Error::BadNCells);
    }
    Ok(n)
}
pub(crate) fn address_cells(data: &[u8], node: i32) -> Result<u32> {
    match cells(data, node, b"#address-cells") {
        Ok(0) => Err(Error::BadNCells),
        Err(Error::NotFound) => Ok(2),
        other => other,
    }
}
pub(crate) fn size_cells(data: &[u8], node: i32) -> Result<u32> {
    match cells(data, node, b"#size-cells") {
        Err(Error::NotFound) => Ok(1),
        other => other,
    }
}
pub(crate) fn appendprop_addrrange(
    data: &mut [u8],
    parent: i32,
    node: i32,
    name: &[u8],
    addr: u64,
    size: u64,
) -> Result<()> {
    let ac = address_cells(data, parent)?;
    let sc = size_cells(data, parent)?;
    let mut value = Vec::new();
    match ac {
        1 => {
            if addr > u32::MAX as u64 || (u32::MAX as u64 + 1 - addr) < size {
                return Err(Error::BadValue);
            }
            value.extend_from_slice(&(addr as u32).to_be_bytes());
        }
        2 => value.extend_from_slice(&addr.to_be_bytes()),
        _ => return Err(Error::BadNCells),
    }
    match sc {
        1 => {
            if size > u32::MAX as u64 {
                return Err(Error::BadValue);
            }
            value.extend_from_slice(&(size as u32).to_be_bytes());
        }
        2 => value.extend_from_slice(&size.to_be_bytes()),
        _ => return Err(Error::BadNCells),
    }
    appendprop(data, node, name, &value)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
