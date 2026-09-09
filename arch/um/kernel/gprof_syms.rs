// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependency: <linux/module.h>

unsafe extern "C" {
    pub fn mcount();
}

// EXPORT_SYMBOL(mcount);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
