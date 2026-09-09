// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the corresponding kernel/FUSE components are
// intentionally referenced here rather than reimplemented.

/* Frequency (in seconds) of request timeout checks, if opted into */
const FUSE_TIMEOUT_TIMER_FREQ: u32 = 15;

/* Frequency (in jiffies) of request timeout checks, if opted into */
static FUSE_TIMEOUT_TIMER_FREQ_JIFFIES: usize = secs_to_jiffies(FUSE_TIMEOUT_TIMER_FREQ);

/*
 * Default timeout (in seconds) for the server to reply to a request
 * before the connection is aborted, if no timeout was specified on mount.
 *
 * Exported via sysctl
 */
pub static mut fuse_default_req_timeout: u32 = 0;

/*
 * Max timeout (in seconds) for the server to reply to a request before
 * the connection is aborted.
 *
 * Exported via sysctl
 */
pub static mut fuse_max_req_timeout: u32 = 0;

pub unsafe fn fuse_request_expired(
    fch: *mut fuse_chan,
    list: *mut list_head,
) -> bool {
    let req = list_first_entry_or_null(list, fuse_req, list);
    if req.is_null() {
        return false;
    }
    time_is_before_jiffies((*req).create_time + (*fch).timeout.req_timeout)
}

unsafe fn fuse_fpq_processing_expired(
    fch: *mut fuse_chan,
    processing: *mut list_head,
) -> bool {
    let mut i: i32 = 0;
    while i < FUSE_PQ_HASH_SIZE {
        if fuse_request_expired(fch, processing.add(i as usize)) {
            return true;
        }
        i += 1;
    }
    false
}

/*
 * Check if any requests aren't being completed by the time the request timeout
 * elapses. To do so, we:
 * - check the fiq pending list
 * - check the bg queue
 * - check the fpq io and processing lists
 *
 * To make this fast, we only check against the head request on each list since
 * these are generally queued in order of creation time (eg newer requests get
 * queued to the tail). We might miss a few edge cases (eg requests transitioning
 * between lists, re-sent requests at the head of the pending list having a
 * later creation time than other requests on that list, etc.) but that is fine
 * since if the request never gets fulfilled, it will eventually be caught.
 */
unsafe fn fuse_check_timeout(work: *mut work_struct) {
    let dwork = to_delayed_work(work);
    let fch = container_of!(dwork, fuse_chan, timeout.work);
    let fiq = &mut (*fch).iq;
    let mut fud: *mut fuse_dev;
    let mut fpq: *mut fuse_pqueue;
    let mut expired = false;

    if atomic_read(&(*fch).num_waiting) == 0 {
        goto_out(fch);
        return;
    }

    spin_lock(&mut fiq.lock);
    expired = fuse_request_expired(fch, &mut fiq.pending);
    spin_unlock(&mut fiq.lock);
    if expired {
        goto_chan_abort(fch);
        return;
    }

    spin_lock(&mut (*fch).bg_lock);
    expired = fuse_request_expired(fch, &mut (*fch).bg_queue);
    spin_unlock(&mut (*fch).bg_lock);
    if expired {
        goto_chan_abort(fch);
        return;
    }

    spin_lock(&mut (*fch).lock);
    if !(*fch).connected {
        spin_unlock(&mut (*fch).lock);
        return;
    }
    list_for_each_entry!(fud, &mut (*fch).devices, entry, {
        fpq = &mut (*fud).pq;
        spin_lock(&mut (*fpq).lock);
        if fuse_request_expired(fch, &mut (*fpq).io)
            || fuse_fpq_processing_expired(fch, (*fpq).processing.as_mut_ptr())
        {
            spin_unlock(&mut (*fpq).lock);
            spin_unlock(&mut (*fch).lock);
            goto_chan_abort(fch);
            return;
        }
        spin_unlock(&mut (*fpq).lock);
    });
    spin_unlock(&mut (*fch).lock);

    if fuse_uring_request_expired(fch) {
        goto_chan_abort(fch);
        return;
    }

    goto_out(fch);
}

unsafe fn goto_out(fch: *mut fuse_chan) {
    queue_delayed_work(
        system_percpu_wq,
        &mut (*fch).timeout.work,
        FUSE_TIMEOUT_TIMER_FREQ_JIFFIES,
    );
}

unsafe fn goto_chan_abort(fch: *mut fuse_chan) {
    fuse_chan_abort(fch, false);
}

unsafe fn set_request_timeout(fch: *mut fuse_chan, timeout: u32) {
    (*fch).timeout.req_timeout = secs_to_jiffies(timeout);
    INIT_DELAYED_WORK(&mut (*fch).timeout.work, fuse_check_timeout);
    queue_delayed_work(
        system_percpu_wq,
        &mut (*fch).timeout.work,
        FUSE_TIMEOUT_TIMER_FREQ_JIFFIES,
    );
}

pub unsafe fn fuse_init_server_timeout(fch: *mut fuse_chan, mut timeout: u32) {
    if timeout == 0 {
        timeout = fuse_default_req_timeout;
    }

    timeout = min_not_zero(timeout, fuse_max_req_timeout);
    if timeout == 0 {
        return;
    }

    timeout = core::cmp::max(FUSE_TIMEOUT_TIMER_FREQ, timeout);
    set_request_timeout(fch, timeout);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
