/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */
/* Direct Rust translation of mana.h. External kernel types are intentionally unresolved. */

pub type mana_handle_t = u64;
pub const INVALID_MANA_HANDLE: mana_handle_t = u64::MAX;
pub const MANA_MAJOR_VERSION: u32 = 0;
pub const MANA_MINOR_VERSION: u32 = 1;
pub const MANA_MICRO_VERSION: u32 = 1;

#[repr(i32)] pub enum TRI_STATE { TRI_STATE_UNKNOWN = -1, TRI_STATE_FALSE = 0, TRI_STATE_TRUE = 1 }
#[repr(u32)] pub enum mana_priv_flag_bits { MANA_PRIV_FLAG_USE_FULL_PAGE_RXBUF = 0, MANA_PRIV_FLAG_MAX }
pub const MANA_INDIRECT_TABLE_MAX_SIZE: usize = 512;
pub const MANA_INDIRECT_TABLE_DEF_SIZE: usize = 64;
pub const MANA_HASH_KEY_SIZE: usize = 40;
pub const COMP_ENTRY_SIZE: usize = 64;
pub const MAX_RX_BUFFERS_PER_QUEUE: usize = 8192;
pub const DEF_RX_BUFFERS_PER_QUEUE: usize = 1024;
pub const MIN_RX_BUFFERS_PER_QUEUE: usize = 128;
pub const MAX_TX_BUFFERS_PER_QUEUE: usize = 16384;
pub const DEF_TX_BUFFERS_PER_QUEUE: usize = 256;
pub const MIN_TX_BUFFERS_PER_QUEUE: usize = 128;
pub const EQ_SIZE: usize = 8 * MANA_PAGE_SIZE;
pub const LOG2_EQ_THROTTLE: u32 = 3;
pub const MAX_PORTS_IN_MANA_DEV: usize = 256;
pub const MANA_RXCOMP_OOB_NUM_PPI: usize = 4;
pub const MANA_CQE_COAL_PKTS_8: usize = 8;
pub const MANA_INTR_MODR_USEC_DEF: u32 = 0;
pub const MANA_INTR_MODR_COMP_DEF: u32 = 0;
pub const MANA_ADAPTIVE_RX_DEF: bool = true;
pub const MANA_ADAPTIVE_TX_DEF: bool = true;
pub const MANA_INTR_MODR_USEC_MAX: u32 = GENMASK(9, 0);
pub const MANA_INTR_MODR_USEC_VLD: u32 = BIT(15);
pub const MANA_INTR_MODR_COMP_MAX: u32 = GENMASK(7, 0);
pub const MANA_INTR_MODR_COMP_MASK: u32 = GENMASK(23, 16);
pub const MANA_STATS_RX_COUNT: usize = 6 + MANA_CQE_COAL_PKTS_8 - 1;
pub const MANA_STATS_TX_COUNT: usize = 11;
pub const MANA_RX_FRAG_ALIGNMENT: usize = 64;

#[repr(C)] pub struct mana_stats_rx { pub packets:u64, pub bytes:u64, pub xdp_drop:u64, pub xdp_tx:u64, pub xdp_redirect:u64, pub pkt_len0_err:u64, pub coalesced_cqe:[u64; MANA_CQE_COAL_PKTS_8-1], pub syncp:u64_stats_sync }
#[repr(C)] pub struct mana_stats_tx { pub packets:u64, pub bytes:u64, pub xdp_xmit:u64, pub tso_packets:u64, pub tso_bytes:u64, pub tso_inner_packets:u64, pub tso_inner_bytes:u64, pub short_pkt_fmt:u64, pub long_pkt_fmt:u64, pub csum_partial:u64, pub mana_map_err:u64, pub syncp:u64_stats_sync }
#[repr(C)] pub struct mana_txq { pub gdma_sq:*mut gdma_queue, pub gdma_txq_id:u32, pub vp_offset:u16, pub ndev:*mut net_device, pub pending_skbs:sk_buff_head, pub net_txq:*mut netdev_queue, pub pending_sends:atomic_t, pub napi_initialized:bool, pub stats:mana_stats_tx }
#[repr(C)] pub struct mana_skb_head { pub dma_handle:[dma_addr_t; MAX_SKB_FRAGS+2], pub size:[u32; MAX_SKB_FRAGS+2] }
pub const MANA_HEADROOM: usize = core::mem::size_of::<mana_skb_head>();
#[repr(u32)] pub enum mana_tx_pkt_format { MANA_SHORT_PKT_FMT=0, MANA_LONG_PKT_FMT=1 }
#[repr(C)] pub struct mana_tx_short_oob { pub pkt_fmt:u32, pub is_outer_ipv4:u32, pub is_outer_ipv6:u32, pub comp_iphdr_csum:u32, pub comp_tcp_csum:u32, pub comp_udp_csum:u32, pub supress_txcqe_gen:u32, pub vcq_num:u32, pub trans_off:u32, pub vsq_frame:u32, pub short_vp_offset:u32 }
#[repr(C)] pub struct mana_tx_long_oob { pub is_encap:u32, pub inner_is_ipv6:u32, pub inner_tcp_opt:u32, pub inject_vlan_pri_tag:u32, pub reserved1:u32, pub pcp:u32, pub dei:u32, pub vlan_id:u32, pub inner_frame_offset:u32, pub inner_ip_rel_offset:u32, pub long_vp_offset:u32, pub reserved2:u32, pub reserved3:u32, pub reserved4:u32 }
#[repr(C)] pub struct mana_tx_oob { pub s_oob:mana_tx_short_oob, pub l_oob:mana_tx_long_oob }
#[repr(u32)] pub enum mana_cq_type { MANA_CQ_TYPE_RX, MANA_CQ_TYPE_TX }
#[repr(u32)] pub enum mana_cqe_type { CQE_INVALID=0, CQE_RX_OKAY=1, CQE_RX_COALESCED_4=2, CQE_RX_OBJECT_FENCE=3, CQE_RX_TRUNCATED=4, CQE_RX_COALESCED_8=7, CQE_TX_OKAY=32, CQE_TX_SA_DROP, CQE_TX_MTU_DROP, CQE_TX_INVALID_OOB, CQE_TX_INVALID_ETH_TYPE, CQE_TX_HDR_PROCESSING_ERROR, CQE_TX_VF_DISABLED, CQE_TX_VPORT_IDX_OUT_OF_RANGE, CQE_TX_VPORT_DISABLED, CQE_TX_VLAN_TAGGING_VIOLATION }
#[repr(C)] pub struct mana_cqe_header { pub cqe_type:u32, pub client_type:u32, pub vendor_err:u32 }
pub const MANA_CQE_COMPLETION:u32=1;
pub const NDIS_HASH_IPV4:u32=BIT(0); pub const NDIS_HASH_TCP_IPV4:u32=BIT(1); pub const NDIS_HASH_UDP_IPV4:u32=BIT(2); pub const NDIS_HASH_IPV6:u32=BIT(3); pub const NDIS_HASH_TCP_IPV6:u32=BIT(4); pub const NDIS_HASH_UDP_IPV6:u32=BIT(5); pub const NDIS_HASH_IPV6_EX:u32=BIT(6); pub const NDIS_HASH_TCP_IPV6_EX:u32=BIT(7); pub const NDIS_HASH_UDP_IPV6_EX:u32=BIT(8);
pub const MANA_HASH_L3:u32=NDIS_HASH_IPV4|NDIS_HASH_IPV6|NDIS_HASH_IPV6_EX;
pub const MANA_HASH_L4:u32=NDIS_HASH_TCP_IPV4|NDIS_HASH_UDP_IPV4|NDIS_HASH_TCP_IPV6|NDIS_HASH_UDP_IPV6|NDIS_HASH_TCP_IPV6_EX|NDIS_HASH_UDP_IPV6_EX;
pub const MANA_HASH_ENABLE_SUPPORTED:u32=NDIS_HASH_IPV4|NDIS_HASH_TCP_IPV4|NDIS_HASH_UDP_IPV4|NDIS_HASH_IPV6|NDIS_HASH_TCP_IPV6|NDIS_HASH_UDP_IPV6;

#[repr(C)] pub union mana_rxcomp_perpkt_info { pub first: mana_rxcomp_perpkt_info_first, pub second: mana_rxcomp_perpkt_info_second }
#[repr(C)] pub struct mana_rxcomp_perpkt_info_first { pub pkt_len:u32, pub reserved1:u32, pub reserved2:u32, pub pkt_hash:u32 }
#[repr(C)] pub struct mana_rxcomp_perpkt_info_second { pub pkt_hash0:u32, pub pkt_len0:u16, pub pkt_len1:u16, pub pkt_hash1:u32 }
#[repr(C)] pub struct mana_rxcomp_oob { pub cqe_hdr:mana_cqe_header, pub rx_vlan_id:u32, pub rx_vlantag_present:u32, pub rx_outer_iphdr_csum_succeed:u32, pub rx_outer_iphdr_csum_fail:u32, pub reserved1:u32, pub rx_hashtype:u32, pub rx_iphdr_csum_succeed:u32, pub rx_iphdr_csum_fail:u32, pub rx_tcp_csum_succeed:u32, pub rx_tcp_csum_fail:u32, pub rx_udp_csum_succeed:u32, pub rx_udp_csum_fail:u32, pub reserved2:u32, pub ppi:[mana_rxcomp_perpkt_info;4], pub rx_wqe_offset:u32 }
#[repr(C)] pub struct mana_tx_comp_oob { pub cqe_hdr:mana_cqe_header, pub tx_data_offset:u32, pub tx_sgl_offset:u32, pub tx_wqe_offset:u32, pub reserved:[u32;12] }
#[repr(C)] pub struct mana_obj_spec { pub queue_index:u32, pub gdma_region:u64, pub queue_size:u32, pub attached_eq:u32, pub modr_ctx_id:u32, pub req_cq_moderation:u8, pub cq_moderation_comp:u16, pub cq_moderation_usec:u16 }
#[repr(u32)] pub enum mana_command_code { MANA_QUERY_DEV_CONFIG=0x20001, MANA_QUERY_GF_STAT, MANA_CONFIG_VPORT_TX, MANA_CREATE_WQ_OBJ, MANA_DESTROY_WQ_OBJ, MANA_FENCE_RQ, MANA_CONFIG_VPORT_RX, MANA_QUERY_VPORT_CONFIG, MANA_QUERY_LINK_CONFIG=0x2000a, MANA_SET_BW_CLAMP, MANA_QUERY_PHY_STAT=0x2000c, MANA_REGISTER_FILTER=0x28000, MANA_DEREGISTER_FILTER, MANA_REGISTER_HW_PORT=0x28003, MANA_DEREGISTER_HW_PORT }
pub const GDMA_CQ_NO_EQ:u16=0xffff; pub const MANA_MAX_NUM_QUEUES:u32=64; pub const MANA_DEF_NUM_QUEUES:u32=16; pub const MANA_SHORT_VPORT_OFFSET_MAX:u32=(1u32<<8)-1;

// The remaining declarations retain the C ABI and external kernel dependencies.
extern "C" {
    pub fn mana_start_xmit(skb:*mut sk_buff, ndev:*mut net_device) -> netdev_tx_t;
    pub fn mana_config_rss(ac:*mut mana_port_context, rx:TRI_STATE, update_hash:bool, update_tab:bool)->i32;
    pub fn mana_disable_vport_rx(apc:*mut mana_port_context)->i32;
    pub fn mana_alloc_queues(ndev:*mut net_device)->i32; pub fn mana_attach(ndev:*mut net_device)->i32; pub fn mana_detach(ndev:*mut net_device, from_close:bool)->i32;
    pub fn mana_probe(gd:*mut gdma_dev, resuming:bool)->i32; pub fn mana_remove(gd:*mut gdma_dev, suspending:bool);
    pub fn mana_rdma_probe(gd:*mut gdma_dev)->i32; pub fn mana_rdma_remove(gd:*mut gdma_dev);
    pub fn mana_xdp_tx(skb:*mut sk_buff, ndev:*mut net_device); pub fn mana_xdp_xmit(ndev:*mut net_device,n:i32,frames:*mut *mut xdp_frame,flags:u32)->i32;
    pub fn mana_query_gf_stats(ac:*mut mana_context)->i32; pub fn mana_query_link_cfg(apc:*mut mana_port_context)->i32; pub fn mana_set_bw_clamp(apc:*mut mana_port_context,speed:u32,enable_clamping:i32)->i32;
    pub fn mana_pre_dealloc_rxbufs(apc:*mut mana_port_context); pub fn mana_unmap_skb(skb:*mut sk_buff,apc:*mut mana_port_context);
    pub fn mana_create_wq_obj(apc:*mut mana_port_context,vport:mana_handle_t,wq_type:u32,wq_spec:*mut mana_obj_spec,cq_spec:*mut mana_obj_spec,wq_obj:*mut mana_handle_t)->i32;
    pub fn mana_destroy_wq_obj(apc:*mut mana_port_context,wq_type:u32,wq_obj:mana_handle_t);
    pub fn mana_cfg_vport(apc:*mut mana_port_context,protection_dom_id:u32,doorbell_pg_id:u32,check_channel_changing:bool)->i32; pub fn mana_uncfg_vport(apc:*mut mana_port_context);
    pub fn mana_create_eq(apc:*mut mana_port_context)->i32; pub fn mana_destroy_eq(apc:*mut mana_port_context);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
