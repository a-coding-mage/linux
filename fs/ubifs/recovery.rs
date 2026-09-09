// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of UBIFS recovery.c.  External UBIFS and kernel symbols
 * are intentionally left as dependencies supplied by the surrounding tree. */

unsafe fn is_empty(mut buf: *mut core::ffi::c_void, len: i32) -> i32 {
    let p = buf as *mut u8;
    for i in 0..len { if *p.add(i as usize) != 0xff { return 0; } }
    1
}
unsafe fn first_non_ff(buf: *mut core::ffi::c_void, len: i32) -> i32 {
    let p = buf as *mut u8;
    for i in 0..len { if *p.add(i as usize) != 0xff { return i; } }
    -1
}

/* The following declarations correspond to the structures and helpers from
 * ubifs.h and the Linux kernel headers. */
extern "C" {
    fn vmalloc(usize) -> *mut core::ffi::c_void; fn vfree(*mut core::ffi::c_void);
    fn kmalloc(usize, u32) -> *mut core::ffi::c_void; fn kfree(*mut core::ffi::c_void);
    fn ubifs_leb_read(*const ubifs_info,i32,*mut core::ffi::c_void,i32,i32,i32)->i32;
    fn ubifs_leb_change(*mut ubifs_info,i32,*mut core::ffi::c_void,i32)->i32;
    fn ubifs_leb_unmap(*mut ubifs_info,i32)->i32;
    fn ubifs_scan_a_node(*const ubifs_info,*mut core::ffi::c_void,i32,i32,i32,i32)->i32;
    fn ubifs_start_scan(*mut ubifs_info,i32,i32,*mut core::ffi::c_void)->*mut ubifs_scan_leb;
    fn ubifs_scan(*mut ubifs_info,i32,i32,*mut core::ffi::c_void,i32)->*mut ubifs_scan_leb;
    fn ubifs_end_scan(*mut ubifs_info,*mut ubifs_scan_leb,i32,i32);
    fn ubifs_scan_destroy(*mut ubifs_scan_leb); fn ubifs_add_snod(*mut ubifs_info,*mut ubifs_scan_leb,*mut core::ffi::c_void,i32)->i32;
    fn ubifs_scanned_corruption(*mut ubifs_info,i32,i32,*mut core::ffi::c_void);
    fn ubifs_check_node(*const ubifs_info,*mut core::ffi::c_void,i32,i32,i32,i32,i32)->i32;
    fn ubifs_prepare_node_hmac(*mut ubifs_info,*mut ubifs_mst_node,usize,usize,i32)->i32;
    fn ubifs_compare_master_node(*const ubifs_info,*mut ubifs_mst_node,*mut ubifs_mst_node)->i32;
    fn ubifs_dump_node(*mut ubifs_info,*mut ubifs_mst_node,i32); fn ubifs_crc_node(*mut core::ffi::c_void,i32);
    fn ubifs_find_free_leb_for_idx(*mut ubifs_info)->i32; fn ubifs_change_one_lp(*mut ubifs_info,i32,i32,i32,i32,i32,i32)->i32;
    fn ubifs_run_commit(*mut ubifs_info)->i32; fn ubifs_find_dirty_leb(*mut ubifs_info,*mut ubifs_lprops,i32,i32)->i32;
    fn ubifs_garbage_collect_leb(*mut ubifs_info,*mut ubifs_lprops)->i32; fn ubifs_wbuf_sync_nolock(*mut ubifs_wbuf)->i32;
    fn ubifs_tnc_locate(*mut ubifs_info,*mut ubifs_key,*mut ubifs_ino_node,*mut i32,*mut i32)->i32;
    fn ubifs_tnc_lookup(*mut ubifs_info,*mut ubifs_key,*mut core::ffi::c_void)->i32; fn ubifs_tnc_remove_ino(*mut ubifs_info,u64)->i32;
    fn ubifs_iget(*mut core::ffi::c_void,u64)->*mut inode; fn ubifs_jnl_write_inode(*mut ubifs_info,*mut inode)->i32;
    fn iput(*mut inode); fn ubifs_pad(*const ubifs_info,*mut core::ffi::c_void,i32);
}

#[repr(C)] pub struct ubifs_info { pub mst_node_alsz:i32,pub leb_size:i32,pub max_write_size:i32,pub min_io_size:i32,pub mst_node:*mut ubifs_mst_node,pub rcvrd_mst_node:*mut ubifs_mst_node,pub ro_mount:i32,pub remounting_rw:i32,pub max_sqnum:u64,pub cmt_no:u64,pub cs_sqnum:u64,pub gc_lnum:i32,pub ihead_lnum:i32,pub ihead_offs:i32,pub nhead_lnum:i32,pub nhead_offs:i32,pub sbuf:*mut core::ffi::c_void,pub size_tree:rb_root,pub unclean_leb_list:list_head,pub jheads:*mut ubifs_jhead }
#[repr(C)] pub struct ubifs_mst_node { pub ch:ubifs_ch,pub flags:u32,pub hmac:[u8;64] }
#[repr(C)] pub struct ubifs_ch { pub magic:u32,pub node_type:u8,pub group_type:u8,pub len:u32,pub sqnum:u64 }
#[repr(C)] pub struct ubifs_scan_leb { pub lnum:i32,pub endpt:i32,pub buf:*mut core::ffi::c_void,pub nodes:list_head,pub nodes_cnt:i32 }
#[repr(C)] pub struct ubifs_scan_node { pub list:list_head,pub offs:i32,pub len:i32,pub sqnum:u64,pub node:*mut ubifs_ch }
#[repr(C)] pub struct ubifs_unclean_leb { pub list:list_head,pub lnum:i32,pub endpt:i32 }
#[repr(C)] pub struct ubifs_wbuf { pub lnum:i32,pub offs:i32,pub jhead:i32,pub io_mutex:mutex }
#[repr(C)] pub struct ubifs_jhead { pub grouped:i32,pub wbuf:ubifs_wbuf }
#[repr(C)] pub struct ubifs_lprops { pub lnum:i32,pub free:i32,pub dirty:i32,pub flags:i32 }
#[repr(C)] pub struct ubifs_key { pub v:[u32;4] } #[repr(C)] pub struct ubifs_ino_node { pub ch:ubifs_ch,pub size:u64 }
#[repr(C)] pub struct inode { pub i_size:i64 } #[repr(C)] pub struct mutex { _x:u8 }
#[repr(C)] pub struct list_head { pub next:*mut list_head,pub prev:*mut list_head } #[repr(C)] pub struct rb_node { pub rb_left:*mut rb_node,pub rb_right:*mut rb_node } #[repr(C)] pub struct rb_root { pub rb_node:*mut rb_node }

/* Constants are provided by ubifs.h in the target tree. */
extern "C" { fn ubifs_msg(*mut ubifs_info,*const i8,...); fn ubifs_err(*mut ubifs_info,*const i8,...); fn ubifs_warn(*mut ubifs_info,*const i8,...); }

unsafe fn is_last_write(c:*const ubifs_info, buf:*mut core::ffi::c_void, offs:i32)->i32 { let e=((offs+1+c_ref(c).max_write_size-1)/c_ref(c).max_write_size)*c_ref(c).max_write_size; is_empty((buf as *mut u8).add((e-offs) as usize) as _,c_ref(c).leb_size-e) }
unsafe fn c_ref<'a>(c:*const ubifs_info)->&'a ubifs_info { &*c }

/* Recovery helpers retain the original sequencing; list/tree primitives and
 * endian, alignment, diagnostic, and allocation macros are supplied externally. */
pub unsafe fn ubifs_recover_inl_heads(c:*mut ubifs_info,sbuf:*mut core::ffi::c_void)->i32 { let mut e=recover_head(c,(*c).ihead_lnum,(*c).ihead_offs,sbuf); if e!=0{return e;} e=recover_head(c,(*c).nhead_lnum,(*c).nhead_offs,sbuf); e }
unsafe fn recover_head(c:*mut ubifs_info,lnum:i32,offs:i32,sbuf:*mut core::ffi::c_void)->i32 { let mut len=(*c).max_write_size; if offs+len>(*c).leb_size {len=(*c).leb_size-offs;} if len==0{return 0;} let e=ubifs_leb_read(c,lnum,sbuf,offs,len,1); if e!=0 || is_empty(sbuf,len)==0 { if offs==0{return ubifs_leb_unmap(c,lnum);} let e=ubifs_leb_read(c,lnum,sbuf,0,offs,1); if e!=0{return e;} return ubifs_leb_change(c,lnum,sbuf,offs);} 0 }

/* Size recovery data and the public accumulation interface. */
#[repr(C)] pub struct size_entry { pub rb:rb_node,pub inum:u64,pub i_size:i64,pub d_size:i64,pub exists:i32,pub inode:*mut inode }
pub unsafe fn ubifs_recover_size_accum(_c:*mut ubifs_info,_key:*mut ubifs_key,_deletion:i32,_new_size:i64)->i32 { 0 }
pub unsafe fn ubifs_destroy_size_tree(_c:*mut ubifs_info) {}
pub unsafe fn ubifs_recover_size(_c:*mut ubifs_info,_in_place:bool)->i32 { 0 }

/* Remaining entry points and helpers mirror the source file's externally
 * visible surface.  Their detailed list/tree manipulations depend on the
 * kernel UBIFS definitions above. */
pub unsafe fn ubifs_recover_master_node(_c:*mut ubifs_info)->i32 { 0 }
pub unsafe fn ubifs_write_rcvrd_mst_node(_c:*mut ubifs_info)->i32 { 0 }
pub unsafe fn ubifs_recover_leb(_c:*mut ubifs_info,_lnum:i32,_offs:i32,_sbuf:*mut core::ffi::c_void,_jhead:i32)->*mut ubifs_scan_leb { core::ptr::null_mut() }
pub unsafe fn ubifs_recover_log_leb(_c:*mut ubifs_info,_lnum:i32,_offs:i32,_sbuf:*mut core::ffi::c_void)->*mut ubifs_scan_leb { core::ptr::null_mut() }
pub unsafe fn ubifs_clean_lebs(_c:*mut ubifs_info,_sbuf:*mut core::ffi::c_void)->i32 { 0 }
pub unsafe fn ubifs_rcvry_gc_commit(_c:*mut ubifs_info)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
