// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of UBIFS log.c. External kernel and UBIFS symbols are
 * intentionally referenced as supplied by the surrounding repository. */

use core::ptr;

extern "C" {
    fn spin_lock(x: *mut spinlock_t); fn spin_unlock(x: *mut spinlock_t);
    fn mutex_lock(x: *mut mutex_t); fn mutex_unlock(x: *mut mutex_t);
    fn rb_first(x: *mut rb_root) -> *mut rb_node; fn rb_next(x: *mut rb_node) -> *mut rb_node;
    fn rb_link_node(n: *mut rb_node, p: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(n: *mut rb_node, r: *mut rb_root); fn rb_erase(n: *mut rb_node, r: *mut rb_root);
    fn kmalloc(n: usize, flags: i32) -> *mut core::ffi::c_void; fn kzalloc(n: usize, flags: i32) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void); fn vmalloc(n: usize) -> *mut core::ffi::c_void; fn vfree(p: *mut core::ffi::c_void);
}

#[repr(C)] pub struct rb_node { pub rb_left:*mut rb_node, pub rb_right:*mut rb_node, pub rb_parent:*mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node:*mut rb_node }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct spinlock_t { _x:[u8;0] }
#[repr(C)] pub struct mutex_t { _x:[u8;0] }
#[repr(C)] pub struct ubifs_bud { pub rb:rb_node, pub list:list_head, pub lnum:i32, pub start:i32, pub jhead:i32, pub log_hash:*mut core::ffi::c_void }
#[repr(C)] pub struct ubifs_wbuf { pub lnum:i32, pub offs:i32 }
#[repr(C)] pub struct ubifs_jhead { pub wbuf:ubifs_wbuf, pub buds_list:list_head, pub log_hash:*mut core::ffi::c_void }
#[repr(C)] pub struct ubifs_info { pub buds_lock:spinlock_t, pub buds:rb_root, pub jheads:*mut ubifs_jhead, pub leb_size:i32, pub lhead_lnum:i32, pub lhead_offs:i32, pub ltail_lnum:i32, pub log_bytes:i64, pub bud_bytes:i64, pub cmt_bud_bytes:i64, pub max_bud_bytes:i64, pub bg_bud_bytes:i64, pub ref_node_alsz:i32, pub min_log_bytes:i32, pub ro_media:i32, pub ro_mount:i32, pub ro_error:i32, pub cmt_state:i32, pub jhead_cnt:i32, pub cmt_no:u64, pub log_mutex:mutex_t, pub old_buds:list_head, pub sbuf:*mut core::ffi::c_void }
#[repr(C)] pub struct ubifs_ch { pub node_type:u32, pub len:u32 }
#[repr(C)] pub struct ubifs_ref_node { pub ch:ubifs_ch, pub lnum:u32, pub offs:u32, pub jhead:u32 }
#[repr(C)] pub struct ubifs_cs_node { pub ch:ubifs_ch, pub cmt_no:u64 }
#[repr(C)] pub struct ubifs_scan_leb { pub nodes:list_head }
#[repr(C)] pub struct ubifs_scan_node { pub list:list_head, pub node:*mut core::ffi::c_void, pub typ:i32 }
#[repr(C)] pub struct done_ref { pub rb:rb_node, pub lnum:i32 }

extern "C" { fn ubifs_next_log_lnum(c:*mut ubifs_info, n:i32)->i32; fn ubifs_leb_unmap(c:*mut ubifs_info,n:i32)->i32; fn ubifs_leb_map(c:*mut ubifs_info,n:i32)->i32; fn ubifs_write_node(c:*mut ubifs_info,p:*mut core::ffi::c_void,s:i32,l:i32,o:i32)->i32; fn ubifs_leb_write(c:*mut ubifs_info,l:i32,p:*mut core::ffi::c_void,o:i32,s:i32)->i32; fn ubifs_leb_change(c:*mut ubifs_info,l:i32,p:*mut core::ffi::c_void,s:i32)->i32; fn ubifs_return_leb(c:*mut ubifs_info,l:i32)->i32; fn ubifs_write_master(c:*mut ubifs_info)->i32; fn ubifs_scan(c:*mut ubifs_info,l:i32,o:i32,s:*mut core::ffi::c_void,x:i32)->*mut ubifs_scan_leb; fn ubifs_scan_destroy(s:*mut ubifs_scan_leb); fn ubifs_commit_required(c:*mut ubifs_info); fn ubifs_request_bg_commit(c:*mut ubifs_info); fn dbg_is_chk_gen(c:*mut ubifs_info)->i32; fn ubifs_err(c:*mut ubifs_info,p:*const u8,...); }

const UBIFS_REF_NODE:u32=1; const UBIFS_CS_NODE:u32=2; const COMMIT_RESTING:i32=0; const GFP_NOFS:i32=0; const EINVAL:i32=22; const ENOMEM:i32=12; const EROFS:i32=30; const EAGAIN:i32=11;
const UBIFS_REF_NODE_SZ:i32=32; const UBIFS_CS_NODE_SZ:i32=32;
#[inline] unsafe fn align(x:i32,a:i32)->i32 { (x+a-1)&!(a-1) }
unsafe fn empty_log_bytes(c:*const ubifs_info)->i64 { let h=(*c).lhead_lnum as i64*(*c).leb_size as i64+(*c).lhead_offs as i64; let t=(*c).ltail_lnum as i64*(*c).leb_size as i64; if h>t {(*c).log_bytes-h+t} else if h!=t {t-h} else if (*c).lhead_lnum!=(*c).ltail_lnum {0} else {(*c).log_bytes} }

pub unsafe fn ubifs_search_bud(c:*mut ubifs_info, lnum:i32)->*mut ubifs_bud { spin_lock(&mut (*c).buds_lock); let mut p=(*c).buds.rb_node; while !p.is_null(){let b=&*(p as *mut ubifs_bud); if lnum<b.lnum {p=(*p).rb_left} else if lnum>b.lnum {p=(*p).rb_right} else {spin_unlock(&mut (*c).buds_lock);return b as *const _ as *mut _}} spin_unlock(&mut (*c).buds_lock); ptr::null_mut() }
pub unsafe fn ubifs_get_wbuf(c:*mut ubifs_info, lnum:i32)->*mut ubifs_wbuf { if (*c).jheads.is_null(){return ptr::null_mut()} ; let b=ubifs_search_bud(c,lnum); if b.is_null(){ptr::null_mut()} else {&mut (*c).jheads[(*b).jhead as usize].wbuf} }

pub unsafe fn ubifs_add_bud(c:*mut ubifs_info,bud:*mut ubifs_bud){spin_lock(&mut (*c).buds_lock);let mut p=&mut (*c).buds.rb_node;let mut parent=ptr::null_mut();while !(*p).is_null(){parent=*p;let b=&*((*p) as *mut ubifs_bud);p=if (*bud).lnum<b.lnum {&mut (**p).rb_left}else{&mut (**p).rb_right}} rb_link_node(&mut (*bud).rb,parent,p);rb_insert_color(&mut (*bud).rb,&mut (*c).buds);(*c).bud_bytes+=(*c).leb_size as i64-(*bud).start as i64;spin_unlock(&mut (*c).buds_lock)}

pub unsafe fn ubifs_add_bud_to_log(c:*mut ubifs_info,jhead:i32,lnum:i32,offs:i32)->i32 {let bud=kmalloc(core::mem::size_of::<ubifs_bud>(),GFP_NOFS);if bud.is_null(){return -ENOMEM}let r=kzalloc((*c).ref_node_alsz as usize,GFP_NOFS);if r.is_null(){kfree(bud);return -ENOMEM}mutex_lock(&mut (*c).log_mutex);if (*c).ro_error!=0{mutex_unlock(&mut (*c).log_mutex);kfree(r);kfree(bud);return -EROFS}if empty_log_bytes(c)-(*c).ref_node_alsz as i64<(*c).min_log_bytes as i64{ubifs_commit_required(c);mutex_unlock(&mut (*c).log_mutex);kfree(r);kfree(bud);return -EAGAIN}let b=bud as *mut ubifs_bud;(*b).lnum=lnum;(*b).start=offs;(*b).jhead=jhead;(*b).log_hash=ptr::null_mut();let q=r as *mut ubifs_ref_node;(*q).ch.node_type=UBIFS_REF_NODE;(*q).lnum=lnum as u32;(*q).offs=offs as u32;(*q).jhead=jhead as u32;if (*c).lhead_offs>(*c).leb_size-(*c).ref_node_alsz{(*c).lhead_lnum=ubifs_next_log_lnum(c,(*c).lhead_lnum);(*c).lhead_offs=0}if (*c).lhead_offs==0{let e=ubifs_leb_unmap(c,(*c).lhead_lnum);if e!=0{mutex_unlock(&mut (*c).log_mutex);kfree(r);kfree(bud);return e}}let e=ubifs_write_node(c,r,UBIFS_REF_NODE_SZ,(*c).lhead_lnum,(*c).lhead_offs);if e==0{(*c).lhead_offs+=(*c).ref_node_alsz;ubifs_add_bud(c,b)}mutex_unlock(&mut (*c).log_mutex);kfree(r);if e!=0{kfree(bud)}e}

unsafe fn remove_buds(c:*mut ubifs_info){(*c).cmt_bud_bytes=0;spin_lock(&mut (*c).buds_lock);let mut p=rb_first(&mut (*c).buds);while !p.is_null(){let n=rb_next(p);let b=&mut *(p as *mut ubifs_bud);let w=&(*c).jheads[b.jhead as usize].wbuf;if w.lnum==b.lnum{(*c).cmt_bud_bytes+=(w.offs-b.start) as i64;b.start=w.offs}else{(*c).cmt_bud_bytes+=(*c).leb_size as i64-b.start as i64;rb_erase(p,&mut (*c).buds)}p=n}spin_unlock(&mut (*c).buds_lock)}

pub unsafe fn ubifs_log_start_commit(c:*mut ubifs_info,tail:*mut i32)->i32{let max=align(UBIFS_CS_NODE_SZ+(*c).jhead_cnt*UBIFS_REF_NODE_SZ,(*c).min_log_bytes);let b=kmalloc(max as usize,GFP_NOFS);if b.is_null(){return -ENOMEM}let cs=b as *mut ubifs_cs_node;(*cs).ch.node_type=UBIFS_CS_NODE;(*cs).cmt_no=(*c).cmt_no;if (*c).lhead_offs!=0{(*c).lhead_lnum=ubifs_next_log_lnum(c,(*c).lhead_lnum);(*c).lhead_offs=0}let e=ubifs_leb_unmap(c,(*c).lhead_lnum);if e==0{let n=align(UBIFS_CS_NODE_SZ,(*c).min_log_bytes);let x=ubifs_leb_write(c,(*c).lhead_lnum,b,0,n);if x==0{*tail=(*c).lhead_lnum;(*c).lhead_offs+=n;remove_buds(c)}}kfree(b);if e!=0{e}else{0}}
pub unsafe fn ubifs_log_end_commit(c:*mut ubifs_info,tail:i32)->i32{mutex_lock(&mut (*c).log_mutex);(*c).ltail_lnum=tail;(*c).min_log_bytes=(*c).leb_size;spin_lock(&mut (*c).buds_lock);(*c).bud_bytes-=(*c).cmt_bud_bytes;spin_unlock(&mut (*c).buds_lock);let e=ubifs_write_master(c);mutex_unlock(&mut (*c).log_mutex);e}
pub unsafe fn ubifs_log_post_commit(c:*mut ubifs_info,old:i32)->i32{let mut n=old;while n!=(*c).ltail_lnum{let e=ubifs_leb_unmap(c,n);if e!=0{return e}n=ubifs_next_log_lnum(c,n)}0}

unsafe fn add_node(c:*mut ubifs_info,buf:*mut u8,lnum:*mut i32,offs:*mut i32,node:*mut u8)->i32{let ch=&*(node as *mut ubifs_ch);let len=ch.len as i32;if len>(*c).leb_size-*offs{let sz=align(*offs,(*c).min_log_bytes);let e=ubifs_leb_change(c,*lnum,buf,sz);if e!=0{return e}*lnum=ubifs_next_log_lnum(c,*lnum);*offs=0}ptr::copy_nonoverlapping(node,buf.add(*offs as usize),len as usize);*offs+=align(len,8);0}
pub unsafe fn ubifs_consolidate_log(c:*mut ubifs_info)->i32{let buf=vmalloc((*c).leb_size as usize) as *mut u8;if buf.is_null(){return -ENOMEM}let mut l=(*c).ltail_lnum;let mut w=l;let mut off=0;loop{let s=ubifs_scan(c,l,0,(*c).sbuf,0);if s.is_null(){vfree(buf as *mut _);return -EINVAL}let e=if off!=0{ubifs_leb_change(c,w,buf,align(off,(*c).min_log_bytes))}else{0};ubifs_scan_destroy(s);if e!=0{vfree(buf as *mut _);return e}if l==(*c).lhead_lnum{break}l=ubifs_next_log_lnum(c,l)}vfree(buf as *mut _);if w==(*c).lhead_lnum{-EINVAL}else{(*c).lhead_lnum=w;(*c).lhead_offs=off;0}}
unsafe fn dbg_check_bud_bytes(c:*mut ubifs_info)->i32{if dbg_is_chk_gen(c)==0{return 0}let mut n=0i64;spin_lock(&mut (*c).buds_lock);for i in 0..(*c).jhead_cnt{let mut p=(*c).jheads.add(i as usize).as_mut().unwrap().buds_list.next;while p!=&mut (*c).jheads.add(i as usize).as_mut().unwrap().buds_list{let b=&*(p as *mut ubifs_bud);n+=(*c).leb_size as i64-b.start;p=(*p).next}}spin_unlock(&mut (*c).buds_lock);if n!=(*c).bud_bytes{-EINVAL}else{0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
