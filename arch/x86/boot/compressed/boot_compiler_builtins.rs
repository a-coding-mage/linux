// SPDX-License-Identifier: GPL-2.0
//! Marker crate for the compiler's freestanding builtin dependency.
// No intrinsic has a fake implementation. A used intrinsic without a genuine
// boot owner remains an unresolved link error and must be ported explicitly.
#![no_std]
#![no_builtins]
#![allow(internal_features)]
#![feature(compiler_builtins)]
#![compiler_builtins]
