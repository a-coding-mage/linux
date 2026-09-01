// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct tcpbpf_globals {
    pub event_map: __u32,
    pub total_retrans: __u32,
    pub data_segs_in: __u32,
    pub data_segs_out: __u32,
    pub bad_cb_test_rv: __u32,
    pub good_cb_test_rv: __u32,
    pub bytes_received: __u64,
    pub bytes_acked: __u64,
    pub num_listen: __u32,
    pub num_close_events: __u32,
    pub tcp_save_syn: __u32,
    pub tcp_saved_syn: __u32,
    pub window_clamp_client: __u32,
    pub window_clamp_server: __u32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
