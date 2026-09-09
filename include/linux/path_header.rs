/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declarations corresponding to the C forward declarations.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vfsmount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct path {
    pub mnt: *mut vfsmount,
    pub dentry: *mut dentry,
}

extern "C" {
    pub fn path_get(path: *const path);
    pub fn path_put(path: *const path);
}

pub unsafe fn path_equal(path1: *const path, path2: *const path) -> i32 {
    ((*path1).mnt == (*path2).mnt && (*path1).dentry == (*path2).dentry) as i32
}

/*
 * Cleanup macro for use with __free(path_put). Avoids dereference and
 * copying @path unlike DEFINE_FREE(). path_put() will handle the empty
 * path correctly just ensure @path is initialized:
 *
 * struct path path __free(path_put) = {};
 */
// C macro equivalent: __free_path_put expands to path_put.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
