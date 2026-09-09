// SPDX-License-Identifier: GPL-2.0-or-later
/* miscellaneous bits
 *
 * Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * convert an AFS abort code to a Linux error number
 */
pub unsafe fn afs_abort_to_error(abort_code: u32) -> i32 {
    match abort_code {
        /* Low errno codes inserted into abort namespace */
        13 => -EACCES,
        27 => -EFBIG,
        30 => -EROFS,

        /* VICE "special error" codes; 101 - 111 */
        VSALVAGE => -EIO,
        VNOVNODE => -ENOENT,
        VNOVOL => -ENOMEDIUM,
        VVOLEXISTS => -EEXIST,
        VNOSERVICE => -EIO,
        VOFFLINE => -ENOENT,
        VONLINE => -EEXIST,
        VDISKFULL => -ENOSPC,
        VOVERQUOTA => -EDQUOT,
        VBUSY => -EBUSY,
        VMOVED => -ENXIO,

        /* Volume Location server errors */
        AFSVL_IDEXIST => -EEXIST,
        AFSVL_IO => -EREMOTEIO,
        AFSVL_NAMEEXIST => -EEXIST,
        AFSVL_CREATEFAIL => -EREMOTEIO,
        AFSVL_NOENT | AFSVL_EMPTY | AFSVL_ENTDELETED => -ENOMEDIUM,
        AFSVL_BADNAME | AFSVL_BADINDEX | AFSVL_BADVOLTYPE | AFSVL_BADSERVER |
        AFSVL_BADPARTITION | AFSVL_BADREFCOUNT | AFSVL_SIZEEXCEEDED |
        AFSVL_BADENTRY | AFSVL_BADVOLIDBUMP | AFSVL_IDALREADYHASHED |
        AFSVL_BADRELLOCKTYPE | AFSVL_BADSERVERFLAG => -EINVAL,
        AFSVL_REPSFULL => -EFBIG,
        AFSVL_NOREPSERVER | AFSVL_RWNOTFOUND => -ENOENT,
        AFSVL_DUPREPSERVER => -EEXIST,
        AFSVL_ENTRYLOCKED => -EBUSY,
        AFSVL_BADVOLOPER => -EBADRQC,
        AFSVL_RERELEASE => -EREMOTEIO,
        AFSVL_PERM => -EACCES,
        AFSVL_NOMEM => -EREMOTEIO,

        /* Unified AFS error table */
        UAEPERM => -EPERM,
        UAENOENT => -ENOENT,
        UAEAGAIN => -EAGAIN,
        UAEACCES => -EACCES,
        UAEBUSY => -EBUSY,
        UAEEXIST => -EEXIST,
        UAENOTDIR => -ENOTDIR,
        UAEISDIR => -EISDIR,
        UAEFBIG => -EFBIG,
        UAENOSPC => -ENOSPC,
        UAEROFS => -EROFS,
        UAEMLINK => -EMLINK,
        UAEDEADLK => -EDEADLK,
        UAENAMETOOLONG => -ENAMETOOLONG,
        UAENOLCK => -ENOLCK,
        UAENOTEMPTY => -ENOTEMPTY,
        UAELOOP => -ELOOP,
        UAEOVERFLOW => -EOVERFLOW,
        UAENOMEDIUM => -ENOMEDIUM,
        UAEDQUOT => -EDQUOT,

        /* RXKAD abort codes; from include/rxrpc/packet.h. ET "RXK" == 0x1260B00 */
        RXKADINCONSISTENCY | RXKADPACKETSHORT | RXKADOUTOFSEQUENCE => -EPROTO,
        RXKADLEVELFAIL | RXKADTICKETLEN | RXKADNOAUTH | RXKADBADKEY |
        RXKADBADTICKET | RXKADUNKNOWNKEY | RXKADSEALEDINCON | RXKADDATALEN |
        RXKADILLEGALLEVEL => -EKEYREJECTED,
        RXKADEXPIRED => -EKEYEXPIRED,

        RXGK_INCONSISTENCY | RXGK_PACKETSHORT | RXGK_BADCHALLENGE => -EPROTO,
        RXGK_SEALEDINCON | RXGK_NOTAUTH | RXGK_BADLEVEL | RXGK_BADKEYNO |
        RXGK_NOTRXGK | RXGK_UNSUPPORTED | RXGK_GSSERROR => -EKEYREJECTED,
        RXGK_EXPIRED => -EKEYEXPIRED,
        // Conditional in the C source: RXGK_BADETYPE, RXGK_BADTOKEN,
        // RXGK_DATALEN, and RXGK_BADQOP are supplied when enabled by the build.

        KRB5_PROG_KEYTYPE_NOSUPP => -ENOPKG,
        RXGEN_OPCODE | RX_INVALID_OPERATION => -ENOTSUPP,
        _ => -EREMOTEIO,
    }
}

/*
 * Select the error to report from a set of errors.
 */
pub unsafe fn afs_prioritise_error(e: *mut afs_error, error: i32, abort_code: u32) {
    match error {
        0 => {
            (*e).aborted = false;
            (*e).error = 0;
            return;
        }
        _ if (*e).error == -ETIMEDOUT || (*e).error == -ETIME => return,
        _ => {}
    }

    if (error == -ETIMEDOUT || error == -ETIME) &&
       ((*e).error == -ENOMEM || (*e).error == -ENONET) { return; }
    if (error == -ENOMEM || error == -ENONET) && (*e).error == -ERFKILL { return; }
    if error == -ERFKILL && (*e).error == -EADDRNOTAVAIL { return; }
    if error == -EADDRNOTAVAIL && (*e).error == -ENETUNREACH { return; }
    if error == -ENETUNREACH && (*e).error == -EHOSTUNREACH { return; }
    if error == -EHOSTUNREACH && (*e).error == -EHOSTDOWN { return; }
    if error == -EHOSTDOWN && (*e).error == -ECONNREFUSED { return; }
    if error == -ECONNREFUSED && (*e).error == -ECONNRESET { return; }
    if error == -ECONNRESET && (*e).responded { return; }
    if error != -ECONNABORTED && error != -ENETRESET {
        (*e).error = error;
        (*e).aborted = false;
        return;
    }
    match error {
        -ECONNABORTED => {
            (*e).error = afs_abort_to_error(abort_code);
            (*e).aborted = true;
            (*e).responded = true;
        }
        -ENETRESET => {
            (*e).aborted = false;
            (*e).responded = true;
            (*e).error = error;
        }
        _ => {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
