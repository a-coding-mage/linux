/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Global definitions for the ANSI FDDI interface.
 *
 * Version:	@(#)if_fddi.h	1.0.2	Sep 29 2004
 *
 * Author:	Lawrence V. Stefani, <stefani@lkg.dec.com>
 *
 *		if_fddi.h is based on previous if_ether.h and if_tr.h work by
 *			Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 *			Donald Becker, <becker@super.org>
 *			Alan Cox, <alan@lxorguk.ukuu.org.uk>
 *			Steve Whitehouse, <gw7rrm@eeshack3.swan.ac.uk>
 *			Peter De Schrijver, <stud11@cc4.kuleuven.ac.be>
 */

// External dependency supplied by linux/netdevice.h.
// External dependency supplied by uapi/linux/if_fddi.h.

/* Define FDDI statistics structure */
#[repr(C)]
pub struct fddi_statistics {
    /* Generic statistics. */
    pub gen: net_device_stats,

    /* Detailed FDDI statistics.  Adopted from RFC 1512 */
    pub smt_station_id: [u8; 8],
    pub smt_op_version_id: u32,
    pub smt_hi_version_id: u32,
    pub smt_lo_version_id: u32,
    pub smt_user_data: [u8; 32],
    pub smt_mib_version_id: u32,
    pub smt_mac_cts: u32,
    pub smt_non_master_cts: u32,
    pub smt_master_cts: u32,
    pub smt_available_paths: u32,
    pub smt_config_capabilities: u32,
    pub smt_config_policy: u32,
    pub smt_connection_policy: u32,
    pub smt_t_notify: u32,
    pub smt_stat_rpt_policy: u32,
    pub smt_trace_max_expiration: u32,
    pub smt_bypass_present: u32,
    pub smt_ecm_state: u32,
    pub smt_cf_state: u32,
    pub smt_remote_disconnect_flag: u32,
    pub smt_station_status: u32,
    pub smt_peer_wrap_flag: u32,
    pub smt_time_stamp: u32,
    pub smt_transition_time_stamp: u32,
    pub mac_frame_status_functions: u32,
    pub mac_t_max_capability: u32,
    pub mac_tvx_capability: u32,
    pub mac_available_paths: u32,
    pub mac_current_path: u32,
    pub mac_upstream_nbr: [u8; FDDI_K_ALEN],
    pub mac_downstream_nbr: [u8; FDDI_K_ALEN],
    pub mac_old_upstream_nbr: [u8; FDDI_K_ALEN],
    pub mac_old_downstream_nbr: [u8; FDDI_K_ALEN],
    pub mac_dup_address_test: u32,
    pub mac_requested_paths: u32,
    pub mac_downstream_port_type: u32,
    pub mac_smt_address: [u8; FDDI_K_ALEN],
    pub mac_t_req: u32,
    pub mac_t_neg: u32,
    pub mac_t_max: u32,
    pub mac_tvx_value: u32,
    pub mac_frame_cts: u32,
    pub mac_copied_cts: u32,
    pub mac_transmit_cts: u32,
    pub mac_error_cts: u32,
    pub mac_lost_cts: u32,
    pub mac_frame_error_threshold: u32,
    pub mac_frame_error_ratio: u32,
    pub mac_rmt_state: u32,
    pub mac_da_flag: u32,
    pub mac_una_da_flag: u32,
    pub mac_frame_error_flag: u32,
    pub mac_ma_unitdata_available: u32,
    pub mac_hardware_present: u32,
    pub mac_ma_unitdata_enable: u32,
    pub path_tvx_lower_bound: u32,
    pub path_t_max_lower_bound: u32,
    pub path_max_t_req: u32,
    pub path_configuration: [u32; 8],
    pub port_my_type: [u32; 2],
    pub port_neighbor_type: [u32; 2],
    pub port_connection_policies: [u32; 2],
    pub port_mac_indicated: [u32; 2],
    pub port_current_path: [u32; 2],
    pub port_requested_paths: [u8; 3 * 2],
    pub port_mac_placement: [u32; 2],
    pub port_available_paths: [u32; 2],
    pub port_pmd_class: [u32; 2],
    pub port_connection_capabilities: [u32; 2],
    pub port_bs_flag: [u32; 2],
    pub port_lct_fail_cts: [u32; 2],
    pub port_ler_estimate: [u32; 2],
    pub port_lem_reject_cts: [u32; 2],
    pub port_lem_cts: [u32; 2],
    pub port_ler_cutoff: [u32; 2],
    pub port_ler_alarm: [u32; 2],
    pub port_connect_state: [u32; 2],
    pub port_pcm_state: [u32; 2],
    pub port_pc_withhold: [u32; 2],
    pub port_ler_flag: [u32; 2],
    pub port_hardware_present: [u32; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
