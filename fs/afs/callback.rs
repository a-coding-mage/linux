/*
 * Copyright (c) 2002, 2007 Red Hat, Inc. All rights reserved.
 *
 * This software may be freely redistributed under the terms of the
 * GNU General Public License.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 *
 * Authors: David Woodhouse <dwmw2@infradead.org>
 *          David Howells <dhowells@redhat.com>
 *
 */

// Dependencies supplied by the surrounding kernel and AFS implementation.

/*
 * Handle invalidation of an mmap'd file.  We invalidate all the PTEs referring
 * to the pages in this file's pagecache, forcing the kernel to go through
 * ->fault() or ->page_mkwrite() - at which point we can handle invalidation
 * more fully.
 */
pub unsafe fn afs_invalidate_mmap_work(work: *mut work_struct) {
    let vnode = container_of!(work, afs_vnode, cb_work);

    unmap_mapping_pages((*vnode).netfs.inode.i_mapping, 0, 0, false);
}

unsafe fn afs_volume_init_callback(volume: *mut afs_volume) {
    let mut vnode: *mut afs_vnode;

    down_read(&(*volume).open_mmaps_lock);

    list_for_each_entry!(vnode, &(*volume).open_mmaps, cb_mmap_link, {
        if (*vnode).cb_v_check != atomic_read(&(*volume).cb_v_break) {
            afs_clear_cb_promise(vnode, afs_cb_promise_clear_vol_init_cb);
            queue_work(system_dfl_wq, &(*vnode).cb_work);
        }
    });

    up_read(&(*volume).open_mmaps_lock);
}

/*
 * Allow the fileserver to request callback state (re-)initialisation.
 * Unfortunately, UUIDs are not guaranteed unique.
 */
pub unsafe fn afs_init_callback_state(server: *mut afs_server) {
    let mut se: *mut afs_server_entry;

    down_read(&(*(*server).cell).vs_lock);

    list_for_each_entry!(se, &(*server).volumes, slink, {
        (*se).cb_expires_at = AFS_NO_CB_PROMISE;
        (*(*se).volume).cb_expires_at = AFS_NO_CB_PROMISE;
        trace_afs_cb_v_break((*se).volume.vid, atomic_read(&(*(*se).volume).cb_v_break),
                             afs_cb_break_for_s_reinit);
        if !list_empty(&(*(*se).volume).open_mmaps) {
            afs_volume_init_callback((*se).volume);
        }
    });

    up_read(&(*(*server).cell).vs_lock);
}

/*
 * actually break a callback
 */
pub unsafe fn __afs_break_callback(vnode: *mut afs_vnode, reason: afs_cb_break_reason) {
    _enter!("");

    clear_bit(AFS_VNODE_NEW_CONTENT, &mut (*vnode).flags);
    if afs_clear_cb_promise(vnode, afs_cb_promise_clear_cb_break) {
        (*vnode).cb_break += 1;
        (*vnode).cb_v_check = atomic_read(&(*(*vnode).volume).cb_v_break);
        afs_clear_permits(vnode);

        if (*vnode).lock_state == AFS_VNODE_LOCK_WAITING_FOR_CB {
            afs_lock_may_be_available(vnode);
        }

        if reason != afs_cb_break_for_deleted &&
           (*vnode).status.type_ == AFS_FTYPE_FILE &&
           atomic_read(&(*vnode).cb_nr_mmap) != 0 {
            queue_work(system_dfl_wq, &(*vnode).cb_work);
        }

        trace_afs_cb_break(&(*vnode).fid, (*vnode).cb_break, reason, true);
    } else {
        trace_afs_cb_break(&(*vnode).fid, (*vnode).cb_break, reason, false);
    }
}

pub unsafe fn afs_break_callback(vnode: *mut afs_vnode, reason: afs_cb_break_reason) {
    write_seqlock(&mut (*vnode).cb_lock);
    __afs_break_callback(vnode, reason);
    write_sequnlock(&mut (*vnode).cb_lock);
}

/*
 * Look up a volume by volume ID under RCU conditions.
 */
unsafe fn afs_lookup_volume_rcu(cell: *mut afs_cell, vid: afs_volid_t) -> *mut afs_volume {
    let mut volume: *mut afs_volume = core::ptr::null_mut();
    let mut p: *mut rb_node;

    scoped_seqlock_read!(&(*cell).volume_lock, ss_lock, {
        /* Unfortunately, rbtree walking doesn't give reliable results
         * under just the RCU read lock, so we have to check for
         * changes.
         */
        p = rcu_dereference_raw((*cell).volumes.rb_node);
        while !p.is_null() {
            volume = rb_entry!(p, afs_volume, cell_node);

            if (*volume).vid < vid {
                p = rcu_dereference_raw((*p).rb_left);
            } else if (*volume).vid > vid {
                p = rcu_dereference_raw((*p).rb_right);
            } else {
                break;
            }
            volume = core::ptr::null_mut();
        }

        if !volume.is_null() && afs_try_get_volume(volume, afs_volume_trace_get_callback) {
            break;
        }
        volume = core::ptr::null_mut();
    });

    volume
}

/*
 * Allow the fileserver to break callbacks at the volume-level.  This is
 * typically done when, for example, a R/W volume is snapshotted to a R/O
 * volume (the only way to change an R/O volume).  It may also, however, happen
 * when a volserver takes control of a volume (offlining it, moving it, etc.).
 *
 * Every file in that volume will need to be reevaluated.
 */
unsafe fn afs_break_volume_callback(server: *mut afs_server, volume: *mut afs_volume) {
    let slist = rcu_dereference((*volume).servers);
    let mut cb_v_break: u32;

    write_lock(&mut (*volume).cb_v_break_lock);

    for i in 0..(*slist).nr_servers {
        if (*slist).servers[i].server == server {
            (*slist).servers[i].cb_expires_at = AFS_NO_CB_PROMISE;
        }
    }
    (*volume).cb_expires_at = AFS_NO_CB_PROMISE;

    cb_v_break = atomic_inc_return_release(&mut (*volume).cb_v_break);
    trace_afs_cb_v_break((*volume).vid, cb_v_break, afs_cb_break_for_volume_callback);

    write_unlock(&mut (*volume).cb_v_break_lock);
    rcu_read_unlock();

    if !list_empty(&(*volume).open_mmaps) {
        afs_volume_init_callback(volume);
    }
}

/*
 * allow the fileserver to explicitly break one callback
 * - happens when
 *   - the backing file is changed
 *   - a lock is released
 */
unsafe fn afs_break_one_callback(server: *mut afs_server, volume: *mut afs_volume,
                                 fid: *mut afs_fid) {
    let sb: *mut super_block;
    let vnode: *mut afs_vnode;
    let inode: *mut inode;

    /* See if we can find a matching inode - even an I_NEW inode needs to
     * be marked as it can have its callback broken before we finish
     * setting up the local inode.
     */
    sb = rcu_dereference((*volume).sb);
    if sb.is_null() {
        return;
    }

    inode = find_inode_rcu(sb, (*fid).vnode, afs_ilookup5_test_by_fid, fid);
    if !inode.is_null() {
        vnode = AFS_FS_I(inode);
        afs_break_callback(vnode, afs_cb_break_for_callback);
    } else {
        trace_afs_cb_miss(fid, afs_cb_break_for_callback);
    }
}

unsafe fn afs_break_some_callbacks(server: *mut afs_server,
                                   cbb: *mut afs_callback_break,
                                   count: *mut usize) {
    let mut residue = cbb;
    let mut volume: *mut afs_volume;
    let vid = (*cbb).fid.vid;
    let mut i: usize;

    rcu_read_lock();
    volume = afs_lookup_volume_rcu((*server).cell, vid);
    if volume.is_null() {
        /* Ignore breaks on unknown volumes. */
        rcu_read_unlock();
        *count = 0;
    } else if (*cbb).fid.vnode == 0 && (*cbb).fid.unique == 0 {
        afs_break_volume_callback(server, volume);
        *count -= 1;
        if *count != 0 {
            memmove(cbb, cbb.add(1), core::mem::size_of::<afs_callback_break>() * *count);
        }
    } else {
        /* TODO: Find all matching volumes if we couldn't match the server and
         * break them anyway.
         */

        i = *count;
        while i > 0 {
            if (*cbb).fid.vid == vid {
                _debug!("- Fid {{ vl=%08llx n=%llu u=%u }}", (*cbb).fid.vid,
                       (*cbb).fid.vnode, (*cbb).fid.unique);
                *count -= 1;
                if !volume.is_null() {
                    afs_break_one_callback(server, volume, &mut (*cbb).fid);
                }
            } else {
                *residue = *cbb;
                residue = residue.add(1);
            }
            cbb = cbb.add(1);
            i -= 1;
        }
        rcu_read_unlock();
    }

    afs_put_volume(volume, afs_volume_trace_put_callback);
}

/*
 * allow the fileserver to break callback promises
 */
pub unsafe fn afs_break_callbacks(server: *mut afs_server, mut count: usize,
                                  callbacks: *mut afs_callback_break) {
    _enter!("%p,%zu,", server, count);

    ASSERT!(!server.is_null());

    while count > 0 {
        afs_break_some_callbacks(server, callbacks, &mut count);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
