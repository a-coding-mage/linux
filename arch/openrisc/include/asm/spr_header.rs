/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

/*
 * C header guard: __ASM_OPENRISC_SPR_H
 */

#[macro_export]
macro_rules! mtspr {
    ($spr:expr, $val:expr) => {{
        unsafe {
            core::arch::asm!(
                "l.mtspr r0, {val}, {spr}",
                spr = const $spr,
                val = in(reg) $val,
                options(nostack)
            );
        }
    }};
}

#[macro_export]
macro_rules! mtspr_off {
    ($spr:expr, $off:expr, $val:expr) => {{
        unsafe {
            core::arch::asm!(
                "l.mtspr {off}, {val}, {spr}",
                off = in(reg) $off,
                val = in(reg) $val,
                spr = const $spr,
                options(nostack)
            );
        }
    }};
}

#[inline]
pub unsafe fn mfspr(add: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "l.mfspr {ret}, r0, {add}",
        ret = out(reg) ret,
        add = in(reg) add,
        options(nostack)
    );
    ret
}

#[inline]
pub unsafe fn mfspr_off(add: usize, offset: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "l.mfspr {ret}, {offset}, {add}",
        ret = out(reg) ret,
        offset = in(reg) offset,
        add = in(reg) add,
        options(nostack)
    );
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
