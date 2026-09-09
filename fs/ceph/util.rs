// SPDX-License-Identifier: GPL-2.0
/*
 * Some non-inline ceph helpers
 */

/* External Ceph types, constants, and platform flag definitions are supplied
 * by the surrounding kernel translation. */

/*
 * return true if @layout appears to be valid
 */
pub unsafe fn ceph_file_layout_is_valid(
    layout: *const ceph_file_layout,
) -> i32 {
    let su: u32 = (*layout).stripe_unit;
    let sc: u32 = (*layout).stripe_count;
    let os: u32 = (*layout).object_size;

    /* stripe unit, object size must be non-zero, 64k increment */
    if su == 0 || (su & (CEPH_MIN_STRIPE_UNIT - 1)) != 0 {
        return 0;
    }
    if os == 0 || (os & (CEPH_MIN_STRIPE_UNIT - 1)) != 0 {
        return 0;
    }
    /* object size must be a multiple of stripe unit */
    if os < su || os % su != 0 {
        return 0;
    }
    /* stripe count must be non-zero */
    if sc == 0 {
        return 0;
    }
    1
}

pub unsafe fn ceph_file_layout_from_legacy(
    fl: *mut ceph_file_layout,
    legacy: *mut ceph_file_layout_legacy,
) {
    (*fl).stripe_unit = u32::from_le((*legacy).fl_stripe_unit);
    (*fl).stripe_count = u32::from_le((*legacy).fl_stripe_count);
    (*fl).object_size = u32::from_le((*legacy).fl_object_size);
    (*fl).pool_id = u32::from_le((*legacy).fl_pg_pool);
    if (*fl).pool_id == 0
        && (*fl).stripe_unit == 0
        && (*fl).stripe_count == 0
        && (*fl).object_size == 0
    {
        (*fl).pool_id = -1;
    }
}

pub unsafe fn ceph_file_layout_to_legacy(
    fl: *mut ceph_file_layout,
    legacy: *mut ceph_file_layout_legacy,
) {
    (*legacy).fl_stripe_unit = (*fl).stripe_unit.to_le();
    (*legacy).fl_stripe_count = (*fl).stripe_count.to_le();
    (*legacy).fl_object_size = (*fl).object_size.to_le();
    if (*fl).pool_id >= 0 {
        (*legacy).fl_pg_pool = (*fl).pool_id.to_le();
    } else {
        (*legacy).fl_pg_pool = 0;
    }
}

pub fn ceph_flags_to_mode(flags: i32) -> i32 {
    let mut mode: i32 = 0;

    /* #ifdef O_DIRECTORY -- fixme */
    #[cfg(has_o_directory)]
    if (flags & O_DIRECTORY) == O_DIRECTORY {
        return CEPH_FILE_MODE_PIN;
    }

    match flags & O_ACCMODE {
        O_WRONLY => {
            mode = CEPH_FILE_MODE_WR;
        }
        O_RDONLY => {
            mode = CEPH_FILE_MODE_RD;
        }
        O_RDWR | O_ACCMODE => {
            /* this is what the VFS does */
            mode = CEPH_FILE_MODE_RDWR;
        }
        _ => {}
    }
    /* #ifdef O_LAZY */
    #[cfg(has_o_lazy)]
    if flags & O_LAZY != 0 {
        mode |= CEPH_FILE_MODE_LAZY;
    }

    mode
}

pub fn ceph_caps_for_mode(mode: i32) -> i32 {
    let mut caps = CEPH_CAP_PIN;

    if mode & CEPH_FILE_MODE_RD != 0 {
        caps |= CEPH_CAP_FILE_SHARED | CEPH_CAP_FILE_RD | CEPH_CAP_FILE_CACHE;
    }
    if mode & CEPH_FILE_MODE_WR != 0 {
        caps |= CEPH_CAP_FILE_EXCL
            | CEPH_CAP_FILE_WR
            | CEPH_CAP_FILE_BUFFER
            | CEPH_CAP_AUTH_SHARED
            | CEPH_CAP_AUTH_EXCL
            | CEPH_CAP_XATTR_SHARED
            | CEPH_CAP_XATTR_EXCL;
    }
    if mode & CEPH_FILE_MODE_LAZY != 0 {
        caps |= CEPH_CAP_FILE_LAZYIO;
    }

    caps
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
