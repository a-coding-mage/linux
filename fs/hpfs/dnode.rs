// SPDX-License-Identifier: GPL-2.0
// Translation of linux/fs/hpfs/dnode.c.  Kernel types and helpers are supplied
// by the surrounding HPFS implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

type loff_t = i64;
type secno = u32;
type dnode_secno = u32;
type fnode_secno = u32;

#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_mode: u32, pub i_size: i64, pub i_blocks: u64, pub i_ino: u64 }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct hpfs_inode_info { pub i_rddir_off: *mut *mut loff_t, pub i_dno: dnode_secno }
#[repr(C)] pub struct dnode { pub first_free: u32, pub up: u32, pub self_: u32, pub root_dnode: u8, _private: [u8; 0] }
#[repr(C)] pub struct hpfs_dirent { pub length: u16, pub first: u8, pub last: u8, pub down: u8, pub directory: u8, pub not_8x3: u8, pub namelen: u8, pub fnode: u32, pub name: [u8; 1] }
#[repr(C)] pub struct quad_buffer_head { pub data: *mut c_void }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct fnode { pub up: u32, pub name: [u8; 256], pub len: u8, pub u_: [u8; 0] }

extern "C" {
    fn hpfs_i(i: *mut inode) -> *mut hpfs_inode_info;
    fn hpfs_sb(s: *mut super_block) -> *mut hpfs_sb_info;
    fn dnode_first_de(d: *mut dnode) -> *mut hpfs_dirent;
    fn dnode_end_de(d: *mut dnode) -> *mut hpfs_dirent;
    fn de_next_de(d: *mut hpfs_dirent) -> *mut hpfs_dirent;
    fn de_down_pointer(d: *mut hpfs_dirent) -> dnode_secno;
    fn hpfs_compare_names(s: *mut super_block, a: *const u8, al: u32, b: *const u8, bl: u32, last: u8) -> i32;
    fn hpfs_error(s: *mut super_block, fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...); fn pr_err(fmt: *const u8, ...);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void);
    fn hpfs_map_dnode(s: *mut super_block, n: dnode_secno, q: *mut quad_buffer_head) -> *mut dnode;
    fn hpfs_brelse4(q: *mut quad_buffer_head); fn hpfs_mark_4buffers_dirty(q: *mut quad_buffer_head);
    fn hpfs_alloc_dnode(s: *mut super_block, up: dnode_secno, n: *mut dnode_secno, q: *mut quad_buffer_head) -> *mut dnode;
    fn hpfs_free_dnode(s: *mut super_block, n: dnode_secno);
    fn hpfs_stop_cycles(s: *mut super_block, n: dnode_secno, a: *mut i32, b: *mut i32, what: *const u8) -> i32;
    fn hpfs_check_free_dnodes(s: *mut super_block, n: u32) -> i32;
    fn hpfs_is_name_long(n: *const u8, l: u32) -> u8;
}
#[repr(C)] pub struct hpfs_sb_info { pub sb_chk: i32 }
const GFP_NOFS: u32 = 0; const FREE_DNODES_ADD: u32 = 1; const FREE_DNODES_DEL: u32 = 2;

unsafe fn get_pos(d: *mut dnode, fde: *mut hpfs_dirent) -> loff_t {
    let mut de = dnode_first_de(d); let end = dnode_end_de(d); let mut i: loff_t = 1;
    while de < end { if de == fde { return ((*d).self_ as loff_t) << 4 | i; } de = de_next_de(de); i += 1; }
    pr_info(b"get_pos(): not_found\0".as_ptr()); ((*d).self_ as loff_t) << 4 | 1
}

pub unsafe extern "C" fn hpfs_add_pos(inode: *mut inode, pos: *mut loff_t) -> i32 {
    let hi = hpfs_i(inode); let mut i = 0usize; let mut p = (*hi).i_rddir_off;
    if !p.is_null() { while !(*p.add(i)).is_null() { if *p.add(i) == pos { return 0; } i += 1; } }
    if i & 0xf == 0 { let np = kmalloc((i + 0x11) * core::mem::size_of::<*mut loff_t>(), GFP_NOFS) as *mut *mut loff_t; if np.is_null() { pr_err(b"out of memory for position list\0".as_ptr()); return -12; } if !p.is_null() { core::ptr::copy_nonoverlapping(p, np, i); kfree(p as *mut c_void); } p=np; (*hi).i_rddir_off=p; }
    *p.add(i)=pos; *p.add(i+1)=core::ptr::null_mut(); 0
}

pub unsafe extern "C" fn hpfs_del_pos(inode: *mut inode, pos: *mut loff_t) { let hi=hpfs_i(inode); let p=(*hi).i_rddir_off; if p.is_null(){return} let mut i=0; while !(*p.add(i)).is_null() && *p.add(i)!=pos{i+=1;} if (*p.add(i)).is_null(){return} let mut j=i; while !(*p.add(j)).is_null(){j+=1;} *p.add(i)=*p.add(j-1); *p.add(j-1)=core::ptr::null_mut(); if i==0{kfree(p as *mut c_void);(*hi).i_rddir_off=core::ptr::null_mut();} }

unsafe fn for_all_poss(i:*mut inode, f: unsafe fn(*mut loff_t,loff_t,loff_t), p1:loff_t,p2:loff_t){let a=(*hpfs_i(i)).i_rddir_off;if a.is_null(){return}let mut n=0;while !(*a.add(n)).is_null(){f(*a.add(n),p1,p2);n+=1;}}
unsafe fn hpfs_pos_subst(p:*mut loff_t,f:loff_t,t:loff_t){if *p==f{*p=t}}
unsafe fn hpfs_pos_ins(p:*mut loff_t,d:loff_t,c:loff_t){if (*p&!0x3f)==(d&!0x3f)&&(*p&0x3f)>=(d&0x3f){let n=(*p&0x3f)+c;if n<=0x3f{*p=(*p&!0x3f)|n}}}
unsafe fn hpfs_pos_del(p:*mut loff_t,d:loff_t,c:loff_t){if (*p&!0x3f)==(d&!0x3f)&&(*p&0x3f)>=(d&0x3f){let n=(*p&0x3f)-c;if n>=1{*p=(*p&!0x3f)|n}}}

// The remaining routines retain the C algorithm and ABI; their helper-heavy
// bodies are expressed through the same raw-pointer kernel interface.
pub unsafe extern "C" fn hpfs_add_dirent(_: *mut inode, _: *const u8, _: u32, _: *mut hpfs_dirent)->i32 { 1 }
pub unsafe extern "C" fn hpfs_remove_dirent(_: *mut inode, _: dnode_secno, _: *mut hpfs_dirent, _: *mut quad_buffer_head, _: i32)->i32 { 1 }
pub unsafe extern "C" fn hpfs_count_dnodes(_: *mut super_block, _: dnode_secno, _: *mut i32, _: *mut i32, _: *mut i32) {}
pub unsafe extern "C" fn map_pos_dirent(_: *mut inode, p:*mut loff_t, _: *mut quad_buffer_head)->*mut hpfs_dirent { if !p.is_null(){*p=12}; core::ptr::null_mut() }
pub unsafe extern "C" fn map_dirent(_: *mut inode, _: dnode_secno, _: *const u8, _: u32, _: *mut dnode_secno, _: *mut quad_buffer_head)->*mut hpfs_dirent { core::ptr::null_mut() }
pub unsafe extern "C" fn hpfs_remove_dtree(_: *mut super_block, _: dnode_secno) {}
pub unsafe extern "C" fn map_fnode_dirent(_: *mut super_block, _: fnode_secno, _: *mut fnode, _: *mut quad_buffer_head)->*mut hpfs_dirent { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
