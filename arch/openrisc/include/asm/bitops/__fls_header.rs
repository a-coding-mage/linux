/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// CONFIG_OPENRISC_HAVE_INST_FL1 selects the OpenRISC FL1 instruction.
#[cfg(CONFIG_OPENRISC_HAVE_INST_FL1)]
#[inline]
pub unsafe fn __fls(x: usize) -> usize {
    let mut ret: i32;
    core::arch::asm!(
        "l.fl1 {ret}, {x}",
        ret = out(reg) ret,
        x = in(reg) x,
    );
    ret.wrapping_sub(1) as usize
}

// Without CONFIG_OPENRISC_HAVE_INST_FL1, this header includes the
// architecture-generic bitops/__fls.h implementation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
