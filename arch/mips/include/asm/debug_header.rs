/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Imagination Technologies
 */

// Dependency: <linux/dcache.h> supplies `struct dentry`.

/*
 * mips_debugfs_dir corresponds to the "mips" directory at the top level
 * of the DebugFS hierarchy. MIPS-specific DebugFS entries should be
 * placed beneath this directory.
 */
extern "C" {
    pub static mut mips_debugfs_dir: *mut dentry;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
