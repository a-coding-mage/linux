/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Transparent compression codec interface.
 *
 * Copyright (c) 2026 LG Electronics Co., Ltd.
 */

// C dependencies: <linux/types.h>, <linux/fs.h>, and <linux/mm.h>.

#[repr(C)]
pub struct compress_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ntfs_codec_id {
    NTFS_CODEC_LZNT1,
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    NTFS_CODEC_XPRESS4K,
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    NTFS_CODEC_XPRESS8K,
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    NTFS_CODEC_XPRESS16K,
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    NTFS_CODEC_LZX32K,
}

#[repr(C)]
pub struct ntfs_codec_ops {
    pub id: ntfs_codec_id,
    pub name: *const ::kernel::ffi::c_char,
    pub scratch_size: Option<unsafe extern "C" fn(chunk_size: u32) -> usize>,
    pub decompress_chunk: Option<
        unsafe extern "C" fn(
            scratch: *mut ::kernel::ffi::c_void,
            src: *const ::kernel::ffi::c_void,
            src_len: usize,
            dst: *mut ::kernel::ffi::c_void,
            dst_len: usize,
            chunk_size: u32,
        ) -> ::kernel::ffi::c_int,
    >,
    pub decompress_pages: Option<
        unsafe extern "C" fn(
            dest_pages: *mut *mut page,
            completed_pages: *mut ::kernel::ffi::c_int,
            dest_index: *mut ::kernel::ffi::c_int,
            dest_ofs: *mut ::kernel::ffi::c_int,
            dest_max_index: ::kernel::ffi::c_int,
            dest_max_ofs: ::kernel::ffi::c_int,
            xpage: ::kernel::ffi::c_int,
            xpage_done: *mut ::kernel::ffi::c_char,
            cb_start: *mut u8,
            cb_size: u32,
            i_size: i64,
            initialized_size: i64,
        ) -> ::kernel::ffi::c_int,
    >,
    pub compress_subblock: Option<
        unsafe extern "C" fn(
            pctx: *mut compress_context,
            inbuf: *const ::kernel::ffi::c_char,
            bufsize: ::kernel::ffi::c_int,
            outbuf: *mut ::kernel::ffi::c_char,
        ) -> ::kernel::ffi::c_int,
    >,
}

unsafe extern "C" {
    pub static ntfs_lznt1_codec_ops: ntfs_codec_ops;

    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    pub static ntfs_xpress4k_codec_ops: ntfs_codec_ops;
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    pub static ntfs_xpress8k_codec_ops: ntfs_codec_ops;
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    pub static ntfs_xpress16k_codec_ops: ntfs_codec_ops;
    #[cfg(CONFIG_NTFS_FS_WOF_COMPRESSION)]
    pub static ntfs_lzx32k_codec_ops: ntfs_codec_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
