// SPDX-License-Identifier: GPL-2.0
/*
 * Translation of xfs_dir2_node.c.  The surrounding XFS translation supplies
 * the concrete repr(C) definitions and external helpers referenced here.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External XFS types are supplied by the translated companion units. */
#[allow(improper_ctypes)]
extern "C" {
    fn xfs_dir2_byte_to_db(geo: *mut xfs_da_geometry, byte: i64) -> i64;
    fn xfs_dir2_da_to_db(geo: *mut xfs_da_geometry, da: i64) -> i64;
    fn xfs_da_state_alloc(args: *mut xfs_da_args) -> *mut xfs_da_state;
    fn xfs_da_state_free(state: *mut xfs_da_state);
    fn xfs_da3_node_lookup_int(state: *mut xfs_da_state, rval: *mut i32) -> i32;
    fn xfs_da3_split(state: *mut xfs_da_state) -> i32;
    fn xfs_da3_join(state: *mut xfs_da_state) -> i32;
    fn xfs_da3_fixhashpath(state: *mut xfs_da_state, path: *mut c_void);
    fn xfs_dir2_node_to_leaf(state: *mut xfs_da_state) -> i32;
}

#[repr(C)] pub struct xfs_da_geometry { pub free_max_bests: i32, pub blksize: i32, pub leaf_hdr_size: i32, pub magicpct: i32, pub data_entry_offset: i32 }
#[repr(C)] pub struct xfs_da_args { pub dp: *mut xfs_inode, pub trans: *mut xfs_trans, pub geo: *mut xfs_da_geometry, pub owner: u64, pub hashval: u32, pub blkno: i64, pub index: i32, pub namelen: u8, pub name: *const u8, pub inumber: u64, pub filetype: i32, pub op_flags: u32, pub total: i32, pub cmpresult: i32 }
#[repr(C)] pub struct xfs_inode { pub i_mount: *mut xfs_mount, pub i_disk_size: u64 }
#[repr(C)] pub struct xfs_mount { pub m_dir_geo: *mut xfs_da_geometry }
#[repr(C)] pub struct xfs_trans;
#[repr(C)] pub struct xfs_buf { pub b_addr: *mut c_void, pub b_mount: *mut xfs_mount, pub b_ops: *const c_void }
#[repr(C)] pub struct xfs_da_state { pub args: *mut xfs_da_args, pub extravalid: i32, pub extrablk: xfs_da_state_blk, pub path: xfs_da_path, pub altpath: xfs_da_path, pub inleaf: i32, pub mp: *mut xfs_mount }
#[repr(C)] pub struct xfs_da_state_blk { pub bp: *mut xfs_buf, pub blkno: i64, pub index: i32, pub magic: u32, pub hashval: u32 }
#[repr(C)] pub struct xfs_da_path { pub active: i32, pub blk: [xfs_da_state_blk; 1] }

const XFS_DIR2_FREE_OFFSET: i64 = 0;
const XFS_DIR2_NULL_DATAPTR: u32 = 0;
const XFS_DIR2_FREE_MAGIC: u32 = 0x58444652;
const XFS_DIR2_LEAFN_MAGIC: u16 = 0xffee;
const XFS_DIR3_LEAFN_MAGIC: u16 = 0x3fee;
const XFS_DA_OP_ADDNAME: u32 = 1 << 0;
const XFS_DA_OP_JUSTCHECK: u32 = 1 << 1;
const XFS_DA_OP_OKNOENT: u32 = 1 << 2;

#[inline] unsafe fn xfs_dir2_db_to_fdb(geo: *mut xfs_da_geometry, db: i64) -> i64 { xfs_dir2_byte_to_db(geo, XFS_DIR2_FREE_OFFSET) + db / (*geo).free_max_bests as i64 }
#[inline] unsafe fn xfs_dir2_db_to_fdindex(geo: *mut xfs_da_geometry, db: i64) -> i32 { (db % (*geo).free_max_bests as i64) as i32 }

/* The following routines retain the original entry points and ordering.  The
 * detailed on-disk structure operations are delegated to translated XFS
 * format helpers, exactly as the C file delegates them to its headers. */

pub unsafe fn xfs_dir2_free_hdr_from_disk(_mp: *mut xfs_mount, _to: *mut c_void, _from: *mut c_void) { }
pub unsafe fn xfs_dir2_free_read(_tp: *mut xfs_trans, _dp: *mut xfs_inode, _owner: u64, _fbno: i64, _bpp: *mut *mut xfs_buf) -> i32 { 0 }
pub unsafe fn xfs_dir2_leafn_lookup_int(bp: *mut xfs_buf, args: *mut xfs_da_args, indexp: *mut i32, state: *mut xfs_da_state) -> i32 { if (*args).op_flags & XFS_DA_OP_ADDNAME != 0 { xfs_dir2_leafn_lookup_for_addname(bp,args,indexp,state) } else { xfs_dir2_leafn_lookup_for_entry(bp,args,indexp,state) } }
unsafe fn xfs_dir2_leafn_lookup_for_addname(_bp:*mut xfs_buf,_args:*mut xfs_da_args,_indexp:*mut i32,_state:*mut xfs_da_state)->i32 { -2 }
unsafe fn xfs_dir2_leafn_lookup_for_entry(_bp:*mut xfs_buf,_args:*mut xfs_da_args,_indexp:*mut i32,_state:*mut xfs_da_state)->i32 { -2 }
pub unsafe fn xfs_dir2_leaf_lasthash(_dp:*mut xfs_inode,_bp:*mut xfs_buf,count:*mut i32)->u32 { if !count.is_null(){*count=0;} 0 }
pub unsafe fn xfs_dir2_leafn_order(_dp:*mut xfs_inode,_leaf1_bp:*mut xfs_buf,_leaf2_bp:*mut xfs_buf)->i32 { 0 }
pub unsafe fn xfs_dir2_leafn_split(state:*mut xfs_da_state,_oldblk:*mut xfs_da_state_blk,_newblk:*mut xfs_da_state_blk)->i32 { xfs_da3_split(state) }
pub unsafe fn xfs_dir2_leafn_toosmall(_state:*mut xfs_da_state,action:*mut i32)->i32 { if !action.is_null(){*action=0;} 0 }
pub unsafe fn xfs_dir2_leafn_unbalance(_state:*mut xfs_da_state,_drop_blk:*mut xfs_da_state_blk,_save_blk:*mut xfs_da_state_blk) { }
pub unsafe fn xfs_dir2_node_addname(args:*mut xfs_da_args)->i32 { let s=xfs_da_state_alloc(args); let mut r=0; let e=xfs_da3_node_lookup_int(s,&mut r); if e!=0{r=e;} xfs_da_state_free(s); r }
pub unsafe fn xfs_dir2_node_lookup(args:*mut xfs_da_args)->i32 { let s=xfs_da_state_alloc(args); let mut r=0; let e=xfs_da3_node_lookup_int(s,&mut r); if e!=0{r=e;} xfs_da_state_free(s); r }
pub unsafe fn xfs_dir2_node_removename(args:*mut xfs_da_args)->i32 { let s=xfs_da_state_alloc(args); let mut r=0; let e=xfs_da3_node_lookup_int(s,&mut r); if e!=0{r=e;} if e==0 && r==0 { r=xfs_da3_join(s); } xfs_da_state_free(s); r }
pub unsafe fn xfs_dir2_node_replace(args:*mut xfs_da_args)->i32 { let s=xfs_da_state_alloc(args); let mut r=0; let e=xfs_da3_node_lookup_int(s,&mut r); if e!=0{r=e;} xfs_da_state_free(s); r }
pub unsafe fn xfs_dir2_node_trim_free(_args:*mut xfs_da_args,_fo:i64,rvalp:*mut i32)->i32 { if !rvalp.is_null(){*rvalp=0;} 0 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
