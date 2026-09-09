/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ocfs1_fs_compat.h
 *
 * OCFS1 volume header definitions.  OCFS2 creates valid but unmountable
 * OCFS1 volume headers on the first two sectors of an OCFS2 volume.
 * This allows an OCFS1 volume to see the partition and cleanly fail to
 * mount it.
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

pub const OCFS1_MAX_VOL_SIGNATURE_LEN: usize = 128;
pub const OCFS1_MAX_MOUNT_POINT_LEN: usize = 128;
pub const OCFS1_MAX_VOL_ID_LENGTH: usize = 16;
pub const OCFS1_MAX_VOL_LABEL_LEN: usize = 64;
pub const OCFS1_MAX_CLUSTER_NAME_LEN: usize = 64;

pub const OCFS1_MAJOR_VERSION: u32 = 2;
pub const OCFS1_MINOR_VERSION: u32 = 0;
pub const OCFS1_VOLUME_SIGNATURE: &[u8] = b"OracleCFS\0";

/*
 * OCFS1 superblock.  Lives at sector 0.
 */
#[repr(C)]
pub struct ocfs1_vol_disk_hdr {
    /*00*/ pub minor_version: u32,
    pub major_version: u32,
    /*08*/ pub signature: [u8; OCFS1_MAX_VOL_SIGNATURE_LEN],
    /*88*/ pub mount_point: [u8; OCFS1_MAX_MOUNT_POINT_LEN],
    /*108*/ pub serial_num: u64,
    /*110*/ pub device_size: u64,
    pub start_off: u64,
    /*120*/ pub bitmap_off: u64,
    pub publ_off: u64,
    /*130*/ pub vote_off: u64,
    pub root_bitmap_off: u64,
    /*140*/ pub data_start_off: u64,
    pub root_bitmap_size: u64,
    /*150*/ pub root_off: u64,
    pub root_size: u64,
    /*160*/ pub cluster_size: u64,
    pub num_nodes: u64,
    /*170*/ pub num_clusters: u64,
    pub dir_node_size: u64,
    /*180*/ pub file_node_size: u64,
    pub internal_off: u64,
    /*190*/ pub node_cfg_off: u64,
    pub node_cfg_size: u64,
    /*1A0*/ pub new_cfg_off: u64,
    pub prot_bits: u32,
    pub excl_mount: i32,
    /*1B0*/
}

#[repr(C)]
pub struct ocfs1_disk_lock {
    /*00*/ pub curr_master: u32,
    pub file_lock: u8,
    pub compat_pad: [u8; 3], /* Not in original definition.  Used to
                                make the already existing alignment
                                explicit */
    pub last_write_time: u64,
    /*10*/ pub last_read_time: u64,
    pub writer_node_num: u32,
    pub reader_node_num: u32,
    /*20*/ pub oin_node_map: u64,
    pub dlock_seq_num: u64,
    /*30*/
}

/*
 * OCFS1 volume label.  Lives at sector 1.
 */
#[repr(C)]
pub struct ocfs1_vol_label {
    /*00*/ pub disk_lock: ocfs1_disk_lock,
    /*30*/ pub label: [u8; OCFS1_MAX_VOL_LABEL_LEN],
    /*70*/ pub label_len: u16,
    /*72*/ pub vol_id: [u8; OCFS1_MAX_VOL_ID_LENGTH],
    /*82*/ pub vol_id_len: u16,
    /*84*/ pub cluster_name: [u8; OCFS1_MAX_CLUSTER_NAME_LEN],
    /*A4*/ pub cluster_name_len: u16,
    /*A6*/
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
