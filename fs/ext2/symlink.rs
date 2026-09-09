// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ext2/symlink.c
 *
 * Only fast symlinks left here - the rest is done by generic code. AV, 1999
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from
 *
 *  linux/fs/minix/symlink.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  ext2 symlink handling code
 */

// Dependencies supplied by ext2.h and xattr.h are expected to provide these
// types and functions in the containing translation unit/module.

pub const ext2_symlink_inode_operations: inode_operations = inode_operations {
    get_link: page_get_link,
    getattr: ext2_getattr,
    setattr: ext2_setattr,
    listxattr: ext2_listxattr,
};

pub const ext2_fast_symlink_inode_operations: inode_operations = inode_operations {
    get_link: simple_get_link,
    getattr: ext2_getattr,
    setattr: ext2_setattr,
    listxattr: ext2_listxattr,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
