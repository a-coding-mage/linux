// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Covalent IO, Inc. http://covalent.io

pub const SOCKMAP: bool = true;
pub const TEST_MAP_TYPE: u32 = BPF_MAP_TYPE_SOCKMAP;

// C source included "./test_sockmap_kern.h" here; declarations and definitions
// supplied by that file are expected as external dependencies in the Rust build.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
