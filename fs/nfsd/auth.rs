// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de> */

// C dependencies: linux/sched.h, nfsd.h, export.h, and auth.h.

pub unsafe fn nfsexp_flags(
    cred: *mut svc_cred,
    exp: *mut svc_export,
) -> i32 {
    let mut f: *mut exp_flavor_info = (*exp).ex_flavors;
    let end: *mut exp_flavor_info = (*exp).ex_flavors.add((*exp).ex_nflavors as usize);

    while f < end {
        if (*f).pseudoflavor == (*cred).cr_flavor {
            return (*f).flags;
        }
        f = f.add(1);
    }
    (*exp).ex_flags
}

pub unsafe fn nfsd_setuser(
    cred: *mut svc_cred,
    exp: *mut svc_export,
) -> i32 {
    let mut rqgi: *mut group_info;
    let mut gi: *mut group_info;
    let mut new: *mut cred;
    let mut i: i32;
    let flags: i32 = nfsexp_flags(cred, exp);

    /* discard any old override before preparing the new set */
    put_cred(revert_creds(get_cred(current_real_cred())));
    new = prepare_creds();
    if new.is_null() {
        return -ENOMEM;
    }

    (*new).fsuid = (*cred).cr_uid;
    (*new).fsgid = (*cred).cr_gid;

    rqgi = (*cred).cr_group_info;

    if (flags & NFSEXP_ALLSQUASH) != 0 {
        (*new).fsuid = (*exp).ex_anon_uid;
        (*new).fsgid = (*exp).ex_anon_gid;
        gi = groups_alloc(0);
        if gi.is_null() {
            goto oom;
        }
    } else if (flags & NFSEXP_ROOTSQUASH) != 0 {
        if uid_eq((*new).fsuid, GLOBAL_ROOT_UID) {
            (*new).fsuid = (*exp).ex_anon_uid;
        }
        if gid_eq((*new).fsgid, GLOBAL_ROOT_GID) {
            (*new).fsgid = (*exp).ex_anon_gid;
        }

        gi = groups_alloc((*rqgi).ngroups);
        if gi.is_null() {
            goto oom;
        }

        i = 0;
        while i < (*rqgi).ngroups {
            if gid_eq(GLOBAL_ROOT_GID, (*rqgi).gid[i as usize]) {
                (*gi).gid[i as usize] = (*exp).ex_anon_gid;
            } else {
                (*gi).gid[i as usize] = (*rqgi).gid[i as usize];
            }
            i += 1;
        }

        /* Each thread allocates its own gi, no race */
        groups_sort(gi);
    } else {
        gi = get_group_info(rqgi);
    }

    if uid_eq((*new).fsuid, INVALID_UID) {
        (*new).fsuid = (*exp).ex_anon_uid;
    }
    if gid_eq((*new).fsgid, INVALID_GID) {
        (*new).fsgid = (*exp).ex_anon_gid;
    }

    set_groups(new, gi);
    put_group_info(gi);

    if !uid_eq((*new).fsuid, GLOBAL_ROOT_UID) {
        (*new).cap_effective = cap_drop_nfsd_set((*new).cap_effective);
    } else {
        (*new).cap_effective = cap_raise_nfsd_set(
            (*new).cap_effective,
            (*new).cap_permitted,
        );
    }
    put_cred(override_creds(new));
    return 0;

oom:
    abort_creds(new);
    -ENOMEM
}

/**
 * nfsd_user_namespace - Get user_namespace in effect for an RPC request
 * @rqstp: RPC execution context
 *
 * xpt_cred is set once at transport creation and never modified. The
 * transport itself is reference-counted during request processing, so
 * no explicit reference on the namespace is necessary.
 *
 * Return: the user_namespace from the transport credential, or
 * init_user_ns if no credential was set. The returned namespace pointer
 * is valid for the duration of the RPC request.
 */
pub unsafe fn nfsd_user_namespace(rqstp: *const svc_rqst) -> *mut user_namespace {
    let cred: *const cred = (*(*rqstp).rq_xprt).xpt_cred;

    if !cred.is_null() {
        (*cred).user_ns
    } else {
        &raw mut init_user_ns
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
