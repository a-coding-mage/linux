// SPDX-License-Identifier: GPL-2.0
/*
 * Implementation of the multi-level security (MLS) policy.
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
 *          Copyright (C) Hewlett-Packard Development Company, L.P., 2006
 */

/* Rust translation of dependencies from:
 * <linux/kernel.h>, <linux/slab.h>, <linux/string.h>, <linux/errno.h>,
 * <net/netlabel.h>, "sidtab.h", "mls.h", "policydb.h", "services.h"
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

/*
 * Return the length in bytes for the MLS fields of the
 * security context string representation of `context'.
 */
pub unsafe fn mls_compute_context_len(p: *mut policydb, context: *mut context) -> c_int {
    let mut i: c_int = 0;
    let mut l: c_int;
    let mut len: c_int;
    let mut head: c_int;
    let mut prev: c_int;
    let mut nm: *const c_char;
    let mut e: *mut ebitmap;
    let mut node: *mut ebitmap_node = ptr::null_mut();

    if !(*p).mls_enabled {
        return 0;
    }

    len = 1; /* for the beginning ":" */
    l = 0;
    while l < 2 {
        let index_sens: u32 = (*context).range.level[l as usize].sens;
        len += strlen(sym_name(p, SYM_LEVELS, index_sens - 1)) as c_int;

        /* categories */
        head = -2;
        prev = -2;
        e = &mut (*context).range.level[l as usize].cat;
        ebitmap_for_each_positive_bit!(e, node, i, {
            if i - prev > 1 {
                /* one or more negative bits are skipped */
                if head != prev {
                    nm = sym_name(p, SYM_CATS, prev as u32);
                    len += strlen(nm) as c_int + 1;
                }
                nm = sym_name(p, SYM_CATS, i as u32);
                len += strlen(nm) as c_int + 1;
                head = i;
            }
            prev = i;
        });
        if prev != head {
            nm = sym_name(p, SYM_CATS, prev as u32);
            len += strlen(nm) as c_int + 1;
        }
        if l == 0 {
            if mls_level_eq(
                &mut (*context).range.level[0],
                &mut (*context).range.level[1],
            ) {
                break;
            } else {
                len += 1;
            }
        }
        l += 1;
    }

    len
}

/*
 * Write the security context string representation of
 * the MLS fields of `context' into the string `*scontext'.
 * Update `*scontext' to point to the end of the MLS fields.
 */
pub unsafe fn mls_sid_to_context(
    p: *mut policydb,
    context: *mut context,
    scontext: *mut *mut c_char,
) {
    let mut nm: *const c_char;
    let mut scontextp: *mut c_char;
    let mut i: c_int = 0;
    let mut l: c_int;
    let mut head: c_int;
    let mut prev: c_int;
    let mut e: *mut ebitmap;
    let mut node: *mut ebitmap_node = ptr::null_mut();

    if !(*p).mls_enabled {
        return;
    }

    scontextp = *scontext;

    *scontextp = b':' as c_char;
    scontextp = scontextp.add(1);

    l = 0;
    while l < 2 {
        strcpy(
            scontextp,
            sym_name(p, SYM_LEVELS, (*context).range.level[l as usize].sens - 1),
        );
        scontextp = scontextp.add(strlen(scontextp));

        /* categories */
        head = -2;
        prev = -2;
        e = &mut (*context).range.level[l as usize].cat;
        ebitmap_for_each_positive_bit!(e, node, i, {
            if i - prev > 1 {
                /* one or more negative bits are skipped */
                if prev != head {
                    if prev - head > 1 {
                        *scontextp = b'.' as c_char;
                    } else {
                        *scontextp = b',' as c_char;
                    }
                    scontextp = scontextp.add(1);
                    nm = sym_name(p, SYM_CATS, prev as u32);
                    strcpy(scontextp, nm);
                    scontextp = scontextp.add(strlen(nm));
                }
                if prev < 0 {
                    *scontextp = b':' as c_char;
                } else {
                    *scontextp = b',' as c_char;
                }
                scontextp = scontextp.add(1);
                nm = sym_name(p, SYM_CATS, i as u32);
                strcpy(scontextp, nm);
                scontextp = scontextp.add(strlen(nm));
                head = i;
            }
            prev = i;
        });

        if prev != head {
            if prev - head > 1 {
                *scontextp = b'.' as c_char;
            } else {
                *scontextp = b',' as c_char;
            }
            scontextp = scontextp.add(1);
            nm = sym_name(p, SYM_CATS, prev as u32);
            strcpy(scontextp, nm);
            scontextp = scontextp.add(strlen(nm));
        }

        if l == 0 {
            if mls_level_eq(
                &mut (*context).range.level[0],
                &mut (*context).range.level[1],
            ) {
                break;
            } else {
                *scontextp = b'-' as c_char;
                scontextp = scontextp.add(1);
            }
        }
        l += 1;
    }

    *scontext = scontextp;
}

pub unsafe fn mls_level_isvalid(p: *const policydb, l: *const mls_level) -> bool {
    let name: *const c_char;
    let levdatum: *const level_datum;

    if (*l).sens == 0 || (*l).sens > (*p).p_levels.nprim {
        return false;
    }

    name = sym_name(p as *mut policydb, SYM_LEVELS, (*l).sens - 1);
    if name.is_null() {
        return false;
    }

    levdatum = symtab_search(&(*p).p_levels as *const _ as *mut symtab, name) as *const level_datum;
    if levdatum.is_null() {
        return false;
    }

    /*
     * l is valid iff every bit in l->cat is set in levdatum->level.cat
     * and no bit in l->cat is larger than p->p_cats.nprim.
     * policydb_index() has already verified that every bit set in
     * levdatum->level.cat names a defined category, so containment is
     * sufficient here.
     */
    ebitmap_contains(
        &(*levdatum).level.cat as *const _ as *mut ebitmap,
        &(*l).cat as *const _ as *mut ebitmap,
        (*p).p_cats.nprim,
    )
}

pub unsafe fn mls_range_isvalid(p: *const policydb, r: *const mls_range) -> bool {
    mls_level_isvalid(p, &(*r).level[0])
        && mls_level_isvalid(p, &(*r).level[1])
        && mls_level_dom(&(*r).level[1], &(*r).level[0])
}

/*
 * Return true if the MLS fields in the security context
 * structure `c' are valid.  Return 0 otherwise.
 */
pub unsafe fn mls_context_isvalid(p: *const policydb, c: *const context) -> bool {
    let usrdatum: *const user_datum;

    if !(*p).mls_enabled {
        return true;
    }

    if !mls_range_isvalid(p, &(*c).range) {
        return false;
    }

    if (*c).role == OBJECT_R_VAL {
        return true;
    }

    /*
     * User must be authorized for the MLS range.
     */
    if (*c).user == 0 || (*c).user > (*p).p_users.nprim {
        return false;
    }
    usrdatum = *(*p).user_val_to_struct.add(((*c).user - 1) as usize);
    if usrdatum.is_null() || !mls_range_contains((*usrdatum).range, (*c).range) {
        return false; /* user may not be associated with range */
    }

    true
}

/*
 * Set the MLS fields in the security context structure
 * `context' based on the string representation in
 * the string `scontext'.
 *
 * This function modifies the string in place, inserting
 * NULL characters to terminate the MLS fields.
 *
 * If a def_sid is provided and no MLS field is present,
 * copy the MLS field of the associated default context.
 * Used for upgraded to MLS systems where objects may lack
 * MLS fields.
 *
 * Policy read-lock must be held for sidtab lookup.
 *
 */
pub unsafe fn mls_context_to_sid(
    pol: *mut policydb,
    oldc: c_char,
    scontext: *mut c_char,
    context: *mut context,
    s: *mut sidtab,
    def_sid: u32,
) -> c_int {
    let mut sensitivity: *mut c_char;
    let mut cur_cat: *mut c_char;
    let mut next_cat: *mut c_char;
    let mut rngptr: *mut c_char;
    let mut levdatum: *mut level_datum;
    let mut catdatum: *mut cat_datum;
    let mut rngdatum: *mut cat_datum;
    let mut i: u32;
    let mut l: c_int;
    let mut rc: c_int;
    let mut rangep: [*mut c_char; 2] = [ptr::null_mut(); 2];

    if !(*pol).mls_enabled {
        /*
         * With no MLS, only return -EINVAL if there is a MLS field
         * and it did not come from an xattr.
         */
        if oldc != 0 && def_sid == SECSID_NULL {
            return -EINVAL;
        }
        return 0;
    }

    /*
     * No MLS component to the security context, try and map to
     * default if provided.
     */
    if oldc == 0 {
        let defcon: *mut context;

        if def_sid == SECSID_NULL {
            return -EINVAL;
        }

        defcon = sidtab_search(s, def_sid);
        if defcon.is_null() {
            return -EINVAL;
        }

        return mls_context_cpy(context, defcon);
    }

    /*
     * If we're dealing with a range, figure out where the two parts
     * of the range begin.
     */
    rangep[0] = scontext;
    rangep[1] = strchr(scontext, b'-' as c_int);
    if !rangep[1].is_null() {
        *rangep[1] = 0;
        rangep[1] = rangep[1].add(1);
    }

    /* For each part of the range: */
    l = 0;
    while l < 2 {
        /* Split sensitivity and category set. */
        sensitivity = rangep[l as usize];
        if sensitivity.is_null() {
            break;
        }
        next_cat = strchr(sensitivity, b':' as c_int);
        if !next_cat.is_null() {
            *next_cat = 0;
            next_cat = next_cat.add(1);
        }

        /* Parse sensitivity. */
        levdatum = symtab_search(&mut (*pol).p_levels, sensitivity) as *mut level_datum;
        if levdatum.is_null() {
            return -EINVAL;
        }
        (*context).range.level[l as usize].sens = (*levdatum).level.sens;

        /* Extract category set. */
        while !next_cat.is_null() {
            cur_cat = next_cat;
            next_cat = strchr(next_cat, b',' as c_int);
            if !next_cat.is_null() {
                *next_cat = 0;
                next_cat = next_cat.add(1);
            }

            /* Separate into range if exists */
            rngptr = strchr(cur_cat, b'.' as c_int);
            if !rngptr.is_null() {
                /* Remove '.' */
                *rngptr = 0;
                rngptr = rngptr.add(1);
            }

            catdatum = symtab_search(&mut (*pol).p_cats, cur_cat) as *mut cat_datum;
            if catdatum.is_null() {
                return -EINVAL;
            }

            rc = ebitmap_set_bit(
                &mut (*context).range.level[l as usize].cat,
                (*catdatum).value - 1,
                1,
            );
            if rc != 0 {
                return rc;
            }

            /* If range, set all categories in range */
            if rngptr.is_null() {
                continue;
            }

            rngdatum = symtab_search(&mut (*pol).p_cats, rngptr) as *mut cat_datum;
            if rngdatum.is_null() {
                return -EINVAL;
            }

            if (*catdatum).value >= (*rngdatum).value {
                return -EINVAL;
            }

            i = (*catdatum).value;
            while i < (*rngdatum).value {
                rc = ebitmap_set_bit(&mut (*context).range.level[l as usize].cat, i, 1);
                if rc != 0 {
                    return rc;
                }
                i += 1;
            }
        }
        l += 1;
    }

    /* If we didn't see a '-', the range start is also the range end. */
    if rangep[1].is_null() {
        (*context).range.level[1].sens = (*context).range.level[0].sens;
        rc = ebitmap_cpy(
            &mut (*context).range.level[1].cat,
            &mut (*context).range.level[0].cat,
        );
        if rc != 0 {
            return rc;
        }
    }

    0
}

/*
 * Set the MLS fields in the security context structure
 * `context' based on the string representation in
 * the string `str'.  This function will allocate temporary memory with the
 * given constraints of gfp_mask.
 */
pub unsafe fn mls_from_string(
    p: *mut policydb,
    str_: *mut c_char,
    context: *mut context,
    gfp_mask: gfp_t,
) -> c_int {
    let tmpstr: *mut c_char;
    let rc: c_int;

    if !(*p).mls_enabled {
        return -EINVAL;
    }

    tmpstr = kstrdup(str_, gfp_mask);
    if tmpstr.is_null() {
        rc = -ENOMEM;
    } else {
        rc = mls_context_to_sid(p, b':' as c_char, tmpstr, context, ptr::null_mut(), SECSID_NULL);
        kfree(tmpstr as *mut c_void);
    }

    rc
}

/*
 * Copies the MLS range `range' into `context'.
 */
pub unsafe fn mls_range_set(context: *mut context, range: *mut mls_range) -> c_int {
    let mut l: c_int;
    let mut rc: c_int = 0;

    /* Copy the MLS range into the  context */
    l = 0;
    while l < 2 {
        (*context).range.level[l as usize].sens = (*range).level[l as usize].sens;
        rc = ebitmap_cpy(
            &mut (*context).range.level[l as usize].cat,
            &mut (*range).level[l as usize].cat,
        );
        if rc != 0 {
            break;
        }
        l += 1;
    }

    rc
}

pub unsafe fn mls_setup_user_range(
    p: *mut policydb,
    fromcon: *mut context,
    user: *mut user_datum,
    usercon: *mut context,
) -> c_int {
    if (*p).mls_enabled {
        let fromcon_sen: *mut mls_level = &mut (*fromcon).range.level[0];
        let fromcon_clr: *mut mls_level = &mut (*fromcon).range.level[1];
        let user_low: *mut mls_level = &mut (*user).range.level[0];
        let user_clr: *mut mls_level = &mut (*user).range.level[1];
        let user_def: *mut mls_level = &mut (*user).dfltlevel;
        let usercon_sen: *mut mls_level = &mut (*usercon).range.level[0];
        let usercon_clr: *mut mls_level = &mut (*usercon).range.level[1];

        /* Honor the user's default level if we can */
        if mls_level_between(user_def, fromcon_sen, fromcon_clr) {
            *usercon_sen = *user_def;
        } else if mls_level_between(fromcon_sen, user_def, user_clr) {
            *usercon_sen = *fromcon_sen;
        } else if mls_level_between(fromcon_clr, user_low, user_def) {
            *usercon_sen = *user_low;
        } else {
            return -EINVAL;
        }

        /* Lower the clearance of available contexts
           if the clearance of "fromcon" is lower than
           that of the user's default clearance (but
           only if the "fromcon" clearance dominates
           the user's computed sensitivity level) */
        if mls_level_dom(user_clr, fromcon_clr) {
            *usercon_clr = *fromcon_clr;
        } else if mls_level_dom(fromcon_clr, user_clr) {
            *usercon_clr = *user_clr;
        } else {
            return -EINVAL;
        }
    }

    0
}

/*
 * Convert the MLS fields in the security context
 * structure `oldc' from the values specified in the
 * policy `oldp' to the values specified in the policy `newp',
 * storing the resulting context in `newc'.
 */
pub unsafe fn mls_convert_context(
    oldp: *mut policydb,
    newp: *mut policydb,
    oldc: *mut context,
    newc: *mut context,
) -> c_int {
    let mut levdatum: *mut level_datum;
    let mut catdatum: *mut cat_datum;
    let mut node: *mut ebitmap_node = ptr::null_mut();
    let mut i: u32 = 0;
    let mut l: c_int;

    if !(*oldp).mls_enabled || !(*newp).mls_enabled {
        return 0;
    }

    l = 0;
    while l < 2 {
        let name: *const c_char =
            sym_name(oldp, SYM_LEVELS, (*oldc).range.level[l as usize].sens - 1);

        levdatum = symtab_search(&mut (*newp).p_levels, name) as *mut level_datum;

        if levdatum.is_null() {
            return -EINVAL;
        }
        (*newc).range.level[l as usize].sens = (*levdatum).level.sens;

        ebitmap_for_each_positive_bit!(&mut (*oldc).range.level[l as usize].cat, node, i, {
            let rc: c_int;

            catdatum = symtab_search(
                &mut (*newp).p_cats,
                sym_name(oldp, SYM_CATS, i) as *mut c_char,
            ) as *mut cat_datum;
            if catdatum.is_null() {
                return -EINVAL;
            }
            rc = ebitmap_set_bit(
                &mut (*newc).range.level[l as usize].cat,
                (*catdatum).value - 1,
                1,
            );
            if rc != 0 {
                return rc;
            }
        });
        l += 1;
    }

    0
}

pub unsafe fn mls_compute_sid(
    p: *mut policydb,
    scontext: *mut context,
    tcontext: *mut context,
    tclass: u16,
    specified: u32,
    newcontext: *mut context,
    sock: bool,
) -> c_int {
    let mut rtr: range_trans = core::mem::zeroed();
    let mut r: *mut mls_range;
    let mut cladatum: *mut class_datum;
    let mut default_range: c_char = 0;

    if !(*p).mls_enabled {
        return 0;
    }

    match specified {
        AVTAB_TRANSITION => {
            /* Look for a range transition rule. */
            rtr.source_type = (*scontext).type_;
            rtr.target_type = (*tcontext).type_;
            rtr.target_class = tclass;
            r = policydb_rangetr_search(p, &mut rtr);
            if !r.is_null() {
                return mls_range_set(newcontext, r);
            }

            if tclass != 0 && (tclass as u32) <= (*p).p_classes.nprim {
                cladatum = *(*p).class_val_to_struct.add((tclass - 1) as usize);
                if !cladatum.is_null() {
                    default_range = (*cladatum).default_range;
                }
            }

            match default_range as u32 {
                DEFAULT_SOURCE_LOW => {
                    return mls_context_cpy_low(newcontext, scontext);
                }
                DEFAULT_SOURCE_HIGH => {
                    return mls_context_cpy_high(newcontext, scontext);
                }
                DEFAULT_SOURCE_LOW_HIGH => {
                    return mls_context_cpy(newcontext, scontext);
                }
                DEFAULT_TARGET_LOW => {
                    return mls_context_cpy_low(newcontext, tcontext);
                }
                DEFAULT_TARGET_HIGH => {
                    return mls_context_cpy_high(newcontext, tcontext);
                }
                DEFAULT_TARGET_LOW_HIGH => {
                    return mls_context_cpy(newcontext, tcontext);
                }
                DEFAULT_GLBLUB => {
                    return mls_context_glblub(newcontext, scontext, tcontext);
                }
                _ => {}
            }

            /* fallthrough */
            if tclass == (*p).process_class || sock {
                /* Use the process MLS attributes. */
                return mls_context_cpy(newcontext, scontext);
            } else {
                /* Use the process effective MLS attributes. */
                return mls_context_cpy_low(newcontext, scontext);
            }
        }
        AVTAB_CHANGE => {
            if tclass == (*p).process_class || sock {
                /* Use the process MLS attributes. */
                return mls_context_cpy(newcontext, scontext);
            } else {
                /* Use the process effective MLS attributes. */
                return mls_context_cpy_low(newcontext, scontext);
            }
        }
        AVTAB_MEMBER => {
            /* Use the process effective MLS attributes. */
            return mls_context_cpy_low(newcontext, scontext);
        }
        _ => {}
    }
    -EINVAL
}

/* CONFIG_NETLABEL */
/**
 * mls_export_netlbl_lvl - Export the MLS sensitivity levels to NetLabel
 * @p: the policy
 * @context: the security context
 * @secattr: the NetLabel security attributes
 *
 * Description:
 * Given the security context copy the low MLS sensitivity level into the
 * NetLabel MLS sensitivity level field.
 *
 */
pub unsafe fn mls_export_netlbl_lvl(
    p: *mut policydb,
    context: *mut context,
    secattr: *mut netlbl_lsm_secattr,
) {
    if !(*p).mls_enabled {
        return;
    }

    (*secattr).attr.mls.lvl = (*context).range.level[0].sens - 1;
    (*secattr).flags |= NETLBL_SECATTR_MLS_LVL;
}

/**
 * mls_import_netlbl_lvl - Import the NetLabel MLS sensitivity levels
 * @p: the policy
 * @context: the security context
 * @secattr: the NetLabel security attributes
 *
 * Description:
 * Given the security context and the NetLabel security attributes, copy the
 * NetLabel MLS sensitivity level into the context.
 *
 */
pub unsafe fn mls_import_netlbl_lvl(
    p: *mut policydb,
    context: *mut context,
    secattr: *mut netlbl_lsm_secattr,
) {
    if !(*p).mls_enabled {
        return;
    }

    (*context).range.level[0].sens = (*secattr).attr.mls.lvl + 1;
    (*context).range.level[1].sens = (*context).range.level[0].sens;
}

/**
 * mls_export_netlbl_cat - Export the MLS categories to NetLabel
 * @p: the policy
 * @context: the security context
 * @secattr: the NetLabel security attributes
 *
 * Description:
 * Given the security context copy the low MLS categories into the NetLabel
 * MLS category field.  Returns zero on success, negative values on failure.
 *
 */
pub unsafe fn mls_export_netlbl_cat(
    p: *mut policydb,
    context: *mut context,
    secattr: *mut netlbl_lsm_secattr,
) -> c_int {
    let rc: c_int;

    if !(*p).mls_enabled {
        return 0;
    }

    rc = ebitmap_netlbl_export(
        &mut (*context).range.level[0].cat,
        &mut (*secattr).attr.mls.cat,
    );
    if rc == 0 && !(*secattr).attr.mls.cat.is_null() {
        (*secattr).flags |= NETLBL_SECATTR_MLS_CAT;
    }

    rc
}

/**
 * mls_import_netlbl_cat - Import the MLS categories from NetLabel
 * @p: the policy
 * @context: the security context
 * @secattr: the NetLabel security attributes
 *
 * Description:
 * Copy the NetLabel security attributes into the SELinux context; since the
 * NetLabel security attribute only contains a single MLS category use it for
 * both the low and high categories of the context.  Returns zero on success,
 * negative values on failure.
 *
 */
pub unsafe fn mls_import_netlbl_cat(
    p: *mut policydb,
    context: *mut context,
    secattr: *mut netlbl_lsm_secattr,
) -> c_int {
    let rc: c_int;

    if !(*p).mls_enabled {
        return 0;
    }

    rc = ebitmap_netlbl_import(&mut (*context).range.level[0].cat, (*secattr).attr.mls.cat);
    if rc != 0 {
        ebitmap_destroy(&mut (*context).range.level[0].cat);
        return rc;
    }
    ptr::copy_nonoverlapping(
        &(*context).range.level[0].cat as *const ebitmap,
        &mut (*context).range.level[1].cat as *mut ebitmap,
        size_of::<ebitmap>(),
    );

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
