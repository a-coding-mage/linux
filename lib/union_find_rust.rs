// SPDX-License-Identifier: GPL-2.0
//! Built-in union-find owner. The original functions are not module exports.

#[path = "union_find.rs"]
mod implementation;

pub use implementation::{uf_find, uf_union};
