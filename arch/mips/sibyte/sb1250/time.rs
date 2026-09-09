// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001 Broadcom Corporation
 */

// linux/init.h: the C __init annotation is a build/linker attribute.

extern "C" {
    pub fn sb1250_clocksource_init();
    pub fn sb1250_clockevent_init();
}

#[no_mangle]
pub extern "C" fn plat_time_init() {
    unsafe {
        sb1250_clocksource_init();
        sb1250_clockevent_init();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
