// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation:
// linux/module.h, linux/nfs_common.h, linux/nfs4.h

/*
 * We need to translate between nfs status return values and
 * the local errno values which may not be the same.
 */
#[repr(C)]
struct NfsErr {
    stat: i32,
    errno: i32,
}

static NFS_ERRTBL: &[NfsErr] = &[
    NfsErr { stat: NFS_OK, errno: 0 },
    NfsErr { stat: NFSERR_PERM, errno: -EPERM },
    NfsErr { stat: NFSERR_NOENT, errno: -ENOENT },
    NfsErr { stat: NFSERR_IO, errno: -EIO },
    NfsErr { stat: NFSERR_NXIO, errno: -ENXIO },
    NfsErr { stat: NFSERR_ACCES, errno: -EACCES },
    NfsErr { stat: NFSERR_EXIST, errno: -EEXIST },
    NfsErr { stat: NFSERR_XDEV, errno: -EXDEV },
    NfsErr { stat: NFSERR_NODEV, errno: -ENODEV },
    NfsErr { stat: NFSERR_NOTDIR, errno: -ENOTDIR },
    NfsErr { stat: NFSERR_ISDIR, errno: -EISDIR },
    NfsErr { stat: NFSERR_INVAL, errno: -EINVAL },
    NfsErr { stat: NFSERR_FBIG, errno: -EFBIG },
    NfsErr { stat: NFSERR_NOSPC, errno: -ENOSPC },
    NfsErr { stat: NFSERR_ROFS, errno: -EROFS },
    NfsErr { stat: NFSERR_MLINK, errno: -EMLINK },
    NfsErr { stat: NFSERR_NAMETOOLONG, errno: -ENAMETOOLONG },
    NfsErr { stat: NFSERR_NOTEMPTY, errno: -ENOTEMPTY },
    NfsErr { stat: NFSERR_DQUOT, errno: -EDQUOT },
    NfsErr { stat: NFSERR_STALE, errno: -ESTALE },
    NfsErr { stat: NFSERR_REMOTE, errno: -EREMOTE },
    #[cfg(feature = "EWFLUSH")]
    NfsErr { stat: NFSERR_WFLUSH, errno: -EWFLUSH },
    NfsErr { stat: NFSERR_BADHANDLE, errno: -EBADHANDLE },
    NfsErr { stat: NFSERR_NOT_SYNC, errno: -ENOTSYNC },
    NfsErr { stat: NFSERR_BAD_COOKIE, errno: -EBADCOOKIE },
    NfsErr { stat: NFSERR_NOTSUPP, errno: -ENOTSUPP },
    NfsErr { stat: NFSERR_TOOSMALL, errno: -ETOOSMALL },
    NfsErr { stat: NFSERR_SERVERFAULT, errno: -EREMOTEIO },
    NfsErr { stat: NFSERR_BADTYPE, errno: -EBADTYPE },
    NfsErr { stat: NFSERR_JUKEBOX, errno: -EJUKEBOX },
];

/**
 * nfs_stat_to_errno - convert an NFS status code to a local errno
 * @status: NFS status code to convert
 *
 * Returns a local errno value, or -EIO if the NFS status code is
 * not recognized.  This function is used jointly by NFSv2 and NFSv3.
 */
pub unsafe fn nfs_stat_to_errno(status: nfs_stat) -> i32 {
    for entry in NFS_ERRTBL {
        if entry.stat == status as i32 {
            return entry.errno;
        }
    }
    -EIO
}
// EXPORT_SYMBOL_GPL(nfs_stat_to_errno);

/*
 * We need to translate between nfs v4 status return values and
 * the local errno values which may not be the same.
 *
 * nfs4_errtbl_common[] is used before more specialized mappings
 * available in nfs4_errtbl[] or nfs4_errtbl_localio[].
 */
static NFS4_ERRTBL_COMMON: &[NfsErr] = &[
    NfsErr { stat: NFS4_OK, errno: 0 },
    NfsErr { stat: NFS4ERR_PERM, errno: -EPERM },
    NfsErr { stat: NFS4ERR_NOENT, errno: -ENOENT },
    NfsErr { stat: NFS4ERR_IO, errno: -EIO },
    NfsErr { stat: NFS4ERR_NXIO, errno: -ENXIO },
    NfsErr { stat: NFS4ERR_ACCESS, errno: -EACCES },
    NfsErr { stat: NFS4ERR_EXIST, errno: -EEXIST },
    NfsErr { stat: NFS4ERR_XDEV, errno: -EXDEV },
    NfsErr { stat: NFS4ERR_NOTDIR, errno: -ENOTDIR },
    NfsErr { stat: NFS4ERR_ISDIR, errno: -EISDIR },
    NfsErr { stat: NFS4ERR_INVAL, errno: -EINVAL },
    NfsErr { stat: NFS4ERR_FBIG, errno: -EFBIG },
    NfsErr { stat: NFS4ERR_NOSPC, errno: -ENOSPC },
    NfsErr { stat: NFS4ERR_ROFS, errno: -EROFS },
    NfsErr { stat: NFS4ERR_MLINK, errno: -EMLINK },
    NfsErr { stat: NFS4ERR_NAMETOOLONG, errno: -ENAMETOOLONG },
    NfsErr { stat: NFS4ERR_NOTEMPTY, errno: -ENOTEMPTY },
    NfsErr { stat: NFS4ERR_DQUOT, errno: -EDQUOT },
    NfsErr { stat: NFS4ERR_STALE, errno: -ESTALE },
    NfsErr { stat: NFS4ERR_BADHANDLE, errno: -EBADHANDLE },
    NfsErr { stat: NFS4ERR_BAD_COOKIE, errno: -EBADCOOKIE },
    NfsErr { stat: NFS4ERR_NOTSUPP, errno: -ENOTSUPP },
    NfsErr { stat: NFS4ERR_TOOSMALL, errno: -ETOOSMALL },
    NfsErr { stat: NFS4ERR_BADTYPE, errno: -EBADTYPE },
    NfsErr { stat: NFS4ERR_SYMLINK, errno: -ELOOP },
    NfsErr { stat: NFS4ERR_DEADLOCK, errno: -EDEADLK },
];

static NFS4_ERRTBL: &[NfsErr] = &[
    NfsErr { stat: NFS4ERR_SERVERFAULT, errno: -EREMOTEIO },
    NfsErr { stat: NFS4ERR_LOCKED, errno: -EAGAIN },
    NfsErr { stat: NFS4ERR_OP_ILLEGAL, errno: -EOPNOTSUPP },
    NfsErr { stat: NFS4ERR_NOXATTR, errno: -ENODATA },
    NfsErr { stat: NFS4ERR_XATTR2BIG, errno: -E2BIG },
];

/*
 * Convert an NFS error code to a local one.
 * This one is used by NFSv4.
 */
pub fn nfs4_stat_to_errno(stat: i32) -> i32 {
    /* First check nfs4_errtbl_common */
    for entry in NFS4_ERRTBL_COMMON {
        if entry.stat == stat { return entry.errno; }
    }
    /* Then check nfs4_errtbl */
    for entry in NFS4_ERRTBL {
        if entry.stat == stat { return entry.errno; }
    }
    if stat <= 10000 || stat > 10100 {
        /* The server is looney tunes. */
        return -EREMOTEIO;
    }
    /* If we cannot translate the error, the recovery routines should
     * handle it.
     * Note: remaining NFSv4 error codes have values > 10000, so should
     * not conflict with native Linux error codes.
     */
    -stat
}
// EXPORT_SYMBOL_GPL(nfs4_stat_to_errno);

/*
 * This table is useful for conversion from local errno to NFS error.
 * It provides more logically correct mappings for use with LOCALIO
 * (which is focused on converting from errno to NFS status).
 */
static NFS4_ERRTBL_LOCALIO: &[NfsErr] = &[
    /* Map errors differently than nfs4_errtbl */
    NfsErr { stat: NFS4ERR_IO, errno: -EREMOTEIO },
    NfsErr { stat: NFS4ERR_DELAY, errno: -EAGAIN },
    NfsErr { stat: NFS4ERR_FBIG, errno: -E2BIG },
    /* Map errors not handled by nfs4_errtbl */
    NfsErr { stat: NFS4ERR_STALE, errno: -EBADF },
    NfsErr { stat: NFS4ERR_STALE, errno: -EOPENSTALE },
    NfsErr { stat: NFS4ERR_DELAY, errno: -ETIMEDOUT },
    NfsErr { stat: NFS4ERR_DELAY, errno: -ERESTARTSYS },
    NfsErr { stat: NFS4ERR_DELAY, errno: -ENOMEM },
    NfsErr { stat: NFS4ERR_IO, errno: -ETXTBSY },
    NfsErr { stat: NFS4ERR_IO, errno: -EBUSY },
    NfsErr { stat: NFS4ERR_SERVERFAULT, errno: -ESERVERFAULT },
    NfsErr { stat: NFS4ERR_SERVERFAULT, errno: -ENFILE },
    NfsErr { stat: NFS4ERR_IO, errno: -EUCLEAN },
    NfsErr { stat: NFS4ERR_PERM, errno: -ENOKEY },
];

/*
 * Convert an errno to an NFS error code for LOCALIO.
 */
pub fn nfs_localio_errno_to_nfs4_stat(errno: i32) -> u32 {
    /* First check nfs4_errtbl_common */
    for entry in NFS4_ERRTBL_COMMON {
        if entry.errno == errno { return entry.stat as u32; }
    }
    /* Then check nfs4_errtbl_localio */
    for entry in NFS4_ERRTBL_LOCALIO {
        if entry.errno == errno { return entry.stat as u32; }
    }
    /* If we cannot translate the error, the recovery routines should
     * handle it.
     * Note: remaining NFSv4 error codes have values > 10000, so should
     * not conflict with native Linux error codes.
     */
    NFS4ERR_SERVERFAULT as u32
}
// EXPORT_SYMBOL_GPL(nfs_localio_errno_to_nfs4_stat);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
