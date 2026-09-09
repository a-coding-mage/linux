/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Definitions for vfsv0 quota format
 */

// Dependency supplied by the translated qtree header:
// QTREE_INIT_ALLOC, QTREE_INIT_REWRITE, QTREE_DEL_ALLOC, and QTREE_DEL_REWRITE.

/* Numbers of blocks needed for updates */
pub const V2_INIT_ALLOC: usize = QTREE_INIT_ALLOC;
pub const V2_INIT_REWRITE: usize = QTREE_INIT_REWRITE;
pub const V2_DEL_ALLOC: usize = QTREE_DEL_ALLOC;
pub const V2_DEL_REWRITE: usize = QTREE_DEL_REWRITE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
