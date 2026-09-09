// SPDX-License-Identifier: GPL-2.0-or-later
/* Board-specific reboot/shutdown routines
 * Copyright (c) 2009 Philippe Vachon <philippe@cowpig.ca>
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// Dependency supplied by loongson.h.
unsafe extern "C" {
    static mut LOONGSON_GENCFG: u32;
}

pub unsafe fn mach_prepare_reboot() {
    LOONGSON_GENCFG &= !(1u32 << 2);
    LOONGSON_GENCFG |= 1u32 << 2;
}

pub unsafe fn mach_prepare_shutdown() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
