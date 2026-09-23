// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Construct an empty, writable flattened tree.
// Copyright (C) 2012 David Gibson, IBM Corporation.
use super::*;
pub(crate) fn create_empty_tree(data: &mut [u8]) -> Result<()> {
    create(data)?;
    finish_reservemap(data)?;
    begin_node(data, b"")?;
    end_node(data)?;
    finish(data)?;
    open_inplace(data)
}
