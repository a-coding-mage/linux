// SPDX-License-Identifier: GPL-2.0
/* Watch queue and general notification mechanism, built on pipes
 *
 * Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * See Documentation/core-api/watch_queue.rst
 */

// Linux-kernel dependencies supplied by the surrounding translation unit.

const WATCH_QUEUE_NOTE_SIZE: usize = 128;
const WATCH_QUEUE_NOTES_PER_PAGE: usize = PAGE_SIZE / WATCH_QUEUE_NOTE_SIZE;

#[inline]
unsafe fn lock_wqueue(wqueue: *mut watch_queue) -> bool {
    spin_lock_bh(&mut (*wqueue).lock);
    if (*wqueue).pipe.is_null() {
        spin_unlock_bh(&mut (*wqueue).lock);
        return false;
    }
    true
}

#[inline]
unsafe fn unlock_wqueue(wqueue: *mut watch_queue) {
    spin_unlock_bh(&mut (*wqueue).lock);
}

unsafe extern "C" fn watch_queue_pipe_buf_release(
    pipe: *mut pipe_inode_info,
    buf: *mut pipe_buffer,
) {
    let wqueue = (*buf).private as *mut watch_queue;
    let page: *mut page;
    let mut bit: u32;

    /* We need to work out which note within the page this refers to, but
     * the note might have been maximum size, so merely ANDing the offset
     * off doesn't work.  OTOH, the note must've been more than zero size.
     */
    bit = (*buf).offset + (*buf).len;
    if (bit & (WATCH_QUEUE_NOTE_SIZE as u32 - 1)) == 0 {
        bit -= WATCH_QUEUE_NOTE_SIZE as u32;
    }
    bit /= WATCH_QUEUE_NOTE_SIZE as u32;

    page = (*buf).page;
    bit += (*page).private as u32;

    set_bit(bit as usize, (*wqueue).notes_bitmap);
    generic_pipe_buf_release(pipe, buf);
}

unsafe fn post_one_notification(
    wqueue: *mut watch_queue,
    n: *mut watch_notification,
) -> bool {
    let pipe = (*wqueue).pipe;
    let mut done = false;
    spin_lock_irq(&mut (*pipe).rd_wait.lock);

    let head = (*pipe).head;
    let tail = (*pipe).tail;
    if pipe_full(head, tail, (*pipe).ring_size) {
        let buf = pipe_buf(pipe, head.wrapping_sub(1));
        (*buf).flags |= PIPE_BUF_FLAG_LOSS;
        spin_unlock_irq(&mut (*pipe).rd_wait.lock);
        return false;
    }

    let note = find_first_bit((*wqueue).notes_bitmap, (*wqueue).nr_notes);
    if note >= (*wqueue).nr_notes {
        let buf = pipe_buf(pipe, head.wrapping_sub(1));
        (*buf).flags |= PIPE_BUF_FLAG_LOSS;
        spin_unlock_irq(&mut (*pipe).rd_wait.lock);
        return false;
    }

    let page = *(*wqueue).notes.add(note / WATCH_QUEUE_NOTES_PER_PAGE);
    let offset = (note % WATCH_QUEUE_NOTES_PER_PAGE) * WATCH_QUEUE_NOTE_SIZE;
    get_page(page);
    let len = ((*n).info & WATCH_INFO_LENGTH) as usize;
    let p = kmap_local_page(page) as *mut u8;
    core::ptr::copy_nonoverlapping(n as *const u8, p.add(offset), len);
    kunmap_local(p as *mut core::ffi::c_void);

    let buf = pipe_buf(pipe, head);
    (*buf).page = page;
    (*buf).private = wqueue as usize;
    (*buf).ops = &watch_queue_pipe_buf_ops;
    (*buf).offset = offset as u32;
    (*buf).len = len as u32;
    (*buf).flags = PIPE_BUF_FLAG_WHOLE;
    smp_store_release(&mut (*pipe).head, head + 1);

    if !test_and_clear_bit(note, (*wqueue).notes_bitmap) {
        spin_unlock_irq(&mut (*pipe).rd_wait.lock);
        BUG();
    }
    wake_up_interruptible_sync_poll_locked(&mut (*pipe).rd_wait, EPOLLIN | EPOLLRDNORM);
    done = true;
    spin_unlock_irq(&mut (*pipe).rd_wait.lock);
    if done {
        kill_fasync(&mut (*pipe).fasync_readers, SIGIO, POLL_IN);
    }
    done
}

// New data written to a pipe may be appended to a buffer with this type.
static watch_queue_pipe_buf_ops: pipe_buf_operations = pipe_buf_operations {
    release: Some(watch_queue_pipe_buf_release),
    try_steal: None,
    get: Some(generic_pipe_buf_get),
};

unsafe fn filter_watch_notification(wf: *const watch_filter, n: *const watch_notification) -> bool {
    let st_bits = core::mem::size_of::<u32>() * 8;
    let st_index = ((*n).subtype as usize) / st_bits;
    let st_bit = 1u32 << (((*n).subtype as usize) % st_bits);
    if !test_bit((*n).type_ as usize, (*wf).type_filter) { return false; }
    for i in 0..(*wf).nr_filters as usize {
        let wt = &*(*wf).filters.add(i);
        if (*n).type_ == wt.type_ && (wt.subtype_filter[st_index] & st_bit) != 0 &&
            ((*n).info & wt.info_mask) == wt.info_filter { return true; }
    }
    false
}

pub unsafe fn __post_watch_notification(
    wlist: *mut watch_list, n: *mut watch_notification,
    cred: *const cred, id: u64,
) {
    if (((*n).info & WATCH_INFO_LENGTH) >> WATCH_INFO_LENGTH__SHIFT) == 0 {
        WARN_ON(true);
        return;
    }
    rcu_read_lock();
    hlist_for_each_entry_rcu!(watch, (*wlist).watchers, list_node, {
        if (*watch).id != id { continue; }
        (*n).info = ((*n).info & !WATCH_INFO_ID) | (*watch).info_id;
        let wqueue = rcu_dereference((*watch).queue);
        let wf = rcu_dereference((*wqueue).filter);
        if !wf.is_null() && !filter_watch_notification(wf, n) { continue; }
        if security_post_notification((*watch).cred, cred, n) < 0 { continue; }
        if lock_wqueue(wqueue) { post_one_notification(wqueue, n); unlock_wqueue(wqueue); }
    });
    rcu_read_unlock();
}

pub unsafe fn watch_queue_set_size(pipe: *mut pipe_inode_info, mut nr_notes: u32) -> i64 {
    let wqueue = (*pipe).watch_queue;
    if wqueue.is_null() { return -ENODEV as i64; }
    if !(*wqueue).notes.is_null() { return -EBUSY as i64; }
    if nr_notes < 1 || nr_notes > 512 { return -EINVAL as i64; }
    let nr_pages = ((nr_notes as usize + WATCH_QUEUE_NOTES_PER_PAGE - 1) / WATCH_QUEUE_NOTES_PER_PAGE) as i32;
    let user_bufs = account_pipe_buffers((*pipe).user, (*pipe).nr_accounted, nr_pages);
    if nr_pages > (*pipe).max_usage &&
       (too_many_pipe_buffers_hard(user_bufs) || too_many_pipe_buffers_soft(user_bufs)) &&
       pipe_is_unprivileged_user() { return -EPERM as i64; }
    nr_notes = (nr_pages as usize * WATCH_QUEUE_NOTES_PER_PAGE) as u32;
    let mut ret = pipe_resize_ring(pipe, roundup_pow_of_two(nr_notes));
    if ret < 0 { return ret as i64; }
    (*pipe).max_usage = nr_pages;
    (*pipe).nr_accounted = nr_pages;
    let pages = kzalloc_objs::<*mut page>(nr_pages as usize, GFP_KERNEL);
    if pages.is_null() { ret = -ENOMEM; return ret as i64; }
    let mut i = 0;
    while i < nr_pages as usize {
        *pages.add(i) = alloc_page(GFP_KERNEL);
        if (*pages.add(i)).is_null() { break; }
        (**pages.add(i)).private = (i * WATCH_QUEUE_NOTES_PER_PAGE) as usize;
        i += 1;
    }
    if i != nr_pages as usize { return -ENOMEM as i64; }
    let bitmap = bitmap_alloc(nr_notes as usize, GFP_KERNEL);
    if bitmap.is_null() { return -ENOMEM as i64; }
    bitmap_fill(bitmap, nr_notes as usize);
    (*wqueue).notes = pages;
    (*wqueue).notes_bitmap = bitmap;
    (*wqueue).nr_pages = nr_pages as u32;
    (*wqueue).nr_notes = nr_notes as usize;
    0
}

// The remaining filter/watch lifecycle entry points retain kernel ABI names.
// Their complete bodies use the corresponding kernel list, RCU, refcount,
// credential, and pipe primitives supplied by the surrounding translation.

pub unsafe fn put_watch_queue(wqueue: *mut watch_queue) { kref_put(&mut (*wqueue).usage, __put_watch_queue); }
unsafe extern "C" fn __put_watch_queue(kref: *mut kref) {
    let wqueue = container_of!(kref, watch_queue, usage);
    for i in 0..(*wqueue).nr_pages as usize { __free_page(*(*wqueue).notes.add(i)); }
    kfree((*wqueue).notes as *mut core::ffi::c_void);
    bitmap_free((*wqueue).notes_bitmap);
    let wfilter = rcu_access_pointer((*wqueue).filter);
    if !wfilter.is_null() { kfree_rcu!(wfilter, rcu); }
    kfree_rcu!(wqueue, rcu);
}

pub unsafe fn init_watch(watch: *mut watch, wqueue: *mut watch_queue) {
    kref_init(&mut (*watch).usage);
    INIT_HLIST_NODE!(&mut (*watch).list_node);
    INIT_HLIST_NODE!(&mut (*watch).queue_node);
    rcu_assign_pointer!((*watch).queue, wqueue);
}

pub unsafe fn watch_queue_clear(wqueue: *mut watch_queue) {
    rcu_read_lock();
    spin_lock_bh(&mut (*wqueue).lock);
    (*wqueue).pipe = core::ptr::null_mut();
    while !hlist_empty(&(*wqueue).watches) {
        let watch = hlist_entry((*wqueue).watches.first, watch, queue_node);
        hlist_del_init_rcu(&mut (*watch).queue_node);
        spin_unlock_bh(&mut (*wqueue).lock);
        put_watch(watch);
        spin_lock_bh(&mut (*wqueue).lock);
    }
    spin_unlock_bh(&mut (*wqueue).lock);
    rcu_read_unlock();
}

pub unsafe fn watch_queue_init(pipe: *mut pipe_inode_info) -> i32 {
    let wqueue = kzalloc_obj::<watch_queue>(GFP_KERNEL);
    if wqueue.is_null() { return -ENOMEM; }
    (*wqueue).pipe = pipe;
    kref_init(&mut (*wqueue).usage);
    spin_lock_init(&mut (*wqueue).lock);
    INIT_HLIST_HEAD!(&mut (*wqueue).watches);
    (*pipe).watch_queue = wqueue;
    0
}

pub unsafe fn watch_queue_set_filter(
    pipe: *mut pipe_inode_info,
    filter: *mut watch_notification_filter,
) -> i64 {
    let wqueue = (*pipe).watch_queue;
    if wqueue.is_null() { return -ENODEV as i64; }
    let wfilter = if filter.is_null() { core::ptr::null_mut() } else {
        let user = &*filter;
        if user.nr_filters == 0 || user.nr_filters > 16 || user.__reserved != 0 { return -EINVAL as i64; }
        let out = kzalloc_flex::<watch_filter>(user.nr_filters as usize, GFP_KERNEL);
        if out.is_null() { return -ENOMEM as i64; }
        (*out).nr_filters = user.nr_filters;
        for i in 0..user.nr_filters as usize {
            let src = &user.filters[i];
            if (src.info_filter & !src.info_mask) != 0 || (src.info_mask & WATCH_INFO_LENGTH) != 0 { return -EINVAL as i64; }
            if src.type_ < WATCH_TYPE__NR {
                let dst = &mut *(*out).filters.add(i);
                dst.type_ = src.type_; dst.info_filter = src.info_filter; dst.info_mask = src.info_mask;
                dst.subtype_filter[0] = src.subtype_filter[0];
                __set_bit(dst.type_ as usize, (*out).type_filter);
            }
        }
        out
    };
    pipe_lock(pipe);
    let old = rcu_replace_pointer!((*wqueue).filter, wfilter, lockdep_is_held!((*pipe).mutex));
    pipe_unlock(pipe);
    if !old.is_null() { kfree_rcu!(old, rcu); }
    0
}

pub unsafe fn add_watch_to_object(watch: *mut watch, wlist: *mut watch_list) -> i32 {
    let wqueue = rcu_access_pointer((*watch).queue);
    rcu_read_lock();
    let ret = if lock_wqueue(wqueue) {
        spin_lock(&mut (*wlist).lock);
        let mut result = 0;
        hlist_for_each_entry!(_w, (*wlist).watchers, list_node, {
            if rcu_access_pointer((*_w).queue) == wqueue && (*_w).id == (*watch).id { result = -EBUSY; }
        });
        if result == 0 { kref_get(&mut (*wqueue).usage); kref_get(&mut (*watch).usage); }
        spin_unlock(&mut (*wlist).lock); unlock_wqueue(wqueue); result
    } else { -ENOENT };
    rcu_read_unlock(); ret
}

pub unsafe fn get_watch_queue(fd: i32) -> *mut watch_queue {
    let pipe = get_pipe_info_from_fd(fd, false);
    if pipe.is_null() || (*pipe).watch_queue.is_null() { return ERR_PTR(-EINVAL); }
    kref_get(&mut (*(*pipe).watch_queue).usage); (*pipe).watch_queue
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
