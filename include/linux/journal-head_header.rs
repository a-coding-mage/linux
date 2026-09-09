/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/journal-head.h
 *
 * buffer_head fields for JBD
 *
 * 27 May 2001 Andrew Morton
 *	Created - pulled out of fs.h
 */

/* The C header guard and include are represented by Rust's external symbols. */

pub type tid_t = ::core::ffi::c_uint; /* Unique transaction ID */

/* Compound transaction type. */
#[repr(C)]
pub struct transaction_s {
    _private: [u8; 0],
}
pub type transaction_t = transaction_s;

#[repr(C)]
pub struct buffer_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jbd2_buffer_trigger_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct journal_head {
    /* Points back to our buffer_head. [jbd_lock_bh_journal_head()] */
    pub b_bh: *mut buffer_head,

    /* Protect the buffer head state */
    pub b_state_lock: spinlock_t,

    /* Reference count - see description in journal.c
     * [jbd_lock_bh_journal_head()] */
    pub b_jcount: ::core::ffi::c_int,

    /*
     * Journalling list for this buffer [b_state_lock]
     * NOTE: We *cannot* combine this with b_modified into a bitfield
     * as gcc would then (which the C standard allows but which is
     * very unuseful) make 64-bit accesses to the bitfield and clobber
     * b_jcount if its update races with bitfield modification.
     */
    pub b_jlist: ::core::ffi::c_uint,

    /* This flag signals the buffer has been modified by
     * the currently running transaction
     * [b_state_lock] */
    pub b_modified: ::core::ffi::c_uint,

    /* Copy of the buffer data frozen for writing to the log.
     * [b_state_lock] */
    pub b_frozen_data: *mut ::core::ffi::c_char,

    /* Pointer to a saved copy of the buffer containing no uncommitted
     * deallocation references, so that allocations can avoid overwriting
     * uncommitted deletes. [b_state_lock] */
    pub b_committed_data: *mut ::core::ffi::c_char,

    /* Pointer to the compound transaction which owns this buffer's
     * metadata: either the running transaction or the committing
     * transaction (if there is one).  Only applies to buffers on a
     * transaction's data or metadata journaling list.
     * [j_list_lock] [b_state_lock]
     * Either of these locks is enough for reading, both are needed for
     * changes. */
    pub b_transaction: *mut transaction_t,

    /* Pointer to the running compound transaction which is currently
     * modifying the buffer's metadata, if there was already a transaction
     * committing it when the new transaction touched it.
     * [t_list_lock] [b_state_lock] */
    pub b_next_transaction: *mut transaction_t,

    /* Doubly-linked list of buffers on a transaction's data, metadata or
     * forget queue. [t_list_lock] [b_state_lock] */
    pub b_tnext: *mut journal_head,
    pub b_tprev: *mut journal_head,

    /* Pointer to the compound transaction against which this buffer
     * is checkpointed.  Only dirty buffers can be checkpointed.
     * [j_list_lock] */
    pub b_cp_transaction: *mut transaction_t,

    /* Doubly-linked list of buffers still remaining to be flushed
     * before an old transaction can be checkpointed.
     * [j_list_lock] */
    pub b_cpnext: *mut journal_head,
    pub b_cpprev: *mut journal_head,

    /* Trigger type */
    pub b_triggers: *mut jbd2_buffer_trigger_type,

    /* Trigger type for the committing transaction's frozen data */
    pub b_frozen_triggers: *mut jbd2_buffer_trigger_type,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
