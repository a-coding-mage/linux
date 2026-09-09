// SPDX-License-Identifier: GPL-2.0
/*
 * Inode operations for Coda filesystem
 * Original version: (C) 1996 P. Braam and M. Callahan
 * Rewritten for Linux 2.1. (C) 1997 Carnegie Mellon University
 *
 * Carnegie Mellon encourages users to contribute improvements to
 * the Coda project. Contact Peter Braam (coda@cs.cmu.edu).
 */

// Linux kernel headers and local Coda headers are supplied by other files.

/* initialize the debugging variables */
pub static mut coda_fake_statfs: ::core::ffi::c_int = 0;

/* print a fid */
pub unsafe fn coda_f2s(f: *mut CodaFid) -> *mut ::core::ffi::c_char {
    static mut S: [::core::ffi::c_char; 60] = [0; 60];
    unsafe extern "C" {
        fn sprintf(
            s: *mut ::core::ffi::c_char,
            format: *const ::core::ffi::c_char,
            ...,
        ) -> ::core::ffi::c_int;
    }
    static FORMAT: &[u8] = b"(%08x.%08x.%08x.%08x)\0";
    sprintf(
        S.as_mut_ptr(),
        FORMAT.as_ptr() as *const ::core::ffi::c_char,
        (*f).opaque[0],
        (*f).opaque[1],
        (*f).opaque[2],
        (*f).opaque[3],
    );
    S.as_mut_ptr()
}

/* recognize special .CONTROL name */
pub unsafe fn coda_iscontrol(name: *const ::core::ffi::c_char, length: usize) -> ::core::ffi::c_int {
    unsafe extern "C" {
        fn strncmp(
            lhs: *const ::core::ffi::c_char,
            rhs: *const ::core::ffi::c_char,
            count: usize,
        ) -> ::core::ffi::c_int;
    }
    ((CODA_CONTROLLEN == length)
        && (strncmp(name, CODA_CONTROL.as_ptr() as *const ::core::ffi::c_char, CODA_CONTROLLEN) == 0)) as ::core::ffi::c_int
}

pub fn coda_flags_to_cflags(flags: u16) -> u16 {
    let mut coda_flags: u16 = 0;

    if (flags & O_ACCMODE) == O_RDONLY { coda_flags |= C_O_READ; }
    if (flags & O_ACCMODE) == O_RDWR { coda_flags |= C_O_READ | C_O_WRITE; }
    if (flags & O_ACCMODE) == O_WRONLY { coda_flags |= C_O_WRITE; }
    if (flags & O_TRUNC) != 0 { coda_flags |= C_O_TRUNC; }
    if (flags & O_CREAT) != 0 { coda_flags |= C_O_CREAT; }
    if (flags & O_EXCL) != 0 { coda_flags |= C_O_EXCL; }
    coda_flags
}

unsafe fn coda_to_timespec64(ts: coda_timespec) -> timespec64 {
    timespec64 { tv_sec: ts.tv_sec, tv_nsec: ts.tv_nsec }
}

unsafe fn timespec64_to_coda(ts64: timespec64) -> coda_timespec {
    coda_timespec { tv_sec: ts64.tv_sec, tv_nsec: ts64.tv_nsec }
}

/* utility functions below */
pub unsafe fn coda_inode_type(attr: *mut coda_vattr) -> umode_t {
    match (*attr).va_type {
        C_VREG => S_IFREG,
        C_VDIR => S_IFDIR,
        C_VLNK => S_IFLNK,
        C_VNON => 0,
        _ => 0,
    }
}

pub unsafe fn coda_vattr_to_iattr(inode: *mut inode, attr: *mut coda_vattr) {
    /* inode's i_flags, i_ino are set by iget
     * XXX: is this all we need ??
     */
    let inode_type = coda_inode_type(attr);
    (*inode).i_mode |= inode_type;

    if (*attr).va_mode != (-1i16 as u16) { (*inode).i_mode = (*attr).va_mode | inode_type; }
    if (*attr).va_uid != -1 { (*inode).i_uid = make_kuid(&init_user_ns, (*attr).va_uid as uid_t); }
    if (*attr).va_gid != -1 { (*inode).i_gid = make_kgid(&init_user_ns, (*attr).va_gid as gid_t); }
    if (*attr).va_nlink != -1 { set_nlink(inode, (*attr).va_nlink); }
    if (*attr).va_size != -1 { (*inode).i_size = (*attr).va_size; }
    if (*attr).va_size != -1 { (*inode).i_blocks = ((*attr).va_size + 511) >> 9; }
    if (*attr).va_atime.tv_sec != -1 { inode_set_atime_to_ts(inode, coda_to_timespec64((*attr).va_atime)); }
    if (*attr).va_mtime.tv_sec != -1 { inode_set_mtime_to_ts(inode, coda_to_timespec64((*attr).va_mtime)); }
    if (*attr).va_ctime.tv_sec != -1 { inode_set_ctime_to_ts(inode, coda_to_timespec64((*attr).va_ctime)); }
}

/*
 * BSD sets attributes that need not be modified to -1.
 * Linux uses the valid field to indicate what should be looked at.  The BSD type field needs to be deduced from linux mode.
 * So we have to do some translations here.
 */
pub unsafe fn coda_iattr_to_vattr(iattr: *mut iattr, vattr: *mut coda_vattr) {
    let valid: u32;
    (*vattr).va_mode = -1i16 as u16;
    (*vattr).va_uid = -1 as vuid_t;
    (*vattr).va_gid = -1 as vgid_t;
    (*vattr).va_size = -1 as off_t;
    (*vattr).va_atime.tv_sec = -1i64;
    (*vattr).va_atime.tv_nsec = -1isize;
    (*vattr).va_mtime.tv_sec = -1i64;
    (*vattr).va_mtime.tv_nsec = -1isize;
    (*vattr).va_ctime.tv_sec = -1i64;
    (*vattr).va_ctime.tv_nsec = -1isize;
    (*vattr).va_type = C_VNON;
    (*vattr).va_fileid = -1;
    (*vattr).va_gen = -1;
    (*vattr).va_bytes = -1;
    (*vattr).va_nlink = -1;
    (*vattr).va_blocksize = -1;
    (*vattr).va_rdev = -1;
    (*vattr).va_flags = 0;

    /* determine the type (the original C implementation keeps this code disabled) */
    valid = (*iattr).ia_valid;
    if (valid & ATTR_MODE) != 0 { (*vattr).va_mode = (*iattr).ia_mode; }
    if (valid & ATTR_UID) != 0 { (*vattr).va_uid = from_kuid(&init_user_ns, (*iattr).ia_uid) as vuid_t; }
    if (valid & ATTR_GID) != 0 { (*vattr).va_gid = from_kgid(&init_user_ns, (*iattr).ia_gid) as vgid_t; }
    if (valid & ATTR_SIZE) != 0 { (*vattr).va_size = (*iattr).ia_size; }
    if (valid & ATTR_ATIME) != 0 { (*vattr).va_atime = timespec64_to_coda((*iattr).ia_atime); }
    if (valid & ATTR_MTIME) != 0 { (*vattr).va_mtime = timespec64_to_coda((*iattr).ia_mtime); }
    if (valid & ATTR_CTIME) != 0 { (*vattr).va_ctime = timespec64_to_coda((*iattr).ia_ctime); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
