// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/fs/lockd/svcsubs.c
 *
 * Various support routines for the NLM server.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Kernel headers and lockd.h/share.h provide the types, constants, globals,
// and helper functions referenced below.

const FILE_HASH_BITS: usize = 7;
const FILE_NRHASH: usize = 1 << FILE_HASH_BITS;

static mut nlm_files: [hlist_head; FILE_NRHASH] = [/* supplied by kernel */ unsafe { core::mem::zeroed() }; FILE_NRHASH];
static mut nlm_file_mutex: mutex = unsafe { core::mem::zeroed() };

#[cfg(CONFIG_SUNRPC_DEBUG)]
#[inline]
unsafe fn nlm_debug_print_fh(msg: *mut c_char, f: *mut nfs_fh) {
    let fhp = (*f).data.as_ptr() as *const u32;
    dprintk!("lockd: %s (%08x %08x %08x %08x %08x %08x %08x %08x)\\n",
        msg, *fhp.add(0), *fhp.add(1), *fhp.add(2), *fhp.add(3),
        *fhp.add(4), *fhp.add(5), *fhp.add(6), *fhp.add(7));
}

#[cfg(CONFIG_SUNRPC_DEBUG)]
#[inline]
unsafe fn nlm_debug_print_file(msg: *mut c_char, file: *mut nlm_file) {
    let inode = nlmsvc_file_inode(file);
    dprintk!("lockd: %s %s/%llu\\n", msg, (*(*inode).i_sb).s_id, (*inode).i_ino);
}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
unsafe fn nlm_debug_print_fh(_msg: *mut c_char, _f: *mut nfs_fh) {}

#[cfg(not(CONFIG_SUNRPC_DEBUG))]
#[inline]
unsafe fn nlm_debug_print_file(_msg: *mut c_char, _file: *mut nlm_file) {}

#[inline]
unsafe fn file_hash(f: *mut nfs_fh) -> c_uint {
    let mut tmp: c_uint = 0;
    let mut i = 0;
    while i < LOCKD_FH_HASH_SIZE {
        tmp = tmp.wrapping_add((*f).data[i] as c_uint);
        i += 1;
    }
    tmp & (FILE_NRHASH as c_uint - 1)
}

pub unsafe fn lock_to_openmode(lock: *mut file_lock) -> c_int {
    if lock_is_write(lock) { O_WRONLY } else { O_RDONLY }
}

unsafe fn nlm_do_fopen(rqstp: *mut svc_rqst, file: *mut nlm_file, mode: c_int) -> __be32 {
    let mut ops: *const nlmsvc_binding;
    let mut nlmerr = nlm__int__failed;
    let mut deferred: __be32 = 0;
    let mut m = O_RDONLY;

    rcu_read_lock();
    ops = rcu_dereference(nlmsvc_ops);
    if ops.is_null() || !try_module_get((*ops).owner) {
        rcu_read_unlock();
        return nlm__int__failed;
    }
    rcu_read_unlock();

    while m <= O_WRONLY {
        let fp = &mut (*file).f_file[m as usize];
        if mode != O_RDWR && mode != m { m += 1; continue; }
        if !(*fp).is_null() {
            module_put((*ops).owner);
            return nlm_granted;
        }
        let error = ((*ops).fopen)(rqstp, &mut (*file).f_handle, fp, m);
        if error == 0 {
            module_put((*ops).owner);
            return nlm_granted;
        }
        dprintk!("lockd: open failed (errno %d)\\n", error);
        match error {
            -EWOULDBLOCK => { nlmerr = nlm__int__drop_reply; deferred = nlmerr; }
            -ESTALE => nlmerr = nlm__int__stale_fh,
            _ => nlmerr = nlm__int__failed,
        }
        m += 1;
    }
    module_put((*ops).owner);
    if deferred != 0 { deferred } else { nlmerr }
}

pub unsafe fn nlm_lookup_file(rqstp: *mut svc_rqst, result: *mut *mut nlm_file,
                              lock: *mut lockd_lock, mode: c_int) -> __be32 {
    let mut file: *mut nlm_file;
    let hash = file_hash(&mut (*lock).fh);
    nlm_debug_print_fh(c"nlm_lookup_file".as_ptr() as *mut c_char, &mut (*lock).fh);
    mutex_lock(&mut nlm_file_mutex);
    hlist_for_each_entry!(file, nlm_files[hash as usize], f_list) {
        if nfs_compare_fh(&mut (*file).f_handle, &mut (*lock).fh) == 0 {
            mutex_lock(&mut (*file).f_mutex);
            let err = nlm_do_fopen(rqstp, file, mode);
            mutex_unlock(&mut (*file).f_mutex);
            if err != 0 { mutex_unlock(&mut nlm_file_mutex); return err; }
            (*result) = file; (*file).f_count += 1;
            mutex_unlock(&mut nlm_file_mutex); return 0;
        }
    }
    nlm_debug_print_fh(c"creating file for".as_ptr() as *mut c_char, &mut (*lock).fh);
    file = kzalloc_obj!(*file);
    if file.is_null() { mutex_unlock(&mut nlm_file_mutex); return nlm_lck_denied_nolocks; }
    core::ptr::copy_nonoverlapping(&(*lock).fh, &mut (*file).f_handle, 1);
    mutex_init(&mut (*file).f_mutex); INIT_HLIST_NODE!(&mut (*file).f_list); INIT_LIST_HEAD!(&mut (*file).f_blocks);
    let err = nlm_do_fopen(rqstp, file, mode);
    if err != 0 { kfree(file); mutex_unlock(&mut nlm_file_mutex); return err; }
    hlist_add_head!(&mut (*file).f_list, &mut nlm_files[hash as usize]);
    (*result) = file; (*file).f_count += 1;
    mutex_unlock(&mut nlm_file_mutex); 0
}

unsafe fn nlm_release_files(file: *mut nlm_file) {
    let ops = rcu_dereference(nlmsvc_ops);
    if !ops.is_null() && try_module_get((*ops).owner) {
        if !(*file).f_file[O_RDONLY as usize].is_null() { ((*ops).fclose)((*file).f_file[O_RDONLY as usize]); }
        if !(*file).f_file[O_WRONLY as usize].is_null() { ((*ops).fclose)((*file).f_file[O_WRONLY as usize]); }
        module_put((*ops).owner);
    } else {
        if !(*file).f_file[O_RDONLY as usize].is_null() { fput((*file).f_file[O_RDONLY as usize]); }
        if !(*file).f_file[O_WRONLY as usize].is_null() { fput((*file).f_file[O_WRONLY as usize]); }
    }
}

#[inline]
unsafe fn nlm_delete_file(file: *mut nlm_file) {
    nlm_debug_print_file(c"closing file".as_ptr() as *mut c_char, file);
    if !hlist_unhashed(&mut (*file).f_list) { hlist_del(&mut (*file).f_list); nlm_release_files(file); kfree(file); }
    else { printk!(KERN_WARNING, "lockd: attempt to release unknown file!\\n"); }
}

unsafe fn nlm_unlock_files(file: *mut nlm_file, fl: *const file_lock) -> c_int {
    let mut lock: file_lock = core::mem::zeroed();
    locks_init_lock(&mut lock);
    lock.c.flc_type = F_UNLCK; lock.fl_start = 0; lock.fl_end = OFFSET_MAX;
    lock.c.flc_owner = (*fl).c.flc_owner; lock.c.flc_pid = (*fl).c.flc_pid; lock.c.flc_flags = FL_POSIX;
    lock.c.flc_file = (*file).f_file[O_RDONLY as usize];
    if !lock.c.flc_file.is_null() && vfs_lock_file(lock.c.flc_file, F_SETLK, &mut lock, core::ptr::null_mut()) != 0 { return 1; }
    lock.c.flc_file = (*file).f_file[O_WRONLY as usize];
    if !lock.c.flc_file.is_null() && vfs_lock_file(lock.c.flc_file, F_SETLK, &mut lock, core::ptr::null_mut()) != 0 { return 1; }
    0
}

unsafe fn nlm_traverse_locks(host: *mut nlm_host, file: *mut nlm_file, match_fn: nlm_host_match_fn_t) -> c_int {
    let inode = nlmsvc_file_inode(file); let flctx = locks_inode_context(inode);
    if flctx.is_null() || list_empty_careful(&mut (*flctx).flc_posix) { return 0; }
    loop {
        (*file).f_locks = 0; spin_lock(&mut (*flctx).flc_lock);
        for_each_file_lock!(fl, (*flctx).flc_posix) {
            if (*fl).fl_lmops != &nlmsvc_lock_operations { continue; }
            (*file).f_locks += 1;
            let lockhost = (*(fl as *mut nlm_lockowner)).host;
            if match_fn(lockhost as *mut _, host) { spin_unlock(&mut (*flctx).flc_lock); if nlm_unlock_files(file, fl) != 0 { return 1; } continue; }
        }
        spin_unlock(&mut (*flctx).flc_lock); return 0;
    }
}

unsafe fn nlmsvc_always_match(_dummy1: *mut c_void, _dummy2: *mut nlm_host) -> c_int { 1 }

#[inline]
unsafe fn nlm_inspect_file(host: *mut nlm_host, file: *mut nlm_file, match_fn: nlm_host_match_fn_t) -> c_int {
    nlmsvc_traverse_blocks(host, file, match_fn); nlmsvc_traverse_shares(host, file, match_fn); nlm_traverse_locks(host, file, match_fn)
}

#[inline]
unsafe fn nlm_file_inuse(file: *mut nlm_file) -> c_int {
    let inode = nlmsvc_file_inode(file); let flctx = locks_inode_context(inode);
    if (*file).f_count != 0 || !list_empty(&mut (*file).f_blocks) || (*file).f_shares != 0 { return 1; }
    if !flctx.is_null() && !list_empty_careful(&mut (*flctx).flc_posix) { spin_lock(&mut (*flctx).flc_lock); for_each_file_lock!(fl, (*flctx).flc_posix) { if (*fl).fl_lmops == &nlmsvc_lock_operations { spin_unlock(&mut (*flctx).flc_lock); return 1; } } spin_unlock(&mut (*flctx).flc_lock); }
    (*file).f_locks = 0; 0
}

unsafe fn nlm_file_release(file: *mut nlm_file) { if nlm_file_inuse(file) == 0 { nlm_delete_file(file); } }

unsafe fn nlm_traverse_files(data: *mut c_void, match_fn: nlm_host_match_fn_t, is_failover_file: Option<unsafe extern "C" fn(*mut c_void, *mut nlm_file) -> c_int>) -> c_int {
    let mut ret = 0; mutex_lock(&mut nlm_file_mutex);
    for i in 0..FILE_NRHASH { let mut file = hlist_entry_safe!(nlm_files[i].first, nlm_file, f_list); if !file.is_null() { (*file).f_count += 1; }
        while !file.is_null() { let next = hlist_entry_safe!((*file).f_list.next, nlm_file, f_list); if !next.is_null() { (*next).f_count += 1; }
            if is_failover_file.is_none() || is_failover_file.unwrap()(data, file) != 0 { mutex_unlock(&mut nlm_file_mutex); if nlm_inspect_file(data as *mut nlm_host, file, match_fn) != 0 { ret = 1; } mutex_lock(&mut nlm_file_mutex); }
            (*file).f_count -= 1; nlm_file_release(file); file = next;
        }
    }
    mutex_unlock(&mut nlm_file_mutex); ret
}

pub unsafe fn nlm_release_file(file: *mut nlm_file) { mutex_lock(&mut nlm_file_mutex); (*file).f_count -= 1; if (*file).f_count == 0 && nlm_file_inuse(file) == 0 { nlm_delete_file(file); } mutex_unlock(&mut nlm_file_mutex); }

unsafe fn nlmsvc_mark_host(data: *mut c_void, hint: *mut nlm_host) -> c_int { let host = data as *mut nlm_host; if (*hint).net.is_null() || (*host).net == (*hint).net { (*host).h_inuse = 1; } 0 }
unsafe fn nlmsvc_same_host(data: *mut c_void, other: *mut nlm_host) -> c_int { (data as *mut nlm_host == other) as c_int }
unsafe fn nlmsvc_is_client(data: *mut c_void, _dummy: *mut nlm_host) -> c_int { let host = data as *mut nlm_host; if (*host).h_server { if !(*host).h_nsmhandle.is_null() { (*(*host).h_nsmhandle).sm_sticky = 1; } 1 } else { 0 } }

pub unsafe fn nlmsvc_mark_resources(net: *mut net) { let mut hint: nlm_host = core::mem::zeroed(); hint.net = net; nlm_traverse_files(&mut hint as *mut _ as *mut c_void, nlmsvc_mark_host, None); }
pub unsafe fn nlmsvc_free_host_resources(host: *mut nlm_host) { if nlm_traverse_files(host as *mut c_void, nlmsvc_same_host, None) != 0 { BUG!(); } }
pub unsafe fn nlmsvc_invalidate_all() { nlm_traverse_files(core::ptr::null_mut(), nlmsvc_is_client, None); }

unsafe fn nlmsvc_match_sb(data: *mut c_void, file: *mut nlm_file) -> c_int { (data as *mut super_block == nlmsvc_file_inode(file).as_ref().unwrap().i_sb) as c_int }
pub unsafe fn nlmsvc_unlock_all_by_sb(sb: *mut super_block) -> c_int { if nlm_traverse_files(sb as *mut c_void, nlmsvc_always_match, Some(nlmsvc_match_sb)) != 0 { -EIO } else { 0 } }
unsafe fn nlmsvc_match_ip(data: *mut c_void, host: *mut nlm_host) -> c_int { rpc_cmp_addr(nlm_srcaddr(data), host as *mut sockaddr) }
pub unsafe fn nlmsvc_unlock_all_by_ip(server_addr: *mut sockaddr) -> c_int { if nlm_traverse_files(server_addr as *mut c_void, nlmsvc_match_ip, None) != 0 { -EIO } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
