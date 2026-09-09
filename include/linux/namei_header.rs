/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const MAX_NESTED_LINKS: i32 = 8;
pub const MAXSYMLINKS: u32 = 40;

/* pathwalk mode */
pub const LOOKUP_FOLLOW: u32 = 1 << 0; /* follow links at the end */
pub const LOOKUP_DIRECTORY: u32 = 1 << 1; /* require a directory */
pub const LOOKUP_AUTOMOUNT: u32 = 1 << 2; /* force terminal automount */
pub const LOOKUP_EMPTY: u32 = 1 << 3; /* accept empty path [user_... only] */
pub const LOOKUP_LINKAT_EMPTY: u32 = 1 << 4; /* Linkat request with empty path. */
pub const LOOKUP_DOWN: u32 = 1 << 5; /* follow mounts in the starting point */
pub const LOOKUP_MOUNTPOINT: u32 = 1 << 6; /* follow mounts in the end */
pub const LOOKUP_REVAL: u32 = 1 << 7; /* tell ->d_revalidate() to trust no cache */
pub const LOOKUP_RCU: u32 = 1 << 8; /* RCU pathwalk mode; semi-internal */
pub const LOOKUP_CACHED: u32 = 1 << 9; /* Only do cached lookup */
pub const LOOKUP_PARENT: u32 = 1 << 10; /* Looking up final parent in path */

/* These tell filesystem methods that we are dealing with the final component... */
pub const LOOKUP_OPEN: u32 = 1 << 16; /* ... in open */
pub const LOOKUP_CREATE: u32 = 1 << 17; /* ... in object creation */
pub const LOOKUP_EXCL: u32 = 1 << 18; /* ... in target must not exist */
pub const LOOKUP_RENAME_TARGET: u32 = 1 << 19; /* ... in destination of rename() */

/* Scoping flags for lookup. */
pub const LOOKUP_NO_SYMLINKS: u32 = 1 << 24; /* No symlink crossing. */
pub const LOOKUP_NO_MAGICLINKS: u32 = 1 << 25; /* No nd_jump_link() crossing. */
pub const LOOKUP_NO_XDEV: u32 = 1 << 26; /* No mountpoint crossing. */
pub const LOOKUP_BENEATH: u32 = 1 << 27; /* No escaping from starting point. */
pub const LOOKUP_IN_ROOT: u32 = 1 << 28; /* Treat dirfd as fs root. */
pub const LOOKUP_IS_SCOPED: u32 = LOOKUP_BENEATH | LOOKUP_IN_ROOT;

extern "C" {
    pub fn path_pts(path: *mut path) -> i32;
    pub fn user_path_at(fd: i32, name: *const c_char, flags: u32, path: *mut path) -> i32;
    pub fn kern_path(name: *const c_char, flags: u32, path: *mut path) -> i32;
    pub fn kern_path_parent(name: *const c_char, parent: *mut path) -> *mut dentry;
    pub fn start_creating_path(fd: i32, name: *const c_char, path: *mut path, flags: u32) -> *mut dentry;
    pub fn start_creating_user_path(name: *const c_char, path: *mut path, flags: u32) -> *mut dentry;
    pub fn end_creating_path(path: *const path, dentry: *mut dentry);
    pub fn start_removing_path(name: *const c_char, path: *mut path) -> *mut dentry;
    pub fn vfs_path_parent_lookup(filename: *mut filename, flags: u32, parent: *mut path, last: *mut qstr, root: *const path) -> i32;
    pub fn vfs_path_lookup(dentry: *mut dentry, mnt: *mut vfsmount, name: *const c_char, flags: u32, path: *mut path) -> i32;
    pub fn try_lookup_noperm(name: *mut qstr, dir: *mut dentry) -> *mut dentry;
    pub fn lookup_noperm(name: *mut qstr, dir: *mut dentry) -> *mut dentry;
    pub fn lookup_noperm_unlocked(name: *mut qstr, dir: *mut dentry) -> *mut dentry;
    pub fn lookup_noperm_positive_unlocked(name: *mut qstr, dir: *mut dentry) -> *mut dentry;
    pub fn lookup_one(idmap: *mut mnt_idmap, name: *mut qstr, base: *mut dentry) -> *mut dentry;
    pub fn lookup_one_unlocked(idmap: *mut mnt_idmap, name: *mut qstr, base: *mut dentry) -> *mut dentry;
    pub fn lookup_one_positive_unlocked(idmap: *mut mnt_idmap, name: *mut qstr, base: *mut dentry) -> *mut dentry;
    pub fn lookup_one_positive_killable(idmap: *mut mnt_idmap, name: *mut qstr, base: *mut dentry) -> *mut dentry;
    pub fn start_creating(idmap: *mut mnt_idmap, parent: *mut dentry, name: *mut qstr) -> *mut dentry;
    pub fn start_removing(idmap: *mut mnt_idmap, parent: *mut dentry, name: *mut qstr) -> *mut dentry;
    pub fn start_creating_killable(idmap: *mut mnt_idmap, parent: *mut dentry, name: *mut qstr) -> *mut dentry;
    pub fn start_removing_killable(idmap: *mut mnt_idmap, parent: *mut dentry, name: *mut qstr) -> *mut dentry;
    pub fn start_creating_noperm(parent: *mut dentry, name: *mut qstr) -> *mut dentry;
    pub fn start_removing_noperm(parent: *mut dentry, name: *mut qstr) -> *mut dentry;
    pub fn start_creating_dentry(parent: *mut dentry, child: *mut dentry) -> *mut dentry;
    pub fn start_removing_dentry(parent: *mut dentry, child: *mut dentry) -> *mut dentry;
    pub fn vfs_lookup_open(parent: *mut path, last: *mut qstr, open_flag: i32, mode: umode_t) -> *mut file;
    pub fn end_dirop(child: *mut dentry);
    pub fn dget(child: *mut dentry);
    pub fn follow_down_one(path: *mut path) -> i32;
    pub fn follow_down(path: *mut path, flags: u32) -> i32;
    pub fn follow_up(path: *mut path) -> i32;
    pub fn start_renaming(rd: *mut renamedata, lookup_flags: i32, old_last: *mut qstr, new_last: *mut qstr) -> i32;
    pub fn start_renaming_dentry(rd: *mut renamedata, lookup_flags: i32, old_dentry: *mut dentry, new_last: *mut qstr) -> i32;
    pub fn start_renaming_two_dentries(rd: *mut renamedata, old_dentry: *mut dentry, new_dentry: *mut dentry) -> i32;
    pub fn end_renaming(rd: *mut renamedata);
    pub fn IS_POSIXACL(dir: *const inode) -> bool;
    pub fn current_umask() -> umode_t;
    pub fn nd_jump_link(path: *const path) -> i32;
    pub fn IS_ERR(ptr: *const dentry) -> bool;
    pub fn min(a: usize, b: usize) -> usize;
    pub fn unlikely(value: bool) -> bool;
}

#[inline]
pub unsafe fn end_removing_path(path: *const path, dentry: *mut dentry) {
    end_creating_path(path, dentry);
}

#[inline]
pub unsafe fn end_creating(child: *mut dentry) {
    end_dirop(child);
}

#[inline]
pub unsafe fn end_creating_keep(child: *mut dentry) -> *mut dentry {
    if !IS_ERR(child) {
        dget(child);
    }
    end_dirop(child);
    child
}

#[inline]
pub unsafe fn end_removing(child: *mut dentry) {
    end_dirop(child);
}

#[inline]
pub unsafe fn mode_strip_umask(dir: *const inode, mut mode: umode_t) -> umode_t {
    if !IS_POSIXACL(dir) && unsafe { ((*dir).i_sb.s_iflags & SB_I_NOUMASK) == 0 } {
        mode &= !current_umask();
    }
    mode
}

#[inline]
pub unsafe fn nd_terminate_link(name: *mut c_void, len: usize, maxlen: usize) {
    *(name as *mut c_char).add(min(len, maxlen)) = 0;
}

#[inline]
pub unsafe fn retry_estale(error: i64, flags: u32) -> bool {
    unlikely(error == -(ESTALE as i64) && (flags & LOOKUP_REVAL) == 0)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
