/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

/* CONFIG_OPENRISC_HAVE_INST_FF1 */
#[cfg(feature = "CONFIG_OPENRISC_HAVE_INST_FF1")]
#[inline]
pub unsafe fn ffs(x: i32) -> i32 {
    let mut ret: i32;

    core::arch::asm!(
        "l.ff1 {ret}, {x}",
        ret = lateout(reg) ret,
        x = in(reg) x,
    );

    ret
}

/* The generic implementation is supplied by asm-generic/bitops/ffs.h. */
#[cfg(not(feature = "CONFIG_OPENRISC_HAVE_INST_FF1"))]
unsafe extern "C" {
    pub fn ffs(x: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
