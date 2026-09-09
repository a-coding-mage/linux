// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000,2001,2004 Broadcom Corporation
 */

// C dependency: <linux/init.h>; `__init` is a kernel initialization annotation.

unsafe extern "C" {
    fn sb1480_clockevent_init();
    fn sb1480_clocksource_init();
}

/// C: `void __init plat_time_init(void)`
pub unsafe extern "C" fn plat_time_init() {
    sb1480_clocksource_init();
    sb1480_clockevent_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
