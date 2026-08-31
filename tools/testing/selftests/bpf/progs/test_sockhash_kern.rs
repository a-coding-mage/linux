// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Covalent IO, Inc. http://covalent.io

// C preprocessor intent:
//   #undef SOCKMAP
//   #define TEST_MAP_TYPE BPF_MAP_TYPE_SOCKHASH
//   #include "./test_sockmap_kern.h"

pub const TEST_MAP_TYPE: u32 = BPF_MAP_TYPE_SOCKHASH;
