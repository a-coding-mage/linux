/* SPDX-License-Identifier: GPL-2.0-or-later */
/* internal.h: ramfs internal definitions
 *
 * Copyright (C) 2005 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// External declaration corresponding to:
// extern const struct inode_operations ramfs_file_inode_operations;
extern "C" {
    pub static ramfs_file_inode_operations: inode_operations;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
