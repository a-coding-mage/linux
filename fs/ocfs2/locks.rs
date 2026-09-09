// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * locks.rs
 *
 * Userspace file locking support
 *
 * Copyright (C) 2007 Oracle.  All rights reserved.
 */

// C dependencies supplied by the surrounding kernel/OCFS2 sources are
// intentionally referenced here without reimplementing them.

unsafe fn ocfs2_do_flock(
    file: *mut file,
    inode: *mut inode,
    cmd: c_int,
    fl: *mut file_lock,
) -> c_int {
    let mut ret: c_int = 0;
    let mut level: c_int = 0;
    let mut trylock: c_int = 0;
    let mut goto_out = false;
    let fp: *mut ocfs2_file_private = (*file).private_data as *mut ocfs2_file_private;
    let lockres: *mut ocfs2_lock_res = &mut (*fp).fp_flock;

    if lock_is_write(fl) {
        level = 1;
    }
    if !IS_SETLKW(cmd) {
        trylock = 1;
    }

    mutex_lock(&mut (*fp).fp_mutex);

    if ((*lockres).l_flags & OCFS2_LOCK_ATTACHED) != 0
        && (*lockres).l_level > LKM_NLMODE
    {
        let mut old_level: c_int = 0;
        let mut request: file_lock = core::mem::zeroed();

        if (*lockres).l_level == LKM_EXMODE {
            old_level = 1;
        }

        if level == old_level {
            goto_out = true;
        } else {
            /*
             * Converting an existing lock is not guaranteed to be
             * atomic, so we can get away with simply unlocking
             * here and allowing the lock code to try at the new
             * level.
             */

            locks_init_lock(&mut request);
            request.c.flc_type = F_UNLCK;
            request.c.flc_flags = FL_FLOCK;
            locks_lock_file_wait(file, &mut request);

            ocfs2_file_unlock(file);
            goto_out = false;
        }
    }

    if !goto_out {
        ret = ocfs2_file_lock(file, level, trylock);
        if ret != 0 {
            if ret == -EAGAIN && trylock != 0 {
                ret = -EWOULDBLOCK;
            } else {
                mlog_errno(ret);
            }
            goto_out = true;
        } else {
            ret = locks_lock_file_wait(file, fl);
            if ret != 0 {
                ocfs2_file_unlock(file);
            }
        }
    }

    mutex_unlock(&mut (*fp).fp_mutex);

    ret
}

unsafe fn ocfs2_do_funlock(
    file: *mut file,
    cmd: c_int,
    fl: *mut file_lock,
) -> c_int {
    let ret: c_int;
    let fp: *mut ocfs2_file_private = (*file).private_data as *mut ocfs2_file_private;

    mutex_lock(&mut (*fp).fp_mutex);
    ocfs2_file_unlock(file);
    ret = locks_lock_file_wait(file, fl);
    mutex_unlock(&mut (*fp).fp_mutex);

    ret
}

/*
 * Overall flow of ocfs2_flock() was influenced by gfs2_flock().
 */
unsafe fn ocfs2_flock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    let inode: *mut inode = (*(*file).f_mapping).host;
    let osb: *mut ocfs2_super = OCFS2_SB((*inode).i_sb);

    if ((*fl).c.flc_flags & FL_FLOCK) == 0 {
        return -ENOLCK;
    }

    if ((*osb).s_mount_opt & OCFS2_MOUNT_LOCALFLOCKS) != 0 || ocfs2_mount_local(osb) {
        return locks_lock_file_wait(file, fl);
    }

    if lock_is_unlock(fl) {
        ocfs2_do_funlock(file, cmd, fl)
    } else {
        ocfs2_do_flock(file, inode, cmd, fl)
    }
}

unsafe fn ocfs2_lock(file: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    let inode: *mut inode = (*(*file).f_mapping).host;
    let osb: *mut ocfs2_super = OCFS2_SB((*inode).i_sb);

    if ((*fl).c.flc_flags & FL_POSIX) == 0 {
        return -ENOLCK;
    }

    ocfs2_plock((*osb).cconn, OCFS2_I(inode).ip_blkno, file, cmd, fl)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
