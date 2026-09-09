/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2004   Ferenc Havasi <havasi@inf.u-szeged.hu>,
 *		      University of Szeged, Hungary
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// C header guard: __JFFS2_COMPR_H__
// Dependencies supplied by the surrounding translation unit:
// linux/kernel.h, linux/vmalloc.h, linux/list.h, linux/types.h,
// linux/string.h, linux/slab.h, linux/errno.h, linux/fs.h, linux/jffs2.h,
// jffs2_fs_i.h, jffs2_fs_sb.h, and nodelist.h.

pub const JFFS2_RUBINMIPS_PRIORITY: i32 = 10;
pub const JFFS2_DYNRUBIN_PRIORITY: i32 = 20;
pub const JFFS2_LZARI_PRIORITY: i32 = 30;
pub const JFFS2_RTIME_PRIORITY: i32 = 50;
pub const JFFS2_ZLIB_PRIORITY: i32 = 60;
pub const JFFS2_LZO_PRIORITY: i32 = 80;

// JFFS2_RUBINMIPS_DISABLED: RUBINs will be used only for decompression.
// JFFS2_DYNRUBIN_DISABLED: RUBINs will be used only for decompression.

pub const JFFS2_COMPR_MODE_NONE: i32 = 0;
pub const JFFS2_COMPR_MODE_PRIORITY: i32 = 1;
pub const JFFS2_COMPR_MODE_SIZE: i32 = 2;
pub const JFFS2_COMPR_MODE_FAVOURLZO: i32 = 3;
pub const JFFS2_COMPR_MODE_FORCELZO: i32 = 4;
pub const JFFS2_COMPR_MODE_FORCEZLIB: i32 = 5;

pub const FAVOUR_LZO_PERCENT: i32 = 80;

#[repr(C)]
pub struct jffs2_compressor {
    pub list: crate::list_head,
    pub priority: i32, // used by priority compression mode
    pub name: *mut i8,
    pub compr: i8, // JFFS2_COMPR_XXX
    pub compress: Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u32, *mut u32) -> i32>,
    pub decompress: Option<unsafe extern "C" fn(*mut u8, *mut u8, u32, u32) -> i32>,
    pub usecount: i32,
    pub disabled: i32, // if set the compressor won't compress
    pub compr_buf: *mut u8, // used by size compression mode
    pub compr_buf_size: u32, // used by size compression mode
    pub stat_compr_orig_size: u32,
    pub stat_compr_new_size: u32,
    pub stat_compr_blocks: u32,
    pub stat_decompr_blocks: u32,
}

extern "C" {
    pub fn jffs2_register_compressor(comp: *mut jffs2_compressor) -> i32;
    pub fn jffs2_unregister_compressor(comp: *mut jffs2_compressor) -> i32;

    pub fn jffs2_compressors_init() -> i32;
    pub fn jffs2_compressors_exit() -> i32;

    pub fn jffs2_compress(
        c: *mut crate::jffs2_sb_info,
        f: *mut crate::jffs2_inode_info,
        data_in: *mut u8,
        cpage_out: *mut *mut u8,
        datalen: *mut u32,
        cdatalen: *mut u32,
    ) -> u16;

    pub fn jffs2_decompress(
        c: *mut crate::jffs2_sb_info,
        f: *mut crate::jffs2_inode_info,
        comprtype: u16,
        cdata_in: *mut u8,
        data_out: *mut u8,
        cdatalen: u32,
        datalen: u32,
    ) -> i32;

    pub fn jffs2_free_comprbuf(comprbuf: *mut u8, orig: *mut u8);
}

// Compressor modules. These functions are called by jffs2_compressors_init/exit.

// CONFIG_JFFS2_RUBIN controls whether the external declarations are available.
#[cfg(feature = "CONFIG_JFFS2_RUBIN")]
extern "C" {
    pub fn jffs2_rubinmips_init() -> i32;
    pub fn jffs2_rubinmips_exit();
    pub fn jffs2_dynrubin_init() -> i32;
    pub fn jffs2_dynrubin_exit();
}
#[cfg(not(feature = "CONFIG_JFFS2_RUBIN"))]
pub unsafe fn jffs2_rubinmips_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_RUBIN"))]
pub unsafe fn jffs2_rubinmips_exit() {}
#[cfg(not(feature = "CONFIG_JFFS2_RUBIN"))]
pub unsafe fn jffs2_dynrubin_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_RUBIN"))]
pub unsafe fn jffs2_dynrubin_exit() {}

#[cfg(feature = "CONFIG_JFFS2_RTIME")]
extern "C" { pub fn jffs2_rtime_init() -> i32; pub fn jffs2_rtime_exit(); }
#[cfg(not(feature = "CONFIG_JFFS2_RTIME"))]
pub unsafe fn jffs2_rtime_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_RTIME"))]
pub unsafe fn jffs2_rtime_exit() {}

#[cfg(feature = "CONFIG_JFFS2_ZLIB")]
extern "C" { pub fn jffs2_zlib_init() -> i32; pub fn jffs2_zlib_exit(); }
#[cfg(not(feature = "CONFIG_JFFS2_ZLIB"))]
pub unsafe fn jffs2_zlib_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_ZLIB"))]
pub unsafe fn jffs2_zlib_exit() {}

#[cfg(feature = "CONFIG_JFFS2_LZO")]
extern "C" { pub fn jffs2_lzo_init() -> i32; pub fn jffs2_lzo_exit(); }
#[cfg(not(feature = "CONFIG_JFFS2_LZO"))]
pub unsafe fn jffs2_lzo_init() -> i32 { 0 }
#[cfg(not(feature = "CONFIG_JFFS2_LZO"))]
pub unsafe fn jffs2_lzo_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
