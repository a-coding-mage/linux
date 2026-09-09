// SPDX-License-Identifier: GPL-2.0-or-later
/* Rolling buffer helpers
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux dependencies and symbols supplied by other translation units.

static mut DEBUG_IDS: atomic_t = atomic_t { }; // Declaration supplied externally.

/**
 * netfs_folioq_alloc - Allocate a folio_queue struct
 * @rreq_id: Associated debugging ID for tracing purposes
 * @gfp: Allocation constraints
 * @trace: Trace tag to indicate the purpose of the allocation
 *
 * Allocate, initialise and account the folio_queue struct and log a trace line
 * to mark the allocation.
 */
pub unsafe fn netfs_folioq_alloc(
    rreq_id: ::core::ffi::c_uint,
    gfp: gfp_t,
    trace: ::core::ffi::c_uint,
) -> *mut folio_queue {
    let mut fq: *mut folio_queue;

    if gfp == GFP_KERNEL {
        fq = netfs_folioq_pool.alloc(gfp, netfs_folioq_pool.pool_data);
    } else {
        fq = mempool_alloc(&mut netfs_folioq_pool, gfp);
    }
    if !fq.is_null() {
        netfs_stat(&netfs_n_folioq);
        folioq_init(fq, rreq_id);
        (*fq).debug_id = atomic_inc_return(&mut DEBUG_IDS);
        trace_netfs_folioq(fq, trace);
    }
    fq
}

/**
 * netfs_folioq_free - Free a folio_queue struct
 * @folioq: The object to free
 * @trace: Trace tag to indicate which free
 *
 * Free and unaccount the folio_queue struct.
 */
pub unsafe fn netfs_folioq_free(folioq: *mut folio_queue, trace: ::core::ffi::c_uint) {
    trace_netfs_folioq(folioq, trace);
    netfs_stat_d(&netfs_n_folioq);
    mempool_free(folioq, &mut netfs_folioq_pool);
}

/*
 * Initialise a rolling buffer.  We allocate an empty folio queue struct to so
 * that the pointers can be independently driven by the producer and the
 * consumer.
 */
pub unsafe fn rolling_buffer_init(
    roll: *mut rolling_buffer,
    rreq_id: ::core::ffi::c_uint,
    direction: ::core::ffi::c_uint,
    gfp: gfp_t,
) -> ::core::ffi::c_int {
    let fq = netfs_folioq_alloc(rreq_id, gfp, netfs_trace_folioq_rollbuf_init);
    if fq.is_null() { return -ENOMEM; }
    (*roll).head = fq;
    (*roll).tail = fq;
    iov_iter_folio_queue(&mut (*roll).iter, direction, fq, 0, 0, 0);
    0
}

/* Add another folio_queue to a rolling buffer if there's no space left. */
pub unsafe fn rolling_buffer_make_space(roll: *mut rolling_buffer, gfp: gfp_t) -> ::core::ffi::c_int {
    let head = (*roll).head;
    if !folioq_full(head) { return 0; }
    let fq = netfs_folioq_alloc((*head).rreq_id, gfp, netfs_trace_folioq_make_space);
    if fq.is_null() { return -ENOMEM; }
    (*fq).prev = head;
    (*roll).head = fq;
    if folioq_full(head) {
        if (*roll).iter.folioq == head && (*roll).iter.folioq_slot == folioq_nr_slots(head) {
            (*roll).iter.folioq = fq;
            (*roll).iter.folioq_slot = 0;
        }
    }
    smp_store_release(&mut (*head).next, fq);
    0
}

/* Decant the list of folios to read into a rolling buffer. */
pub unsafe fn rolling_buffer_load_from_ra(
    roll: *mut rolling_buffer, ractl: *mut readahead_control, put_batch: *mut folio_batch,
) -> ssize_t {
    if rolling_buffer_make_space(roll, GFP_KERNEL) < 0 { return -ENOMEM as ssize_t; }
    let fq = (*roll).head;
    let vec = (*fq).vec.folios as *mut *mut page;
    let nr = __readahead_batch(ractl, vec.add(folio_batch_count(&(*fq).vec)), folio_batch_space(&(*fq).vec));
    let ix = (*fq).vec.nr;
    let to = ix + nr;
    (*fq).vec.nr = to;
    let mut size: ssize_t = 0;
    for i in ix..to {
        let folio = folioq_folio(fq, i);
        let order = folio_order(folio);
        (*fq).orders[i] = order;
        size += (PAGE_SIZE << order) as ssize_t;
        trace_netfs_folio(folio, netfs_folio_trace_read);
        if !folio_batch_add(put_batch, folio) { folio_batch_release(put_batch); }
    }
    WRITE_ONCE((*roll).iter.count, (*roll).iter.count + size);
    smp_store_release(&mut (*roll).next_head_slot, to);
    size
}

/* Append a folio to the rolling buffer. */
pub unsafe fn rolling_buffer_append(roll: *mut rolling_buffer, folio: *mut folio, flags: ::core::ffi::c_uint, gfp: gfp_t) -> ssize_t {
    let size = folio_size(folio) as ssize_t;
    if rolling_buffer_make_space(roll, gfp) < 0 { return -ENOMEM as ssize_t; }
    let slot = folioq_append((*roll).head, folio);
    if flags & ROLLBUF_MARK_1 != 0 { folioq_mark((*roll).head, slot); }
    if flags & ROLLBUF_MARK_2 != 0 { folioq_mark2((*roll).head, slot); }
    WRITE_ONCE((*roll).iter.count, (*roll).iter.count + size);
    smp_store_release(&mut (*roll).next_head_slot, slot);
    size
}

/* Delete a spent buffer from a rolling queue and return the next in line. */
pub unsafe fn rolling_buffer_delete_spent(roll: *mut rolling_buffer) -> *mut folio_queue {
    let spent = (*roll).tail;
    let next = READ_ONCE((*spent).next);
    if next.is_null() { return core::ptr::null_mut(); }
    (*next).prev = core::ptr::null_mut();
    netfs_folioq_free(spent, netfs_trace_folioq_delete);
    (*roll).tail = next;
    next
}

/* Clear out a rolling queue.  Folios that have mark 1 set are put. */
pub unsafe fn rolling_buffer_clear(roll: *mut rolling_buffer) {
    let mut fbatch: folio_batch = ::core::mem::zeroed();
    folio_batch_init(&mut fbatch);
    loop {
        let p = (*roll).tail;
        if p.is_null() { break; }
        (*roll).tail = (*p).next;
        for slot in 0..folioq_count(p) {
            let folio = folioq_folio(p, slot);
            if folio.is_null() { continue; }
            if folioq_is_marked(p, slot) {
                trace_netfs_folio(folio, netfs_folio_trace_put);
                if !folio_batch_add(&mut fbatch, folio) { folio_batch_release(&mut fbatch); }
            }
        }
        netfs_folioq_free(p, netfs_trace_folioq_clear);
    }
    folio_batch_release(&mut fbatch);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
