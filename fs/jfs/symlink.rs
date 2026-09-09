// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) Christoph Hellwig, 2001-2002
 */

// Dependency intent preserved from <linux/fs.h>, "jfs_incore.h",
// "jfs_inode.h", and "jfs_xattr.h". The referenced types and symbols are
// supplied by the surrounding translation unit.

pub static jfs_fast_symlink_inode_operations: inode_operations = inode_operations {
    get_link: simple_get_link,
    setattr: jfs_setattr,
    listxattr: jfs_listxattr,
};

pub static jfs_symlink_inode_operations: inode_operations = inode_operations {
    get_link: page_get_link,
    setattr: jfs_setattr,
    listxattr: jfs_listxattr,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
