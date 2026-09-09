/* SPDX-License-Identifier: GPL-2.0 */

pub const NFS4_MAXTAGLEN: usize = 20;

pub const NFS4_enc_cb_null_sz: usize = 0;
pub const NFS4_dec_cb_null_sz: usize = 0;
pub const cb_compound_enc_hdr_sz: usize = 4;
pub const cb_compound_dec_hdr_sz: usize = 3 + (NFS4_MAXTAGLEN >> 2);
pub const sessionid_sz: usize = NFS4_MAX_SESSIONID_LEN >> 2;
pub const enc_referring_call4_sz: usize = 1 + 1;
pub const enc_referring_call_list4_sz: usize = sessionid_sz + 1 + enc_referring_call4_sz;
pub const cb_sequence_enc_sz: usize = sessionid_sz + 4 + enc_referring_call_list4_sz;
pub const cb_sequence_dec_sz: usize = op_dec_sz + sessionid_sz + 4;

pub const op_enc_sz: usize = 1;
pub const op_dec_sz: usize = 2;
pub const enc_nfs4_fh_sz: usize = 1 + (NFS4_FHSIZE >> 2);
pub const enc_stateid_sz: usize = NFS4_STATEID_SIZE >> 2;
pub const NFS4_enc_cb_recall_sz: usize = cb_compound_enc_hdr_sz
    + cb_sequence_enc_sz
    + 1
    + enc_stateid_sz
    + enc_nfs4_fh_sz;

pub const NFS4_dec_cb_recall_sz: usize = cb_compound_dec_hdr_sz
    + cb_sequence_dec_sz
    + op_dec_sz;
pub const NFS4_enc_cb_layout_sz: usize = cb_compound_enc_hdr_sz
    + cb_sequence_enc_sz
    + 1
    + 3
    + enc_nfs4_fh_sz
    + 4;
pub const NFS4_dec_cb_layout_sz: usize = cb_compound_dec_hdr_sz
    + cb_sequence_dec_sz
    + op_dec_sz;

pub const NFS4_enc_cb_notify_sz: usize = cb_compound_enc_hdr_sz
    + cb_sequence_enc_sz
    + 1
    + enc_stateid_sz
    + enc_nfs4_fh_sz
    + 1
    + NOTIFY4_EVENT_QUEUE_SIZE * (2 + (NFS4_OPAQUE_LIMIT >> 2));

pub const NFS4_dec_cb_notify_sz: usize = cb_compound_dec_hdr_sz
    + cb_sequence_dec_sz
    + op_dec_sz;

pub const NFS4_enc_cb_notify_lock_sz: usize = cb_compound_enc_hdr_sz
    + cb_sequence_enc_sz
    + 2
    + 1
    + XDR_QUADLEN(NFS4_OPAQUE_LIMIT)
    + enc_nfs4_fh_sz;
pub const NFS4_dec_cb_notify_lock_sz: usize = cb_compound_dec_hdr_sz
    + cb_sequence_dec_sz
    + op_dec_sz;
pub const enc_cb_offload_info_sz: usize = 1
    + 1
    + 2
    + 1
    + XDR_QUADLEN(NFS4_VERIFIER_SIZE);
pub const NFS4_enc_cb_offload_sz: usize = cb_compound_enc_hdr_sz
    + cb_sequence_enc_sz
    + enc_nfs4_fh_sz
    + enc_stateid_sz
    + enc_cb_offload_info_sz;
pub const NFS4_dec_cb_offload_sz: usize = cb_compound_dec_hdr_sz
    + cb_sequence_dec_sz
    + op_dec_sz;
pub const NFS4_enc_cb_recall_any_sz: usize = cb_compound_enc_hdr_sz
    + cb_sequence_enc_sz
    + 1
    + 1
    + 1;
pub const NFS4_dec_cb_recall_any_sz: usize = cb_compound_dec_hdr_sz
    + cb_sequence_dec_sz
    + op_dec_sz;

/*
 * 1: CB_GETATTR opcode (32-bit)
 * N: file_handle
 * 1: number of entry in attribute array (32-bit)
 * 3: entry 0-2 in attribute array (32-bit * 3)
 */
pub const NFS4_enc_cb_getattr_sz: usize = cb_compound_enc_hdr_sz
    + cb_sequence_enc_sz
    + 1
    + enc_nfs4_fh_sz
    + 1
    + 3;
/*
 * 4: fattr_bitmap_maxsz
 * 1: attribute array len
 * 2: change attr (64-bit)
 * 2: size (64-bit)
 * 2: atime.seconds (64-bit)
 * 1: atime.nanoseconds (32-bit)
 * 2: mtime.seconds (64-bit)
 * 1: mtime.nanoseconds (32-bit)
 */
pub const NFS4_dec_cb_getattr_sz: usize = cb_compound_dec_hdr_sz
    + cb_sequence_dec_sz
    + 4
    + 1
    + 2
    + 2
    + 2
    + 1
    + 2
    + 1
    + op_dec_sz;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
