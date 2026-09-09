/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 *
 */

// Dependency declarations supplied by nodelist.h and other translation units.
#[repr(C)]
pub struct inode_operations {
    pub get_link: Option<unsafe extern "C" fn()>,
    pub setattr: Option<unsafe extern "C" fn()>,
    pub listxattr: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    pub fn simple_get_link();
    pub fn jffs2_setattr();
    pub fn jffs2_listxattr();
}

pub static jffs2_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(simple_get_link),
    setattr: Some(jffs2_setattr),
    listxattr: Some(jffs2_listxattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
