/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * export.h
 *
 * Function prototypes
 *
 * Copyright (C) 2002, 2005 Oracle.  All rights reserved.
 */

// C dependency: #include <linux/exportfs.h>
// `export_operations` is supplied by the translated Linux exportfs dependency.

#[repr(C)]
pub struct export_operations {
    _private: [u8; 0],
}

extern "C" {
    pub static ocfs2_export_ops: export_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
