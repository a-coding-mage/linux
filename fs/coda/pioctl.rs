// SPDX-License-Identifier: GPL-2.0
/*
 * Pioctl operations for Coda.
 * Original version: (C) 1996 Peter Braam
 * Rewritten for Linux 2.1: (C) 1997 Carnegie Mellon University
 *
 * Carnegie Mellon encourages users of this code to contribute improvements
 * to the Coda project. Contact Peter Braam <coda@cs.cmu.edu>.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Linux/kernel and Coda declarations are supplied by the surrounding sources.
#[repr(C)]
pub struct MntIdmap { _private: [u8; 0] }
#[repr(C)]
pub struct Inode { _private: [u8; 0] }
#[repr(C)]
pub struct File { _private: [u8; 0] }
#[repr(C)]
pub struct SuperBlock { _private: [u8; 0] }
#[repr(C)]
pub struct Dentry { _private: [u8; 0] }
#[repr(C)]
pub struct Path { pub mnt: *mut c_void, pub dentry: *mut Dentry }
#[repr(C)]
pub struct CodaInodeInfo { pub c_fid: Fid }
#[repr(C)]
pub struct Fid { _private: [u8; 0] }

// The complete definition is provided by linux/coda.h in the containing build.
#[repr(C)]
pub struct PioctlData {
    pub path: *const c_char,
    pub follow: c_int,
    _remaining: [u8; 0],
}

#[repr(C)]
pub struct InodeOperations {
    pub permission: Option<unsafe extern "C" fn(*mut MntIdmap, *mut Inode, c_int) -> c_int>,
    pub setattr: Option<unsafe extern "C" fn(*mut MntIdmap, *mut Inode, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct FileOperations {
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut File, c_uint, c_ulong) -> c_long>,
    pub llseek: Option<unsafe extern "C" fn(*mut File, c_long, c_int) -> c_long>,
}

extern "C" {
    fn coda_setattr(idmap: *mut MntIdmap, inode: *mut Inode, attr: *mut c_void) -> c_int;
    fn noop_llseek(file: *mut File, offset: c_long, whence: c_int) -> c_long;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn user_path_at(dfd: c_int, name: *const c_char, flags: c_uint, path: *mut Path) -> c_int;
    fn path_put(path: *mut Path);
    fn file_inode(file: *mut File) -> *mut Inode;
    fn d_inode(dentry: *mut Dentry) -> *mut Inode;
    fn coda_inode_sb(inode: *mut Inode) -> *mut SuperBlock;
    fn itoc(inode: *mut Inode) -> *mut CodaInodeInfo;
    fn venus_pioctl(sb: *mut SuperBlock, fid: *mut Fid, cmd: c_uint,
                    data: *mut PioctlData) -> c_long;
}

const MAY_EXEC: c_int = 1 << 2;
const EACCES: c_int = 13;
const EINVAL: c_long = 22;
const AT_FDCWD: c_int = -100;
const LOOKUP_FOLLOW: c_uint = 1 << 0;

/* pioctl ops */
unsafe extern "C" fn coda_ioctl_permission(
    _idmap: *mut MntIdmap,
    _inode: *mut Inode,
    mask: c_int,
) -> c_int {
    if mask & MAY_EXEC != 0 { -EACCES } else { 0 }
}

unsafe extern "C" fn coda_pioctl(
    filp: *mut File,
    cmd: c_uint,
    user_data: c_ulong,
) -> c_long {
    let mut path: Path = core::mem::zeroed();
    let mut error: c_long;
    let mut data: PioctlData = core::mem::zeroed();
    let inode: *mut Inode = file_inode(filp);
    let mut target_inode: *mut Inode = core::ptr::null_mut();
    let cnp: *mut CodaInodeInfo;

    /* get the Pioctl data arguments from user space */
    if copy_from_user(
        &mut data as *mut PioctlData as *mut c_void,
        user_data as *const c_void,
        core::mem::size_of::<PioctlData>(),
    ) != 0 {
        return -EINVAL;
    }

    /*
     * Look up the pathname. Note that the pathname is in
     * user memory, and namei takes care of this
     */
    error = user_path_at(
        AT_FDCWD,
        data.path,
        if data.follow != 0 { LOOKUP_FOLLOW } else { 0 },
        &mut path,
    ) as c_long;
    if error != 0 { return error; }

    target_inode = d_inode(path.dentry);

    /* return if it is not a Coda inode */
    if coda_inode_sb(target_inode) != coda_inode_sb(inode) {
        error = -EINVAL;
        path_put(&mut path);
        return error;
    }

    /* now proceed to make the upcall */
    cnp = itoc(target_inode);
    error = venus_pioctl(coda_inode_sb(inode), &mut (*cnp).c_fid, cmd, &mut data);
    path_put(&mut path);
    error
}

/* exported from this file */
#[no_mangle]
pub static coda_ioctl_inode_operations: InodeOperations = InodeOperations {
    permission: Some(coda_ioctl_permission),
    setattr: Some(coda_setattr),
};

#[no_mangle]
pub static coda_ioctl_operations: FileOperations = FileOperations {
    unlocked_ioctl: Some(coda_pioctl),
    llseek: Some(noop_llseek),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
