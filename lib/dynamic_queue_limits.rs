// SPDX-License-Identifier: GPL-2.0
/*
 * Dynamic byte queue limits.  See include/linux/dynamic_queue_limits.h
 *
 * Copyright (c) 2011, Tom Herbert <therbert@google.com>
 */
// C dependencies supplied by the surrounding kernel translation.

#[inline]
fn posdiff(a: u32, b: u32) -> u32 {
    if (a.wrapping_sub(b) as i32) > 0 { a.wrapping_sub(b) } else { 0 }
}

#[inline]
fn after_eq(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) >= 0
}

unsafe fn dql_check_stall(dql: *mut dql, stall_thrs: u16) {
    let now: unsigned_long;

    if stall_thrs == 0 { return; }

    now = jiffies;
    /* Check for a potential stall */
    if time_after_eq(now, (*dql).last_reap + stall_thrs as unsigned_long) {
        let (mut hist_head, mut t, mut start, mut end):
            (unsigned_long, unsigned_long, unsigned_long, unsigned_long);

        /* We are trying to detect a period of at least @stall_thrs
         * jiffies without any Tx completions, but during first half
         * of which some Tx was posted.
         */
        'dqs_again: loop {
            hist_head = READ_ONCE((*dql).history_head);
            /* pairs with smp_wmb() in dql_queued() */
            smp_rmb();

            /* Get the previous entry in the ring buffer, which is the
             * oldest sample.
             */
            start = (hist_head - DQL_HIST_LEN as unsigned_long + 1)
                * BITS_PER_LONG as unsigned_long;

            /* Advance start to continue from the last reap time */
            if time_before(start, (*dql).last_reap + 1) {
                start = (*dql).last_reap + 1;
            }

            /* Newest sample we should have already seen a completion for */
            end = hist_head * BITS_PER_LONG as unsigned_long
                + (BITS_PER_LONG as unsigned_long - 1);

            /* Shrink the search space to [start, (now - start_thrs/2)] if
             * `end` is beyond the stall zone
             */
            if time_before(now, end + stall_thrs as unsigned_long / 2) {
                end = now - stall_thrs as unsigned_long / 2;
            }

            /* Search for the queued time in [t, end] */
            t = start;
            while time_before_eq(t, end) {
                if test_bit(t % (DQL_HIST_LEN as unsigned_long * BITS_PER_LONG as unsigned_long),
                            (*dql).history) { break; }
                t += 1;
            }

            /* Variable t contains the time of the queue */
            if !time_before_eq(t, end) { break; }

            /* The ring buffer was modified in the meantime, retry */
            if hist_head != READ_ONCE((*dql).history_head) { continue 'dqs_again; }

            (*dql).stall_cnt += 1;
            (*dql).stall_max = max_t(unsigned_short, (*dql).stall_max, (now - t) as unsigned_short);

            trace_dql_stall_detected((*dql).stall_thrs, now - t,
                                     (*dql).last_reap, (*dql).history_head,
                                     now, (*dql).history);
            break;
        }
    }
    (*dql).last_reap = now;
}

/* Records completed count and recalculates the queue limit */
pub unsafe fn dql_completed(dql: *mut dql, count: u32) {
    let (inprogress, prev_inprogress, mut limit): (u32, u32, u32);
    let (mut ovlimit, completed, num_queued): (u32, u32, u32);
    let stall_thrs: u16;
    let all_prev_completed: bool;

    num_queued = READ_ONCE((*dql).num_queued);
    /* Read stall_thrs in advance since it belongs to the same (first)
     * cache line as ->num_queued. This way, dql_check_stall() does not
     * need to touch the first cache line again later, reducing the window
     * of possible false sharing.
     */
    stall_thrs = READ_ONCE((*dql).stall_thrs);

    /* Can't complete more than what's in queue */
    BUG_ON(count > num_queued - (*dql).num_completed);

    completed = (*dql).num_completed + count;
    limit = (*dql).limit;
    ovlimit = posdiff(num_queued - (*dql).num_completed, limit);
    inprogress = num_queued - completed;
    prev_inprogress = (*dql).prev_num_queued - (*dql).num_completed;
    all_prev_completed = after_eq(completed, (*dql).prev_num_queued);

    if ( (ovlimit != 0 && inprogress == 0) ||
         ((*dql).prev_ovlimit != 0 && all_prev_completed) ) {
        limit += posdiff(completed, (*dql).prev_num_queued) + (*dql).prev_ovlimit;
        (*dql).slack_start_time = jiffies;
        (*dql).lowest_slack = UINT_MAX;
    } else if inprogress != 0 && prev_inprogress != 0 && !all_prev_completed {
        let slack: u32;
        let slack_last_objs: u32;
        slack = posdiff(limit + (*dql).prev_ovlimit, 2 * (completed - (*dql).num_completed));
        slack_last_objs = if (*dql).prev_ovlimit != 0 {
            posdiff((*dql).prev_last_obj_cnt, (*dql).prev_ovlimit)
        } else { 0 };
        let slack = max(slack, slack_last_objs);
        if slack < (*dql).lowest_slack { (*dql).lowest_slack = slack; }
        if time_after(jiffies, (*dql).slack_start_time + (*dql).slack_hold_time) {
            limit = posdiff(limit, (*dql).lowest_slack);
            (*dql).slack_start_time = jiffies;
            (*dql).lowest_slack = UINT_MAX;
        }
    }

    limit = clamp(limit, (*dql).min_limit, (*dql).max_limit);
    if limit != (*dql).limit { (*dql).limit = limit; ovlimit = 0; }
    (*dql).adj_limit = limit + completed;
    (*dql).prev_ovlimit = ovlimit;
    (*dql).prev_last_obj_cnt = READ_ONCE((*dql).last_obj_cnt);
    (*dql).num_completed = completed;
    (*dql).prev_num_queued = num_queued;
    dql_check_stall(dql, stall_thrs);
}

pub unsafe fn dql_reset(dql: *mut dql) {
    (*dql).limit = (*dql).min_limit;
    (*dql).num_queued = 0;
    (*dql).num_completed = 0;
    (*dql).last_obj_cnt = 0;
    (*dql).prev_num_queued = 0;
    (*dql).prev_last_obj_cnt = 0;
    (*dql).prev_ovlimit = 0;
    (*dql).lowest_slack = UINT_MAX;
    (*dql).slack_start_time = jiffies;
    (*dql).last_reap = jiffies;
    (*dql).history_head = jiffies / BITS_PER_LONG as unsigned_long;
    memset((*dql).history, 0, size_of_val((*dql).history));
}

pub unsafe fn dql_init(dql: *mut dql, hold_time: u32) {
    (*dql).max_limit = DQL_MAX_LIMIT;
    (*dql).min_limit = 0;
    (*dql).slack_hold_time = hold_time;
    (*dql).stall_thrs = 0;
    dql_reset(dql);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
