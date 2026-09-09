// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************
 ******************************************************************************
 **
 **  Copyright (C) 2005-2007 Red Hat, Inc.  All rights reserved.
 **
 ******************************************************************************
 ******************************************************************************/

// Dependencies supplied by the surrounding DLM implementation:
// dlm_internal.h, member.h, lock.h, dir.h, config.h, requestqueue.h, util.h

#[repr(C)]
pub struct rq_entry {
    pub list: list_head,
    pub recover_seq: u32,
    pub nodeid: i32,
    pub request: dlm_message,
}

/*
 * Requests received while the lockspace is in recovery get added to the
 * request queue and processed when recovery is complete.  This happens when
 * the lockspace is suspended on some nodes before it is on others, or the
 * lockspace is enabled on some while still suspended on others.
 */
pub unsafe fn dlm_add_requestqueue(
    ls: *mut dlm_ls,
    nodeid: i32,
    ms: *const dlm_message,
) {
    let length: i32 = (le16_to_cpu((*ms).m_header.h_length) as i32)
        - core::mem::size_of::<dlm_message>() as i32;

    let e = kmalloc(
        core::mem::size_of::<rq_entry>() + length as usize,
        GFP_ATOMIC,
    ) as *mut rq_entry;
    if e.is_null() {
        log_print(c"dlm_add_requestqueue: out of memory len %d", length);
        return;
    }

    (*e).recover_seq = (*ls).ls_recover_seq & 0xFFFF_FFFF;
    (*e).nodeid = nodeid;
    memcpy(
        core::ptr::addr_of_mut!((*e).request) as *mut core::ffi::c_void,
        ms as *const core::ffi::c_void,
        core::mem::size_of::<dlm_message>(),
    );
    memcpy(
        core::ptr::addr_of_mut!((*e).request.m_extra) as *mut core::ffi::c_void,
        (*ms).m_extra as *const core::ffi::c_void,
        length as usize,
    );

    list_add_tail(core::ptr::addr_of_mut!((*e).list), &mut (*ls).ls_requestqueue);
}

/*
 * Called by dlm_recoverd to process normal messages saved while recovery was
 * happening.  Normal locking has been enabled before this is called.  dlm_recv
 * upon receiving a message, will wait for all saved messages to be drained
 * here before processing the message it got.  If a new dlm_ls_stop() arrives
 * while we're processing these saved messages, it may block trying to suspend
 * dlm_recv if dlm_recv is waiting for us in dlm_wait_requestqueue.  In that
 * case, we don't abort since locking_stopped is still 0.  If dlm_recv is not
 * waiting for us, then this processing may be aborted due to locking_stopped.
 */
pub unsafe fn dlm_process_requestqueue(ls: *mut dlm_ls) -> i32 {
    let mut error: i32 = 0;

    write_lock_bh(&mut (*ls).ls_requestqueue_lock);
    loop {
        if list_empty(&(*ls).ls_requestqueue) {
            clear_bit(LSFL_RECV_MSG_BLOCKED, &mut (*ls).ls_flags);
            error = 0;
            break;
        }
        let e = list_first_entry::<rq_entry>(&mut (*ls).ls_requestqueue, 0);
        let ms = core::ptr::addr_of_mut!((*e).request);

        log_limit(
            ls,
            c"dlm_process_requestqueue msg %d from %d lkid %x remid %x result %d seq %u",
            le32_to_cpu((*ms).m_type),
            le32_to_cpu((*ms).m_header.h_nodeid),
            le32_to_cpu((*ms).m_lkid),
            le32_to_cpu((*ms).m_remid),
            from_dlm_errno(le32_to_cpu((*ms).m_result)),
            (*e).recover_seq,
        );

        dlm_receive_message_saved(ls, ms, (*e).recover_seq);
        list_del(&mut (*e).list);
        kfree(e as *mut core::ffi::c_void);

        if dlm_locking_stopped(ls) {
            log_debug(ls, c"process_requestqueue abort running");
            error = -EINTR;
            break;
        }
        write_unlock_bh(&mut (*ls).ls_requestqueue_lock);
        schedule();
        write_lock_bh(&mut (*ls).ls_requestqueue_lock);
    }
    write_unlock_bh(&mut (*ls).ls_requestqueue_lock);

    error
}

unsafe fn purge_request(ls: *mut dlm_ls, ms: *mut dlm_message, nodeid: i32) -> i32 {
    let type_ = (*ms).m_type;

    // the ls is being cleaned up and freed by release_lockspace
    if atomic_read(&(*ls).ls_count) == 0 {
        return 1;
    }

    if dlm_is_removed(ls, nodeid) {
        return 1;
    }

    // Directory operations are always purged because the directory is always
    // rebuilt during recovery and the lookups resent.
    if type_ == cpu_to_le32(DLM_MSG_REMOVE)
        || type_ == cpu_to_le32(DLM_MSG_LOOKUP)
        || type_ == cpu_to_le32(DLM_MSG_LOOKUP_REPLY)
    {
        return 1;
    }

    if !dlm_no_directory(ls) {
        return 0;
    }

    1
}

pub unsafe fn dlm_purge_requestqueue(ls: *mut dlm_ls) {
    write_lock_bh(&mut (*ls).ls_requestqueue_lock);
    let mut e = list_first_entry_or_null::<rq_entry>(&mut (*ls).ls_requestqueue, 0);
    while !e.is_null() {
        let next = list_next_entry_or_null::<rq_entry>(e, 0);
        let ms = core::ptr::addr_of_mut!((*e).request);

        if purge_request(ls, ms, (*e).nodeid) != 0 {
            list_del(&mut (*e).list);
            kfree(e as *mut core::ffi::c_void);
        }
        e = next;
    }
    write_unlock_bh(&mut (*ls).ls_requestqueue_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
