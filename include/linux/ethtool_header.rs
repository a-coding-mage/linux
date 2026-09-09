/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/ethtool.h. Included Linux dependencies are external. */

pub const ETHTOOL_MM_MAX_VERIFY_TIME_MS: u32 = 128;
pub const ETHTOOL_MM_MAX_VERIFY_RETRIES: u32 = 3;

#[repr(C)] pub struct compat_ethtool_rx_flow_spec { pub flow_type:u32, pub h_u: ethtool_flow_union, pub h_ext:ethtool_flow_ext, pub m_u:ethtool_flow_union, pub m_ext:ethtool_flow_ext, pub ring_cookie:u64, pub location:u32 }
#[repr(C)] pub struct compat_ethtool_rxnfc { pub cmd:u32, pub flow_type:u32, pub data:u64, pub fs:compat_ethtool_rx_flow_spec, pub rule_cnt:u32, pub rule_locs:[u32;0] }

#[repr(C)] pub enum ethtool_phys_id_state { ETHTOOL_ID_INACTIVE, ETHTOOL_ID_ACTIVE, ETHTOOL_ID_ON, ETHTOOL_ID_OFF }
pub const ETH_RSS_HASH_TOP_BIT:u32=0; pub const ETH_RSS_HASH_XOR_BIT:u32=1; pub const ETH_RSS_HASH_CRC32_BIT:u32=2; pub const ETH_RSS_HASH_FUNCS_COUNT:u32=3;
#[repr(C)] pub struct kernel_ethtool_ringparam { pub rx_buf_len:u32,pub tcp_data_split:u8,pub tx_push:u8,pub rx_push:u8,pub cqe_size:u32,pub tx_push_buf_len:u32,pub tx_push_buf_max_len:u32,pub hds_thresh:u32,pub hds_thresh_max:u32 }
pub const ETHTOOL_RING_USE_RX_BUF_LEN:u32=1<<0; pub const ETHTOOL_RING_USE_CQE_SIZE:u32=1<<1; pub const ETHTOOL_RING_USE_TX_PUSH:u32=1<<2; pub const ETHTOOL_RING_USE_RX_PUSH:u32=1<<3; pub const ETHTOOL_RING_USE_TX_PUSH_BUF_LEN:u32=1<<4; pub const ETHTOOL_RING_USE_TCP_DATA_SPLIT:u32=1<<5; pub const ETHTOOL_RING_USE_HDS_THRS:u32=1<<6;
pub const ETH_RSS_HASH_TOP:u32=1<<ETH_RSS_HASH_TOP_BIT; pub const ETH_RSS_HASH_XOR:u32=1<<ETH_RSS_HASH_XOR_BIT; pub const ETH_RSS_HASH_CRC32:u32=1<<ETH_RSS_HASH_CRC32_BIT; pub const ETH_RSS_HASH_UNKNOWN:u32=0; pub const ETH_RSS_HASH_NO_CHANGE:u32=0;

#[repr(C)] pub struct ethtool_link_ext_state_info { pub link_ext_state:ethtool_link_ext_state, pub substate:ethtool_link_ext_state_info_substate }
#[repr(C)] pub union ethtool_link_ext_state_info_substate { pub autoneg:ethtool_link_ext_substate_autoneg,pub link_training:ethtool_link_ext_substate_link_training,pub link_logical_mismatch:ethtool_link_ext_substate_link_logical_mismatch,pub bad_signal_integrity:ethtool_link_ext_substate_bad_signal_integrity,pub cable_issue:ethtool_link_ext_substate_cable_issue,pub module:ethtool_link_ext_substate_module,pub link_ext_substate:u32 }
#[repr(C)] pub struct ethtool_link_ext_stats { pub link_down_events:u64 }
pub unsafe fn ethtool_rxfh_indir_default(index:u32,n_rx_rings:u32)->u32 { index % n_rx_rings }
#[repr(C)] pub struct ethtool_rxfh_context { pub indir_size:u32,pub key_size:u32,pub indir_user_size:u32,pub priv_size:u16,pub hfunc:u8,pub input_xfrm:u8,pub indir_configured:u8,pub key_configured:u8,pub key_off:u32,pub data:[u8;0] }
pub unsafe fn ethtool_rxfh_context_priv(ctx:*mut ethtool_rxfh_context)->*mut u8 { (*ctx).data.as_mut_ptr() }
pub unsafe fn ethtool_rxfh_context_indir(ctx:*mut ethtool_rxfh_context)->*mut u32 { (*ctx).data.as_mut_ptr().add(((*ctx).priv_size as usize + 3)&!3) as *mut u32 }
pub unsafe fn ethtool_rxfh_context_key(ctx:*mut ethtool_rxfh_context)->*mut u8 { (*ctx).data.as_mut_ptr().add((*ctx).key_off as usize) }

extern "C" { pub fn ethtool_rxfh_context_lost(dev:*mut net_device,context_id:u32); pub fn ethtool_rxfh_indir_lost(dev:*mut net_device); pub fn ethtool_rxfh_indir_can_resize(dev:*mut net_device,tbl:*const u32,old_size:u32,new_size:u32)->bool; pub fn ethtool_rxfh_indir_resize(dev:*mut net_device,tbl:*mut u32,old_size:u32,new_size:u32); pub fn ethtool_rxfh_ctxs_can_resize(dev:*mut net_device,new_indir_size:u32)->i32; pub fn ethtool_rxfh_ctxs_resize(dev:*mut net_device,new_indir_size:u32); }
#[repr(C)] pub struct link_mode_info { pub speed:i32,pub lanes:u8,pub min_pairs:u8,pub pairs:u8,pub duplex:u8,pub mediums:u16 }
extern "C" { pub static link_mode_params:*const link_mode_info; }
#[repr(C)] pub enum ethtool_link_medium { ETHTOOL_LINK_MEDIUM_BASET,ETHTOOL_LINK_MEDIUM_BASEK,ETHTOOL_LINK_MEDIUM_BASES,ETHTOOL_LINK_MEDIUM_BASEC,ETHTOOL_LINK_MEDIUM_BASEL,ETHTOOL_LINK_MEDIUM_BASED,ETHTOOL_LINK_MEDIUM_BASEE,ETHTOOL_LINK_MEDIUM_BASEF,ETHTOOL_LINK_MEDIUM_BASEV,ETHTOOL_LINK_MEDIUM_BASEMLD,ETHTOOL_LINK_MEDIUM_NONE }
pub const __ETHTOOL_LINK_MEDIUM_LAST:u32=11;
pub const ETHTOOL_MEDIUM_FIBER_BITS:u32=(1<<2)|(1<<4)|(1<<7);
extern "C" { pub fn ethtool_str_to_medium(s:*const i8)->ethtool_link_medium; }
pub unsafe fn ethtool_linkmode_n_pairs(mode:usize)->u8 { (*link_mode_params.add(mode)).pairs }

/* Dependent bitmap size and Linux declarations are supplied by the translated uapi headers. */
#[repr(C)] pub struct ethtool_link_ksettings { pub base:ethtool_link_settings, pub link_modes:ethtool_link_ksettings_link_modes, pub lanes:u32 }
#[repr(C)] pub struct ethtool_link_ksettings_link_modes { pub supported:[usize;1],pub advertising:[usize;1],pub lp_advertising:[usize;1] }
#[repr(C)] pub struct ethtool_keee { pub supported:[usize;1],pub advertised:[usize;1],pub lp_advertised:[usize;1],pub tx_lpi_timer:u32,pub tx_lpi_enabled:bool,pub eee_active:bool,pub eee_enabled:bool }
#[repr(C)] pub struct kernel_ethtool_coalesce { pub use_cqe_mode_tx:u8,pub use_cqe_mode_rx:u8,pub tx_aggr_max_bytes:u32,pub tx_aggr_max_frames:u32,pub tx_aggr_time_usecs:u32,pub rx_cqe_frames:u32,pub rx_cqe_nsecs:u32 }

pub const ETHTOOL_STAT_NOT_SET:u64=!0;
pub unsafe fn ethtool_stats_init(stats:*mut u64,mut n:usize){ while n!=0 { n-=1; *stats.add(n)=ETHTOOL_STAT_NOT_SET; } }
#[repr(C)] pub struct ethtool_phy_stats { pub rx_packets:u64,pub rx_bytes:u64,pub rx_errors:u64,pub tx_packets:u64,pub tx_bytes:u64,pub tx_errors:u64 }
#[repr(C)] pub struct ethtool_eth_mac_stats { pub src:ethtool_mac_stats_src,pub FramesTransmittedOK:u64,pub SingleCollisionFrames:u64,pub MultipleCollisionFrames:u64,pub FramesReceivedOK:u64,pub FrameCheckSequenceErrors:u64,pub AlignmentErrors:u64,pub OctetsTransmittedOK:u64,pub FramesWithDeferredXmissions:u64,pub LateCollisions:u64,pub FramesAbortedDueToXSColls:u64,pub FramesLostDueToIntMACXmitError:u64,pub CarrierSenseErrors:u64,pub OctetsReceivedOK:u64,pub FramesLostDueToIntMACRcvError:u64,pub MulticastFramesXmittedOK:u64,pub BroadcastFramesXmittedOK:u64,pub FramesWithExcessiveDeferral:u64,pub MulticastFramesReceivedOK:u64,pub BroadcastFramesReceivedOK:u64,pub InRangeLengthErrors:u64,pub OutOfRangeLengthField:u64,pub FrameTooLongErrors:u64 }
#[repr(C)] pub struct ethtool_eth_phy_stats { pub src:ethtool_mac_stats_src,pub SymbolErrorDuringCarrier:u64 }
#[repr(C)] pub struct ethtool_eth_ctrl_stats { pub src:ethtool_mac_stats_src,pub MACControlFramesTransmitted:u64,pub MACControlFramesReceived:u64,pub UnsupportedOpcodesReceived:u64 }
#[repr(C)] pub struct ethtool_pause_stats { pub src:ethtool_mac_stats_src,pub tx_pause_frames:u64,pub rx_pause_frames:u64,pub tx_pause_storm_events:u64 }
pub const ETHTOOL_MAX_LANES:usize=8; pub const ETHTOOL_FEC_HIST_MAX:usize=17;
#[repr(C)] pub struct ethtool_fec_hist_range{pub low:u16,pub high:u16}
#[repr(C)] pub struct ethtool_fec_hist_value{pub sum:u64,pub per_lane:[u64;8]}
#[repr(C)] pub struct ethtool_fec_hist{pub values:[ethtool_fec_hist_value;17],pub ranges:*const ethtool_fec_hist_range,pub ranges_buf:[ethtool_fec_hist_range;17]}
#[repr(C)] pub struct ethtool_fec_stat{pub total:u64,pub lanes:[u64;8]}
#[repr(C)] pub struct ethtool_fec_stats{pub corrected_blocks:ethtool_fec_stat,pub uncorrectable_blocks:ethtool_fec_stat,pub corrected_bits:ethtool_fec_stat}
#[repr(C)] pub struct ethtool_rmon_hist_range{pub low:u16,pub high:u16}
pub const ETHTOOL_RMON_HIST_MAX:usize=11;
#[repr(C)] pub struct ethtool_rmon_stats{pub src:ethtool_mac_stats_src,pub undersize_pkts:u64,pub oversize_pkts:u64,pub fragments:u64,pub jabbers:u64,pub hist:[u64;11],pub hist_tx:[u64;11]}
#[repr(C)] pub struct ethtool_ts_stats{pub pkts:u64,pub onestep_pkts_unconfirmed:u64,pub lost:u64,pub err:u64}
pub const ETH_MODULE_EEPROM_PAGE_LEN:u32=128; pub const ETH_MODULE_MAX_I2C_ADDRESS:u8=0x7f;
#[repr(C)] pub struct ethtool_module_eeprom{pub offset:u32,pub length:u32,pub page:u8,pub bank:u8,pub i2c_address:u8,pub data:*mut u8}
#[repr(C)] pub struct ethtool_module_power_mode_params{pub policy:ethtool_module_power_mode_policy,pub mode:ethtool_module_power_mode}
#[repr(C)] pub struct ethtool_mm_state{pub verify_time:u32,pub max_verify_time:u32,pub verify_status:ethtool_mm_verify_status,pub tx_enabled:bool,pub tx_active:bool,pub pmac_enabled:bool,pub verify_enabled:bool,pub tx_min_frag_size:u32,pub rx_min_frag_size:u32}
#[repr(C)] pub struct ethtool_mm_cfg{pub verify_time:u32,pub verify_enabled:bool,pub tx_enabled:bool,pub pmac_enabled:bool,pub tx_min_frag_size:u32}
#[repr(C)] pub struct ethtool_mm_stats{pub MACMergeFrameAssErrorCount:u64,pub MACMergeFrameSmdErrorCount:u64,pub MACMergeFrameAssOkCount:u64,pub MACMergeFragCountRx:u64,pub MACMergeFragCountTx:u64,pub MACMergeHoldCount:u64}
#[repr(C)] pub enum ethtool_mmsv_event{ETHTOOL_MMSV_LP_SENT_VERIFY_MPACKET,ETHTOOL_MMSV_LD_SENT_VERIFY_MPACKET,ETHTOOL_MMSV_LP_SENT_RESPONSE_MPACKET}
#[repr(C)] pub enum ethtool_mpacket{ETHTOOL_MPACKET_VERIFY,ETHTOOL_MPACKET_RESPONSE}
#[repr(C)] pub struct ethtool_mmsv_ops{pub configure_tx:Option<unsafe extern "C" fn(*mut ethtool_mmsv,bool)>,pub configure_pmac:Option<unsafe extern "C" fn(*mut ethtool_mmsv,bool)>,pub send_mpacket:Option<unsafe extern "C" fn(*mut ethtool_mmsv,ethtool_mpacket)>}
#[repr(C)] pub struct ethtool_mmsv{pub ops:*const ethtool_mmsv_ops,pub dev:*mut net_device,pub lock:spinlock_t,pub status:ethtool_mm_verify_status,pub verify_timer:timer_list,pub verify_enabled:bool,pub verify_retries:i32,pub pmac_enabled:bool,pub verify_time:u32,pub tx_enabled:bool}
#[repr(C)] pub struct ethtool_rxfh_param{pub hfunc:u8,pub indir_size:u32,pub indir:*mut u32,pub key_size:u32,pub key:*mut u8,pub rss_context:u32,pub rss_delete:u8,pub input_xfrm:u8}
#[repr(C)] pub struct ethtool_rxfh_fields{pub data:u32,pub flow_type:u32,pub rss_context:u32}
#[repr(C)] pub struct kernel_ethtool_ts_info{pub cmd:u32,pub so_timestamping:u32,pub phc_index:i32,pub phc_qualifier:hwtstamp_provider_qualifier,pub phc_source:hwtstamp_source,pub phc_phyindex:i32,pub tx_types:u32,pub rx_filters:u32}

pub const ETHTOOL_COALESCE_RX_USECS:u32=1<<0; pub const ETHTOOL_COALESCE_RX_MAX_FRAMES:u32=1<<1; pub const ETHTOOL_COALESCE_RX_USECS_IRQ:u32=1<<2; pub const ETHTOOL_COALESCE_RX_MAX_FRAMES_IRQ:u32=1<<3; pub const ETHTOOL_COALESCE_TX_USECS:u32=1<<4; pub const ETHTOOL_COALESCE_TX_MAX_FRAMES:u32=1<<5; pub const ETHTOOL_COALESCE_TX_USECS_IRQ:u32=1<<6; pub const ETHTOOL_COALESCE_TX_MAX_FRAMES_IRQ:u32=1<<7; pub const ETHTOOL_COALESCE_STATS_BLOCK_USECS:u32=1<<8; pub const ETHTOOL_COALESCE_USE_ADAPTIVE_RX:u32=1<<9; pub const ETHTOOL_COALESCE_USE_ADAPTIVE_TX:u32=1<<10; pub const ETHTOOL_COALESCE_PKT_RATE_LOW:u32=1<<11; pub const ETHTOOL_COALESCE_RX_USECS_LOW:u32=1<<12; pub const ETHTOOL_COALESCE_RX_MAX_FRAMES_LOW:u32=1<<13; pub const ETHTOOL_COALESCE_TX_USECS_LOW:u32=1<<14; pub const ETHTOOL_COALESCE_TX_MAX_FRAMES_LOW:u32=1<<15; pub const ETHTOOL_COALESCE_PKT_RATE_HIGH:u32=1<<16; pub const ETHTOOL_COALESCE_RX_USECS_HIGH:u32=1<<17; pub const ETHTOOL_COALESCE_RX_MAX_FRAMES_HIGH:u32=1<<18; pub const ETHTOOL_COALESCE_TX_USECS_HIGH:u32=1<<19; pub const ETHTOOL_COALESCE_TX_MAX_FRAMES_HIGH:u32=1<<20; pub const ETHTOOL_COALESCE_RATE_SAMPLE_INTERVAL:u32=1<<21; pub const ETHTOOL_COALESCE_USE_CQE_RX:u32=1<<22; pub const ETHTOOL_COALESCE_USE_CQE_TX:u32=1<<23; pub const ETHTOOL_COALESCE_TX_AGGR_MAX_BYTES:u32=1<<24; pub const ETHTOOL_COALESCE_TX_AGGR_MAX_FRAMES:u32=1<<25; pub const ETHTOOL_COALESCE_TX_AGGR_TIME_USECS:u32=1<<26; pub const ETHTOOL_COALESCE_RX_PROFILE:u32=1<<27; pub const ETHTOOL_COALESCE_TX_PROFILE:u32=1<<28; pub const ETHTOOL_COALESCE_RX_CQE_FRAMES:u32=1<<29; pub const ETHTOOL_COALESCE_RX_CQE_NSECS:u32=1<<30; pub const ETHTOOL_COALESCE_ALL_PARAMS:u32=(1<<31)-1;
pub const ETHTOOL_OP_NEEDS_RTNL_LINKSETTINGS:u32=1<<0; pub const ETHTOOL_OP_NEEDS_RTNL_SPFLAGS:u32=1<<1; pub const ETHTOOL_OP_NEEDS_RTNL_SRINGPARAM:u32=1<<2; pub const ETHTOOL_OP_NEEDS_RTNL_SCHANNELS:u32=1<<3; pub const ETHTOOL_OP_NEEDS_RTNL_SCOALESCE:u32=1<<4; pub const ETHTOOL_OP_NEEDS_RTNL_GPAUSEPARAM:u32=1<<5; pub const ETHTOOL_OP_NEEDS_RTNL_SPAUSEPARAM:u32=1<<6; pub const ETHTOOL_OP_NEEDS_RTNL_RSS:u32=1<<7; pub const ETHTOOL_OP_NEEDS_RTNL_GLINK:u32=1<<8;

#[repr(C)] pub struct ethtool_rx_flow_rule{pub rule:*mut flow_rule,pub priv_: [usize;0]}
#[repr(C)] pub struct ethtool_rx_flow_spec_input{pub fs:*const ethtool_rx_flow_spec,pub rss_ctx:u32}
#[repr(C)] pub struct ethtool_netdev_state{pub rss_ctx:xarray,pub rss_lock:mutex,pub rss_indir_user_size:u32,pub phys_id_busy:u8,pub wol_enabled:u8,pub module_fw_flash_in_progress:u8}
#[repr(C)] pub struct ethtool_phy_ops{pub get_sset_count:Option<unsafe extern "C" fn(*mut phy_device)->i32>,pub get_strings:Option<unsafe extern "C" fn(*mut phy_device,*mut u8)->i32>,pub get_stats:Option<unsafe extern "C" fn(*mut phy_device,*mut ethtool_stats,*mut u64)->i32>,pub get_plca_cfg:Option<unsafe extern "C" fn(*mut phy_device,*mut phy_plca_cfg)->i32>,pub set_plca_cfg:Option<unsafe extern "C" fn(*mut phy_device,*const phy_plca_cfg,*mut netlink_ext_ack)->i32>,pub get_plca_status:Option<unsafe extern "C" fn(*mut phy_device,*mut phy_plca_status)->i32>,pub start_cable_test:Option<unsafe extern "C" fn(*mut phy_device,*mut netlink_ext_ack)->i32>,pub start_cable_test_tdr:Option<unsafe extern "C" fn(*mut phy_device,*mut netlink_ext_ack,*const phy_tdr_config)->i32>}

extern "C" { pub fn ethtool_set_ethtool_phy_ops(ops:*const ethtool_phy_ops); pub fn ethtool_params_from_link_mode(s:*mut ethtool_link_ksettings,m:ethtool_link_mode_bit_indices); pub fn ethtool_get_phc_vclocks(dev:*mut net_device,v:*mut *mut i32)->i32; pub fn ethtool_op_get_link(dev:*mut net_device)->u32; pub fn ethtool_op_get_ts_info(dev:*mut net_device,eti:*mut kernel_ethtool_ts_info)->i32; pub fn ethtool_get_ts_info_by_layer(dev:*mut net_device,info:*mut kernel_ethtool_ts_info)->i32; pub fn ethtool_sprintf(data:*mut *mut u8,fmt:*const i8,...); pub fn ethtool_puts(data:*mut *mut u8,s:*const i8); }
pub const fn ethtool_mm_frag_size_add_to_min(val_add:u32)->u32 { (64+4)*(1+val_add)-4 }
pub unsafe fn ethtool_mm_frag_size_min_to_add(val_min:u32,val_add:*mut u32,_extack:*mut netlink_ext_ack)->i32 { let mut a=0; while a<4 { if ethtool_mm_frag_size_add_to_min(a)==val_min {*val_add=a;return 0} a+=1 } -22 }

/* Opaque types supplied by included Linux/uapi translations. */
extern "C" { type ethtool_flow_union; type ethtool_flow_ext; type net_device; type netlink_ext_ack; type ethtool_link_ext_state; type ethtool_link_ext_substate_autoneg; type ethtool_link_ext_substate_link_training; type ethtool_link_ext_substate_link_logical_mismatch; type ethtool_link_ext_substate_bad_signal_integrity; type ethtool_link_ext_substate_cable_issue; type ethtool_link_ext_substate_module; type ethtool_link_settings; type ethtool_mac_stats_src; type ethtool_module_power_mode_policy; type ethtool_module_power_mode; type ethtool_mm_verify_status; type ethtool_link_mode_bit_indices; type spinlock_t; type timer_list; type flow_rule; type xarray; type mutex; type phy_device; type phy_plca_cfg; type phy_plca_status; type phy_tdr_config; type ethtool_stats; }

/* The operation tables retain the C ABI and callback ordering; callback signatures
 * are represented by opaque function pointers because their argument structures
 * are declared by the included uapi/kernel headers. */
#[repr(C)] pub struct ethtool_ops {
    pub supported_input_xfrm:u32, pub cap_link_lanes_supported:u32, pub rxfh_per_ctx_fields:u32, pub rxfh_per_ctx_key:u32, pub cap_rss_rxnfc_adds:u32,
    pub rxfh_indir_space:u32, pub rxfh_key_space:u16, pub rxfh_priv_size:u16, pub rxfh_max_num_contexts:u32, pub supported_coalesce_params:u32,
    pub supported_ring_params:u32, pub supported_hwtstamp_qualifiers:u32, pub op_needs_rtnl:u32,
    pub get_drvinfo:Option<unsafe extern "C" fn(*mut net_device,*mut core::ffi::c_void)>, pub get_regs_len:Option<unsafe extern "C" fn(*mut net_device)->i32>,
    pub get_link:Option<unsafe extern "C" fn(*mut net_device)->u32>, pub begin:Option<unsafe extern "C" fn(*mut net_device)->i32>, pub complete:Option<unsafe extern "C" fn(*mut net_device)>,
    pub get_rxfh:Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_param)->i32>, pub set_rxfh:Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_param,*mut netlink_ext_ack)->i32>,
    pub get_link_ksettings:Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_link_ksettings)->i32>, pub set_link_ksettings:Option<unsafe extern "C" fn(*mut net_device,*const ethtool_link_ksettings)->i32>,
    pub get_mm:Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_mm_state)->i32>, pub set_mm:Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_mm_cfg,*mut netlink_ext_ack)->i32>,
    pub get_mm_stats:Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_mm_stats)>,
}
extern "C" { pub fn ethtool_check_ops(ops:*const ethtool_ops)->i32; pub fn ethtool_rx_flow_rule_create(input:*const ethtool_rx_flow_spec_input)->*mut ethtool_rx_flow_rule; pub fn ethtool_rx_flow_rule_destroy(rule:*mut ethtool_rx_flow_rule); pub fn ethtool_virtdev_validate_cmd(cmd:*const ethtool_link_ksettings)->bool; pub fn ethtool_virtdev_set_link_ksettings(dev:*mut net_device,cmd:*const ethtool_link_ksettings,dev_speed:*mut u32,dev_duplex:*mut u8)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
