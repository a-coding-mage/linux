/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of ocfs2_ioctl.h.
 *
 * Kernel integer types and ioctl encoding macros are supplied by dependencies.
 */

#[repr(C)]
pub struct ocfs2_space_resv {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: i64,
    pub l_len: i64, /* len == 0 means until end of file */
    pub l_sysid: i32,
    pub l_pid: u32,
    pub l_pad: [i32; 4], /* reserve area */
}

pub const OCFS2_IOC_ALLOCSP: _ = _IOW(b'X', 10, core::mem::size_of::<ocfs2_space_resv>());
pub const OCFS2_IOC_FREESP: _ = _IOW(b'X', 11, core::mem::size_of::<ocfs2_space_resv>());
pub const OCFS2_IOC_RESVSP: _ = _IOW(b'X', 40, core::mem::size_of::<ocfs2_space_resv>());
pub const OCFS2_IOC_UNRESVSP: _ = _IOW(b'X', 41, core::mem::size_of::<ocfs2_space_resv>());
pub const OCFS2_IOC_ALLOCSP64: _ = _IOW(b'X', 36, core::mem::size_of::<ocfs2_space_resv>());
pub const OCFS2_IOC_FREESP64: _ = _IOW(b'X', 37, core::mem::size_of::<ocfs2_space_resv>());
pub const OCFS2_IOC_RESVSP64: _ = _IOW(b'X', 42, core::mem::size_of::<ocfs2_space_resv>());
pub const OCFS2_IOC_UNRESVSP64: _ = _IOW(b'X', 43, core::mem::size_of::<ocfs2_space_resv>());

#[repr(C)]
pub struct ocfs2_new_group_input {
    pub group: u64,
    pub clusters: u32,
    pub frees: u32,
    pub chain: u16,
    pub reserved1: u16,
    pub reserved2: u32,
}

pub const OCFS2_IOC_GROUP_EXTEND: _ = _IOW(b'o', 1, core::mem::size_of::<i32>());
pub const OCFS2_IOC_GROUP_ADD: _ = _IOW(b'o', 2, core::mem::size_of::<ocfs2_new_group_input>());
pub const OCFS2_IOC_GROUP_ADD64: _ = _IOW(b'o', 3, core::mem::size_of::<ocfs2_new_group_input>());

#[repr(C)]
pub struct reflink_arguments {
    pub old_path: u64,
    pub new_path: u64,
    pub preserve: u64,
}
pub const OCFS2_IOC_REFLINK: _ = _IOW(b'o', 4, core::mem::size_of::<reflink_arguments>());

pub const OCFS2_INFO_MAX_REQUEST: u32 = 50;
pub const OCFS2_TEXT_UUID_LEN: usize = (OCFS2_VOL_UUID_LEN * 2);
pub const OCFS2_INFO_MAGIC: u32 = 0x4F32494E;

#[repr(C)]
pub struct ocfs2_info {
    pub oi_requests: u64,
    pub oi_count: u32,
    pub oi_pad: u32,
}

#[repr(C)]
pub struct ocfs2_info_request {
    pub ir_magic: u32,
    pub ir_code: u32,
    pub ir_size: u32,
    pub ir_flags: u32,
}

#[repr(C)]
pub struct ocfs2_info_clustersize { pub ic_req: ocfs2_info_request, pub ic_clustersize: u32, pub ic_pad: u32 }
#[repr(C)]
pub struct ocfs2_info_blocksize { pub ib_req: ocfs2_info_request, pub ib_blocksize: u32, pub ib_pad: u32 }
#[repr(C)]
pub struct ocfs2_info_maxslots { pub im_req: ocfs2_info_request, pub im_max_slots: u32, pub im_pad: u32 }

#[repr(C, packed)]
pub struct ocfs2_info_label { pub il_req: ocfs2_info_request, pub il_label: [u8; OCFS2_MAX_VOL_LABEL_LEN] }
#[repr(C, packed)]
pub struct ocfs2_info_uuid { pub iu_req: ocfs2_info_request, pub iu_uuid_str: [u8; OCFS2_TEXT_UUID_LEN + 1] }

#[repr(C)]
pub struct ocfs2_info_fs_features {
    pub if_req: ocfs2_info_request,
    pub if_compat_features: u32,
    pub if_incompat_features: u32,
    pub if_ro_compat_features: u32,
    pub if_pad: u32,
}
#[repr(C)]
pub struct ocfs2_info_journal_size { pub ij_req: ocfs2_info_request, pub ij_journal_size: u64 }

#[repr(C)]
pub struct ocfs2_info_local_freeinode { pub lfi_total: u64, pub lfi_free: u64 }
#[repr(C)]
pub struct ocfs2_info_freeinode {
    pub ifi_req: ocfs2_info_request,
    pub ifi_stat: [ocfs2_info_local_freeinode; OCFS2_MAX_SLOTS],
    pub ifi_slotnum: u32,
    pub ifi_pad: u32,
}

pub const OCFS2_INFO_MAX_HIST: usize = 32;
#[repr(C)]
pub struct ocfs2_info_free_chunk_list { pub fc_chunks: [u32; OCFS2_INFO_MAX_HIST], pub fc_clusters: [u32; OCFS2_INFO_MAX_HIST] }
#[repr(C)]
pub struct ocfs2_info_freefrag_stats {
    pub ffs_fc_hist: ocfs2_info_free_chunk_list,
    pub ffs_clusters: u32, pub ffs_free_clusters: u32, pub ffs_free_chunks: u32,
    pub ffs_free_chunks_real: u32, pub ffs_min: u32, pub ffs_max: u32,
    pub ffs_avg: u32, pub ffs_pad: u32,
}
#[repr(C)]
pub struct ocfs2_info_freefrag { pub iff_req: ocfs2_info_request, pub iff_ffs: ocfs2_info_freefrag_stats, pub iff_chunksize: u32, pub iff_pad: u32 }

#[repr(i32)]
pub enum ocfs2_info_type {
    OCFS2_INFO_CLUSTERSIZE = 1,
    OCFS2_INFO_BLOCKSIZE,
    OCFS2_INFO_MAXSLOTS,
    OCFS2_INFO_LABEL,
    OCFS2_INFO_UUID,
    OCFS2_INFO_FS_FEATURES,
    OCFS2_INFO_JOURNAL_SIZE,
    OCFS2_INFO_FREEINODE,
    OCFS2_INFO_FREEFRAG,
    OCFS2_INFO_NUM_TYPES,
}

pub const OCFS2_INFO_FL_NON_COHERENT: u32 = 0x00000001;
pub const OCFS2_INFO_FL_FILLED: u32 = 0x40000000;
pub const OCFS2_INFO_FL_ERROR: u32 = 0x80000000;
pub const OCFS2_IOC_INFO: _ = _IOR(b'o', 5, core::mem::size_of::<ocfs2_info>());

#[repr(C)]
pub struct ocfs2_move_extents {
    pub me_start: u64, pub me_len: u64, pub me_goal: u64, pub me_threshold: u64,
    pub me_flags: u64, pub me_moved_len: u64, pub me_new_offset: u64,
    pub me_reserved: [u32; 2],
}
pub const OCFS2_MOVE_EXT_FL_AUTO_DEFRAG: u32 = 0x00000001;
pub const OCFS2_MOVE_EXT_FL_PART_DEFRAG: u32 = 0x00000002;
pub const OCFS2_MOVE_EXT_FL_COMPLETE: u32 = 0x00000004;
pub const OCFS2_IOC_MOVE_EXT: _ = _IOW(b'o', 6, core::mem::size_of::<ocfs2_move_extents>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
