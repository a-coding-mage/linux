// SPDX-License-Identifier: GPL-2.0
//! Raw union-find nodes shared with the C interface.
//!
//! Nodes must remain at fixed addresses and callers must serialize access to
//! each forest. The native provider may be C or Rust. Its original functions
//! are built-in interfaces and are not exported to loadable modules.

#[path = "../../include/linux/union_find_header.rs"]
mod declarations;

pub use declarations::*;
