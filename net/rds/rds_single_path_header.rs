/* SPDX-License-Identifier: GPL-2.0 */

// C compatibility accessors for the first connection path.
// The original header guard was: _RDS_RDS_SINGLE_H

macro_rules! c_xmit_rm { ($obj:expr) => { $obj.c_path[0].cp_xmit_rm }; }
macro_rules! c_xmit_sg { ($obj:expr) => { $obj.c_path[0].cp_xmit_sg }; }
macro_rules! c_xmit_hdr_off { ($obj:expr) => { $obj.c_path[0].cp_xmit_hdr_off }; }
macro_rules! c_xmit_data_off { ($obj:expr) => { $obj.c_path[0].cp_xmit_data_off }; }
macro_rules! c_xmit_atomic_sent { ($obj:expr) => { $obj.c_path[0].cp_xmit_atomic_sent }; }
macro_rules! c_xmit_rdma_sent { ($obj:expr) => { $obj.c_path[0].cp_xmit_rdma_sent }; }
macro_rules! c_xmit_data_sent { ($obj:expr) => { $obj.c_path[0].cp_xmit_data_sent }; }
macro_rules! c_lock { ($obj:expr) => { $obj.c_path[0].cp_lock }; }
macro_rules! c_next_tx_seq { ($obj:expr) => { $obj.c_path[0].cp_next_tx_seq }; }
macro_rules! c_send_queue { ($obj:expr) => { $obj.c_path[0].cp_send_queue }; }
macro_rules! c_retrans { ($obj:expr) => { $obj.c_path[0].cp_retrans }; }
macro_rules! c_next_rx_seq { ($obj:expr) => { $obj.c_path[0].cp_next_rx_seq }; }
macro_rules! c_transport_data { ($obj:expr) => { $obj.c_path[0].cp_transport_data }; }
macro_rules! c_state { ($obj:expr) => { $obj.c_path[0].cp_state }; }
macro_rules! c_send_gen { ($obj:expr) => { $obj.c_path[0].cp_send_gen }; }
macro_rules! c_flags { ($obj:expr) => { $obj.c_path[0].cp_flags }; }
macro_rules! c_reconnect_jiffies { ($obj:expr) => { $obj.c_path[0].cp_reconnect_jiffies }; }
macro_rules! c_send_w { ($obj:expr) => { $obj.c_path[0].cp_send_w }; }
macro_rules! c_recv_w { ($obj:expr) => { $obj.c_path[0].cp_recv_w }; }
macro_rules! c_conn_w { ($obj:expr) => { $obj.c_path[0].cp_conn_w }; }
macro_rules! c_down_w { ($obj:expr) => { $obj.c_path[0].cp_down_w }; }
macro_rules! c_cm_lock { ($obj:expr) => { $obj.c_path[0].cp_cm_lock }; }
macro_rules! c_waitq { ($obj:expr) => { $obj.c_path[0].cp_waitq }; }
macro_rules! c_unacked_packets { ($obj:expr) => { $obj.c_path[0].cp_unacked_packets }; }
macro_rules! c_unacked_bytes { ($obj:expr) => { $obj.c_path[0].cp_unacked_bytes }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
