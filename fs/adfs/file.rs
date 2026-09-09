// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/adfs/file.c
 *
 * Copyright (C) 1997-1999 Russell King
 * from:
 *
 *  linux/fs/ext2/file.c
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from
 *
 *  linux/fs/minix/file.c
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  adfs regular file handling primitives
 */

// Supplied by the corresponding ADFS and VFS declarations.
extern "C" {
    fn generic_file_llseek();
    fn generic_file_read_iter();
    fn generic_file_mmap_prepare();
    fn simple_fsync();
    fn generic_file_write_iter();
    fn filemap_splice_read();
    fn adfs_setattr();
}

pub static adfs_file_operations: file_operations = file_operations {
    llseek: Some(generic_file_llseek),
    read_iter: Some(generic_file_read_iter),
    mmap_prepare: Some(generic_file_mmap_prepare),
    fsync: Some(simple_fsync),
    write_iter: Some(generic_file_write_iter),
    splice_read: Some(filemap_splice_read),
};

pub static adfs_file_inode_operations: inode_operations = inode_operations {
    setattr: Some(adfs_setattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
