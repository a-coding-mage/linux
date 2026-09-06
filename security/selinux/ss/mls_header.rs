/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Multi-level security (MLS) policy operations.
 *
 * Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
 */

/*
 * Updated: Trusted Computer Solutions, Inc. <dgoeddel@trustedcs.com>
 *          Support for enhanced MLS infrastructure.
 *          Copyright (C) 2004-2006 Trusted Computer Solutions, Inc.
 *
 * Updated: Hewlett-Packard <paul@paul-moore.com>
 *          Added support to import/export the MLS label from NetLabel
 *          Copyright (X) Hewlett-Packard Development Company, L.P., 2006
 */

/* C dependencies: <linux/jhash.h>, "context.h", "ebitmap.h", "policydb.h". */

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    pub fn mls_compute_context_len(p: *mut policydb, context: *mut context) -> c_int;
    pub fn mls_sid_to_context(p: *mut policydb, context: *mut context, scontext: *mut *mut c_char);
    pub fn mls_context_isvalid(p: *const policydb, c: *const context) -> bool;
    pub fn mls_range_isvalid(p: *const policydb, r: *const mls_range) -> bool;
    pub fn mls_level_isvalid(p: *const policydb, l: *const mls_level) -> bool;

    pub fn mls_context_to_sid(
        p: *mut policydb,
        oldc: c_char,
        scontext: *mut c_char,
        context: *mut context,
        s: *mut sidtab,
        def_sid: u32,
    ) -> c_int;

    pub fn mls_from_string(
        p: *mut policydb,
        str: *mut c_char,
        context: *mut context,
        gfp_mask: gfp_t,
    ) -> c_int;

    pub fn mls_range_set(context: *mut context, range: *mut mls_range) -> c_int;

    pub fn mls_convert_context(
        oldp: *mut policydb,
        newp: *mut policydb,
        oldc: *mut context,
        newc: *mut context,
    ) -> c_int;

    pub fn mls_compute_sid(
        p: *mut policydb,
        scontext: *mut context,
        tcontext: *mut context,
        tclass: u16,
        specified: u32,
        newcontext: *mut context,
        sock: bool,
    ) -> c_int;

    pub fn mls_setup_user_range(
        p: *mut policydb,
        fromcon: *mut context,
        user: *mut user_datum,
        usercon: *mut context,
    ) -> c_int;
}

/* CONFIG_NETLABEL declarations when enabled. */
#[cfg(CONFIG_NETLABEL)]
unsafe extern "C" {
    pub fn mls_export_netlbl_lvl(
        p: *mut policydb,
        context: *mut context,
        secattr: *mut netlbl_lsm_secattr,
    );
    pub fn mls_import_netlbl_lvl(
        p: *mut policydb,
        context: *mut context,
        secattr: *mut netlbl_lsm_secattr,
    );
    pub fn mls_export_netlbl_cat(
        p: *mut policydb,
        context: *mut context,
        secattr: *mut netlbl_lsm_secattr,
    ) -> c_int;
    pub fn mls_import_netlbl_cat(
        p: *mut policydb,
        context: *mut context,
        secattr: *mut netlbl_lsm_secattr,
    ) -> c_int;
}

/* Fallback static inline definitions when CONFIG_NETLABEL is disabled. */
#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn mls_export_netlbl_lvl(
    _p: *mut policydb,
    _context: *mut context,
    _secattr: *mut netlbl_lsm_secattr,
) {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn mls_import_netlbl_lvl(
    _p: *mut policydb,
    _context: *mut context,
    _secattr: *mut netlbl_lsm_secattr,
) {
    return;
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn mls_export_netlbl_cat(
    _p: *mut policydb,
    _context: *mut context,
    _secattr: *mut netlbl_lsm_secattr,
) -> c_int {
    return -(ENOMEM as c_int);
}

#[cfg(not(CONFIG_NETLABEL))]
#[inline]
pub unsafe fn mls_import_netlbl_cat(
    _p: *mut policydb,
    _context: *mut context,
    _secattr: *mut netlbl_lsm_secattr,
) -> c_int {
    return -(ENOMEM as c_int);
}

#[inline]
pub unsafe fn mls_range_hash(r: *const mls_range, mut hash: u32) -> u32 {
    hash = unsafe { jhash_2words((*r).level[0].sens, (*r).level[1].sens, hash) };
    hash = unsafe { ebitmap_hash(&raw const (*r).level[0].cat, hash) };
    hash = unsafe { ebitmap_hash(&raw const (*r).level[1].cat, hash) };
    return hash;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
