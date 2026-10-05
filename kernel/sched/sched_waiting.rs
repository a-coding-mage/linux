// SPDX-License-Identifier: GPL-2.0-only
//! Coupled completion, simple-wait and regular/bit-wait source owner proposal.
#![no_std]

compile_error!("Lupos waiting source-only hold: native ABI/CFI/locking admission pending");

mod completion;
mod swait;
mod wait;
mod wait_bit;
