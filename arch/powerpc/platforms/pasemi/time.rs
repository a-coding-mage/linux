// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006 PA Semi, Inc
 *
 * Maintained by: Olof Johansson <olof@lixom.net>
 */

// C dependencies: <linux/time.h>, <asm/time.h>, and "pasemi.h".

pub type time64_t = i64;

unsafe extern "C" {
    fn mktime64(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64) -> time64_t;
}

// __init
#[no_mangle]
pub unsafe extern "C" fn pas_get_boot_time() -> time64_t {
    /* Let's just return a fake date right now */
    mktime64(2006, 1, 1, 12, 0, 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
