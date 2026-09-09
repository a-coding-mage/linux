/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of smb/client/trace.h.
 *
 * The Linux TRACE_EVENT/DECLARE_EVENT_CLASS machinery is supplied by the
 * surrounding kernel tracepoint implementation.  These declarations retain
 * the source names, enum values, field types, and event interfaces; the
 * tracepoint backend is intentionally an external dependency.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type pid_t = i32;
pub type uid_t = u32;

#[repr(u16)]
#[derive(Copy, Clone, Debug)]
pub enum smb_eio_trace {
    smb_eio_trace_compress_copy,
    smb_eio_trace_copychunk_inv_rsp,
    smb_eio_trace_copychunk_overcopy_b,
    smb_eio_trace_copychunk_overcopy_c,
    smb_eio_trace_create_rsp_too_small,
    smb_eio_trace_dfsref_no_rsp,
    smb_eio_trace_ea_overrun,
    smb_eio_trace_extract_will_pin,
    smb_eio_trace_forced_shutdown,
    smb_eio_trace_getacl_bcc_too_small,
    smb_eio_trace_getcifsacl_param_count,
    smb_eio_trace_getdfsrefer_bcc_too_small,
    smb_eio_trace_getextattr_bcc_too_small,
    smb_eio_trace_getextattr_inv_size,
    smb_eio_trace_getsrvinonum_bcc_too_small,
    smb_eio_trace_getsrvinonum_size,
    smb_eio_trace_ioctl_data_len,
    smb_eio_trace_ioctl_no_rsp,
    smb_eio_trace_ioctl_out_off,
    smb_eio_trace_lock_bcc_too_small,
    smb_eio_trace_lock_data_too_small,
    smb_eio_trace_malformed_ksid_key,
    smb_eio_trace_malformed_sid_key,
    smb_eio_trace_mkdir_no_rsp,
    smb_eio_trace_neg_bad_rsplen,
    smb_eio_trace_neg_decode_token,
    smb_eio_trace_neg_info_caps,
    smb_eio_trace_neg_info_dialect,
    smb_eio_trace_neg_info_fail,
    smb_eio_trace_neg_info_sec_mode,
    smb_eio_trace_neg_inval_dialect,
    smb_eio_trace_neg_no_crypt_key,
    smb_eio_trace_neg_sec_blob_too_small,
    smb_eio_trace_neg_unreq_dialect,
    smb_eio_trace_no_auth_key,
    smb_eio_trace_no_lease_key,
    smb_eio_trace_not_netfs_writeback,
    smb_eio_trace_null_pointers,
    smb_eio_trace_oldqfsinfo_bcc_too_small,
    smb_eio_trace_pend_del_fail,
    smb_eio_trace_qalleas_bcc_too_small,
    smb_eio_trace_qalleas_ea_overlong,
    smb_eio_trace_qalleas_overlong,
    smb_eio_trace_qfileinfo_bcc_too_small,
    smb_eio_trace_qfileinfo_invalid,
    smb_eio_trace_qfsattrinfo_bcc_too_small,
    smb_eio_trace_qfsdevinfo_bcc_too_small,
    smb_eio_trace_qfsinfo_bcc_too_small,
    smb_eio_trace_qfsposixinfo_bcc_too_small,
    smb_eio_trace_qfsunixinfo_bcc_too_small,
    smb_eio_trace_qpathinfo_bcc_too_small,
    smb_eio_trace_qpathinfo_invalid,
    smb_eio_trace_qreparse_data_area,
    smb_eio_trace_qreparse_rep_datalen,
    smb_eio_trace_qreparse_ret_datalen,
    smb_eio_trace_qreparse_setup_count,
    smb_eio_trace_qreparse_sizes_wrong,
    smb_eio_trace_qsym_bcc_too_small,
    smb_eio_trace_read_mid_state_unknown,
    smb_eio_trace_read_overlarge,
    smb_eio_trace_read_rsp_malformed,
    smb_eio_trace_read_rsp_short,
    smb_eio_trace_read_too_far,
    smb_eio_trace_reparse_data_len,
    smb_eio_trace_reparse_native_len,
    smb_eio_trace_reparse_native_nul,
    smb_eio_trace_reparse_native_sym_len,
    smb_eio_trace_reparse_nfs_dev,
    smb_eio_trace_reparse_nfs_nul,
    smb_eio_trace_reparse_nfs_sockfifo,
    smb_eio_trace_reparse_nfs_symbuf,
    smb_eio_trace_reparse_nfs_too_short,
    smb_eio_trace_reparse_overlong,
    smb_eio_trace_reparse_rdlen,
    smb_eio_trace_reparse_wsl_nul,
    smb_eio_trace_reparse_wsl_symbuf,
    smb_eio_trace_reparse_wsl_ver,
    smb_eio_trace_rx_b_read_short,
    smb_eio_trace_rx_bad_datalen,
    smb_eio_trace_rx_both_buf,
    smb_eio_trace_rx_calc_len_too_big,
    smb_eio_trace_rx_check_rsp,
    smb_eio_trace_rx_copy_to_iter,
    smb_eio_trace_rx_insuff_res,
    smb_eio_trace_rx_inv_bcc,
    smb_eio_trace_rx_mid_unready,
    smb_eio_trace_rx_neg_sess_resp,
    smb_eio_trace_rx_overlong,
    smb_eio_trace_rx_overpage,
    smb_eio_trace_rx_pos_sess_resp,
    smb_eio_trace_rx_rfc1002_magic,
    smb_eio_trace_rx_sync_mid_invalid,
    smb_eio_trace_rx_sync_mid_malformed,
    smb_eio_trace_rx_too_short,
    smb_eio_trace_rx_trans2_extract,
    smb_eio_trace_rx_unknown_resp,
    smb_eio_trace_rx_unspec_error,
    smb_eio_trace_sess_buf_off,
    smb_eio_trace_sess_exiting,
    smb_eio_trace_sess_krb_wcc,
    smb_eio_trace_sess_nl2_wcc,
    smb_eio_trace_sess_rawnl_auth_wcc,
    smb_eio_trace_sess_rawnl_neg_wcc,
    smb_eio_trace_short_symlink_write,
    smb_eio_trace_sid_too_many_auth,
    smb_eio_trace_sig_data_too_small,
    smb_eio_trace_sig_iter,
    smb_eio_trace_smb1_received_error,
    smb_eio_trace_smb2_received_error,
    smb_eio_trace_sym_slash,
    smb_eio_trace_sym_target_len,
    smb_eio_trace_symlink_file_size,
    smb_eio_trace_tcon_bcc_too_small,
    smb_eio_trace_tdis_in_reconnect,
    smb_eio_trace_tx_chained_async,
    smb_eio_trace_tx_compress_failed,
    smb_eio_trace_tx_copy_iter_to_buf,
    smb_eio_trace_tx_copy_to_buf,
    smb_eio_trace_tx_max_compound,
    smb_eio_trace_tx_miscopy_to_buf,
    smb_eio_trace_tx_need_transform,
    smb_eio_trace_tx_too_long,
    smb_eio_trace_unixqfileinfo_bcc_too_small,
    smb_eio_trace_unixqpathinfo_bcc_too_small,
    smb_eio_trace_user_iter,
    smb_eio_trace_write_bad_buf_type,
    smb_eio_trace_write_mid_state_unknown,
    smb_eio_trace_write_rsp_malformed,
    smb_eio_trace_write_too_far,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum smb3_rw_credits_trace {
    cifs_trace_rw_credits_call_readv_adjust,
    cifs_trace_rw_credits_call_writev_adjust,
    cifs_trace_rw_credits_free_subreq,
    cifs_trace_rw_credits_issue_read_adjust,
    cifs_trace_rw_credits_issue_write_adjust,
    cifs_trace_rw_credits_no_adjust_up,
    cifs_trace_rw_credits_old_session,
    cifs_trace_rw_credits_read_response_add,
    cifs_trace_rw_credits_read_response_clear,
    cifs_trace_rw_credits_read_resubmit,
    cifs_trace_rw_credits_read_submit,
    cifs_trace_rw_credits_write_prepare,
    cifs_trace_rw_credits_write_response_add,
    cifs_trace_rw_credits_write_response_clear,
    cifs_trace_rw_credits_zero_in_flight,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum smb3_tcon_ref_trace {
    netfs_trace_tcon_ref_free,
    netfs_trace_tcon_ref_free_fail,
    netfs_trace_tcon_ref_free_ipc,
    netfs_trace_tcon_ref_free_ipc_fail,
    netfs_trace_tcon_ref_free_reconnect_server,
    netfs_trace_tcon_ref_get_cached_laundromat,
    netfs_trace_tcon_ref_get_cached_lease_break,
    netfs_trace_tcon_ref_get_cancelled_close,
    netfs_trace_tcon_ref_get_close_defer_files,
    netfs_trace_tcon_ref_get_dfs_refer,
    netfs_trace_tcon_ref_get_find,
    netfs_trace_tcon_ref_get_find_sess_tcon,
    netfs_trace_tcon_ref_get_reconnect_server,
    netfs_trace_tcon_ref_get_swn_notify,
    netfs_trace_tcon_ref_new,
    netfs_trace_tcon_ref_new_ipc,
    netfs_trace_tcon_ref_new_reconnect_server,
    netfs_trace_tcon_ref_put_cached_close,
    netfs_trace_tcon_ref_put_cancelled_close,
    netfs_trace_tcon_ref_put_cancelled_close_fid,
    netfs_trace_tcon_ref_put_cancelled_mid,
    netfs_trace_tcon_ref_put_close_defer_files,
    netfs_trace_tcon_ref_put_mnt_ctx,
    netfs_trace_tcon_ref_put_dfs_refer,
    netfs_trace_tcon_ref_put_reconnect_server,
    netfs_trace_tcon_ref_put_swn_notify,
    netfs_trace_tcon_ref_put_tlink,
    netfs_trace_tcon_ref_see_cancelled_close,
    netfs_trace_tcon_ref_see_fscache_collision,
    netfs_trace_tcon_ref_see_fscache_okay,
    netfs_trace_tcon_ref_see_fscache_relinq,
    netfs_trace_tcon_ref_see_umount,
}

/* TRACE_EVENT declarations are external kernel tracepoints. */
#[macro_export]
macro_rules! declare_trace_event { ($name:ident $(, $arg:tt)*) => {
    #[allow(non_upper_case_globals)] pub const $name: &str = stringify!($name);
}; }

/* The header's event classes and generated events, preserving their public names. */
declare_trace_event!(smb3_read_enter);
declare_trace_event!(smb3_read_done);
declare_trace_event!(smb3_read_err);
declare_trace_event!(smb3_write_enter);
declare_trace_event!(smb3_write_done);
declare_trace_event!(smb3_write_err);
declare_trace_event!(smb3_query_dir_enter);
declare_trace_event!(smb3_query_dir_done);
declare_trace_event!(smb3_query_dir_err);
declare_trace_event!(smb3_zero_enter);
declare_trace_event!(smb3_zero_done);
declare_trace_event!(smb3_zero_err);
declare_trace_event!(smb3_falloc_enter);
declare_trace_event!(smb3_falloc_done);
declare_trace_event!(smb3_falloc_err);
declare_trace_event!(smb3_clone_enter);
declare_trace_event!(smb3_clone_done);
declare_trace_event!(smb3_clone_err);
declare_trace_event!(smb3_copychunk_enter);
declare_trace_event!(smb3_copychunk_done);
declare_trace_event!(smb3_copychunk_err);
declare_trace_event!(smb3_set_eof);
declare_trace_event!(smb3_flush_enter);
declare_trace_event!(smb3_flush_done);
declare_trace_event!(smb3_flush_err);
declare_trace_event!(smb3_close_enter);
declare_trace_event!(smb3_close_done);
declare_trace_event!(smb3_close_err);
declare_trace_event!(smb3_lock_enter);
declare_trace_event!(smb3_lock_done);
declare_trace_event!(smb3_lock_err);
declare_trace_event!(smb3_lock_cached);
declare_trace_event!(smb3_lock_conflict);
declare_trace_event!(smb3_cmd_enter);
declare_trace_event!(smb3_cmd_done);
declare_trace_event!(smb3_cmd_err);
declare_trace_event!(smb3_slow_rsp);
declare_trace_event!(smb3_exit_err);
declare_trace_event!(smb3_enter);
declare_trace_event!(smb3_exit_done);
declare_trace_event!(smb3_tcon);
declare_trace_event!(smb3_qfs_done);
declare_trace_event!(smb3_open_enter);
declare_trace_event!(smb3_open_done);
declare_trace_event!(smb3_open_err);
declare_trace_event!(smb3_posix_mkdir_enter);
declare_trace_event!(smb3_posix_mkdir_done);
declare_trace_event!(smb3_posix_mkdir_err);
declare_trace_event!(smb3_open_cached);
declare_trace_event!(smb3_close_cached);
declare_trace_event!(smb3_lease_ack_done);
declare_trace_event!(smb3_lease_break_enter);
declare_trace_event!(smb3_lease_not_found);
declare_trace_event!(smb3_lease_ack_err);
declare_trace_event!(smb3_connect_done);
declare_trace_event!(smb3_smbd_connect_done);
declare_trace_event!(smb3_smbd_connect_err);
declare_trace_event!(smb3_connect_err);
declare_trace_event!(smb3_key_expired);
declare_trace_event!(smb3_reconnect);
declare_trace_event!(smb3_partial_send_reconnect);
declare_trace_event!(smb3_ses_not_found);
declare_trace_event!(smb3_ioctl);
declare_trace_event!(smb3_unsupported_ioctl);
declare_trace_event!(smb3_shutdown_enter);
declare_trace_event!(smb3_shutdown_done);
declare_trace_event!(smb3_shutdown_err);
declare_trace_event!(smb3_reconnect_with_invalid_credits);
declare_trace_event!(smb3_reconnect_detected);
declare_trace_event!(smb3_credit_timeout);
declare_trace_event!(smb3_insufficient_credits);
declare_trace_event!(smb3_too_many_credits);
declare_trace_event!(smb3_add_credits);
declare_trace_event!(smb3_adj_credits);
declare_trace_event!(smb3_hdr_credits);
declare_trace_event!(smb3_nblk_credits);
declare_trace_event!(smb3_pend_credits);
declare_trace_event!(smb3_wait_credits);
declare_trace_event!(smb3_waitff_credits);
declare_trace_event!(smb3_overflow_credits);
declare_trace_event!(smb3_set_credits);
declare_trace_event!(smb3_kerberos_auth);
declare_trace_event!(smb3_tcon_ref);
declare_trace_event!(smb3_rw_credits);
declare_trace_event!(smb3_eio);

declare_trace_event!(cifs_fsync_err);
declare_trace_event!(cifs_flush_err);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
