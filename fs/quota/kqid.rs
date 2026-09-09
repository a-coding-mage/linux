// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel headers are referenced below.

/**
 * qid_eq - Test to see if two kqid values are the same
 * @left: A qid value
 * @right: Another qid value
 *
 * Return true if the two qid values are equal and false otherwise.
 */
pub unsafe fn qid_eq(left: kqid, right: kqid) -> bool {
    if left.type_ != right.type_ {
        return false;
    }
    match left.type_ {
        USRQUOTA => uid_eq(left.uid, right.uid),
        GRPQUOTA => gid_eq(left.gid, right.gid),
        PRJQUOTA => projid_eq(left.projid, right.projid),
        _ => core::hint::unreachable_unchecked(),
    }
}

// EXPORT_SYMBOL(qid_eq);

/**
 * qid_lt - Test to see if one qid value is less than another
 * @left: The possibly lesser qid value
 * @right: The possibly greater qid value
 *
 * Return true if left is less than right and false otherwise.
 */
pub unsafe fn qid_lt(left: kqid, right: kqid) -> bool {
    if left.type_ < right.type_ {
        return true;
    }
    if left.type_ > right.type_ {
        return false;
    }
    match left.type_ {
        USRQUOTA => uid_lt(left.uid, right.uid),
        GRPQUOTA => gid_lt(left.gid, right.gid),
        PRJQUOTA => projid_lt(left.projid, right.projid),
        _ => core::hint::unreachable_unchecked(),
    }
}

// EXPORT_SYMBOL(qid_lt);

/**
 * from_kqid - Create a qid from a kqid user-namespace pair.
 * @targ: The user namespace we want a qid in.
 * @kqid: The kernel internal quota identifier to start with.
 *
 * Map @kqid into the user-namespace specified by @targ and
 * return the resulting qid.
 *
 * There is always a mapping into the initial user_namespace.
 *
 * If @kqid has no mapping in @targ (qid_t)-1 is returned.
 */
pub unsafe fn from_kqid(targ: *mut user_namespace, kqid: kqid) -> qid_t {
    match kqid.type_ {
        USRQUOTA => from_kuid(targ, kqid.uid),
        GRPQUOTA => from_kgid(targ, kqid.gid),
        PRJQUOTA => from_kprojid(targ, kqid.projid),
        _ => core::hint::unreachable_unchecked(),
    }
}

// EXPORT_SYMBOL(from_kqid);

/**
 * from_kqid_munged - Create a qid from a kqid user-namespace pair.
 * @targ: The user namespace we want a qid in.
 * @kqid: The kernel internal quota identifier to start with.
 *
 * Map @kqid into the user-namespace specified by @targ and
 * return the resulting qid.
 *
 * There is always a mapping into the initial user_namespace.
 *
 * Unlike from_kqid, from_kqid_munged never fails and always
 * returns a valid projid. This makes from_kqid_munged
 * appropriate for use in places where failing to provide
 * a qid_t is not a good option.
 *
 * If @kqid has no mapping in @targ the kqid.type specific
 * overflow identifier is returned.
 */
pub unsafe fn from_kqid_munged(targ: *mut user_namespace, kqid: kqid) -> qid_t {
    match kqid.type_ {
        USRQUOTA => from_kuid_munged(targ, kqid.uid),
        GRPQUOTA => from_kgid_munged(targ, kqid.gid),
        PRJQUOTA => from_kprojid_munged(targ, kqid.projid),
        _ => core::hint::unreachable_unchecked(),
    }
}

// EXPORT_SYMBOL(from_kqid_munged);

/**
 * qid_valid - Report if a valid value is stored in a kqid.
 * @qid: The kernel internal quota identifier to test.
 */
pub unsafe fn qid_valid(qid: kqid) -> bool {
    match qid.type_ {
        USRQUOTA => uid_valid(qid.uid),
        GRPQUOTA => gid_valid(qid.gid),
        PRJQUOTA => projid_valid(qid.projid),
        _ => core::hint::unreachable_unchecked(),
    }
}

// EXPORT_SYMBOL(qid_valid);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
