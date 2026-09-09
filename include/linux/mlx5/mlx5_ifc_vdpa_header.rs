/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2020 Mellanox Technologies Ltd. */

pub const MLX5_VIRTIO_Q_EVENT_MODE_NO_MSIX_MODE: u32 = 0x0;
pub const MLX5_VIRTIO_Q_EVENT_MODE_QP_MODE: u32 = 0x1;
pub const MLX5_VIRTIO_Q_EVENT_MODE_MSIX_MODE: u32 = 0x2;

pub const MLX5_VIRTIO_EMULATION_VIRTIO_QUEUE_TYPE_SPLIT: u32 = 0;
pub const MLX5_VIRTIO_EMULATION_VIRTIO_QUEUE_TYPE_PACKED: u32 = 1;

pub const MLX5_VIRTIO_EMULATION_CAP_VIRTIO_QUEUE_TYPE_SPLIT: u32 =
    1u32 << MLX5_VIRTIO_EMULATION_VIRTIO_QUEUE_TYPE_SPLIT;
pub const MLX5_VIRTIO_EMULATION_CAP_VIRTIO_QUEUE_TYPE_PACKED: u32 =
    1u32 << MLX5_VIRTIO_EMULATION_VIRTIO_QUEUE_TYPE_PACKED;

#[repr(C)]
pub struct mlx5_ifc_virtio_q_bits {
    pub virtio_q_type: [u8; 0x8],
    pub reserved_at_8: [u8; 0x5],
    pub event_mode: [u8; 0x3],
    pub queue_index: [u8; 0x10],
    pub full_emulation: [u8; 0x1],
    pub virtio_version_1_0: [u8; 0x1],
    pub reserved_at_22: [u8; 0x2],
    pub offload_type: [u8; 0x4],
    pub event_qpn_or_msix: [u8; 0x18],
    pub doorbell_stride_index: [u8; 0x10],
    pub queue_size: [u8; 0x10],
    pub device_emulation_id: [u8; 0x20],
    pub desc_addr: [u8; 0x40],
    pub used_addr: [u8; 0x40],
    pub available_addr: [u8; 0x40],
    pub virtio_q_mkey: [u8; 0x20],
    pub max_tunnel_desc: [u8; 0x10],
    pub reserved_at_170: [u8; 0x8],
    pub error_type: [u8; 0x8],
    pub umem_1_id: [u8; 0x20],
    pub umem_1_size: [u8; 0x20],
    pub umem_1_offset: [u8; 0x40],
    pub umem_2_id: [u8; 0x20],
    pub umem_2_size: [u8; 0x20],
    pub umem_2_offset: [u8; 0x40],
    pub umem_3_id: [u8; 0x20],
    pub umem_3_size: [u8; 0x20],
    pub umem_3_offset: [u8; 0x40],
    pub counter_set_id: [u8; 0x20],
    pub reserved_at_320: [u8; 0x8],
    pub pd: [u8; 0x18],
    pub reserved_at_340: [u8; 0x20],
    pub desc_group_mkey: [u8; 0x20],
    pub reserved_at_380: [u8; 0x80],
}

#[repr(C)]
pub struct mlx5_ifc_virtio_net_q_object_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x20],
    pub vhca_id: [u8; 0x10],
    pub reserved_at_70: [u8; 0x10],
    pub queue_feature_bit_mask_12_3: [u8; 0xa],
    pub dirty_bitmap_dump_enable: [u8; 0x1],
    pub vhost_log_page: [u8; 0x5],
    pub reserved_at_90: [u8; 0xc],
    pub state: [u8; 0x4],
    pub reserved_at_a0: [u8; 0x5],
    pub queue_feature_bit_mask_2_0: [u8; 0x3],
    pub tisn_or_qpn: [u8; 0x18],
    pub dirty_bitmap_mkey: [u8; 0x20],
    pub dirty_bitmap_size: [u8; 0x20],
    pub dirty_bitmap_addr: [u8; 0x40],
    pub hw_available_index: [u8; 0x10],
    pub hw_used_index: [u8; 0x10],
    pub reserved_at_160: [u8; 0xa0],
    pub virtio_q_context: mlx5_ifc_virtio_q_bits,
}

#[repr(C)]
pub struct mlx5_ifc_create_virtio_net_q_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_virtio_net_q_object_bits,
}
#[repr(C)]
pub struct mlx5_ifc_create_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}
#[repr(C)]
pub struct mlx5_ifc_destroy_virtio_net_q_in_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}
#[repr(C)]
pub struct mlx5_ifc_destroy_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}
#[repr(C)]
pub struct mlx5_ifc_query_virtio_net_q_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}
#[repr(C)]
pub struct mlx5_ifc_query_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_virtio_net_q_object_bits,
}

pub const MLX5_VIRTQ_MODIFY_MASK_STATE: u64 = (1u64) << 0;
pub const MLX5_VIRTQ_MODIFY_MASK_DIRTY_BITMAP_PARAMS: u64 = (1u64) << 3;
pub const MLX5_VIRTQ_MODIFY_MASK_DIRTY_BITMAP_DUMP_ENABLE: u64 = (1u64) << 4;
pub const MLX5_VIRTQ_MODIFY_MASK_VIRTIO_Q_ADDRS: u64 = (1u64) << 6;
pub const MLX5_VIRTQ_MODIFY_MASK_VIRTIO_Q_AVAIL_IDX: u64 = (1u64) << 7;
pub const MLX5_VIRTQ_MODIFY_MASK_VIRTIO_Q_USED_IDX: u64 = (1u64) << 8;
pub const MLX5_VIRTQ_MODIFY_MASK_QUEUE_VIRTIO_VERSION: u64 = (1u64) << 10;
pub const MLX5_VIRTQ_MODIFY_MASK_VIRTIO_Q_MKEY: u64 = (1u64) << 11;
pub const MLX5_VIRTQ_MODIFY_MASK_QUEUE_FEATURES: u64 = (1u64) << 12;
pub const MLX5_VIRTQ_MODIFY_MASK_DESC_GROUP_MKEY: u64 = (1u64) << 14;

pub const MLX5_VIRTIO_NET_Q_OBJECT_STATE_INIT: u32 = 0x0;
pub const MLX5_VIRTIO_NET_Q_OBJECT_STATE_RDY: u32 = 0x1;
pub const MLX5_VIRTIO_NET_Q_OBJECT_STATE_SUSPEND: u32 = 0x2;
pub const MLX5_VIRTIO_NET_Q_OBJECT_STATE_ERR: u32 = 0x3;

/* This indicates that the object was not created or has already
 * been desroyed. It is very safe to assume that this object will never
 * have so many states
 */
pub const MLX5_VIRTIO_NET_Q_OBJECT_NONE: u32 = 0xffffffff;

pub const MLX5_RQTC_LIST_Q_TYPE_RQ: u32 = 0x0;
pub const MLX5_RQTC_LIST_Q_TYPE_VIRTIO_NET_Q: u32 = 0x1;

#[repr(C)]
pub struct mlx5_ifc_modify_virtio_net_q_in_bits {
    pub general_obj_in_cmd_hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub obj_context: mlx5_ifc_virtio_net_q_object_bits,
}
#[repr(C)]
pub struct mlx5_ifc_modify_virtio_net_q_out_bits {
    pub general_obj_out_cmd_hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}

#[repr(C)]
pub struct mlx5_ifc_virtio_q_counters_bits {
    pub modify_field_select: [u8; 0x40],
    pub reserved_at_40: [u8; 0x40],
    pub received_desc: [u8; 0x40],
    pub completed_desc: [u8; 0x40],
    pub error_cqes: [u8; 0x20],
    pub bad_desc_errors: [u8; 0x20],
    pub exceed_max_chain: [u8; 0x20],
    pub invalid_buffer: [u8; 0x20],
    pub reserved_at_180: [u8; 0x280],
}
#[repr(C)]
pub struct mlx5_ifc_create_virtio_q_counters_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub virtio_q_counters: mlx5_ifc_virtio_q_counters_bits,
}
#[repr(C)]
pub struct mlx5_ifc_create_virtio_q_counters_out_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub virtio_q_counters: mlx5_ifc_virtio_q_counters_bits,
}
#[repr(C)]
pub struct mlx5_ifc_destroy_virtio_q_counters_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}
#[repr(C)]
pub struct mlx5_ifc_destroy_virtio_q_counters_out_bits {
    pub hdr: mlx5_ifc_general_obj_out_cmd_hdr_bits,
}
#[repr(C)]
pub struct mlx5_ifc_query_virtio_q_counters_in_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
}
#[repr(C)]
pub struct mlx5_ifc_query_virtio_q_counters_out_bits {
    pub hdr: mlx5_ifc_general_obj_in_cmd_hdr_bits,
    pub counters: mlx5_ifc_virtio_q_counters_bits,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
