// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 ARM Limited
 *
 * Author: Vladimir Murzin <vladimir.murzin@arm.com>
 */

use core::ffi::c_char;

// Equivalent to the C __initconst-qualified static compatibility table.
static MPS2_COMPAT: [*const c_char; 2] = [
    b"arm,mps2\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

// DT_MACHINE_START(MPS2DT, "MPS2 (Device Tree Support)")
//     .dt_compat = mps2_compat,
// MACHINE_END
//
// The machine-descriptor definition is supplied by the architecture headers
// and is preserved here as the corresponding external build-time construct.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
