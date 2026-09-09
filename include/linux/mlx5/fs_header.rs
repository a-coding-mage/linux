/* Translated from fs.h. */

// Dependencies supplied by the surrounding mlx5 translation.
pub const MLX5_FS_DEFAULT_FLOW_TAG: u32 = 0x0;
pub const MLX5_RDMA_TRANSPORT_BYPASS_PRIO: i32 = 16;
pub const MLX5_FS_MAX_POOL_SIZE: u32 = 1u32 << 30;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mlx5_flow_destination_type {
    MLX5_FLOW_DESTINATION_TYPE_NONE,
    MLX5_FLOW_DESTINATION_TYPE_VPORT,
    MLX5_FLOW_DESTINATION_TYPE_FLOW_TABLE,
    MLX5_FLOW_DESTINATION_TYPE_TIR,
    MLX5_FLOW_DESTINATION_TYPE_FLOW_SAMPLER,
    MLX5_FLOW_DESTINATION_TYPE_UPLINK,
    MLX5_FLOW_DESTINATION_TYPE_PORT,
    MLX5_FLOW_DESTINATION_TYPE_COUNTER,
    MLX5_FLOW_DESTINATION_TYPE_FLOW_TABLE_NUM,
    MLX5_FLOW_DESTINATION_TYPE_RANGE,
    MLX5_FLOW_DESTINATION_TYPE_TABLE_TYPE,
    MLX5_FLOW_DESTINATION_TYPE_VHCA_RX,
}

pub const MLX5_FLOW_CONTEXT_ACTION_FWD_NEXT_PRIO: u32 = 1 << 16;
pub const MLX5_FLOW_CONTEXT_ACTION_ENCRYPT: u32 = 1 << 17;
pub const MLX5_FLOW_CONTEXT_ACTION_DECRYPT: u32 = 1 << 18;
pub const MLX5_FLOW_CONTEXT_ACTION_FWD_NEXT_NS: u32 = 1 << 19;
pub const MLX5_FLOW_TABLE_TUNNEL_EN_REFORMAT: u32 = 1 << 0;
pub const MLX5_FLOW_TABLE_TUNNEL_EN_DECAP: u32 = 1 << 1;
pub const MLX5_FLOW_TABLE_TERMINATION: u32 = 1 << 2;
pub const MLX5_FLOW_TABLE_UNMANAGED: u32 = 1 << 3;
pub const MLX5_FLOW_TABLE_OTHER_VPORT: u32 = 1 << 4;
pub const MLX5_FLOW_TABLE_UPLINK_VPORT: u32 = 1 << 5;
pub const MLX5_FLOW_TABLE_OTHER_ESWITCH: u32 = 1 << 6;

pub const LEFTOVERS_RULE_NUM: i32 = 2;

#[inline]
pub unsafe fn build_leftovers_ft_param(priority: *mut i32, n_ent: *mut i32, n_grp: *mut i32) {
    *priority = 0;
    *n_ent = LEFTOVERS_RULE_NUM;
    *n_grp = LEFTOVERS_RULE_NUM;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mlx5_flow_namespace_type {
    MLX5_FLOW_NAMESPACE_BYPASS,
    MLX5_FLOW_NAMESPACE_KERNEL_RX_MACSEC,
    MLX5_FLOW_NAMESPACE_LAG,
    MLX5_FLOW_NAMESPACE_OFFLOADS,
    MLX5_FLOW_NAMESPACE_ETHTOOL,
    MLX5_FLOW_NAMESPACE_KERNEL,
    MLX5_FLOW_NAMESPACE_LEFTOVERS,
    MLX5_FLOW_NAMESPACE_ANCHOR,
    MLX5_FLOW_NAMESPACE_FDB_BYPASS,
    MLX5_FLOW_NAMESPACE_FDB,
    MLX5_FLOW_NAMESPACE_ESW_EGRESS,
    MLX5_FLOW_NAMESPACE_ESW_INGRESS,
    MLX5_FLOW_NAMESPACE_SNIFFER_RX,
    MLX5_FLOW_NAMESPACE_SNIFFER_TX,
    MLX5_FLOW_NAMESPACE_EGRESS,
    MLX5_FLOW_NAMESPACE_EGRESS_IPSEC,
    MLX5_FLOW_NAMESPACE_EGRESS_MACSEC,
    MLX5_FLOW_NAMESPACE_RDMA_RX,
    MLX5_FLOW_NAMESPACE_RDMA_RX_KERNEL,
    MLX5_FLOW_NAMESPACE_RDMA_TX,
    MLX5_FLOW_NAMESPACE_PORT_SEL,
    MLX5_FLOW_NAMESPACE_RDMA_RX_COUNTERS,
    MLX5_FLOW_NAMESPACE_RDMA_TX_COUNTERS,
    MLX5_FLOW_NAMESPACE_RDMA_RX_IPSEC,
    MLX5_FLOW_NAMESPACE_RDMA_TX_IPSEC,
    MLX5_FLOW_NAMESPACE_RDMA_RX_MACSEC,
    MLX5_FLOW_NAMESPACE_RDMA_TX_MACSEC,
    MLX5_FLOW_NAMESPACE_RDMA_TRANSPORT_RX,
    MLX5_FLOW_NAMESPACE_RDMA_TRANSPORT_TX,
}

pub const FDB_DROP_ROOT: u32 = 0;
pub const FDB_BYPASS_PATH: u32 = 1;
pub const FDB_CRYPTO_INGRESS: u32 = 2;
pub const FDB_TC_OFFLOAD: u32 = 3;
pub const FDB_FT_OFFLOAD: u32 = 4;
pub const FDB_TC_MISS: u32 = 5;
pub const FDB_BR_OFFLOAD: u32 = 6;
pub const FDB_SLOW_PATH: u32 = 7;
pub const FDB_CRYPTO_EGRESS: u32 = 8;
pub const FDB_PER_VPORT: u32 = 9;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fs_flow_table_type {
    FS_FT_NIC_RX = 0x0,
    FS_FT_NIC_TX = 0x1,
    FS_FT_ESW_EGRESS_ACL = 0x2,
    FS_FT_ESW_INGRESS_ACL = 0x3,
    FS_FT_FDB = 0x4,
    FS_FT_SNIFFER_RX = 0x5,
    FS_FT_SNIFFER_TX = 0x6,
    FS_FT_RDMA_RX = 0x7,
    FS_FT_RDMA_TX = 0x8,
    FS_FT_PORT_SEL = 0x9,
    FS_FT_FDB_RX = 0xa,
    FS_FT_FDB_TX = 0xb,
    FS_FT_RDMA_TRANSPORT_RX = 0xd,
    FS_FT_RDMA_TRANSPORT_TX = 0xe,
    FS_FT_MAX_TYPE = 0xe,
}

pub enum mlx5_pkt_reformat {}
pub enum mlx5_modify_hdr {}
pub enum mlx5_flow_definer {}
pub enum mlx5_flow_table {}
pub enum mlx5_flow_group {}
pub enum mlx5_flow_namespace {}
pub enum mlx5_flow_handle {}
pub enum mlx5_core_dev {}
pub enum mlx5_fc {}
pub enum ib_counters {}
pub enum mutex {}

pub const FLOW_CONTEXT_HAS_TAG: u32 = 1 << 0;
pub const FLOW_CONTEXT_UPLINK_HAIRPIN_EN: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_context { pub flags: u32, pub flow_tag: u32, pub flow_source: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_spec {
    pub match_criteria_enable: u8,
    pub match_criteria: [u32; MLX5_ST_SZ_DW_fte_match_param],
    pub match_value: [u32; MLX5_ST_SZ_DW_fte_match_param],
    pub flow_context: mlx5_flow_context,
}

pub const MLX5_FLOW_DEST_VPORT_VHCA_ID: u32 = 1 << 0;
pub const MLX5_FLOW_DEST_VPORT_REFORMAT_ID: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mlx5_flow_dest_range_field { MLX5_FLOW_DEST_RANGE_FIELD_PKT_LEN = 0 }

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_flow_destination_data {
    pub tir_num: u32,
    pub ft_num: u32,
    pub ft: *mut mlx5_flow_table,
    pub counter: *mut mlx5_fc,
    pub vhca: mlx5_flow_destination_vhca,
    pub vport: mlx5_flow_destination_vport,
    pub range: mlx5_flow_destination_range,
    pub sampler_id: u32,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_flow_destination_vhca { pub id: u16 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_flow_destination_vport { pub num: u16, pub vhca_id: u16, pub pkt_reformat: *mut mlx5_pkt_reformat, pub flags: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_flow_destination_range { pub hit_ft: *mut mlx5_flow_table, pub miss_ft: *mut mlx5_flow_table, pub field: mlx5_flow_dest_range_field, pub min: u32, pub max: u32 }
#[repr(C)] pub struct mlx5_flow_destination { pub type_: mlx5_flow_destination_type, pub data: mlx5_flow_destination_data }

#[repr(C)] pub struct mod_hdr_tbl { pub lock: mutex, pub hlist: [u8; 1] }

#[repr(C)] pub struct mlx5_flow_table_attr { pub prio: i32, pub max_fte: i32, pub level: u32, pub flags: u32, pub uid: u16, pub vport: u16, pub esw_owner_vhca_id: u16, pub next_ft: *mut mlx5_flow_table, pub autogroup: mlx5_flow_table_autogroup }
#[repr(C)] pub struct mlx5_flow_table_autogroup { pub max_num_groups: i32, pub num_reserved_entries: i32 }

#[repr(C)] pub struct mlx5_exe_aso { pub object_id: u32, pub base_id: i32, pub type_: u8, pub return_reg_id: u8, pub data: mlx5_exe_aso_data }
#[repr(C)] #[derive(Copy, Clone)] pub union mlx5_exe_aso_data { pub ctrl_data: u32, pub flow_meter: mlx5_exe_aso_flow_meter }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_exe_aso_flow_meter { pub meter_idx: u8, pub init_color: u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct mlx5_fs_vlan { pub ethtype: u16, pub vid: u16, pub prio: u8 }
pub const MLX5_FS_VLAN_DEPTH: usize = 2;
pub const FLOW_ACT_NO_APPEND: u32 = 1 << 0;
pub const FLOW_ACT_IGNORE_FLOW_LEVEL: u32 = 1 << 1;
#[repr(C)] pub struct mlx5_flow_act { pub action: u32, pub modify_hdr: *mut mlx5_modify_hdr, pub pkt_reformat: *mut mlx5_pkt_reformat, pub crypto: mlx5_flow_act_crypto_params, pub flags: u32, pub vlan: [mlx5_fs_vlan; 2], pub counters: *mut ib_counters, pub fg: *mut mlx5_flow_group, pub exe_aso: mlx5_exe_aso }
#[repr(C)] pub struct mlx5_flow_act_crypto_params { pub type_: u8, pub obj_id: u32 }

// External declarations from the included mlx5 headers and implementation.
extern "C" {
    pub fn mlx5_get_fdb_sub_ns(dev: *mut mlx5_core_dev, n: i32) -> *mut mlx5_flow_namespace;
    pub fn mlx5_get_flow_namespace(dev: *mut mlx5_core_dev, type_: mlx5_flow_namespace_type) -> *mut mlx5_flow_namespace;
    pub fn mlx5_get_flow_vport_namespace(dev: *mut mlx5_core_dev, type_: mlx5_flow_namespace_type, vport_idx: i32) -> *mut mlx5_flow_namespace;
    pub fn mlx5_create_flow_table(ns: *mut mlx5_flow_namespace, attr: *mut mlx5_flow_table_attr) -> *mut mlx5_flow_table;
    pub fn mlx5_create_auto_grouped_flow_table(ns: *mut mlx5_flow_namespace, attr: *mut mlx5_flow_table_attr) -> *mut mlx5_flow_table;
    pub fn mlx5_create_vport_flow_table(ns: *mut mlx5_flow_namespace, attr: *mut mlx5_flow_table_attr, vport: u16) -> *mut mlx5_flow_table;
    pub fn mlx5_create_lag_demux_flow_table(ns: *mut mlx5_flow_namespace, attr: *mut mlx5_flow_table_attr) -> *mut mlx5_flow_table;
    pub fn mlx5_destroy_flow_table(ft: *mut mlx5_flow_table) -> i32;
    pub fn mlx5_create_flow_group(ft: *mut mlx5_flow_table, input: *mut u32) -> *mut mlx5_flow_group;
    pub fn mlx5_destroy_flow_group(fg: *mut mlx5_flow_group);
    pub fn mlx5_add_flow_rules(ft: *mut mlx5_flow_table, spec: *const mlx5_flow_spec, act: *mut mlx5_flow_act, dest: *mut mlx5_flow_destination, num_dest: i32) -> *mut mlx5_flow_handle;
    pub fn mlx5_del_flow_rules(fr: *mut mlx5_flow_handle);
    pub fn mlx5_modify_rule_destination(handler: *mut mlx5_flow_handle, new_dest: *mut mlx5_flow_destination, old_dest: *mut mlx5_flow_destination) -> i32;
    pub fn mlx5_fc_create(dev: *mut mlx5_core_dev, aging: bool) -> *mut mlx5_fc;
    pub fn mlx5_fc_destroy(dev: *mut mlx5_core_dev, counter: *mut mlx5_fc);
    pub fn mlx5_fc_local_create(counter_id: u32, offset: u32, bulk_size: u32) -> *mut mlx5_fc;
    pub fn mlx5_fc_local_destroy(counter: *mut mlx5_fc);
    pub fn mlx5_fc_local_get(counter: *mut mlx5_fc);
    pub fn mlx5_fc_local_put(counter: *mut mlx5_fc);
    pub fn mlx5_fc_query_lastuse(counter: *mut mlx5_fc) -> u64;
    pub fn mlx5_fc_query_cached(counter: *mut mlx5_fc, bytes: *mut u64, packets: *mut u64, lastuse: *mut u64);
    pub fn mlx5_fc_query_cached_raw(counter: *mut mlx5_fc, bytes: *mut u64, packets: *mut u64, lastuse: *mut u64);
    pub fn mlx5_fc_query(dev: *mut mlx5_core_dev, counter: *mut mlx5_fc, packets: *mut u64, bytes: *mut u64) -> i32;
    pub fn mlx5_fc_id(counter: *mut mlx5_fc) -> u32;
    pub fn mlx5_fs_add_rx_underlay_qpn(dev: *mut mlx5_core_dev, underlay_qpn: u32) -> i32;
    pub fn mlx5_fs_remove_rx_underlay_qpn(dev: *mut mlx5_core_dev, underlay_qpn: u32) -> i32;
    pub fn mlx5_modify_header_alloc(dev: *mut mlx5_core_dev, ns_type: u8, num_actions: u8, modify_actions: *mut core::ffi::c_void) -> *mut mlx5_modify_hdr;
    pub fn mlx5_modify_header_dealloc(dev: *mut mlx5_core_dev, modify_hdr: *mut mlx5_modify_hdr);
    pub fn mlx5_create_match_definer(dev: *mut mlx5_core_dev, ns_type: mlx5_flow_namespace_type, format_id: u16, match_mask: *mut u32) -> *mut mlx5_flow_definer;
    pub fn mlx5_destroy_match_definer(dev: *mut mlx5_core_dev, definer: *mut mlx5_flow_definer);
    pub fn mlx5_get_match_definer_id(definer: *mut mlx5_flow_definer) -> i32;
    pub fn mlx5_packet_reformat_alloc(dev: *mut mlx5_core_dev, params: *mut mlx5_pkt_reformat_params, ns_type: mlx5_flow_namespace_type) -> *mut mlx5_pkt_reformat;
    pub fn mlx5_packet_reformat_dealloc(dev: *mut mlx5_core_dev, reformat: *mut mlx5_pkt_reformat);
    pub fn mlx5_flow_table_id(ft: *mut mlx5_flow_table) -> u32;
    pub fn mlx5_get_root_namespace(dev: *mut mlx5_core_dev, ns_type: mlx5_flow_namespace_type) -> *mut mlx5_flow_root_namespace;
    pub fn mlx5_fs_set_root_dev(dev: *mut mlx5_core_dev, new_dev: *mut mlx5_core_dev, table_type: fs_flow_table_type) -> i32;
}

pub enum mlx5_flow_root_namespace {}

#[repr(C)]
pub struct mlx5_pkt_reformat_params {
    pub type_: i32,
    pub param_0: u8,
    pub param_1: u8,
    pub size: usize,
    pub data: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
