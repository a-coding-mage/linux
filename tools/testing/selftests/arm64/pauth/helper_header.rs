// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C dependency intent: #include <stdlib.h> for size_t.

pub const NKEYS: usize = 5;

#[repr(C)]
pub struct signatures {
    pub keyia: usize,
    pub keyib: usize,
    pub keyda: usize,
    pub keydb: usize,
    pub keyg: usize,
}

unsafe extern "C" {
    pub fn pac_corruptor();

    // PAuth sign a value with key ia and modifier value 0
    pub fn keyia_sign(val: usize) -> usize;
    pub fn keyib_sign(val: usize) -> usize;
    pub fn keyda_sign(val: usize) -> usize;
    pub fn keydb_sign(val: usize) -> usize;
    pub fn keyg_sign(val: usize) -> usize;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
