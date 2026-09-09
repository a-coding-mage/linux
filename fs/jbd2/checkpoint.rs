// SPDX-License-Identifier: GPL-2.0+
/* Direct low-level translation of linux/fs/jbd2/checkpoint.c. */

unsafe fn __buffer_unlink(jh: *mut journal_head) {
    let transaction = (*jh).b_cp_transaction;
    (*(*jh).b_cpnext).b_cpprev = (*jh).b_cpprev;
    (*(*jh).b_cpprev).b_cpnext = (*jh).b_cpnext;
    if (*transaction).t_checkpoint_list == jh {
        (*transaction).t_checkpoint_list = (*jh).b_cpnext;
        if (*transaction).t_checkpoint_list == jh { (*transaction).t_checkpoint_list = core::ptr::null_mut(); }
    }
}

pub unsafe fn __jbd2_log_wait_for_space(journal: *mut journal_t) {
    let nblocks = (*journal).j_max_transaction_buffers;
    while jbd2_log_space_left(journal) < nblocks {
        write_unlock(&mut (*journal).j_state_lock);
        mutex_lock_io(&mut (*journal).j_checkpoint_mutex);
        write_lock(&mut (*journal).j_state_lock);
        if (*journal).j_flags & JBD2_ABORT != 0 { mutex_unlock(&mut (*journal).j_checkpoint_mutex); return; }
        spin_lock(&mut (*journal).j_list_lock);
        let space_left = jbd2_log_space_left(journal);
        if space_left < nblocks {
            let chkpt = !(*journal).j_checkpoint_transactions.is_null();
            let mut tid: tid_t = 0;
            let mut has_transaction = false;
            if !(*journal).j_committing_transaction.is_null() { tid = (*(*journal).j_committing_transaction).t_tid; has_transaction = true; }
            spin_unlock(&mut (*journal).j_list_lock);
            write_unlock(&mut (*journal).j_state_lock);
            if chkpt { jbd2_log_do_checkpoint(journal); }
            else if jbd2_cleanup_journal_tail(journal) <= 0 { }
            else if has_transaction {
                mutex_unlock(&mut (*journal).j_checkpoint_mutex);
                jbd2_log_wait_commit(journal, tid);
                write_lock(&mut (*journal).j_state_lock);
                continue;
            } else {
                printk(KERN_ERR, nblocks, space_left, (*journal).j_devname);
                WARN_ON(1);
                jbd2_journal_abort(journal, -ENOSPC);
            }
            write_lock(&mut (*journal).j_state_lock);
        } else { spin_unlock(&mut (*journal).j_list_lock); }
        mutex_unlock(&mut (*journal).j_checkpoint_mutex);
    }
}

unsafe fn __flush_batch(journal: *mut journal_t, batch_count: *mut i32) {
    let mut plug: blk_plug = core::mem::zeroed();
    blk_start_plug(&mut plug);
    for i in 0..*batch_count { write_dirty_buffer((*journal).j_chkpt_bhs[i as usize], JBD2_JOURNAL_REQ_FLAGS); }
    blk_finish_plug(&mut plug);
    for i in 0..*batch_count { let bh = (*journal).j_chkpt_bhs[i as usize]; __brelse(bh); (*journal).j_chkpt_bhs[i as usize] = core::ptr::null_mut(); }
    *batch_count = 0;
}

pub unsafe fn jbd2_log_do_checkpoint(journal: *mut journal_t) -> i32 {
    let mut batch_count = 0i32;
    let mut result = jbd2_cleanup_journal_tail(journal);
    trace_jbd2_checkpoint(journal, result);
    if result <= 0 { return result; }
    spin_lock(&mut (*journal).j_list_lock);
    if (*journal).j_checkpoint_transactions.is_null() { spin_unlock(&mut (*journal).j_list_lock); return jbd2_cleanup_journal_tail(journal).min(0); }
    let transaction = (*journal).j_checkpoint_transactions;
    if (*transaction).t_chp_stats.cs_chp_time == 0 { (*transaction).t_chp_stats.cs_chp_time = jiffies; }
    let this_tid = (*transaction).t_tid;
    'restart: loop {
        if (*journal).j_checkpoint_transactions != transaction || (*transaction).t_tid != this_tid { break; }
        while !(*transaction).t_checkpoint_list.is_null() {
            let jh = (*transaction).t_checkpoint_list;
            let bh = jh2bh(jh);
            if !(*jh).b_transaction.is_null() {
                let t = (*jh).b_transaction; let tid = (*t).t_tid;
                (*transaction).t_chp_stats.cs_forced_to_close += 1;
                spin_unlock(&mut (*journal).j_list_lock);
                if (*journal).j_flags & JBD2_UNMOUNT != 0 { printk(KERN_ERR, (*journal).j_devname, (*bh).b_blocknr); }
                if batch_count != 0 { __flush_batch(journal, &mut batch_count); }
                jbd2_log_start_commit(journal, tid);
                mutex_unlock(&mut (*journal).j_checkpoint_mutex); jbd2_log_wait_commit(journal, tid);
                mutex_lock_io(&mut (*journal).j_checkpoint_mutex); spin_lock(&mut (*journal).j_list_lock); continue 'restart;
            }
            if !trylock_buffer(bh) { get_bh(bh); spin_unlock(&mut (*journal).j_list_lock); wait_on_buffer(bh); __brelse(bh); continue 'restart; }
            if !buffer_dirty(bh) {
                unlock_buffer(bh);
                if __jbd2_journal_remove_checkpoint(jh) != 0 || (*transaction).t_checkpoint_list.is_null() { break; }
            } else {
                unlock_buffer(bh); get_bh(bh);
                if buffer_jwrite(bh) != 0 { put_bh(bh); spin_unlock(&mut (*journal).j_list_lock); if batch_count != 0 { __flush_batch(journal, &mut batch_count); } jbd2_journal_abort(journal, -EFSCORRUPTED); return -EFSCORRUPTED; }
                (*journal).j_chkpt_bhs[batch_count as usize] = bh; batch_count += 1; (*transaction).t_chp_stats.cs_written += 1; (*transaction).t_checkpoint_list = (*jh).b_cpnext;
            }
            if batch_count == JBD2_NR_BATCH || need_resched() || spin_needbreak(&(*journal).j_list_lock) || (!(*transaction).t_checkpoint_list.is_null() && jh2bh((*transaction).t_checkpoint_list) == (*journal).j_chkpt_bhs[0]) { break; }
        }
        if batch_count != 0 { spin_unlock(&mut (*journal).j_list_lock); __flush_batch(journal, &mut batch_count); cond_resched(); spin_lock(&mut (*journal).j_list_lock); continue; }
        break;
    }
    spin_unlock(&mut (*journal).j_list_lock);
    result = jbd2_cleanup_journal_tail(journal); if result < 0 { result } else { 0 }
}

pub unsafe fn jbd2_cleanup_journal_tail(journal: *mut journal_t) -> i32 {
    if is_journal_aborted(journal) { return -EIO; }
    let mut first_tid = 0; let mut blocknr = 0u64;
    if !jbd2_journal_get_log_tail(journal, &mut first_tid, &mut blocknr) { return 1; }
    if blocknr == 0 { jbd2_journal_abort(journal, -EFSCORRUPTED); return -EFSCORRUPTED; }
    if (*journal).j_flags & JBD2_BARRIER != 0 { blkdev_issue_flush((*journal).j_fs_dev); }
    __jbd2_update_log_tail(journal, first_tid, blocknr)
}

// The remaining checkpoint-list routines preserve the original locking and list operations.
pub unsafe fn jbd2_journal_try_remove_checkpoint(jh: *mut journal_head) -> i32 {
    let bh = jh2bh(jh); if !(*jh).b_transaction.is_null() || !trylock_buffer(bh) { return -EBUSY; }
    if buffer_dirty(bh) { unlock_buffer(bh); return -EBUSY; } unlock_buffer(bh); __jbd2_journal_remove_checkpoint(jh)
}

pub unsafe fn __jbd2_journal_remove_checkpoint(jh: *mut journal_head) -> i32 {
    let transaction = (*jh).b_cp_transaction; if transaction.is_null() { return 0; }
    let journal = (*transaction).t_journal; __buffer_unlink(jh); (*jh).b_cp_transaction = core::ptr::null_mut(); percpu_counter_dec(&mut (*journal).j_checkpoint_jh_count); jbd2_journal_put_journal_head(jh);
    if !(*transaction).t_checkpoint_list.is_null() || (*transaction).t_state != T_FINISHED { return 0; }
    jbd2_time_diff((*transaction).t_chp_stats.cs_chp_time, jiffies); __jbd2_journal_drop_transaction(journal, transaction); jbd2_journal_free_transaction(transaction); 1
}

pub unsafe fn __jbd2_journal_insert_checkpoint(jh: *mut journal_head, transaction: *mut transaction_t) {
    jbd2_journal_grab_journal_head(jh2bh(jh)); (*jh).b_cp_transaction = transaction;
    if (*transaction).t_checkpoint_list.is_null() { (*jh).b_cpnext = jh; (*jh).b_cpprev = jh; } else { (*jh).b_cpnext = (*transaction).t_checkpoint_list; (*jh).b_cpprev = (*(*transaction).t_checkpoint_list).b_cpprev; (*(*jh).b_cpprev).b_cpnext = jh; (*(*jh).b_cpnext).b_cpprev = jh; }
    (*transaction).t_checkpoint_list = jh; percpu_counter_inc(&mut (*transaction).t_journal.j_checkpoint_jh_count);
}

pub unsafe fn __jbd2_journal_drop_transaction(journal: *mut journal_t, transaction: *mut transaction_t) {
    (*journal).j_shrink_transaction = core::ptr::null_mut();
    if !(*transaction).t_cpnext.is_null() { (*(*transaction).t_cpnext).t_cpprev = (*transaction).t_cpprev; (*(*transaction).t_cpprev).t_cpnext = (*transaction).t_cpnext; if (*journal).j_checkpoint_transactions == transaction { (*journal).j_checkpoint_transactions = (*transaction).t_cpnext; } if (*journal).j_checkpoint_transactions == transaction { (*journal).j_checkpoint_transactions = core::ptr::null_mut(); } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
