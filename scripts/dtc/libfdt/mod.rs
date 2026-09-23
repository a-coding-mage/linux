// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Safe, byte-oriented flattened device-tree library.
//!
//! Offsets refer to the structure block, as in libfdt. Names and values are
//! bytes, not UTF-8 strings. All wire reads are checked and big-endian; no
//! alignment or native-layout assumptions are made about the backing slice.
#![allow(dead_code)] // Different tools use different parts of the full library.
#![allow(unused_imports)] // Keep one complete API facade for all consumers.

mod fdt;
mod fdt_addresses;
mod fdt_empty_tree;
mod fdt_header;
mod fdt_overlay;
mod fdt_ro;
mod fdt_rw;
mod fdt_strerror;
mod fdt_sw;
mod fdt_wip;
mod libfdt_env_header;
mod libfdt_header;
mod libfdt_internal_header;
pub(crate) mod tools;

pub(crate) use fdt::*;
pub(crate) use fdt_addresses::*;
pub(crate) use fdt_empty_tree::*;
pub(crate) use fdt_header::*;
pub(crate) use fdt_overlay::*;
pub(crate) use fdt_ro::*;
pub(crate) use fdt_rw::*;
pub(crate) use fdt_strerror::*;
pub(crate) use fdt_sw::*;
pub(crate) use fdt_wip::*;
use libfdt_env_header::*;
pub(crate) use libfdt_header::*;
pub(crate) use libfdt_internal_header::Header;
use libfdt_internal_header::*;
