// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS security handling
 *
 * Copyright (C) 2007, 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the kernel and the surrounding AFS implementation.

static mut afs_permits_cache: HasherTable = DEFINE_HASHTABLE!(10);
static afs_permits_lock: SpinLock = DEFINE_SPINLOCK!();
static afs_key_lock: Mutex = DEFINE_MUTEX!();

/* Allocate a key to use as a placeholder for anonymous user security. */
unsafe fn afs_alloc_anon_key(cell: *mut afs_cell) -> c_int {
    let mut key: *mut key;

    mutex_lock(&afs_key_lock);
    key = (*cell).anonymous_key;
    if key.is_null() {
        key = rxrpc_get_null_key((*cell).key_desc);
        if !IS_ERR(key) {
            (*cell).anonymous_key = key;
        }
    }
    mutex_unlock(&afs_key_lock);

    if IS_ERR(key) {
        return PTR_ERR(key);
    }
    _debug!("anon key %p{%x}", (*cell).anonymous_key, key_serial((*cell).anonymous_key));
    0
}

/* get a key */
pub unsafe fn afs_request_key(cell: *mut afs_cell) -> *mut key {
    let key: *mut key;
    let ret: c_int;

    _enter!("{%s}", (*cell).key_desc);
    _debug!("key %s", (*cell).key_desc);
    key = request_key_net(&key_type_rxrpc, (*cell).key_desc, (*(*cell).net).net, core::ptr::null_mut());
    if IS_ERR(key) {
        if PTR_ERR(key) != -ENOKEY {
            _leave!(" = %ld", PTR_ERR(key));
            return key;
        }
        if (*cell).anonymous_key.is_null() {
            ret = afs_alloc_anon_key(cell);
            if ret < 0 { return ERR_PTR(ret); }
        }
        _leave!(" = {%x} [anon]", key_serial((*cell).anonymous_key));
        return key_get((*cell).anonymous_key);
    }
    _leave!(" = {%x} [auth]", key_serial(key));
    key
}

/* Get a key when pathwalk is in rcuwalk mode. */
pub unsafe fn afs_request_key_rcu(cell: *mut afs_cell) -> *mut key {
    let key: *mut key;
    _enter!("{%s}", (*cell).key_desc);
    _debug!("key %s", (*cell).key_desc);
    key = request_key_net_rcu(&key_type_rxrpc, (*cell).key_desc, (*(*cell).net).net);
    if IS_ERR(key) {
        if PTR_ERR(key) != -ENOKEY {
            _leave!(" = %ld", PTR_ERR(key));
            return key;
        }
        if (*cell).anonymous_key.is_null() { return core::ptr::null_mut(); }
        _leave!(" = {%x} [anon]", key_serial((*cell).anonymous_key));
        return key_get((*cell).anonymous_key);
    }
    _leave!(" = {%x} [auth]", key_serial(key));
    key
}

/* Dispose of a list of permits. */
unsafe fn afs_permits_rcu(rcu: *mut rcu_head) {
    let permits = container_of!(rcu, afs_permits, rcu);
    for i in 0..(*permits).nr_permits {
        key_put((*permits).permits[i].key);
    }
    kfree(permits.cast());
}

/* Discard a permission cache. */
pub unsafe fn afs_put_permits(permits: *mut afs_permits) {
    if !permits.is_null() && refcount_dec_and_test(&mut (*permits).usage) {
        spin_lock(&afs_permits_lock);
        hash_del_rcu!(&mut (*permits).hash_node);
        spin_unlock(&afs_permits_lock);
        call_rcu!(&mut (*permits).rcu, afs_permits_rcu);
    }
}

/* Clear a permit cache on callback break. */
pub unsafe fn afs_clear_permits(vnode: *mut afs_vnode) {
    spin_lock(&(*vnode).lock);
    let permits = rcu_dereference_protected!((*vnode).permit_cache, lockdep_is_held!(&(*vnode).lock));
    RCU_INIT_POINTER!((*vnode).permit_cache, core::ptr::null_mut());
    spin_unlock(&(*vnode).lock);
    afs_put_permits(permits);
}

/* Hash a list of permits. */
unsafe fn afs_hash_permits(permits: *mut afs_permits) {
    let mut h = (*permits).nr_permits as c_ulong;
    for i in 0..(*permits).nr_permits {
        h = h.wrapping_add((*permits).permits[i].key as c_ulong / core::mem::size_of::<*mut c_void>() as c_ulong);
        h = h.wrapping_add((*permits).permits[i].access as c_ulong);
    }
    (*permits).h = h;
}

pub unsafe fn afs_cache_permit(vnode: *mut afs_vnode, key: *mut key, cb_break: c_uint, scb: *mut afs_status_cb) {
    let caller_access = (*scb).status.caller_access;
    let mut permits: *mut afs_permits;
    let mut xpermits: *mut afs_permits;
    let mut replacement: *mut afs_permits;
    let mut zap: *mut afs_permits;
    let mut new: *mut afs_permits = core::ptr::null_mut();
    let mut size: usize = 0;
    let mut changed = false;
    let mut i: usize;
    let mut j: usize;

    _enter!("{%llx:%llu},%x,%x", (*vnode).fid.vid, (*vnode).fid.vnode, key_serial(key), caller_access);
    rcu_read_lock();
    permits = rcu_dereference!((*vnode).permit_cache);
    if !permits.is_null() {
        if !(*permits).invalidated {
            for n in 0..(*permits).nr_permits {
                if (*permits).permits[n].key < key { continue; }
                if (*permits).permits[n].key > key { break; }
                if (*permits).permits[n].access != caller_access { changed = true; break; }
                if afs_cb_is_broken(cb_break, vnode) { changed = true; break; }
                rcu_read_unlock(); return;
            }
        }
        changed |= (*permits).invalidated;
        size = (*permits).nr_permits;
        if changed {
            spin_lock(&(*vnode).lock);
            if permits != rcu_access_pointer!((*vnode).permit_cache) {
                spin_unlock(&(*vnode).lock); rcu_read_unlock(); return;
            }
            RCU_INIT_POINTER!((*vnode).permit_cache, core::ptr::null_mut());
            spin_unlock(&(*vnode).lock);
            afs_put_permits(permits); permits = core::ptr::null_mut(); size = 0;
        }
    }
    if afs_cb_is_broken(cb_break, vnode) { rcu_read_unlock(); return; }
    if !permits.is_null() && !refcount_inc_not_zero(&mut (*permits).usage) { rcu_read_unlock(); return; }
    rcu_read_unlock();
    size += 1;
    new = kzalloc_flex!(size);
    if new.is_null() { afs_put_permits(permits); return; }
    refcount_set(&mut (*new).usage, 1); (*new).nr_permits = size; i = 0; j = 0;
    if !permits.is_null() {
        while i < (*permits).nr_permits {
            if j == i && (*permits).permits[i].key > key { (*new).permits[j] = afs_permit { key, access: caller_access }; j += 1; }
            (*new).permits[j] = (*permits).permits[i]; j += 1; i += 1;
        }
    }
    if j == i { (*new).permits[j] = afs_permit { key, access: caller_access }; }
    afs_hash_permits(new);
    spin_lock(&afs_permits_lock);
    hash_for_each_possible!(afs_permits_cache, xpermits, hash_node, (*new).h, {
        if (*xpermits).h != (*new).h || (*xpermits).invalidated || (*xpermits).nr_permits != (*new).nr_permits ||
           core::slice::from_raw_parts((*xpermits).permits.as_ptr() as *const u8, (*new).nr_permits * core::mem::size_of::<afs_permit>()) !=
           core::slice::from_raw_parts((*new).permits.as_ptr() as *const u8, (*new).nr_permits * core::mem::size_of::<afs_permit>()) { continue; }
        if refcount_inc_not_zero(&mut (*xpermits).usage) { replacement = xpermits; goto!(found); }
        break;
    });
    for n in 0..(*new).nr_permits { key_get((*new).permits[n].key); }
    hash_add_rcu!(afs_permits_cache, &mut (*new).hash_node, (*new).h);
    replacement = new; new = core::ptr::null_mut();
found:
    spin_unlock(&afs_permits_lock); kfree(new.cast());
    rcu_read_lock(); spin_lock(&(*vnode).lock); zap = rcu_access_pointer!((*vnode).permit_cache);
    if !afs_cb_is_broken(cb_break, vnode) && zap == permits { rcu_assign_pointer!((*vnode).permit_cache, replacement); } else { zap = replacement; }
    spin_unlock(&(*vnode).lock); rcu_read_unlock(); afs_put_permits(zap); afs_put_permits(permits);
}

unsafe fn afs_check_permit_rcu(vnode: *mut afs_vnode, key: *mut key, access: *mut afs_access_t) -> bool {
    if key == (*(*(*vnode).volume).cell).anonymous_key { *access = (*vnode).status.anon_access; return true; }
    let permits = rcu_dereference!((*vnode).permit_cache);
    if !permits.is_null() { for i in 0..(*permits).nr_permits { if (*permits).permits[i].key < key { continue; } if (*permits).permits[i].key > key { break; } *access = (*permits).permits[i].access; return !(*permits).invalidated; } }
    false
}

pub unsafe fn afs_check_permit(vnode: *mut afs_vnode, key: *mut key, access: *mut afs_access_t) -> c_int {
    let mut valid = false;
    if key == (*(*(*vnode).volume).cell).anonymous_key { *access = (*vnode).status.anon_access; valid = true; }
    else { rcu_read_lock(); let permits = rcu_dereference!((*vnode).permit_cache); if !permits.is_null() { for i in 0..(*permits).nr_permits { if (*permits).permits[i].key < key { continue; } if (*permits).permits[i].key > key { break; } *access = (*permits).permits[i].access; valid = !(*permits).invalidated; break; } } rcu_read_unlock(); }
    if !valid { let ret = afs_fetch_status(vnode, key, false, access); if ret < 0 { *access = 0; return ret; } }
    0
}

pub unsafe fn afs_permission(_idmap: *mut mnt_idmap, inode: *mut inode, mask: c_int) -> c_int {
    let vnode = AFS_FS_I(inode); let mut access: afs_access_t = 0; let key: *mut key;
    if mask & MAY_NOT_BLOCK != 0 { key = afs_request_key_rcu((*(*vnode).volume).cell); if IS_ERR_OR_NULL(key) { return -ECHILD; } if !afs_check_validity(vnode) || !afs_check_permit_rcu(vnode, key, &mut access) { key_put(key); return -ECHILD; } }
    else { key = afs_request_key((*(*vnode).volume).cell); if IS_ERR(key) { return PTR_ERR(key); } let ret = afs_validate(vnode, key); if ret < 0 { key_put(key); return ret; } let ret = afs_check_permit(vnode, key, &mut access); if ret < 0 { key_put(key); return ret; } }
    if S_ISDIR((*inode).i_mode) { if mask & (MAY_EXEC | MAY_READ | MAY_CHDIR) != 0 && access & AFS_ACE_LOOKUP == 0 { key_put(key); return -EACCES; } if mask & MAY_WRITE != 0 && access & (AFS_ACE_DELETE | AFS_ACE_INSERT) == 0 { key_put(key); return -EACCES; } }
    else { if access & AFS_ACE_LOOKUP == 0 || mask & MAY_EXEC != 0 && (*inode).i_mode & S_IXUSR == 0 || mask & (MAY_EXEC | MAY_READ) != 0 && (access & AFS_ACE_READ == 0 || (*inode).i_mode & S_IRUSR == 0) || mask & MAY_WRITE != 0 && (access & AFS_ACE_WRITE == 0 || (*inode).i_mode & S_IWUSR == 0) { key_put(key); return -EACCES; } }
    key_put(key); 0
}

pub unsafe fn afs_clean_up_permit_cache() { for i in 0..HASH_SIZE!(afs_permits_cache) { WARN_ON_ONCE!(!hlist_empty!(&afs_permits_cache[i])); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
