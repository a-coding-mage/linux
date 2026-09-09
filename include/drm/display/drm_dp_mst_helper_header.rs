/* Translated from drm_dp_mst_helper.h. C dependencies are supplied externally. */

#[repr(C)]
pub enum drm_dp_mst_topology_ref_type { DRM_DP_MST_TOPOLOGY_REF_GET, DRM_DP_MST_TOPOLOGY_REF_PUT }
#[repr(C)] pub struct drm_dp_mst_topology_ref_history { pub entries: *mut drm_dp_mst_topology_ref_entry, pub len: i32 }
#[repr(C)] pub struct drm_dp_mst_topology_ref_entry { pub r#type: drm_dp_mst_topology_ref_type, pub count: i32, pub ts_nsec: ktime_t, pub backtrace: depot_stack_handle_t }

#[repr(C)] pub enum drm_dp_mst_payload_allocation { DRM_DP_MST_PAYLOAD_ALLOCATION_NONE, DRM_DP_MST_PAYLOAD_ALLOCATION_LOCAL, DRM_DP_MST_PAYLOAD_ALLOCATION_DFP, DRM_DP_MST_PAYLOAD_ALLOCATION_REMOTE }

#[repr(C)] pub struct drm_dp_mst_port {
    pub topology_kref: kref, pub malloc_kref: kref, pub topology_ref_history: drm_dp_mst_topology_ref_history,
    pub port_num: u8, pub input: bool, pub mcs: bool, pub ddps: bool, pub pdt: u8, pub ldps: bool,
    pub dpcd_rev: u8, pub num_sdp_streams: u8, pub num_sdp_stream_sinks: u8, pub full_pbn: u16,
    pub next: list_head, pub mstb: *mut drm_dp_mst_branch, pub aux: drm_dp_aux,
    pub passthrough_aux: *mut drm_dp_aux, pub parent: *mut drm_dp_mst_branch, pub connector: *mut drm_connector,
    pub mgr: *mut drm_dp_mst_topology_mgr, pub cached_edid: *const drm_edid, pub fec_capable: bool,
}
#[repr(C)] pub struct drm_dp_sideband_msg_hdr { pub lct:u8,pub lcr:u8,pub rad:[u8;8],pub broadcast:bool,pub path_msg:bool,pub msg_len:u8,pub somt:bool,pub eomt:bool,pub seqno:bool }
#[repr(C)] pub struct drm_dp_sideband_msg_rx { pub chunk:[u8;48],pub msg:[u8;256],pub curchunk_len:u8,pub curchunk_idx:u8,pub curchunk_hdrlen:u8,pub curlen:u8,pub have_somt:bool,pub have_eomt:bool,pub initial_hdr:drm_dp_sideband_msg_hdr }
#[repr(C)] pub struct drm_dp_mst_branch { pub topology_kref:kref,pub malloc_kref:kref,pub topology_ref_history:drm_dp_mst_topology_ref_history,pub destroy_next:list_head,pub rad:[u8;8],pub lct:u8,pub num_ports:i32,pub ports:list_head,pub port_parent:*mut drm_dp_mst_port,pub mgr:*mut drm_dp_mst_topology_mgr,pub link_address_sent:bool,pub guid:guid_t }

#[repr(C)] pub struct drm_dp_nak_reply { pub guid:guid_t,pub reason:u8,pub nak_data:u8 }
#[repr(C)] pub struct drm_dp_link_address_ack_reply { pub guid:guid_t,pub nports:u8,pub ports:[drm_dp_link_addr_reply_port;16] }
#[repr(C)] pub struct drm_dp_link_addr_reply_port { pub input_port:bool,pub peer_device_type:u8,pub port_number:u8,pub mcs:bool,pub ddps:bool,pub legacy_device_plug_status:bool,pub dpcd_revision:u8,pub peer_guid:guid_t,pub num_sdp_streams:u8,pub num_sdp_stream_sinks:u8 }
#[repr(C)] pub struct drm_dp_remote_dpcd_read_ack_reply { pub port_number:u8,pub num_bytes:u8,pub bytes:[u8;255] }
#[repr(C)] pub struct drm_dp_remote_dpcd_write_ack_reply { pub port_number:u8 }
#[repr(C)] pub struct drm_dp_remote_dpcd_write_nak_reply { pub port_number:u8,pub reason:u8,pub bytes_written_before_failure:u8 }
#[repr(C)] pub struct drm_dp_remote_i2c_read_ack_reply { pub port_number:u8,pub num_bytes:u8,pub bytes:[u8;255] }
#[repr(C)] pub struct drm_dp_remote_i2c_read_nak_reply { pub port_number:u8,pub nak_reason:u8,pub i2c_nak_transaction:u8 }
#[repr(C)] pub struct drm_dp_remote_i2c_write_ack_reply { pub port_number:u8 }
#[repr(C)] pub struct drm_dp_query_stream_enc_status_ack_reply { pub stream_id:u8,pub reply_signed:bool,pub unauthorizable_device_present:bool,pub legacy_device_present:bool,pub query_capable_device_present:bool,pub hdcp_1x_device_present:bool,pub hdcp_2x_device_present:bool,pub auth_completed:bool,pub encryption_enabled:bool,pub repeater_present:bool,pub state:u8 }
pub const DRM_DP_MAX_SDP_STREAMS:u32=16;
#[repr(C)] pub struct drm_dp_allocate_payload { pub port_number:u8,pub number_sdp_streams:u8,pub vcpi:u8,pub pbn:u16,pub sdp_stream_sink:[u8;16] }
#[repr(C)] pub struct drm_dp_allocate_payload_ack_reply { pub port_number:u8,pub vcpi:u8,pub allocated_pbn:u16 }
#[repr(C)] pub struct drm_dp_connection_status_notify { pub guid:guid_t,pub port_number:u8,pub legacy_device_plug_status:bool,pub displayport_device_plug_status:bool,pub message_capability_status:bool,pub input_port:bool,pub peer_device_type:u8 }
#[repr(C)] pub struct drm_dp_remote_dpcd_read { pub port_number:u8,pub dpcd_address:u32,pub num_bytes:u8 }
#[repr(C)] pub struct drm_dp_remote_dpcd_write { pub port_number:u8,pub dpcd_address:u32,pub num_bytes:u8,pub bytes:*mut u8 }
#[repr(C)] pub struct drm_dp_remote_i2c_read_tx { pub i2c_dev_id:u8,pub num_bytes:u8,pub bytes:*mut u8,pub no_stop_bit:u8,pub i2c_transaction_delay:u8 }
#[repr(C)] pub struct drm_dp_remote_i2c_read { pub num_transactions:u8,pub port_number:u8,pub transactions:[drm_dp_remote_i2c_read_tx;4],pub read_i2c_device_id:u8,pub num_bytes_read:u8 }
#[repr(C)] pub struct drm_dp_remote_i2c_write { pub port_number:u8,pub write_i2c_device_id:u8,pub num_bytes:u8,pub bytes:*mut u8 }
#[repr(C)] pub struct drm_dp_query_stream_enc_status { pub stream_id:u8,pub client_id:[u8;7],pub stream_event:u8,pub valid_stream_event:bool,pub stream_behavior:u8,pub valid_stream_behavior:u8 }
#[repr(C)] pub struct drm_dp_port_number_req { pub port_number:u8 }
#[repr(C)] pub struct drm_dp_enum_path_resources_ack_reply { pub port_number:u8,pub fec_capable:bool,pub full_payload_bw_number:u16,pub avail_payload_bw_number:u16 }
#[repr(C)] pub struct drm_dp_port_number_rep { pub port_number:u8 }
#[repr(C)] pub struct drm_dp_query_payload { pub port_number:u8,pub vcpi:u8 }
#[repr(C)] pub struct drm_dp_resource_status_notify { pub port_number:u8,pub guid:guid_t,pub available_pbn:u16 }
#[repr(C)] pub struct drm_dp_query_payload_ack_reply { pub port_number:u8,pub allocated_pbn:u16 }

#[repr(C)] pub union ack_req { pub conn_stat:drm_dp_connection_status_notify,pub port_num:drm_dp_port_number_req,pub resource_stat:drm_dp_resource_status_notify,pub query_payload:drm_dp_query_payload,pub allocate_payload:drm_dp_allocate_payload,pub dpcd_read:drm_dp_remote_dpcd_read,pub dpcd_write:drm_dp_remote_dpcd_write,pub i2c_read:drm_dp_remote_i2c_read,pub i2c_write:drm_dp_remote_i2c_write,pub enc_status:drm_dp_query_stream_enc_status }
#[repr(C)] pub struct drm_dp_sideband_msg_req_body { pub req_type:u8,pub u:ack_req }
#[repr(C)] pub union ack_replies { pub nak:drm_dp_nak_reply,pub link_addr:drm_dp_link_address_ack_reply,pub port_number:drm_dp_port_number_rep,pub path_resources:drm_dp_enum_path_resources_ack_reply,pub allocate_payload:drm_dp_allocate_payload_ack_reply,pub query_payload:drm_dp_query_payload_ack_reply,pub remote_dpcd_read_ack:drm_dp_remote_dpcd_read_ack_reply,pub remote_dpcd_write_ack:drm_dp_remote_dpcd_write_ack_reply,pub remote_dpcd_write_nack:drm_dp_remote_dpcd_write_nak_reply,pub remote_i2c_read_ack:drm_dp_remote_i2c_read_ack_reply,pub remote_i2c_read_nack:drm_dp_remote_i2c_read_nak_reply,pub remote_i2c_write_ack:drm_dp_remote_i2c_write_ack_reply,pub enc_status:drm_dp_query_stream_enc_status_ack_reply }
#[repr(C)] pub struct drm_dp_sideband_msg_reply_body { pub reply_type:u8,pub req_type:u8,pub u:ack_replies }
pub const DRM_DP_SIDEBAND_TX_QUEUED:i32=0; pub const DRM_DP_SIDEBAND_TX_START_SEND:i32=1; pub const DRM_DP_SIDEBAND_TX_SENT:i32=2; pub const DRM_DP_SIDEBAND_TX_RX:i32=3; pub const DRM_DP_SIDEBAND_TX_TIMEOUT:i32=4;
#[repr(C)] pub struct drm_dp_sideband_msg_tx { pub msg:[u8;256],pub chunk:[u8;48],pub cur_offset:u8,pub cur_len:u8,pub dst:*mut drm_dp_mst_branch,pub next:list_head,pub seqno:i32,pub state:i32,pub path_msg:bool,pub reply:drm_dp_sideband_msg_reply_body }
#[repr(C)] pub struct drm_dp_mst_topology_cbs { pub add_connector:Option<unsafe extern "C" fn(*mut drm_dp_mst_topology_mgr,*mut drm_dp_mst_port,*const c_char)->*mut drm_connector>,pub poll_hpd_irq:Option<unsafe extern "C" fn(*mut drm_dp_mst_topology_mgr)> }

#[repr(C)] pub struct drm_dp_mst_atomic_payload { pub port:*mut drm_dp_mst_port,pub vc_start_slot:i8,pub vcpi:u8,pub time_slots:i32,pub pbn:i32,pub delete:bool,pub dsc_enabled:bool,pub payload_allocation_status:drm_dp_mst_payload_allocation,pub next:list_head }
#[repr(C)] pub struct drm_dp_mst_topology_state { pub base:drm_private_state,pub mgr:*mut drm_dp_mst_topology_mgr,pub pending_crtc_mask:u32,pub commit_deps:*mut *mut drm_crtc_commit,pub num_commit_deps:usize,pub payload_mask:u32,pub payloads:list_head,pub total_avail_slots:u8,pub start_slot:u8,pub pbn_div:fixed20_12 }
#[repr(C)] pub struct drm_dp_mst_topology_mgr { pub base:drm_private_obj,pub dev:*mut drm_device,pub cbs:*const drm_dp_mst_topology_cbs,pub max_dpcd_transaction_bytes:i32,pub aux:*mut drm_dp_aux,pub max_payloads:i32,pub conn_base_id:i32,pub up_req_recv:drm_dp_sideband_msg_rx,pub down_rep_recv:drm_dp_sideband_msg_rx,pub lock:mutex,pub probe_lock:mutex,pub mst_state:bool,pub payload_id_table_cleared:bool,pub reset_rx_state:bool,pub payload_count:u8,pub next_start_slot:u8,pub mst_primary:*mut drm_dp_mst_branch,pub dpcd:[u8;DP_RECEIVER_CAP_SIZE],pub sink_count:u8,pub funcs:*const drm_private_state_funcs,pub qlock:mutex,pub tx_msg_downq:list_head,pub tx_waitq:wait_queue_head_t,pub work:work_struct,pub tx_work:work_struct,pub destroy_port_list:list_head,pub destroy_branch_device_list:list_head,pub delayed_destroy_lock:mutex,pub delayed_destroy_wq:*mut workqueue_struct,pub delayed_destroy_work:work_struct,pub up_req_list:list_head,pub up_req_lock:mutex,pub up_req_work:work_struct,pub topology_ref_history_lock:mutex }

extern "C" { pub fn drm_dp_mst_topology_mgr_init(mgr:*mut drm_dp_mst_topology_mgr,dev:*mut drm_device,aux:*mut drm_dp_aux,max_dpcd_transaction_bytes:i32,max_payloads:i32,conn_base_id:i32)->i32; pub fn drm_dp_mst_topology_mgr_destroy(mgr:*mut drm_dp_mst_topology_mgr); pub fn drm_dp_mst_port_is_logical(port:*mut drm_dp_mst_port)->bool; pub fn drm_dp_mst_get_port_malloc(port:*mut drm_dp_mst_port); pub fn drm_dp_mst_put_port_malloc(port:*mut drm_dp_mst_port); }
#[inline] pub unsafe fn drm_dp_mst_port_is_logical_inline(port:*mut drm_dp_mst_port)->bool { (*port).port_num >= DP_MST_LOGICAL_PORT_0 }

#[repr(C)] pub enum drm_dp_mst_mode { DRM_DP_SST, DRM_DP_MST, DRM_DP_SST_SIDEBAND_MSG }
extern "C" {
 pub fn drm_dp_read_mst_cap(aux:*mut drm_dp_aux,dpcd:*const u8)->drm_dp_mst_mode;
 pub fn drm_dp_mst_topology_mgr_set_mst(mgr:*mut drm_dp_mst_topology_mgr,mst_state:bool)->i32;
 pub fn drm_dp_mst_hpd_irq_handle_event(mgr:*mut drm_dp_mst_topology_mgr,esi:*const u8,ack:*mut u8,handled:*mut bool)->i32;
 pub fn drm_dp_mst_hpd_irq_send_new_request(mgr:*mut drm_dp_mst_topology_mgr);
 pub fn drm_dp_mst_detect_port(connector:*mut drm_connector,ctx:*mut drm_modeset_acquire_ctx,mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port)->i32;
 pub fn drm_dp_mst_edid_read(connector:*mut drm_connector,mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port)->*const drm_edid;
 pub fn drm_dp_mst_get_edid(connector:*mut drm_connector,mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port)->*mut edid;
 pub fn drm_dp_get_vc_payload_bw(link_rate:i32,link_lane_count:i32)->fixed20_12;
 pub fn drm_dp_calc_pbn_mode(clock:i32,bpp:i32)->i32;
 pub fn drm_dp_mst_update_slots(state:*mut drm_dp_mst_topology_state,link_encoding_cap:u8);
 pub fn drm_dp_add_payload_part1(mgr:*mut drm_dp_mst_topology_mgr,state:*mut drm_dp_mst_topology_state,payload:*mut drm_dp_mst_atomic_payload)->i32;
 pub fn drm_dp_add_payload_part2(mgr:*mut drm_dp_mst_topology_mgr,payload:*mut drm_dp_mst_atomic_payload)->i32;
 pub fn drm_dp_remove_payload_part1(mgr:*mut drm_dp_mst_topology_mgr,state:*mut drm_dp_mst_topology_state,payload:*mut drm_dp_mst_atomic_payload);
 pub fn drm_dp_remove_payload_part2(mgr:*mut drm_dp_mst_topology_mgr,state:*mut drm_dp_mst_topology_state,old_payload:*const drm_dp_mst_atomic_payload,new_payload:*mut drm_dp_mst_atomic_payload);
 pub fn drm_dp_check_act_status(mgr:*mut drm_dp_mst_topology_mgr)->i32;
 pub fn drm_dp_mst_dump_topology(m:*mut seq_file,mgr:*mut drm_dp_mst_topology_mgr);
 pub fn drm_dp_mst_topology_queue_probe(mgr:*mut drm_dp_mst_topology_mgr);
 pub fn drm_dp_mst_topology_mgr_suspend(mgr:*mut drm_dp_mst_topology_mgr);
 pub fn drm_dp_mst_topology_mgr_resume(mgr:*mut drm_dp_mst_topology_mgr,sync:bool)->i32;
 pub fn drm_dp_mst_dpcd_read(aux:*mut drm_dp_aux,offset:c_uint,buffer:*mut c_void,size:usize)->isize;
 pub fn drm_dp_mst_dpcd_write(aux:*mut drm_dp_aux,offset:c_uint,buffer:*mut c_void,size:usize)->isize;
 pub fn drm_dp_mst_connector_late_register(connector:*mut drm_connector,port:*mut drm_dp_mst_port)->i32;
 pub fn drm_dp_mst_connector_early_unregister(connector:*mut drm_connector,port:*mut drm_dp_mst_port);
 pub fn drm_atomic_get_mst_topology_state(state:*mut drm_atomic_commit,mgr:*mut drm_dp_mst_topology_mgr)->*mut drm_dp_mst_topology_state;
 pub fn drm_atomic_get_old_mst_topology_state(state:*mut drm_atomic_commit,mgr:*mut drm_dp_mst_topology_mgr)->*mut drm_dp_mst_topology_state;
 pub fn drm_atomic_get_new_mst_topology_state(state:*mut drm_atomic_commit,mgr:*mut drm_dp_mst_topology_mgr)->*mut drm_dp_mst_topology_state;
 pub fn drm_atomic_get_mst_payload_state(state:*mut drm_dp_mst_topology_state,port:*mut drm_dp_mst_port)->*mut drm_dp_mst_atomic_payload;
 pub fn drm_dp_mst_port_downstream_of_parent(mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port,parent:*mut drm_dp_mst_port)->bool;
 pub fn drm_dp_atomic_find_time_slots(state:*mut drm_atomic_commit,mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port,pbn:i32)->i32;
 pub fn drm_dp_mst_atomic_enable_dsc(state:*mut drm_atomic_commit,port:*mut drm_dp_mst_port,pbn:i32,enable:bool)->i32;
 pub fn drm_dp_mst_add_affected_dsc_crtcs(state:*mut drm_atomic_commit,mgr:*mut drm_dp_mst_topology_mgr)->i32;
 pub fn drm_dp_atomic_release_time_slots(state:*mut drm_atomic_commit,mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port)->i32;
 pub fn drm_dp_mst_atomic_wait_for_dependencies(state:*mut drm_atomic_commit);
 pub fn drm_dp_mst_atomic_setup_commit(state:*mut drm_atomic_commit)->i32;
 pub fn drm_dp_send_power_updown_phy(mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port,power_up:bool)->i32;
 pub fn drm_dp_send_query_stream_enc_status(mgr:*mut drm_dp_mst_topology_mgr,port:*mut drm_dp_mst_port,status:*mut drm_dp_query_stream_enc_status_ack_reply)->i32;
 pub fn drm_dp_mst_atomic_check_mgr(state:*mut drm_atomic_commit,mgr:*mut drm_dp_mst_topology_mgr,mst_state:*mut drm_dp_mst_topology_state,failing_port:*mut *mut drm_dp_mst_port)->i32;
 pub fn drm_dp_mst_atomic_check(state:*mut drm_atomic_commit)->i32;
 pub fn drm_dp_mst_root_conn_atomic_check(new_conn_state:*mut drm_connector_state,mgr:*mut drm_dp_mst_topology_mgr)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
