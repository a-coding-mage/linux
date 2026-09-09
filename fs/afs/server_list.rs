// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS fileserver list management.
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependencies: <linux/kernel.h>, <linux/slab.h>, and "internal.h".

pub unsafe fn afs_put_serverlist(net: *mut afs_net, slist: *mut afs_server_list) {
    let mut i: c_int;

    if !slist.is_null() && refcount_dec_and_test(&mut (*slist).usage) {
        i = 0;
        while i < (*slist).nr_servers {
            afs_unuse_server(
                net,
                (*slist).servers[i as usize].server,
                afs_server_trace_unuse_slist,
            );
            i += 1;
        }
        kfree_rcu(slist, rcu);
    }
}

/*
 * Build a server list from a VLDB record.
 */
pub unsafe fn afs_alloc_server_list(
    volume: *mut afs_volume,
    key: *mut key,
    vldb: *mut afs_vldb_entry,
) -> *mut afs_server_list {
    let mut slist: *mut afs_server_list;
    let mut server: *mut afs_server;
    let type_mask: c_uint = 1u32 << (*volume).type_;
    let mut use_newrepsites = false;
    let mut ret: c_int = -ENOMEM;
    let mut nr_servers: c_int = 0;
    let mut newrep: c_int = 0;
    let mut i: c_int;
    let mut j: c_int;
    let mut usable: c_int = 0;

    /* Work out if we're going to restrict to NEWREPSITE-marked servers or
     * not.  If at least one site is marked as NEWREPSITE, then it's likely
     * that "vos release" is busy updating RO sites.  We cut over from one
     * to the other when >=50% of the sites have been updated.  Sites that
     * are in the process of being updated are marked DONTUSE.
     */
    i = 0;
    while i < (*vldb).nr_servers {
        if ((*vldb).fs_mask[i as usize] & type_mask) == 0 {
            i += 1;
            continue;
        }
        nr_servers += 1;
        if ((*vldb).vlsf_flags[i as usize] & AFS_VLSF_DONTUSE) != 0 {
            i += 1;
            continue;
        }
        usable += 1;
        if ((*vldb).vlsf_flags[i as usize] & AFS_VLSF_NEWREPSITE) != 0 {
            newrep += 1;
        }
        i += 1;
    }

    slist = kzalloc_flex(nr_servers);
    if slist.is_null() {
        return ERR_PTR(ret);
    }

    if newrep != 0 {
        if newrep < usable / 2 {
            (*slist).ro_replicating = AFS_RO_REPLICATING_USE_OLD;
        } else {
            (*slist).ro_replicating = AFS_RO_REPLICATING_USE_NEW;
            use_newrepsites = true;
        }
    }

    refcount_set(&mut (*slist).usage, 1);
    rwlock_init(&mut (*slist).lock);

    /* Make sure a records exists for each server in the list. */
    i = 0;
    while i < (*vldb).nr_servers {
        let mut se_flags: c_ulong = 0;
        let newrepsite = ((*vldb).vlsf_flags[i as usize] & AFS_VLSF_NEWREPSITE) != 0;

        if ((*vldb).fs_mask[i as usize] & type_mask) == 0 {
            i += 1;
            continue;
        }
        if ((*vldb).vlsf_flags[i as usize] & AFS_VLSF_DONTUSE) != 0 {
            __set_bit(AFS_SE_EXCLUDED, &mut se_flags);
        }
        if newrep != 0 && (newrepsite != use_newrepsites) {
            __set_bit(AFS_SE_EXCLUDED, &mut se_flags);
        }

        server = afs_lookup_server(
            (*(*volume).cell).net,
            key,
            &(*vldb).fs_server[i as usize],
            (*vldb).addr_version[i as usize],
        );
        if IS_ERR(server) {
            ret = PTR_ERR(server);
            if ret == -ENOENT || ret == -ENOMEDIUM {
                i += 1;
                continue;
            }
            afs_put_serverlist((*(*volume).cell).net, slist);
            return ERR_PTR(ret);
        }

        /* Insertion-sort by UUID */
        j = 0;
        while j < (*slist).nr_servers {
            if memcmp(
                &(*(*slist).servers[j as usize].server).uuid,
                &(*server).uuid,
                core::mem::size_of_val(&(*server).uuid),
            ) >= 0 {
                break;
            }
            j += 1;
        }
        if j < (*slist).nr_servers {
            if (*slist).servers[j as usize].server == server {
                afs_unuse_server_notime(
                    (*(*volume).cell).net,
                    server,
                    afs_server_trace_unuse_slist_isort,
                );
                i += 1;
                continue;
            }
            memmove(
                (*slist).servers.as_mut_ptr().add(j as usize + 1),
                (*slist).servers.as_ptr().add(j as usize),
                ((*slist).nr_servers - j) as usize * core::mem::size_of::<afs_server_entry>(),
            );
        }

        (*slist).servers[j as usize].server = server;
        (*slist).servers[j as usize].volume = volume;
        (*slist).servers[j as usize].flags = se_flags;
        (*slist).servers[j as usize].cb_expires_at = AFS_NO_CB_PROMISE;
        (*slist).nr_servers += 1;
        i += 1;
    }

    if (*slist).nr_servers == 0 {
        ret = -EDESTADDRREQ;
        afs_put_serverlist((*(*volume).cell).net, slist);
        return ERR_PTR(ret);
    }
    slist
}

/* Copy the annotations from an old server list to its potential replacement. */
pub unsafe fn afs_annotate_server_list(new: *mut afs_server_list, old: *mut afs_server_list) -> bool {
    let mask: c_ulong = 1u64 << AFS_SE_EXCLUDED;
    if (*old).nr_servers != (*new).nr_servers || (*old).ro_replicating != (*new).ro_replicating {
        return true;
    }
    let mut i = 0;
    while i < (*old).nr_servers {
        if (*old).servers[i as usize].server != (*new).servers[i as usize].server
            || ((*old).servers[i as usize].flags & mask) != ((*new).servers[i as usize].flags & mask)
        {
            return true;
        }
        i += 1;
    }
    false
}

/* Attach a volume to the servers it is going to use. */
pub unsafe fn afs_attach_volume_to_servers(volume: *mut afs_volume, slist: *mut afs_server_list) {
    down_write(&mut (*(*volume).cell).vs_lock);
    let mut i = 0;
    while i < (*slist).nr_servers {
        let se = &mut (*slist).servers[i as usize];
        let server = se.server;
        let mut p: *mut list_head = &mut (*server).volumes;
        while !list_at_end(p, &(*server).volumes) {
            let pe = list_entry(p, afs_server_entry, slink);
            if (*volume).vid <= (*pe).volume.as_ref().vid { break; }
            p = (*p).next;
        }
        list_add_tail(&mut se.slink, p);
        i += 1;
    }
    (*slist).attached = true;
    up_write(&mut (*(*volume).cell).vs_lock);
}

/* Reattach a volume to the servers it is going to use when server list is
 * replaced.  We try to switch the attachment points to avoid rewalking the
 * lists.
 */
pub unsafe fn afs_reattach_volume_to_servers(volume: *mut afs_volume, new: *mut afs_server_list, old: *mut afs_server_list) {
    let mut n = 0;
    let mut o = 0;
    down_write(&mut (*(*volume).cell).vs_lock);
    while n < (*new).nr_servers || o < (*old).nr_servers {
        let pn = if n < (*new).nr_servers { &mut (*new).servers[n as usize] as *mut _ } else { core::ptr::null_mut() };
        let po = if o < (*old).nr_servers { &mut (*old).servers[o as usize] as *mut _ } else { core::ptr::null_mut() };
        if !pn.is_null() && !po.is_null() && (*pn).server == (*po).server {
            (*pn).cb_expires_at = (*po).cb_expires_at;
            list_replace(&mut (*po).slink, &mut (*pn).slink);
            n += 1; o += 1; continue;
        }
        let diff = if !pn.is_null() && !po.is_null() { memcmp(&(*(*pn).server).uuid, &(*(*po).server).uuid, core::mem::size_of_val(&(*(*pn).server).uuid)) } else if !pn.is_null() { -1 } else { 1 };
        if diff < 0 {
            let mut p = &mut (*(*pn).server).volumes as *mut _;
            while !list_at_end(p, &(*(*pn).server).volumes) { let s = list_entry(p, afs_server_entry, slink); if (*volume).vid <= (*s).volume.as_ref().vid { break; } p = (*p).next; }
            list_add_tail(&mut (*pn).slink, p); n += 1;
        } else { list_del(&mut (*po).slink); o += 1; }
    }
    up_write(&mut (*(*volume).cell).vs_lock);
}

/* Detach a volume from the servers it has been using. */
pub unsafe fn afs_detach_volume_from_servers(volume: *mut afs_volume, slist: *mut afs_server_list) {
    if !(*slist).attached { return; }
    down_write(&mut (*(*volume).cell).vs_lock);
    let mut i = 0;
    while i < (*slist).nr_servers { list_del(&mut (*slist).servers[i as usize].slink); i += 1; }
    (*slist).attached = false;
    up_write(&mut (*(*volume).cell).vs_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
