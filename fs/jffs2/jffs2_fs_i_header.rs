/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// C dependencies: linux/rbtree.h, linux/posix_acl.h, and linux/mutex.h.

// External types supplied by the corresponding translated dependencies.
pub use crate::{inode, jffs2_full_dirent, jffs2_full_dnode, jffs2_inode_cache, mutex, rb_root};

#[repr(C)]
pub struct jffs2_inode_info {
    /* We need an internal mutex similar to inode->i_rwsem.
       Unfortunately, we can't used the existing one, because
       either the GC would deadlock, or we'd have to release it
       before letting GC proceed. Or we'd have to put ugliness
       into the GC code so it didn't attempt to obtain the i_rwsem
       for the inode(s) which are already locked */
    pub sem: mutex,

    /* The highest (datanode) version number used for this ino */
    pub highest_version: u32,

    /* List of data fragments which make up the file */
    pub fragtree: rb_root,

    /* There may be one datanode which isn't referenced by any of the
       above fragments, if it contains a metadata update but no actual
       data - or if this is a directory inode */
    /* This also holds the _only_ dnode for symlinks/device nodes,
       etc. */
    pub metadata: *mut jffs2_full_dnode,

    /* Directory entries */
    pub dents: *mut jffs2_full_dirent,

    /* The target path if this is the inode of a symlink */
    pub target: *mut u8,

    /* Some stuff we just have to keep in-core at all times, for each inode. */
    pub inocache: *mut jffs2_inode_cache,

    pub flags: u16,
    pub usercompr: u8,
    pub vfs_inode: inode,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
