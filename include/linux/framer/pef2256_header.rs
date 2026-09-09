/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * PEF2256 consumer API
 *
 * Copyright 2023 CS GROUP France
 *
 * Author: Herve Codina <herve.codina@bootlin.com>
 */

#[repr(C)]
pub struct pef2256 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

extern "C" {
    /* Retrieve the PEF2256 regmap */
    pub fn pef2256_get_regmap(pef2256: *mut pef2256) -> *mut regmap;
}

/* PEF2256 hardware versions */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pef2256_version {
    PEF2256_VERSION_UNKNOWN = 0,
    PEF2256_VERSION_1_2 = 1,
    PEF2256_VERSION_2_1 = 2,
    PEF2256_VERSION_2_2 = 3,
}

extern "C" {
    /* Get the PEF2256 hardware version */
    pub fn pef2256_get_version(pef2256: *mut pef2256) -> pef2256_version;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
