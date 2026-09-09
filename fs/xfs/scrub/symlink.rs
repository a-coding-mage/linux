// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017-2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// Dependencies supplied by the surrounding XFS implementation.

/* Set us up to scrub a symbolic link. */
pub unsafe fn xchk_setup_symlink(sc: *mut xfs_scrub) -> i32 {
    let mut resblks: u32 = 0;
    let error: i32;

    /* Allocate the buffer without the inode lock held. */
    (*sc).buf = kvzalloc(XFS_SYMLINK_MAXLEN + 1, XCHK_GFP_FLAGS);
    if (*sc).buf.is_null() {
        return -ENOMEM;
    }

    if xchk_could_repair(sc) {
        error = xrep_setup_symlink(sc, &mut resblks);
        if error != 0 {
            return error;
        }
    }

    xchk_setup_inode_contents(sc, resblks)
}

/* Symbolic links. */

pub unsafe fn xchk_symlink(sc: *mut xfs_scrub) -> i32 {
    let ip: *mut xfs_inode = (*sc).ip;
    let ifp: *mut xfs_ifork;
    let len: i64;
    let mut error: i32 = 0;

    if !S_ISLNK((*VFS_I(ip)).i_mode) {
        return -ENOENT;
    }

    if xchk_file_looks_zapped(sc, XFS_SICK_INO_SYMLINK_ZAPPED) {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
        return 0;
    }

    ifp = xfs_ifork_ptr(ip, XFS_DATA_FORK);
    len = (*ip).i_disk_size;

    /* Plausible size? */
    if len > XFS_SYMLINK_MAXLEN as i64 || len <= 0 {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
        return 0;
    }

    /* Inline symlink? */
    if (*ifp).if_format == XFS_DINODE_FMT_LOCAL {
        if len > xfs_inode_data_fork_size(ip) as i64
            || len > strnlen((*ifp).if_data, xfs_inode_data_fork_size(ip)) as i64
        {
            xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
        }
        return 0;
    }

    /* Remote symlink; must read the contents. */
    error = xfs_symlink_remote_read((*sc).ip, (*sc).buf);
    if !xchk_fblock_process_error(sc, XFS_DATA_FORK, 0, &mut error) {
        return error;
    }
    if strnlen((*sc).buf, XFS_SYMLINK_MAXLEN) as i64 < len {
        xchk_fblock_set_corrupt(sc, XFS_DATA_FORK, 0);
    }

    /* If a remote symlink is clean, it is clearly not zapped. */
    xchk_mark_healthy_if_clean(sc, XFS_SICK_INO_SYMLINK_ZAPPED);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
