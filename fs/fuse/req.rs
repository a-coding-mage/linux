// SPDX-License-Identifier: GPL-2.0-only

// Translated from dev.h and fuse_i.h dependencies.

unsafe fn fuse_fill_creds(
    fm: *mut fuse_mount,
    args: *mut fuse_args,
    idmap: *mut mnt_idmap,
) -> i32 {
    let fc = (*fm).fc;
    let no_idmap = (*fm).sb.is_null()
        || ((*(*fm).sb).s_iflags & SB_I_NOIDMAP) != 0;
    let mut fsuid = mapped_fsuid(idmap, (*fc).user_ns);
    let mut fsgid = mapped_fsgid(idmap, (*fc).user_ns);

    (*args).pid = pid_nr_ns(task_pid(current), (*fc).pid_ns);

    if (*args).force {
        if (*args).nocreds {
            return 0;
        }

        if no_idmap {
            (*args).uid = from_kuid_munged((*fc).user_ns, current_fsuid());
            (*args).gid = from_kgid_munged((*fc).user_ns, current_fsgid());
        } else {
            (*args).uid = FUSE_INVALID_UIDGID;
            (*args).gid = FUSE_INVALID_UIDGID;
        }
        return 0;
    }

    WARN_ON((*args).nocreds);
    /*
     * Keep the old behavior when idmappings support was not
     * declared by a FUSE server.
     *
     * For those FUSE servers who support idmapped mounts, we send UID/GID
     * only along with "inode creation" fuse requests, otherwise idmap ==
     * &invalid_mnt_idmap and req->in.h.{u,g}id will be equal to
     * FUSE_INVALID_UIDGID.
     */
    if no_idmap {
        fsuid = current_fsuid();
        fsgid = current_fsgid();
    }
    (*args).uid = from_kuid((*fc).user_ns, fsuid);
    (*args).gid = from_kgid((*fc).user_ns, fsgid);

    if no_idmap
        && unlikely((*args).uid == ((-1i32) as uid_t)
            || (*args).gid == ((-1i32) as gid_t))
    {
        return -EOVERFLOW;
    }

    0
}

unsafe fn fuse_req_prep(
    fm: *mut fuse_mount,
    args: *mut fuse_args,
    idmap: *mut mnt_idmap,
) -> i32 {
    if !(*args).force && (*(*fm).fc).conn_error {
        return -ECONNREFUSED;
    }

    fuse_fill_creds(fm, args, idmap)
}

pub unsafe fn __fuse_simple_request(
    idmap: *mut mnt_idmap,
    fm: *mut fuse_mount,
    args: *mut fuse_args,
) -> ssize_t {
    let fc = (*fm).fc;
    let err = fuse_req_prep(fm, args, idmap);

    if err != 0 {
        return err as ssize_t;
    }

    fuse_chan_send((*fc).chan, args)
}

pub unsafe fn fuse_simple_background(
    fm: *mut fuse_mount,
    args: *mut fuse_args,
    gfp_flags: gfp_t,
) -> i32 {
    let fc = (*fm).fc;
    let err: i32;

    WARN_ON((*args).force && !(*args).nocreds);

    err = fuse_req_prep(fm, args, &mut invalid_mnt_idmap);
    if err != 0 {
        return err;
    }

    fuse_chan_send_bg((*fc).chan, args, gfp_flags)
}

// EXPORT_SYMBOL_GPL(fuse_simple_background);

pub unsafe fn fuse_simple_notify_reply(
    fm: *mut fuse_mount,
    args: *mut fuse_args,
    unique: u64,
) -> i32 {
    let fc = (*fm).fc;
    let err: i32;

    WARN_ON((*args).force && !(*args).nocreds);

    err = fuse_req_prep(fm, args, &mut invalid_mnt_idmap);
    if err != 0 {
        return err;
    }

    fuse_chan_send_notify_reply((*fc).chan, args, unique)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
