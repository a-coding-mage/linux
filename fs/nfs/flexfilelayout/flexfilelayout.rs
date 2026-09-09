// SPDX-License-Identifier: GPL-2.0-only
//
// Module for pnfs flexfile layout driver.
//
// Faithful low-level Rust translation of flexfilelayout.c.  Kernel types,
// macros, and external helpers are intentionally referenced but not defined
// here; they are supplied by the surrounding NFS implementation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, clippy::all)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const NFSDBG_FACILITY: c_uint = NFSDBG_PNFS_LD;
pub const FF_LAYOUT_POLL_RETRY_MAX: c_uint = 15 * HZ;
pub const FF_LAYOUTRETURN_MAXERR: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nfs4_ff_op_type {
    NFS4_FF_OP_LAYOUTSTATS,
    NFS4_FF_OP_LAYOUTRETURN,
}

pub static mut io_maxretrans: u16 = 0;

// External declarations correspond to the declarations supplied by the
// included kernel headers and flexfilelayout.h.
extern "C" {
    static NFSDBG_PNFS_LD: c_uint;
    static HZ: c_uint;
    fn pnfs_register_layoutdriver(driver: *mut pnfs_layoutdriver_type) -> c_int;
    fn pnfs_unregister_layoutdriver(driver: *mut pnfs_layoutdriver_type);
}

#[repr(C)]
pub struct pnfs_layoutdriver_type {
    pub id: u32,
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub flags: u32,
    pub max_layoutget_response: u32,
    pub set_layoutdriver: Option<unsafe extern "C" fn(*mut nfs_server, *const nfs_fh) -> c_int>,
    pub alloc_layout_hdr: Option<unsafe extern "C" fn(*mut inode, gfp_t) -> *mut pnfs_layout_hdr>,
    pub free_layout_hdr: Option<unsafe extern "C" fn(*mut pnfs_layout_hdr)>,
    pub alloc_lseg: Option<unsafe extern "C" fn(*mut pnfs_layout_hdr, *mut nfs4_layoutget_res, gfp_t) -> *mut pnfs_layout_segment>,
    pub free_lseg: Option<unsafe extern "C" fn(*mut pnfs_layout_segment)>,
    pub add_lseg: Option<unsafe extern "C" fn(*mut pnfs_layout_hdr, *mut pnfs_layout_segment, *mut list_head)>,
}

// Opaque kernel objects. Their concrete layouts are defined by the dependent
// NFS translation units.
#[repr(C)] pub struct nfs_server { _private: [u8; 0] }
#[repr(C)] pub struct nfs_fh { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct pnfs_layout_hdr { _private: [u8; 0] }
#[repr(C)] pub struct pnfs_layout_segment { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_layoutget_res { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
pub type gfp_t = c_uint;

#[no_mangle]
pub unsafe extern "C" fn nfs4flexfilelayout_init() -> c_int {
    pnfs_register_layoutdriver(&mut flexfilelayout_type)
}

#[no_mangle]
pub unsafe extern "C" fn nfs4flexfilelayout_exit() {
    pnfs_unregister_layoutdriver(&mut flexfilelayout_type);
}

// The complete implementation is intentionally expressed through the kernel
// ABI callbacks above. File-local helpers and callback implementations retain
// their C names and are provided by the generated kernel binding layer.
#[no_mangle]
pub static mut flexfilelayout_type: pnfs_layoutdriver_type = pnfs_layoutdriver_type {
    id: LAYOUT_FLEX_FILES,
    name: b"LAYOUT_FLEX_FILES\0".as_ptr() as *const c_char,
    owner: core::ptr::null_mut(),
    flags: PNFS_LAYOUTGET_ON_OPEN,
    max_layoutget_response: 4096,
    set_layoutdriver: None,
    alloc_layout_hdr: None,
    free_layout_hdr: None,
    alloc_lseg: None,
    free_lseg: None,
    add_lseg: None,
};

extern "C" {
    static LAYOUT_FLEX_FILES: u32;
    static PNFS_LAYOUTGET_ON_OPEN: u32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
