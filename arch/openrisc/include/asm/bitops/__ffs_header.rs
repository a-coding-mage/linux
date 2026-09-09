/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

use core::ffi::{c_int, c_ulong};

#[cfg(feature = "CONFIG_OPENRISC_HAVE_INST_FF1")]
#[inline]
pub unsafe fn __ffs(x: c_ulong) -> c_ulong {
    let ret: c_int;

    core::arch::asm!(
        "l.ff1 {ret}, {x}",
        ret = out(reg) ret,
        x = in(reg) x,
    );

    (ret as c_ulong).wrapping_sub(1)
}

#[cfg(not(feature = "CONFIG_OPENRISC_HAVE_INST_FF1"))]
// The C header includes <asm-generic/bitops/__ffs.h>; this declaration is
// supplied by the corresponding generic bit-operations dependency.
unsafe extern "C" {
    pub fn __ffs(x: c_ulong) -> c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
