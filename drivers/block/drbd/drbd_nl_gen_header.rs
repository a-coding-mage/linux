/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */

// Translated from drbd_nl_gen.h. Kernel and UAPI types/constants are supplied
// by the corresponding external dependencies.
use core::ffi::c_char;

extern "C" {
    pub static drbd_connection_info_nl_policy: [nla_policy; DRBD_A_CONNECTION_INFO_CONN_ROLE as usize + 1];
    pub static drbd_connection_statistics_nl_policy: [nla_policy; DRBD_A_CONNECTION_STATISTICS_CONN_CONGESTED as usize + 1];
    pub static drbd_detach_parms_nl_policy: [nla_policy; DRBD_A_DETACH_PARMS_FORCE_DETACH as usize + 1];
    pub static drbd_device_info_nl_policy: [nla_policy; DRBD_A_DEVICE_INFO_DEV_DISK_STATE as usize + 1];
    pub static drbd_device_statistics_nl_policy: [nla_policy; DRBD_A_DEVICE_STATISTICS_HISTORY_UUIDS as usize + 1];
    pub static drbd_disconnect_parms_nl_policy: [nla_policy; DRBD_A_DISCONNECT_PARMS_FORCE_DISCONNECT as usize + 1];
    pub static drbd_disk_conf_nl_policy: [nla_policy; DRBD_A_DISK_CONF_DISABLE_WRITE_SAME as usize + 1];
    pub static drbd_drbd_cfg_context_nl_policy: [nla_policy; DRBD_A_DRBD_CFG_CONTEXT_CTX_PEER_ADDR as usize + 1];
    pub static drbd_net_conf_nl_policy: [nla_policy; DRBD_A_NET_CONF_SOCK_CHECK_TIMEO as usize + 1];
    pub static drbd_new_c_uuid_parms_nl_policy: [nla_policy; DRBD_A_NEW_C_UUID_PARMS_CLEAR_BM as usize + 1];
    pub static drbd_peer_device_info_nl_policy: [nla_policy; DRBD_A_PEER_DEVICE_INFO_PEER_RESYNC_SUSP_DEPENDENCY as usize + 1];
    pub static drbd_peer_device_statistics_nl_policy: [nla_policy; DRBD_A_PEER_DEVICE_STATISTICS_PEER_DEV_FLAGS as usize + 1];
    pub static drbd_res_opts_nl_policy: [nla_policy; DRBD_A_RES_OPTS_ON_NO_DATA as usize + 1];
    pub static drbd_resize_parms_nl_policy: [nla_policy; DRBD_A_RESIZE_PARMS_AL_STRIPE_SIZE as usize + 1];
    pub static drbd_resource_info_nl_policy: [nla_policy; DRBD_A_RESOURCE_INFO_RES_SUSP_FEN as usize + 1];
    pub static drbd_resource_statistics_nl_policy: [nla_policy; DRBD_A_RESOURCE_STATISTICS_RES_STAT_WRITE_ORDERING as usize + 1];
    pub static drbd_set_role_parms_nl_policy: [nla_policy; DRBD_A_SET_ROLE_PARMS_ASSUME_UPTODATE as usize + 1];
    pub static drbd_start_ov_parms_nl_policy: [nla_policy; DRBD_A_START_OV_PARMS_OV_STOP_SECTOR as usize + 1];
    pub static drbd_nl_ops: [genl_split_ops; 32];
}

pub const DRBD_NLGRP_EVENTS: u32 = 0;

#[repr(C)] pub struct drbd_cfg_reply { pub info_text: [c_char; 0], pub info_text_len: __u32 }
#[repr(C)] pub struct drbd_cfg_context { pub ctx_volume: __u32, pub ctx_resource_name: [c_char; 128], pub ctx_resource_name_len: __u32, pub ctx_my_addr: [c_char; 128], pub ctx_my_addr_len: __u32, pub ctx_peer_addr: [c_char; 128], pub ctx_peer_addr_len: __u32 }
#[repr(C)] pub struct disk_conf { pub backing_dev: [c_char; 128], pub backing_dev_len: __u32, pub meta_dev: [c_char; 128], pub meta_dev_len: __u32, pub meta_dev_idx: __s32, pub disk_size: __u64, pub max_bio_bvecs: __u32, pub on_io_error: __u32, pub fencing: __u32, pub resync_rate: __u32, pub resync_after: __s32, pub al_extents: __u32, pub c_plan_ahead: __u32, pub c_delay_target: __u32, pub c_fill_target: __u32, pub c_max_rate: __u32, pub c_min_rate: __u32, pub disk_barrier: u8, pub disk_flushes: u8, pub disk_drain: u8, pub md_flushes: u8, pub disk_timeout: __u32, pub read_balancing: __u32, pub al_updates: u8, pub discard_zeroes_if_aligned: u8, pub rs_discard_granularity: __u32, pub disable_write_same: u8 }
#[repr(C)] pub struct res_opts { pub cpu_mask: [c_char; DRBD_CPU_MASK_SIZE as usize], pub cpu_mask_len: __u32, pub on_no_data: __u32 }
#[repr(C)] pub struct net_conf { pub shared_secret: [c_char; SHARED_SECRET_MAX as usize], pub shared_secret_len: __u32, pub cram_hmac_alg: [c_char; SHARED_SECRET_MAX as usize], pub cram_hmac_alg_len: __u32, pub integrity_alg: [c_char; SHARED_SECRET_MAX as usize], pub integrity_alg_len: __u32, pub verify_alg: [c_char; SHARED_SECRET_MAX as usize], pub verify_alg_len: __u32, pub csums_alg: [c_char; SHARED_SECRET_MAX as usize], pub csums_alg_len: __u32, pub wire_protocol: __u32, pub connect_int: __u32, pub timeout: __u32, pub ping_int: __u32, pub ping_timeo: __u32, pub sndbuf_size: __u32, pub rcvbuf_size: __u32, pub ko_count: __u32, pub max_buffers: __u32, pub max_epoch_size: __u32, pub unplug_watermark: __u32, pub after_sb_0p: __u32, pub after_sb_1p: __u32, pub after_sb_2p: __u32, pub rr_conflict: __u32, pub on_congestion: __u32, pub cong_fill: __u32, pub cong_extents: __u32, pub two_primaries: u8, pub discard_my_data: u8, pub tcp_cork: u8, pub always_asbp: u8, pub tentative: u8, pub use_rle: u8, pub csums_after_crash_only: u8, pub sock_check_timeo: __u32 }
#[repr(C)] pub struct set_role_parms { pub assume_uptodate: u8 }
#[repr(C)] pub struct resize_parms { pub resize_size: __u64, pub resize_force: u8, pub no_resync: u8, pub al_stripes: __u32, pub al_stripe_size: __u32 }
#[repr(C)] pub struct state_info { pub sib_reason: __u32, pub current_state: __u32, pub capacity: __u64, pub ed_uuid: __u64, pub prev_state: __u32, pub new_state: __u32, pub uuids: [c_char; DRBD_NL_UUIDS_SIZE as usize], pub uuids_len: __u32, pub disk_flags: __u32, pub bits_total: __u64, pub bits_oos: __u64, pub bits_rs_total: __u64, pub bits_rs_failed: __u64, pub helper: [c_char; 32], pub helper_len: __u32, pub helper_exit_code: __u32, pub send_cnt: __u64, pub recv_cnt: __u64, pub read_cnt: __u64, pub writ_cnt: __u64, pub al_writ_cnt: __u64, pub bm_writ_cnt: __u64, pub ap_bio_cnt: __u32, pub ap_pending_cnt: __u32, pub rs_pending_cnt: __u32 }
#[repr(C)] pub struct start_ov_parms { pub ov_start_sector: __u64, pub ov_stop_sector: __u64 }
#[repr(C)] pub struct new_c_uuid_parms { pub clear_bm: u8 }
#[repr(C)] pub struct timeout_parms { pub timeout_type: __u32 }
#[repr(C)] pub struct disconnect_parms { pub force_disconnect: u8 }
#[repr(C)] pub struct detach_parms { pub force_detach: u8 }
#[repr(C)] pub struct resource_info { pub res_role: __u32, pub res_susp: u8, pub res_susp_nod: u8, pub res_susp_fen: u8 }
#[repr(C)] pub struct device_info { pub dev_disk_state: __u32 }
#[repr(C)] pub struct connection_info { pub conn_connection_state: __u32, pub conn_role: __u32 }
#[repr(C)] pub struct peer_device_info { pub peer_repl_state: __u32, pub peer_disk_state: __u32, pub peer_resync_susp_user: __u32, pub peer_resync_susp_peer: __u32, pub peer_resync_susp_dependency: __u32 }
#[repr(C)] pub struct resource_statistics { pub res_stat_write_ordering: __u32 }
#[repr(C)] pub struct device_statistics { pub dev_size: __u64, pub dev_read: __u64, pub dev_write: __u64, pub dev_al_writes: __u64, pub dev_bm_writes: __u64, pub dev_upper_pending: __u32, pub dev_lower_pending: __u32, pub dev_upper_blocked: u8, pub dev_lower_blocked: u8, pub dev_al_suspended: u8, pub dev_exposed_data_uuid: __u64, pub dev_current_uuid: __u64, pub dev_disk_flags: __u32, pub history_uuids: [c_char; DRBD_NL_HISTORY_UUIDS_SIZE as usize], pub history_uuids_len: __u32 }
#[repr(C)] pub struct connection_statistics { pub conn_congested: u8 }
#[repr(C)] pub struct peer_device_statistics { pub peer_dev_received: __u64, pub peer_dev_sent: __u64, pub peer_dev_pending: __u32, pub peer_dev_unacked: __u32, pub peer_dev_out_of_sync: __u64, pub peer_dev_resync_failed: __u64, pub peer_dev_bitmap_uuid: __u64, pub peer_dev_flags: __u32 }
#[repr(C)] pub struct drbd_notification_header { pub nh_type: __u32 }
#[repr(C)] pub struct drbd_helper_info { pub helper_name: [c_char; 32], pub helper_name_len: __u32, pub helper_status: __u32 }

extern "C" {
    pub fn drbd_pre_doit(ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_post_doit(ops: *const genl_split_ops, skb: *mut sk_buff, info: *mut genl_info);
    pub fn drbd_adm_dump_devices_done(cb: *mut netlink_callback) -> c_int;
    pub fn drbd_adm_dump_connections_done(cb: *mut netlink_callback) -> c_int;
    pub fn drbd_adm_dump_peer_devices_done(cb: *mut netlink_callback) -> c_int;
    pub fn drbd_nl_get_status_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_get_status_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn drbd_nl_new_minor_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_del_minor_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_new_resource_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_del_resource_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_resource_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_connect_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_disconnect_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_attach_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_resize_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_primary_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_secondary_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_new_c_uuid_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_start_ov_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_detach_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_invalidate_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_inval_peer_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_pause_sync_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_resume_sync_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_suspend_io_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_resume_io_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_outdate_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_get_timeout_type_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_down_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_chg_disk_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_chg_net_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn drbd_nl_get_resources_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn drbd_nl_get_devices_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn drbd_nl_get_connections_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn drbd_nl_get_peer_devices_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn drbd_nl_get_initial_state_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn drbd_cfg_reply_to_skb(skb: *mut sk_buff, s: *mut drbd_cfg_reply) -> c_int;
    pub fn drbd_cfg_context_from_attrs(s: *mut drbd_cfg_context, info: *mut genl_info) -> c_int;
    pub fn drbd_cfg_context_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn drbd_cfg_context_to_skb(skb: *mut sk_buff, s: *mut drbd_cfg_context) -> c_int;
    pub fn disk_conf_from_attrs(s: *mut disk_conf, info: *mut genl_info) -> c_int;
    pub fn disk_conf_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn disk_conf_to_skb(skb: *mut sk_buff, s: *mut disk_conf) -> c_int;
    pub fn set_disk_conf_defaults(x: *mut disk_conf);
    pub fn res_opts_from_attrs(s: *mut res_opts, info: *mut genl_info) -> c_int;
    pub fn res_opts_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn res_opts_to_skb(skb: *mut sk_buff, s: *mut res_opts) -> c_int;
    pub fn set_res_opts_defaults(x: *mut res_opts);
    pub fn net_conf_from_attrs(s: *mut net_conf, info: *mut genl_info) -> c_int;
    pub fn net_conf_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn net_conf_to_skb(skb: *mut sk_buff, s: *mut net_conf) -> c_int;
    pub fn set_net_conf_defaults(x: *mut net_conf);
    pub fn set_role_parms_from_attrs(s: *mut set_role_parms, info: *mut genl_info) -> c_int;
    pub fn set_role_parms_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn set_role_parms_to_skb(skb: *mut sk_buff, s: *mut set_role_parms) -> c_int;
    pub fn resize_parms_from_attrs(s: *mut resize_parms, info: *mut genl_info) -> c_int;
    pub fn resize_parms_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn resize_parms_to_skb(skb: *mut sk_buff, s: *mut resize_parms) -> c_int;
    pub fn set_resize_parms_defaults(x: *mut resize_parms);
    pub fn state_info_to_skb(skb: *mut sk_buff, s: *mut state_info) -> c_int;
    pub fn start_ov_parms_from_attrs(s: *mut start_ov_parms, info: *mut genl_info) -> c_int;
    pub fn start_ov_parms_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn start_ov_parms_to_skb(skb: *mut sk_buff, s: *mut start_ov_parms) -> c_int;
    pub fn new_c_uuid_parms_from_attrs(s: *mut new_c_uuid_parms, info: *mut genl_info) -> c_int;
    pub fn new_c_uuid_parms_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn new_c_uuid_parms_to_skb(skb: *mut sk_buff, s: *mut new_c_uuid_parms) -> c_int;
    pub fn timeout_parms_to_skb(skb: *mut sk_buff, s: *mut timeout_parms) -> c_int;
    pub fn disconnect_parms_from_attrs(s: *mut disconnect_parms, info: *mut genl_info) -> c_int;
    pub fn disconnect_parms_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn disconnect_parms_to_skb(skb: *mut sk_buff, s: *mut disconnect_parms) -> c_int;
    pub fn detach_parms_from_attrs(s: *mut detach_parms, info: *mut genl_info) -> c_int;
    pub fn detach_parms_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn detach_parms_to_skb(skb: *mut sk_buff, s: *mut detach_parms) -> c_int;
    pub fn resource_info_from_attrs(s: *mut resource_info, info: *mut genl_info) -> c_int;
    pub fn resource_info_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn resource_info_to_skb(skb: *mut sk_buff, s: *mut resource_info) -> c_int;
    pub fn device_info_from_attrs(s: *mut device_info, info: *mut genl_info) -> c_int;
    pub fn device_info_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn device_info_to_skb(skb: *mut sk_buff, s: *mut device_info) -> c_int;
    pub fn connection_info_from_attrs(s: *mut connection_info, info: *mut genl_info) -> c_int;
    pub fn connection_info_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn connection_info_to_skb(skb: *mut sk_buff, s: *mut connection_info) -> c_int;
    pub fn peer_device_info_from_attrs(s: *mut peer_device_info, info: *mut genl_info) -> c_int;
    pub fn peer_device_info_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn peer_device_info_to_skb(skb: *mut sk_buff, s: *mut peer_device_info) -> c_int;
    pub fn resource_statistics_from_attrs(s: *mut resource_statistics, info: *mut genl_info) -> c_int;
    pub fn resource_statistics_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn resource_statistics_to_skb(skb: *mut sk_buff, s: *mut resource_statistics) -> c_int;
    pub fn device_statistics_from_attrs(s: *mut device_statistics, info: *mut genl_info) -> c_int;
    pub fn device_statistics_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn device_statistics_to_skb(skb: *mut sk_buff, s: *mut device_statistics) -> c_int;
    pub fn connection_statistics_from_attrs(s: *mut connection_statistics, info: *mut genl_info) -> c_int;
    pub fn connection_statistics_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn connection_statistics_to_skb(skb: *mut sk_buff, s: *mut connection_statistics) -> c_int;
    pub fn peer_device_statistics_from_attrs(s: *mut peer_device_statistics, info: *mut genl_info) -> c_int;
    pub fn peer_device_statistics_ntb_from_attrs(ret: *mut *mut *mut nlattr, info: *mut genl_info) -> c_int;
    pub fn peer_device_statistics_to_skb(skb: *mut sk_buff, s: *mut peer_device_statistics) -> c_int;
    pub fn drbd_notification_header_to_skb(skb: *mut sk_buff, s: *mut drbd_notification_header) -> c_int;
    pub fn drbd_helper_info_to_skb(skb: *mut sk_buff, s: *mut drbd_helper_info) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
