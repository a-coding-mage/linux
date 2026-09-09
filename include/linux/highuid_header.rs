/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: linux/types.h

/*
 * general notes:
 *
 * CONFIG_UID16 is defined if the given architecture needs to
 * support backwards compatibility for old system calls.
 *
 * kernel code should use uid_t and gid_t at all times when dealing with
 * kernel-private data.
 *
 * old_uid_t and old_gid_t should only be different if CONFIG_UID16 is
 * defined, else the platform should provide dummy typedefs for them such
 * that they are equivalent to __kernel_{u,g}id_t.
 *
 * uid16_t and gid16_t are used on all architectures. (when dealing
 * with structures hard coded to 16 bits, such as in filesystems)
 */

/*
 * This is the "overflow" UID and GID. They are used to signify uid/gid
 * overflow to old programs when they request uid/gid information but are
 * using the old 16 bit interfaces.
 * When you run a libc5 program, it will think that all highuid files or
 * processes are owned by this uid/gid.
 * The idea is that it's better to do so than possibly return 0 in lieu of
 * 65536, etc.
 */

unsafe extern "C" {
    pub static mut overflowuid: core::ffi::c_int;
    pub static mut overflowgid: core::ffi::c_int;

    pub fn __bad_uid();
    pub fn __bad_gid();
}

pub const DEFAULT_OVERFLOWUID: i32 = 65534;
pub const DEFAULT_OVERFLOWGID: i32 = 65534;

// CONFIG_UID16 conditional declarations and macros.
#[cfg(CONFIG_UID16)]
macro_rules! high2lowuid {
    ($uid:expr) => {
        if ($uid) & !0xFFFF != 0 {
            overflowuid as old_uid_t
        } else {
            ($uid) as old_uid_t
        }
    };
}

#[cfg(CONFIG_UID16)]
macro_rules! high2lowgid {
    ($gid:expr) => {
        if ($gid) & !0xFFFF != 0 {
            overflowgid as old_gid_t
        } else {
            ($gid) as old_gid_t
        }
    };
}

#[cfg(CONFIG_UID16)]
macro_rules! low2highuid {
    ($uid:expr) => {
        if ($uid) == (-1i32 as old_uid_t) {
            -1i32 as uid_t
        } else {
            ($uid) as uid_t
        }
    };
}

#[cfg(CONFIG_UID16)]
macro_rules! low2highgid {
    ($gid:expr) => {
        if ($gid) == (-1i32 as old_gid_t) {
            -1i32 as gid_t
        } else {
            ($gid) as gid_t
        }
    };
}

#[cfg(CONFIG_UID16)]
macro_rules! __convert_uid {
    ($size:expr, $uid:expr) => {
        if $size >= core::mem::size_of_val(&$uid) {
            ($uid)
        } else {
            high2lowuid!($uid)
        }
    };
}

#[cfg(CONFIG_UID16)]
macro_rules! __convert_gid {
    ($size:expr, $gid:expr) => {
        if $size >= core::mem::size_of_val(&$gid) {
            ($gid)
        } else {
            high2lowgid!($gid)
        }
    };
}

#[cfg(not(CONFIG_UID16))]
macro_rules! __convert_uid {
    ($size:expr, $uid:expr) => { ($uid) };
}

#[cfg(not(CONFIG_UID16))]
macro_rules! __convert_gid {
    ($size:expr, $gid:expr) => { ($gid) };
}

/* uid/gid input should be always 32bit uid_t */
macro_rules! SET_UID {
    ($var:expr, $uid:expr) => {{
        ($var) = __convert_uid!(core::mem::size_of_val(&$var), $uid);
    }};
}

macro_rules! SET_GID {
    ($var:expr, $gid:expr) => {{
        ($var) = __convert_gid!(core::mem::size_of_val(&$var), $gid);
    }};
}

/*
 * Everything below this line is needed on all architectures, to deal with
 * filesystems that only store 16 bits of the UID/GID, etc.
 */

/*
 * This is the UID and GID that will get written to disk if a filesystem
 * only supports 16-bit UIDs and the kernel has a high UID/GID to write
 */
unsafe extern "C" {
    pub static mut fs_overflowuid: core::ffi::c_int;
    pub static mut fs_overflowgid: core::ffi::c_int;
}

pub const DEFAULT_FS_OVERFLOWUID: i32 = 65534;
pub const DEFAULT_FS_OVERFLOWGID: i32 = 65534;

/*
 * Since these macros are used in architectures that only need limited
 * 16-bit UID back compatibility, we won't use old_uid_t and old_gid_t
 */
macro_rules! fs_high2lowuid {
    ($uid:expr) => {
        if ($uid) & !0xFFFF != 0 {
            fs_overflowuid as uid16_t
        } else {
            ($uid) as uid16_t
        }
    };
}

macro_rules! fs_high2lowgid {
    ($gid:expr) => {
        if ($gid) & !0xFFFF != 0 {
            fs_overflowgid as gid16_t
        } else {
            ($gid) as gid16_t
        }
    };
}

macro_rules! low_16_bits {
    ($x:expr) => { ($x) & 0xFFFF };
}

macro_rules! high_16_bits {
    ($x:expr) => { (($x) & 0xFFFF0000) >> 16 };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
