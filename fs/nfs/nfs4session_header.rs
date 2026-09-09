/* SPDX-License-Identifier: GPL-2.0 */
/*
 * fs/nfs/nfs4session.h
 *
 * Copyright (c) 2012 Trond Myklebust <Trond.Myklebust@netapp.com>
 */

/* maximum number of slots to use */
pub const NFS4_DEF_SLOT_TABLE_SIZE: u32 = 64u32;
pub const NFS4_DEF_CB_SLOT_TABLE_SIZE: u32 = 16u32;
pub const NFS4_MAX_SLOT_TABLE: u32 = 1024u32;
pub const NFS4_MAX_SLOTID: u32 = NFS4_MAX_SLOT_TABLE - 1u32;
pub const NFS4_NO_SLOT: u32 = u32::MAX;

/* The following declarations are enabled when CONFIG_NFS_V4 is enabled. */

/* Sessions slot seqid */
#[repr(C)]
pub struct nfs4_slot {
    pub table: *mut nfs4_slot_table,
    pub next: *mut nfs4_slot,
    pub generation: ::core::ffi::c_ulong,
    pub slot_nr: u32,
    pub seq_nr: u32,
    pub seq_nr_last_acked: u32,
    pub seq_nr_highest_sent: u32,
    pub privileged: u32,
    pub seq_done: u32,
}

/* Sessions */
#[repr(C)]
pub enum nfs4_slot_tbl_state {
    NFS4_SLOT_TBL_DRAINING,
}

/* Direct equivalent of DIV_ROUND_UP(NFS4_MAX_SLOT_TABLE, BITS_PER_LONG). */
pub const SLOT_TABLE_SZ: usize = ((NFS4_MAX_SLOT_TABLE as usize)
    + (usize::BITS as usize) - 1) / (usize::BITS as usize);

#[repr(C)]
pub struct nfs4_slot_table {
    pub session: *mut nfs4_session,
    pub slots: *mut nfs4_slot,
    pub used_slots: [::core::ffi::c_ulong; SLOT_TABLE_SZ],
    pub slot_tbl_lock: spinlock_t,
    pub slot_tbl_waitq: rpc_wait_queue,
    pub slot_waitq: wait_queue_head_t,
    pub max_slots: u32,
    pub max_slotid: u32,
    pub highest_used_slotid: u32,
    pub target_highest_slotid: u32,
    pub server_highest_slotid: u32,
    pub d_target_highest_slotid: i32,
    pub d2_target_highest_slotid: i32,
    pub generation: ::core::ffi::c_ulong,
    pub complete: completion,
    pub slot_tbl_state: ::core::ffi::c_ulong,
}

/* Session related parameters */
#[repr(C)]
pub struct nfs4_session {
    pub sess_id: nfs4_sessionid,
    pub flags: u32,
    pub session_state: ::core::ffi::c_ulong,
    pub hash_alg: u32,
    pub ssv_len: u32,
    /* The fore and back channel */
    pub fc_attrs: nfs4_channel_attrs,
    pub fc_slot_table: nfs4_slot_table,
    pub bc_attrs: nfs4_channel_attrs,
    pub bc_slot_table: nfs4_slot_table,
    pub clp: *mut nfs_client,
}

#[repr(C)]
pub enum nfs4_session_state {
    NFS4_SESSION_INITING,
    NFS4_SESSION_ESTABLISHED,
}

extern "C" {
    pub fn nfs4_setup_slot_table(tbl: *mut nfs4_slot_table, max_reqs: u32, queue: *const ::core::ffi::c_char) -> i32;
    pub fn nfs4_shutdown_slot_table(tbl: *mut nfs4_slot_table);
    pub fn nfs4_alloc_slot(tbl: *mut nfs4_slot_table) -> *mut nfs4_slot;
    pub fn nfs4_lookup_slot(tbl: *mut nfs4_slot_table, slotid: u32) -> *mut nfs4_slot;
    pub fn nfs4_slot_wait_on_seqid(tbl: *mut nfs4_slot_table, slotid: u32, seq_nr: u32, timeout: ::core::ffi::c_ulong) -> i32;
    pub fn nfs4_try_to_lock_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) -> bool;
    pub fn nfs4_free_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot);
    pub fn nfs4_slot_tbl_drain_complete(tbl: *mut nfs4_slot_table);
    pub fn nfs41_wake_and_assign_slot(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot) -> bool;
    pub fn nfs41_wake_slot_table(tbl: *mut nfs4_slot_table);
    pub fn nfs41_set_target_slotid(tbl: *mut nfs4_slot_table, target_highest_slotid: u32);
    pub fn nfs41_update_target_slotid(tbl: *mut nfs4_slot_table, slot: *mut nfs4_slot, res: *mut nfs4_sequence_res);
    pub fn nfs4_setup_session_slot_tables(ses: *mut nfs4_session) -> i32;
    pub fn nfs4_alloc_session(clp: *mut nfs_client) -> *mut nfs4_session;
    pub fn nfs4_destroy_session(session: *mut nfs4_session);
    pub fn nfs4_init_session(clp: *mut nfs_client) -> i32;
    pub fn nfs4_init_ds_session(clp: *mut nfs_client, lease_time: ::core::ffi::c_ulong, tightly_coupled: bool) -> i32;
}

#[inline]
pub unsafe fn nfs4_slot_tbl_draining(tbl: *mut nfs4_slot_table) -> bool {
    test_bit(NFS4_SLOT_TBL_DRAINING as usize, &(*tbl).slot_tbl_state) != 0
}

#[inline]
pub unsafe fn nfs4_test_locked_slot(tbl: *const nfs4_slot_table, slotid: u32) -> bool {
    test_bit(slotid as usize, (*tbl).used_slots.as_ptr()) != 0
}

#[inline]
pub unsafe fn nfs4_get_session(clp: *const nfs_client) -> *mut nfs4_session {
    (*clp).cl_session
}

/* Determine if sessions are in use. */
#[inline]
pub unsafe fn nfs4_has_session(clp: *const nfs_client) -> i32 {
    if !(*clp).cl_session.is_null() { 1 } else { 0 }
}

#[inline]
pub unsafe fn nfs4_has_persistent_session(clp: *const nfs_client) -> i32 {
    if nfs4_has_session(clp) != 0 { ((*(*clp).cl_session).flags & SESSION4_PERSIST) as i32 } else { 0 }
}

#[inline]
pub unsafe fn nfs4_copy_sessionid(dst: *mut nfs4_sessionid, src: *const nfs4_sessionid) {
    memcpy((*dst).data.as_mut_ptr().cast(), (*src).data.as_ptr().cast(), NFS4_MAX_SESSIONID_LEN);
}

/* nfs_session_id_hash - calculate the crc32 hash for the session id */
#[inline]
pub unsafe fn nfs_session_id_hash(sess_id: *mut nfs4_sessionid) -> u32 {
    !crc32_le(0xFFFFFFFF, (*sess_id).data.as_ptr().cast(), ::core::mem::size_of_val(&(*sess_id).data))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
