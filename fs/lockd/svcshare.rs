// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/lockd/svcshare.c
 *
 * Management of DOS shares.
 *
 * Copyright (C) 1996 Olaf Kirch <okir@monad.swb.de>
 */

// linux/time.h, linux/unistd.h, linux/string.h, linux/slab.h,
// linux/sunrpc/clnt.h, linux/sunrpc/svc.h, lockd.h, and share.h
// provide the external types, constants, functions, and macros used here.

#[inline]
unsafe fn nlm_cmp_owner(share: *mut lockd_share, oh: *mut xdr_netobj) -> bool {
    (*share).s_owner.len == (*oh).len
        && libc::memcmp(
            (*share).s_owner.data as *const libc::c_void,
            (*oh).data as *const libc::c_void,
            (*oh).len as usize,
        ) == 0
}

/*
 * Recompute s_access / s_mode as the union of every (access, deny) pair
 * whose bit is currently set in s_access_deny_bmap.
 */
unsafe fn nlm_recompute_share(share: *mut lockd_share) {
    let mut new_access: u32 = 0;
    let mut new_mode: u32 = 0;
    let mut i: libc::c_uint = 0;

    while i < 16 {
        if (*share).s_access_deny_bmap & (1u32 << i) != 0 {
            new_access |= i >> 2;
            new_mode |= i & 3;
        }
        i += 1;
    }
    (*share).s_access = new_access;
    (*share).s_mode = new_mode;
}

/**
 * nlmsvc_share_file - create a share
 * @host: Network client peer
 * @file: File to be shared
 * @oh: Share owner handle
 * @access: Requested access mode
 * @mode: Requested file sharing mode
 *
 * Returns an NLM status code.
 */
pub unsafe fn nlmsvc_share_file(
    host: *mut nlm_host,
    file: *mut nlm_file,
    oh: *mut xdr_netobj,
    access: u32,
    mode: u32,
) -> __be32 {
    let mut share: *mut lockd_share;
    let ohdata: *mut u8;

    if nlmsvc_file_cannot_lock(file) {
        return nlm_lck_denied_nolocks;
    }

    share = (*file).f_shares;
    while !share.is_null() {
        if (*share).s_host == host && nlm_cmp_owner(share, oh) {
            break;
        }
        if (access & (*share).s_mode) != 0 || (mode & (*share).s_access) != 0 {
            return nlm_lck_denied;
        }
        share = (*share).s_next;
    }

    if share.is_null() {
        share = kmalloc(
            core::mem::size_of::<lockd_share>() + (*oh).len as usize,
            GFP_KERNEL,
        ) as *mut lockd_share;
        if share.is_null() {
            return nlm_lck_denied_nolocks;
        }

        /* Copy owner handle */
        ohdata = (share.add(1)) as *mut u8;
        libc::memcpy(
            ohdata as *mut libc::c_void,
            (*oh).data as *const libc::c_void,
            (*oh).len as usize,
        );

        (*share).s_file = file;
        (*share).s_host = host;
        (*share).s_owner.data = ohdata;
        (*share).s_owner.len = (*oh).len;
        (*share).s_access_deny_bmap = 0;
        (*share).s_next = (*file).f_shares;
        (*file).f_shares = share;
    }

    (*share).s_access_deny_bmap |= LOCKD_FSH_BIT(access, mode);
    nlm_recompute_share(share);
    nlm_granted
}

/**
 * nlmsvc_unshare_file - delete a share
 * @host: Network client peer
 * @file: File to be unshared
 * @oh: Share owner handle
 * @access: Access mode of the SHARE being released
 * @mode: Deny mode of the SHARE being released
 *
 * Returns an NLM status code.
 */
pub unsafe fn nlmsvc_unshare_file(
    host: *mut nlm_host,
    file: *mut nlm_file,
    oh: *mut xdr_netobj,
    access: u32,
    mode: u32,
) -> __be32 {
    let mut share: *mut lockd_share;
    let mut shpp: *mut *mut lockd_share;

    if nlmsvc_file_cannot_lock(file) {
        return nlm_lck_denied_nolocks;
    }

    shpp = &mut (*file).f_shares;
    while {
        share = *shpp;
        !share.is_null()
    } {
        if (*share).s_host == host && nlm_cmp_owner(share, oh) {
            (*share).s_access_deny_bmap &= !LOCKD_FSH_BIT(access, mode);
            nlm_recompute_share(share);
            if (*share).s_access_deny_bmap == 0 {
                *shpp = (*share).s_next;
                kfree(share as *mut libc::c_void);
            }
            return nlm_granted;
        }
        shpp = &mut (*share).s_next;
    }

    /* X/Open spec says return success even if there was no
     * corresponding share. */
    nlm_granted
}

/*
 * Traverse all shares for a given file, and delete
 * those owned by the given (type of) host
 */
pub unsafe fn nlmsvc_traverse_shares(
    host: *mut nlm_host,
    file: *mut nlm_file,
    match_fn: nlm_host_match_fn_t,
) {
    let mut share: *mut lockd_share;
    let mut shpp: *mut *mut lockd_share;

    shpp = &mut (*file).f_shares;
    while {
        share = *shpp;
        !share.is_null()
    } {
        if match_fn((*share).s_host, host) {
            *shpp = (*share).s_next;
            kfree(share as *mut libc::c_void);
            continue;
        }
        shpp = &mut (*share).s_next;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
