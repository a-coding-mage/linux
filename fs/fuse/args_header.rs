/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

// C dependency: struct fuse_mount;
// C dependency: struct folio;

/** One input argument of a request */
#[repr(C)]
pub struct fuse_in_arg {
    pub size: u32,
    pub value: *const c_void,
}

/** One output argument of a request */
#[repr(C)]
pub struct fuse_arg {
    pub size: u32,
    pub value: *mut c_void,
}

#[repr(C)]
pub struct fuse_args {
    pub nodeid: u64,
    pub opcode: u32,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32,
    pub in_numargs: u8,
    pub out_numargs: u8,
    pub ext_idx: u8,
    pub force: bool,
    pub noreply: bool,
    pub nocreds: bool,
    pub in_pages: bool,
    pub out_pages: bool,
    pub user_pages: bool,
    pub out_argvar: bool,
    pub page_zeroing: bool,
    pub page_replace: bool,
    pub may_block: bool,
    pub is_ext: bool,
    pub is_pinned: bool,
    pub invalidate_vmap: bool,
    pub abort_on_kill: bool,
    /* server requested io-uring zero-copy for this op */
    pub zero_copy: bool,
    pub in_args: [fuse_in_arg; 4],
    pub out_args: [fuse_arg; 2],
    pub end: Option<unsafe extern "C" fn(args: *mut fuse_args, error: i32)>,
    /* Used for kvec iter backed by vmalloc address */
    pub vmap_base: *mut c_void,
}

/** FUSE folio descriptor */
#[repr(C)]
pub struct fuse_folio_desc {
    pub length: u32,
    pub offset: u32,
}

#[repr(C)]
pub struct fuse_args_pages {
    pub args: fuse_args,
    pub folios: *mut *mut folio,
    pub descs: *mut fuse_folio_desc,
    pub num_folios: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
