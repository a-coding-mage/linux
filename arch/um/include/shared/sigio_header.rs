/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 Jeff Dike (jdike@karaya.com)
 */

extern "C" {
    pub fn sigio_lock();
    pub fn sigio_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
