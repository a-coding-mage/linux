/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Synopsys, Inc. (www.synopsys.com)
 */

/*
 * Early 8250 (now earlycon) requires BASE_BAUD to be defined in this header.
 * However, to still determine it dynamically (for multi-platform images),
 * this is done in a helper by parsing the FDT early.
 */

// C attribute: __init
unsafe extern "C" {
    pub fn arc_early_base_baud() -> ::core::ffi::c_uint;
}

macro_rules! BASE_BAUD {
    () => {
        unsafe { arc_early_base_baud() }
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
