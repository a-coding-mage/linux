// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level Rust translation of UBIFS io.c.  Kernel and UBIFS types,
// constants, locks, allocators, and helpers are supplied by other units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    fn ubi_read(ubi: *mut c_void, lnum: i32, buf: *mut c_void, offs: i32, len: i32) -> i32;
    fn ubi_leb_write(ubi: *mut c_void, lnum: i32, buf: *const c_void, offs: i32, len: i32) -> i32;
    fn ubi_leb_change(ubi: *mut c_void, lnum: i32, buf: *const c_void, len: i32) -> i32;
    fn ubi_leb_unmap(ubi: *mut c_void, lnum: i32) -> i32;
    fn ubi_leb_map(ubi: *mut c_void, lnum: i32) -> i32;
    fn ubi_is_mapped(ubi: *mut c_void, lnum: i32) -> i32;
    fn crc32(seed: u32, p: *const u8, len: usize) -> u32;
}

// The following declarations intentionally mirror the structures and helpers
// from ubifs.h; their definitions are provided by the surrounding translation.
#[repr(C)] pub struct ubifs_info { pub ro_error:i32, pub no_chk_data_crc:i32, pub vfs_sb:*mut vfs_super_block, pub ubi:*mut c_void, pub leb_cnt:i32, pub leb_size:i32, pub min_io_size:i32, pub max_write_size:i32, pub max_write_shift:i32, pub max_sqnum:u64, pub mounting:i32, pub remounting_rw:i32, pub probing:i32, pub need_wbuf_sync:i32, pub jhead_cnt:i32, pub leb_start:i32, pub space_fixup:i32, pub stats:*mut ubifs_stats_info, pub ranges:*mut ubifs_node_range, pub jheads:*mut ubifs_jhead, pub cnt_lock: spinlock_t }
#[repr(C)] pub struct vfs_super_block { pub s_flags:u32 }
#[repr(C)] pub struct ubifs_stats_info { pub magic_errors:u32, pub node_errors:u32, pub crc_errors:u32 }
#[repr(C)] pub struct ubifs_node_range { pub len:i32, pub min_len:i32, pub max_len:i32 }
#[repr(C)] pub struct ubifs_jhead { pub wbuf:ubifs_wbuf }
#[repr(C)] pub struct spinlock_t { _x:u8 }
#[repr(C)] pub struct mutex { _x:u8 }
#[repr(C)] pub struct hrtimer { _x:u8 }
#[repr(C)] pub struct inode { pub i_ino:u64 }
#[repr(C)] pub struct ubifs_wbuf { pub buf:*mut u8, pub inodes:*mut u64, pub used:i32, pub avail:i32, pub size:i32, pub lnum:i32, pub offs:i32, pub next_ino:i32, pub jhead:i32, pub need_sync:i32, pub no_timer:i32, pub c:*mut ubifs_info, pub io_mutex:mutex, pub lock:spinlock_t, pub timer:hrtimer, pub sync_callback:Option<unsafe extern "C" fn(*mut ubifs_info,i32,i32,i32)->i32> }
#[repr(C)] pub struct ubifs_ch { pub magic:u32, pub crc:u32, pub node_type:u8, pub group_type:u8, pub padding:[u8;2], pub sqnum:u64, pub len:u32 }
#[repr(C)] pub struct ubifs_pad_node { pub ch:ubifs_ch, pub pad_len:u32 }

const EINVAL:i32=22; const EUCLEAN:i32=117; const EROFS:i32=30; const ENOSPC:i32=28;
const UBIFS_NODE_MAGIC:u32=0x06101831; const UBIFS_CH_SZ:i32=24; const UBIFS_PAD_NODE_SZ:i32= padding_size();
const UBIFS_PADDING_BYTE:u8=0xff; const UBIFS_CRC32_INIT:u32=0; const UBIFS_DATA_NODE:i32=1;
const UBIFS_PAD_NODE:i32=8; const UBIFS_NO_NODE_GROUP:u8=0; const UBIFS_LAST_OF_NODE_GROUP:u8=2; const UBIFS_IN_NODE_GROUP:u8=1; const UBIFS_NODE_TYPES_CNT:i32=32; const GCHD:i32=1;
const SB_RDONLY:u32=1; const SQNUM_WARN_WATERMARK:u64=0xffff_ffff_ffff_0000; const SQNUM_WATERMARK:u64=0xffff_ffff_ffff_ff00;
const fn padding_size()->i32 { 32 }
#[inline] unsafe fn le32(x:u32)->u32 { u32::from_le(x) } #[inline] unsafe fn le64(x:u64)->u64 { u64::from_le(x) }
#[inline] fn align(x:i32,a:i32)->i32 { (x+a-1)&!(a-1) }

extern "C" { fn ubifs_warn(c:*const ubifs_info,fmt:*const u8,...); fn ubifs_err(c:*const ubifs_info,fmt:*const u8,...); fn ubifs_errc(c:*const ubifs_info,fmt:*const u8,...); fn ubifs_dump_node(c:*const ubifs_info,p:*const c_void,len:i32); fn ubifs_dump_leb(c:*const ubifs_info,lnum:i32); fn dump_stack(); fn ubifs_ro_mode(c:*mut ubifs_info,err:i32); fn ubifs_node_insert_hmac(c:*mut ubifs_info,node:*mut c_void,len:i32,offs:i32)->i32; fn dbg_is_tst_rcvry(c:*const ubifs_info)->i32; fn dbg_leb_write(c:*mut ubifs_info,l:i32,b:*const c_void,o:i32,n:i32)->i32; fn dbg_leb_change(c:*mut ubifs_info,l:i32,b:*const c_void,n:i32)->i32; fn dbg_leb_unmap(c:*mut ubifs_info,l:i32)->i32; fn dbg_leb_map(c:*mut ubifs_info,l:i32)->i32; }

pub unsafe fn ubifs_leb_read(c:*const ubifs_info,l:i32,b:*mut c_void,o:i32,n:i32,even:i32)->i32 { let e=ubi_read((*c).ubi,l,b,o,n); if e!=0 && (e!=-74 || even!=0) { ubifs_err(c,b"reading LEB\0".as_ptr(),n,l,o,e); dump_stack(); } e }
pub unsafe fn ubifs_leb_write(c:*mut ubifs_info,l:i32,b:*const c_void,o:i32,n:i32)->i32 { if (*c).ro_error!=0{return -EROFS}; let e=if dbg_is_tst_rcvry(c)!=0 {dbg_leb_write(c,l,b,o,n)} else {ubi_leb_write((*c).ubi,l,b,o,n)}; if e!=0 { ubifs_ro_mode(c,e); dump_stack(); } e }
pub unsafe fn ubifs_leb_change(c:*mut ubifs_info,l:i32,b:*const c_void,n:i32)->i32 { if (*c).ro_error!=0{return -EROFS}; let e=if dbg_is_tst_rcvry(c)!=0 {dbg_leb_change(c,l,b,n)} else {ubi_leb_change((*c).ubi,l,b,n)}; if e!=0 {ubifs_ro_mode(c,e);dump_stack();} e }
pub unsafe fn ubifs_leb_unmap(c:*mut ubifs_info,l:i32)->i32 { if (*c).ro_error!=0{return -EROFS}; let e=if dbg_is_tst_rcvry(c)!=0{dbg_leb_unmap(c,l)}else{ubi_leb_unmap((*c).ubi,l)};if e!=0{ubifs_ro_mode(c,e);dump_stack();}e }
pub unsafe fn ubifs_leb_map(c:*mut ubifs_info,l:i32)->i32 { if (*c).ro_error!=0{return -EROFS}; let e=if dbg_is_tst_rcvry(c)!=0{dbg_leb_map(c,l)}else{ubi_leb_map((*c).ubi,l)};if e!=0{ubifs_ro_mode(c,e);dump_stack();}e }
pub unsafe fn ubifs_is_mapped(c:*const ubifs_info,l:i32)->i32 { ubi_is_mapped((*c).ubi,l) }

unsafe fn record_magic_error(s:*mut ubifs_stats_info){if !s.is_null(){(*s).magic_errors+=1;}} unsafe fn record_node_error(s:*mut ubifs_stats_info){if !s.is_null(){(*s).node_errors+=1;}} unsafe fn record_crc_error(s:*mut ubifs_stats_info){if !s.is_null(){(*s).crc_errors+=1;}}
pub unsafe fn ubifs_check_node(c:*const ubifs_info,b:*const u8,len:i32,lnum:i32,offs:i32,quiet:i32,must:i32)->i32 { let ch=&*(b as *const ubifs_ch); let magic=le32(ch.magic); if magic!=UBIFS_NODE_MAGIC {record_magic_error((*c).stats);return -EUCLEAN} let t=ch.node_type as i32;if t<0||t>=UBIFS_NODE_TYPES_CNT{record_node_error((*c).stats);return -EINVAL} let nl=le32(ch.len) as i32;if nl+offs>(*c).leb_size{return -EINVAL} if must==0&&t==UBIFS_DATA_NODE&&(*c).mounting==0&&(*c).remounting_rw==0&&(*c).no_chk_data_crc!=0{return 0} let crc=crc32(UBIFS_CRC32_INIT,b.add(8), (nl-8) as usize);if crc!=le32(ch.crc){record_crc_error((*c).stats);return -EUCLEAN} let _=quiet;let _=len;0 }
pub unsafe fn ubifs_pad(_c:*const ubifs_info,b:*mut u8,mut pad:i32){if pad>=UBIFS_PAD_NODE_SZ{let ch=&mut*(b as *mut ubifs_ch);ch.magic=UBIFS_NODE_MAGIC.to_le();ch.node_type=UBIFS_PAD_NODE as u8;ch.group_type=0;ch.len=(UBIFS_PAD_NODE_SZ as u32).to_le();pad-=UBIFS_PAD_NODE_SZ;(*(b as *mut ubifs_pad_node)).pad_len=(pad as u32).to_le();core::ptr::write_bytes(b.add(UBIFS_PAD_NODE_SZ as usize),0,pad as usize)}else if pad>0{core::ptr::write_bytes(b,UBIFS_PADDING_BYTE,pad as usize)}}
pub unsafe fn ubifs_crc_node(node:*mut u8,len:i32){let ch=&mut*(node as *mut ubifs_ch);ch.crc=crc32(0,node.add(8),(len-8)as usize).to_le()}
pub unsafe fn ubifs_init_node(c:*mut ubifs_info,node:*mut u8,len:i32,pad:i32){let ch=&mut*(node as *mut ubifs_ch);ch.magic=UBIFS_NODE_MAGIC.to_le();ch.len=(len as u32).to_le();ch.group_type=0;ch.padding=[0,0];if pad!=0{let l=align(len,8);ubifs_pad(c,node.add(l as usize),align(l,(*c).min_io_size)-l)}}
pub unsafe fn ubifs_prepare_node_hmac(c:*mut ubifs_info,node:*mut u8,len:i32,h:i32,pad:i32)->i32{ubifs_init_node(c,node,len,pad);if h>0{let e=ubifs_node_insert_hmac(c,node as *mut c_void,len,h);if e!=0{return e}}ubifs_crc_node(node,len);0}
pub unsafe fn ubifs_prepare_node(c:*mut ubifs_info,node:*mut u8,len:i32,pad:i32){let _=ubifs_prepare_node_hmac(c,node,len,0,pad);}
pub unsafe fn ubifs_write_node_hmac(c:*mut ubifs_info,b:*mut u8,len:i32,l:i32,o:i32,h:i32)->i32{let bl=align(len,(*c).min_io_size);let e=ubifs_prepare_node_hmac(c,b,len,h,1);if e!=0{return e}ubifs_leb_write(c,l,b as *const c_void,o,bl)}
pub unsafe fn ubifs_write_node(c:*mut ubifs_info,b:*mut u8,len:i32,l:i32,o:i32)->i32{ubifs_write_node_hmac(c,b,len,l,o,-1)}

// Remaining write-buffer operations preserve the original externally visible
// entry points; synchronization and allocation primitives are kernel-owned.
pub unsafe fn ubifs_wbuf_sync_nolock(w:*mut ubifs_wbuf)->i32{if (*w).used==0||(*w).lnum<0{return 0}let c=(*w).c;let n=align((*w).used,(*c).min_io_size);let e=ubifs_leb_write(c,(*w).lnum,(*w).buf as *const c_void,(*w).offs,n);if e==0{(*w).offs+=n;(*w).used=0;(*w).avail=(*w).size;}e}
pub unsafe fn ubifs_wbuf_seek_nolock(w:*mut ubifs_wbuf,l:i32,o:i32)->i32{(*w).lnum=l;(*w).offs=o;(*w).used=0;(*w).size=(*w).c.as_ref().unwrap().max_write_size;(*w).avail=(*w).size;0}
pub unsafe fn ubifs_wbuf_write_nolock(w:*mut ubifs_wbuf,b:*mut u8,len:i32)->i32{let c=(*w).c;let n=align(len,8);if n>(*w).avail{let e=ubifs_wbuf_sync_nolock(w);if e!=0{return e}}core::ptr::copy_nonoverlapping(b,(*w).buf.add((*w).used as usize),len as usize);(*w).used+=n;(*w).avail-=n;if (*w).avail==0{ubifs_wbuf_sync_nolock(w)}else{0}}
pub unsafe fn ubifs_read_node(c:*const ubifs_info,b:*mut u8,t:i32,len:i32,l:i32,o:i32)->i32{let e=ubifs_leb_read(c,l,b as *mut c_void,o,len,0);if e!=0&&e!=-74{return e}if (*(b as *const ubifs_ch)).node_type as i32!=t{return -EINVAL}ubifs_check_node(c,b,len,l,o,0,0)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
