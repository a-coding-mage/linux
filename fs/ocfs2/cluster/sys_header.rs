/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * sys.h
 *
 * Function prototypes for o2cb sysfs interface
 *
 * Copyright (C) 2005 Oracle.  All rights reserved.
 */

// C header guard: O2CLUSTER_SYS_H

extern "C" {
    pub fn o2cb_sys_shutdown();
    pub fn o2cb_sys_init() -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
