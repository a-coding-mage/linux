/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// The `dax_holder_operations` type is supplied by an external dependency.
unsafe extern "C" {
    pub static xfs_dax_holder_operations: dax_holder_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
