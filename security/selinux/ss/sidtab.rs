// SPDX-License-Identifier: GPL-2.0
/*
 * Implementation of the SID table type.
 *
 * Original author: Stephen Smalley, <stephen.smalley.work@gmail.com>
 * Author: Ondrej Mosnacek, <omosnacek@gmail.com>
 *
 * Copyright (C) 2018 Red Hat, Inc.
 */

/* Dependencies from the original C includes:
 * <linux/errno.h>, <linux/kernel.h>, <linux/list.h>, <linux/rcupdate.h>,
 * <linux/slab.h>, <linux/sched.h>, <linux/spinlock.h>, <asm/barrier.h>,
 * "flask.h", "security.h", "sidtab.h", and "services.h".
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of_val;
use core::ptr;

#[repr(C)]
pub struct sidtab_str_cache {
    pub rcu_member: rcu_head,
    pub lru_member: list_head,
    pub parent: *mut sidtab_entry,
    pub len: u32,
    pub str: [c_char; 0],
}

#[inline]
unsafe fn index_to_sid(index: u32) -> u32 {
    index + SECINITSID_NUM + 1
}

#[inline]
unsafe fn sid_to_index(sid: u32) -> u32 {
    sid - (SECINITSID_NUM + 1)
}

pub unsafe extern "C" fn sidtab_init(s: *mut sidtab) -> c_int {
    let mut i: u32;

    memset(
        (*s).roots.as_mut_ptr() as *mut c_void,
        0,
        size_of_val(&(*s).roots),
    );

    i = 0;
    while i < SECINITSID_NUM {
        (*s).isids[i as usize].set = 0;
        i += 1;
    }

    (*s).frozen = false;
    (*s).count = 0;
    (*s).convert = ptr::null_mut();
    hash_init!((*s).context_to_sid);

    spin_lock_init(&mut (*s).lock);

    /* #if CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 */
    if CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 {
        (*s).cache_free_slots = CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE;
        INIT_LIST_HEAD(&mut (*s).cache_lru_list);
        spin_lock_init(&mut (*s).cache_lock);
    }

    0
}

unsafe fn context_to_sid(s: *mut sidtab, context: *mut context, hash: u32) -> u32 {
    let mut sid: u32 = 0;

    rcu_read_lock();
    hash_for_each_possible_rcu!((*s).context_to_sid, entry: *mut sidtab_entry, list, hash, {
        if (*entry).hash != hash {
            continue;
        }
        if context_equal(&mut (*entry).context, context) {
            sid = (*entry).sid;
            break;
        }
    });
    rcu_read_unlock();
    sid
}

pub unsafe extern "C" fn sidtab_set_initial(
    s: *mut sidtab,
    sid: u32,
    context: *mut context,
) -> c_int {
    let isid: *mut sidtab_isid_entry;
    let hash: u32;
    let rc: c_int;

    if sid == 0 || sid > SECINITSID_NUM {
        return -EINVAL;
    }

    isid = &mut (*s).isids[(sid - 1) as usize];

    rc = context_cpy(&mut (*isid).entry.context, context);
    if rc != 0 {
        return rc;
    }

    /* #if CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 */
    if CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 {
        (*isid).entry.cache = ptr::null_mut();
    }
    (*isid).set = 1;

    hash = context_compute_hash(context);

    /*
     * Multiple initial sids may map to the same context. Check that this
     * context is not already represented in the context_to_sid hashtable
     * to avoid duplicate entries and long linked lists upon hash
     * collision.
     */
    if context_to_sid(s, context, hash) == 0 {
        (*isid).entry.sid = sid;
        (*isid).entry.hash = hash;
        hash_add!((*s).context_to_sid, &mut (*isid).entry.list, hash);
    }

    0
}

pub unsafe extern "C" fn sidtab_hash_stats(sidtab: *mut sidtab, page: *mut c_char) -> c_int {
    let mut chain_len: c_int = 0;
    let mut slots_used: c_int = 0;
    let mut entries: c_int = 0;
    let mut max_chain_len: c_int = 0;
    let mut cur_bucket: u32 = 0;

    rcu_read_lock();
    hash_for_each_rcu!((*sidtab).context_to_sid, i: u32, entry: *mut sidtab_entry, list, {
        entries += 1;
        if i == cur_bucket {
            chain_len += 1;
            if chain_len == 1 {
                slots_used += 1;
            }
        } else {
            cur_bucket = i;
            if chain_len > max_chain_len {
                max_chain_len = chain_len;
            }
            chain_len = 0;
        }
    });
    rcu_read_unlock();

    if chain_len > max_chain_len {
        max_chain_len = chain_len;
    }

    scnprintf(
        page,
        PAGE_SIZE,
        c_str!("entries: %d\nbuckets used: %d/%d\nlongest chain: %d\n"),
        entries,
        slots_used,
        SIDTAB_HASH_BUCKETS,
        max_chain_len,
    )
}

unsafe fn sidtab_level_from_count(count: u32) -> u32 {
    let mut capacity: u32 = SIDTAB_LEAF_ENTRIES;
    let mut level: u32 = 0;

    while count > capacity {
        capacity <<= SIDTAB_INNER_SHIFT;
        level += 1;
    }
    level
}

unsafe fn sidtab_alloc_roots(s: *mut sidtab, level: u32) -> c_int {
    let mut l: u32;

    if (*s).roots[0].ptr_leaf.is_null() {
        (*s).roots[0].ptr_leaf = kzalloc(SIDTAB_NODE_ALLOC_SIZE, GFP_ATOMIC) as *mut sidtab_node_leaf;
        if (*s).roots[0].ptr_leaf.is_null() {
            return -ENOMEM;
        }
    }
    l = 1;
    while l <= level {
        if (*s).roots[l as usize].ptr_inner.is_null() {
            (*s).roots[l as usize].ptr_inner =
                kzalloc(SIDTAB_NODE_ALLOC_SIZE, GFP_ATOMIC) as *mut sidtab_node_inner;
            if (*s).roots[l as usize].ptr_inner.is_null() {
                return -ENOMEM;
            }
            (*(*s).roots[l as usize].ptr_inner).entries[0] = (*s).roots[(l - 1) as usize];
        }
        l += 1;
    }
    0
}

unsafe fn sidtab_do_lookup(s: *mut sidtab, index: u32, alloc: c_int) -> *mut sidtab_entry {
    let mut entry: *mut sidtab_entry_inner;
    let mut level: u32;
    let mut capacity_shift: u32;
    let mut leaf_index: u32 = index / SIDTAB_LEAF_ENTRIES;

    /* find the level of the subtree we need */
    level = sidtab_level_from_count(index + 1);
    capacity_shift = level * SIDTAB_INNER_SHIFT;

    /* allocate roots if needed */
    if alloc != 0 && sidtab_alloc_roots(s, level) != 0 {
        return ptr::null_mut();
    }

    /* lookup inside the subtree */
    entry = &mut (*s).roots[level as usize];
    while level != 0 {
        capacity_shift -= SIDTAB_INNER_SHIFT;
        level -= 1;

        entry = &mut (*(*entry).ptr_inner).entries[(leaf_index >> capacity_shift) as usize];
        leaf_index &= (1u32 << capacity_shift) - 1;

        if (*entry).ptr_inner.is_null() {
            if alloc != 0 {
                (*entry).ptr_inner =
                    kzalloc(SIDTAB_NODE_ALLOC_SIZE, GFP_ATOMIC) as *mut sidtab_node_inner;
            }
            if (*entry).ptr_inner.is_null() {
                return ptr::null_mut();
            }
        }
    }
    if (*entry).ptr_leaf.is_null() {
        if alloc != 0 {
            (*entry).ptr_leaf =
                kzalloc(SIDTAB_NODE_ALLOC_SIZE, GFP_ATOMIC) as *mut sidtab_node_leaf;
        }
        if (*entry).ptr_leaf.is_null() {
            return ptr::null_mut();
        }
    }
    &mut (*(*entry).ptr_leaf).entries[(index % SIDTAB_LEAF_ENTRIES) as usize]
}

unsafe fn sidtab_lookup(s: *mut sidtab, index: u32) -> *mut sidtab_entry {
    /* read entries only after reading count */
    let count: u32 = smp_load_acquire(&mut (*s).count);

    if index >= count {
        return ptr::null_mut();
    }

    sidtab_do_lookup(s, index, 0)
}

unsafe fn sidtab_lookup_initial(s: *mut sidtab, sid: u32) -> *mut sidtab_entry {
    if (*s).isids[(sid - 1) as usize].set != 0 {
        &mut (*s).isids[(sid - 1) as usize].entry
    } else {
        ptr::null_mut()
    }
}

unsafe fn sidtab_search_core(s: *mut sidtab, sid: u32, force: c_int) -> *mut sidtab_entry {
    if sid != 0 {
        let entry: *mut sidtab_entry;

        if sid > SECINITSID_NUM {
            entry = sidtab_lookup(s, sid_to_index(sid));
        } else {
            entry = sidtab_lookup_initial(s, sid);
        }
        if !entry.is_null() && ((*entry).context.len == 0 || force != 0) {
            return entry;
        }
    }

    sidtab_lookup_initial(s, SECINITSID_UNLABELED)
}

pub unsafe extern "C" fn sidtab_search_entry(s: *mut sidtab, sid: u32) -> *mut sidtab_entry {
    sidtab_search_core(s, sid, 0)
}

pub unsafe extern "C" fn sidtab_search_entry_force(s: *mut sidtab, sid: u32) -> *mut sidtab_entry {
    sidtab_search_core(s, sid, 1)
}

pub unsafe extern "C" fn sidtab_context_to_sid(
    s: *mut sidtab,
    context: *mut context,
    sid: *mut u32,
) -> c_int {
    let mut flags: c_ulong = 0;
    let count: u32;
    let hash: u32 = context_compute_hash(context);
    let convert: *mut sidtab_convert_params;
    let dst: *mut sidtab_entry;
    let dst_convert: *mut sidtab_entry;
    let mut rc: c_int;

    *sid = context_to_sid(s, context, hash);
    if *sid != 0 {
        return 0;
    }

    /* lock-free search failed: lock, re-search, and insert if not found */
    spin_lock_irqsave(&mut (*s).lock, &mut flags);

    rc = 0;
    *sid = context_to_sid(s, context, hash);
    if *sid != 0 {
        goto_out_unlock!(rc, s, flags);
        return rc;
    }

    if unlikely((*s).frozen) {
        /*
         * This sidtab is now frozen - tell the caller to abort and
         * get the new one.
         */
        rc = -ESTALE;
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return rc;
    }

    count = (*s).count;

    /* bail out if we already reached max entries */
    rc = -EOVERFLOW;
    if count >= SIDTAB_MAX {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return rc;
    }

    /* insert context into new entry */
    rc = -ENOMEM;
    dst = sidtab_do_lookup(s, count, 1);
    if dst.is_null() {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return rc;
    }

    (*dst).sid = index_to_sid(count);
    (*dst).hash = hash;

    rc = context_cpy(&mut (*dst).context, context);
    if rc != 0 {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return rc;
    }

    /*
     * if we are building a new sidtab, we need to convert the context
     * and insert it there as well
     */
    convert = (*s).convert;
    if !convert.is_null() {
        let target: *mut sidtab = (*convert).target;

        rc = -ENOMEM;
        dst_convert = sidtab_do_lookup(target, count, 1);
        if dst_convert.is_null() {
            context_destroy(&mut (*dst).context);
            spin_unlock_irqrestore(&mut (*s).lock, flags);
            return rc;
        }

        rc = services_convert_context(
            (*convert).args,
            context,
            &mut (*dst_convert).context,
            GFP_ATOMIC,
        );
        if rc != 0 {
            context_destroy(&mut (*dst).context);
            spin_unlock_irqrestore(&mut (*s).lock, flags);
            return rc;
        }
        (*dst_convert).sid = index_to_sid(count);
        (*dst_convert).hash = context_compute_hash(&mut (*dst_convert).context);
        (*target).count = count + 1;

        hash_add_rcu!(
            (*target).context_to_sid,
            &mut (*dst_convert).list,
            (*dst_convert).hash
        );
    }

    if (*context).len != 0 {
        pr_info(
            c_str!("SELinux:  Context %s is not valid (left unmapped).\n"),
            (*context).str,
        );
    }

    *sid = index_to_sid(count);

    /* write entries before updating count */
    smp_store_release(&mut (*s).count, count + 1);
    hash_add_rcu!((*s).context_to_sid, &mut (*dst).list, (*dst).hash);

    rc = 0;
    spin_unlock_irqrestore(&mut (*s).lock, flags);
    rc
}

unsafe fn sidtab_convert_hashtable(s: *mut sidtab, count: u32) {
    let mut i: u32;
    let mut entry: *mut sidtab_entry;

    i = 0;
    while i < count {
        entry = sidtab_do_lookup(s, i, 0);
        (*entry).sid = index_to_sid(i);
        (*entry).hash = context_compute_hash(&mut (*entry).context);

        hash_add_rcu!((*s).context_to_sid, &mut (*entry).list, (*entry).hash);
        i += 1;
    }
}

unsafe fn sidtab_convert_tree(
    edst: *mut sidtab_entry_inner,
    esrc: *mut sidtab_entry_inner,
    pos: *mut u32,
    count: u32,
    level: u32,
    convert: *mut sidtab_convert_params,
) -> c_int {
    let mut rc: c_int;
    let mut i: u32;

    if level != 0 {
        if (*edst).ptr_inner.is_null() {
            (*edst).ptr_inner =
                kzalloc(SIDTAB_NODE_ALLOC_SIZE, GFP_KERNEL) as *mut sidtab_node_inner;
            if (*edst).ptr_inner.is_null() {
                return -ENOMEM;
            }
        }
        i = 0;
        while i < SIDTAB_INNER_ENTRIES && *pos < count {
            rc = sidtab_convert_tree(
                &mut (*(*edst).ptr_inner).entries[i as usize],
                &mut (*(*esrc).ptr_inner).entries[i as usize],
                pos,
                count,
                level - 1,
                convert,
            );
            if rc != 0 {
                return rc;
            }
            i += 1;
        }
    } else {
        if (*edst).ptr_leaf.is_null() {
            (*edst).ptr_leaf =
                kzalloc(SIDTAB_NODE_ALLOC_SIZE, GFP_KERNEL) as *mut sidtab_node_leaf;
            if (*edst).ptr_leaf.is_null() {
                return -ENOMEM;
            }
        }
        i = 0;
        while i < SIDTAB_LEAF_ENTRIES && *pos < count {
            rc = services_convert_context(
                (*convert).args,
                &mut (*(*esrc).ptr_leaf).entries[i as usize].context,
                &mut (*(*edst).ptr_leaf).entries[i as usize].context,
                GFP_KERNEL,
            );
            if rc != 0 {
                return rc;
            }
            *pos += 1;
            i += 1;
        }
        cond_resched();
    }
    0
}

pub unsafe extern "C" fn sidtab_convert(
    s: *mut sidtab,
    params: *mut sidtab_convert_params,
) -> c_int {
    let mut flags: c_ulong = 0;
    let count: u32;
    let level: u32;
    let mut pos: u32;
    let mut rc: c_int;

    spin_lock_irqsave(&mut (*s).lock, &mut flags);

    /* concurrent policy loads are not allowed */
    if !(*s).convert.is_null() {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return -EBUSY;
    }

    count = (*s).count;
    level = sidtab_level_from_count(count);

    /* allocate last leaf in the new sidtab (to avoid race with
     * live convert)
     */
    rc = if !sidtab_do_lookup((*params).target, count - 1, 1).is_null() {
        0
    } else {
        -ENOMEM
    };
    if rc != 0 {
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return rc;
    }

    /* set count in case no new entries are added during conversion */
    (*(*params).target).count = count;

    /* enable live convert of new entries */
    (*s).convert = params;

    /* we can safely convert the tree outside the lock */
    spin_unlock_irqrestore(&mut (*s).lock, flags);

    pr_info(c_str!("SELinux:  Converting %u SID table entries...\n"), count);

    /* convert all entries not covered by live convert */
    pos = 0;
    rc = sidtab_convert_tree(
        &mut (*(*params).target).roots[level as usize],
        &mut (*s).roots[level as usize],
        &mut pos,
        count,
        level,
        params,
    );
    if rc != 0 {
        /* we need to keep the old table - disable live convert */
        spin_lock_irqsave(&mut (*s).lock, &mut flags);
        (*s).convert = ptr::null_mut();
        spin_unlock_irqrestore(&mut (*s).lock, flags);
        return rc;
    }
    /*
     * The hashtable can also be modified in sidtab_context_to_sid()
     * so we must re-acquire the lock here.
     */
    spin_lock_irqsave(&mut (*s).lock, &mut flags);
    sidtab_convert_hashtable((*params).target, count);
    spin_unlock_irqrestore(&mut (*s).lock, flags);

    0
}

pub unsafe extern "C" fn sidtab_cancel_convert(s: *mut sidtab) {
    let mut flags: c_ulong = 0;

    /* cancelling policy load - disable live convert of sidtab */
    spin_lock_irqsave(&mut (*s).lock, &mut flags);
    (*s).convert = ptr::null_mut();
    spin_unlock_irqrestore(&mut (*s).lock, flags);
}

/* Original annotations: __acquires(&s->lock) */
pub unsafe extern "C" fn sidtab_freeze_begin(s: *mut sidtab, flags: *mut c_ulong) {
    spin_lock_irqsave(&mut (*s).lock, flags);
    (*s).frozen = true;
    (*s).convert = ptr::null_mut();
}

/* Original annotations: __releases(&s->lock) */
pub unsafe extern "C" fn sidtab_freeze_end(s: *mut sidtab, flags: *mut c_ulong) {
    spin_unlock_irqrestore(&mut (*s).lock, *flags);
}

unsafe fn sidtab_destroy_entry(entry: *mut sidtab_entry) {
    context_destroy(&mut (*entry).context);
    /* #if CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 */
    if CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 {
        kfree(rcu_dereference_raw((*entry).cache) as *mut c_void);
    }
}

unsafe fn sidtab_destroy_tree(entry: sidtab_entry_inner, level: u32) {
    let mut i: u32;

    if level != 0 {
        let node: *mut sidtab_node_inner = entry.ptr_inner;

        if node.is_null() {
            return;
        }

        i = 0;
        while i < SIDTAB_INNER_ENTRIES {
            sidtab_destroy_tree((*node).entries[i as usize], level - 1);
            i += 1;
        }
        kfree(node as *mut c_void);
    } else {
        let node: *mut sidtab_node_leaf = entry.ptr_leaf;

        if node.is_null() {
            return;
        }

        i = 0;
        while i < SIDTAB_LEAF_ENTRIES {
            sidtab_destroy_entry(&mut (*node).entries[i as usize]);
            i += 1;
        }
        kfree(node as *mut c_void);
    }
}

pub unsafe extern "C" fn sidtab_destroy(s: *mut sidtab) {
    let mut i: u32;
    let mut level: u32;

    i = 0;
    while i < SECINITSID_NUM {
        if (*s).isids[i as usize].set != 0 {
            sidtab_destroy_entry(&mut (*s).isids[i as usize].entry);
        }
        i += 1;
    }

    level = SIDTAB_MAX_LEVEL;
    while level != 0 && (*s).roots[level as usize].ptr_inner.is_null() {
        level -= 1;
    }

    sidtab_destroy_tree((*s).roots[level as usize], level);
    /*
     * The context_to_sid hashtable's objects are all shared
     * with the isids array and context tree, and so don't need
     * to be cleaned up here.
     */
}

/* #if CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 */

pub unsafe extern "C" fn sidtab_sid2str_put(
    s: *mut sidtab,
    entry: *mut sidtab_entry,
    str_: *const c_char,
    str_len: u32,
) {
    let mut cache: *mut sidtab_str_cache;
    let mut victim: *mut sidtab_str_cache = ptr::null_mut();
    let mut flags: c_ulong = 0;

    /* do not cache invalid contexts */
    if (*entry).context.len != 0 {
        return;
    }

    spin_lock_irqsave(&mut (*s).cache_lock, &mut flags);

    cache = rcu_dereference_protected((*entry).cache, lockdep_is_held(&mut (*s).cache_lock));
    if !cache.is_null() {
        /* entry in cache - just bump to the head of LRU list */
        list_move(&mut (*cache).lru_member, &mut (*s).cache_lru_list);
        spin_unlock_irqrestore(&mut (*s).cache_lock, flags);
        kfree_rcu(victim, rcu_member);
        return;
    }

    cache = kmalloc_flex!(
        sidtab_str_cache,
        str,
        str_len,
        GFP_ATOMIC
    ) as *mut sidtab_str_cache;
    if cache.is_null() {
        spin_unlock_irqrestore(&mut (*s).cache_lock, flags);
        kfree_rcu(victim, rcu_member);
        return;
    }

    if (*s).cache_free_slots == 0 {
        /* pop a cache entry from the tail and free it */
        victim = container_of!(
            (*s).cache_lru_list.prev,
            sidtab_str_cache,
            lru_member
        );
        list_del(&mut (*victim).lru_member);
        rcu_assign_pointer!((*(*victim).parent).cache, ptr::null_mut());
    } else {
        (*s).cache_free_slots -= 1;
    }
    (*cache).parent = entry;
    (*cache).len = str_len;
    memcpy((*cache).str.as_mut_ptr() as *mut c_void, str_ as *const c_void, str_len as usize);
    list_add(&mut (*cache).lru_member, &mut (*s).cache_lru_list);

    rcu_assign_pointer!((*entry).cache, cache);

    spin_unlock_irqrestore(&mut (*s).cache_lock, flags);
    kfree_rcu(victim, rcu_member);
}

pub unsafe extern "C" fn sidtab_sid2str_get(
    s: *mut sidtab,
    entry: *mut sidtab_entry,
    out: *mut *mut c_char,
    out_len: *mut u32,
) -> c_int {
    let mut cache: *mut sidtab_str_cache;
    let mut rc: c_int = 0;

    if (*entry).context.len != 0 {
        return -ENOENT; /* do not cache invalid contexts */
    }

    rcu_read_lock();

    cache = rcu_dereference((*entry).cache);
    if cache.is_null() {
        rc = -ENOENT;
    } else {
        *out_len = (*cache).len;
        if !out.is_null() {
            *out = kmemdup(
                (*cache).str.as_ptr() as *const c_void,
                (*cache).len as usize,
                GFP_ATOMIC,
            ) as *mut c_char;
            if (*out).is_null() {
                rc = -ENOMEM;
            }
        }
    }

    rcu_read_unlock();

    if rc == 0 && !out.is_null() {
        sidtab_sid2str_put(s, entry, *out, *out_len);
    }
    rc
}

/* #endif CONFIG_SECURITY_SELINUX_SID2STR_CACHE_SIZE > 0 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
