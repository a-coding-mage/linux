/* SPDX-License-Identifier: GPL-2.0 */

// Translation of trace/events/fsverity.h.
// The Linux tracepoint machinery supplied by the included headers expands the
// declarations below into trace events; those dependencies remain external.

#[repr(C)]
pub struct fsverity_descriptor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct merkle_tree_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsverity_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FsverityEnableEntry {
    pub ino: u64,
    pub data_size: u64,
    pub tree_size: u64,
    pub merkle_block: ::core::ffi::c_uint,
    pub num_levels: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct FsverityTreeDoneEntry {
    pub ino: u64,
    pub data_size: u64,
    pub tree_size: u64,
    pub merkle_block: ::core::ffi::c_uint,
    pub levels: ::core::ffi::c_uint,
    pub root_hash: *mut u8,
    pub root_hash_len: usize,
    pub file_digest: *mut u8,
    pub file_digest_len: usize,
}

#[repr(C)]
pub struct FsverityVerifyDataBlockEntry {
    pub ino: u64,
    pub data_pos: u64,
    pub merkle_block: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct FsverityMerkleHitEntry {
    pub ino: u64,
    pub data_pos: u64,
    pub hblock_idx: ::core::ffi::c_ulong,
    pub level: ::core::ffi::c_uint,
    pub hidx: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct FsverityVerifyMerkleBlockEntry {
    pub ino: u64,
    pub hblock_idx: ::core::ffi::c_ulong,
    pub level: ::core::ffi::c_uint,
    pub hidx: ::core::ffi::c_uint,
}

// TRACE_EVENT(fsverity_enable,
//     TP_PROTO(const struct inode *inode,
//              const struct merkle_tree_params *params),
//     TP_ARGS(inode, params),
//     TP_fast_assign(__entry->ino = inode->i_ino;
//         __entry->data_size = i_size_read(inode);
//         __entry->tree_size = params->tree_size;
//         __entry->merkle_block = params->block_size;
//         __entry->num_levels = params->num_levels;),
//     TP_printk("ino %llu data_size %llu tree_size %llu merkle_block %u levels %u", ...))

// TRACE_EVENT(fsverity_tree_done,
//     TP_PROTO(const struct inode *inode, const struct fsverity_info *vi,
//              const struct merkle_tree_params *params),
//     TP_ARGS(inode, vi, params),
//     TP_fast_assign(__entry->ino = inode->i_ino;
//         __entry->data_size = i_size_read(inode);
//         __entry->tree_size = params->tree_size;
//         __entry->merkle_block = params->block_size;
//         __entry->levels = params->num_levels;
//         memcpy(__get_dynamic_array(root_hash), vi->root_hash,
//                __get_dynamic_array_len(root_hash));
//         memcpy(__get_dynamic_array(file_digest), vi->file_digest,
//                __get_dynamic_array_len(file_digest));),
//     TP_printk("ino %llu data_size %llu tree_size %lld merkle_block %u levels %u root_hash %s digest %s", ...))

// TRACE_EVENT(fsverity_verify_data_block,
//     TP_PROTO(const struct inode *inode,
//              const struct merkle_tree_params *params, u64 data_pos),
//     TP_ARGS(inode, params, data_pos),
//     TP_fast_assign(__entry->ino = inode->i_ino;
//         __entry->data_pos = data_pos;
//         __entry->merkle_block = params->block_size;),
//     TP_printk("ino %llu data_pos %llu merkle_block %u", ...))

// TRACE_EVENT(fsverity_merkle_hit,
//     TP_PROTO(const struct inode *inode, u64 data_pos,
//              unsigned long hblock_idx, unsigned int level,
//              unsigned int hidx),
//     TP_ARGS(inode, data_pos, hblock_idx, level, hidx),
//     TP_fast_assign(__entry->ino = inode->i_ino;
//         __entry->data_pos = data_pos;
//         __entry->hblock_idx = hblock_idx;
//         __entry->level = level;
//         __entry->hidx = hidx;),
//     TP_printk("ino %llu data_pos %llu hblock_idx %lu level %u hidx %u", ...))

// TRACE_EVENT(fsverity_verify_merkle_block,
//     TP_PROTO(const struct inode *inode, unsigned long hblock_idx,
//              unsigned int level, unsigned int hidx),
//     TP_ARGS(inode, hblock_idx, level, hidx),
//     TP_fast_assign(__entry->ino = inode->i_ino;
//         __entry->hblock_idx = hblock_idx;
//         __entry->level = level;
//         __entry->hidx = hidx;),
//     TP_printk("ino %llu hblock_idx %lu level %u hidx %u", ...))

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
